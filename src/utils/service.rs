use async_trait::async_trait;
use crate::utils::Error;
use tokio::signal::unix::{signal, SignalKind};

#[async_trait]
pub trait Service {
    async fn stop(&self) -> Result<(), Error>;
}