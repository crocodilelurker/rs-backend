use std::env;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use actix_web::middleware::Logger;

#[get("/")]
async fn default_route() -> impl Responder{
    HttpResponse::Ok().body("Welcome to the Actix Web Backend!")
}
#[post("/submit")]
async fn submit_data(req_body: String) -> impl Responder {
    println!("Received data: {}", req_body);
    if(req_body.is_empty()) {
        return HttpResponse::BadRequest().body("No data provided!");
    }
    HttpResponse::Ok().body("Data received successfully!")
}
async fn api_route() -> impl Responder {
   HttpResponse::Ok().body("Hello World from the API!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err()
    {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }
    dotenv::dotenv().ok();
    env_logger::init();


    HttpServer::new(|| {
        App::new()
        .wrap(Logger::default())
            .service(default_route)
            .service(submit_data)
            .service(
                web::scope("/api")
                .route("index.html", web::get().to(api_route))
                .route("/", web::get().to(api_route))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}