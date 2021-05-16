use crate::config::AppConfig;
use crate::utils::{Error, new_logger, Service};
use slog::{info, Logger};
use crate::services::eth::{new_web3, Transport};
use crate::services::collector::Collector;
use crate::utils::graceful_shutdown;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

pub struct App {
    logger: Logger,
    collector: Arc<Collector>
}

impl App {
    pub async fn new(config: &AppConfig) -> Result<Self, Error> {
        let logger = new_logger(&config.log)?;

        info!(logger, "starting application"; config);

        let eth = new_web3(config.eth.endpoint.as_str()).await?;

        Ok(Self {
            logger: logger.clone(),
            collector: Arc::new(Collector::new())
        })
    }

    pub async fn run(&self) -> Result<(), Error> {
        let collector = self.collector.clone();
        // let collector2 = self.collector.clone();

        tokio::select! {
            reason = tokio::spawn(async {
                graceful_shutdown().await
            }) => {
                info!(self.logger, "graceful shutdown"; "reason" => reason??);
                self.collector.stop().await
            }
            res = tokio::spawn(async move {
                collector.run().await
            }) => {
                info!(self.logger, "collector shutdown");
                res?
            }
        };

        let collector2 = self.collector.clone();

        Ok(())
    }
}