# E114 core is the lowest layer in the e114 architecture
It provides the `WebResult` type to be returned by business logic functions.
It also contains the web framework connectors (feature gated) as well as in rust either the type or the trait should be in your crate in order to be able to implement it.

This crate is:
```rust
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
```

# ☠☡☣☢ This crate is in rapid flux, do not rely on it just yet! ☢☣☡☠
You have been warned.

## Optional features
```toml
e114_core = { version = "0.1.0", features = ["jsonschema"] }
```
Adds `#[derive(JsonSchema)]` to certain types and the `schemars` dependency.

```toml
e114_core = { version = "0.1.0", features = ["actix_web"] }
```
Adds `actix-web` `From` impl for `WebResult` and the `actix-web` dependency.

```toml
e114_core = { version = "0.1.0", features = ["hyper_body"] }
```
Adds `http::Response<hyper::Body>` impl for `WebResult` and the `hyper` dependency.
This is intended to be used by `warp` and other frameworks which are built on `http` and `hyper`.

## Minimum rust version
1.40

## Build, debug and release tools
- cargo fmt & cargo +1.45.2 clippy --tests --features actix_web,hyper_body,jsonschema
