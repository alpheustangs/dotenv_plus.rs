use std::{env::current_dir, path::PathBuf};

use dotenvy::from_path_override as env_from;

use crate::{get_var, set_var};

/// An environment variables initializer.
///
/// Loads environment variables from env files in the specified directory.
/// After initialization, `RUST_ENV` is set to the specified environment,
/// and `var("RUST_ENV")` can be used to retrieve it.
///
/// The following files are loaded in order,
/// with variables in later files overriding those in earlier ones:
///
/// 1. `.env`
/// 2. `.env.local`
/// 3. `.env.<environment>`
/// 4. `.env.<environment>.local`
///
/// ## Example
///
/// ```no_run
/// use dotenv_plus::DotEnv;
///
/// DotEnv::new().run();
/// ```
#[derive(Debug, Clone)]
pub struct DotEnv {
    dir: PathBuf,
    env: String,
}

impl DotEnv {
    /// Creates a new instance.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use dotenv_plus::DotEnv;
    ///
    /// DotEnv::new().run();
    /// ```
    pub fn new() -> Self {
        Self {
            dir: match current_dir() {
                | Ok(dir) => dir,
                | Err(_) => PathBuf::from("."),
            },
            env: match get_var("RUST_ENV") {
                | Ok(env) => env,
                | Err(_) => "development".to_string(),
            },
        }
    }

    /// Creates a new instance from an existing one.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use dotenv_plus::DotEnv;
    ///
    /// let prev: DotEnv = DotEnv::new();
    ///
    /// DotEnv::from(prev).run();
    /// ```
    pub fn from(dotenv: DotEnv) -> Self {
        dotenv
    }

    /// Sets the directory containing the env files.
    ///
    /// By default, it uses[`std::env::current_dir`].
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use std::env::current_dir;
    ///
    /// use dotenv_plus::DotEnv;
    ///
    /// DotEnv::new()
    ///     .directory(current_dir().unwrap())
    ///     .run();
    /// ```
    pub fn directory<Dir: Into<PathBuf>>(
        mut self,
        dir: Dir,
    ) -> Self {
        self.dir = dir.into();
        self
    }

    /// Alias for [`DotEnv::directory`] function.
    pub fn dir<Dir: Into<PathBuf>>(
        mut self,
        dir: Dir,
    ) -> Self {
        self.dir = dir.into();
        self
    }

    /// Set the environment.
    ///
    /// By default, it checks the `RUST_ENV` environment variable.
    /// If it is missing, it defaults to `development`.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use dotenv_plus::DotEnv;
    ///
    /// DotEnv::new()
    ///     .environment("test")
    ///     .run();
    /// ```
    pub fn environment<S: Into<String>>(
        mut self,
        env: S,
    ) -> Self {
        self.env = env.into();
        self
    }

    /// Alias for [`DotEnv::environment`] function.
    pub fn env<S: Into<String>>(
        mut self,
        env: S,
    ) -> Self {
        self.env = env.into();
        self
    }

    /// Loads environment variables and finalizes initialization.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use dotenv_plus::DotEnv;
    ///
    /// DotEnv::new().run();
    /// ```
    pub fn run(self) {
        let files = vec![
            // .env
            self.dir.join(".env"),
            // .env.local
            self.dir.join(".env.local"),
            // .env.<environment>
            self.dir.join(format!(".env.{}", self.env)),
            // .env.<environment>.local
            self.dir.join(format!(".env.{}.local", self.env)),
        ];

        for file in files {
            env_from(file).ok();
        }

        // RUST_ENV
        set_var("RUST_ENV", self.env);
    }
}

impl Default for DotEnv {
    fn default() -> Self {
        Self::new()
    }
}
