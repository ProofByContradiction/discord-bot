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
    println!("Hello, world!");
}
