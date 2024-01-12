use std::sync::Arc;
use tokio::sync::RwLock;
use log::{info, error};
use axum::{
    response::Html,
    routing::{get, get_service},
    Router
};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
mod site;
mod update;

async fn health() -> Html<String> {
    Html(String::from("OK"))
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Cloud {
    is_down: bool,
    down_since: String,
}

#[derive(Clone)]
pub struct SiteState {
    cloud: Cloud,
    last_updated: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting up!");

    let state = Arc::new(RwLock::new(SiteState {
        last_updated: String::from(""),
        cloud: Cloud {
            is_down: false,
            down_since: String::from(""),
        },
    }));

    let cloned_state = Arc::clone(&state);
    tokio::spawn(async move {
        loop {
            if let Err(e) = update::update(cloned_state.clone()).await {
                error!("Error updating: {}", e);
            };
            // wait 1 min
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    });

    let app = Router::new()
        .nest_service("/assets", get_service(ServeDir::new("./assets")))
        .route("/health", get(health))
        .route("/", get(site::home::home))
        .route("/landing", get(site::landing::landing))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().expect("Invalid address"))
        .serve(app.into_make_service())
        .await
        .expect("Server failed");
}

