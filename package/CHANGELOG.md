## 0.5.0 (2025-02-25)

### Breaking Changes

- Remove `common` module
- Remove `Environment` enum
- `env` and `var` module move to the top level
- `DotEnv` function rename:
    - `done` -> `run`

### What's New

- Add `from` function to `DotEnv`
- Add `set_vars` function to set multiple environment variables

### What's Changed

- Update Rust edition to `2024`
- Remove `init` function (deprecated)

## 0.4.0 (2024-10-26)

### Breaking Changes

- `Environment` enum use `as_code` instead of `as_str` now
- `Environment` enum use `to_code` instead of `to_string` now
- `DotEnv` struct use `new()` instead of `init()` now

### What's New

- Add `from_code` function to `Environment` enum

### What's Changed

- `dir` in `DotEnvOptions` struct now accept more types of input
- `environment` in `DotEnvOptions` struct now accept more types of input
- `get_var` function now accept more types of input
- `var` function now accept more types of input

## 0.3.0 (2024-10-13)

### Breaking Changes

- Changes in accepted value type of `environment` in `DotEnv::init()`:
    - `String` => `&str`

### What's New

- Add different derives for different structs
- Add `as_str` function for `Environment` enum

### What's Changed

- Updates in documentation

## 0.2.0 (2024-10-07)

### What's New

- Add functions:
    - `is_environment_development`
    - `is_environment_test`
    - `is_environment_production`

### What's Changed

- Updates in documentation

## 0.1.0 (2024-09-27)

First release
