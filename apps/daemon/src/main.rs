use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    tracing::info!("AI Rust Programmer daemon starting (local, no-net)...");
    // TODO: bind local JSON-RPC (stdio/unix/tcp disabled by default)
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
