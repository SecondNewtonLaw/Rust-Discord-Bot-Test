
use serenity::{
    async_trait, 
    framework::standard::macros::group, 
    prelude::{
        EventHandler, Context
    }, 
    model::prelude::{
        Message, Ready
    }
};


/*
    Get wether or not the Message was sent by a bot.
*/
pub fn is_bot(msg: &Message) -> bool {
    msg.author.bot
}
