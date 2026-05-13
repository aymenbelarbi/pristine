//! Pristine MCP Server
//!
//! Model Context Protocol server for Pristine, exposing repository
//! context compilation capabilities to AI assistants and coding agents.

pub mod server;
pub mod tools;
pub mod config;

pub use server::PristineMcpServer;
pub use config::McpServerConfig;
