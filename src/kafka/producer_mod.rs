pub mod producer {

    use std::io;
    use futures::*;

    use rdkafka::client::DefaultClientContext;
    use rdkafka::config::ClientConfig;
    use rdkafka::producer::{FutureProducer, FutureRecord};
    use rdkafka::error::KafkaError;
    use rdkafka::message::OwnedMessage;

    pub struct MSProducer {
        client: FutureProducer<DefaultClientContext>
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

        pub fn produce(&self, topic_name: &str, key: &str, value: &str, partition: i32) -> Result<(i32, i64), (KafkaError, OwnedMessage)> {
            debug!("Producing {} to {}/{}.", key, topic_name, partition);
            let record = FutureRecord{
                topic: topic_name,
                partition: Some(partition),
                payload: Some(value),
                key: Some(key),
                timestamp: None,
                headers: None,
            };
            self.client
                .send(record, 0)
                .map(move |delivery_status| {
                    debug!("Received delivery status for {} on {}/{}.", key, topic_name, partition);
                    delivery_status
                })
                .wait()
                .unwrap()
        }
    }
}