struct Config {
    host: String,
    port: u16,
    max_connections: u32,
    timeout_secs: u64,
}

struct ConfigBuilder {
    host: String,
    port: u16,
    max_connections: u32,
    timeout_secs: u64,
}

impl ConfigBuilder {
    fn new() -> Self {
        Self {
            host: String::new(),      // required → validated in build()
            port: 80,                 // sensible default
            max_connections: 10,      // sensible default
            timeout_secs: 30,         // sensible default
        }
    }

    fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    fn max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    fn timeout_secs(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    fn build(self) -> Result<Config, String> {
        if self.host.is_empty() {
            return Err("host cannot be empty".to_string());
        }

        Ok(Config {
            host: self.host,
            port: self.port,
            max_connections: self.max_connections,
            timeout_secs: self.timeout_secs,
        })
    }
}

fn main() {
    let config = ConfigBuilder::new()
        .host("localhost")
        .port(8080)
        .max_connections(100)
        .timeout_secs(10)
        .build()
        .expect("Failed to build config");

    println!("Server: {}:{}", config.host, config.port);
    println!("Max connections: {}", config.max_connections);
    println!("Timeout: {}s", config.timeout_secs);
}