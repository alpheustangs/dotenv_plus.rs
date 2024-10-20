use std::{env::current_dir, fmt::Display, path::PathBuf};

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

#[derive(Debug, Clone)]
struct DotEnvState<'a> {
    dir: PathBuf,
    environment: &'a str,
}

/// Options for DotEnv initialization
#[derive(Debug, Clone)]
pub struct DotEnvOptions<'a> {
    state: DotEnvState<'a>,
}

impl<'a> DotEnvOptions<'a> {
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
    pub fn dir(
        mut self,
        dir: PathBuf,
    ) -> Self {
        self.state.dir = dir;
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
    pub fn environment(
        mut self,
        environment: &'a str,
    ) -> Self {
        self.state.environment = environment;
        self
    }

    /// Finish initialization.
    pub fn done(self) {
        // .env
        env_from(self.state.dir.join(".env")).ok();

        // .env.local
        env_from(self.state.dir.join(".env.local")).ok();

        // .env.<environment>
        env_from(
            self.state.dir.join(format!(".env.{}", self.state.environment)),
        )
        .ok();

        // .env.<environment>.local
        env_from(
            self.state
                .dir
                .join(format!(".env.{}.local", self.state.environment)),
        )
        .ok();

        // RUST_ENV
        set_var("RUST_ENV", self.state.environment);
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
    pub fn init<'a>() -> DotEnvOptions<'a> {
        DotEnvOptions {
            state: DotEnvState {
                dir: current_dir().unwrap(),
                environment: "development",
            },
        }
    }
}
