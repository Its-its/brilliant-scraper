use std::time::Duration;

use reqwest::{Client, header::HeaderMap};

mod files;
mod requesting;
mod scraping;
mod web;

use files::*;
use requesting::*;
use scraping::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	match std::env::args().last().as_deref() {
		Some("web") => {
			web::start().await?;
		}

		Some("scrape") => {
			println!("Executing Scraper in 5 seconds. If this is an error Ctrl-C now.");
			tokio::time::sleep(Duration::from_secs(5)).await;

			println!("Executing Step 1: Grabbing ALL Contribution URLS and caching.");

			let contributions = step_1_grab_all_contributions().await?;

			println!("Executing Step 2: Scraping all contributions and placing them in archive folder.");

			step_2_scrape_contributions(contributions).await?;
		}

		_ => panic!(r#"Please specify either "web" or "scrape" by doing "./executableName scrape""#)
	}

	Ok(())
}


async fn step_1_grab_all_contributions() -> Result<Vec<CommunityListContribution>, Box<dyn std::error::Error>> {
	if does_contributions_file_exist().await {
		println!(" - Contributions File already exists. Using it.");
		return read_contributions_file().await;
	}

	let problem_urls = vec![
		// "Popular" Contributions
		"https://brilliant.org/community/home/problems/popular/all/all/?&deferred=true&page_key=community_portal_problems&version=1",
		// "New" Contributions
		"https://brilliant.org/community/home/problems/new/all/all/?&deferred=true&page_key=community_portal_problems&version=1",
		// "Needs Solution" Contributions
		"https://brilliant.org/community/home/need-solution/all/all/?&deferred=true&page_key=community_portal_problems&version=1",
		// "Discussions Popular" Contributions
		"https://brilliant.org/community/home/discussions/popular/all/?&deferred=true&page_key=community_portal_problems&version=1",
		// "Discussions New" Contributions
		"https://brilliant.org/community/home/discussions/new/all/?&deferred=true&page_key=community_portal_problems&version=1"
	];

	let mut found: Vec<CommunityListContribution> = Vec::new();

	let client = Client::builder()
		.default_headers(default_headers())
		.cookie_store(true)
		.build()?;

	for problem_list_url in problem_urls {
		let mut next_page_url = Some(problem_list_url.to_string());

		while let Some(next_page) = next_page_url.take() {
			let mut list = scrape_community_url(&next_page, &client).await?;

			found.append(&mut list.contributions);

			println!("Found: {} -- {:?}", found.len(), list.next_page_path);

			next_page_url = list.next_page_path;

			tokio::time::sleep(Duration::from_millis(1000)).await;
		}

		// Too lazy to stream data to file.
		save_contributions_file(&found).await?;
	}

	Ok(found)
}

async fn step_2_scrape_contributions(contributions: Vec<CommunityListContribution>) -> Result<(), Box<dyn std::error::Error>> {
	create_save_directory().await?;

	for contribution in contributions {
		let mut problem = contribution.scrape_problem().await?;

		println!("{:#?}", problem.images);

		// Save Styles.
		for style_url in problem.styles {
			if !does_data_url_exist(&style_url).await? {
				let data = reqwest::get(&style_url)
					.await?
					.text()
					.await?;

				save_data_to_directory(&style_url, data.as_bytes()).await?;
			}
		}

		// Save Images
		for image_path in problem.images {
			let image_url = correct_url(image_path.clone());

			if !does_data_url_exist(&image_url).await? {
				let data = reqwest::get(&image_url)
					.await?
					.bytes()
					.await?;

				// Replace external image with downloaded one.
				if let Some(local_path) = save_data_to_directory(&image_url, &data).await? {
					problem.html = problem.html.replace(&image_path, &local_path);
				}
			}
		}


		if !does_data_url_exist(&contribution.url).await? {
			let mut url = contribution.url;

			// Fix url so it ends with "".html".
			if url.ends_with('/') {
				url.pop();
				url.push_str(".html");
			}

			// Show hidden comments
			problem.html = problem.html.replace("hide\" data-level", "\" data-level");

			save_data_to_directory(&url, problem.html.as_bytes()).await?;
		}

		tokio::time::sleep(Duration::from_millis(1000)).await;
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

pub fn correct_url(mut value: String) -> String {
	if value.starts_with("//") {
		value.insert_str(0, "https:");
	} else if value.starts_with('/') {
		value.insert_str(0, "https://brilliant.org");
	}

	value
}