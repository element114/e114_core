# E114 core is the lowest layer in the e114 architecture
It provides the `WebResult` type to be returned by business logic functions.
It provides standardized error structs based on `JSONAPI#Error` format.

It also contains the web framework connectors (feature gated), because in rust either the type or the trait should be in your crate in order to be able to implement it.

This crate is:
```rust
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
```

# This crate implements JSONAPI#Error format
https://jsonapi.org/format/#errors

## Optional features
```toml
e114_core = { version = "0.3.0", features = ["jsonschema"] }
```
Adds `#[derive(JsonSchema)]` to certain types and the `schemars` dependency.

```toml
e114_core = { version = "0.3.0", features = ["actix_web"] }
```
Adds `actix-web` `From` impl for `WebResult` and the `actix-web` dependency.

```toml
e114_core = { version = "0.3.0", features = ["hyper_body"] }
```
Adds `http::Response<hyper::Body>` impl for `WebResult` and the `hyper` dependency.
This is intended to be used by `warp` and other frameworks which are built on `http` and `hyper`.

## Minimum rust version
1.50.0

## Build, debug and release tools
- cargo fmt & cargo +1.50.0 clippy --tests --features actix_web,hyper_body,jsonschema
