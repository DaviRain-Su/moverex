use std::fs;
use toml::Value;

#[derive(Debug, Clone)]
pub struct Config {
    pub net_type: String,
    pub rpc_endpoint: String,
}

impl Config {
    pub fn new(net_type: String, rpc_endpoint: String) -> Config {
        Self {
            net_type, 
            rpc_endpoint,
        }
    }

    pub fn read_config(config_file: &str) -> Config {
        let toml_str = fs::read_to_string(config_file).expect("Failed to read config file");
    
        let config: Value = toml::from_str(&toml_str).expect("Failed to parse config file");
        let net_type = config["ether"]["net_type"].as_str().expect("Missing NET type in config file");
        let rpc_endpoint = config["ether"]["rpc_endpoint"].as_str().expect("Missing rpc_endpoint in config file");

        Config::new(net_type.into(), rpc_endpoint.into())
    }

    pub fn default_config() -> Config {
        Config::read_config("config.toml")
    }
}