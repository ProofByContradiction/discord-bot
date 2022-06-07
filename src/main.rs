extern crate ron;
pub mod config;
use config::Config;
use serenity::{
    prelude::*,
    model::prelude::*,
    Client,
};

struct Handler;
impl EventHandler for Handler{
    fn message(&self, context: Context, msg: Message) {
        unimplemented!();
    }
}

fn main() {
    let _ = Config::new().save();
    let config = Config::load().unwrap();

    let mut client  = Client::builder(config.token(), GatewayIntents::default()).event_handler(Handler);
}
