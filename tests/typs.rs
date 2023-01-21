use typs::{PubSub, TopicName};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TestMsg {
    num: i32,
}

impl TopicName for TestMsg {
    fn topic_name() -> &'static [u8] {
        b"TestMsg:num"
    }
}

impl Into<fops::BinaryMsg> for TestMsg {
    fn into(self) -> fops::BinaryMsg {
        fops::BinaryMsg::from(self.num.to_be_bytes())
    }
}

impl TryFrom<fops::BinaryMsg> for TestMsg {
    type Error = anyhow::Error;

    fn try_from(value: fops::BinaryMsg) -> anyhow::Result<Self> {
        let num = i32::from_be_bytes(value.as_slice().try_into()?);
        Ok(Self { num })
    }
}

impl typs::Msg for TestMsg {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AnotherTestMsg {
    string: String,
}

impl TopicName for AnotherTestMsg {
    fn topic_name() -> &'static [u8] {
        b"AnotherTestMsg:string"
    }
}

impl Into<fops::BinaryMsg> for AnotherTestMsg {
    fn into(self) -> fops::BinaryMsg {
        fops::BinaryMsg::from(self.string.as_bytes())
    }
}

impl TryFrom<fops::BinaryMsg> for AnotherTestMsg {
    type Error = anyhow::Error;

    fn try_from(value: fops::BinaryMsg) -> anyhow::Result<Self> {
        let string = String::from_utf8(value.into())?;
        Ok(Self { string })
    }
}

impl typs::Msg for AnotherTestMsg {}

#[tokio::test]
async fn publish_and_receive() {
    let pubsub = PubSub::new();

    let msg = TestMsg { num: 42 };
    let topic: typs::Topic<TestMsg> = pubsub.topic().await;

    let mut subscription = topic.subscribe();
    let handle = tokio::spawn(async move { subscription.receive().await });

    topic.publish(msg.clone()).unwrap();

    let recv_msg = handle.await.unwrap().unwrap();
    assert_eq!(recv_msg, msg);
}

#[tokio::test]
async fn publish_and_receive_two() {
    let pubsub = PubSub::new();

    let test_msg = TestMsg { num: 42 };
    let another_test_msg = AnotherTestMsg {
        string: "test".to_string(),
    };

    let test_topic: typs::Topic<TestMsg> = pubsub.topic().await;
    let another_test_topic: typs::Topic<AnotherTestMsg> = pubsub.topic().await;

    let mut test_subscription = test_topic.subscribe();
    let test_handle = tokio::spawn(async move { test_subscription.receive().await });

    let mut another_test_subscription = another_test_topic.subscribe();
    let another_test_handle =
        tokio::spawn(async move { another_test_subscription.receive().await });

    test_topic.publish(test_msg.clone()).unwrap();
    another_test_topic
        .publish(another_test_msg.clone())
        .unwrap();

    let recv_test_msg = test_handle.await.unwrap().unwrap();
    let recv_another_test_msg = another_test_handle.await.unwrap().unwrap();

    assert_eq!(recv_test_msg, test_msg);
    assert_eq!(recv_another_test_msg, another_test_msg);
}
