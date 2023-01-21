use crate::msg::Msg;
use std::marker::PhantomData;

pub struct Subscription<M: Msg> {
    raw: fops::Subscription<fops::BinaryMsg>,
    phantom: PhantomData<M>,
}

impl<M: Msg> Subscription<M> {
    pub(crate) fn new(raw: fops::Subscription<fops::BinaryMsg>) -> Self {
        Self {
            raw,
            phantom: PhantomData,
        }
    }

    pub async fn receive(&mut self) -> anyhow::Result<M>
    where
        M: Send,
        <M as TryFrom<fops::BinaryMsg>>::Error: Into<anyhow::Error> + Send + Sync + 'static,
    {
        let msg = self.raw.receive().await?;
        Ok(msg.try_into().map_err(|e: M::Error| e.into())?)
    }
}