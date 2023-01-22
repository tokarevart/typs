pub trait TopicName {
    fn topic_name() -> &'static [u8];
}

pub trait Encode {
    fn encode(&self, buf: &mut Vec<u8>) -> anyhow::Result<()>;
}

pub trait Decode: Sized {
    fn decode(buf: &[u8]) -> anyhow::Result<Self>;
}

pub trait Msg: TopicName + Encode + Decode + Clone {}
