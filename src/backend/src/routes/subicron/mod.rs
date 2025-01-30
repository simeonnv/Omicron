use actix_web::{dev::{ServiceFactory, ServiceRequest, ServiceResponse}, Error, web, Scope};
use crate::libs::auth::auth_middleware::AuthMiddleware;

pub mod post_subicron;
pub mod get_subicron;
pub mod get_subicron_id;
pub mod posts;



pub fn subicron() -> Scope<impl ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse, Error = Error, InitError = ()>> {
    web::scope("/subicron")
        .wrap(AuthMiddleware)
        .service(post_subicron::post_subicron)
        .service(get_subicron::get_subicron)
        .service(get_subicron_id::get_subicron_id)
        
        //posts
        .service(posts::post_subicron_id_posts::post_subicron_id_posts)
        .service(posts::get_subicron_id_posts::get_subicron_id_posts)
        .service(posts::get_subicron_id_posts_id::get_subicron_id_posts_id)
        .service(posts::get_subicron_id_posts_id_upvote::get_subicron_id_posts_id_upvote)
        .service(posts::delete_subicron_id_posts_id_upvote::delete_subicron_id_posts_id_upvote)
}
