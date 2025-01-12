use std::path::Path;

use serde::{de::DeserializeOwned, Serialize};

use crate::{http_client, Result};

pub async fn load_file_or_download_serialized<T>(path: String, download_url: String) -> Result<T>
where
    T: Serialize + DeserializeOwned,
{
    if let Some(file_content) = load_serialized_file(path.clone()).await? {
        Ok(file_content)
    } else {
        let downloaded_content = http_client::download(download_url.clone()).await?;
        save_file(path.clone(), downloaded_content.clone()).await?;

        let parsed_content = serde_json::from_slice(&downloaded_content)?;
        Ok(parsed_content)
    }
}

pub async fn load_file_or_download(path: String, download_url: String) -> Result<Vec<u8>> {
    if let Some(file_content) = load_file(path.clone()).await? {
        Ok(file_content)
    } else {
        let downloaded_content = http_client::download(download_url).await?;
        save_file(path, downloaded_content.clone()).await?;
        Ok(downloaded_content)
    }
}

pub async fn load_serialized_file<T>(path: String) -> Result<Option<T>>
where
    T: Serialize + DeserializeOwned,
{
    if !Path::new(&path).exists() {
        return Ok(None);
    }

    let file_content = tokio::fs::read_to_string(path).await?;
    let parsed_file: T = serde_json::from_str(&file_content)?;
    Ok(Some(parsed_file))
}

pub async fn load_file(path: String) -> Result<Option<Vec<u8>>> {
    if !Path::new(&path).exists() {
        return Ok(None);
    }

    let file_content = tokio::fs::read(path).await?;
    Ok(Some(file_content))
}

pub async fn save_serialized_file(path: String, content: impl Serialize) -> Result<()> {
    create_parent_folders(path.clone()).await?;
    let serialized_content = serde_json::to_string(&content)?;
    save_file(path, serialized_content.into_bytes()).await
}

pub async fn save_file(path: String, content: Vec<u8>) -> Result<()> {
    create_parent_folders(path.clone()).await?;
    tokio::fs::write(path, content).await?;
    Ok(())
}

pub async fn create_parent_folders(path: String) -> Result<()> {
    let parent = Path::new(&path).parent().unwrap();
    tokio::fs::create_dir_all(parent).await?;
    Ok(())
}
