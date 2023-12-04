use sqlite::State;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use axum_template::{engine::Engine, Key, RenderHtml};
use minijinja::Environment;

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt::init();
	let app = Router::new()
        .route("/", get(root))
        .route("/metrics", get(metrics));




}
