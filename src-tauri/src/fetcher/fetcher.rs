use std::fs;

use reqwest;
use serde::{Deserialize, Serialize};
use reqwest::header::USER_AGENT;

use crate::{environmnet::is_prod, test_data};

const GITHUB_URL: &str = "https://api.github.com/repos/godotengine/godot/releases";

#[derive(Serialize, Deserialize, Clone)]
pub struct Asset {
    pub browser_download_url: String,
    pub name: String,
    pub created_at: String,
    pub size: i64
}

#[derive(Serialize, Deserialize)]
struct Release {
    assets: Vec<Asset>
}

fn filter_assets(releases: &Vec<Release>, filter: String) -> Vec<Asset> {
    let assets: Vec<&Asset> = releases.into_iter().map(|release|{
        &release.assets
    }).flatten().collect();

    let filtered: Vec<Asset> = assets.into_iter().filter(|asset|{
        asset.name.contains(&filter)
    }).map(|asset| asset.clone()).collect();

    return filtered;
}

async fn get_releases() -> Result<Vec<Release>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body: String = if is_prod() { 
        client
        .get(GITHUB_URL)
        .header(USER_AGENT, "My Rust Program 1.0")
        .send()
        .await?
        .text()
        .await?
    } else {
        test_data::TEST_DATA.to_string()
    };

    let value: Vec<Release> = serde_json::from_str(&body).unwrap();

    return Ok(value);
}

#[cfg(test)]
mod tests {
    use crate::fetcher::fetcher::{get_releases, filter_assets};

    #[tokio::test]
    async fn test_releases() {
        let releases = get_releases().await.unwrap();

        assert!(releases.len() >= 1)
    }

    #[tokio::test]
    async fn test_filter_assets() {
        let releases = get_releases().await.unwrap();

        let filtered = filter_assets(&releases, "win64".to_string());

        for asset in &filtered {
            println!("{}", asset.name)
        }

        assert!(filtered.len() >= 10)
    }
}
