use serenity::builder::CreateApplicationCommand;

/// Register the 'Help' application commands.
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("help")
        .description("Get help on how the bot works!")
}
