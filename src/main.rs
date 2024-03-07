use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).context("error setting global default subscriber")?;
    Ok(())
}
