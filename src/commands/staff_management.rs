use serenity::{
    builder::{CreateApplicationCommand, CreateEmbed},
    model::prelude::{application_command::CommandDataOption, command::CommandOptionType},
};

use crate::{
    services::player_service::{change_player_staff_state, get_player_info},
    utils::discord::{
        convert_bool_option_to_bool, convert_interaction_value_to_string, convert_value_to_bool,
    },
};

pub async fn run(options: &[CommandDataOption]) -> (Option<String>, Option<CreateEmbed>) {
    let player: &CommandDataOption = options.get(0).expect("Expected player username");
    let staff_state = options.get(1).expect("Expected staff state");

    let username = convert_interaction_value_to_string(player);
    let staff_state = staff_state.value.clone().unwrap();
    let converted_staff_state = convert_value_to_bool(&staff_state);

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

    let current_status = convert_bool_option_to_bool(response.staff);
    if current_status == staff_state {
        return (
            Some(format!(
                "Could set {} as staff. User already has a staff state of {}",
                username, current_status
            )),
            None,
        );
    };

    let staff_changed_result = change_player_staff_state(&username, converted_staff_state).await;
    match staff_changed_result {
        Ok(_) => (
            Some(format!(
                "Changed {} staff state to {}",
                &username, converted_staff_state
            )),
            None,
        ),
        Err(why) => (
            Some(format!(
                "Failed to update {} staff state. {}",
                username, why
            )),
            None,
        ),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("staff_management")
        .description("Change a player staff status")
        .create_option(|user| {
            user.name("user")
                .description("ROBLOX username of the player")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|state| {
            state
                .name("state")
                .description("State of the staff position")
                .kind(CommandOptionType::Boolean)
                .required(true)
        })
}
