pub trait TopicName {
    fn topic_name() -> &'static [u8];
}

pub trait Msg: TopicName + TryFrom<Vec<u8>> + Into<Vec<u8>> + Clone {}
