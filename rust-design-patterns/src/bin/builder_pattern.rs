//! Builder pattern — separates construction from representation, enforces
//! required fields at *compile time* via type-state (see type_state_pattern),
//! or at *runtime* via `build()` validation as shown here.

use thiserror::Error;

// ── Error type ───────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("host cannot be empty")]
    EmptyHost,
    #[error("port {0} is reserved (must be > 1023)")]
    ReservedPort(u16),
}

// ── Domain types ─────────────────────────────────────────────────────────────

/// Validated, immutable server configuration.
#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub max_connections: u32,
    pub timeout_secs: u64,
}

/// Fluent builder for [`Config`].
///
/// Call [`ConfigBuilder::new`], chain setters, then call [`ConfigBuilder::build`].
/// Fields not explicitly set use sensible defaults.
///
/// # Example
/// ```
/// let cfg = ConfigBuilder::new()
///     .host("localhost")
///     .port(8080)
///     .build()
///     .unwrap();
/// assert_eq!(cfg.host, "localhost");
/// ```
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    host: String,
    port: u16,
    max_connections: u32,
    timeout_secs: u64,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            host: String::new(),
            port: 80,
            max_connections: 10,
            timeout_secs: 30,
        }
    }

    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_owned();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    pub fn timeout_secs(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// Validates inputs and returns a [`Config`], or a [`ConfigError`].
    pub fn build(self) -> Result<Config, ConfigError> {
        if self.host.is_empty() {
            return Err(ConfigError::EmptyHost);
        }
        if self.port < 1024 {
            return Err(ConfigError::ReservedPort(self.port));
        }
        Ok(Config {
            host: self.host,
            port: self.port,
            max_connections: self.max_connections,
            timeout_secs: self.timeout_secs,
        })
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    let config = ConfigBuilder::new()
        .host("localhost")
        .port(8080)
        .max_connections(100)
        .timeout_secs(10)
        .build()
        .expect("valid config");

    println!("Server : {}:{}", config.host, config.port);
    println!("Max conns : {}", config.max_connections);
    println!("Timeout   : {}s", config.timeout_secs);
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_with_valid_inputs_succeeds() {
        let cfg = ConfigBuilder::new()
            .host("example.com")
            .port(8443)
            .build()
            .unwrap();
        assert_eq!(cfg.host, "example.com");
        assert_eq!(cfg.port, 8443);
        assert_eq!(cfg.max_connections, 10); // default
        assert_eq!(cfg.timeout_secs, 30);    // default
    }

    #[test]
    fn empty_host_returns_error() {
        let err = ConfigBuilder::new().port(8080).build().unwrap_err();
        assert!(matches!(err, ConfigError::EmptyHost));
    }

    #[test]
    fn reserved_port_returns_error() {
        let err = ConfigBuilder::new()
            .host("localhost")
            .port(80) // <= 1023 — reserved
            .build()
            .unwrap_err();
        assert!(matches!(err, ConfigError::ReservedPort(80)));
    }

    #[test]
    fn error_messages_are_human_readable() {
        assert_eq!(ConfigError::EmptyHost.to_string(), "host cannot be empty");
        assert_eq!(
            ConfigError::ReservedPort(22).to_string(),
            "port 22 is reserved (must be > 1023)"
        );
    }
}
