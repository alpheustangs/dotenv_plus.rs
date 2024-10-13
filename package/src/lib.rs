//! # Dotenv+
//!
//! A dotenv extension for Rust.
//!
//! ## Define Different Environments
//!
//! To define different environments,
//! you may use the `features` attribute in `Cargo.toml`:
//!
//! ```toml
//! [features]
//! dev = []
//! test = []
//! prd = []
//! ```
//!
//! Then initialize the environment variables with the following code:
//!
//! ```no_run
//! use dotenv_plus::env::{DotEnv, Environment};
//!
//! let environment: Environment = if cfg!(feature = "prd") {
//!     Environment::Production
//! } else if cfg!(feature = "test") {
//!     Environment::Test
//! } else {
//!     Environment::Development
//! };
//!
//! DotEnv::init()
//!     .environment(environment.as_str())
//!     .done();
//! ```
//!
//! And run the process with the environment planned to be used:
//!
//! ```bash
//! cargo run --features dev
//! ```
//!
//! You may setup and read different environment variables
//! with `set_var`, `get_vars`, `get_var` and `var`:
//!
//! ```no_run
//! use dotenv_plus::var::{set_var, var};
//!
//! set_var("key", "value");
//! assert_eq!(var("key"), "value");
//! ```

mod internal;

/// Read and write environment variables in the current process.
pub mod var {
    pub use crate::internal::utils::var::{get_var, get_vars, set_var, var};
}

/// Dotenv process related functions
pub mod env {
    pub use crate::internal::utils::env::{DotEnv, DotEnvOptions, Environment};
}

/// Common functions
pub mod common {
    // base
    pub use crate::internal::common::get_rust_env;

    // environment
    pub use crate::internal::common::{
        is_environment_development, is_environment_production,
        is_environment_test,
    };

    // environment aliases
    pub use crate::internal::common::{is_dev, is_prd, is_test};
}
