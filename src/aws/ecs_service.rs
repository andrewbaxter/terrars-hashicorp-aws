#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcsServiceData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone_rebalancing: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_maximum_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_minimum_healthy_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_ecs_managed_tags: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_execute_command: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_delete: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_new_deployment: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check_grace_period_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_type: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagate_tags: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduling_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sigint_rollback: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_definition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    triggers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for_steady_state: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alarms: std::option::Option<std::vec::Vec<EcsService_Alarms>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_provider_strategy: std::option::Option<std::vec::Vec<EcsService_CapacityProviderStrategy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_circuit_breaker: std::option::Option<std::vec::Vec<EcsService_DeploymentCircuitBreaker>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_configuration: std::option::Option<std::vec::Vec<EcsService_DeploymentConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_controller: std::option::Option<std::vec::Vec<EcsService_DeploymentController>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer: std::option::Option<std::vec::Vec<EcsService_LoadBalancer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std::option::Option<std::vec::Vec<EcsService_NetworkConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ordered_placement_strategy: std::option::Option<std::vec::Vec<EcsService_OrderedPlacementStrategy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_constraints: std::option::Option<std::vec::Vec<EcsService_PlacementConstraints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_connect_configuration: std::option::Option<std::vec::Vec<EcsService_ServiceConnectConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_registries: std::option::Option<std::vec::Vec<EcsService_ServiceRegistries>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EcsService_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_configuration: std::option::Option<std::vec::Vec<EcsService_VolumeConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_lattice_configurations: std::option::Option<std::vec::Vec<EcsService_VpcLatticeConfigurations>>,
    dynamic: EcsServiceDynamic,
}

struct EcsService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EcsServiceData_>,
}

#[derive(Clone)]
pub struct EcsService(std::rc::Rc<EcsService_>);

