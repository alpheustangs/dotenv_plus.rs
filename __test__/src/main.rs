use dotenv_plus::{DotEnv, var};

fn main() {
    DotEnv::new().run();

    println!("Rust environment: {}", var("RUST_ENV"));
    println!("A: {}", var("A"));
    println!("B: {}", var("B"));
    println!("C: {}", var("C"));
}

#[cfg(test)]
mod tests {
    use dotenv_plus::{DotEnv, get_var, set_var, set_vars, var};

    #[test]
    fn test_env() {
        // development

        DotEnv::new().run();

        assert_eq!(var("RUST_ENV"), "development");

        assert_eq!(get_var("A").unwrap(), ".env");
        assert_eq!(var("A"), ".env");

        assert_eq!(get_var("B").unwrap(), ".env.development");
        assert_eq!(var("B"), ".env.development");

        assert_eq!(get_var("C").unwrap(), ".env.development");
        assert_eq!(var("C"), ".env.development");

        // test

        DotEnv::new().environment("test").run();

        assert_eq!(var("RUST_ENV"), "test");

        assert_eq!(get_var("A").unwrap(), ".env");
        assert_eq!(var("A"), ".env");

        assert_eq!(get_var("B").unwrap(), ".env.test");
        assert_eq!(var("B"), ".env.test");

        assert_eq!(get_var("C").unwrap(), ".env.test");
        assert_eq!(var("C"), ".env.test");
    }

    #[test]
    fn test_set_vars_get_var() {
        set_vars(vec![
            ("__TEST_SET_VARS_1", "abc1"),
            ("__TEST_SET_VARS_2", "abc2"),
        ]);

        assert_eq!(get_var("__TEST_SET_VARS_1").unwrap(), "abc1");
        assert_eq!(var("__TEST_SET_VARS_1"), "abc1");

        assert_eq!(get_var("__TEST_SET_VARS_2").unwrap(), "abc2");
        assert_eq!(var("__TEST_SET_VARS_2"), "abc2");
    }

    #[test]
    fn test_set_var_get_var() {
        set_var("__TEST_SET_VAR", "abc");

        assert_eq!(get_var("__TEST_SET_VAR").unwrap(), "abc");
        assert_eq!(var("__TEST_SET_VAR"), "abc");
    }

    #[test]
    fn test_get_var_error() {
        if let Ok(_) = get_var("__TEST_GET_VAR_ERROR") {
            panic!();
        }
    }
}
