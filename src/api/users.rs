use ntex::web::{self, HttpResponse, ServiceConfig};

/// Dummy: Returns current user's profile.
/// In a real app, you'd extract the user ID from the auth/session/JWT.
pub async fn get_me() -> HttpResponse {
    // TODO: Actually fetch the user from the database using session info!
    HttpResponse::Ok().json(&serde_json::json!({
        "id": "placeholder-uuid",
        "username": "aj-stanley",
        "email": "aj@spotfeed.com",
        "is_guest": false,
        "created_at": "2024-01-01T00:00:00"
    }))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/api/users").route("/me", web::get().to(get_me)));
}
