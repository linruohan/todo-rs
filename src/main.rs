use axum::Router;
use axum::routing::{delete, get, post};
use diesel::r2d2::ConnectionManager;
use diesel::{SqliteConnection, r2d2};
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use tokio::signal;

mod handlers;
mod models;
mod schema;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create pool.");
    let db_connection = Arc::new(pool);

    let app = Router::new()
        .route("/todos", post(handlers::create_todo))
        .route("/todos", get(handlers::get_todos))
        .route("/todos/{id}", get(handlers::get_todo))
        .route("/todos/{id}", post(handlers::update_todo))
        .route("/todos/{id}", delete(handlers::delete_todo))
        .with_state(db_connection.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5002")
        .await
        .unwrap();
    let server = axum::serve(listener, app).with_graceful_shutdown(shutdown_signal());

    tokio::spawn(async move {
        println!("Server is running: http://127.0.0.1:5002");
    });

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
