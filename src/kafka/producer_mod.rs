pub mod producer {

    extern crate futures;
    extern crate rdkafka;

    use std::io;
    use self::futures::*;

    use self::rdkafka::client::EmptyContext;
    use self::rdkafka::config::ClientConfig;
    use self::rdkafka::producer::FutureProducer;
    use self::rdkafka::error::KafkaError;
    use self::rdkafka::message::OwnedMessage;

    pub struct MSProducer {
        client: FutureProducer<EmptyContext>
    }

    impl MSProducer {
        
        pub fn new(brokers: &str) -> Result<MSProducer, io::Error>{

            let client = ClientConfig::new()
                .set("bootstrap.servers", brokers)
                .set("produce.offset.report", "true")
                .set("message.timeout.ms", "5000")
                .create::<FutureProducer<_>>()
                .expect("Producer creation error");

            info!("New Kafka Producer created.");
            Ok(MSProducer{client})
        }

        pub fn produce(&self, topic_name: &str, key: &str, value: &str, partition: i64) -> Result<(i32, i64), (KafkaError, OwnedMessage)> {
            debug!("Producing {} to {}/{}.", key, topic_name, partition);
            self.client
                .send_copy(topic_name, None, Some(value), Some(key), None, partition)
                .map(move |delivery_status| {
                    debug!("Received delivery status for {} on {}/{}.", key, topic_name, partition);
                    delivery_status
                })
                .wait()
                .unwrap()
        }
    }
}