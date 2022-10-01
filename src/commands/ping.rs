use serenity::{
    builder::{CreateApplicationCommand, CreateInteractionResponseData},
    model::prelude::interaction::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
    prelude::Context,
};

/// Register the 'Ping' application commands.
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ping")
        .description("Get the latency from the bot to the Discord Gateway!")
}

/// The logic that represents the Ping command, returns a bool that represents wether the command was successfully sent.
pub async fn run(ctx: &Context, options: &ApplicationCommandInteraction) -> Result<bool, String> {
    match options
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|msg: &mut CreateInteractionResponseData| {
                    msg.content("Pong!")
                })
        })
        .await
    {
        Ok(_) => return Ok(true),
        Err(err) => return Err(format!("Error: {}", err)),
    }
}
