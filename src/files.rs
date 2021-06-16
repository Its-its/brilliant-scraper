use std::{path::PathBuf, str::FromStr};

use reqwest::Url;

use crate::scraping::CommunityListContribution;


static CONTRIBUTION_FILE_NAME: &str = ".contributions.cache";
static SAVE_DIRECTORY_PATH: &str = "./archive";


// CONTRIBUTIONS

pub async fn save_contributions_file(contributions: &[CommunityListContribution]) -> Result<(), Box<dyn std::error::Error>> {
	tokio::fs::write(CONTRIBUTION_FILE_NAME, serde_json::to_string_pretty(contributions)?).await?;

	Ok(())
}

pub async fn read_contributions_file() -> Result<Vec<CommunityListContribution>, Box<dyn std::error::Error>> {
	let value = tokio::fs::read(CONTRIBUTION_FILE_NAME).await?;
	Ok(serde_json::from_slice(&value)?)
}

pub async fn does_contributions_file_exist() -> bool {
	tokio::fs::metadata(CONTRIBUTION_FILE_NAME).await.is_ok()
}


// SAVE DIRECTORY

pub async fn create_save_directory() -> Result<(), Box<dyn std::error::Error>> {
	if tokio::fs::metadata(SAVE_DIRECTORY_PATH).await.is_err() {
		tokio::fs::create_dir(SAVE_DIRECTORY_PATH).await?;
	}

	Ok(())
}

pub async fn does_data_url_exist(url: &str) -> Result<bool, Box<dyn std::error::Error>> {
	let url: Url = url.parse()?;

	let mut path = PathBuf::from_str(SAVE_DIRECTORY_PATH)?;
	path.push(&url.path()[1..]);

	Ok(tokio::fs::metadata(path).await.is_ok())
}


pub async fn save_data_to_directory(url: &str, contents: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
	let url: Url = url.parse()?;

	let mut path = PathBuf::from_str(SAVE_DIRECTORY_PATH)?;
	path.push(&url.path()[1..]);

	let directory_path = path.parent().expect("no directory.");

	// Create directories if they don't exist.
	if tokio::fs::metadata(directory_path).await.is_err() {
		tokio::fs::create_dir_all(directory_path).await?;
	}

	tokio::fs::write(path, contents).await?;

	Ok(())
}