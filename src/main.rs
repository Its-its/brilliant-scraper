use std::{error::Error, time::Duration};

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
	if std::env::args().any(|v| &v == "web") {
		web::start().await?;
	}

	if std::env::args().any(|v| &v == "scrape") {
		println!("Executing Scraper in 5 seconds. If this is an error Ctrl-C now.");
		tokio::time::sleep(Duration::from_secs(5)).await;

		println!("Executing Step 1: Grabbing ALL Contribution URLS and caching.");

		let contributions = step_1_grab_all_contributions().await?;

		println!("Executing Step 2: Scraping all contributions and placing them in archive folder.");

		step_2_scrape_contributions(contributions).await?;

		return Ok(());
	}

	panic!(r#"Please specify either "web" or "scrape" by doing "./executableName scrape""#);
}


async fn step_1_grab_all_contributions() -> Result<Vec<CommunityListContribution>, Box<dyn std::error::Error>> {
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


	let mut found: Vec<CommunityListContribution> = if does_contributions_file_exist().await {
		println!(" - Contributions File already exists. Using it and checking if we're missing any problem pages.");

		let value = read_contributions_file().await?;

		// Args Param to force cached without ensuring we have all data.
		if std::env::args().any(|v| &v == "no-cache-check") {
			return Ok(value);
		}

		value
	} else {
		Vec::new()
	};


	let client = Client::builder()
		.default_headers(default_headers())
		.cookie_store(true)
		.build()?;

	'base: for (i, problem_list_url) in problem_urls.into_iter().enumerate() {
		let mut should_skip_checked = false;

		let mut next_page_url = Some(problem_list_url.to_string());

		while let Some(next_page) = next_page_url.take() {
			let mut list = scrape_community_url(&next_page, &client).await?;

			// Correct contributions (my xpath parser doesn't support OR yet.) Mainly used for Discussions/Need Solutions
			list.contributions.iter_mut()
			.for_each(|c| {
				if c.title.is_empty() && !c._title_alt.is_empty() {
					c.title = c._title_alt.join("");
					c._title_alt.clear();
				}

				// Exists and not empty.
				if c._pop_alt.as_ref().map(|v| !v.is_empty()).unwrap_or_default() {
					c.popularity = c._pop_alt.take();
				}
			});

			// (i) Skip the check for "Need Solution"
			if !should_skip_checked && i != 2 {
				// If our cache already contains one of the URLs break out of the while loop and continue.
				if let Some(found_cont) = list.contributions.first() {
					if let Some((index, found)) = found.iter().enumerate().find(|(_, v)| v.url == found_cont.url) {
						println!(" - - Already cached: {:?}\n\t\t => [{}]: {:?}", next_page, index, found.url);
						continue 'base;
					}

					should_skip_checked = true;
				}
			}

			// "Need Solution" Popularity/Difficulty mix-up fix.
			if i == 2 {
				list.contributions.iter_mut()
				.for_each(|v| {
					v.difficulty = v.popularity.take();
				});

				// Need Solution returns some which are in New/Popular so we're going to check each one of these just in-case.
				// Intensive? I know.
				for item in list.contributions {
					if !found.iter().any(|v| v.url == item.url) {
						found.push(item);
					}
				}
			} else {
				found.append(&mut list.contributions);
			}

			println!("Found: {} -- {:?}", found.len(), list.next_page_path);

			next_page_url = list.next_page_path;

			tokio::time::sleep(Duration::from_millis(500)).await;
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

		println!("Archiving: {}", contribution.url);

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
				match reqwest::get(&image_url).await {
					Ok(resp) => {
						let data = resp.bytes().await?;

						// Replace external image with downloaded one.
						if let Some(local_path) = save_data_to_directory(&image_url, &data).await? {
							problem.html = problem.html.replace(&image_path, &local_path);
						}
					}
					Err(e) => {
						eprintln!("\t Error Occurred While trying to request {:?}\n\t\t {:?}", image_url, e.source());
					}
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

			// Show hidden comments in discussion threads since we don't download the Javascript.
			if url.contains("discussions/thread") {
				problem.html = problem.html.replace(r#"hide" data-level"#, r#"" data-level"#);
			}

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