use log::{error, info, warn};
use std::env;

use axum::{extract::Extension, response::Json, routing::get, Router};
use tokio::{fs, signal};

use crate::types::StreamingContext;

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "healty"}))
}

// fn make_service_router(state: Arc<StreamingContext>) -> Router {
//     Router::new()
//         .route("/api/v1/health", get(health_check()))
//         .layer(Extension(app_state))
// }

// pub async fn run_axum_server(
//     app_state: Arc<StreamingContext>,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let listener = tokio::net::TcpListener::bind(BIND_ADDR).await;
//     listener.is_err() {
//
//     }
//     info!("listening on {}", listener.local_addr().unwrap());
//     axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await?;
//     Ok(())
// }

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install handler")
            .recv()
            .await;
    };

    // #[cfg(not(unix))]
    // let terminate = std::future::pending()::<()>();

    // tokio::select! {
    //     _ = ctrl_c => {},
    //     _ = terminate => {}
    // }
}

pub async fn load_config() -> StreamingContext {
    let config_path = env::current_dir()
        .map(|p| p.join("config.json"))
        .expect("Couldn't get current dir");
    let config = fs::read(config_path)
        .await
        .expect("Couldn't read server config");
    let config: StreamingContext =
        serde_json::from_slice(&config).expect("bad file format, deserialize error");
    info!("service config loaded");
    config
}
