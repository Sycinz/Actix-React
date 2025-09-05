use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files::{NamedFile};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = 8080;

    println!("Serving React on: http://localhost:{}", port);
    HttpServer::new(|| {
        App::new()
            .service(actix_files::Files::new("/", "../frontend/dist").index_file("index.html"))
    })
    .bind(("localhost", port))?
    .run()
    .await
}