use crate::config::config_manager::ConfigTrait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub env_var_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub user_name: String,
    pub accounts: Vec<Account>
}

impl Config {
    pub fn new (created_config: CreatedConfig) -> Config {
        let user_name = created_config.user_name;
        let accounts = created_config.accounts
            .iter()
            .map(|account| {
                Account {
                    name: account.name.clone(),
                    env_var_name: account.env_var_name.clone()
                }
            }).collect::<Vec<_>>();
        Config {
            user_name: user_name,
            accounts
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreatedAccount {
    pub name: String,
    pub env_var_name: String,
    pub env_var_value: String,
    pub create_env_var: bool,
}

#[derive(Debug, Clone)]
pub struct CreatedConfig {
    pub user_name: String,
    pub accounts: Vec<CreatedAccount>
}

impl ConfigTrait for Config {
    fn config_directory() -> &'static str {
        ".aita/configuration"
    }

    fn config_filename() -> &'static str {
        "user_profile.json"
    }
}