use traceserver::*;
use axum::{routing::{get, post}, Json, Router};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();
  let app = Router::new()
    .route("/health", get(|| async { "ok" }))
    .route("/ingest", post(ingest));
  let listener = tokio::net::TcpListener::bind(("127.0.0.1", 8787)).await.unwrap();
  tracing::info!("TraceServer listening on 127.0.0.1:8787");
  axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Serialize, serde::Deserialize)]
struct IngestReq{ lines: Vec<trace::TraceEnvelope> }

async fn ingest(Json(req): Json<IngestReq>) -> Json<serde_json::Value> {
  for env in &req.lines { let _ = validator::validate(env); }
  Json(serde_json::json!({ "ok": true, "n": req.lines.len() }))
}
