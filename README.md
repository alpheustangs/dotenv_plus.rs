# Dotenv+

A dotenv extension for Rust.

## Installation

To install this package, run the following command:

```bash
cargo add dotenv_plus
```

## Quick Start

Initialize the environment variables and get different variables with the following code:

```rust
use dotenv_plus::{
    env::DotEnv,
    common::get_rust_env,
    var::{set_var, var},
};

DotEnv::new().done();

assert_eq!(get_rust_env(), "development");

set_env("key", "value");

assert_eq!(var("key"), "value");
```

## License

This project is licensed under the terms of the MIT license.
