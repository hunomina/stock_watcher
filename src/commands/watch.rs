use serenity::{
    builder::CreateApplicationCommand,
    model::interactions::application_command::ApplicationCommandOptionType,
};

pub const COMMAND_NAME: &str = "watch";

pub(super) fn get_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(COMMAND_NAME)
        .description("Add a symbol to follow in this channel")
        .create_option(|option| {
            option
                .name("symbol")
                .description("Symbol to follow (stock/ETF)")
                .kind(ApplicationCommandOptionType::String)
                .required(true)
        })
}
