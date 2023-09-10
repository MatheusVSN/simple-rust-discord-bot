use serenity::{
    builder::{CreateApplicationCommand, CreateEmbed},
    model::prelude::{application_command::CommandDataOption, command::CommandOptionType},
};

use crate::{
    services::player_service::get_players_by_team,
    utils::{discord::convert_interaction_value_to_string, roblox::get_player_information_by_id},
};

pub async fn run(options: &[CommandDataOption]) -> (Option<String>, Option<CreateEmbed>) {
    let team = options.get(0).expect("Expected team name");
    let team_name = convert_interaction_value_to_string(team);

    let result = get_players_by_team(&team_name).await;
    let response = match result {
        Ok(team) => team,
        Err(why) => {
            return (
                Some(format!(
                    "Something wrong happened while getting the team {} information. {}",
                    team_name, why
                )),
                None,
            );
        }
    };

    let mut players_ids: Vec<String> = Vec::new();
    for index in response.keys() {
        players_ids.push(index.to_string());
    }

    let mut player_names: Vec<String> = Vec::new();
    for id in players_ids.iter() {
        let converted_id: u64 = id.parse().unwrap();
        let player_information = get_player_information_by_id(converted_id).await;
        player_names.push(player_information.name);
    }

    let mut players_list_string = String::new();
    for player in player_names {
        players_list_string += &format!("{}\n", player);
    }

    let team_embed = CreateEmbed::default()
        .title(format!("{} Team information", team_name).to_uppercase())
        .field("Players", players_list_string, false)
        .to_owned();

    (None, Some(team_embed))
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("team_info")
        .description("Gets the team information")
        .create_option(|team| {
            team.name("team")
                .description("Name of the team")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
