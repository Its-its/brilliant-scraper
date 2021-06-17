use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, get, rt::System, web::{self, HttpRequest}};

#[get("/")]
async fn index_view() -> actix_web::Result<HttpResponse> {
	Ok(
		HttpResponse::Ok()
		.body(r#"
			<!DOCTYPE html>
			<html lang="en">
				<head>
					<meta charset="UTF-8">
					<meta http-equiv="X-UA-Compatible" content="IE=edge">
					<meta name="viewport" content="width=device-width, initial-scale=1.0">
					<title>Document</title>
				</head>
				<body>
					<ul>
						<li><a href="/problems">Problems</a></li>
						<li><a href="/discussions/thread">Discussions</a></li>
					</ul>
				</body>
			</html>
		"#)
	)
}

async fn html_views(req: HttpRequest) -> actix_web::Result<HttpResponse> {
	let path = format!("./archive{}", req.path());

	let mut content = std::fs::read_to_string(path)?;

	// Inject styling to make comment replies shown.
	content = content.replace("<head>", "<head>\n\t\t<style>.cmmnt-container.hide { display: block !important; }</style>");

	Ok(HttpResponse::Ok().body(content))
}


pub async fn start() -> std::io::Result<()> {
	println!("Starting Website on 127.0.0.1:6533");

	System::new("web::start")
		.block_on(async move {
			HttpServer::new(||
				App::new()
				.service(index_view)
				.route("/problems/{name}.html", web::get().to(html_views))
				.service(Files::new("/", "./archive").show_files_listing())
			)
				.bind("127.0.0.1:6533")?
				.run()
				.await
		})
}