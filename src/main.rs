use std::time::Duration;

use reqwest::{Client, header::HeaderMap};


mod requesting;
mod scraping;

use requesting::*;
use scraping::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("Hello, world!");

	let mut found: Vec<CommunityContribution> = Vec::new();

	let mut next_page_path = Some("https://brilliant.org/community/home/problems/popular/all/all/?&deferred=true&page_key=community_portal_problems&version=1".to_string());

	let client = Client::builder()
		.default_headers(default_headers())
		.cookie_store(true)
		.build()?;

	while let Some(next_page) = next_page_path {
		let mut res = scrape_community_url(&next_page, &client).await?;

		found.append(&mut res.contributions);

		println!("Found: {}", found.len());

		next_page_path = res.next_page_path;

		tokio::time::sleep(Duration::from_secs(1)).await;
	}


	Ok(())
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