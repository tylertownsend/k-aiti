use super::user::settings::{SettingsConfig, ModelConfig};

pub enum ModeSelection {
    Completion,
    Chat,
    // Add more modes here as needed
}

fn get_mode_id_by_name(config: &SettingsConfig, mode: ModeSelection) -> Option<String> {
    let res = match mode {
        ModeSelection::Completion => config.modes.completion.id.clone(),
        ModeSelection::Chat => config.modes.chat.id.clone(),
        // Add more match arms here for additional modes
    };
    Option::Some(res)
}

pub fn get_model_by_mode(config: &SettingsConfig, mode: ModeSelection) -> Option<&ModelConfig> {
    let mode_id = get_mode_id_by_name(config, mode);

    let model = config.models.iter().find(|&m| {
        match &mode_id {
            Some(id) => m.id == *id,
            None => false
        }
    });
    model
}