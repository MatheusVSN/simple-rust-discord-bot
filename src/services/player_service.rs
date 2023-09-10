use optional_struct::optional_struct;
use optional_struct::Applyable;
use std::collections::HashMap;

use crate::handlers::firebase::get_database;
use crate::utils::roblox::get_player_by_username;
use serde::{Deserialize, Serialize};

pub type PlayerList = HashMap<String, OptionalPlayer>;

#[optional_struct]
#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub team: String,
    pub manager: bool,
    pub assistant_manager: bool,
    pub staff: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct UpdatePlayerTeamData {
    team: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UpdateManagementData {
    manager: bool,
    assistant_manager: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct UpdateStaffData {
    staff: bool,
}

pub async fn get_players_by_team(name: &str) -> Result<PlayerList, String> {
    let formatted_string = format!("\"{}\"", name.to_lowercase());

    let players: PlayerList = get_database()
        .at("users")
        .with_params()
        .add_param("orderBy", "\"team\"")
        .add_param("equalTo", formatted_string)
        .finish()
        .get()
        .await
        .expect("Error connecting into firebase database");

    if players.is_empty() {
        return Err(
            "Failed to get the team data. Make sure the teams exists and contains players"
                .to_string(),
        );
    }

    Ok(players)
}

pub async fn rank_player(team: String, username: String) -> Result<(), String> {
    let response = get_player_by_username(username).await;

    if response.data.is_empty() {
        return Err(
            "The data received from ROBLOX is empty. Make sure the user does exists".to_string(),
        );
    }

    let user_id = response.data[0].id as u64;
    let new_data = UpdatePlayerTeamData { team };
    let player_path = get_database().at("users").at(user_id.to_string().as_str());
    player_path
        .update(&new_data)
        .await
        .expect("An error occurred while updating the player data");

    Ok(())
}

pub async fn get_player_info(username: &String) -> Result<OptionalPlayer, String> {
    let response = get_player_by_username(username.to_string()).await;

    if response.data.is_empty() {
        return Err(
            "The data received from ROBLOX is empty. Make sure the user does exists".to_string(),
        );
    };

    let user_id = response.data[0].id as u64;
    let player_path = get_database().at("users").at(user_id.to_string().as_str());
    let player = player_path
        .get::<OptionalPlayer>()
        .await
        .expect("Failed to get the player");

    Ok(player)
}

pub async fn change_player_manager_status(username: &String, state: bool) -> Result<(), String> {
    let response = get_player_by_username(username.to_string()).await;

    if response.data.is_empty() {
        return Err(
            "The data received from ROBLOX is empty. Make sure the user does exists".to_string(),
        );
    };

    let user_id = response.data[0].id as u64;
    let player_path = get_database().at("users").at(user_id.to_string().as_str());
    let new_data = UpdateManagementData {
        manager: state,
        assistant_manager: false,
    };
    player_path
        .update(&new_data)
        .await
        .expect("Failed to update the player management status");

    Ok(())
}

pub async fn change_player_assistant_management_status(
    username: &String,
    state: bool,
) -> Result<(), String> {
    let response = get_player_by_username(username.to_string()).await;

    if response.data.is_empty() {
        return Err(
            "The data received from ROBLOX is empty. Make sure the user does exists".to_string(),
        );
    };

    let user_id = response.data[0].id as u64;
    let players_path = get_database().at("users").at(user_id.to_string().as_str());
    let new_data = UpdateManagementData {
        manager: false,
        assistant_manager: state,
    };
    players_path
        .update(&new_data)
        .await
        .expect("Failed to update the player assistant manager status");

    Ok(())
}

pub async fn change_player_staff_state(username: &String, state: bool) -> Result<(), String> {
    let response = get_player_by_username(username.to_string()).await;

    if response.data.is_empty() {
        return Err(
            "The data received from ROBLOX is empty. Make sure the user does exists".to_string(),
        );
    }

    let user_id = response.data[0].id as u64;
    let player_path = get_database().at("users").at(user_id.to_string().as_str());
    let new_data = UpdateStaffData { staff: state };
    player_path
        .update(&new_data)
        .await
        .expect("Failed to update the player staff state");

    Ok(())
}
