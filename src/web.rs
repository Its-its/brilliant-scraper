use actix_files::Files;
use actix_web::{App, HttpServer};



pub async fn start() -> std::io::Result<()> {
	println!("Starting Website on 0.0.0.0:6533");

	actix_web::rt::System::new("web::start")
		.block_on(async move {
			HttpServer::new(||
				App::new()
				.service(Files::new("/", "./archive").show_files_listing())
			)
				.bind("0.0.0.0:6533")?
				.run()
				.await
		})
}