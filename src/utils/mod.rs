use std::{env, fs};
use serde::Deserialize;
use toml;

// Constant
const ENV_FILE_PATH: &str = "env.toml";
const USERNAME_KEY: &str = "username";
const PASSWORD_KEY: &str = "password";
const HOST_KEY: &str = "host";

#[derive(Debug, Deserialize)]
pub struct Env {
    pub username: String,
    pub password: String,
    pub host: String
}

/// Load Env
///
/// # Description
/// Load the environment variables in 2 way
/// - If it find a env.toml will load the env.toml
/// - Otherwise load from os environment variables
pub fn load_env() -> Result<Env, Box<dyn std::error::Error>> {
    let env_file = fs::read_to_string(ENV_FILE_PATH);

    match env_file {
        Ok(res) => toml::from_str(&res).map_err(|err| err.into()),
        Err(_) => load_global_env()
    }
}

/// Load Global Env
///
/// # Description
/// Create the env handle from the global env
fn load_global_env() -> Result<Env, Box<dyn std::error::Error>> {
    println!("Will use global env");

    let username = env::var(USERNAME_KEY).unwrap_or_default();
    let password = env::var(PASSWORD_KEY).unwrap_or_default();
    let host = env::var(HOST_KEY).unwrap_or("127.0.0.1:9091".to_owned());

    Ok(Env {
        username,
        password,
        host
    })
}