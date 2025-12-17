//! Shared RPC API definitions and types for FastEVM consensus
//!
//! This crate provides independent RPC API definitions and consensus-related types
//! that can be used across different projects. It has minimal external dependencies
//! to ensure maximum compatibility and independence.

pub mod api;
pub mod types;

// Re-export commonly used types
pub use api::*;
pub use types::*;
