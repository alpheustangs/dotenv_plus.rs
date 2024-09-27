use crate::internal::utils::var::get_var;

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
pub fn is_dev() -> bool {
    get_rust_env() == "development"
}

/// Whether the current environment(`RUST_ENV`) is `test`.
pub fn is_test() -> bool {
    get_rust_env() == "test"
}

/// Whether the current environment(`RUST_ENV`) is `production`.
pub fn is_prd() -> bool {
    get_rust_env() == "production"
}
