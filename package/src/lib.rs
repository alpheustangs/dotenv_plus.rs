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
//! ```ignore
//! use dotenv_plus::env::{DotEnv, Environment};
//!
//! let env: Environment = if cfg!(feature = "prd") {
//!     Environment::Production
//! } else if cfg!(feature = "test") {
//!     Environment::Test
//! } else {
//!     Environment::Development
//! };
//!
//! DotEnv::new()
//!     .env(env.as_code())
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

/// Dotenv process related functions
pub mod env;

/// Read and write environment variables in the current process.
pub mod var;

/// Common functions
pub mod common;
