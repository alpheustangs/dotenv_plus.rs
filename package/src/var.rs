use std::{
    env::{VarError, set_var as _set_var, var as _var, vars},
    ffi::OsStr,
};

/// Set multiple environment variable for the current process.
///
/// ## Example
///
/// ```no_run
/// use dotenv_plus::var::set_vars;
///
/// set_vars(vec![
///     ("key1", "value1"),
///     ("key2", "value2")
/// ]);
/// ```
pub fn set_vars(vars: Vec<(&str, &str)>) {
    vars.iter().for_each(|(k, v)| set_var(k, v));
}

/// Set environment variable for the current process.
///
/// ## Example
///
/// ```no_run
/// use dotenv_plus::var::set_var;
///
/// set_var("key", "value");
/// ```
pub fn set_var<K: AsRef<OsStr>, V: AsRef<OsStr>>(
    key: K,
    value: V,
) {
    unsafe { _set_var(key, value) };
}

/// Get all environment variables for the current process.
///
/// ## Example
///
/// ```no_run
/// use dotenv_plus::var::get_vars;
///
/// let vars: Vec<(String, String)> = get_vars();
/// ```
pub fn get_vars() -> Vec<(String, String)> {
    vars().collect()
}

/// Get environment variable,
/// and have to handle the error manually.
/// Use [`var`] if you want an automatic handling.
///
/// ## Example
///
/// ```no_run
/// use dotenv_plus::var::get_var;
///
/// let environment: String = get_var("RUST_ENV").unwrap();
/// ```
pub fn get_var<K: AsRef<OsStr>>(name: K) -> Result<String, VarError> {
    _var(name)
}

/// Get environment variable as string,
/// and panic if the environment variable is missing.
/// Use [`get_var`] if you want to handle the error manually.
///
/// ## Example
///
/// ```no_run
/// use dotenv_plus::var::var;
///
/// let environment: String = var("RUST_ENV");
/// ```
pub fn var<K: AsRef<OsStr>>(name: K) -> String {
    match _var(&name) {
        | Ok(val) => val,
        | Err(_) => panic!(
            "Failed to get environment variable: {}",
            name.as_ref()
                .to_str()
                .unwrap_or("Failed to get environment variable.")
        ),
    }
}
