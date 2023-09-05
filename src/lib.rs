#![forbid(unsafe_code)]
#![allow(clippy::result_large_err)]
pub mod authentication;
pub mod controller;
pub mod database;
pub mod entities;
pub mod error;
pub mod launch;
pub mod route;
pub mod session_state;
pub mod telemetry;
pub mod template_engine;
pub mod utils;
