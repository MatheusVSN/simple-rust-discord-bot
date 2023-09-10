use serenity::{
    builder::{CreateApplicationCommand, CreateEmbed},
    model::prelude::{application_command::CommandDataOption, command::CommandOptionType},
};

use crate::{
    services::player_service::{change_player_assistant_management_status, get_player_info},
    utils::discord::{
        convert_bool_option_to_bool, convert_interaction_value_to_string, convert_value_to_bool,
    },
};

pub async fn run(options: &[CommandDataOption]) -> (Option<String>, Option<CreateEmbed>) {
    let player = options.get(0).expect("Expected a player");
    let assistant_manager = options
        .get(1)
        .expect("Expected a assistant management state");

    let username = convert_interaction_value_to_string(player);
    let assistant_manager_status = assistant_manager.value.clone().unwrap();
    let converted_assistant_manager_status = convert_value_to_bool(&assistant_manager_status);

    let result = get_player_info(&username).await;
    let response = match result {
        Ok(player) => player,
        Err(why) => {
            return (
                Some(format!(
                    "Something wrong happened while getting the player information. {}",
                    why
                )),
                None,
            );
        }
    };

    if response.team.is_none() {
        return (
            Some(format!(
                "Operation cancelled. The player {} is not in any team to get his management status changed",
                username
            )),
            None,
        );
    }

    let current_status = convert_bool_option_to_bool(response.assistant_manager);
    if current_status == assistant_manager_status {
        return (
            Some(format!(
                "Could not rank {} as assistant manager. User already has a management state of {}",
                username, current_status
            )),
            None,
        );
    }

    let rank_assistant_management_status =
        change_player_assistant_management_status(&username, converted_assistant_manager_status)
            .await;
    match rank_assistant_management_status {
        Ok(_) => (
            Some(format!(
                "Changed {} assistant manager status to {}",
                &username, converted_assistant_manager_status
            )),
            None,
        ),
        Err(why) => (
            Some(format!(
                "Failed to update {} assistant manager status. {}",
                username, why
            )),
            None,
        ),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("assistant_manager_status")
        .description("Sets the mentioned as a team management status")
        .create_option(|player| {
            player
                .name("player")
                .description("ROBLOX username to the who is going to manage")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|status| {
            status
                .name("state")
                .description("Set the player team assistant management state")
                .kind(CommandOptionType::Boolean)
                .required(true)
        })
}
