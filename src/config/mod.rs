use serde::{Deserialize};
use crate::utils::LogConfig;

mod eth;
pub use eth::*;
use slog::{KV, Record, Serializer};

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub log: LogConfig,
    pub eth: eth::EthConfig,
}

impl KV for AppConfig {
    fn serialize(&self, record: &Record, serializer: &mut dyn Serializer) -> slog::Result {
        self.log.serialize(record, serializer)?;
        self.eth.serialize(record, serializer)?;
        Ok(())
    }
}