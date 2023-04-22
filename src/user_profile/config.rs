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