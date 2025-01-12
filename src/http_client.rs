use serde::de::DeserializeOwned;

use crate::Result;

pub async fn download_serialized<T>(url: String) -> Result<T>
where
    T: DeserializeOwned,
{
    let response = reqwest::get(url).await?;

    response.json::<T>().await.map_err(crate::Error::from)
}

pub async fn download(url: String) -> Result<Vec<u8>> {
    println!("Starting to download form url {}", url.clone());
    let response = reqwest::get(url).await?;

    response
        .bytes()
        .await
        .map(|bytes| bytes.to_vec())
        .map_err(crate::Error::from)
}
