use crate::{LocalFile, VectorFile};
use async_openai::{
    config::OpenAIConfig,
    types::{CreateFileRequest, CreateVectorStoreFileRequest, FileInput, FilePurpose, InputSource},
    Client,
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
        last_id = files_list.last_id;
        for file in files_list.data {
            file_ids.push(file.id);
        }
    }
    let pb = ProgressBar::new(file_ids.len() as u64);
    for file_id in &file_ids {
        let file_data = files.retrieve(file_id).await.unwrap();
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
) {
    let full_path = Path::new(local_dir).join(ft.path.clone());
    let data: Vec<u8> = std::fs::read(full_path).unwrap();
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
    })
    .await
    .unwrap();
}

pub async fn remove_file(client: &Client<OpenAIConfig>, file_id: &str) {
    let files = client.files();
    files.delete(file_id).await.unwrap();
    // println!("{result:?}");
}
