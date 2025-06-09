use crate::{LocalFile, VectorFile, git::get_git_info};
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{CreateFileRequest, CreateVectorStoreFileRequest, FileInput, FilePurpose, InputSource},
};
use indicatif::ProgressBar;
use std::path::Path;

pub async fn get_files_from_vector_store(
    client: &Client<OpenAIConfig>,
    store: &str,
) -> Result<Vec<VectorFile>, String> {
    let stores = client.vector_stores();
    let files = client.files();
    let store_files = stores.files(store);
    let mut has_more = true;
    let mut result = vec![];
    let mut last_id = "".to_string();
    let mut file_ids = vec![];
    while has_more {
        let files_list = store_files
            .list(&[("limit", "100"), ("after", &last_id)])
            .await
            .unwrap();
        has_more = files_list.has_more;
        last_id = files_list.last_id.unwrap_or_default();
        for file in files_list.data {
            file_ids.push(file.id);
        }
    }
    let pb = ProgressBar::new(file_ids.len() as u64);
    for file_id in &file_ids {
        let mut retries = 3;
        let file_data = loop {
            match files.retrieve(file_id).await {
                Ok(data) => {
                    break data;
                }
                Err(e) => {
                    retries -= 1;
                    if retries == 0 {
                        eprintln!("error while retrieving file {file_id} after 3 attempts: {e:?}");
                        continue;
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                }
            }
        };
        result.push(VectorFile {
            id: file_id.to_string(),
            filename: file_data.filename,
        });
        pb.inc(1);
    }
    pb.finish_with_message("done");
    Ok(result)
}

pub async fn upload_file(
    client: &Client<OpenAIConfig>,
    vector_store: &str,
    local_dir: &str,
    ft: &LocalFile,
    embed_git_info: bool,
) {
    let full_path = Path::new(local_dir).join(ft.path.clone());
    // embed git metadata into markdown files
    let data: Vec<u8> = if embed_git_info && ft.path.extension().unwrap_or_default() == "md" {
        let mut git_info = get_git_info(&full_path);
        let content = std::fs::read_to_string(full_path).unwrap();
        git_info.push_str(&content);
        git_info.as_bytes().to_vec()
    } else {
        std::fs::read(full_path).unwrap()
    };
    let files = client.files();
    let vector_stores = client.vector_stores();
    let result = files
        .create(CreateFileRequest {
            file: FileInput {
                source: InputSource::VecU8 {
                    filename: ft.openai_filename.to_string(),
                    vec: data,
                },
            },
            purpose: FilePurpose::Assistants,
        })
        .await
        .unwrap();
    // attach to vectore store
    let x = vector_stores.files(vector_store);
    x.create(CreateVectorStoreFileRequest {
        file_id: result.id,
        chunking_strategy: None,
        attributes: None,
    })
    .await
    .unwrap();
}

pub async fn remove_file(client: &Client<OpenAIConfig>, file_id: &str) {
    let files = client.files();
    let result = files.delete(file_id).await;
    if let Err(err) = result {
        eprintln!("error while deleteing file {file_id}: {err:?}");
    }
}
