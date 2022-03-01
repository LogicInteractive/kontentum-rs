use actix_web::client::Client;
pub use kontentum_core::*;
use serde::de::DeserializeOwned;
use std::io::Write;

const KONTENTUM_DOWNLOAD_LIMIT_BYTES: usize = 1_000_000_000; // 1 GB limit

pub fn get_json_path(exhibit_token: &str) -> std::io::Result<std::path::PathBuf> {
    let mut local_path_buf = std::env::current_dir()?;
    local_path_buf.push("kontentum");
    local_path_buf.push(format!("{}.json", exhibit_token).as_str());
    Ok(local_path_buf)
}

pub async fn download_exhibit<T: DeserializeOwned>(
    exhibit_token: &str,
) -> Result<KontentumExhibit<T>, Box<dyn std::error::Error>> {
    let exhibit_url = format!("{}/rest/getExhibit/{}", KONTENTUM_URL, exhibit_token);
    log::info!("Downloading from {}", exhibit_url);

    let client = Client::default();
    let data = client
        .get(exhibit_url)
        .send()
        .await?
        .body()
        .limit(KONTENTUM_DOWNLOAD_LIMIT_BYTES)
        .await?;

    // Write to local file

    let local_path = get_json_path(exhibit_token)?;
    let _ = std::fs::create_dir_all(local_path.parent().ok_or("can't resolve parent")?)?; // Create directory if it does not yet exist
    let mut file = std::fs::File::create(&local_path)?;
    let _ = file.write(&data);
    log::info!("Downloaded exhibit: {}", &local_path.display().to_string());

    let json_string = std::fs::read_to_string(&local_path)?;
    let kontentum_exhibit = serde_json::from_str(&json_string);

    kontentum_exhibit.map_err(|e| {
        log::warn!("Kontentum parse error: {:?}", e);
        e.into()
    })
}

pub async fn download_file(
    kontentum_file: &KontentumFile,
) -> Result<String, Box<dyn std::error::Error>> {
    // Local file path

    let mut local_path = std::env::current_dir()?;
    local_path.push("kontentum");
    local_path.push("files");
    let _ = std::fs::create_dir_all(local_path.as_path())?; // Create directory if it does not yet exist
    local_path.push(&kontentum_file.file.as_str());

    // Skip if file already exists, otherwise download and write to disk

    if std::path::Path::exists(&local_path) {
        log::info!("File already exists: {}", local_path.display().to_string())
    } else {
        let client = Client::default();
        let url = format!("{}/{}", KONTENTUM_FILEVAULT_URL, kontentum_file.file);
        log::info!(
            "Downloading file: {}, saving to {}",
            url,
            &local_path.display()
        );
        let request = client.get(url).send().await;
        if request.is_err() {
            log::warn!("{:?}", request);
        }
        let data = request?.body().limit(KONTENTUM_DOWNLOAD_LIMIT_BYTES).await;
        if data.is_err() {
            log::warn!("{:?}", data);
        }
        let mut file = std::fs::File::create(&local_path)?;
        let _ = file.write(&data?);
        log::info!("Downloaded file: {}", &local_path.display().to_string());
    }

    Ok(local_path.display().to_string())
}
