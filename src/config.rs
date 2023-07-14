use clap::Parser;
use std::env;

/// Q&A web service API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// Which errors we want to log (info, warn or error)
    #[clap(short, long, default_value = "warn")]
    pub log_level: String,
    /// Which PORT the server is listening to
    #[clap(short, long, default_value = "8080")]
    pub port: u16,
    /// Database user
    #[clap(long, default_value = "username")]
    pub db_user: String,
    /// Database user
    #[clap(long)]
    pub db_password: String,
    /// URL for the postgres database
    #[clap(long, default_value = "localhost")]
    pub db_host: String,
    /// PORT number for the database connection
    #[clap(long, default_value = "5432")]
    pub db_port: u16,
    /// Database name
    #[clap(long, default_value = "rustwebdev")]
    pub db_name: String,
}

impl Config {
    pub fn new() -> Result<Config, handle_errors::Error> {
        let config = Config::parse();

        if env::var("BAD_WORDS_API_KEY").is_err() {
            panic!("BadWords API key not set");
        }

        if env::var("PASETO_KEY").is_err() {
            panic!("PASETO_KEY not set");
        }

        let port = env::var("PORT")
            .ok()
            .map(|val| val.parse::<u16>())
            .unwrap_or(Ok(config.port))
            .map_err(handle_errors::Error::ParseError)?;

        let db_user = env::var("POSTGRES_USER").unwrap_or(config.db_user.to_owned());
        let db_password = env::var("POSTGRES_PASSWORD").unwrap_or(config.db_password.to_owned());
        let db_host = env::var("POSTGRES_HOST").unwrap_or(config.db_host.to_owned());
        let db_port = env::var("POSTGRES_PORT").unwrap_or(config.db_port.to_string());
        let db_name = env::var("POSTGRES_DB").unwrap_or(config.db_name.to_owned());

        Ok(Config {
            log_level: config.log_level,
            port,
            db_user,
            db_password,
            db_host,
            db_port: db_port
                .parse::<u16>()
                .map_err(handle_errors::Error::ParseError)?,
            db_name,
        })
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;

    // #[test]
    // fn unset_api_key() {
    //     let result = std::panic::catch_unwind(|| Config::new());
    //     assert!(result.is_err());
    // }
}
