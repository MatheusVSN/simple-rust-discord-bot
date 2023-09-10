use serenity::{
    builder::{CreateApplicationCommand, CreateEmbed},
    model::prelude::{application_command::CommandDataOption, command::CommandOptionType},
};

use crate::{
    services::player_service::rank_player, utils::discord::convert_interaction_value_to_string,
};

pub async fn run(options: &[CommandDataOption]) -> (Option<String>, Option<CreateEmbed>) {
    let player = options.get(0).expect("Expected player username");
    let team = options.get(1).expect("Expected team name");

    let username = convert_interaction_value_to_string(player);
    let team_name = convert_interaction_value_to_string(team);

    let result = rank_player(team_name, username).await;

    match result {
        Ok(_result) => (
            Some(format!(
                "Player {} has been ranked successfully to {}",
                player.value.clone().unwrap(),
                team.value.clone().unwrap()
            )),
            None,
        ),
        Err(why) => (
            Some(format!("An error happened processing your request. {}", why).to_string()),
            None,
        ),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("rank")
        .description("Ranks a mentioned player to a team")
        .create_option(|player| {
            player
                .name("player")
                .description("ROBLOX username of the player who'll get ranked")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|team| {
            team.name("team")
                .description("Name of the team that the player is getting ranked to")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
