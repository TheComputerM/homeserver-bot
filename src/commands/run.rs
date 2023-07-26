use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::CommandDataOptionValue;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use std::process::Command;

pub fn run(options: &[CommandDataOption]) -> String {
    let option = options
        .get(0)
        .expect("Expected command string")
        .resolved
        .as_ref()
        .expect("Expected command string");

    if let CommandDataOptionValue::String(command) = option {
        let output = Command::new("bash").arg("-c").arg(command).output();

        if output.is_ok() {
            let output = output.unwrap();
            return [
                format!("**Output:**\n{}", String::from_utf8_lossy(&output.stdout)),
                format!("**Error:**\n{}", String::from_utf8_lossy(&output.stderr)),
            ]
            .join("\n-----------\n");
        } else {
            return "Error while running command".to_string();
        }
    } else {
        return "Please provide a valid command".to_string();
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("run")
        .description("Runs a command on your system")
        .create_option(|option| {
            option
                .name("command")
                .description("Command to run on your system")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
