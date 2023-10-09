use anyhow::{Error, Result};
use std::env;

#[derive(Clone)]
pub struct Config {
    pub app_address: String,
    pub app_port: u16,
    pub gql_api: String,
    pub season_gql_id: String,
}

impl Config {
    pub fn new() -> Result<Config, Error> {
        let app_address = env::var("APP_ADDRESS")?;
        let app_port = env::var("APP_PORT")?.parse::<u16>()?;
        let gql_api = env::var("GQL_URI")?;
        let season_gql_id = env::var("SEASON_GQL_ID")?;

        Ok(Config {
            app_address,
            app_port,
            gql_api,
            season_gql_id,
        })
    }
}
