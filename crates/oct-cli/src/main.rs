use std::fs;

use clap::{Parser, Subcommand};
use log;
use oct_cloud::aws;
use oct_cloud::aws::Resource;
use oct_cloud::state;

mod config;
mod oct_ctl_sdk;
mod user_state;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// Path to the infra state file
    #[clap(long, default_value = "./state.json")]
    state_file_path: String,

    /// Path to the user state file
    #[clap(long, default_value = "./user_state.json")]
    user_state_file_path: String,

    /// Path to the Dockerfile
    #[clap(long, default_value = ".")]
    dockerfile_path: String,

    /// Context path
    #[clap(long, default_value = ".")]
    context_path: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Deploy the application
    Deploy,
    /// Destroy the application
    Destroy,
}

/// Deploys and destroys user services and manages underlying cloud resources
struct Orchestrator {
    state_file_path: String,
    user_state_file_path: String,
}

impl Orchestrator {
    fn new(state_file_path: String, user_state_file_path: String) -> Self {
        Orchestrator {
            state_file_path,
            user_state_file_path,
        }
    }

    async fn deploy(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Get project config
        let config = config::Config::new(None)?;

        // Create EC2 instance
        let mut instance = aws::Ec2Instance::new(
            None,
            None,
            None,
            "us-west-2".to_string(),
            "ami-04dd23e62ed049936".to_string(),
            aws::aws_sdk_ec2::types::InstanceType::T2Micro,
            "oct-cli".to_string(),
            None,
        )
        .await;

        instance.create().await?;

        let instance_id = instance.id.clone().ok_or("No instance id")?;

        log::info!("Instance created: {}", instance_id);

        // Save to state file
        let instance_state = state::Ec2InstanceState::new(&instance).await;
        let json_data = serde_json::to_string_pretty(&instance_state)?;
        fs::write(&self.state_file_path, json_data)?;

        log::info!("Waiting for oct-ctl to be ready");
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;

        let public_ip = instance.public_ip.ok_or("Public IP not found")?;

        let oct_ctl_client = oct_ctl_sdk::Client::new(public_ip.clone(), None);

        for service in config.project.services {
            log::info!("Running container for service: {}", service.name);

            let response = oct_ctl_client
                .run_container(
                    service.name.to_string(),
                    service.image.to_string(),
                    service.external_port.to_string(),
                    service.internal_port.to_string(),
                )
                .await;

            if response.is_err() {
                log::error!("Failed to run '{}' service", service.name);
                continue;
            }

            log::info!(
                "Service is available at http://{}:{}",
                public_ip,
                service.external_port
            );

            // Save service to user state file
            let deployed_service =
                user_state::UserState::new(service.name.to_string(), public_ip.to_string());
            fs::write(
                &self.user_state_file_path,
                serde_json::to_string_pretty(&deployed_service)?,
            )?;
            log::info!(
                "Service: {} - saved to user state file",
                service.name.to_string()
            );
        }

        Ok(())
    }

