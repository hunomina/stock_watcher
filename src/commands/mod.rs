// https://github.com/serenity-rs/serenity/blob/current/examples/e14_slash_commands/src/main.rs
use serenity::builder::CreateApplicationCommands;

pub mod init;
pub mod list;
pub mod watch;

pub fn get_commands(app: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    app.create_application_command(init::get_command)
        .create_application_command(list::get_command)
        .create_application_command(watch::get_command)
}
