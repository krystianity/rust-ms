pub mod consumer {
    
    use std::io;
    use futures::stream::Stream;

    use rdkafka::Message;
    use rdkafka::client::{ClientContext};
    use rdkafka::consumer::{Consumer, ConsumerContext, CommitMode, Rebalance};
    use rdkafka::consumer::stream_consumer::StreamConsumer;
    use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
    use rdkafka::error::KafkaResult;

    //holds custom callbacks for consumer
    struct MSConsumerContext;

    impl ClientContext for MSConsumerContext {}

    impl ConsumerContext for MSConsumerContext {

        fn pre_rebalance(&self, rebalance: &Rebalance) {
            debug!("Pre rebalance {:?}", rebalance);
        }

        fn post_rebalance(&self, rebalance: &Rebalance) {
            debug!("Post rebalance {:?}", rebalance);
        }

        fn commit_callback(&self, _result: KafkaResult<()>, _offsets: *mut rdkafka_sys::RDKafkaTopicPartitionList) {
            debug!("Committing offsets");
        }
    }

    //type alias for consumer
    type CustomConsumer = StreamConsumer<MSConsumerContext>;

    pub struct MSConsumer {
        client: CustomConsumer
    }

    impl MSConsumer {

        pub fn new(brokers: &str, group_id: &str) -> Result<MSConsumer, io::Error> {

            let context = MSConsumerContext;

            let client = ClientConfig::new()
                .set("group.id", group_id)
                .set("bootstrap.servers", brokers)
                .set("enable.partition.eof", "false")
                .set("session.timeout.ms", "6000")
                .set("enable.auto.commit", "true")
                .set("statistics.interval.ms", "30000")
                .set("auto.offset.reset", "earliest")
                .set_log_level(RDKafkaLogLevel::Debug)
                .create_with_context::<_, CustomConsumer>(context)
                .expect("Consumer creation failed");

            info!("New Kafka Consumer created.");
            Ok(MSConsumer{client})
        }

        pub fn consume(&self, topics: &[&str]) -> Result<(), io::Error> {
            
            self.client
                .subscribe(&topics.to_vec())
                .expect("Can not subscribe to specified topics.");
            
            let message_stream = self.client.start();

            for _message in message_stream.wait() {
                match _message {
                    Err(_) => {
                        warn!("Error while reading from stream.");
                    },
                    Ok(Err(error)) => {
                        warn!("Kafka error: {}", error);
                    },
                    Ok(Ok(message)) => {

                        let payload = match message.payload_view::<str>() {
                            None => "",
                            Some(Ok(s)) => s,
                            Some(Err(e)) => {
                                warn!("Error while deserializing message payload: {:?}", e);
                                ""
                            },
                        };

                        info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}",
                            message.key(), payload, message.topic(), message.partition(), message.offset());

                        //TODO callback here?
                        self.client.commit_message(&message, CommitMode::Async).unwrap();
                    }
                }
            }

            Ok(())
        }
    }
}