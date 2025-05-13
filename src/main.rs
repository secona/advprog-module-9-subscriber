#![allow(clippy::empty_loop)]

use borsh_derive::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, HandleError, MessageHandler, QueueProperties};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        println!("In 2306152411's computer. Message received: {:?}", message);

        Ok(())
    }

    fn get_handler_action(&self) -> String {
        String::from("user_created")
    }
}

fn main() -> ! {
    let listener = CrosstownBus::new_queue_listener("amqp://guest:guest@localhost:5672".to_string()).unwrap();

    let _ = listener.listen(String::from("user_created"), UserCreatedHandler, QueueProperties {
        auto_delete: false,
        durable: false,
        use_dead_letter: true,
    });

    loop {}
}
