#[allow(warnings)]
pub mod api {
    include!(concat!(env!("OUT_DIR"), "/openapi_codegen.rs"));
}
pub mod access;
pub mod authentication;
pub mod controller;
pub mod parameter_context;
