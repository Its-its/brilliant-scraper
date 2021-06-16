use crate::scraping::CommunityListContribution;


static CONTRIBUTION_FILE_NAME: &str = ".contributions.cache";


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