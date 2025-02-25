//! # Dotenv+
//!
//! A dotenv solution for Rust.
//!
//! ## Quick Start
//!
//! Write the environment variables in the env files
//! and access them later using the `var` function:
//!
//! ```txt
//! KEY=value
//! ```
//!
//! ```no_run
//! use dotenv_plus::{
//!     DotEnv,
//!     var,
//! };
//!
//! DotEnv::new().run();
//!
//! assert_eq!(var("RUST_ENV"), "production");
//!
//! assert_eq!(var("KEY"), "value");
//! ```
//!
//! ```sh
//! # By default, `RUST_ENV` is set to `development`
//! RUST_ENV=production cargo run
//! ```

pub(crate) mod env;

pub(crate) mod var;

pub use env::*;

pub use var::*;
