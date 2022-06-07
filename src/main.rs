extern crate ron;
use serenity::{
    prelude::*,
    model::prelude::*,
    Client,
};
struct Handler;

// #[serenity::async_trait]
impl EventHandler for Handler{
    fn message(&self, context: Context, msg: Message) {
        unimplemented!();
    }
}

fn main() {
    let mut client  = Client::builder("<token>", GatewayIntents::default()).event_handler(Handler);
}
