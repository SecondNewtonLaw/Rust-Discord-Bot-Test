use serenity::{
    builder::{CreateApplicationCommand, CreateEmbed, CreateInteractionResponseData},
    model::prelude::interaction::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
    prelude::Context,
};

/// Register the 'Help' application commands.
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("help")
        .description("Get help on how the bot works!")
}
/// The logic that represents the Help command, returns a bool that represents wether the command was successfully sent.
pub async fn run(ctx: &Context, options: &ApplicationCommandInteraction) -> Result<bool, String> {
    match options
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|msg: &mut CreateInteractionResponseData| {
                    // Attempt to avoid making this too hard to read.
                    process_command_inner(msg);
                    msg
                })
        })
        .await
    {
        Ok(_) => return Ok(true),
        Err(err) => return Err(format!("Error: {}", err)),
    }
}

fn process_command_inner(command: &mut CreateInteractionResponseData) {
    command.add_embed(make_help_embed());
    command.allowed_mentions(|allowed_mentions| {
        return allowed_mentions.replied_user(true);
    });
}

fn make_help_embed() -> serenity::builder::CreateEmbed {
    let mut embed = CreateEmbed::default();

    embed.title("Help");
    embed.description("This is a test of a Slash Command for a Rust Discord Bot");

    return embed;
}
