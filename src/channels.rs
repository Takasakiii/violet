use std::sync::{Arc, Mutex, mpsc::{self, Receiver, Sender}};

use crate::{mysql_db::AppTable, webserver::dtos::EventTrackerReceive};

lazy_static! {
    static ref CHANNEL: Channel = Channel::new();
}

pub type ChannelType = (AppTable, EventTrackerReceive);
pub struct Channel {
    receiver: Arc<Mutex<Receiver<ChannelType>>>,
    sender: Arc<Mutex<Sender<ChannelType>>>
}

impl Channel {
    fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            sender: Arc::new(Mutex::from(sender)),
            receiver: Arc::new(Mutex::from(receiver))
        }
    }

    pub fn get() -> &'static Self {
        &CHANNEL
    }

    pub fn send(&self, data: ChannelType) -> Result<(), crate::GenericError> {
        self.sender
            .lock()
            .map_err(|why| format!("{:?}", why))?
            .send(data)?;
        Ok(())
    }

    pub fn recv(&self) -> Result<ChannelType, crate::GenericError> {
        let rec = self.receiver
            .lock()
            .map_err(|why| format!("{:?}", why))?
            .recv()?;
        Ok(rec)
    }
}
