use crate::utils::Error;
use tokio::signal::unix::{signal, SignalKind};

pub async fn graceful_shutdown() -> Result<&'static str, Error> {
    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigquit = signal(SignalKind::quit())?;

    Ok(tokio::select! {
        _ = sigterm.recv() => { "terminating application" }
        _ = sigint.recv() => { "interrupt application" }
        _ = sigquit.recv() => { "quit application" }
    })
}