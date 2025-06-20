use actix_web::{web};
use super::handlers::home_handler::{default_route, greet};

pub fn config(config: &mut web::ServiceConfig) {
    config
           .service(default_route)
           .service(greet);
    
}