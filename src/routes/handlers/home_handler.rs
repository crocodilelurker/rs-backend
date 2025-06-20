use actix_web::{get, web, Responder};
use crate::utils::api_response;

#[get("/")]
pub async fn default_route() -> impl Responder{
    api_response::ApiResponse::new(
        200,
        "Welcome to the Actix Web API!".to_string(),
    )

}
#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    let greeting = format!("Hello, {}!", name);
    api_response::ApiResponse::new(
        200,
        greeting,
    )
}