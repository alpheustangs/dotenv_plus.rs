use std::{
    env::current_dir,
    fmt::Display,
    path::{Path, PathBuf},
};

use dotenvy::from_path_override as env_from;

use crate::var::set_var;

/// Rust Environment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Development,
    Test,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &str {
        match self {
            | Self::Development => "development",
            | Self::Test => "test",
            | Self::Production => "production",
        }
    }
}

impl Display for Environment {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Options for DotEnv initialization
#[derive(Debug, Clone)]
pub struct DotEnvOptions {
    dir: PathBuf,
    environment: String,
}

impl DotEnvOptions {
    /// Set the directory of the `.env` file.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use std::env::current_dir;
    /// use dotenv_plus::env::DotEnv;
    ///
    /// DotEnv::init()
    ///     .dir(current_dir().unwrap())
    ///     .done();
    /// ```
    pub fn dir<P: AsRef<Path>>(
        mut self,
        dir: P,
    ) -> Self {
        self.dir = dir.as_ref().to_path_buf();
        self
    }

    /// Set the environment.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use dotenv_plus::env::{DotEnv, Environment};
    ///
    /// DotEnv::init()
    ///     .environment(Environment::Development.as_str())
    ///     .done();
    /// ```
    pub fn environment<S: Into<String>>(
        mut self,
        environment: S,
    ) -> Self {
        self.environment = environment.into();
        self
    }

    /// Finish initialization.
    pub fn done(self) {
        // .env
        env_from(self.dir.join(".env")).ok();

        // .env.local
        env_from(self.dir.join(".env.local")).ok();

        // .env.<environment>
        env_from(self.dir.join(format!(".env.{}", self.environment))).ok();

        // .env.<environment>.local
        env_from(self.dir.join(format!(".env.{}.local", self.environment)))
            .ok();

        // RUST_ENV
        set_var("RUST_ENV", self.environment);
    }
}

/// Process will search for and load the `.env` file,
/// and `.env.xxx` file based on the environment.
/// For example, the `.env.production` file will be loaded
/// if the environment is `production`.
///
/// ## Example
///
/// ```no_run
/// use dotenv_plus::env::DotEnv;
///
/// DotEnv::init().done();
/// ```
#[derive(Debug, Clone)]
pub struct DotEnv;

impl DotEnv {
    pub fn init() -> DotEnvOptions {
        DotEnvOptions {
            dir: match current_dir() {
                | Ok(dir) => dir,
                | Err(_) => PathBuf::from("."),
            },
            environment: "development".to_string(),
        }
    }
}
