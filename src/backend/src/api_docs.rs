use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa::{Modify, ToSchema};

use crate::routes;

use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Req {
    pub username: String,
    pub password: String,
}

struct BearerAuthAddon;

impl Modify for BearerAuthAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    components(schemas(Req)),
    security(("bearer_auth" = [])),
    paths(

        routes::debug::health::health,
        routes::debug::auth::auth,

        routes::auth::signup::signup,
        routes::auth::login::login,

        routes::subicron::post_subicron::post_subicron,
        routes::subicron::get_subicron::get_subicron,
        routes::subicron::get_subicron_id::get_subicron_id,
        
        routes::subicron::posts::post_subicron_id_posts::post_subicron_id_posts,
        routes::subicron::posts::get_subicron_id_posts::get_subicron_id_posts,
        routes::subicron::posts::get_subicron_id_posts_id::get_subicron_id_posts_id,
        routes::subicron::posts::get_subicron_id_posts_id_upvote::get_subicron_id_posts_id_upvote,
        routes::subicron::posts::delete_subicron_id_posts_id_upvote::delete_subicron_id_posts_id_upvote
    ),
    modifiers(&BearerAuthAddon),
    // tags(
    //     (name = "Auth", description = "Authentication endpoints"),
    //     (name = "Users", description = "User management endpoints")
    // ),
    security(
        ("bearer_auth" = [])
    )
)]
pub struct ApiDoc;