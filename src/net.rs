use anyhow::{anyhow, Ok};
use std::path::Path;

use tokio::{fs::OpenOptions, io::AsyncWriteExt};

pub async fn download_to<P: AsRef<Path>>(url: &str, save_path: P) -> anyhow::Result<()> {
    let response = reqwest::get(url).await?;

    // 检查响应状态码
    if !response.status().is_success() {
        return Err(anyhow!("Request failed"));
    }

    // 打开文件以保存响应内容
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(save_path)
        .await?;

    file.write_all(&response.bytes().await?).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::download_to;

    #[tokio::test]
    pub async fn download_config_json() -> anyhow::Result<()> {
        download_to(
            "https://huggingface.co/Helsinki-NLP/opus-mt-zh-en/raw/main/config.json",
            "/tmp/config.json",
        )
        .await
    }
}
