use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use std::process::Command;

pub fn run(_options: &[CommandDataOption]) -> String {
    let public_ip = Command::new("curl")
        .arg("ipecho.net/plain")
        .output()
        .unwrap()
        .stdout;
    let public_ipv6 = Command::new("curl")
        .arg("ident.me")
        .output()
        .unwrap()
        .stdout;
    
    let local_ip = local_ip_address::local_ip().unwrap();
    let local_ipv6 = local_ip_address::local_ipv6().unwrap();

    return [
        format!("**Public IP**: {}", String::from_utf8_lossy(&public_ip)),
        format!("**Public IPv6**: {}", String::from_utf8_lossy(&public_ipv6)),
        format!("**Local IP**: {}", &local_ip),
        format!("**Local IPv6**: {}", &local_ipv6),
    ]
    .join("\n");
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ip")
        .description("Get the IP address of the system")
}
