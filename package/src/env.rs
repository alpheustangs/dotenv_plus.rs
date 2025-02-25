use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use dotenvy::from_path_override as env_from;

use crate::var::set_var;

/// Rust Environment
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Environment {
    Development,
    Test,
    Production,
    Custom(String),
}

impl Environment {
    /// Get Environment from code.
    pub fn from_code<C: AsRef<str>>(code: C) -> Environment {
        let code: &str = code.as_ref();

        match code {
            | "development" => Self::Development,
            | "test" => Self::Test,
            | "production" => Self::Production,
            | _ => Self::Custom(code.to_string()),
        }
    }

    /// Get Environment code as `&str`.
    pub fn as_code(&self) -> &str {
        match self {
            | Self::Development => "development",
            | Self::Test => "test",
            | Self::Production => "production",
            | Self::Custom(code) => code.as_str(),
        }
    }

    /// Get Environment code as `String`.
    pub fn to_code(&self) -> String {
        self.as_code().to_string()
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
/// DotEnv::new().done();
/// ```
#[derive(Debug, Clone)]
pub struct DotEnv {
    dir: PathBuf,
    environment: String,
}

impl DotEnv {
    /// Create a new `DotEnv` struct.
    pub fn new() -> Self {
        Self {
            dir: match current_dir() {
                | Ok(dir) => dir,
                | Err(_) => PathBuf::from("."),
            },
            environment: "development".to_string(),
        }
    }

    /// Create a new `DotEnv` struct from an existing `DotEnv` struct.
    pub fn from(dotenv: DotEnv) -> Self {
        dotenv
    }

    /// Set the directory of the `.env` file.
    ///
    /// By default, the current directory is used
    /// as the directory of the `.env` file.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use std::env::current_dir;
    /// use dotenv_plus::env::DotEnv;
    ///
    /// DotEnv::new()
    ///     .directory(current_dir().unwrap())
    ///     .done();
    /// ```
    pub fn directory<Dir: AsRef<Path>>(
        mut self,
        dir: Dir,
    ) -> Self {
        self.dir = dir.as_ref().to_path_buf();
        self
    }

    /// Alias of `directory` function.
    pub fn dir<Dir: AsRef<Path>>(
        mut self,
        dir: Dir,
    ) -> Self {
        self.dir = dir.as_ref().to_path_buf();
        self
    }

    /// Set the environment.
    ///
    /// By default, the environment is `development`.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use dotenv_plus::env::{DotEnv, Environment};
    ///
    /// DotEnv::new()
    ///     .environment(Environment::Development.as_code())
    ///     .done();
    /// ```
    pub fn environment<S: Into<String>>(
        mut self,
        environment: S,
    ) -> Self {
        self.environment = environment.into();
        self
    }

    /// Alias of `environment` function.
    pub fn env<S: Into<String>>(
        mut self,
        environment: S,
    ) -> Self {
        self.environment = environment.into();
        self
    }

    /// Finish the initialization.
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

impl Default for DotEnv {
    fn default() -> Self {
        Self::new()
    }
}
