mod models;
mod routes;
mod auth;

use axum::{
    routing::{get, post, put},
    Router,
};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    str::FromStr,
};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use routes::{
    admin_auth::{login_admin, logout_admin, signup_admin},
    create_poll::create_poll,
    delete_poll::delete_poll,
    get_poll::get_poll,
    list_events::list_events,
    submit_vote::submit_vote,
    update_votes::update_votes,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file if present
    dotenvy::dotenv().ok();

    // Set up tracing
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./poll.db".to_string());

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let (sqlite_path, sqlite_existed) = match sqlite_path_from_url(&database_url) {
        Some(path) => {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let existed = path.exists();
            (Some(path), existed)
        }
        None => (None, true),
    };

    // Set up the database pool
    let options = SqliteConnectOptions::from_str(&database_url)?.create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    if let (Some(path), false) = (sqlite_path, sqlite_existed) {
        tracing::info!("Created new database at {}", path.display());
    }
    tracing::info!("Database ready");

    // CORS — allow all origins for local dev; tighten this for production
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/admin/signup", post(signup_admin))
        .route("/api/admin/login", post(login_admin))
        .route("/api/admin/logout", post(logout_admin))
        .route("/api/poll", post(create_poll))
        .route("/api/poll/:id", get(get_poll).delete(delete_poll))
        .route("/api/poll/:id/vote", post(submit_vote))
        .route("/api/poll/:id/participant/:participant_id", put(update_votes))
        .route("/api/events", get(list_events))
        .layer(cors)
        .with_state(pool);

    let addr: SocketAddr = format!("{host}:{port}").parse()?;
    tracing::info!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn sqlite_path_from_url(database_url: &str) -> Option<PathBuf> {
    let rest = database_url.strip_prefix("sqlite:")?;
    let path = rest.trim_start_matches("//");
    if path.is_empty() {
        None
    } else {
        Some(Path::new(path).to_path_buf())
    }
}
