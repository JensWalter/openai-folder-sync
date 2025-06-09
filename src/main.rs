use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use indicatif::ProgressBar;
use local::get_files_from_filesystem;
use openai::{get_files_from_vector_store, upload_file};
use std::{path::PathBuf, process::Command};

mod local;
mod openai;

#[derive(Debug)]
pub struct LocalFile {
    pub path: PathBuf,
    pub md5: String,
    pub openai_filename: String,
}

#[derive(Debug)]
pub struct VectorFile {
    pub id: String,
    pub filename: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, env)]
    openai_api_key: String,

    #[arg(short, long, env)]
    vector_store: String,

    #[arg(short, long, env)]
    local_dir: String,

    #[arg(
        short,
        long,
        env,
        help = "comma separated list of file extensions to sync"
    )]
    extensions: Option<String>,

    #[arg(
        short,
        long,
        env,
        help = "embed git info from git cli into the file content, if filetype is markdown"
    )]
    git_info: Option<bool>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let c = OpenAIConfig::new().with_api_key(args.openai_api_key);
    let client = Client::with_config(c);

    println!("fetching file list from openai");
    let vector_files = get_files_from_vector_store(&client, &args.vector_store)
        .await
        .unwrap();
    println!("got {} files from openai", vector_files.len());

    println!("fetching file list from filesystem");
    let x = args.extensions.clone().unwrap_or_default();
    let exts: Vec<&str> = x.split(',').collect();
    let local_files = get_files_from_filesystem(&args.local_dir, exts);
    println!("got {} files from filesystem", local_files.len());

    // files to add to openai
    println!("uploading files ...");
    let pb = ProgressBar::new(local_files.len() as u64);
    for file in &local_files {
        pb.inc(1);
        let mut found = false;
        for e in &vector_files {
            if e.filename == file.openai_filename {
                found = true;
            }
        }
        if !found {
            upload_file(
                &client,
                &args.vector_store,
                &args.local_dir,
                file,
                args.git_info.unwrap_or(false),
            )
            .await;
        }
    }
    pb.finish_with_message("done");

    // files to remove from openai
    println!("checking for file removals ...");
    let pb = ProgressBar::new(vector_files.len() as u64);
    for file in &vector_files {
        pb.inc(1);
        let mut found = false;
        for e in &local_files {
            if e.openai_filename == file.filename {
                found = true;
            }
        }
        if !found {
            openai::remove_file(&client, &file.id).await;
        }
    }
    pb.finish_with_message("done");
    println!("done.");
}

fn get_git_info(path: &PathBuf) -> String {
    let output = Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--pretty=format:'%an'")
        .arg("--")
        .arg(path)
        .output()
        .unwrap();
    let username = String::from_utf8(output.stdout).unwrap().trim().to_string();
    let output = Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--pretty=format:'%ai'")
        .arg("--")
        .arg(path)
        .output()
        .unwrap();
    let date = String::from_utf8(output.stdout).unwrap().trim().to_string();
    format!("last commit at {}\nlast commit from {}\n\n", date, username)
}

#[test]
fn test_git_info() {
    let local_files = get_files_from_filesystem("./src", vec!["rs"]);
    println!("got {} files from filesystem", local_files.len());
    assert_eq!(local_files.len(), 3);
    for file in &local_files {
        let final_path = std::path::Path::new("./src").join(file.path.clone());
        println!("{}", final_path.display());
        let content = std::fs::read_to_string(&final_path).unwrap();
        let mut git_info = get_git_info(&final_path);
        println!("{} {}", final_path.display(), git_info);
        git_info.push_str(&content);
        println!("{}", git_info);
    }
}
