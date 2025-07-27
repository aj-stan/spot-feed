use crate::db::users::queries::*;

use deadpool_postgres::Pool;
use ntex::web::{self, HttpResponse, ServiceConfig, types::Json};
use serde::Deserialize;

// ---- Request structs ----
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct VerifyOtpRequest {
    pub email: String,
    pub otp: String,
}

#[derive(Deserialize)]
pub struct GuestRequest {
    pub device_id: Option<String>,
}

// ---- Handler functions ----
pub async fn register(
    pool: web::types::State<Pool>,
    Json(req): Json<RegisterRequest>,
) -> HttpResponse {
    match create_user(&pool, &req.username, &req.email, &req.password).await {
        Ok(user) => HttpResponse::Ok().json(&user),
        Err(e) => HttpResponse::Conflict().body(format!("Registration failed: {e}")),
    }
}

pub async fn login(pool: web::types::State<Pool>, Json(req): Json<LoginRequest>) -> HttpResponse {
    match find_user_by_username(&pool, &req.username).await {
        Ok(user) => {
            if user.password_hash == req.password {
                // TODO: generate token/session
                HttpResponse::Ok().json(&serde_json::json!({
                    "user_id": user.id,
                    "message": "Login successful"
                }))
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("User not found"),
    }
}

pub async fn verify_otp(Json(req): Json<VerifyOtpRequest>) -> HttpResponse {
    // TODO: Plug in OTP verification logic
    HttpResponse::Ok().body(format!("Stub: OTP for {}", req.email))
}

pub async fn guest(Json(req): Json<GuestRequest>) -> HttpResponse {
    // TODO: Plug in guest session logic
    HttpResponse::Ok().body(format!("Stub: Guest login {:?}", req.device_id))
}

// ---- Route registration ----
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/verify-otp", web::post().to(verify_otp))
            .route("/guest", web::post().to(guest)),
    );
}
