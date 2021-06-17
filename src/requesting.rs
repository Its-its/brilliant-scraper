use std::{collections::HashMap, io::Cursor};

use reqwest::Client;

use serde::{Serialize, Deserialize};
use scraper_main::{ScraperMain, xpather::parse_document};

use crate::scraping::CommunityList;


/// Main Community scraping.
pub async fn scrape_community_url(url: &str, client: &Client) -> Result<CommunityList, Box<dyn std::error::Error>> {
	let recv = client
		.get(url)
		.send()
		.await?;

	let text = recv.text().await?;

	let resp: Response = serde_json::from_str(&text)?;

	let found = resp.actions.values().next().unwrap();

	let doc = parse_document(&mut Cursor::new(&found.new_html))?;

	let list = CommunityList::scrape(&doc, None)?;

	Ok(list)
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	pub actions: HashMap<String, ResponseCommunityPortalProblems>
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseCommunityPortalProblems {
	pub component_key: String,
	pub js_failure_mode: String,
	pub new_html: String,
	pub version: isize
}

