use serenity::{
    builder::{CreateApplicationCommand, CreateEmbed},
    model::prelude::{application_command::CommandDataOption, command::CommandOptionType},
};

use crate::{
    services::player_service::get_player_info,
    utils::{
        discord::{convert_bool_option_to_string, convert_interaction_value_to_string},
        roblox::{get_avatar_url_by_id, get_player_by_username},
    },
};

pub async fn run(options: &[CommandDataOption]) -> (Option<String>, Option<CreateEmbed>) {
    let player = options.get(0).expect("Expected a player username");
    let username = convert_interaction_value_to_string(player);

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

    let roblox_player = get_player_by_username(username.to_string()).await;
    let user_id = roblox_player.data[0].id;

    let manager_string = convert_bool_option_to_string(response.manager);
    let assistant_manager = convert_bool_option_to_string(response.assistant_manager);
    let staff = convert_bool_option_to_string(response.staff);

    let player_avatar = get_avatar_url_by_id(user_id).await;
    let avatar_response = match player_avatar {
        Ok(avatar) => avatar,
        Err(why) => {
            return (
                Some(format!(
                    "Something wrong happened while getting the player avatar. {}",
                    why
                )),
                None,
            );
        }
    };

    let player_embed = CreateEmbed::default()
        .title(format!("[{}] information", username))
        .thumbnail(&avatar_response.data[0].image_url)
        .field("Manager", manager_string, false)
        .field("Assistant Manager", assistant_manager, false)
        .field("Team", response.team.unwrap_or("F/A".to_string()), false)
        .field("Staff", staff, false)
        .to_owned();

    (None, Some(player_embed))
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("player_info")
        .description("Gets the player information by his ROBLOX username")
        .create_option(|username| {
            username
                .name("username")
                .description("ROBLOX username of the player")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
