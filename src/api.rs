use reqwest::Error;
use serde::Deserialize;

const CRATES_IO_API: &str = "https://crates.io/api/v1/crates/";

#[derive(Deserialize, Debug)]
pub struct CrateInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub downloads: u32,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub max_version: String,
    pub versions: Vec<VersionInfo>,
}

#[derive(Deserialize, Debug)]
pub struct VersionInfo {
    pub num: String,
    pub downloads: u32,
    pub created_at: String,
    pub dl_path: String,
    pub license: Option<String>,
    pub published_by: Option<User>,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub login: String,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub url: Option<String>,
}

#[derive(Deserialize, Debug)]
struct CrateResponse {
    crate: CrateInfo,
    versions: Vec<VersionInfo>,
}

pub async fn fetch_crate_info(crate_name: &str) -> Result<CrateResponse, Error> {
    let url = format!("{}{}", CRATES_IO_API, crate_name);
    let response = reqwest::get(&url).await?;
    let crate_info = response.json::<CrateResponse>().await?;
    Ok(crate_info)
}
