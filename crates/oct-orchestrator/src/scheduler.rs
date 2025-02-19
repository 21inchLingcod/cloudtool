use crate::config::Service;
use crate::oct_ctl_sdk;
use crate::user_state;

/// Schedules services on EC2 instances
/// TODO:
/// - Implement custom errors (Not enough capacity)
pub(crate) struct Scheduler<'a> {
    user_state: &'a mut user_state::UserState,
}

impl<'a> Scheduler<'a> {
    pub(crate) fn new(user_state: &'a mut user_state::UserState) -> Self {
        Self { user_state }
    }

    /// Runs a service on a first available instance and adds it to the state
    pub(crate) async fn run(
        &mut self,
        service_name: &str,
        service: &Service,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let services_context = self.user_state.get_services_context();

        for (public_ip, instance) in &mut self.user_state.instances {
            let (available_cpus, available_memory) = instance.get_available_resources();

            if available_cpus < service.cpus || available_memory < service.memory {
                log::info!(
                    "Not enough capacity to run '{service_name}' service on instance {public_ip}"
                );
                continue;
            }

            let oct_ctl_client = oct_ctl_sdk::Client::new(public_ip.clone());

            let response = oct_ctl_client
                .run_container(
                    service_name.to_string(),
                    service.image.clone(),
                    service.external_port,
                    service.internal_port,
                    service.cpus,
                    service.memory,
                    service.render_envs(&services_context),
                )
                .await;

            match response {
                Ok(()) => {
                    match service.external_port {
                        Some(port) => {
                            log::info!(
                                "Service {} is available at http://{}:{port}",
                                service_name,
                                oct_ctl_client.public_ip
                            );
                        }
                        None => {
                            log::info!("Service '{}' is running", service_name);
                        }
                    };

                    instance.services.insert(
                        service_name.to_string(),
                        user_state::Service {
                            cpus: service.cpus,
                            memory: service.memory,
                        },
                    );

                    break;
                }
                Err(err) => {
                    log::error!("Failed to run '{}' service. Error: {}", service_name, err);

                    continue;
                }
            }
        }

        self.save_state();

        Ok(())
    }

    /// Stops a running container and removes it from the state
    pub(crate) async fn stop(
        &mut self,
        service_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (public_ip, instance) in &mut self.user_state.instances {
            if !instance.services.contains_key(service_name) {
                continue;
            }

            let oct_ctl_client = oct_ctl_sdk::Client::new(public_ip.clone());

            let response = oct_ctl_client
                .remove_container(service_name.to_string())
                .await;

            match response {
                Ok(()) => {
                    instance.services.remove(service_name);

                    break;
                }
                Err(err) => {
                    log::error!("Failed to stop container for service '{service_name}': {err}");

                    continue;
                }
            }
        }

        self.save_state();

        Ok(())
    }

    fn save_state(&self) {
        if let Ok(()) = self.user_state.save() {
            log::info!("User state saved to file");
        } else {
            log::error!("Failed to save user state");
        }
    }
}
