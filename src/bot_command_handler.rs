use crate::extensions::*;

use serenity::{
    async_trait,
    framework::standard::macros::group,
    model::prelude::{Message, Ready},
    prelude::{Context, EventHandler},
};

use crate::commands::ping::*;

#[group]
#[commands(ping)]
pub struct General;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    /// Dispatched when a message is created.
    ///
    /// Provides the message's data.
    async fn message(&self, _ctx: Context, _new_message: Message) {
        if !message_helpers::is_bot(&_new_message) {
            println!(
                "Message received from user {}. Content: {}",
                _new_message.author.name, _new_message.content
            );
        } else {
            println!("Received message from a bot.");
        }
    }

    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("Bot Initialized as User: {}", _data_about_bot.user.name);
    }
}
