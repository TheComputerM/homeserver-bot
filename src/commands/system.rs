use nixinfo;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    return [
        format!("**Device**: {}", nixinfo::device().unwrap()),
        format!("**Hostname**: {}", nixinfo::hostname().unwrap()),
        format!("**Uptime**: {}", nixinfo::uptime().unwrap()),
        format!("**Memory**: {}", nixinfo::memory().unwrap()),
        format!("**CPU**: {}", nixinfo::cpu().unwrap()),
        format!("**Temperature**: {}°C", nixinfo::temp().unwrap()),
        format!("**GPU**: {}", nixinfo::gpu().unwrap()),
        format!("**OS**: {}", nixinfo::distro().unwrap()),
        format!("**Kernel**: {}", nixinfo::kernel().unwrap()),
    ]
    .join("\n");
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("system")
        .description("Returns information about the system.")
}
