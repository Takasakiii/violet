use std::{any::Any, collections::HashMap, sync::mpsc::{self, Receiver, Sender}};

static mut GEN_CHANNEL: Option<GerChannels> = None;

pub struct Channel {
    sender: Sender<Box<dyn Any>>,
    receiver: Receiver<Box<dyn Any>>
}

impl Channel {
    pub fn send_data(&self, any_data: impl Any) -> Result<(), mpsc::SendError<Box<dyn Any>>> {
        let boxed_any = Box::new(any_data);
        self.sender.send(boxed_any)?;
        Ok(())
    }

    pub fn get_data<J>(&self) -> Option<Box<J>>
    where
        J: Any
    {
        let received_value = match self.receiver.recv() {
            Err(_) => return None,
            Ok(data) => data
        };
        let converted_value = received_value.downcast::<J>().ok();
        converted_value
    }
}

pub struct GerChannels {
    data: HashMap<String, Channel>
}

impl GerChannels {
    pub fn get<F>(mut action: F)
    where
        F: FnMut(&mut GerChannels) -> ()
    {
        unsafe {
            if GEN_CHANNEL.is_none() {
                let genchannel = Self{
                    data: HashMap::new()
                };
                GEN_CHANNEL = Some(genchannel);
            }
            if let Some(channel) = &mut GEN_CHANNEL {
                action(channel);
            }
        }
    }

    pub fn create_channel(&mut self, channel_name: &str) {
        let (sender, receiver) = mpsc::channel::<Box<dyn Any>>();
        let channel = Channel {
            sender,
            receiver
        };
        self.data.insert(channel_name.into(), channel);
    }

    pub fn get_channel<F>(&self, channel_name: &str, mut callback: F) -> Result<(), String>
    where
        F: FnMut(&Channel) -> ()
    {
        let channel = self.data.get(channel_name.into());
        match channel {
            Some(channel) => {
                callback(channel);
            },
            None => return Err("Canal não existe.".into())
        }
        Ok(())
    }
}
