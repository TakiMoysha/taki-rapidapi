mod types {
    use serde::Deserialize;

    pub const BIND_ADDR: &str = "0.0.0.0:8080";

    #[derive(Deserialize, Debug)]
    pub struct StreamingContext {
        pub ip_addr: String,
    }

    impl StreamingContext {
        pub fn from_json(input: &str) -> Self {
            Self {
                ..Default::default()
            }
        }
    }

    impl Default for StreamingContext {
        fn default() -> Self {
            Self {
                ip_addr: String::from(BIND_ADDR),
            }
        }
    }
}

mod server {
    use log::{error, info, warn};
    use std::env;

    use axum::{extract::Extension, response::Json, routing::get, Router};
    use tokio::{fs, signal};

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

    pub async fn load_config() -> String {
        let config_path = env::current_dir()
            .map(|p| p.join("config.json"))
            .expect("Couldn't get current dir");
        let config = fs::read(config_path)
            .await
            .expect("Couldn't read server config");
        let config = serde_json::from_slice(&config).expect("bad file format, deserialize error");
        info!("service config loaded");
        config
    }
}

use log::{error, info, set_max_level, LevelFilter};

use std::process;
use std::sync::Arc;
use types::StreamingContext;

mod logging;
mod demo_futures;

#[tokio::main]
async fn main() {
    let _ = logging::setup_logging();
    // let config = server::load_config().await;
    // let ctx = Arc::new(StreamingContext::from_json(&config));
    //
    // let PID = process::id();
    // let ip_addr = ctx.ip_addr.clone();
    // log::info!("Server started as {PID} process, at {ip_addr}");
}
