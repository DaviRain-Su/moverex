use std::fs;
use toml::Value;

#[derive(Debug, Clone)]
pub struct Config {
    pub net_type: String,
    pub main_rpc_endpoint: String,
    pub test_rpc_endpoint: String,
    pub dev_rpc_endpoint: String,
}

impl Config {
    pub fn new(net_type: String, main_rpc_endpoint: String) -> Config {
        Self {
            net_type,
            main_rpc_endpoint,
            test_rpc_endpoint: Default::default(),
            dev_rpc_endpoint: Default::default(),
        }
    }

    pub fn read_config(config_file: &str) -> Config {
        let toml_str = fs::read_to_string(config_file).expect("Failed to read config file");

        let config: Value = toml::from_str(&toml_str).expect("Failed to parse config file");
        let net_type = config["chain"]["net_type"]
            .as_str()
            .expect("Missing NET type in config file");
        let main_rpc_endpoint = config["chain"]["main_rpc_endpoint"]
            .as_str()
            .expect("Missing rpc_endpoint in config file");
        let test_rpc_endpoint = config["chain"]["test_rpc_endpoint"]
            .as_str()
            .expect("Missing rpc_endpoint in config file");
        let dev_rpc_endpoint = config["chain"]["dev_rpc_endpoint"]
            .as_str()
            .expect("Missing rpc_endpoint in config file");

        Config {
            net_type: net_type.into(),
            main_rpc_endpoint: main_rpc_endpoint.into(),
            test_rpc_endpoint: test_rpc_endpoint.into(),
            dev_rpc_endpoint: dev_rpc_endpoint.into(),
        }
    }

    pub fn default_config() -> Config {
        Config::read_config("config.toml")
    }
}
