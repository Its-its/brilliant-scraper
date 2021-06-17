use serde::{Serialize, Deserialize};
use scraper_macros::Scraper;
use scraper_main::{ConvertToValue, ScraperMain, xpather::parse_document};
use std::io::Cursor;

use crate::correct_url;


#[derive(Debug, Scraper)]
pub struct CommunityList {
	#[scrape(xpath = r#"//div[contains(@class, "nf-feed-item-wrapper")]"#)]
	pub contributions: Vec<CommunityListContribution>,

	#[scrape(xpath = r#"//a/@data-next_url"#)]
	#[scrape(transform = "transform_next_page_url")]
	pub next_page_path: Option<String>
}


#[derive(Debug, Scraper, Serialize, Deserialize)]
pub struct CommunityListContribution {
	#[scrape(xpath = r#"./div/a/span/text()"#)]
	#[scrape(transform = "transform_title")]
	pub title: String,

	// My Xpath Parser currently doesn't support OR. So i'm just seperating it into two and combining.
	#[scrape(xpath = r#"./div/a/text()"#)]
	#[scrape(transform = "transform_title2")]
	#[serde(skip)]
	pub _title_alt: Vec<String>,

	#[scrape(xpath = r#"./div/span[1]/text()"#)]
	#[scrape(transform = "transform_trim")]
	pub topic: String,

	#[scrape(xpath = r#"./div/span[2]/text()"#)]
	#[scrape(transform = "transform_trim_opt")]
	pub popularity: Option<String>,

	// My Xpath Parser currently doesn't support OR. So i'm just seperating it into two and combining.
	#[scrape(xpath = r#"./div/span[2]/span/text()"#)]
	#[scrape(transform = "transform_trim_opt")]
	#[serde(skip)]
	pub _pop_alt: Option<String>,

	#[scrape(xpath = r#"./div/span[3]/text()"#)]
	#[scrape(transform = "transform_trim_opt")]
	pub difficulty: Option<String>,

	#[scrape(xpath = r#"./div/a/@href"#)]
	#[scrape(transform = "transform_url")]
	pub url: String
}

impl CommunityListContribution {
	pub async fn scrape_problem(&self) -> Result<ContributionProblem, Box<dyn std::error::Error>> {
		let resp = reqwest::get(&self.url).await?.text().await?;

		let doc = parse_document(&mut Cursor::new(&resp))?;

		let mut program = ContributionProblem::scrape(&doc, None)?;
		program.html = resp;

		Ok(program)
	}
}


#[derive(Debug, Scraper, Serialize, Deserialize)]
pub struct ContributionProblem {
	#[scrape(ignore)]
	pub html: String,

	#[scrape(xpath = r#"//link[@rel="stylesheet"]/@href"#)]
	#[scrape(transform = "transform_styles")]
	pub styles: Vec<String>,

	#[scrape(xpath = r#"//img/@src"#)]
	pub images: Vec<String>
}


// Transforms

fn transform_styles(value: Vec<String>) -> Vec<String> {
	value.into_iter().map(correct_url).collect()
}



fn transform_title(value: Option<String>) -> String {
	value.map(|v| v.trim().to_string()).unwrap_or_default()
}

fn transform_title2(value: Vec<String>) -> Vec<String> {
	value.into_iter()
		.filter_map(|v| {
			let value = v.trim().to_string();

			if value.is_empty() {
				None
			} else {
				Some(value)
			}
		})
		.collect()
}

fn transform_trim_opt(value: Option<String>) -> Option<String> {
	value.map(|v| v.trim().to_string())
}

fn transform_trim(value: String) -> String {
	value.trim().to_string()
}

fn transform_url(value: String) -> String {
	complete_url(&value)
}

fn transform_next_page_url(paths: Option<String>) -> Option<String> {
	paths.map(|v| complete_url(&v) + "&page_key=community_portal_problems&filter_content_type=&reviewed_state=&version=1&deferred=false")
}


fn complete_url(path: &str) -> String {
	String::from("https://brilliant.org") + path
}