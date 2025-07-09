use std::{sync::Arc, time::Duration};

use rabbitmq_stream_client::{
    error::StreamCreateError,
    types::{Message, ResponseCode},
    NoDedup, Producer,
};

pub struct RmqStream {
    producer: Producer<NoDedup>,
}

impl RmqStream {
    pub async fn new(stream: &str) -> Self {
        use rabbitmq_stream_client::Environment;
        let environment = Environment::builder()
            .build()
            .await
            .expect("Failed to create RabbitMQ environment");
        let _ = environment
            .stream_creator()
            .max_length(rabbitmq_stream_client::types::ByteCapacity::MB(100))
            .max_age(Duration::from_secs(60))
            .create(stream)
            .await
            .map_err(|e| {
                if let StreamCreateError::Create { stream, status } = e {
                    match status {
                        ResponseCode::StreamAlreadyExists => {}
                        err => {
                            panic!("Error creating stream {stream}: {err:#?}");
                        }
                    }
                }
            });
        let producer = environment
            .producer()
            .build(stream)
            .await
            .expect("Failed to create producer");
        Self { producer }
    }

    pub async fn new_atomic(stream: &str) -> Arc<Self> {
        let producer = Self::new(stream).await;
        Arc::new(producer)
    }

    pub async fn send_update_files(
        &self,
    ) -> Result<(), rabbitmq_stream_client::error::ProducerPublishError> {
        self.producer
            .send_with_confirm(Message::builder().body("test").build())
            .await
            .map(|_| ())
            .map_err(|e| {
                eprintln!("Error on update files: {e}");
                e
            })
    }
}
