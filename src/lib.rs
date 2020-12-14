use actix_web::{web, App, HttpServer};
use api::{create_user_handler, create_users_handler, get_user_handler};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use std::env;
use tokio_postgres::{Config, NoTls};

mod api;
mod db;
mod model;
mod repository;
mod service;

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut pg_config = Config::new();
    pg_config.host(&env::var("DB_HOST").unwrap());
    pg_config.user(&env::var("DB_USR").unwrap());
    pg_config.password(&env::var("DB_PW").unwrap());
    pg_config.dbname(&env::var("DB_DBN").unwrap());
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    let pool = Pool::new(mgr, 16);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/user/{username}", web::get().to(get_user_handler))
            .route("/user/{username}", web::post().to(create_user_handler))
            .route("/users/{amount}", web::post().to(create_users_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
