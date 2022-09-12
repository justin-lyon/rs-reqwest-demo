use std::env;

const ENVVAR_BASE_URL: &str = "API_CONGRESS_GOV_BASE_URL";
const ENVVAR_VERSION: &str = "API_CONGRESS_GOV_VERSION";
const ENVVAR_API_KEY: &str = "API_CONGRESS_GOV_API_KEY";

pub struct Config {
    pub base_url: String,
    pub version: String,
    pub api_key: String,
}

pub fn get_config() -> Config {
    let base_url = match env::var_os(ENVVAR_BASE_URL) {
        Some(v) => v.into_string().unwrap(),
        None => panic!("{} is not set.", ENVVAR_BASE_URL),
    };
    let version = match env::var_os(ENVVAR_VERSION) {
        Some(v) => v.into_string().unwrap(),
        None => panic!("{} is not set.", ENVVAR_VERSION),
    };
    let api_key = match env::var_os(ENVVAR_API_KEY) {
        Some(v) => v.into_string().unwrap(),
        None => panic!("{} is not set.", ENVVAR_API_KEY),
    };

    Config {
        base_url,
        version,
        api_key,
    }
}
