use scraper_macros::Scraper;
use scraper_main::ConvertFromValue;


#[derive(Debug, Scraper)]
pub struct CommunityList {
    #[scrape(xpath = r#"//div[contains(@class, "nf-feed-item-wrapper")]"#)]
	pub contributions: Vec<CommunityContribution>,

	#[scrape(xpath = r#"//a/@data-next_url"#)]
	#[scrape(transform = "transform_next_page_url")]
	pub next_page_path: Option<String>
}


#[derive(Debug, Scraper)]
pub struct CommunityContribution {
	#[scrape(xpath = r#"./div/a/span/text()"#)]
	#[scrape(transform = "transform_trim")]
	title: String,

	#[scrape(xpath = r#"./div/span[1]/text()"#)]
	#[scrape(transform = "transform_trim")]
	topic: String,

	#[scrape(xpath = r#"./div/span[2]/text()"#)]
	#[scrape(transform = "transform_trim")]
	popularity: String,

	#[scrape(xpath = r#"./div/span[3]/text()"#)]
	#[scrape(transform = "transform_trim")]
	difficulty: String,

	#[scrape(xpath = r#"./div/a/@href"#)]
	#[scrape(transform = "transform_str")]
	url: String
}



// Transforms

fn transform_trim(value: String) -> String {
	value.trim().to_string()
}

fn transform_str(value: String) -> String {
	complete_url(&value)
}

fn transform_next_page_url(paths: Option<String>) -> Option<String> {
	paths.map(|v| complete_url(&v) + "&page_key=community_portal_problems&filter_content_type=&reviewed_state=&version=1&deferred=false")
}

fn complete_url(path: &str) -> String{
	String::from("https://brilliant.org") + path
}