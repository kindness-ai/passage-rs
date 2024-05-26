#![allow(unused_imports)]
#![allow(dead_code)]
//!
//! This crate provides a library for working with [Passage by 1Password](https://passage.1password.com),
//! a modern passwordless authentication experience based on passkeys.
//!
//! See Passage [Authentication API](https://docs.passage.id/api-docs/authentication-api).

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

mod apis;
mod config;
mod error;
mod passage;

pub mod models;
pub use config::Config;
pub use error::*;
pub use passage::Passage;
