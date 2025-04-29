use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use indicatif::ProgressBar;
use local::get_files_from_filesystem;
use openai::{get_files_from_vector_store, upload_file};
use std::path::PathBuf;

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

    #[arg(short, long, env)]
    extensions: Option<String>,
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
            upload_file(&client, &args.vector_store, &args.local_dir, file).await;
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
