use async_trait::async_trait;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::{oneshot, Mutex};
use crate::utils::{Error, Service};
use std::sync::Arc;
use tokio::sync::oneshot::{Receiver, Sender, channel};
use std::ops::Deref;

pub struct Collector {
    quit: Option<Sender<()>>
}

impl Collector {
    pub fn new() -> Self {
        Self {
            quit: None
        }
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        // self.shutdown().await?;
        let (tx, mut quit) = oneshot::channel::<()>();
        self.quit = Some(tx);
        // rx.close();

        loop {
            tokio::select! {
                _ = quit => {
                    println!("working...");
        // // //             sleep(Duration::from_secs(1));
                    break;
                }
            }
        };

        // while !self.quit.load(Ordering::Acquire) {
        //     println!("working...");
        //     sleep(Duration::from_secs(10));
        // }

        println!("t555");
        Ok(())
    }
}

#[async_trait]
impl Service for Collector {
    async fn stop(&self) -> Result<(), Error> {
        println!("EEEXX");
        // self.quit.store(true, Ordering::Release);
        // self.quit.as_ref().map(|quit| {
        //     println!("{:?}", quit);
        // });
        Ok(())
    }
}

