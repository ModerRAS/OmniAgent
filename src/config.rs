use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub llm: LLMConfig,
    pub mcp: McpConfig,
    pub a2a: A2AConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub cors_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub provider: String, // "openai", "claude", "google"
    pub model: String,
    pub api_key: String,
    pub base_url: Option<String>,
    pub temperature: f32,
    pub max_tokens: u32,
    pub use_mock: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    pub servers: HashMap<String, McpServerConfig>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    pub name: String,
    pub description: String,
    pub url: String,
    pub timeout: u64,
    pub retry_attempts: u32,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2AConfig {
    pub servers: HashMap<String, A2AServerConfig>,
    pub enabled: bool,
    pub allow_external: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2AServerConfig {
    pub name: String,
    pub description: String,
    pub url: String,
    pub auth_token: Option<String>,
    pub timeout: u64,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub file: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                port: 8080,
                host: "0.0.0.0".to_string(),
                cors_origins: vec!["*".to_string()],
            },
            llm: LLMConfig {
                provider: "claude".to_string(),
                model: "claude-3-haiku-20240307".to_string(),
                api_key: "YOUR_API_KEY".to_string(),
                base_url: None,
                temperature: 0.7,
                max_tokens: 1000,
                use_mock: true,
            },
            mcp: McpConfig {
                servers: HashMap::new(),
                enabled: true,
            },
            a2a: A2AConfig {
                servers: HashMap::new(),
                enabled: true,
                allow_external: true,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
                file: None,
            },
        }
    }
}

impl AppConfig {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: AppConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn load_from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path =
            std::env::var("OMNI_AGENT_CONFIG").unwrap_or_else(|_| "config.json".to_string());

        if std::path::Path::new(&config_path).exists() {
            Self::load_from_file(&config_path)
        } else {
            let config = AppConfig::default();
            config.save_to_file(&config_path)?;
            println!("Created default config file: {}", config_path);
            Ok(config)
        }
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn override_with_env(&mut self) {
        if let Ok(port) = std::env::var("PORT") {
            if let Ok(port_num) = port.parse() {
                self.server.port = port_num;
            }
        }

        if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
            self.llm.provider = "openai".to_string();
            self.llm.api_key = api_key;
        }

        if let Ok(api_key) = std::env::var("ANTHROPIC_API_KEY") {
            self.llm.provider = "claude".to_string();
            self.llm.api_key = api_key;
        }

        if let Ok(api_key) = std::env::var("GOOGLE_API_KEY") {
            self.llm.provider = "google".to_string();
            self.llm.api_key = api_key;
        }

        if let Ok(level) = std::env::var("LOG_LEVEL") {
            self.logging.level = level;
        }

        if let Ok(mock) = std::env::var("USE_MOCK") {
            self.llm.use_mock = mock == "true" || mock == "1";
        }
    }
}
