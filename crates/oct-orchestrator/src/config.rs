use std::collections::HashMap;
use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct Config {
    pub project: Project,
}

impl Config {
    const DEFAULT_CONFIG_PATH: &'static str = "oct.toml";

    pub(crate) fn new(path: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(path.unwrap_or(Self::DEFAULT_CONFIG_PATH)).map_err(|e| {
            format!(
                "Failed to read config file {}: {}",
                Self::DEFAULT_CONFIG_PATH,
                e
            )
        })?;

        let toml_data: Config = toml::from_str(&data)?;

        Ok(toml_data)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct Project {
    pub name: String,
    pub services: Vec<Service>,
}

/// Configuration for a service
/// This configuration is managed by the user and used to deploy the service
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct Service {
    /// Name of the service
    pub(crate) name: String,
    /// Image to use for the container
    pub(crate) image: String,
    /// Internal port exposed from the container
    pub(crate) internal_port: u32,
    /// External port exposed to the public internet
    pub(crate) external_port: u32,
    /// CPU millicores
    pub(crate) cpus: u32,
    /// Memory in MB
    pub(crate) memory: u64,
    /// Environment variables to set in the container
    pub(crate) env: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use tempfile;

    use super::*;

    #[test]
    fn test_config_new_success_path_privided() {
        // Arrange
        let config_file_content = r#"
[project]
name = "example"

[[project.services]]
name = "app_1"
image = "nginx:latest"
internal_port = 80
external_port = 80
cpus = 250
memory = 64

[project.services.env]
KEY1 = "VALUE1"
KEY2 = """Multiline
string"""

[[project.services]]
name = "app_2"
image = "nginx:latest"
internal_port = 80
external_port = 80
cpus = 250
memory = 64

[project.services.env]
KEY3 = "VALUE3"
KEY4 = "VALUE4"
    "#;

        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(config_file_content.as_bytes()).unwrap();

        // Act
        let config = Config::new(file.path().to_str()).unwrap();

        // Assert
        assert_eq!(
            config,
            Config {
                project: Project {
                    name: "example".to_string(),
                    services: vec![
                        Service {
                            name: "app_1".to_string(),
                            image: "nginx:latest".to_string(),
                            internal_port: 80,
                            external_port: 80,
                            cpus: 250,
                            memory: 64,
                            env: HashMap::from([
                                ("KEY1".to_string(), "VALUE1".to_string()),
                                ("KEY2".to_string(), "Multiline\nstring".to_string()),
                            ]),
                        },
                        Service {
                            name: "app_2".to_string(),
                            image: "nginx:latest".to_string(),
                            internal_port: 80,
                            external_port: 80,
                            cpus: 250,
                            memory: 64,
                            env: HashMap::from([
                                ("KEY3".to_string(), "VALUE3".to_string()),
                                ("KEY4".to_string(), "VALUE4".to_string()),
                            ]),
                        }
                    ]
                }
            }
        );
    }
}