impl EcsService {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc = "Set the field `availability_zone_rebalancing`.\n"]
    pub fn set_availability_zone_rebalancing(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().availability_zone_rebalancing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster`.\n"]
    pub fn set_cluster(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_maximum_percent`.\n"]
    pub fn set_deployment_maximum_percent(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().deployment_maximum_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_minimum_healthy_percent`.\n"]
    pub fn set_deployment_minimum_healthy_percent(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().deployment_minimum_healthy_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `desired_count`.\n"]
    pub fn set_desired_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().desired_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_ecs_managed_tags`.\n"]
    pub fn set_enable_ecs_managed_tags(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_ecs_managed_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_execute_command`.\n"]
    pub fn set_enable_execute_command(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_execute_command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `force_delete`.\n"]
    pub fn set_force_delete(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `force_new_deployment`.\n"]
    pub fn set_force_new_deployment(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_new_deployment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check_grace_period_seconds`.\n"]
    pub fn set_health_check_grace_period_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().health_check_grace_period_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iam_role`.\n"]
    pub fn set_iam_role(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().iam_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_type`.\n"]
    pub fn set_launch_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().launch_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `platform_version`.\n"]
    pub fn set_platform_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().platform_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `propagate_tags`.\n"]
    pub fn set_propagate_tags(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().propagate_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scheduling_strategy`.\n"]
    pub fn set_scheduling_strategy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().scheduling_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sigint_rollback`.\n"]
    pub fn set_sigint_rollback(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().sigint_rollback = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `task_definition`.\n"]
    pub fn set_task_definition(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().task_definition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `triggers`.\n"]
    pub fn set_triggers(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().triggers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wait_for_steady_state`.\n"]
    pub fn set_wait_for_steady_state(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_steady_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alarms`.\n"]
    pub fn set_alarms(self, v: impl std::convert::Into<terrars::BlockAssignable<EcsService_Alarms>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().alarms = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.alarms = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `capacity_provider_strategy`.\n"]
    pub fn set_capacity_provider_strategy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsService_CapacityProviderStrategy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity_provider_strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capacity_provider_strategy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deployment_circuit_breaker`.\n"]
    pub fn set_deployment_circuit_breaker(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsService_DeploymentCircuitBreaker>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deployment_circuit_breaker = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deployment_circuit_breaker = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deployment_configuration`.\n"]
    pub fn set_deployment_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsService_DeploymentConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deployment_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deployment_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deployment_controller`.\n"]
    pub fn set_deployment_controller(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsService_DeploymentController>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deployment_controller = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deployment_controller = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `load_balancer`.\n"]
    pub fn set_load_balancer(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsService_LoadBalancer>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().load_balancer = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.load_balancer = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsService_NetworkConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ordered_placement_strategy`.\n"]
    pub fn set_ordered_placement_strategy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsService_OrderedPlacementStrategy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ordered_placement_strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ordered_placement_strategy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placement_constraints`.\n"]
    pub fn set_placement_constraints(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsService_PlacementConstraints>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().placement_constraints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.placement_constraints = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_connect_configuration`.\n"]
    pub fn set_service_connect_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsService_ServiceConnectConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_connect_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_connect_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_registries`.\n"]
    pub fn set_service_registries(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsService_ServiceRegistries>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_registries = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_registries = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<EcsService_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_configuration`.\n"]
    pub fn set_volume_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsService_VolumeConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().volume_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.volume_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_lattice_configurations`.\n"]
    pub fn set_vpc_lattice_configurations(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsService_VpcLatticeConfigurations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_lattice_configurations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_lattice_configurations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_rebalancing` after provisioning.\n"]
    pub fn availability_zone_rebalancing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_rebalancing", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_maximum_percent` after provisioning.\n"]
    pub fn deployment_maximum_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_maximum_percent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_minimum_healthy_percent` after provisioning.\n"]
    pub fn deployment_minimum_healthy_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_minimum_healthy_percent", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `desired_count` after provisioning.\n"]
    pub fn desired_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_ecs_managed_tags` after provisioning.\n"]
    pub fn enable_ecs_managed_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_ecs_managed_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_execute_command` after provisioning.\n"]
    pub fn enable_execute_command(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_execute_command", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\n"]
    pub fn force_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_delete", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_new_deployment` after provisioning.\n"]
    pub fn force_new_deployment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_new_deployment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_grace_period_seconds` after provisioning.\n"]
    pub fn health_check_grace_period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_grace_period_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `iam_role` after provisioning.\n"]
    pub fn iam_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_type` after provisioning.\n"]
    pub fn launch_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduling_strategy` after provisioning.\n"]
    pub fn scheduling_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduling_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sigint_rollback` after provisioning.\n"]
    pub fn sigint_rollback(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sigint_rollback", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `triggers` after provisioning.\n"]
    pub fn triggers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.triggers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_steady_state` after provisioning.\n"]
    pub fn wait_for_steady_state(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_steady_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alarms` after provisioning.\n"]
    pub fn alarms(&self) -> terrars::ListRef<EcsService_AlarmsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alarms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_circuit_breaker` after provisioning.\n"]
    pub fn deployment_circuit_breaker(&self) -> terrars::ListRef<EcsService_DeploymentCircuitBreakerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_circuit_breaker", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_configuration` after provisioning.\n"]
    pub fn deployment_configuration(&self) -> terrars::ListRef<EcsService_DeploymentConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_controller` after provisioning.\n"]
    pub fn deployment_controller(&self) -> terrars::ListRef<EcsService_DeploymentControllerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_controller", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<EcsService_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ordered_placement_strategy` after provisioning.\n"]
    pub fn ordered_placement_strategy(&self) -> terrars::ListRef<EcsService_OrderedPlacementStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ordered_placement_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_connect_configuration` after provisioning.\n"]
    pub fn service_connect_configuration(&self) -> terrars::ListRef<EcsService_ServiceConnectConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_connect_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_registries` after provisioning.\n"]
    pub fn service_registries(&self) -> terrars::ListRef<EcsService_ServiceRegistriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_registries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EcsService_TimeoutsRef {
        EcsService_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_configuration` after provisioning.\n"]
    pub fn volume_configuration(&self) -> terrars::ListRef<EcsService_VolumeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volume_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for EcsService {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EcsService { }

impl terrars::ToListMappable for EcsService {
    type O = terrars::ListRef<EcsServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EcsService_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ecs_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEcsService {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildEcsService {
    pub fn build(self, stack: &mut terrars::Stack) -> EcsService {
        let out = EcsService(std::rc::Rc::new(EcsService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EcsServiceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                availability_zone_rebalancing: core::default::Default::default(),
                cluster: core::default::Default::default(),
                deployment_maximum_percent: core::default::Default::default(),
                deployment_minimum_healthy_percent: core::default::Default::default(),
                desired_count: core::default::Default::default(),
                enable_ecs_managed_tags: core::default::Default::default(),
                enable_execute_command: core::default::Default::default(),
                force_delete: core::default::Default::default(),
                force_new_deployment: core::default::Default::default(),
                health_check_grace_period_seconds: core::default::Default::default(),
                iam_role: core::default::Default::default(),
                id: core::default::Default::default(),
                launch_type: core::default::Default::default(),
                name: self.name,
                platform_version: core::default::Default::default(),
                propagate_tags: core::default::Default::default(),
                region: core::default::Default::default(),
                scheduling_strategy: core::default::Default::default(),
                sigint_rollback: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                task_definition: core::default::Default::default(),
                triggers: core::default::Default::default(),
                wait_for_steady_state: core::default::Default::default(),
                alarms: core::default::Default::default(),
                capacity_provider_strategy: core::default::Default::default(),
                deployment_circuit_breaker: core::default::Default::default(),
                deployment_configuration: core::default::Default::default(),
                deployment_controller: core::default::Default::default(),
                load_balancer: core::default::Default::default(),
                network_configuration: core::default::Default::default(),
                ordered_placement_strategy: core::default::Default::default(),
                placement_constraints: core::default::Default::default(),
                service_connect_configuration: core::default::Default::default(),
                service_registries: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                volume_configuration: core::default::Default::default(),
                vpc_lattice_configurations: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EcsServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EcsServiceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_rebalancing` after provisioning.\n"]
    pub fn availability_zone_rebalancing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_rebalancing", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_maximum_percent` after provisioning.\n"]
    pub fn deployment_maximum_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_maximum_percent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_minimum_healthy_percent` after provisioning.\n"]
    pub fn deployment_minimum_healthy_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_minimum_healthy_percent", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `desired_count` after provisioning.\n"]
    pub fn desired_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_ecs_managed_tags` after provisioning.\n"]
    pub fn enable_ecs_managed_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_ecs_managed_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_execute_command` after provisioning.\n"]
    pub fn enable_execute_command(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_execute_command", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\n"]
    pub fn force_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_delete", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_new_deployment` after provisioning.\n"]
    pub fn force_new_deployment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_new_deployment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_grace_period_seconds` after provisioning.\n"]
    pub fn health_check_grace_period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_grace_period_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `iam_role` after provisioning.\n"]
    pub fn iam_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_type` after provisioning.\n"]
    pub fn launch_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduling_strategy` after provisioning.\n"]
    pub fn scheduling_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduling_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sigint_rollback` after provisioning.\n"]
    pub fn sigint_rollback(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sigint_rollback", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `triggers` after provisioning.\n"]
    pub fn triggers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.triggers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_steady_state` after provisioning.\n"]
    pub fn wait_for_steady_state(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_steady_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alarms` after provisioning.\n"]
    pub fn alarms(&self) -> terrars::ListRef<EcsService_AlarmsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alarms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_circuit_breaker` after provisioning.\n"]
    pub fn deployment_circuit_breaker(&self) -> terrars::ListRef<EcsService_DeploymentCircuitBreakerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_circuit_breaker", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_configuration` after provisioning.\n"]
    pub fn deployment_configuration(&self) -> terrars::ListRef<EcsService_DeploymentConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_controller` after provisioning.\n"]
    pub fn deployment_controller(&self) -> terrars::ListRef<EcsService_DeploymentControllerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_controller", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<EcsService_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ordered_placement_strategy` after provisioning.\n"]
    pub fn ordered_placement_strategy(&self) -> terrars::ListRef<EcsService_OrderedPlacementStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ordered_placement_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_connect_configuration` after provisioning.\n"]
    pub fn service_connect_configuration(&self) -> terrars::ListRef<EcsService_ServiceConnectConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_connect_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_registries` after provisioning.\n"]
    pub fn service_registries(&self) -> terrars::ListRef<EcsService_ServiceRegistriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_registries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EcsService_TimeoutsRef {
        EcsService_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_configuration` after provisioning.\n"]
    pub fn volume_configuration(&self) -> terrars::ListRef<EcsService_VolumeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volume_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct EcsServiceDynamic {
    alarms: std::option::Option<terrars::DynamicBlock<EcsService_Alarms>>,
    capacity_provider_strategy: std::option::Option<terrars::DynamicBlock<EcsService_CapacityProviderStrategy>>,
    deployment_circuit_breaker: std::option::Option<terrars::DynamicBlock<EcsService_DeploymentCircuitBreaker>>,
    deployment_configuration: std::option::Option<terrars::DynamicBlock<EcsService_DeploymentConfiguration>>,
    deployment_controller: std::option::Option<terrars::DynamicBlock<EcsService_DeploymentController>>,
    load_balancer: std::option::Option<terrars::DynamicBlock<EcsService_LoadBalancer>>,
    network_configuration: std::option::Option<terrars::DynamicBlock<EcsService_NetworkConfiguration>>,
    ordered_placement_strategy: std::option::Option<terrars::DynamicBlock<EcsService_OrderedPlacementStrategy>>,
    placement_constraints: std::option::Option<terrars::DynamicBlock<EcsService_PlacementConstraints>>,
    service_connect_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsService_ServiceConnectConfiguration>>,
    service_registries: std::option::Option<terrars::DynamicBlock<EcsService_ServiceRegistries>>,
    volume_configuration: std::option::Option<terrars::DynamicBlock<EcsService_VolumeConfiguration>>,
    vpc_lattice_configurations: std::option::Option<terrars::DynamicBlock<EcsService_VpcLatticeConfigurations>>,
}

#[derive(Serialize)]
pub struct EcsService_Alarms {
    alarm_names: terrars::SetField<terrars::PrimField<std::string::String>>,
    enable: terrars::PrimField<bool>,
    rollback: terrars::PrimField<bool>,
}

impl EcsService_Alarms { }

impl terrars::ToListMappable for EcsService_Alarms {
    type O = terrars::BlockAssignable<EcsService_Alarms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_Alarms {
    #[doc = ""]
    pub alarm_names: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub enable: terrars::PrimField<bool>,
    #[doc = ""]
    pub rollback: terrars::PrimField<bool>,
}

impl BuildEcsService_Alarms {
    pub fn build(self) -> EcsService_Alarms {
        EcsService_Alarms {
            alarm_names: self.alarm_names,
            enable: self.enable,
            rollback: self.rollback,
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_AlarmsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_AlarmsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsService_AlarmsRef {
        EcsService_AlarmsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_AlarmsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alarm_names` after provisioning.\n"]
    pub fn alarm_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.alarm_names", self.base))
    }

    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc = "Get a reference to the value of field `rollback` after provisioning.\n"]
    pub fn rollback(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollback", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsService_CapacityProviderStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base: std::option::Option<terrars::PrimField<f64>>,
    capacity_provider: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl EcsService_CapacityProviderStrategy {
    #[doc = "Set the field `base`.\n"]
    pub fn set_base(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.base = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsService_CapacityProviderStrategy {
    type O = terrars::BlockAssignable<EcsService_CapacityProviderStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_CapacityProviderStrategy {
    #[doc = ""]
    pub capacity_provider: terrars::PrimField<std::string::String>,
}

impl BuildEcsService_CapacityProviderStrategy {
    pub fn build(self) -> EcsService_CapacityProviderStrategy {
        EcsService_CapacityProviderStrategy {
            base: core::default::Default::default(),
            capacity_provider: self.capacity_provider,
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_CapacityProviderStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_CapacityProviderStrategyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsService_CapacityProviderStrategyRef {
        EcsService_CapacityProviderStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_CapacityProviderStrategyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base` after provisioning.\n"]
    pub fn base(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_provider` after provisioning.\n"]
    pub fn capacity_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_provider", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsService_DeploymentCircuitBreaker {
    enable: terrars::PrimField<bool>,
    rollback: terrars::PrimField<bool>,
}

impl EcsService_DeploymentCircuitBreaker { }

impl terrars::ToListMappable for EcsService_DeploymentCircuitBreaker {
    type O = terrars::BlockAssignable<EcsService_DeploymentCircuitBreaker>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_DeploymentCircuitBreaker {
    #[doc = ""]
    pub enable: terrars::PrimField<bool>,
    #[doc = ""]
    pub rollback: terrars::PrimField<bool>,
}

impl BuildEcsService_DeploymentCircuitBreaker {
    pub fn build(self) -> EcsService_DeploymentCircuitBreaker {
        EcsService_DeploymentCircuitBreaker {
            enable: self.enable,
            rollback: self.rollback,
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_DeploymentCircuitBreakerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_DeploymentCircuitBreakerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsService_DeploymentCircuitBreakerRef {
        EcsService_DeploymentCircuitBreakerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_DeploymentCircuitBreakerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc = "Get a reference to the value of field `rollback` after provisioning.\n"]
    pub fn rollback(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollback", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsService_DeploymentConfiguration_CanaryConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canary_bake_time_in_minutes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canary_percent: std::option::Option<terrars::PrimField<f64>>,
}

impl EcsService_DeploymentConfiguration_CanaryConfiguration {
    #[doc = "Set the field `canary_bake_time_in_minutes`.\n"]
    pub fn set_canary_bake_time_in_minutes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.canary_bake_time_in_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `canary_percent`.\n"]
    pub fn set_canary_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.canary_percent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsService_DeploymentConfiguration_CanaryConfiguration {
    type O = terrars::BlockAssignable<EcsService_DeploymentConfiguration_CanaryConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_DeploymentConfiguration_CanaryConfiguration {}

impl BuildEcsService_DeploymentConfiguration_CanaryConfiguration {
    pub fn build(self) -> EcsService_DeploymentConfiguration_CanaryConfiguration {
        EcsService_DeploymentConfiguration_CanaryConfiguration {
            canary_bake_time_in_minutes: core::default::Default::default(),
            canary_percent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_DeploymentConfiguration_CanaryConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_DeploymentConfiguration_CanaryConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsService_DeploymentConfiguration_CanaryConfigurationRef {
        EcsService_DeploymentConfiguration_CanaryConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_DeploymentConfiguration_CanaryConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `canary_bake_time_in_minutes` after provisioning.\n"]
    pub fn canary_bake_time_in_minutes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.canary_bake_time_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `canary_percent` after provisioning.\n"]
    pub fn canary_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.canary_percent", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_in_minutes: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfiguration {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_in_minutes`.\n"]
    pub fn set_timeout_in_minutes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.timeout_in_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfiguration {
    type O = terrars::BlockAssignable<EcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfiguration {}

impl BuildEcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfiguration {
    pub fn build(self) -> EcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfiguration {
        EcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfiguration {
            action: core::default::Default::default(),
            timeout_in_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfigurationRef {
        EcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_in_minutes` after provisioning.\n"]
    pub fn timeout_in_minutes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_in_minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsService_DeploymentConfiguration_LifecycleHookDynamic {
    timeout_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfiguration>>,
}

#[derive(Serialize)]
pub struct EcsService_DeploymentConfiguration_LifecycleHook {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hook_details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hook_target_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    lifecycle_stages: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_configuration: std
    ::option
    ::Option<std::vec::Vec<EcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfiguration>>,
    dynamic: EcsService_DeploymentConfiguration_LifecycleHookDynamic,
}

impl EcsService_DeploymentConfiguration_LifecycleHook {
    #[doc = "Set the field `hook_details`.\n"]
    pub fn set_hook_details(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hook_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hook_target_arn`.\n"]
    pub fn set_hook_target_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hook_target_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_type`.\n"]
    pub fn set_target_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_configuration`.\n"]
    pub fn set_timeout_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<EcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timeout_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsService_DeploymentConfiguration_LifecycleHook {
    type O = terrars::BlockAssignable<EcsService_DeploymentConfiguration_LifecycleHook>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_DeploymentConfiguration_LifecycleHook {
    #[doc = ""]
    pub lifecycle_stages: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildEcsService_DeploymentConfiguration_LifecycleHook {
    pub fn build(self) -> EcsService_DeploymentConfiguration_LifecycleHook {
        EcsService_DeploymentConfiguration_LifecycleHook {
            hook_details: core::default::Default::default(),
            hook_target_arn: core::default::Default::default(),
            lifecycle_stages: self.lifecycle_stages,
            role_arn: core::default::Default::default(),
            target_type: core::default::Default::default(),
            timeout_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_DeploymentConfiguration_LifecycleHookRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_DeploymentConfiguration_LifecycleHookRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsService_DeploymentConfiguration_LifecycleHookRef {
        EcsService_DeploymentConfiguration_LifecycleHookRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_DeploymentConfiguration_LifecycleHookRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hook_details` after provisioning.\n"]
    pub fn hook_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hook_details", self.base))
    }

    #[doc = "Get a reference to the value of field `hook_target_arn` after provisioning.\n"]
    pub fn hook_target_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hook_target_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_stages` after provisioning.\n"]
    pub fn lifecycle_stages(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_stages", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `target_type` after provisioning.\n"]
    pub fn target_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_type", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_configuration` after provisioning.\n"]
    pub fn timeout_configuration(
        &self,
    ) -> terrars::ListRef<EcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsService_DeploymentConfiguration_LinearConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step_bake_time_in_minutes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step_percent: std::option::Option<terrars::PrimField<f64>>,
}

impl EcsService_DeploymentConfiguration_LinearConfiguration {
    #[doc = "Set the field `step_bake_time_in_minutes`.\n"]
    pub fn set_step_bake_time_in_minutes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.step_bake_time_in_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `step_percent`.\n"]
    pub fn set_step_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.step_percent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsService_DeploymentConfiguration_LinearConfiguration {
    type O = terrars::BlockAssignable<EcsService_DeploymentConfiguration_LinearConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_DeploymentConfiguration_LinearConfiguration {}

impl BuildEcsService_DeploymentConfiguration_LinearConfiguration {
    pub fn build(self) -> EcsService_DeploymentConfiguration_LinearConfiguration {
        EcsService_DeploymentConfiguration_LinearConfiguration {
            step_bake_time_in_minutes: core::default::Default::default(),
            step_percent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_DeploymentConfiguration_LinearConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_DeploymentConfiguration_LinearConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsService_DeploymentConfiguration_LinearConfigurationRef {
        EcsService_DeploymentConfiguration_LinearConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_DeploymentConfiguration_LinearConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `step_bake_time_in_minutes` after provisioning.\n"]
    pub fn step_bake_time_in_minutes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.step_bake_time_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `step_percent` after provisioning.\n"]
    pub fn step_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.step_percent", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsService_DeploymentConfigurationDynamic {
    canary_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsService_DeploymentConfiguration_CanaryConfiguration>>,
    lifecycle_hook: std::option::Option<terrars::DynamicBlock<EcsService_DeploymentConfiguration_LifecycleHook>>,
    linear_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsService_DeploymentConfiguration_LinearConfiguration>>,
}

#[derive(Serialize)]
pub struct EcsService_DeploymentConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bake_time_in_minutes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canary_configuration: std::option::Option<std::vec::Vec<EcsService_DeploymentConfiguration_CanaryConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_hook: std::option::Option<std::vec::Vec<EcsService_DeploymentConfiguration_LifecycleHook>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linear_configuration: std::option::Option<std::vec::Vec<EcsService_DeploymentConfiguration_LinearConfiguration>>,
    dynamic: EcsService_DeploymentConfigurationDynamic,
}

impl EcsService_DeploymentConfiguration {
    #[doc = "Set the field `bake_time_in_minutes`.\n"]
    pub fn set_bake_time_in_minutes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.bake_time_in_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `strategy`.\n"]
    pub fn set_strategy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `canary_configuration`.\n"]
    pub fn set_canary_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsService_DeploymentConfiguration_CanaryConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.canary_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.canary_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lifecycle_hook`.\n"]
    pub fn set_lifecycle_hook(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsService_DeploymentConfiguration_LifecycleHook>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lifecycle_hook = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lifecycle_hook = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `linear_configuration`.\n"]
    pub fn set_linear_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsService_DeploymentConfiguration_LinearConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.linear_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.linear_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsService_DeploymentConfiguration {
    type O = terrars::BlockAssignable<EcsService_DeploymentConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_DeploymentConfiguration {}

impl BuildEcsService_DeploymentConfiguration {
    pub fn build(self) -> EcsService_DeploymentConfiguration {
        EcsService_DeploymentConfiguration {
            bake_time_in_minutes: core::default::Default::default(),
            strategy: core::default::Default::default(),
            canary_configuration: core::default::Default::default(),
            lifecycle_hook: core::default::Default::default(),
            linear_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_DeploymentConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_DeploymentConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsService_DeploymentConfigurationRef {
        EcsService_DeploymentConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_DeploymentConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bake_time_in_minutes` after provisioning.\n"]
    pub fn bake_time_in_minutes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bake_time_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `strategy` after provisioning.\n"]
    pub fn strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `canary_configuration` after provisioning.\n"]
    pub fn canary_configuration(&self) -> terrars::ListRef<EcsService_DeploymentConfiguration_CanaryConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.canary_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `linear_configuration` after provisioning.\n"]
    pub fn linear_configuration(&self) -> terrars::ListRef<EcsService_DeploymentConfiguration_LinearConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linear_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsService_DeploymentController {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsService_DeploymentController {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsService_DeploymentController {
    type O = terrars::BlockAssignable<EcsService_DeploymentController>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_DeploymentController {}

impl BuildEcsService_DeploymentController {
    pub fn build(self) -> EcsService_DeploymentController {
        EcsService_DeploymentController { type_: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EcsService_DeploymentControllerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_DeploymentControllerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsService_DeploymentControllerRef {
        EcsService_DeploymentControllerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_DeploymentControllerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsService_LoadBalancer_AdvancedConfiguration {
    alternate_target_group_arn: terrars::PrimField<std::string::String>,
    production_listener_rule: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_listener_rule: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsService_LoadBalancer_AdvancedConfiguration {
    #[doc = "Set the field `test_listener_rule`.\n"]
    pub fn set_test_listener_rule(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.test_listener_rule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsService_LoadBalancer_AdvancedConfiguration {
    type O = terrars::BlockAssignable<EcsService_LoadBalancer_AdvancedConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_LoadBalancer_AdvancedConfiguration {
    #[doc = ""]
    pub alternate_target_group_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub production_listener_rule: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildEcsService_LoadBalancer_AdvancedConfiguration {
    pub fn build(self) -> EcsService_LoadBalancer_AdvancedConfiguration {
        EcsService_LoadBalancer_AdvancedConfiguration {
            alternate_target_group_arn: self.alternate_target_group_arn,
            production_listener_rule: self.production_listener_rule,
            role_arn: self.role_arn,
            test_listener_rule: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_LoadBalancer_AdvancedConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_LoadBalancer_AdvancedConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsService_LoadBalancer_AdvancedConfigurationRef {
        EcsService_LoadBalancer_AdvancedConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_LoadBalancer_AdvancedConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alternate_target_group_arn` after provisioning.\n"]
    pub fn alternate_target_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alternate_target_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `production_listener_rule` after provisioning.\n"]
    pub fn production_listener_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.production_listener_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `test_listener_rule` after provisioning.\n"]
    pub fn test_listener_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_listener_rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsService_LoadBalancerDynamic {
    advanced_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsService_LoadBalancer_AdvancedConfiguration>>,
}

#[derive(Serialize)]
pub struct EcsService_LoadBalancer {
    container_name: terrars::PrimField<std::string::String>,
    container_port: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elb_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_configuration: std::option::Option<std::vec::Vec<EcsService_LoadBalancer_AdvancedConfiguration>>,
    dynamic: EcsService_LoadBalancerDynamic,
}

impl EcsService_LoadBalancer {
    #[doc = "Set the field `elb_name`.\n"]
    pub fn set_elb_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.elb_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_group_arn`.\n"]
    pub fn set_target_group_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_group_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `advanced_configuration`.\n"]
    pub fn set_advanced_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsService_LoadBalancer_AdvancedConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.advanced_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsService_LoadBalancer {
    type O = terrars::BlockAssignable<EcsService_LoadBalancer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_LoadBalancer {
    #[doc = ""]
    pub container_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub container_port: terrars::PrimField<f64>,
}

impl BuildEcsService_LoadBalancer {
    pub fn build(self) -> EcsService_LoadBalancer {
        EcsService_LoadBalancer {
            container_name: self.container_name,
            container_port: self.container_port,
            elb_name: core::default::Default::default(),
            target_group_arn: core::default::Default::default(),
            advanced_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_LoadBalancerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_LoadBalancerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsService_LoadBalancerRef {
        EcsService_LoadBalancerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_LoadBalancerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_name` after provisioning.\n"]
    pub fn container_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_name", self.base))
    }

    #[doc = "Get a reference to the value of field `container_port` after provisioning.\n"]
    pub fn container_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_port", self.base))
    }

    #[doc = "Get a reference to the value of field `elb_name` after provisioning.\n"]
    pub fn elb_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.elb_name", self.base))
    }

    #[doc = "Get a reference to the value of field `target_group_arn` after provisioning.\n"]
    pub fn target_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_configuration` after provisioning.\n"]
    pub fn advanced_configuration(&self) -> terrars::ListRef<EcsService_LoadBalancer_AdvancedConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsService_NetworkConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    assign_public_ip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl EcsService_NetworkConfiguration {
    #[doc = "Set the field `assign_public_ip`.\n"]
    pub fn set_assign_public_ip(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.assign_public_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_groups = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsService_NetworkConfiguration {
    type O = terrars::BlockAssignable<EcsService_NetworkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_NetworkConfiguration {
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildEcsService_NetworkConfiguration {
    pub fn build(self) -> EcsService_NetworkConfiguration {
        EcsService_NetworkConfiguration {
            assign_public_ip: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            subnets: self.subnets,
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_NetworkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_NetworkConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsService_NetworkConfigurationRef {
        EcsService_NetworkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_NetworkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `assign_public_ip` after provisioning.\n"]
    pub fn assign_public_ip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.assign_public_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsService_OrderedPlacementStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl EcsService_OrderedPlacementStrategy {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsService_OrderedPlacementStrategy {
    type O = terrars::BlockAssignable<EcsService_OrderedPlacementStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_OrderedPlacementStrategy {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildEcsService_OrderedPlacementStrategy {
    pub fn build(self) -> EcsService_OrderedPlacementStrategy {
        EcsService_OrderedPlacementStrategy {
            field: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_OrderedPlacementStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_OrderedPlacementStrategyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsService_OrderedPlacementStrategyRef {
        EcsService_OrderedPlacementStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_OrderedPlacementStrategyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsService_PlacementConstraints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl EcsService_PlacementConstraints {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsService_PlacementConstraints {
    type O = terrars::BlockAssignable<EcsService_PlacementConstraints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_PlacementConstraints {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildEcsService_PlacementConstraints {
    pub fn build(self) -> EcsService_PlacementConstraints {
        EcsService_PlacementConstraints {
            expression: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_PlacementConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_PlacementConstraintsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsService_PlacementConstraintsRef {
        EcsService_PlacementConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_PlacementConstraintsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsService_ServiceConnectConfiguration_AccessLogConfiguration {
    format: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_query_parameters: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsService_ServiceConnectConfiguration_AccessLogConfiguration {
    #[doc = "Set the field `include_query_parameters`.\n"]
    pub fn set_include_query_parameters(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.include_query_parameters = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsService_ServiceConnectConfiguration_AccessLogConfiguration {
    type O = terrars::BlockAssignable<EcsService_ServiceConnectConfiguration_AccessLogConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_ServiceConnectConfiguration_AccessLogConfiguration {
    #[doc = ""]
    pub format: terrars::PrimField<std::string::String>,
}

impl BuildEcsService_ServiceConnectConfiguration_AccessLogConfiguration {
    pub fn build(self) -> EcsService_ServiceConnectConfiguration_AccessLogConfiguration {
        EcsService_ServiceConnectConfiguration_AccessLogConfiguration {
            format: self.format,
            include_query_parameters: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_ServiceConnectConfiguration_AccessLogConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_ServiceConnectConfiguration_AccessLogConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsService_ServiceConnectConfiguration_AccessLogConfigurationRef {
        EcsService_ServiceConnectConfiguration_AccessLogConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_ServiceConnectConfiguration_AccessLogConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `include_query_parameters` after provisioning.\n"]
    pub fn include_query_parameters(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_query_parameters", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsService_ServiceConnectConfiguration_LogConfiguration_SecretOption {
    name: terrars::PrimField<std::string::String>,
    value_from: terrars::PrimField<std::string::String>,
}

impl EcsService_ServiceConnectConfiguration_LogConfiguration_SecretOption { }

impl terrars::ToListMappable for EcsService_ServiceConnectConfiguration_LogConfiguration_SecretOption {
    type O = terrars::BlockAssignable<EcsService_ServiceConnectConfiguration_LogConfiguration_SecretOption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_ServiceConnectConfiguration_LogConfiguration_SecretOption {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value_from: terrars::PrimField<std::string::String>,
}

impl BuildEcsService_ServiceConnectConfiguration_LogConfiguration_SecretOption {
    pub fn build(self) -> EcsService_ServiceConnectConfiguration_LogConfiguration_SecretOption {
        EcsService_ServiceConnectConfiguration_LogConfiguration_SecretOption {
            name: self.name,
            value_from: self.value_from,
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_ServiceConnectConfiguration_LogConfiguration_SecretOptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_ServiceConnectConfiguration_LogConfiguration_SecretOptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsService_ServiceConnectConfiguration_LogConfiguration_SecretOptionRef {
        EcsService_ServiceConnectConfiguration_LogConfiguration_SecretOptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_ServiceConnectConfiguration_LogConfiguration_SecretOptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value_from` after provisioning.\n"]
    pub fn value_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_from", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsService_ServiceConnectConfiguration_LogConfigurationDynamic {
    secret_option: std
    ::option
    ::Option<terrars::DynamicBlock<EcsService_ServiceConnectConfiguration_LogConfiguration_SecretOption>>,
}

#[derive(Serialize)]
pub struct EcsService_ServiceConnectConfiguration_LogConfiguration {
    log_driver: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_option: std
    ::option
    ::Option<std::vec::Vec<EcsService_ServiceConnectConfiguration_LogConfiguration_SecretOption>>,
    dynamic: EcsService_ServiceConnectConfiguration_LogConfigurationDynamic,
}

impl EcsService_ServiceConnectConfiguration_LogConfiguration {
    #[doc = "Set the field `options`.\n"]
    pub fn set_options(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_option`.\n"]
    pub fn set_secret_option(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<EcsService_ServiceConnectConfiguration_LogConfiguration_SecretOption>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secret_option = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_option = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsService_ServiceConnectConfiguration_LogConfiguration {
    type O = terrars::BlockAssignable<EcsService_ServiceConnectConfiguration_LogConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_ServiceConnectConfiguration_LogConfiguration {
    #[doc = ""]
    pub log_driver: terrars::PrimField<std::string::String>,
}

impl BuildEcsService_ServiceConnectConfiguration_LogConfiguration {
    pub fn build(self) -> EcsService_ServiceConnectConfiguration_LogConfiguration {
        EcsService_ServiceConnectConfiguration_LogConfiguration {
            log_driver: self.log_driver,
            options: core::default::Default::default(),
            secret_option: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_ServiceConnectConfiguration_LogConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_ServiceConnectConfiguration_LogConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsService_ServiceConnectConfiguration_LogConfigurationRef {
        EcsService_ServiceConnectConfiguration_LogConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_ServiceConnectConfiguration_LogConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_driver` after provisioning.\n"]
    pub fn log_driver(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_driver", self.base))
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.options", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_option` after provisioning.\n"]
    pub fn secret_option(
        &self,
    ) -> terrars::ListRef<EcsService_ServiceConnectConfiguration_LogConfiguration_SecretOptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_option", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header_Value {
    exact: terrars::PrimField<std::string::String>,
}

impl EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header_Value { }

impl terrars
::ToListMappable for EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header_Value {
    type O =
        terrars
        ::BlockAssignable<EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header_Value>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header_Value {
    #[doc = ""]
    pub exact: terrars::PrimField<std::string::String>,
}

impl BuildEcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header_Value {
    pub fn build(self) -> EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header_Value {
        EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header_Value {
            exact: self.exact,
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header_ValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header_ValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header_ValueRef {
        EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header_ValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header_ValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_HeaderDynamic {
    value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header_Value>,
    >,
}

#[derive(Serialize)]
pub struct EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std
    ::option
    ::Option<std::vec::Vec<EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header_Value>>,
    dynamic: EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_HeaderDynamic,
}

impl EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header_Value,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header {
    type O =
        terrars::BlockAssignable<EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildEcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header {
    pub fn build(self) -> EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header {
        EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header {
            name: self.name,
            value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_HeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_HeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_HeaderRef {
        EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_HeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_HeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(
        &self,
    ) -> terrars
    ::ListRef<EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header_ValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRulesDynamic {
    header: std
    ::option
    ::Option<
        terrars::DynamicBlock<EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header>,
    >,
}

#[derive(Serialize)]
pub struct EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std
    ::option
    ::Option<std::vec::Vec<EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header>>,
    dynamic: EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRulesDynamic,
}

impl EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules {
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_Header,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules {
    type O = terrars::BlockAssignable<EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules {}

impl BuildEcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules {
    pub fn build(self) -> EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules {
        EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules {
            header: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRulesRef {
        EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(
        &self,
    ) -> terrars::ListRef<EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules_HeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsService_ServiceConnectConfiguration_Service_ClientAliasDynamic {
    test_traffic_rules: std
    ::option
    ::Option<terrars::DynamicBlock<EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules>>,
}

#[derive(Serialize)]
pub struct EcsService_ServiceConnectConfiguration_Service_ClientAlias {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    port: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_traffic_rules: std
    ::option
    ::Option<std::vec::Vec<EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules>>,
    dynamic: EcsService_ServiceConnectConfiguration_Service_ClientAliasDynamic,
}

impl EcsService_ServiceConnectConfiguration_Service_ClientAlias {
    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dns_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `test_traffic_rules`.\n"]
    pub fn set_test_traffic_rules(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRules>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.test_traffic_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.test_traffic_rules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsService_ServiceConnectConfiguration_Service_ClientAlias {
    type O = terrars::BlockAssignable<EcsService_ServiceConnectConfiguration_Service_ClientAlias>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_ServiceConnectConfiguration_Service_ClientAlias {
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildEcsService_ServiceConnectConfiguration_Service_ClientAlias {
    pub fn build(self) -> EcsService_ServiceConnectConfiguration_Service_ClientAlias {
        EcsService_ServiceConnectConfiguration_Service_ClientAlias {
            dns_name: core::default::Default::default(),
            port: self.port,
            test_traffic_rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_ServiceConnectConfiguration_Service_ClientAliasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_ServiceConnectConfiguration_Service_ClientAliasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsService_ServiceConnectConfiguration_Service_ClientAliasRef {
        EcsService_ServiceConnectConfiguration_Service_ClientAliasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_ServiceConnectConfiguration_Service_ClientAliasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `test_traffic_rules` after provisioning.\n"]
    pub fn test_traffic_rules(
        &self,
    ) -> terrars::ListRef<EcsService_ServiceConnectConfiguration_Service_ClientAlias_TestTrafficRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.test_traffic_rules", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsService_ServiceConnectConfiguration_Service_Timeout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_request_timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl EcsService_ServiceConnectConfiguration_Service_Timeout {
    #[doc = "Set the field `idle_timeout_seconds`.\n"]
    pub fn set_idle_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.idle_timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `per_request_timeout_seconds`.\n"]
    pub fn set_per_request_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.per_request_timeout_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsService_ServiceConnectConfiguration_Service_Timeout {
    type O = terrars::BlockAssignable<EcsService_ServiceConnectConfiguration_Service_Timeout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_ServiceConnectConfiguration_Service_Timeout {}

impl BuildEcsService_ServiceConnectConfiguration_Service_Timeout {
    pub fn build(self) -> EcsService_ServiceConnectConfiguration_Service_Timeout {
        EcsService_ServiceConnectConfiguration_Service_Timeout {
            idle_timeout_seconds: core::default::Default::default(),
            per_request_timeout_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_ServiceConnectConfiguration_Service_TimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_ServiceConnectConfiguration_Service_TimeoutRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsService_ServiceConnectConfiguration_Service_TimeoutRef {
        EcsService_ServiceConnectConfiguration_Service_TimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_ServiceConnectConfiguration_Service_TimeoutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_timeout_seconds` after provisioning.\n"]
    pub fn idle_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request_timeout_seconds` after provisioning.\n"]
    pub fn per_request_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_request_timeout_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsService_ServiceConnectConfiguration_Service_Tls_IssuerCertAuthority {
    aws_pca_authority_arn: terrars::PrimField<std::string::String>,
}

impl EcsService_ServiceConnectConfiguration_Service_Tls_IssuerCertAuthority { }

impl terrars::ToListMappable for EcsService_ServiceConnectConfiguration_Service_Tls_IssuerCertAuthority {
    type O = terrars::BlockAssignable<EcsService_ServiceConnectConfiguration_Service_Tls_IssuerCertAuthority>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_ServiceConnectConfiguration_Service_Tls_IssuerCertAuthority {
    #[doc = ""]
    pub aws_pca_authority_arn: terrars::PrimField<std::string::String>,
}

impl BuildEcsService_ServiceConnectConfiguration_Service_Tls_IssuerCertAuthority {
    pub fn build(self) -> EcsService_ServiceConnectConfiguration_Service_Tls_IssuerCertAuthority {
        EcsService_ServiceConnectConfiguration_Service_Tls_IssuerCertAuthority {
            aws_pca_authority_arn: self.aws_pca_authority_arn,
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_ServiceConnectConfiguration_Service_Tls_IssuerCertAuthorityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_ServiceConnectConfiguration_Service_Tls_IssuerCertAuthorityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsService_ServiceConnectConfiguration_Service_Tls_IssuerCertAuthorityRef {
        EcsService_ServiceConnectConfiguration_Service_Tls_IssuerCertAuthorityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_ServiceConnectConfiguration_Service_Tls_IssuerCertAuthorityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_pca_authority_arn` after provisioning.\n"]
    pub fn aws_pca_authority_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_pca_authority_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsService_ServiceConnectConfiguration_Service_TlsDynamic {
    issuer_cert_authority: std
    ::option
    ::Option<terrars::DynamicBlock<EcsService_ServiceConnectConfiguration_Service_Tls_IssuerCertAuthority>>,
}

#[derive(Serialize)]
pub struct EcsService_ServiceConnectConfiguration_Service_Tls {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer_cert_authority: std
    ::option
    ::Option<std::vec::Vec<EcsService_ServiceConnectConfiguration_Service_Tls_IssuerCertAuthority>>,
    dynamic: EcsService_ServiceConnectConfiguration_Service_TlsDynamic,
}

impl EcsService_ServiceConnectConfiguration_Service_Tls {
    #[doc = "Set the field `kms_key`.\n"]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issuer_cert_authority`.\n"]
    pub fn set_issuer_cert_authority(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<EcsService_ServiceConnectConfiguration_Service_Tls_IssuerCertAuthority>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.issuer_cert_authority = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.issuer_cert_authority = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsService_ServiceConnectConfiguration_Service_Tls {
    type O = terrars::BlockAssignable<EcsService_ServiceConnectConfiguration_Service_Tls>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_ServiceConnectConfiguration_Service_Tls {}

impl BuildEcsService_ServiceConnectConfiguration_Service_Tls {
    pub fn build(self) -> EcsService_ServiceConnectConfiguration_Service_Tls {
        EcsService_ServiceConnectConfiguration_Service_Tls {
            kms_key: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            issuer_cert_authority: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_ServiceConnectConfiguration_Service_TlsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_ServiceConnectConfiguration_Service_TlsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsService_ServiceConnectConfiguration_Service_TlsRef {
        EcsService_ServiceConnectConfiguration_Service_TlsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_ServiceConnectConfiguration_Service_TlsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer_cert_authority` after provisioning.\n"]
    pub fn issuer_cert_authority(
        &self,
    ) -> terrars::ListRef<EcsService_ServiceConnectConfiguration_Service_Tls_IssuerCertAuthorityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.issuer_cert_authority", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsService_ServiceConnectConfiguration_ServiceDynamic {
    client_alias: std
    ::option
    ::Option<terrars::DynamicBlock<EcsService_ServiceConnectConfiguration_Service_ClientAlias>>,
    timeout: std::option::Option<terrars::DynamicBlock<EcsService_ServiceConnectConfiguration_Service_Timeout>>,
    tls: std::option::Option<terrars::DynamicBlock<EcsService_ServiceConnectConfiguration_Service_Tls>>,
}

#[derive(Serialize)]
pub struct EcsService_ServiceConnectConfiguration_Service {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discovery_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_port_override: std::option::Option<terrars::PrimField<f64>>,
    port_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_alias: std::option::Option<std::vec::Vec<EcsService_ServiceConnectConfiguration_Service_ClientAlias>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<EcsService_ServiceConnectConfiguration_Service_Timeout>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls: std::option::Option<std::vec::Vec<EcsService_ServiceConnectConfiguration_Service_Tls>>,
    dynamic: EcsService_ServiceConnectConfiguration_ServiceDynamic,
}

impl EcsService_ServiceConnectConfiguration_Service {
    #[doc = "Set the field `discovery_name`.\n"]
    pub fn set_discovery_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.discovery_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ingress_port_override`.\n"]
    pub fn set_ingress_port_override(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ingress_port_override = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_alias`.\n"]
    pub fn set_client_alias(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EcsService_ServiceConnectConfiguration_Service_ClientAlias>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_alias = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_alias = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsService_ServiceConnectConfiguration_Service_Timeout>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timeout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsService_ServiceConnectConfiguration_Service_Tls>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tls = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tls = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsService_ServiceConnectConfiguration_Service {
    type O = terrars::BlockAssignable<EcsService_ServiceConnectConfiguration_Service>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_ServiceConnectConfiguration_Service {
    #[doc = ""]
    pub port_name: terrars::PrimField<std::string::String>,
}

impl BuildEcsService_ServiceConnectConfiguration_Service {
    pub fn build(self) -> EcsService_ServiceConnectConfiguration_Service {
        EcsService_ServiceConnectConfiguration_Service {
            discovery_name: core::default::Default::default(),
            ingress_port_override: core::default::Default::default(),
            port_name: self.port_name,
            client_alias: core::default::Default::default(),
            timeout: core::default::Default::default(),
            tls: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_ServiceConnectConfiguration_ServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_ServiceConnectConfiguration_ServiceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsService_ServiceConnectConfiguration_ServiceRef {
        EcsService_ServiceConnectConfiguration_ServiceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_ServiceConnectConfiguration_ServiceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `discovery_name` after provisioning.\n"]
    pub fn discovery_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discovery_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ingress_port_override` after provisioning.\n"]
    pub fn ingress_port_override(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress_port_override", self.base))
    }

    #[doc = "Get a reference to the value of field `port_name` after provisioning.\n"]
    pub fn port_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }

    #[doc = "Get a reference to the value of field `client_alias` after provisioning.\n"]
    pub fn client_alias(&self) -> terrars::ListRef<EcsService_ServiceConnectConfiguration_Service_ClientAliasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_alias", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<EcsService_ServiceConnectConfiguration_Service_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> terrars::ListRef<EcsService_ServiceConnectConfiguration_Service_TlsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsService_ServiceConnectConfigurationDynamic {
    access_log_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsService_ServiceConnectConfiguration_AccessLogConfiguration>>,
    log_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsService_ServiceConnectConfiguration_LogConfiguration>>,
    service: std::option::Option<terrars::DynamicBlock<EcsService_ServiceConnectConfiguration_Service>>,
}

#[derive(Serialize)]
pub struct EcsService_ServiceConnectConfiguration {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_log_configuration: std
    ::option
    ::Option<std::vec::Vec<EcsService_ServiceConnectConfiguration_AccessLogConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_configuration: std::option::Option<std::vec::Vec<EcsService_ServiceConnectConfiguration_LogConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<std::vec::Vec<EcsService_ServiceConnectConfiguration_Service>>,
    dynamic: EcsService_ServiceConnectConfigurationDynamic,
}

impl EcsService_ServiceConnectConfiguration {
    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_log_configuration`.\n"]
    pub fn set_access_log_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EcsService_ServiceConnectConfiguration_AccessLogConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access_log_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access_log_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_configuration`.\n"]
    pub fn set_log_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EcsService_ServiceConnectConfiguration_LogConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.log_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.log_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service`.\n"]
    pub fn set_service(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsService_ServiceConnectConfiguration_Service>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsService_ServiceConnectConfiguration {
    type O = terrars::BlockAssignable<EcsService_ServiceConnectConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_ServiceConnectConfiguration {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildEcsService_ServiceConnectConfiguration {
    pub fn build(self) -> EcsService_ServiceConnectConfiguration {
        EcsService_ServiceConnectConfiguration {
            enabled: self.enabled,
            namespace: core::default::Default::default(),
            access_log_configuration: core::default::Default::default(),
            log_configuration: core::default::Default::default(),
            service: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_ServiceConnectConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_ServiceConnectConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsService_ServiceConnectConfigurationRef {
        EcsService_ServiceConnectConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_ServiceConnectConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `access_log_configuration` after provisioning.\n"]
    pub fn access_log_configuration(
        &self,
    ) -> terrars::ListRef<EcsService_ServiceConnectConfiguration_AccessLogConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_log_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(&self) -> terrars::ListRef<EcsService_ServiceConnectConfiguration_LogConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::ListRef<EcsService_ServiceConnectConfiguration_ServiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsService_ServiceRegistries {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    registry_arn: terrars::PrimField<std::string::String>,
}

impl EcsService_ServiceRegistries {
    #[doc = "Set the field `container_name`.\n"]
    pub fn set_container_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.container_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_port`.\n"]
    pub fn set_container_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.container_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsService_ServiceRegistries {
    type O = terrars::BlockAssignable<EcsService_ServiceRegistries>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_ServiceRegistries {
    #[doc = ""]
    pub registry_arn: terrars::PrimField<std::string::String>,
}

impl BuildEcsService_ServiceRegistries {
    pub fn build(self) -> EcsService_ServiceRegistries {
        EcsService_ServiceRegistries {
            container_name: core::default::Default::default(),
            container_port: core::default::Default::default(),
            port: core::default::Default::default(),
            registry_arn: self.registry_arn,
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_ServiceRegistriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_ServiceRegistriesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsService_ServiceRegistriesRef {
        EcsService_ServiceRegistriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_ServiceRegistriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_name` after provisioning.\n"]
    pub fn container_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_name", self.base))
    }

    #[doc = "Get a reference to the value of field `container_port` after provisioning.\n"]
    pub fn container_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_port", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `registry_arn` after provisioning.\n"]
    pub fn registry_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsService_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsService_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsService_Timeouts {
    type O = terrars::BlockAssignable<EcsService_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_Timeouts {}

impl BuildEcsService_Timeouts {
    pub fn build(self) -> EcsService_Timeouts {
        EcsService_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsService_TimeoutsRef {
        EcsService_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsService_VolumeConfiguration_ManagedEbsVolume_TagSpecifications {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagate_tags: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl EcsService_VolumeConfiguration_ManagedEbsVolume_TagSpecifications {
    #[doc = "Set the field `propagate_tags`.\n"]
    pub fn set_propagate_tags(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.propagate_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsService_VolumeConfiguration_ManagedEbsVolume_TagSpecifications {
    type O = terrars::BlockAssignable<EcsService_VolumeConfiguration_ManagedEbsVolume_TagSpecifications>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_VolumeConfiguration_ManagedEbsVolume_TagSpecifications {
    #[doc = ""]
    pub resource_type: terrars::PrimField<std::string::String>,
}

impl BuildEcsService_VolumeConfiguration_ManagedEbsVolume_TagSpecifications {
    pub fn build(self) -> EcsService_VolumeConfiguration_ManagedEbsVolume_TagSpecifications {
        EcsService_VolumeConfiguration_ManagedEbsVolume_TagSpecifications {
            propagate_tags: core::default::Default::default(),
            resource_type: self.resource_type,
            tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_VolumeConfiguration_ManagedEbsVolume_TagSpecificationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_VolumeConfiguration_ManagedEbsVolume_TagSpecificationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsService_VolumeConfiguration_ManagedEbsVolume_TagSpecificationsRef {
        EcsService_VolumeConfiguration_ManagedEbsVolume_TagSpecificationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_VolumeConfiguration_ManagedEbsVolume_TagSpecificationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsService_VolumeConfiguration_ManagedEbsVolumeDynamic {
    tag_specifications: std
    ::option
    ::Option<terrars::DynamicBlock<EcsService_VolumeConfiguration_ManagedEbsVolume_TagSpecifications>>,
}

#[derive(Serialize)]
pub struct EcsService_VolumeConfiguration_ManagedEbsVolume {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_in_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_initialization_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_specifications: std
    ::option
    ::Option<std::vec::Vec<EcsService_VolumeConfiguration_ManagedEbsVolume_TagSpecifications>>,
    dynamic: EcsService_VolumeConfiguration_ManagedEbsVolumeDynamic,
}

impl EcsService_VolumeConfiguration_ManagedEbsVolume {
    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.encrypted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_system_type`.\n"]
    pub fn set_file_system_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_system_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size_in_gb`.\n"]
    pub fn set_size_in_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_in_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_id`.\n"]
    pub fn set_snapshot_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.snapshot_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `throughput`.\n"]
    pub fn set_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_initialization_rate`.\n"]
    pub fn set_volume_initialization_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_initialization_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_specifications`.\n"]
    pub fn set_tag_specifications(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<EcsService_VolumeConfiguration_ManagedEbsVolume_TagSpecifications>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag_specifications = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_specifications = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsService_VolumeConfiguration_ManagedEbsVolume {
    type O = terrars::BlockAssignable<EcsService_VolumeConfiguration_ManagedEbsVolume>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_VolumeConfiguration_ManagedEbsVolume {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildEcsService_VolumeConfiguration_ManagedEbsVolume {
    pub fn build(self) -> EcsService_VolumeConfiguration_ManagedEbsVolume {
        EcsService_VolumeConfiguration_ManagedEbsVolume {
            encrypted: core::default::Default::default(),
            file_system_type: core::default::Default::default(),
            iops: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            role_arn: self.role_arn,
            size_in_gb: core::default::Default::default(),
            snapshot_id: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_initialization_rate: core::default::Default::default(),
            volume_type: core::default::Default::default(),
            tag_specifications: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_VolumeConfiguration_ManagedEbsVolumeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_VolumeConfiguration_ManagedEbsVolumeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsService_VolumeConfiguration_ManagedEbsVolumeRef {
        EcsService_VolumeConfiguration_ManagedEbsVolumeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_VolumeConfiguration_ManagedEbsVolumeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_type` after provisioning.\n"]
    pub fn file_system_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_type", self.base))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `size_in_gb` after provisioning.\n"]
    pub fn size_in_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_in_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_id` after provisioning.\n"]
    pub fn snapshot_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_id", self.base))
    }

    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_initialization_rate` after provisioning.\n"]
    pub fn volume_initialization_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_initialization_rate", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_specifications` after provisioning.\n"]
    pub fn tag_specifications(
        &self,
    ) -> terrars::ListRef<EcsService_VolumeConfiguration_ManagedEbsVolume_TagSpecificationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_specifications", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsService_VolumeConfigurationDynamic {
    managed_ebs_volume: std::option::Option<terrars::DynamicBlock<EcsService_VolumeConfiguration_ManagedEbsVolume>>,
}

#[derive(Serialize)]
pub struct EcsService_VolumeConfiguration {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_ebs_volume: std::option::Option<std::vec::Vec<EcsService_VolumeConfiguration_ManagedEbsVolume>>,
    dynamic: EcsService_VolumeConfigurationDynamic,
}

impl EcsService_VolumeConfiguration {
    #[doc = "Set the field `managed_ebs_volume`.\n"]
    pub fn set_managed_ebs_volume(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsService_VolumeConfiguration_ManagedEbsVolume>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_ebs_volume = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_ebs_volume = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsService_VolumeConfiguration {
    type O = terrars::BlockAssignable<EcsService_VolumeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_VolumeConfiguration {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildEcsService_VolumeConfiguration {
    pub fn build(self) -> EcsService_VolumeConfiguration {
        EcsService_VolumeConfiguration {
            name: self.name,
            managed_ebs_volume: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_VolumeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_VolumeConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsService_VolumeConfigurationRef {
        EcsService_VolumeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_VolumeConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_ebs_volume` after provisioning.\n"]
    pub fn managed_ebs_volume(&self) -> terrars::ListRef<EcsService_VolumeConfiguration_ManagedEbsVolumeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_ebs_volume", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsService_VpcLatticeConfigurations {
    port_name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    target_group_arn: terrars::PrimField<std::string::String>,
}

impl EcsService_VpcLatticeConfigurations { }

impl terrars::ToListMappable for EcsService_VpcLatticeConfigurations {
    type O = terrars::BlockAssignable<EcsService_VpcLatticeConfigurations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsService_VpcLatticeConfigurations {
    #[doc = ""]
    pub port_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_group_arn: terrars::PrimField<std::string::String>,
}

impl BuildEcsService_VpcLatticeConfigurations {
    pub fn build(self) -> EcsService_VpcLatticeConfigurations {
        EcsService_VpcLatticeConfigurations {
            port_name: self.port_name,
            role_arn: self.role_arn,
            target_group_arn: self.target_group_arn,
        }
    }
}

#[allow(dead_code)]
pub struct EcsService_VpcLatticeConfigurationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsService_VpcLatticeConfigurationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsService_VpcLatticeConfigurationsRef {
        EcsService_VpcLatticeConfigurationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsService_VpcLatticeConfigurationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port_name` after provisioning.\n"]
    pub fn port_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `target_group_arn` after provisioning.\n"]
    pub fn target_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_group_arn", self.base))
    }
}
