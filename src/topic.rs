use crate::{msg::Msg, subscription::Subscription};
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct Topic<M: Msg> {
    raw: fops::Topic<Vec<u8>>,
    phantom: PhantomData<M>,
}

impl<M: Msg> Topic<M> {
    pub fn from_raw(raw: fops::Topic<Vec<u8>>) -> Self {
        Self {
            raw,
            phantom: PhantomData,
        }
    }

    pub fn publish(&self, msg: M) -> anyhow::Result<usize> {
        let mut raw_msg = Vec::new();
        msg.encode(&mut raw_msg)?;
        self.raw.publish(raw_msg).map_err(anyhow::Error::new)
    }

    pub fn subscribe(&self) -> Subscription<M>
    where
        M: Send,
    {
        Subscription::new(self.raw.subscribe())
    }
}
