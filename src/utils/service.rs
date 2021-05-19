use crate::utils::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Service {
    async fn stop(&self) -> Result<(), Error>;
}
