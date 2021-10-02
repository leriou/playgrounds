use std::fmt::Write;
use std::time::Duration;
use kafka::producer::{Producer, Record, RequiredAcks};
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

pub fn producer() {
    let mut producer = Producer::from_hosts(vec!("127.0.0.1:9092".to_owned()))
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::None)
        .create()
        .unwrap();

    let topic_name = "rust_test";
    let mut buf = String::with_capacity(30);
    let mut key = String::with_capacity(20);
    for i in 100..200 {
        let _ = write!(&mut key, "key {}", i);
        let _ = write!(&mut buf, "hello  {}", i); // some computation of the message data to be sent
        let _ = producer.send(&Record::from_key_value(topic_name, key.as_bytes(), buf.as_bytes()));
        buf.clear();
        key.clear();
    }
    println!("send success!")
}

pub fn consumer() {
    let mut consumer =
        Consumer::from_hosts(vec!("localhost:9092".to_owned()))
        .with_topic_partitions("rust_test".to_owned(), &[0])
        .with_fallback_offset(FetchOffset::Earliest)
        .with_group("my-group-2".to_owned())
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .unwrap();
    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                let v = String::from_utf8_lossy(m.value);
                println!("consumed k {:?}, v -> {:?}", m.offset, v);
            }
            let _ = consumer.consume_messageset(ms);
        }
        consumer.commit_consumed().unwrap();
    }
}