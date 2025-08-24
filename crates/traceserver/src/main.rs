use axum::Router;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let _ = FmtSubscriber::builder().with_max_level(tracing::Level::INFO).try_init();
    let app: Router = Router::new();
    println!("traceserver boot ok (no network started)");
    let _ = app;
}