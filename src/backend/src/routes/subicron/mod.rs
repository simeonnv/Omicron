use actix_web::{dev::{ServiceFactory, ServiceRequest, ServiceResponse}, Error, web, Scope};
use crate::libs::auth::auth_middleware::AuthMiddleware;

pub mod post_subicron;
pub mod get_subicron;
pub mod post_subicron_id_posts;
pub mod get_subicron_id_posts;
pub mod get_subicron_id;


pub fn subicron() -> Scope<impl ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse, Error = Error, InitError = ()>> {
    web::scope("/subicron")
        .wrap(AuthMiddleware)
        .service(post_subicron::post_subicron)
        .service(get_subicron::get_subicron)
        .service(get_subicron_id::get_subicron_id)
}