    async fn destroy(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Load instance from state file
        let json_data = fs::read_to_string(&self.state_file_path).expect("Unable to read file");
        let state: state::Ec2InstanceState = serde_json::from_str(&json_data)?;

        // Create EC2 instance from state
        let mut instance = state.new_from_state().await?;

        // Check if user state file exists
        if std::path::Path::new(&self.user_state_file_path).exists() {
            // Load service from user state file
            let service_json_data = fs::read_to_string(&self.user_state_file_path)
                .expect("Unable to read user state file");
            let user_state: user_state::UserState = serde_json::from_str(&service_json_data)?;

            // Remove container from instance
            log::info!(
                "Removing container for service: {}",
                user_state.service_name
            );

            let oct_ctl_client = oct_ctl_sdk::Client::new(user_state.public_ip, None);

            let response = oct_ctl_client
                .remove_container(user_state.service_name)
                .await;

            match response {
                Ok(()) => {
                    fs::remove_file(&self.user_state_file_path).expect("Unable to remove file");
                    log::info!("Service removed from user state file");
                }
                Err(err) => log::error!("Failed to remove service: {}", err),
            }
        } else {
            log::warn!("User state file not found or no containers are running");
        }

        // Destroy EC2 instance
        instance.destroy().await?;

        log::info!("Instance destroyed");

        // Remove instance from state file
        fs::remove_file(&self.state_file_path).expect("Unable to remove file");

        log::info!("Instance removed from state file");

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();

    let orchestrator = Orchestrator::new(cli.state_file_path, cli.user_state_file_path);
    let state_file_path = "./state.json";
    let user_state_file_path = "./user_state.json";

    match &cli.command {
        Commands::Deploy(args) => {
            // Get project config
            let config = config::Config::new(None)?;

            // Create EC2 instance
            let mut instance = aws::Ec2Instance::new(
                None,
                None,
                None,
                "us-west-2".to_string(),
                "ami-04dd23e62ed049936".to_string(),
                aws::aws_sdk_ec2::types::InstanceType::T2Micro,
                "oct-cli".to_string(),
                None,
            )
            .await;

            instance.create().await?;

            let instance_id = instance.id.clone().ok_or("No instance id")?;

            log::info!("Instance created: {}", instance_id);

            // Save to state file
            let instance_state = state::Ec2InstanceState::new(&instance).await;
            let json_data = serde_json::to_string_pretty(&instance_state)?;
            fs::write(state_file_path, json_data)?;

            log::info!("Waiting for oct-ctl to be ready");
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;

            match instance.public_ip {
                Some(public_ip) => {
                    for service in config.project.services {
                        log::info!("Running container for service: {}", service.name);

                        let response = oct_ctl_sdk::run_container(
                            service.name.to_string(),
                            service.image.to_string(),
                            service.external_port.to_string(),
                            service.internal_port.to_string(),
                            public_ip.to_string(),
                        )
                        .await?;

                        log::info!("Response: {}", response.text().await?);
                        log::info!(
                            "Service is available at http://{}:{}",
                            public_ip,
                            service.external_port
                        );

                        // Save service to user state file
                        let deployed_service = user_state::UserState::new(
                            service.name.to_string(),
                            public_ip.to_string(),
                        );
                        fs::write(
                            user_state_file_path,
                            serde_json::to_string_pretty(&deployed_service)?,
                        )?;
                        log::info!(
                            "Service: {} - saved to user state file",
                            service.name.to_string()
                        );
                    }
                }
                None => {
                    log::error!("Public IP not found");
                }
            }
        }
        Commands::Destroy(args) => {
            // Load instance from state file
            let json_data = fs::read_to_string(state_file_path).expect("Unable to read file");
            let state: state::Ec2InstanceState = serde_json::from_str(&json_data)?;

            // Create EC2 instance from state
            let mut instance = state.new_from_state().await?;

            // Load service from user state file
            let service_json_data =
                fs::read_to_string(user_state_file_path).expect("Unable to read user state file");
            let user_state: user_state::UserState = serde_json::from_str(&service_json_data)?;

            // Remove container from instance
            log::info!(
                "Removing container for service: {}",
                user_state.service_name
            );

            let response = oct_ctl_sdk::remove_container(
                user_state.service_name.to_string(),
                user_state.public_ip.to_string(),
            )
            .await?;

            log::info!("Response: {}", response.text().await?);

            // Remove service from user state file
            fs::remove_file(user_state_file_path).expect("Unable to remove file");
            log::info!("Service removed from user state file");

            // Destroy EC2 instance
            instance.destroy().await?;

            log::info!("Instance destroyed");

            // Remove instance from state file
            fs::remove_file(state_file_path).expect("Unable to remove file");

            log::info!("Instance removed from state file");
        }
    }

    Ok(())
}

// TODO: Add tests
// #[cfg(test)]
// mod tests {
// }
