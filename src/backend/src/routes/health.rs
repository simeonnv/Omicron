use actix_web::{get, HttpResponse, Responder};

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "success", body = String, example = "im alive")
    ),
    tag = "Debug"
)]
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("im alive")
}
