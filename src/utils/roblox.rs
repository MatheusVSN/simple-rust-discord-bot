use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct RobloxUsernameResponse {
    pub data: Vec<RobloxUsernameData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobloxUsernameData {
    pub requested_username: String,
    pub has_verified_badge: bool,
    pub id: i64,
    pub name: String,
    pub display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RobloxAvatarResponse {
    pub data: Vec<RobloxAvatarData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobloxAvatarData {
    pub target_id: i64,
    pub state: String,
    pub image_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RobloxPlayerByIdData {
    pub description: String,
    pub created: String,
    pub is_banned: bool,
    pub external_app_display_name: Value,
    pub has_verified_badge: bool,
    pub id: i64,
    pub name: String,
    pub display_name: String,
}

pub async fn get_player_by_username(username: String) -> RobloxUsernameResponse {
    let request_url = "https://users.roblox.com/v1/usernames/users";

    let client = reqwest::Client::new();
    let response: RobloxUsernameResponse = client
        .post(request_url)
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "usernames": [
               username
            ],
            "excludeBannedUsers": false
        }))
        .send()
        .await
        .expect("Failed to request the ROBLOX API")
        .json()
        .await
        .expect("Failed to convert the ROBLOX API Response");

    response
}

pub async fn get_player_information_by_id(id: u64) -> RobloxPlayerByIdData {
    let request_url = format!("https://users.roblox.com/v1/users/{}", id);

    let client = reqwest::Client::new();
    let response: RobloxPlayerByIdData = client
        .get(request_url)
        .send()
        .await
        .expect("Failed to request the ROBLOX API")
        .json()
        .await
        .expect("Failed to convert the ROBLOX data");

    response
}

pub async fn get_avatar_url_by_id(id: i64) -> Result<RobloxAvatarResponse, String> {
    let request_url = "https://thumbnails.roblox.com/v1/users/avatar-headshot";

    let user_id = id.to_string();

    let client = reqwest::Client::new();
    let query = vec![
        ("userIds", user_id),
        ("size", "720x720".to_string()),
        ("format", "Png".to_string()),
        ("isCircular", "true".to_string()),
    ];

    let response: RobloxAvatarResponse = client
        .get(request_url)
        .query(&query)
        .send()
        .await
        .expect("Failed to get avatar picture")
        .json()
        .await
        .expect("Failed to convert roblox user data");

    if response.data.is_empty() {
        return Err(
            "Failed to get the data from ROBLOX. Make sure the user does exists".to_string(),
        );
    }

    Ok(response)
}
