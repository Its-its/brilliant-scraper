use serde::{Serialize, Deserialize};
use scraper_macros::Scraper;
use scraper_main::{ConvertFromValue, ScraperMain, xpather::parse_doc};
use std::io::Cursor;


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
	#[scrape(transform = "transform_trim")]
	pub title: String,

	#[scrape(xpath = r#"./div/span[1]/text()"#)]
	#[scrape(transform = "transform_trim")]
	pub topic: String,

	#[scrape(xpath = r#"./div/span[2]/text()"#)]
	#[scrape(transform = "transform_trim")]
	pub popularity: String,

	#[scrape(xpath = r#"./div/span[3]/text()"#)]
	#[scrape(transform = "transform_trim")]
	pub difficulty: String,

	#[scrape(xpath = r#"./div/a/@href"#)]
	#[scrape(transform = "transform_url")]
	pub url: String
}

impl CommunityListContribution {
	pub async fn scrape_problem(&self) -> Result<ContributionProblem, Box<dyn std::error::Error>> {
		let resp = reqwest::get(&self.url).await?.bytes().await?;

		let doc = parse_doc(&mut Cursor::new(&resp));

		let mut program = ContributionProblem::scrape(&doc, None)?;
		program.html = resp.into_iter().collect();

		Ok(program)
	}
}


#[derive(Debug, Scraper, Serialize, Deserialize)]
pub struct ContributionProblem {
	#[scrape(xpath = r#"/justignorethisIdontHaveAnIgnoreForMyScraper"#)]
	#[scrape(transform = "transform_html")]
	pub html: Vec<u8>,

	#[scrape(xpath = r#"//link[@rel="stylesheet"]/@href"#)]
	#[scrape(transform = "transform_styles")]
	pub styles: Vec<String>
}

impl ContributionProblem {
	//
}



// Transforms

fn transform_styles(value: Vec<String>) -> Vec<String> {
	value.into_iter()
		.map(|mut v| {
			if v.starts_with("//") {
				v.insert_str(0, "https:");
			} else if v.starts_with('/') {
				v.insert_str(0, "https://brilliant.org");
			}

			v
		})
		.collect()
}

fn transform_html(_: Option<String>) -> Vec<u8> {
	Vec::new()
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


fn complete_url(path: &str) -> String{
	String::from("https://brilliant.org") + path
}