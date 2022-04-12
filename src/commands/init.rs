use serenity::builder::CreateApplicationCommand;

pub const COMMAND_NAME: &str = "init";

pub(super) fn get_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(COMMAND_NAME)
        .description("Add the current channel as subscriber")
}
