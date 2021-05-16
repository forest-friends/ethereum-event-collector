use serde::{Deserialize, Serialize};
use slog::{KV, Record, Serializer};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EthConfig {
    pub endpoint: String
}

impl KV for EthConfig {
    fn serialize(&self, _record: &Record, serializer: &mut dyn Serializer) -> slog::Result {
        serializer.emit_str("EthConfig.endpoint", self.endpoint.as_str())
    }
}