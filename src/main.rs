use std::time::Duration;

use reqwest::{Client, header::HeaderMap};

mod files;
mod requesting;
mod scraping;

use files::*;
use requesting::*;
use scraping::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("Executing Step 1: Grabbing ALL Contribution URLS and caching.");
	let _ = step_1_grab_all_contributions().await?;


	Ok(())
}


async fn step_1_grab_all_contributions() -> Result<Vec<CommunityListContribution>, Box<dyn std::error::Error>> {
	if does_contributions_file_exist().await {
		println!(" - Contributions File already exists. Using it.");
		return read_contributions_file().await;
	}

	let mut found: Vec<CommunityListContribution> = Vec::new();

	let mut next_page_url = Some(
		"https://brilliant.org/community/home/problems/popular/all/all/?&deferred=true&page_key=community_portal_problems&version=1".to_string()
	);

	let client = Client::builder()
		.default_headers(default_headers())
		.cookie_store(true)
		.build()?;

	while let Some(next_page) = next_page_url.take() {
		let mut list = scrape_community_url(&next_page, &client).await?;

		found.append(&mut list.contributions);

		println!("Found: {}", found.len());

		next_page_url = list.next_page_path;

		tokio::time::sleep(Duration::from_millis(500)).await;
	}

	// Too lazy to stream data to file.
	save_contributions_file(&found).await?;

	Ok(found)
}



fn default_headers() -> HeaderMap {
	use reqwest::header::*;

	let mut header_map = HeaderMap::new();

	header_map.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; rv:78.0) Gecko/20100101 Firefox/78.0".parse().unwrap());
	header_map.insert(ACCEPT, " */*".parse().unwrap());
	header_map.insert(ACCEPT_LANGUAGE, "en-US,en;q=0.5".parse().unwrap());
	header_map.insert(ACCEPT_ENCODING, "*".parse().unwrap());
	header_map.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
	header_map.insert(DNT, "1".parse().unwrap());
	header_map.insert(CONNECTION, "keep-alive".parse().unwrap());
	header_map.insert(REFERER, "https://brilliant.org/community/home/problems/popular/all/all/".parse().unwrap());
	header_map.insert("Sec-Fetch-Dest", "empty".parse().unwrap());
	header_map.insert("Sec-Fetch-Mode", "cors".parse().unwrap());
	header_map.insert("Sec-Fetch-Site", "same-origin".parse().unwrap());
	header_map.insert("Pragma", "no-cache".parse().unwrap());
	header_map.insert("TE", "trailers".parse().unwrap());
	header_map.insert("Cache-Control", "no-cache".parse().unwrap());


	header_map
}