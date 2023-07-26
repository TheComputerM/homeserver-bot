use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    "TODO".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("system").description("Returns information about the system.")
}