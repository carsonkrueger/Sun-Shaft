use std::env;

#[derive(Debug)]
enum ConfigError {
    ConfigMissingEnv(&'static str),
}

pub type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Config {
    pub ENVIRONMENT: Environment,
    pub DB_DOMAIN: String,
    pub DB_NAME: String,
    pub DB_USER: String,
    pub DB_PORT: String,
    pub DB_PASSWORD: String,
    pub BACK_END_DOMAIN: String,
    pub BACK_END_PORT: String,
}

impl Config {
    pub fn load_config() -> Result<Self> {
        Ok(Config {
            ENVIRONMENT: Self::get_env_var("ENVIRONMENT")?.into(),
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
        dotenvy::var(name).map_err(|_| ConfigError::ConfigMissingEnv(name))
    }
}

#[derive(Debug)]
pub enum Environment {
    DEV,
    PROD,
}

impl From<String> for Environment {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "production" | "prod" => Self::PROD,
            "development" | "dev" | _ => Self::DEV,
        }
    }
}
