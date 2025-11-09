//! # Nifi-rs
//!
//! NiFi bindings for Rust
//! 
//!


pub mod api {
    include!(concat!(env!("OUT_DIR"), "/openapi_codegen.rs"));
}

pub mod common;
pub mod access;
pub mod authentication;
pub mod parameter_context;
pub mod controller;
