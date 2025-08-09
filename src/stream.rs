use tokio::sync::mpsc::Receiver;

use crate::*;

pub struct Stream {
    pub(crate) rx: Receiver<Event>,
}

impl Stream {
    pub(crate) fn new(rx: Receiver<Event>) -> Self {
        Self { rx }
    }

    pub async fn next(&mut self) -> Result<Event> {
        self.rx
            .recv()
            .await
            .ok_or_else(|| eyre::eyre!("Channel closed"))
    }
}
