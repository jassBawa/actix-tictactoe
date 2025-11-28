use std::{env, io::Result};

use actix_web::{App, HttpResponse, HttpServer, web};
use db::pool::DbPool;

use dotenvy::dotenv;
use tic_tac::routes::{self};
use ws::manager;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = DbPool::new(&database_url)
        .await
        .expect("Failed to create DB pool");

    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

    let ws_manager = web::Data::new(
        manager::start_manager(&redis_url)
            .await
            .expect("Failed to create ws manager"),
    );

    let pool_data = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .app_data(ws_manager.clone())
            .configure(routes::auth::config)
            .configure(routes::game::config)
            .configure(|cfg| routes::websocket::config(cfg, ws_manager.clone()))
            .route(
                "/ping",
                web::get().to(|| async { HttpResponse::Ok().body("pong") }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
