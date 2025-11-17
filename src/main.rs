// Main entry point for the API server
use warp::Filter;
use dotenv::dotenv;
use std::net::SocketAddr;

mod db;
mod models;
mod auth;
mod middleware;
mod handlers;
mod routes;
mod schema;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok(); 

    // Initialize database connection pool
    let pool = db::connection::init_pool();

    // Combine routes
    let routes = routes::users::routes(pool.clone())
        .or(routes::items::routes(pool.clone()))
        .or(routes::cart::routes(pool))
        .with(warp::log("api")); // Add logging

    // Start server
    let addr: SocketAddr = ([127, 0, 0, 1], 3030)
        .into();
    println!("Server is running at http://{}", addr);
    warp::serve(routes).run(addr).await;
}