use crate::{msg::Msg, subscription::Subscription};
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct Topic<M: Msg> {
    raw: fops::Topic<Vec<u8>>,
    phantom: PhantomData<M>,
}

impl<M: Msg> Topic<M> {
    pub(crate) fn new(raw: fops::Topic<Vec<u8>>) -> Self {
        Self {
            raw,
            phantom: PhantomData,
        }
    }

    pub fn publish(&self, msg: M) -> anyhow::Result<usize> {
        self.raw.publish(msg.into()).map_err(anyhow::Error::new)
    }

    pub fn subscribe(&self) -> Subscription<M>
    where
        M: Send,
    {
        Subscription::new(self.raw.subscribe())
    }
}
