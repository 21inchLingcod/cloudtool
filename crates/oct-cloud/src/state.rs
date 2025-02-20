use serde::{Deserialize, Serialize};
use std::fs;

use crate::aws::types::InstanceType;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct State {
    pub vpc: VPCState,

    pub ecr: ECRState,

    pub instance_profile: InstanceProfileState,

    pub instances: Vec<Ec2InstanceState>,
}

impl State {
    /// Load state from file or create a new one
    /// Also returns whether the state was loaded from a file
    /// as a boolean
    pub fn new(file_path: &str) -> Result<(Self, bool), Box<dyn std::error::Error>> {
        if std::path::Path::new(file_path).exists() {
            let existing_data = fs::read_to_string(file_path)?;
            Ok((serde_json::from_str::<State>(&existing_data)?, true))
        } else {
            Ok((State::default(), false))
        }
    }

    /// Save state to file
    pub fn save(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(file_path, serde_json::to_string_pretty(self)?)?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Ec2InstanceState {
    pub id: String,
    pub public_ip: String,
    pub public_dns: String,
    pub region: String,
    pub ami: String,
    pub instance_type: String,
    pub name: String,
    pub instance_profile_name: String,
    pub subnet_id: String,
    pub security_group_id: String,
}

#[cfg(test)]
mod mocks {
    use crate::aws::types::InstanceType;

    pub struct MockECR {
        pub id: Option<String>,
        pub name: String,
        pub region: String,
    }

    impl MockECR {
        pub async fn new(id: Option<String>, name: String, region: String) -> Self {
            Self { id, name, region }
        }
    }

    pub struct MockEc2Instance {
        pub id: Option<String>,
        pub public_ip: Option<String>,
        pub public_dns: Option<String>,
        pub region: String,
        pub ami: String,
        pub instance_type: InstanceType,
        pub name: String,
        pub instance_profile_name: String,
        pub subnet_id: String,
        pub security_group_id: String,
    }

    impl MockEc2Instance {
        pub async fn new(
            id: Option<String>,
            public_ip: Option<String>,
            public_dns: Option<String>,
            region: String,
            ami: String,
            instance_type: InstanceType,
            name: String,
            instance_profile_name: String,
            subnet_id: String,
            security_group_id: String,
        ) -> Self {
            Self {
                id,
                public_ip,
                public_dns,
                region,
                ami,
                instance_type,
                name,
                instance_profile_name,
                subnet_id,
                security_group_id,
            }
        }
    }

    pub struct MockInstanceProfile {
        pub name: String,
        pub region: String,
        pub instance_roles: Vec<MockInstanceRole>,
    }

    impl MockInstanceProfile {
        pub async fn new(
            name: String,
            region: String,
            instance_roles: Vec<MockInstanceRole>,
        ) -> Self {
            Self {
                name,
                region,
                instance_roles,
            }
        }
    }

    pub struct MockInstanceRole {
        pub name: String,
        pub region: String,
        pub assume_role_policy: String,
        pub policy_arns: Vec<String>,
    }

    impl MockInstanceRole {
        pub async fn new(name: String, region: String) -> Self {
            Self {
                name,
                region,
                assume_role_policy: "test_assume_role_policy".to_string(),
                policy_arns: vec!["test_policy_arn".to_string()],
            }
        }
    }

    pub struct MockVPC {
        pub id: Option<String>,
        pub region: String,
        pub cidr_block: String,
        pub name: String,
        pub subnet: MockSubnet,
        pub internet_gateway: Option<MockInternetGateway>,
        pub route_table: MockRouteTable,
        pub security_group: MockSecurityGroup,
    }

    impl MockVPC {
        pub async fn new(
            id: Option<String>,
            region: String,
            cidr_block: String,
            name: String,
            subnet: MockSubnet,
            internet_gateway: Option<MockInternetGateway>,
            route_table: MockRouteTable,
            security_group: MockSecurityGroup,
        ) -> Self {
            Self {
                id,
                region,
                cidr_block,
                name,
                subnet,
                internet_gateway,
                route_table,
                security_group,
            }
        }
    }

    pub struct MockSubnet {
        pub id: Option<String>,
        pub region: String,
        pub cidr_block: String,
        pub availability_zone: String,
        pub vpc_id: Option<String>,
        pub name: String,
    }

    impl MockSubnet {
        pub async fn new(
            id: Option<String>,
            region: String,
            cidr_block: String,
            availability_zone: String,
            vpc_id: Option<String>,
            name: String,
        ) -> Self {
            Self {
                id,
                region,
                cidr_block,
                availability_zone,
                name,
                vpc_id,
            }
        }
    }

    pub struct MockInternetGateway {
        pub id: Option<String>,
        pub vpc_id: Option<String>,
        pub route_table_id: Option<String>,
        pub subnet_id: Option<String>,
        pub region: String,
    }

    impl MockInternetGateway {
        pub async fn new(
            id: Option<String>,
            vpc_id: Option<String>,
            route_table_id: Option<String>,
            subnet_id: Option<String>,
            region: String,
        ) -> Self {
            Self {
                id,
                vpc_id,
                route_table_id,
                subnet_id,
                region,
            }
        }
    }

    pub struct MockRouteTable {
        pub id: Option<String>,
        pub vpc_id: Option<String>,
        pub subnet_id: Option<String>,
        pub region: String,
    }

    impl MockRouteTable {
        pub async fn new(
            id: Option<String>,
            vpc_id: Option<String>,
            subnet_id: Option<String>,
            region: String,
        ) -> Self {
            Self {
                id,
                vpc_id,
                subnet_id,
                region,
            }
        }
    }

    pub struct MockSecurityGroup {
        pub id: Option<String>,
        pub name: String,
        pub vpc_id: Option<String>,
        pub description: String,
        pub region: String,
        pub inbound_rules: Vec<MockInboundRule>,
    }

    impl MockSecurityGroup {
        pub async fn new(
            id: Option<String>,
            name: String,
            vpc_id: Option<String>,
            description: String,
            region: String,
            inbound_rules: Vec<MockInboundRule>,
        ) -> Self {
            Self {
                id,
                name,
                vpc_id,
                description,
                region,
                inbound_rules,
            }
        }
    }

    #[derive(Clone)]
    pub struct MockInboundRule {
        pub protocol: String,
        pub port: i32,
        pub cidr_block: String,
    }

    impl MockInboundRule {
        pub fn new(protocol: String, port: i32, cidr_block: String) -> Self {
            Self {
                protocol,
                port,
                cidr_block,
            }
        }
    }
}

#[cfg(not(test))]
use crate::aws::resource::{
    Ec2Instance, EcrRepository, InboundRule, InstanceProfile, InstanceRole, InternetGateway,
    RouteTable, SecurityGroup, Subnet, VPC,
};

#[cfg(test)]
use mocks::{
    MockECR as EcrRepository, MockEc2Instance as Ec2Instance, MockInboundRule as InboundRule,
    MockInstanceProfile as InstanceProfile, MockInstanceRole as InstanceRole,
    MockInternetGateway as InternetGateway, MockRouteTable as RouteTable,
    MockSecurityGroup as SecurityGroup, MockSubnet as Subnet, MockVPC as VPC,
};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct ECRState {
    pub name: String,
    pub region: String,
    pub id: String,
}

impl ECRState {
    pub fn new(ecr_repository: &EcrRepository) -> Self {
        Self {
            name: ecr_repository.name.clone(),
            region: ecr_repository.region.clone(),
            id: ecr_repository.id.clone().expect("ECR id not set"),
        }
    }

    pub async fn new_from_state(&self) -> Result<EcrRepository, Box<dyn std::error::Error>> {
        Ok(EcrRepository::new(
            Some(self.id.clone()),
            self.name.clone(),
            self.region.clone(),
        )
        .await)
    }
}

impl Ec2InstanceState {
    pub fn new(ec2_instance: &Ec2Instance) -> Self {
        Self {
            id: ec2_instance.id.clone().expect("Instance id is not set"),
            public_ip: ec2_instance
                .public_ip
                .clone()
                .expect("Public ip is not set"),
            public_dns: ec2_instance
                .public_dns
                .clone()
                .expect("Public dns is not set"),
            region: ec2_instance.region.clone(),
            ami: ec2_instance.ami.clone(),
            instance_type: ec2_instance.instance_type.name.to_string(),
            name: ec2_instance.name.clone(),
            instance_profile_name: ec2_instance.instance_profile_name.clone(),
            subnet_id: ec2_instance.subnet_id.clone(),
            security_group_id: ec2_instance.security_group_id.clone(),
        }
    }

    pub async fn new_from_state(&self) -> Result<Ec2Instance, Box<dyn std::error::Error>> {
        Ok(Ec2Instance::new(
            Some(self.id.clone()),
            Some(self.public_ip.clone()),
            Some(self.public_dns.clone()),
            self.region.clone(),
            self.ami.clone(),
            InstanceType::from(self.instance_type.as_str()),
            self.name.clone(),
            self.instance_profile_name.clone(),
            self.subnet_id.clone(),
            self.security_group_id.clone(),
        )
        .await)
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstanceProfileState {
    pub name: String,
    pub region: String,
    pub instance_roles: Vec<InstanceRoleState>,
}

impl InstanceProfileState {
    pub fn new(instance_profile: &InstanceProfile) -> Self {
        Self {
            name: instance_profile.name.clone(),
            region: instance_profile.region.clone(),
            instance_roles: instance_profile
                .instance_roles
                .iter()
                .map(InstanceRoleState::new)
                .collect(),
        }
    }

    pub async fn new_from_state(&self) -> InstanceProfile {
        let mut roles = vec![];
        for role in &self.instance_roles {
            roles.push(role.new_from_state().await);
        }

        InstanceProfile::new(self.name.clone(), self.region.clone(), roles).await
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct InstanceRoleState {
    pub name: String,
    pub region: String,
    pub assume_role_policy: String,
    pub policy_arns: Vec<String>,
}

impl InstanceRoleState {
    pub fn new(instance_role: &InstanceRole) -> Self {
        Self {
            name: instance_role.name.clone(),
            region: instance_role.region.clone(),
            assume_role_policy: instance_role.assume_role_policy.clone(),
            policy_arns: instance_role.policy_arns.clone(),
        }
    }

    pub async fn new_from_state(&self) -> InstanceRole {
        InstanceRole::new(self.name.clone(), self.region.clone()).await
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct VPCState {
    pub id: String,
    pub region: String,
    pub cidr_block: String,
    pub name: String,
    pub subnet: SubnetState,
    pub internet_gateway: Option<InternetGatewayState>,
    pub route_table: RouteTableState,
    pub security_group: SecurityGroupState,
}

impl VPCState {
    pub fn new(vpc: &VPC) -> Self {
        Self {
            id: vpc.id.clone().expect("VPC id not set"),
            region: vpc.region.clone(),
            cidr_block: vpc.cidr_block.clone(),
            name: vpc.name.clone(),
            subnet: SubnetState::new(&vpc.subnet),
            internet_gateway: vpc.internet_gateway.as_ref().map(InternetGatewayState::new),
            route_table: RouteTableState::new(&vpc.route_table),
            security_group: SecurityGroupState::new(&vpc.security_group),
        }
    }

    pub async fn new_from_state(&self) -> VPC {
        let internet_gateway = match &self.internet_gateway {
            Some(ig) => Some(ig.new_from_state().await),
            None => None,
        };

        VPC::new(
            Some(self.id.clone()),
            self.region.clone(),
            self.cidr_block.clone(),
            self.name.clone(),
            self.subnet.new_from_state().await,
            internet_gateway,
            self.route_table.new_from_state().await,
            self.security_group.new_from_state().await,
        )
        .await
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct SubnetState {
    pub id: String,
    pub region: String,
    pub cidr_block: String,
    pub availability_zone: String,
    pub vpc_id: String,
    pub name: String,
}

impl SubnetState {
    pub fn new(subnet: &Subnet) -> Self {
        Self {
            id: subnet.id.clone().expect("Subnet id not set"),
            region: subnet.region.clone(),
            cidr_block: subnet.cidr_block.clone(),
            availability_zone: subnet.availability_zone.clone(),
            vpc_id: subnet.vpc_id.clone().expect("vpc id not set"),
            name: subnet.name.clone(),
        }
    }

    pub async fn new_from_state(&self) -> Subnet {
        Subnet::new(
            Some(self.id.clone()),
            self.region.clone(),
            self.cidr_block.clone(),
            self.availability_zone.clone(),
            Some(self.vpc_id.clone()),
            self.name.clone(),
        )
        .await
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct InternetGatewayState {
    pub id: String,
    pub vpc_id: String,
    pub route_table_id: String,
    pub subnet_id: String,
    pub region: String,
}

impl InternetGatewayState {
    pub fn new(gateway: &InternetGateway) -> Self {
        Self {
            id: gateway.id.clone().expect("Internet Gateway id not set"),
            vpc_id: gateway.vpc_id.clone().expect("VPC id not set"),
            route_table_id: gateway
                .route_table_id
                .clone()
                .expect("Route Table id not set"),
            subnet_id: gateway.subnet_id.clone().expect("Subnet id not set"),
            region: gateway.region.clone(),
        }
    }

    pub async fn new_from_state(&self) -> InternetGateway {
        InternetGateway::new(
            Some(self.id.clone()),
            Some(self.vpc_id.clone()),
            Some(self.route_table_id.clone()),
            Some(self.subnet_id.clone()),
            self.region.clone(),
        )
        .await
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct SecurityGroupState {
    pub id: String,
    pub vpc_id: String,
    pub name: String,
    pub description: String,
    pub region: String,
    pub inbound_rules: Vec<InboundRuleState>,
}

impl SecurityGroupState {
    pub fn new(group: &SecurityGroup) -> Self {
        Self {
            id: group.id.clone().expect("Security Group id not set"),
            vpc_id: group.vpc_id.clone().expect("VPC id not set"),
            name: group.name.clone(),
            description: group.description.clone(),
            region: group.region.clone(),
            inbound_rules: group
                .inbound_rules
                .iter()
                .map(InboundRuleState::new)
                .collect(),
        }
    }

    pub async fn new_from_state(&self) -> SecurityGroup {
        let inbound_rules = self
            .inbound_rules
            .iter()
            .map(InboundRuleState::new_from_state)
            .collect();

        SecurityGroup::new(
            Some(self.id.clone()),
            self.name.clone(),
            Some(self.vpc_id.clone()),
            self.description.clone(),
            self.region.clone(),
            inbound_rules,
        )
        .await
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct InboundRuleState {
    pub protocol: String,
    pub port: i32,
    pub cidr_block: String,
}

impl InboundRuleState {
    pub fn new(rule: &InboundRule) -> Self {
        Self {
            protocol: rule.protocol.clone(),
            port: rule.port,
            cidr_block: rule.cidr_block.clone(),
        }
    }

    pub fn new_from_state(&self) -> InboundRule {
        InboundRule::new(self.protocol.clone(), self.port, self.cidr_block.clone())
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct RouteTableState {
    pub id: String,
    pub vpc_id: String,
    pub subnet_id: String,
    pub region: String,
}

impl RouteTableState {
    pub fn new(route_table: &RouteTable) -> Self {
        Self {
            id: route_table.id.clone().expect("Route Table id not set"),
            vpc_id: route_table.vpc_id.clone().expect("VPC id not set"),
            subnet_id: route_table.subnet_id.clone().expect("Subnet id not set"),
            region: route_table.region.clone(),
        }
    }

    pub async fn new_from_state(&self) -> RouteTable {
        RouteTable::new(
            Some(self.id.clone()),
            Some(self.vpc_id.clone()),
            Some(self.subnet_id.clone()),
            self.region.clone(),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use tempfile;

    use super::*;

    #[tokio::test]
    async fn test_state() {
        // Arrange
        let state = State {
            vpc: VPCState {
                id: "id".to_string(),
                region: "region".to_string(),
                cidr_block: "test_cidr_block".to_string(),
                name: "name".to_string(),
                subnet: SubnetState {
                    id: "id".to_string(),
                    region: "region".to_string(),
                    cidr_block: "test_cidr_block".to_string(),
                    availability_zone: "availability_zone".to_string(),
                    vpc_id: "vpc_id".to_string(),
                    name: "name".to_string(),
                },
                internet_gateway: None,
                route_table: RouteTableState {
                    id: "id".to_string(),
                    vpc_id: "vpc_id".to_string(),
                    subnet_id: "subnet_id".to_string(),
                    region: "region".to_string(),
                },
                security_group: SecurityGroupState {
                    id: "id".to_string(),
                    vpc_id: "vpc_id".to_string(),
                    name: "name".to_string(),
                    description: "description".to_string(),
                    region: "region".to_string(),
                    inbound_rules: vec![],
                },
            },
            ecr: ECRState {
                id: "id".to_string(),
                name: "name".to_string(),
                region: "region".to_string(),
            },
            instance_profile: InstanceProfileState {
                name: "instance_profile_name".to_string(),
                region: "region".to_string(),
                instance_roles: vec![InstanceRoleState {
                    name: "instance_role_name".to_string(),
                    region: "region".to_string(),
                    assume_role_policy: "assume_role_policy".to_string(),
                    policy_arns: vec!["policy_arns".to_string()],
                }],
            },
            instances: vec![Ec2InstanceState {
                id: "id".to_string(),
                public_ip: "public_ip".to_string(),
                public_dns: "public_dns".to_string(),
                region: "region".to_string(),
                ami: "ami".to_string(),
                instance_type: "t2.micro".to_string(),
                name: "name".to_string(),
                instance_profile_name: "instance_profile_name".to_string(),
                subnet_id: "subnet_id".to_string(),
                security_group_id: "security_group_id".to_string(),
            }],
        };

        // Assert
        assert_eq!(state.vpc.id, "id".to_string());
        assert_eq!(state.instances.len(), 1);
    }

    #[tokio::test]
    async fn test_ec2_instance_state() {
        let ec2_instance = Ec2Instance::new(
            Some("id".to_string()),
            Some("public_ip".to_string()),
            Some("public_dns".to_string()),
            "region".to_string(),
            "ami".to_string(),
            InstanceType::T2_MICRO,
            "name".to_string(),
            "instance_profile_name".to_string(),
            "subnet_id".to_string(),
            "security_group_id".to_string(),
        )
        .await;

        let ec2_instance_state = Ec2InstanceState::new(&ec2_instance);

        assert_eq!(ec2_instance_state.id, "id");
        assert_eq!(ec2_instance_state.public_ip, "public_ip");
        assert_eq!(ec2_instance_state.public_dns, "public_dns");
        assert_eq!(ec2_instance_state.region, "region");
        assert_eq!(ec2_instance_state.ami, "ami");
        assert_eq!(ec2_instance_state.instance_type, "t2.micro");
        assert_eq!(ec2_instance_state.name, "name");
        assert_eq!(
            ec2_instance_state.instance_profile_name,
            "instance_profile_name"
        );
        assert_eq!(ec2_instance_state.subnet_id, "subnet_id");
        assert_eq!(ec2_instance_state.security_group_id, "security_group_id");
    }

    #[tokio::test]
    async fn test_ec2_instance_state_new_from_state() {
        // Arrange
        let ec2_instance_state = Ec2InstanceState {
            id: "id".to_string(),
            public_ip: "public_ip".to_string(),
            public_dns: "public_dns".to_string(),
            region: "region".to_string(),
            ami: "ami".to_string(),
            instance_type: "t2.micro".to_string(),
            name: "name".to_string(),
            instance_profile_name: "instance_profile_name".to_string(),
            subnet_id: "subnet_id".to_string(),
            security_group_id: "security_group_id".to_string(),
        };

        // Act
        let ec2_instance = ec2_instance_state.new_from_state().await.unwrap();

        // Assert
        assert_eq!(ec2_instance.id, Some("id".to_string()));
        assert_eq!(ec2_instance.public_ip, Some("public_ip".to_string()));
        assert_eq!(ec2_instance.public_dns, Some("public_dns".to_string()));
        assert_eq!(ec2_instance.region, "region".to_string());
        assert_eq!(ec2_instance.ami, "ami".to_string());
        assert_eq!(ec2_instance.instance_type, InstanceType::T2_MICRO);
        assert_eq!(ec2_instance.name, "name".to_string());
        assert_eq!(
            ec2_instance.instance_profile_name,
            "instance_profile_name".to_string()
        );
    }

    #[tokio::test]
    async fn test_instance_profile_state() {
        let instance_profile = InstanceProfile::new(
            "test_instance_profile".to_string(),
            "test_region".to_string(),
            vec![InstanceRole::new("test_role".to_string(), "test_region".to_string()).await],
        )
        .await;

        let instance_profile_state = InstanceProfileState::new(&instance_profile);

        assert_eq!(instance_profile_state.name, "test_instance_profile");
        assert_eq!(instance_profile_state.region, "test_region");
        assert_eq!(instance_profile_state.instance_roles.len(), 1);
    }

    #[tokio::test]
    async fn test_instance_profile_state_new_from_state() {
        // Arrange
        let instance_profile_state = InstanceProfileState {
            name: "test_name".to_string(),
            region: "test_region".to_string(),
            instance_roles: vec![InstanceRoleState {
                name: "test_name".to_string(),
                region: "test_region".to_string(),
                assume_role_policy: "test_assume_role_policy".to_string(),
                policy_arns: vec!["test_policy_arn".to_string()],
            }],
        };

        // Act
        let instance_profile = instance_profile_state.new_from_state().await;

        // Assert
        assert_eq!(instance_profile.name, "test_name".to_string());
        assert_eq!(instance_profile.region, "test_region".to_string());
        assert_eq!(instance_profile.instance_roles.len(), 1);
        assert_eq!(
            instance_profile.instance_roles[0].name,
            "test_name".to_string()
        );
        assert_eq!(
            instance_profile.instance_roles[0].assume_role_policy,
            "test_assume_role_policy".to_string()
        );
        assert_eq!(
            instance_profile.instance_roles[0].policy_arns,
            vec!["test_policy_arn".to_string()]
        );
    }

    #[tokio::test]
    async fn test_instance_role_state() {
        let instance_role =
            InstanceRole::new("test_role".to_string(), "test_region".to_string()).await;

        let instance_role_state = InstanceRoleState::new(&instance_role);

        assert_eq!(instance_role_state.name, "test_role");
        assert_eq!(instance_role_state.region, "test_region");
        assert_eq!(
            instance_role_state.assume_role_policy,
            "test_assume_role_policy"
        );
        assert_eq!(instance_role_state.policy_arns, vec!["test_policy_arn"]);
    }

    #[tokio::test]
    async fn test_instance_role_state_new_from_state() {
        // Arrange
        let instance_role_state = InstanceRoleState {
            name: "test_name".to_string(),
            region: "test_region".to_string(),
            assume_role_policy: "test_assume_role_policy".to_string(),
            policy_arns: vec!["test_policy_arn".to_string()],
        };

        // Act
        let instance_role = instance_role_state.new_from_state().await;

        // Assert
        assert_eq!(instance_role.name, "test_name".to_string());
        assert_eq!(instance_role.region, "test_region".to_string());
        assert_eq!(
            instance_role.assume_role_policy,
            "test_assume_role_policy".to_string()
        );
        assert_eq!(
            instance_role.policy_arns,
            vec!["test_policy_arn".to_string()]
        );
    }

    #[tokio::test]
    async fn test_vpc_state() {
        // Arrange
        let vpc = VPC::new(
            Some("id".to_string()),
            "region".to_string(),
            "test_cidr_block".to_string(),
            "name".to_string(),
            Subnet {
                id: Some("id".to_string()),
                region: "region".to_string(),
                cidr_block: "test_cidr_block".to_string(),
                availability_zone: "availability_zone".to_string(),
                vpc_id: Some("vpc_id".to_string()),
                name: "name".to_string(),
            },
            None,
            RouteTable {
                id: Some("id".to_string()),
                vpc_id: Some("vpc_id".to_string()),
                subnet_id: Some("subnet_id".to_string()),
                region: "region".to_string(),
            },
            SecurityGroup {
                id: Some("id".to_string()),
                vpc_id: Some("vpc_id".to_string()),
                name: "name".to_string(),
                description: "description".to_string(),
                region: "region".to_string(),
                inbound_rules: vec![InboundRule {
                    protocol: "tcp".to_string(),
                    port: 0,
                    cidr_block: "cidr_block".to_string(),
                }],
            },
        )
        .await;

        // Act
        let vpc_state = VPCState::new(&vpc);

        // Assert
        assert_eq!(vpc_state.id, "id".to_string());
        assert_eq!(vpc_state.region, "region".to_string());
        assert_eq!(vpc_state.cidr_block, "test_cidr_block".to_string());
        assert_eq!(vpc_state.name, "name".to_string());
    }

    #[test]
    fn test_state_new_exists() {
        // Arrange
        let state_file_content = r#"
{
    "vpc": {    
        "id": "id",
        "region": "region",
        "cidr_block": "test_cidr_block",
        "name": "name",
        "subnet": {
            "id": "id",
            "region": "region",
            "cidr_block": "test_cidr_block",
            "availability_zone": "availability_zone",
            "vpc_id": "vpc_id",
            "name": "name"
        },
        "internet_gateway": null,
        "route_table": {
            "id": "id",
            "vpc_id": "vpc_id",
            "subnet_id": "subnet_id",
            "region": "region"
        },
        "security_group": {
            "id": "id",
            "vpc_id": "vpc_id",
            "name": "name",
            "description": "description",
            "region": "region",
            "inbound_rules": [
            {
                "protocol": "tcp",
                "port": 0,
                "cidr_block": "cidr_block"
            }
            ]
        }
    },
    "ecr": {
        "name": "name",
        "region": "region",
        "id": "id"
    },
    "instance_profile": {
        "name": "instance_profile_name",
        "region": "region",
        "instance_roles": [
        {
            "name": "instance_role_name",
            "region": "region",
            "assume_role_policy": "assume_role_policy",
            "policy_arns": [
                "policy_arn"
            ]
        }
        ]
    },
    "instances": [
    {
        "id": "id",
        "public_ip": "public_ip",
        "public_dns": "public_dns",
        "region": "region",
        "ami": "ami",
        "instance_type": "t2.micro",
        "name": "name",
        "instance_profile_name": "instance_profile_name",
        "subnet_id": "subnet_id",
        "security_group_id": "security_group_id"
    }]
}"#;

        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(state_file_content.as_bytes()).unwrap();

        // Act
        let (state, loaded) = State::new(file.path().to_str().unwrap()).unwrap();

        // Assert
        assert!(loaded);
        assert_eq!(
            state,
            State {
                vpc: VPCState {
                    id: "id".to_string(),
                    region: "region".to_string(),
                    cidr_block: "test_cidr_block".to_string(),
                    name: "name".to_string(),
                    subnet: SubnetState {
                        id: "id".to_string(),
                        region: "region".to_string(),
                        cidr_block: "test_cidr_block".to_string(),
                        availability_zone: "availability_zone".to_string(),
                        vpc_id: "vpc_id".to_string(),
                        name: "name".to_string(),
                    },
                    internet_gateway: None,
                    route_table: RouteTableState {
                        id: "id".to_string(),
                        vpc_id: "vpc_id".to_string(),
                        subnet_id: "subnet_id".to_string(),
                        region: "region".to_string(),
                    },
                    security_group: SecurityGroupState {
                        id: "id".to_string(),
                        vpc_id: "vpc_id".to_string(),
                        name: "name".to_string(),
                        description: "description".to_string(),
                        region: "region".to_string(),
                        inbound_rules: vec![InboundRuleState {
                            protocol: "tcp".to_string(),
                            port: 0,
                            cidr_block: "cidr_block".to_string(),
                        }],
                    },
                },
                ecr: ECRState {
                    id: "id".to_string(),
                    name: "name".to_string(),
                    region: "region".to_string(),
                },
                instance_profile: InstanceProfileState {
                    name: "instance_profile_name".to_string(),
                    region: "region".to_string(),
                    instance_roles: vec![InstanceRoleState {
                        name: "instance_role_name".to_string(),
                        region: "region".to_string(),
                        assume_role_policy: "assume_role_policy".to_string(),
                        policy_arns: vec!["policy_arn".to_string()],
                    }],
                },
                instances: vec![Ec2InstanceState {
                    id: "id".to_string(),
                    public_ip: "public_ip".to_string(),
                    public_dns: "public_dns".to_string(),
                    region: "region".to_string(),
                    ami: "ami".to_string(),
                    instance_type: "t2.micro".to_string(),
                    name: "name".to_string(),
                    instance_profile_name: "instance_profile_name".to_string(),
                    subnet_id: "subnet_id".to_string(),
                    security_group_id: "security_group_id".to_string(),
                }],
            }
        )
    }

    #[test]
    fn test_state_new_not_exists() {
        // Act
        let (state, loaded) = State::new("NO_FILE").unwrap();

        // Assert
        assert_eq!(state.instances.len(), 0);
        assert!(!loaded);
    }

    #[test]
    fn test_state_save() {
        // Arrange
        let state = State {
            vpc: VPCState {
                id: "id".to_string(),
                region: "region".to_string(),
                cidr_block: "test_cidr_block".to_string(),
                name: "name".to_string(),
                subnet: SubnetState {
                    id: "id".to_string(),
                    region: "region".to_string(),
                    cidr_block: "test_cidr_block".to_string(),
                    availability_zone: "availability_zone".to_string(),
                    vpc_id: "vpc_id".to_string(),
                    name: "name".to_string(),
                },
                internet_gateway: None,
                route_table: RouteTableState {
                    id: "id".to_string(),
                    vpc_id: "vpc_id".to_string(),
                    subnet_id: "subnet_id".to_string(),
                    region: "region".to_string(),
                },
                security_group: SecurityGroupState {
                    id: "id".to_string(),
                    vpc_id: "vpc_id".to_string(),
                    name: "name".to_string(),
                    description: "description".to_string(),
                    region: "region".to_string(),
                    inbound_rules: vec![InboundRuleState {
                        protocol: "tcp".to_string(),
                        port: 0,
                        cidr_block: "cidr_block".to_string(),
                    }],
                },
            },
            ecr: ECRState {
                id: "id".to_string(),
                name: "name".to_string(),
                region: "region".to_string(),
            },
            instance_profile: InstanceProfileState {
                name: "instance_profile_name".to_string(),
                region: "region".to_string(),
                instance_roles: vec![InstanceRoleState {
                    name: "instance_role_name".to_string(),
                    region: "region".to_string(),
                    assume_role_policy: "assume_role_policy".to_string(),
                    policy_arns: vec!["policy_arn".to_string()],
                }],
            },
            instances: vec![Ec2InstanceState {
                id: "id".to_string(),
                public_ip: "public_ip".to_string(),
                public_dns: "public_dns".to_string(),
                region: "region".to_string(),
                ami: "ami".to_string(),
                instance_type: "t2.micro".to_string(),
                name: "name".to_string(),
                instance_profile_name: "instance_profile_name".to_string(),
                subnet_id: "subnet_id".to_string(),
                security_group_id: "security_group_id".to_string(),
            }],
        };

        let state_file = tempfile::NamedTempFile::new().unwrap();

        // Act
        state.save(state_file.path().to_str().unwrap()).unwrap();

        // Assert
        let file_content = fs::read_to_string(state_file.path()).unwrap();

        assert_eq!(
            file_content,
            r#"{
  "vpc": {
    "id": "id",
    "region": "region",
    "cidr_block": "test_cidr_block",
    "name": "name",
    "subnet": {
      "id": "id",
      "region": "region",
      "cidr_block": "test_cidr_block",
      "availability_zone": "availability_zone",
      "vpc_id": "vpc_id",
      "name": "name"
    },
    "internet_gateway": null,
    "route_table": {
      "id": "id",
      "vpc_id": "vpc_id",
      "subnet_id": "subnet_id",
      "region": "region"
    },
    "security_group": {
      "id": "id",
      "vpc_id": "vpc_id",
      "name": "name",
      "description": "description",
      "region": "region",
      "inbound_rules": [
        {
          "protocol": "tcp",
          "port": 0,
          "cidr_block": "cidr_block"
        }
      ]
    }
  },
  "ecr": {
    "name": "name",
    "region": "region",
    "id": "id"
  },
  "instance_profile": {
    "name": "instance_profile_name",
    "region": "region",
    "instance_roles": [
      {
        "name": "instance_role_name",
        "region": "region",
        "assume_role_policy": "assume_role_policy",
        "policy_arns": [
          "policy_arn"
        ]
      }
    ]
  },
  "instances": [
    {
      "id": "id",
      "public_ip": "public_ip",
      "public_dns": "public_dns",
      "region": "region",
      "ami": "ami",
      "instance_type": "t2.micro",
      "name": "name",
      "instance_profile_name": "instance_profile_name",
      "subnet_id": "subnet_id",
      "security_group_id": "security_group_id"
    }
  ]
}"#
        );
    }

    #[tokio::test]
    async fn test_vpc_state_new_from_state_no_internet_gateway() {
        // Arrange
        let vpc_state = VPCState {
            id: "id".to_string(),
            region: "region".to_string(),
            cidr_block: "test_cidr_block".to_string(),
            name: "name".to_string(),
            subnet: SubnetState {
                id: "id".to_string(),
                region: "region".to_string(),
                cidr_block: "test_cidr_block".to_string(),
                availability_zone: "availability_zone".to_string(),
                vpc_id: "vpc_id".to_string(),
                name: "name".to_string(),
            },
            internet_gateway: None,
            route_table: RouteTableState {
                id: "id".to_string(),
                vpc_id: "vpc_id".to_string(),
                subnet_id: "subnet_id".to_string(),
                region: "region".to_string(),
            },
            security_group: SecurityGroupState {
                id: "id".to_string(),
                vpc_id: "vpc_id".to_string(),
                name: "name".to_string(),
                description: "description".to_string(),
                region: "region".to_string(),
                inbound_rules: vec![],
            },
        };

        // Act
        let vpc = vpc_state.new_from_state().await;

        // Assert
        assert_eq!(vpc.id, Some("id".to_string()));
        assert_eq!(vpc.region, "region".to_string());
        assert_eq!(vpc.cidr_block, "test_cidr_block".to_string());
        assert_eq!(vpc.name, "name".to_string());
    }

    #[tokio::test]
    async fn test_vpc_state_new_from_state_internet_gateway() {
        // Arrange
        let vpc_state = VPCState {
            id: "id".to_string(),
            region: "region".to_string(),
            cidr_block: "test_cidr_block".to_string(),
            name: "name".to_string(),
            subnet: SubnetState {
                id: "id".to_string(),
                region: "region".to_string(),
                cidr_block: "test_cidr_block".to_string(),
                availability_zone: "availability_zone".to_string(),
                vpc_id: "vpc_id".to_string(),
                name: "name".to_string(),
            },
            internet_gateway: Some(InternetGatewayState {
                id: "id".to_string(),
                vpc_id: "vpc_id".to_string(),
                route_table_id: "route_table_id".to_string(),
                subnet_id: "subnet_id".to_string(),
                region: "region".to_string(),
            }),
            route_table: RouteTableState {
                id: "id".to_string(),
                vpc_id: "vpc_id".to_string(),
                subnet_id: "subnet_id".to_string(),
                region: "region".to_string(),
            },
            security_group: SecurityGroupState {
                id: "id".to_string(),
                vpc_id: "vpc_id".to_string(),
                name: "name".to_string(),
                description: "description".to_string(),
                region: "region".to_string(),
                inbound_rules: vec![],
            },
        };

        // Act
        let vpc = vpc_state.new_from_state().await;

        // Assert
        assert_eq!(vpc.id, Some("id".to_string()));
        assert_eq!(vpc.region, "region".to_string());
        assert_eq!(vpc.cidr_block, "test_cidr_block".to_string());
        assert_eq!(vpc.name, "name".to_string());
        assert_eq!(vpc.subnet.id, Some("id".to_string()));
        assert_eq!(vpc.subnet.region, "region".to_string());
        assert_eq!(vpc.subnet.cidr_block, "test_cidr_block".to_string());
        assert_eq!(vpc.subnet.vpc_id, Some("vpc_id".to_string()));
        assert_eq!(vpc.subnet.name, "name".to_string());
        assert_eq!(
            vpc.internet_gateway.as_ref().unwrap().id,
            Some("id".to_string())
        );
        assert_eq!(
            vpc.internet_gateway.as_ref().unwrap().vpc_id,
            Some("vpc_id".to_string())
        );
        assert_eq!(
            vpc.internet_gateway.as_ref().unwrap().route_table_id,
            Some("route_table_id".to_string())
        );
        assert_eq!(
            vpc.internet_gateway.as_ref().unwrap().subnet_id,
            Some("subnet_id".to_string())
        );
        assert_eq!(
            vpc.internet_gateway.as_ref().unwrap().region,
            "region".to_string()
        );
        assert_eq!(vpc.route_table.id, Some("id".to_string()));
        assert_eq!(vpc.route_table.vpc_id, Some("vpc_id".to_string()));
        assert_eq!(vpc.route_table.subnet_id, Some("subnet_id".to_string()));
        assert_eq!(vpc.route_table.region, "region".to_string());
        assert_eq!(vpc.security_group.id, Some("id".to_string()));
        assert_eq!(vpc.security_group.vpc_id, Some("vpc_id".to_string()));
        assert_eq!(vpc.security_group.name, "name".to_string());
        assert_eq!(vpc.security_group.description, "description".to_string());
        assert_eq!(vpc.security_group.region, "region".to_string());
        assert_eq!(vpc.security_group.inbound_rules.len(), 0);
    }

    #[tokio::test]
    async fn test_subnet_state() {
        // Arrange
        let subnet = Subnet::new(
            Some("id".to_string()),
            "region".to_string(),
            "test_cidr_block".to_string(),
            "availability_zone".to_string(),
            Some("vpc_id".to_string()),
            "test_name".to_string(),
        )
        .await;

        // Act
        let subnet_state = SubnetState::new(&subnet);

        // Assert
        assert_eq!(subnet_state.id, "id".to_string());
        assert_eq!(subnet_state.region, "region".to_string());
        assert_eq!(subnet_state.cidr_block, "test_cidr_block".to_string());
        assert_eq!(subnet_state.vpc_id, "vpc_id".to_string());
        assert_eq!(subnet_state.name, "test_name".to_string());
    }

    #[tokio::test]
    async fn test_subnet_state_new_from_state() {
        // Arrange
        let subnet_state = SubnetState {
            id: "id".to_string(),
            region: "region".to_string(),
            cidr_block: "test_cidr_block".to_string(),
            availability_zone: "availability_zone".to_string(),
            vpc_id: "vpc_id".to_string(),
            name: "test_name".to_string(),
        };

        // Act
        let subnet = subnet_state.new_from_state().await;

        // Assert
        assert_eq!(subnet.id, Some("id".to_string()));
        assert_eq!(subnet.region, "region".to_string());
        assert_eq!(subnet.cidr_block, "test_cidr_block".to_string());
        assert_eq!(subnet.vpc_id, Some("vpc_id".to_string()));
        assert_eq!(subnet.name, "test_name".to_string());
    }

    #[tokio::test]
    async fn test_security_group_state() {
        // Arrange
        let security_group = SecurityGroup::new(
            Some("id".to_string()),
            "name".to_string(),
            Some("vpc_id".to_string()),
            "description".to_string(),
            "region".to_string(),
            vec![],
        )
        .await;

        // Act
        let security_group_state = SecurityGroupState::new(&security_group);

        // Assert
        assert_eq!(security_group_state.id, "id".to_string());
        assert_eq!(security_group_state.name, "name".to_string());
        assert_eq!(security_group_state.vpc_id, "vpc_id".to_string());
        assert_eq!(security_group_state.description, "description".to_string());
        assert_eq!(security_group_state.region, "region".to_string());
        assert_eq!(security_group_state.inbound_rules.len(), 0);
    }

    #[tokio::test]
    async fn test_security_group_state_new_from_state() {
        // Arrange
        let security_group_state = SecurityGroupState {
            id: "id".to_string(),
            name: "name".to_string(),
            vpc_id: "vpc_id".to_string(),
            description: "description".to_string(),
            region: "region".to_string(),
            inbound_rules: vec![],
        };

        // Act
        let security_group = security_group_state.new_from_state().await;

        // Assert
        assert_eq!(security_group.id, Some("id".to_string()));
        assert_eq!(security_group.name, "name".to_string());
        assert_eq!(security_group.vpc_id, Some("vpc_id".to_string()));
        assert_eq!(security_group.description, "description".to_string());
        assert_eq!(security_group.region, "region".to_string());
        assert_eq!(security_group.inbound_rules.len(), 0);
    }

    #[test]
    fn test_inbound_rule_state() {
        // Arrange
        let inbound_rule = InboundRule {
            protocol: "tcp".to_string(),
            port: 22,
            cidr_block: "0.0.0.0/0".to_string(),
        };

        // Act
        let inbound_rule_state = InboundRuleState::new(&inbound_rule);

        // Assert
        assert_eq!(inbound_rule_state.protocol, "tcp".to_string());
        assert_eq!(inbound_rule_state.port, 22);
        assert_eq!(inbound_rule_state.cidr_block, "0.0.0.0/0".to_string());
    }

    #[test]
    fn test_inbound_rule_new_from_state() {
        // Arrange
        let inbound_rule = InboundRuleState {
            protocol: "tcp".to_string(),
            port: 22,
            cidr_block: "0.0.0.0/0".to_string(),
        };

        // Act
        let inbound_rule = inbound_rule.new_from_state();

        // Assert
        assert_eq!(inbound_rule.protocol, "tcp".to_string());
        assert_eq!(inbound_rule.port, 22);
        assert_eq!(inbound_rule.cidr_block, "0.0.0.0/0".to_string());
    }

    #[tokio::test]
    async fn test_route_table_state() {
        // Arrange
        let route_table = RouteTable::new(
            Some("id".to_string()),
            Some("vpc_id".to_string()),
            Some("subnet_id".to_string()),
            "region".to_string(),
        )
        .await;

        // Act
        let route_table_state = RouteTableState::new(&route_table);

        // Assert
        assert_eq!(route_table_state.id, "id".to_string());
        assert_eq!(route_table_state.vpc_id, "vpc_id".to_string());
        assert_eq!(route_table_state.subnet_id, "subnet_id".to_string());
        assert_eq!(route_table_state.region, "region".to_string());
    }

    #[tokio::test]
    async fn test_route_table_state_new_from_state() {
        // Arrange
        let route_table_state = RouteTableState {
            id: "id".to_string(),
            vpc_id: "vpc_id".to_string(),
            subnet_id: "subnet_id".to_string(),
            region: "region".to_string(),
        };

        // Act
        let route_table = route_table_state.new_from_state().await;

        // Assert
        assert_eq!(route_table.id, Some("id".to_string()));
        assert_eq!(route_table.vpc_id, Some("vpc_id".to_string()));
        assert_eq!(route_table.subnet_id, Some("subnet_id".to_string()));
        assert_eq!(route_table.region, "region".to_string());
    }

    #[tokio::test]
    async fn test_internet_gateway_state() {
        // Arrange
        let internet_gateway = InternetGateway::new(
            Some("id".to_string()),
            Some("vpc_id".to_string()),
            Some("route_table_id".to_string()),
            Some("subnet_id".to_string()),
            "region".to_string(),
        )
        .await;

        // Act
        let internet_gateway_state = InternetGatewayState::new(&internet_gateway);

        // Assert
        assert_eq!(internet_gateway_state.id, "id".to_string());
        assert_eq!(internet_gateway_state.vpc_id, "vpc_id".to_string());
        assert_eq!(
            internet_gateway_state.route_table_id,
            "route_table_id".to_string()
        );
        assert_eq!(internet_gateway_state.subnet_id, "subnet_id".to_string());
        assert_eq!(internet_gateway_state.region, "region".to_string());
    }

    #[tokio::test]
    async fn test_internet_gateway_state_new_from_state() {
        // Arrange
        let internet_gateway_state = InternetGatewayState {
            id: "id".to_string(),
            vpc_id: "vpc_id".to_string(),
            route_table_id: "route_table_id".to_string(),
            subnet_id: "subnet_id".to_string(),
            region: "region".to_string(),
        };

        // Act
        let internet_gateway = internet_gateway_state.new_from_state().await;

        // Assert
        assert_eq!(internet_gateway.id, Some("id".to_string()));
        assert_eq!(internet_gateway.vpc_id, Some("vpc_id".to_string()));
        assert_eq!(
            internet_gateway.route_table_id,
            Some("route_table_id".to_string())
        );
        assert_eq!(internet_gateway.subnet_id, Some("subnet_id".to_string()));
        assert_eq!(internet_gateway.region, "region".to_string());
    }
}
