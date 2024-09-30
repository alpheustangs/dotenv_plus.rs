# Dotenv+

A dotenv extension for Rust.

## Quick Start

Initialize the environment variables and get different variables with the following code:

```rust
use dotenv_plus::{
    env::DotEnv,
    common::get_rust_env,
    var::{set_var, var},
};

DotEnv::init().done();
assert_eq!(get_rust_env(), "development");
set_var("key", "value");
assert_eq!(var("key"), "value");
```

## License

This project is MIT licensed, you can find the license file [here](https://github.com/alpheustangs/dotenv_plus.rs/blob/main/LICENSE).
