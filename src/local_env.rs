use std::{env, time::Duration};

use lazy_static::lazy_static;

/*
    Static variables that are initialized with the environment variables.
    lazy_static != static, (static => compile time, lazy_static => runtime)
*/
lazy_static! {
    pub static ref RUST_ENV: String = env::var("RUST_ENV").unwrap_or("info".to_string());
    pub static ref LOOP_SLEEP_DURATION: u64 = env::var("LOOP_SLEEP_DURATION").unwrap_or("2000".to_string()).parse().unwrap();
    pub static ref ROOMAPI_HOST: String = env::var("ROOMAPI_HOST").expect(&var_not_defined("ROOMAPI_HOST"));
    pub static ref ROOMAPI_PORT: String = env::var("ROOMAPI_PORT").expect(&var_not_defined("ROOMAPI_PORT"));
    pub static ref ROOM_URL: String = env::var("ROOM_URL").expect(&var_not_defined("ROOM_URL"));
    pub static ref X_API_KEY: String = env::var("X_API_KEY").expect(&var_not_defined("X_API_KEY"));
}

/*
    Use this function to get a nice error message when a variable is not defined.
*/
fn var_not_defined(var: &str) -> String {
    format!("{} is not defined in the environment", var)
}

/*
    Check if all variables are defined. If not, panic.
*/
pub fn check_vars() {
    lazy_static::initialize(&RUST_ENV); // don't panic if RUST_ENV is not defined
    lazy_static::initialize(&LOOP_SLEEP_DURATION);
    lazy_static::initialize(&ROOMAPI_HOST);
    lazy_static::initialize(&ROOMAPI_PORT);
    lazy_static::initialize(&ROOM_URL);
    lazy_static::initialize(&X_API_KEY);
}
