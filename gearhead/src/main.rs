use std::net::Ipv4Addr;
use sqlite::State;
use axum::{
    extract::{FromRef, Path},
    response::IntoResponse,
    routing::get,
    serve, Router,
};
use axum_template::{engine::Engine, Key, RenderHtml};
use minijinja::Environment;
use serde::{Serializei, Deserialize};
use tokio::net::TcpListener;
type AppEngine = Engine<Environment<'static>>;
#[derive(Debug, Serialize)]
pub struct Person {
    name: String,
}
async fn get_name(
    // Obtain the engine
    engine: AppEngine,
    // Extract the key
    Key(key): Key,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let person = Person { name };
    RenderHtml(key, engine, person)
}
#[derive(Clone, FromRef)]
struct AppState {
    engine: AppEngine,
}
#[tokio::main]
async fn main() {
    let mut jinja = Environment::new();
    jinja
        .add_template("/:name", "<h1>Hello</h1><p>{{name}}</p>")
        .unwrap();
    let app = Router::new()
        .route("/:name", get(get_name))
        // Create the application state
        .with_state(AppState {
            engine: Engine::from(jinja),
        });

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 1118))
        .await
        .unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}
