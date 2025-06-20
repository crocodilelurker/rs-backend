use std::env;
use actix_web::{ App, HttpServer};
use actix_web::middleware::Logger;

mod routes;
mod utils;


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
    let port: u16=(*utils::constants::PORT).clone();
    let address: String=(*utils::constants::ADDRESS).clone();


    HttpServer::new(|| {
        App::new()
        .wrap(Logger::default())
        .configure(routes::home_routes::config)
    })
    .bind((address , port))?
    .run()
    .await
}