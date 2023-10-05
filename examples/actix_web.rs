use actix_files::Files;
use actix_web::{App, get, HttpServer, middleware::Logger, Responder, Result, web};
use serde::Deserialize;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// extract `Info` using serde
async fn do_post(info: web::Json<Info>) -> Result<String> {
    println!("received request");
    Ok(format!("Welcome {}!", info.username))
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .route("/post", web::post().to(do_post))
            .service(greet)
            .service(Files::new("/web", "./examples/static")
                .show_files_listing()
                .use_last_modified(true)
            )
            .wrap(Logger::default())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
