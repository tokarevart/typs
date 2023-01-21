pub trait TopicName {
    fn topic_name() -> &'static [u8];
}

pub trait Msg: TopicName + TryFrom<fops::BinaryMsg> + Into<fops::BinaryMsg> + Clone {}
