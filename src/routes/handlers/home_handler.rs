use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
pub async fn default_route() -> impl Responder{
    HttpResponse::Ok().body("Welcome to the Actix Web Backend!")
}
#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    let greeting = format!("Hello, {}!", name);
    HttpResponse::Ok().body(greeting)
}