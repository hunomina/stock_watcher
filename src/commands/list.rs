use serenity::builder::CreateApplicationCommand;

pub const COMMAND_NAME: &str = "list";

pub(super) fn get_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(COMMAND_NAME)
        .description("List all the subscribers")
}
