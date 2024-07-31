
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncMysqlConnection;

pub type Pool = diesel_async::pooled_connection::deadpool::Pool<AsyncMysqlConnection>;

/// Set up connection pool
pub fn init() -> Pool {
    
    let config = AsyncDieselConnectionManager::<AsyncMysqlConnection>::new(std::env::var("DATABASE_URL")
        .expect("DATABASE_URL is not defined"));
    
    Pool::builder(config)
        .build()
        .expect("Failed to create database pool conections")
}

// TODO:
// Write a custom axum extractor that grabs a connection from the pool


