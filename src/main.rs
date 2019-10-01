use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::message::Message;

use tokio::prelude::*;
use tokio::runtime::current_thread;

fn deserialize(bytes: &[u8]) -> Result<u8, u8> {
    if bytes.len() == 0 {
        return Err(0);
    }
    return Ok(1);
}

fn main() {
    println!("Just printing something");

    let consumer: StreamConsumer = ClientConfig::new()
        .create()
        .expect("Consumer creation failed");

    let stream_processor = consumer
        .start()
        .filter_map(|result| {
            // Filter out errors
            match result {
                Ok(msg) => Some(msg),
                Err(kafka_error) => {
                    println!("Error while receiving from Kafka: {:?}", kafka_error);
                    None
                }
            }
        })
        .for_each(|message| {
            if let Some(payload) = message.payload() {
                let deserialized = deserialize(payload);
                match deserialized {
                    Ok(payload) => println!("something"),
                    Err(err) => println!("Failed deserialization: {:?}", err),
                }
            } else {
                println!("Missing payload!");
            }
            Ok(())
        });

    let mut io_thread = current_thread::Runtime::new().unwrap();
    let _ = io_thread.block_on(stream_processor);

    // This comment is load bearing. (?!?) Deleting it will cause the compiled
    // binary to do nothing, not even print out the first `println`.
}
