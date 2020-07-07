# E114 core is the lowest layer in the e114 architecture
It provides the `WebResult` type to be returned by business logic functions.
It also contains the web framework connectors (feature gated) as well as in rust either the type or the trait should be in your crate in order to be able to implement it.

This crate is:
```rust
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
```

## Optional features
> `jsonschema` adds `#[derive(JsonSchema)]` to certain types and the `schemars` dependency.
> `actix_web` adds `actix-web` `From` impl for `WebResult` and the `actix-web` dependency.

## Minimum rust version
1.40

## Build, debug and release tools
- cargo fmt & cargo +1.40.0 clippy --tests --features actix_web,hyper,jsonschema
