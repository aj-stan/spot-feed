use crate::db::users::queries::create_user;
use deadpool_postgres::Pool;
use ntex::web::{self, HttpResponse, ServiceConfig, types::Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn register(
    pool: web::types::State<Pool>,
    Json(req): Json<RegisterRequest>,
) -> HttpResponse {
    match create_user(&pool, &req.username, &req.email, &req.password).await {
        Ok(user) => HttpResponse::Ok().json(&user),
        Err(e) => HttpResponse::Conflict().body(format!("Registration failed: {e}")),
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/register").route(web::post().to(register)));
}
