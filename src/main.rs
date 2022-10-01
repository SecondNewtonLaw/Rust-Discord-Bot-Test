mod commands;

use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:?}", command);

            let cmd_response = match command.data.name.as_str() {
                "ping" => commands::ping::run(&ctx, &command).await,
                "help" => commands::help::run(&ctx, &command).await,
                _ => {
                    let txt = format!(
                        "Command Failure: No Known Command logic for command with Name {}",
                        command.data.name
                    );
                    Err(txt)
                }
            };

            match cmd_response {
                Ok(ok_status) => println!("Command Success Status: {}", ok_status),
                Err(ex) => println!("Cannot respond to slash command: {}", ex),
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected as {}.", ready.user.name);

        let guild_id = GuildId(
            std::fs::read_to_string("serverid.secret")
                .unwrap()
                .trim()
                .replace("_", "")
                .parse()
                .unwrap(),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::help::register(command))
        })
        .await;

        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = std::fs::read_to_string("token.secret").unwrap();

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Execute the client's code and lock until it finishes of sorts.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
