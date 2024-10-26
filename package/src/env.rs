use std::{
    env::current_dir,
    io,
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
    /// Get Environment from code.
    pub fn from_code<C: AsRef<str>>(code: C) -> io::Result<Environment> {
        match code.as_ref() {
            | "development" => Ok(Self::Development),
            | "test" => Ok(Self::Test),
            | "production" => Ok(Self::Production),
            | _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid environment code.",
            )),
        }
    }

    /// Get Environment code as `&str`.
    pub fn as_code(&self) -> &str {
        match self {
            | Self::Development => "development",
            | Self::Test => "test",
            | Self::Production => "production",
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

    /// Create a new `DotEnv` struct.
    #[deprecated(since = "0.4.0", note = "please use `new()` instead")]
    pub fn init() -> Self {
        Self::new()
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
    /// DotEnv::init()
    ///     .dir(current_dir().unwrap())
    ///     .done();
    /// ```
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
    /// DotEnv::init()
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
