use std::env;

pub struct Config {
  pub access_token: String,
}

const ACCESS_TOKEN_ENV_VAR_NAME: &'static str = "ACCESS_TOKEN";

impl Config {
  pub fn load() -> Config {
    let access_token =
    env::var(ACCESS_TOKEN_ENV_VAR_NAME)
    .expect(&format!("Environment variable '{}' is not set. Store your access token in this environment variable. You can obtain your access token at https://www.patreon.com/portal/registration/register-clients", ACCESS_TOKEN_ENV_VAR_NAME));
    println!("Using access token stored in environment variable '{}'", ACCESS_TOKEN_ENV_VAR_NAME);

    Config { access_token }
  }
}