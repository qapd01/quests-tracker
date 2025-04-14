use quests_tracker::{config::config_loader, infrastructure::postgres::postgres_connection};
use tracing::{error, info};



#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dot_envy = match config_loader::load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load ENV: {}", e);
            std::process::exit(1);
        }
    };

    info!("Env has been loaded");

    let postgres_pool = match postgres_connection::establish_connection(&dot_envy.database.url) {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to establish connection to Database: {}", e);
            std::process::exit(1);
        }
    };

    info!("Connection to Database has been established");
}
