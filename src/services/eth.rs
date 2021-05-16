use url::{Url};
use crate::utils::Error;

pub type Transport = web3::transports::Either<web3::transports::WebSocket, web3::transports::Http>;

pub async fn new_transport(endpoint: &str) -> Result<Transport, Error> {
    match Url::parse(endpoint)?.scheme() {
        "http" | "https" => Ok(web3::transports::Either::Right(web3::transports::Http::new(endpoint)?)),
        "ws" | "wss" => Ok(web3::transports::Either::Left(web3::transports::WebSocket::new(endpoint).await?)),
        _ => Err(Error::Web3Error(web3::Error::Transport("can't parse scheme".to_string())))
    }
}

pub async fn new_web3(endpoint: &str) -> Result<web3::Web3<Transport>, Error> {
    Ok(web3::Web3::new(new_transport(endpoint).await?))
}