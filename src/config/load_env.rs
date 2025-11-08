use anyhow::{Result, format_err};
use std::env;

#[derive(Clone)]
pub struct EnvConfig {
    pub prime_org_id: u64,
    pub debug_mode: bool,
}

fn get_env_var(name: &str) -> Result<String> {
    env::var(name).map_err(|_| format_err!("{} not set", name))
}

fn get_env_u64(name: &str) -> Result<u64> {
    Ok(get_env_var(name)?.parse::<u64>()
        .map_err(|_| format_err!("{} must be a valid u64", name))?)
}

fn get_env_u16(name: &str) -> Result<u16> {
    Ok(get_env_var(name)?.parse::<u16>()
        .map_err(|_| format_err!("{} must be a valid u16", name))?)
}

fn get_env_bool(name: &str) -> Result<bool> {
    Ok(get_env_var(name)?.parse::<bool>()
        .map_err(|_| format_err!("{} must be true or false", name))?)
}

pub fn load_env() -> Result<EnvConfig> {
    let env_config =EnvConfig {
        prime_org_id: get_env_u64("PRIME_ORG_ID")?,
        debug_mode: get_env_bool("DEBUG_MODE")?,
    };
    println!("env loaded");
    Ok(env_config)
}