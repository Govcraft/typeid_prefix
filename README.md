# `TypeID` Prefix

[![Crates.io](https://img.shields.io/crates/v/typeid_prefix.svg)](https://crates.io/crates/typeid_prefix)
[![Documentation](https://docs.rs/typeid_prefix/badge.svg)](https://docs.rs/typeid_prefix)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

A Rust library that implements a type-safe version of the TypePrefix section of the [TypeID Specification](https://github.com/jetpack-io/typeid).

Combined with the [TypeIdSuffix crate](https://crates.io/crates/typeid_suffix) to comprise the [mti (Magic Type Id) crate](https://crates.io/crates/mti).

Use the [mti (Magic Type Id) crate](https://crates.io/crates/mti) for a holistic implementation of the TypeID specification.
## Features

- **Type-safe**: Ensures that `TypeID` prefixes conform to the specification.
- **Validation**: Provides robust validation for `TypeID` prefixes.
- **Sanitization**: Offers methods to clean and sanitize input strings into valid `TypeID` prefixes.
- **Zero-cost abstractions**: Designed to have minimal runtime overhead.
- **Optional tracing**: Integrates with the `tracing` crate for logging (optional feature).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
typeid_prefix = "1.0.0"
```

To enable tracing support, add:

```toml
[dependencies]
typeid_prefix = { version = "1.0.0", features = ["instrument"] }
```

## Usage

### Basic Usage

```rust
use typeid_prefix::{TypeIdPrefix, Sanitize};
use std::convert::TryFrom;

fn main() {
    // Create a TypeIdPrefix from a valid string
    let prefix = TypeIdPrefix::try_from("user").unwrap();
    println!("Valid prefix: {}", prefix);

    // Attempt to create from an invalid string
    let result = TypeIdPrefix::try_from("Invalid_Prefix");
    assert!(result.is_err());

    // Sanitize an invalid string
    let sanitized = "Invalid_Prefix123".sanitize_and_create();
    println!("Sanitized prefix: {}", sanitized);
}
```

### Validation

The `TypeIdPrefix` type ensures that all instances conform to the `TypeID` specification:

- Maximum length of 63 characters
- Contains only lowercase ASCII letters and underscores
- Does not start or end with an underscore
- Starts and ends with a lowercase letter
- An empty string is **not** considered a valid prefix

```rust
use typeid_prefix::TypeIdPrefix;
use std::convert::TryFrom;

fn validate_prefix(input: &str) {
    match TypeIdPrefix::try_from(input) {
        Ok(prefix) => println!("Valid prefix: {}", prefix),
        Err(e) => println!("Invalid prefix: {}", e),
    }
}

fn main() {
    validate_prefix("valid_prefix");
    validate_prefix("Invalid_Prefix");
    validate_prefix("_invalid");
    validate_prefix("toolong_toolong_toolong_toolong_toolong_toolong_toolong_toolong");
}
```

### Sanitization

The `Sanitize` trait provides a method to clean and create a valid `TypeIdPrefix` from any string:

```rust
use typeid_prefix::Sanitize;

fn main() {
    let sanitized = "Invalid String 123!@#".sanitize_and_create();
    println!("Sanitized: {}", sanitized); // Outputs: invalidstring
}
```

### Optional Tracing

When the `instrument` feature is enabled, the crate will log validation errors using the `tracing` crate:

```toml
[dependencies]
typeid_prefix = { version = "1.0.0", features = ["instrument"] }
```

```rust
use typeid_prefix::Sanitize;

fn main() {
    // Set up your tracing subscriber here
    let _sanitized = "Invalid_Prefix!@#".sanitize_and_create();
    // Validation errors will be logged via tracing
}
```

## Use Cases

- **Database Systems**: Use `TypeIdPrefix` to ensure consistent and valid type prefixes for database schemas or ORM mappings.
- **API Development**: Validate and sanitize user input for API endpoints that require type prefixes.
- **Code Generation**: Generate valid type prefixes for code generation tools or macros.
- **Configuration Management**: Ensure configuration keys or identifiers conform to a consistent format.

## Safety and Correctness

This crate has been thoroughly tested and verified:

- Comprehensive unit tests
- Property-based testing with `proptest`
- Fuzz testing
- Formal verification with Kani

These measures ensure that the crate behaves correctly and never panics under normal usage.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on Rust 1.60.0 and later.

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Credits

This crate implements a portion of the [TypeID Specification](https://github.com/jetpack-io/typeid) created by Jetpack.io.