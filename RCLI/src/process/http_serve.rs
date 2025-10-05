use crate::HttpSubCommand;
use axum::Router;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::log;

#[derive(Debug)]
struct HttpServerState {
    path: PathBuf,
}

pub async fn process_http_serve(cmd: HttpSubCommand) -> anyhow::Result<()> {
    match cmd {
        HttpSubCommand::Serve(opts) => {
            log::info!("Serving HTTP {:?}", opts);

            let state = HttpServerState { path: opts.dir };

            let router = Router::new()
                .route("/test/{*path}", get(file_handler))
                .route("/hello", get(hello))
                .nest_service("/static", ServeDir::new(".").append_index_html_on_directories(true))
                .with_state(Arc::new(state));
            let addr = SocketAddr::from(([0, 0, 0, 0], opts.port));
            let listener = TcpListener::bind(addr).await?;
            axum::serve(listener, router).await?;
        }
    }
    Ok(())
}

async fn hello() -> String {
    "hello".to_string()
}

async fn file_handler(
    State(state): State<Arc<HttpServerState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("Path {} does not exist.", p.display()),
        )
    } else {
        match tokio::fs::read_to_string(&p).await {
            Ok(content) => (StatusCode::OK, content),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        }
    }
}
