use std::env;

pub fn set_env(env: &str, value: &str) {
    env::set_var(env, value);
}

pub fn get_env(env: &str) -> String {
    env::var(env).unwrap_or(String::new())
}
