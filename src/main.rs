use crate::config::AppConfig;
use crate::services::App;
use crate::utils::Error;

mod config;
mod services;
mod utils;

const DEFAULT_CONFIG_PATH: &str = include_str!("../config/collector.yaml");

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config: AppConfig = crate::utils::load(DEFAULT_CONFIG_PATH)?;
    let app = App::new(&config).await?;
    app.run().await
}
