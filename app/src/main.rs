#[cfg(feature = "server")]
use axum::Json;
#[cfg(feature = "server")]
use axum::response::IntoResponse;

mod app;
mod constants;
mod layout;
mod pages;
mod routes;

use app::App;

#[cfg(feature = "server")]
use constants::FAVICON_PNG;

#[cfg(feature = "server")]
async fn manifest() -> impl IntoResponse {
    Json(serde_json::json!({
        "short_name": "Store",
        "name": "Mango³ Store",
        "description": "Digital distribution platform",
        "lang": "en",
        "dir": "ltr",
        "display": "standalone",
        "orientation": "any",
        "start_url": "/",
        "icons": [
            {
                "src": FAVICON_PNG.to_string(),
                "sizes": "64x64",
                "type": "image/png"
            },
        ],
        "background_color": "#09090b",
        "theme_color": "#ff4c00"
    }))
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use std::net::SocketAddr;

    use axum::routing::get;
    use dioxus::prelude::{DioxusRouterExt, ServeConfig};

    let address = dioxus::cli_config::fullstack_address_or_localhost();
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfig::new(), App)
        .route("/manifest.json", get(manifest));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

#[cfg(not(feature = "server"))]
fn main() {
    sdk::app::launch(App);
}
