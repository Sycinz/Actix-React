use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files::{NamedFile};

struct credentials {
    username: String,
    password: String,
}

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

// Credentials handling and listening will be added later
#[post("/login")]
async fn login(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}