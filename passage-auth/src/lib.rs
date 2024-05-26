#![allow(unused_imports)]
#![allow(dead_code)]
//!
//! This crate provides a library for working with [Passage by 1Password](https://passage.1password.com),
//! a modern passwordless authentication experience based on passkeys.
//!
//! See Passage [Authentication API](https://docs.passage.id/api-docs/authentication-api).
//!
//! ## Usage
//!
//! This crate is published on [crates.io](https://crates.io/crates/passage-id)
//! and can be added with `cargo add passage-id` or by manually adding
//! `passage-id` to your Cargo.toml dependencies.
//!
//! ```toml
//! [dependencies]
//! passage-id = "0.2"
//! ```

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

mod apis;
mod config;
mod error;
mod models;
mod passage;

pub use config::Config;
pub use error::*;
pub use passage::Passage;
