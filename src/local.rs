use crate::LocalFile;
use indicatif::{ProgressBar, ProgressStyle};
use md5::{Digest, Md5};
use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::time::Duration;

fn hash_files(base_dir: &Path, files: Vec<&PathBuf>) -> Result<Vec<crate::LocalFile>, io::Error> {
    let mut hashed_files = Vec::new();
    println!("hashing files ...");
    let pb = ProgressBar::new(files.len() as u64);
    for file_path in files {
        let full_path = base_dir.join(file_path);
        // println!("found file {full_path:?}");
        let mut file = File::open(&full_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let mut hasher = Md5::new();
        hasher.update(&buffer);
        let result = hasher.finalize();
        let md5_string = format!("{:x}", result);

        hashed_files.push(crate::LocalFile {
            openai_filename: generate_filename(file_path, &md5_string),
            path: file_path.clone(),
            md5: md5_string,
        });
        pb.inc(1);
    }
    pb.finish_with_message("done");
    Ok(hashed_files)
}

pub fn generate_filename(path: &Path, md5: &str) -> String {
    let ext = path.extension().unwrap_or_default();
    format!(
        "{}_{md5}.{}",
        path.to_string_lossy().replace('.', "_"),
        ext.to_string_lossy()
    )
}

pub fn get_files_from_filesystem(base_path: &str, extensions: Vec<&str>) -> Vec<LocalFile> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );
    pb.set_message("fetching files ...");
    let path = Path::new(base_path);
    let files = crawl_directory(path, path).unwrap();
    let files: Vec<&PathBuf> = files
        .iter()
        .filter(|elem| {
            if extensions.is_empty() {
                true
            } else {
                let ext = elem
                    .extension()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                extensions.contains(&ext.as_str())
            }
        })
        .collect();
    pb.finish_with_message("Done");
    hash_files(path, files).unwrap()
}

fn crawl_directory(base_dir: &Path, current_dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            files.push(path.strip_prefix(base_dir).unwrap().to_path_buf());
        } else if path.is_dir() {
            let mut subdir_files = crawl_directory(base_dir, &path)?;
            files.append(&mut subdir_files);
        }
    }

    Ok(files)
}
