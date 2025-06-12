//! Configuration management

#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub cors: CorsConfig,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub environment: String,
    pub request_timeout_seconds: u64,
    pub max_request_size: usize,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub postgres_url: String,
    pub postgres_max_connections: u32,
    pub clickhouse_url: String,
    pub clickhouse_user: String,
    pub clickhouse_password: String,
    pub clickhouse_database: String,
}

#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub api_key_header: String,
}

#[derive(Debug, Clone)]
pub struct CorsConfig {
    pub origins: Vec<String>,
}

impl Config {
    /// Load configuration from environment variables
    pub fn load() -> anyhow::Result<Self> {
        // Load .env file if it exists
        let _ = dotenvy::dotenv();

        let server = ServerConfig {
            host: std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            environment: std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()),
            request_timeout_seconds: std::env::var("REQUEST_TIMEOUT_SECONDS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            max_request_size: std::env::var("MAX_REQUEST_SIZE")
                .unwrap_or_else(|_| "10485760".to_string())
                .parse()
                .unwrap_or(10485760),
        };

        let database = DatabaseConfig {
            postgres_url: std::env::var("POSTGRES_URL").unwrap_or_else(|_| {
                "postgresql://fusegu_user:fusegu_pass@localhost:5432/fusegu_dev".to_string()
            }),
            postgres_max_connections: std::env::var("POSTGRES_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
            clickhouse_url: std::env::var("CLICKHOUSE_URL")
                .unwrap_or_else(|_| "http://localhost:8123".to_string()),
            clickhouse_user: std::env::var("CLICKHOUSE_USER")
                .unwrap_or_else(|_| "fusegu_analytics".to_string()),
            clickhouse_password: std::env::var("CLICKHOUSE_PASSWORD")
                .unwrap_or_else(|_| "fusegu_analytics_pass".to_string()),
            clickhouse_database: std::env::var("CLICKHOUSE_DATABASE")
                .unwrap_or_else(|_| "fusegu_events".to_string()),
        };

        let auth = AuthConfig {
            jwt_secret: std::env::var("JWT_SECRET").unwrap_or_else(|_| {
                "your-256-bit-secret-key-here-replace-in-production".to_string()
            }),
            api_key_header: std::env::var("API_KEY_HEADER")
                .unwrap_or_else(|_| "X-API-Key".to_string()),
        };

        let cors_origins = std::env::var("CORS_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000,http://localhost:8080".to_string());
        let cors = CorsConfig {
            origins: cors_origins
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        };

        Ok(Config {
            server,
            database,
            auth,
            cors,
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                environment: "development".to_string(),
                request_timeout_seconds: 30,
                max_request_size: 10485760, // 10MB
            },
            database: DatabaseConfig {
                postgres_url: "postgresql://fusegu_user:fusegu_pass@localhost:5432/fusegu_dev"
                    .to_string(),
                postgres_max_connections: 10,
                clickhouse_url: "http://localhost:8123".to_string(),
                clickhouse_user: "fusegu_analytics".to_string(),
                clickhouse_password: "fusegu_analytics_pass".to_string(),
                clickhouse_database: "fusegu_events".to_string(),
            },
            auth: AuthConfig {
                jwt_secret: "your-256-bit-secret-key-here-replace-in-production".to_string(),
                api_key_header: "X-API-Key".to_string(),
            },
            cors: CorsConfig {
                origins: vec![
                    "http://localhost:3000".to_string(), // Frontend dev server
                    "http://localhost:8080".to_string(), // API server (for testing)
                ],
            },
        }
    }
}
