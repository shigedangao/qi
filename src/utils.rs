use std::{env, fs};
use serde::Deserialize;

// Constant
const ENV_FILE_PATH: &str = "env.toml";
const HOST_KEY: &str = "host";

#[derive(Debug, Deserialize)]
pub struct Env {
    pub host: String
}

/// Load the environment variables in 2 way
/// - If it find a env.toml will load the env.toml
/// - Otherwise load from os environment variables
pub fn load_env() -> Env {
    let env_toml = fs::read_to_string(ENV_FILE_PATH);
    
    if let Ok(env) = env_toml {
        if let Ok(t) = toml::from_str::<Env>(&env) {
            info!("Will use local prometheus host {}", t.host);
            return t;
        }
    }

    info!("Will use global prometheus host");
    load_global_env()
}

/// Create the env handle from the global env
fn load_global_env() -> Env {
    info!("Will use global env");
    let host = env::var(HOST_KEY).unwrap_or_else(|_| "127.0.0.1:9091".to_owned());

    Env {
        host
    }
}