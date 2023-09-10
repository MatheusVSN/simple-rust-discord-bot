use serde_json::Value;
use serenity::model::prelude::application_command::CommandDataOption;

pub fn convert_interaction_value_to_string(option: &CommandDataOption) -> String {
    option.value.clone().unwrap().to_string().replace('\"', "")
}

pub fn convert_bool_option_to_string(value: Option<bool>) -> String {
    let result = match value {
        Some(true) => "True",
        Some(false) => "False",
        _ => "False",
    };

    result.to_string()
}

pub fn convert_bool_option_to_bool(value: Option<bool>) -> bool {
    match value {
        Some(true) => true,
        Some(false) => false,
        _ => false,
    }
}

pub fn convert_value_to_bool(value: &Value) -> bool {
    match value {
        Value::Bool(true) => true,
        Value::Bool(false) => false,
        _ => false,
    }
}
