use crate::{msg::Msg, topic::Topic};

pub struct PubSub {
    raw: fops::PubSub<Vec<u8>>,
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
        let raw = self.raw_topic(M::topic_name()).await;
        Topic::new(raw)
    }

    pub async fn raw_topic(&self, name: &[u8]) -> fops::Topic<Vec<u8>> {
        self.raw.topic(name).await
    }
}
