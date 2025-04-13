use quests_tracker::config::config_loader;
use tracing::{error, info};



#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let _dot_envy = match config_loader::load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load ENV: {}", e);
            std::process::exit(1);
        }
    };

    info!("Env has been loaded");
}
