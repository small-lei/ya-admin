use axum::Server;
use std::net::SocketAddr;
use std::sync::Arc;
use crate::api::{api_routes, api_routes_with_auth};
use crate::db::init_db;
use crate::middleware::auth::AuthContext;

mod api;
mod db;
mod model;
mod error;
mod middleware;
mod config;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    println!("Attempting to connect to database...");
    let db = match init_db().await {
        Ok(db) => {
            println!("✅ Database connection successful");
            db
        },
        Err(e) => {
            println!("❌ Database connection failed: {}", e);
            println!("Please check your database credentials in the .env file or make sure your MySQL server is running.");
            println!("Current DATABASE_URL: {}", std::env::var("DATABASE_URL").unwrap_or_else(|_| "Not set".to_string()));
            return;
        }
    };
    
    // Create a default auth context for test purposes
    // In a real application, this would be created from token verification
    let auth_context = Arc::new(AuthContext {
        user_id: 1, // Default user ID for testing
    });

    // Create two separate app instances with different state types
    let auth_app = api_routes().with_state(db.clone());
    let orders_app = api_routes_with_auth(db, auth_context);
    
    // Merge the two app instances
    let app = auth_app.merge(orders_app);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
