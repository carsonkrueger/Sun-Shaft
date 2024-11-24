use std::env;

enum ConfigError {
    ConfigMissingEnv(&'static str),
}

pub type Result<T> = std::result::Result<T, ConfigError>;

#[allow(non_snake_case)]
pub struct Config {
    ENVIRONMENT: String,
    DB_DOMAIN: String,
    DB_NAME: String,
    DB_USER: String,
    DB_PORT: String,
    DB_PASSWORD: String,
    BACK_END_DOMAIN: String,
    BACK_END_PORT: String,
}

impl Config {
    pub fn load_config() -> Result<Self> {
        Ok(Config {
            ENVIRONMENT: Self::get_env_var("ENVIRONMENT")?,
            DB_DOMAIN: Self::get_env_var("DB_DOMAIN")?,
            DB_NAME: Self::get_env_var("DB_NAME")?,
            DB_USER: Self::get_env_var("DB_USER")?,
            DB_PORT: Self::get_env_var("DB_PORT")?,
            DB_PASSWORD: Self::get_env_var("DB_PASSWORD")?,
            BACK_END_DOMAIN: Self::get_env_var("BACK_END_DOMAIN")?,
            BACK_END_PORT: Self::get_env_var("BACK_END_PORT")?,
        })
    }
    fn get_env_var(name: &'static str) -> Result<String> {
        env::var(name).map_err(|_| ConfigError::ConfigMissingEnv(name))
    }
}
