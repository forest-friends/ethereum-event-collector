use crate::services::eth::Transport;
use crate::utils::{Error, Service};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Notify;
use tokio::time::{sleep, Duration};
use web3::Web3;

pub struct Collector {
    eth: Arc<Web3<Transport>>,
    quit: Notify,
}

impl Collector {
    pub fn new(eth: Arc<Web3<Transport>>) -> Self {
        Self {
            quit: Notify::new(),
            eth,
        }
    }
    pub async fn run(&self) -> Result<(), Error> {
        loop {
            tokio::select! {
                _ = async {
                    self.collect_logs().await;
                    sleep(Duration::from_secs(10)).await;
                } => {}
                _ = self.quit.notified() => {
                    break;
                }
            }
        }

        Ok(())
    }

    async fn collect_logs(&self) -> Result<(), Error> {
        println!("1");

        Ok(())
    }
}

#[async_trait]
impl Service for Collector {
    async fn stop(&self) -> Result<(), Error> {
        self.quit.notify_waiters();
        Ok(())
    }
}
