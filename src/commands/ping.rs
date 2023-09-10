use serenity::{
    builder::{CreateApplicationCommand, CreateEmbed},
    model::prelude::application_command::CommandDataOption,
};

pub fn run(_options: &[CommandDataOption]) -> (Option<String>, Option<CreateEmbed>) {
    (Some("Pong!".to_string()), None)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}
