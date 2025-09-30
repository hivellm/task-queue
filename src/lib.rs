//! Task Queue Library
//!
//! A comprehensive task queue system with workflow management, dependency tracking,
//! and MCP (Model Context Protocol) integration.

pub mod cache;
pub mod client;
pub mod config;
pub mod core;
pub mod error;
pub mod logging;
pub mod mcp;
pub mod metrics;
pub mod rate_limiting;
pub mod server;
pub mod storage;
pub mod vectorizer;
pub mod websocket;

// Re-export main types for convenience
pub use core::*;
pub use error::{TaskQueueError, Result};
pub use server::TaskQueueServer;
