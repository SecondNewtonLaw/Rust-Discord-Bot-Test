mod bot_command_handler;
mod commands;
mod extensions;

use bot_command_handler::*;
use serenity::{framework::StandardFramework, prelude::*};

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = load_token();
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

fn load_token() -> String {
    match std::fs::read_to_string("token.secret") {
        Ok(content) => content,
        Err(err) => panic!("Failed operation with error \'{err}\', could not read Bot Token file, and, therefore, can not continue with the execution of the program.")
    } // Load token filev
}
