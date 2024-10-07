use std::env::current_dir;

use dotenv_plus::{
    common::get_rust_env,
    env::{DotEnv, Environment},
    var::var,
};

fn main() {
    let environment: Environment = if cfg!(feature = "prd") {
        Environment::Production
    } else if cfg!(feature = "test") {
        Environment::Test
    } else {
        Environment::Development
    };

    DotEnv::init()
        .dir(current_dir().unwrap())
        .environment(environment.to_string())
        .done();

    println!("Rust environment: {}", get_rust_env());
    println!("A: {}", var("A"));
    println!("B: {}", var("B"));
    println!("C: {}", var("C"));
}

#[cfg(test)]
mod tests {
    use std::env::current_dir;

    use dotenv_plus::{
        common::{
            get_rust_env, is_dev, is_environment_development,
            is_environment_production, is_environment_test, is_prd, is_test,
        },
        env::{DotEnv, Environment},
        var::{get_var, var},
    };

    #[test]
    fn main() {
        DotEnv::init()
            .dir(current_dir().unwrap())
            .environment(Environment::Test.to_string())
            .done();

        assert_eq!(get_rust_env(), "test");
        assert_eq!(is_environment_development(), false);
        assert_eq!(is_dev(), false);
        assert_eq!(is_environment_test(), true);
        assert_eq!(is_test(), true);
        assert_eq!(is_environment_production(), false);
        assert_eq!(is_prd(), false);

        assert_eq!(get_var("A").unwrap(), ".env");
        assert_eq!(var("A"), ".env");
        assert_eq!(get_var("B").unwrap(), ".env.test");
        assert_eq!(var("B"), ".env.test");
        assert_eq!(get_var("C").unwrap(), ".env.test");
        assert_eq!(var("C"), ".env.test");
    }
}
