use crate::{env::Environment, var::get_var};

/// Get the current environment with `RUST_ENV`.
pub fn get_rust_env() -> String {
    match get_var("RUST_ENV") {
        | Ok(var) => var,
        | Err(_) => panic!(
            "RUST_ENV is not set, please initialize it with dotenv_plus::env::DotEnv or set it manually."
        ),
    }
}

/// Whether the current environment(`RUST_ENV`) is `development`.
pub fn is_environment_development() -> bool {
    get_rust_env() == Environment::Development.to_string()
}

/// Alias of [`is_environment_development`].
pub fn is_dev() -> bool {
    is_environment_development()
}

/// Whether the current environment(`RUST_ENV`) is `test`.
pub fn is_environment_test() -> bool {
    get_rust_env() == Environment::Test.to_string()
}

/// Alias of [`is_environment_test`].
pub fn is_test() -> bool {
    is_environment_test()
}

/// Whether the current environment(`RUST_ENV`) is `production`.
pub fn is_environment_production() -> bool {
    get_rust_env() == Environment::Production.to_string()
}

/// Alias of [`is_environment_production`].
pub fn is_prd() -> bool {
    is_environment_production()
}
