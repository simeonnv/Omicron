use actix_web::{dev::{ServiceFactory, ServiceRequest, ServiceResponse}, Error, web, Scope};
use crate::libs::auth::auth_middleware::AuthMiddleware;

pub mod post_subicron;


pub fn subicron() -> Scope<impl ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse, Error = Error, InitError = ()>> {
    web::scope("/subicron")
        .wrap(AuthMiddleware)
        .service(post_subicron::post_subicron)
}
