use std::sync::LazyLock;

use reqwest::Client;
use serde::{Deserialize, Serialize};

const DWNL_URL: &'static str =
  "https://api.github.com/repos/ahq-softwares/llama.cpp/releases/latest";

pub static CLIENT: LazyLock<Client> = LazyLock::new(|| {
  Client::builder()
    .user_agent("AHQ AI Downloader")
    .build()
    .unwrap()
});

#[derive(Debug, Serialize, Deserialize)]
struct Release {
  tag_name: String,
  assets: Vec<Asset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
  pub name: String,
  pub browser_download_url: String,
}

pub async fn dwnl(url: &str) -> Vec<u8> {
  CLIENT.get(url)
    .send()
    .await
    .unwrap()
    .bytes()
    .await
    .unwrap()
    .into()
}

pub async fn get_platform_assets() -> (String, Vec<Asset>) {
  let data: Release = CLIENT
    .get(DWNL_URL)
    .send()
    .await
    .unwrap()
    .json()
    .await
    .unwrap();

  (
    data.tag_name,
    data
      .assets
      .into_iter()
      .filter(|x| {
        #[cfg(windows)]
        let term = "windows";

        #[cfg(target_os = "linux")]
        let term = "windows";
        x.name.contains(term)
      })
      .collect(),
  )
}
