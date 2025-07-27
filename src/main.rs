mod api;
mod config;
mod db;

use config::Config;
use ntex::web::{self, App, HttpServer};

#[web::get("/ping")]
async fn ping() -> impl web::Responder {
    web::HttpResponse::Ok().body("pong")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    let pool = config.make_pg_pool();

    println!(
        "🚀 Server starting at http://{}:{}",
        config.bind_ip, config.bind_port
    );
    HttpServer::new(move || {
        App::new()
            .state(pool.clone())
            .service(ping)
            .configure(api::auth::config)
            .configure(api::users::config)
    })
    .bind((config.bind_ip.as_str(), config.bind_port))?
    .run()
    .await
}
