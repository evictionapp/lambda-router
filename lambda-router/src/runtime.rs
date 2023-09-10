//! # Runtime
//!
//! The runtime of the macro routing system
//!
//! ## Modules
//! There are 3 modules for the runtime
//! - [error](crate::runtime::error)
//! - [json](crate::runtime::json)
//! - [try_route](crate::runtime::try_route)
//!
//! ## error
//! Handles errors for the runtime, mainly serialization / deserialization errors and routes not matching...
//!
//! ## json
//! A simple re-export for wrapping the serialization from serde_json
//!
//! ## try_route
//! the core routing logic, checks if routes and methods are equal, if so then try to deserialize inputs to types that the handlers expect.

pub mod error;
pub mod json;
pub mod try_route;
