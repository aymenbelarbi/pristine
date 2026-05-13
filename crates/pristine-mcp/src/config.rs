//! MCP server configuration

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// MCP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    /// Server name
    pub name: String,
    /// Server version
    pub version: String,
    /// Transport type
    pub transport: McpTransport,
    /// Maximum concurrent requests
    pub max_concurrent_requests: usize,
    /// Request timeout
    pub request_timeout: Duration,
    /// Enable tools
    pub enable_tools: bool,
    /// Enable resources
    pub enable_resources: bool,
    /// Enable prompts
    pub enable_prompts: bool,
}

impl Default for McpServerConfig {
    fn default() -> Self {
        Self {
            name: "pristine".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            transport: McpTransport::Stdio,
            max_concurrent_requests: 10,
            request_timeout: Duration::from_secs(30),
            enable_tools: true,
            enable_resources: true,
            enable_prompts: true,
        }
    }
}

/// MCP transport type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum McpTransport {
    Stdio,
    Http {
        host: String,
        port: u16,
    },
    WebSocket {
        host: String,
        port: u16,
    },
}
