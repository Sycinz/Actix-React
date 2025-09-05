use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files::{NamedFile};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Serving React on: http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .service(actix_files::Files::new("/", "../frontend/dist").index_file("index.html"))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}