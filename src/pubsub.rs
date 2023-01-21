use crate::{msg::Msg, topic::Topic};

pub struct PubSub {
    raw: fops::PubSub<fops::BinaryMsg>,
}

impl PubSub {
    pub const fn new() -> Self {
        Self {
            raw: fops::PubSub::new(),
        }
    }

    pub fn with_topic_capacity(capacity: usize) -> Self {
        Self {
            raw: fops::PubSub::with_topic_capacity(capacity),
        }
    }

    pub async fn topic<M: Msg>(&self) -> Topic<M> {
        let raw = self.raw.topic(M::topic_name()).await;
        Topic::new(raw)
    }
}
