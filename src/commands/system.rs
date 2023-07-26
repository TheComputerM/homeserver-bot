use nixinfo;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    let temp = format!("{}°C", nixinfo::temp().unwrap());
    return [
        ["Device", &nixinfo::device().unwrap()].join(": "),
        ["Hostname", &nixinfo::hostname().unwrap()].join(": "),
        ["Uptime", &nixinfo::uptime().unwrap()].join(": "),
        ["Memory", &nixinfo::memory().unwrap()].join(": "),
        ["CPU", &nixinfo::cpu().unwrap()].join(": "),
        ["Temperature", &temp].join(": "),
        ["GPU", &nixinfo::gpu().unwrap()].join(": "),
        ["OS", &nixinfo::distro().unwrap()].join(": "),
        ["Kernel", &nixinfo::kernel().unwrap()].join(": "),
        ["Environment", &nixinfo::environment().unwrap()].join(": "),
    ]
    .join("\n");
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("system")
        .description("Returns information about the system.")
}
