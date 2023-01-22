use crate::{msg::Msg, Decode};
use std::marker::PhantomData;

pub struct Subscription<M: Msg> {
    raw: fops::Subscription<Vec<u8>>,
    phantom: PhantomData<M>,
}

impl<M: Msg> Subscription<M> {
    pub(crate) fn new(raw: fops::Subscription<Vec<u8>>) -> Self {
        Self {
            raw,
            phantom: PhantomData,
        }
    }

    pub async fn receive(&mut self) -> anyhow::Result<M>
    where
        M: Send,
    {
        let mut raw_msg: Vec<_> = self.raw.receive().await?;
        Decode::decode(&mut raw_msg)
    }
}
