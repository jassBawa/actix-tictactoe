use std::{env, io::Result};

use actix_web::{App, HttpResponse, HttpServer, web};
use db::pool::DbPool;

use dotenvy::dotenv;
use tic_tac::routes::{self};

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = DbPool::new(&database_url)
        .await
        .expect("Failed to create DB pool");

    let pool_data = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .configure(routes::auth::config)
            .configure(routes::game::config)
            .route(
                "/ping",
                web::get().to(|| async { HttpResponse::Ok().body("pong") }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
