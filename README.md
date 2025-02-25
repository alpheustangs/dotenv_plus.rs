# Dotenv+

A dotenv solution for Rust.

## Installation

To install this package, run the following command:

```bash
cargo add dotenv_plus
```

## Quick Start

Write the environment variables in the env files and access them later using the `var` function:

```
KEY=value
```

```rust
use dotenv_plus::{
    DotEnv,
    var,
};

DotEnv::new().run();

assert_eq!(var("RUST_ENV"), "production");

assert_eq!(var("KEY"), "value");
```

```sh
# By default, `RUST_ENV` is set to `development`
RUST_ENV=production cargo run
```

## License

This project is licensed under the terms of the MIT license.
