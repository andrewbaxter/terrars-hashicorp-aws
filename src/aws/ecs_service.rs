use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcsServiceData {
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
    scheduling_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
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
    alarms: std::option::Option<std::vec::Vec<EcsServiceAlarmsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_provider_strategy: std::option::Option<std::vec::Vec<EcsServiceCapacityProviderStrategyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_circuit_breaker: std::option::Option<std::vec::Vec<EcsServiceDeploymentCircuitBreakerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_controller: std::option::Option<std::vec::Vec<EcsServiceDeploymentControllerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer: std::option::Option<std::vec::Vec<EcsServiceLoadBalancerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std::option::Option<std::vec::Vec<EcsServiceNetworkConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ordered_placement_strategy: std::option::Option<std::vec::Vec<EcsServiceOrderedPlacementStrategyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_constraints: std::option::Option<std::vec::Vec<EcsServicePlacementConstraintsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_connect_configuration: std::option::Option<std::vec::Vec<EcsServiceServiceConnectConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_registries: std::option::Option<std::vec::Vec<EcsServiceServiceRegistriesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EcsServiceTimeoutsEl>,
    dynamic: EcsServiceDynamic,
}

struct EcsService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EcsServiceData>,
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

    #[doc = "Set the field `scheduling_strategy`.\n"]
    pub fn set_scheduling_strategy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().scheduling_strategy = std::option::Option::Some(v.into());
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
    pub fn set_alarms(self, v: impl std::convert::Into<terrars::BlockAssignable<EcsServiceAlarmsEl>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<EcsServiceCapacityProviderStrategyEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<EcsServiceDeploymentCircuitBreakerEl>>,
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

    #[doc = "Set the field `deployment_controller`.\n"]
    pub fn set_deployment_controller(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsServiceDeploymentControllerEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<EcsServiceLoadBalancerEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<EcsServiceNetworkConfigurationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<EcsServiceOrderedPlacementStrategyEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<EcsServicePlacementConstraintsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<EcsServiceServiceConnectConfigurationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<EcsServiceServiceRegistriesEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<EcsServiceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
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

    #[doc = "Get a reference to the value of field `scheduling_strategy` after provisioning.\n"]
    pub fn scheduling_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduling_strategy", self.extract_ref()))
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
    pub fn alarms(&self) -> terrars::ListRef<EcsServiceAlarmsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alarms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_circuit_breaker` after provisioning.\n"]
    pub fn deployment_circuit_breaker(&self) -> terrars::ListRef<EcsServiceDeploymentCircuitBreakerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_circuit_breaker", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_controller` after provisioning.\n"]
    pub fn deployment_controller(&self) -> terrars::ListRef<EcsServiceDeploymentControllerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_controller", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<EcsServiceNetworkConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ordered_placement_strategy` after provisioning.\n"]
    pub fn ordered_placement_strategy(&self) -> terrars::ListRef<EcsServiceOrderedPlacementStrategyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ordered_placement_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_connect_configuration` after provisioning.\n"]
    pub fn service_connect_configuration(&self) -> terrars::ListRef<EcsServiceServiceConnectConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_connect_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_registries` after provisioning.\n"]
    pub fn service_registries(&self) -> terrars::ListRef<EcsServiceServiceRegistriesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_registries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EcsServiceTimeoutsElRef {
        EcsServiceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(EcsServiceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cluster: core::default::Default::default(),
                deployment_maximum_percent: core::default::Default::default(),
                deployment_minimum_healthy_percent: core::default::Default::default(),
                desired_count: core::default::Default::default(),
                enable_ecs_managed_tags: core::default::Default::default(),
                enable_execute_command: core::default::Default::default(),
                force_new_deployment: core::default::Default::default(),
                health_check_grace_period_seconds: core::default::Default::default(),
                iam_role: core::default::Default::default(),
                id: core::default::Default::default(),
                launch_type: core::default::Default::default(),
                name: self.name,
                platform_version: core::default::Default::default(),
                propagate_tags: core::default::Default::default(),
                scheduling_strategy: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                task_definition: core::default::Default::default(),
                triggers: core::default::Default::default(),
                wait_for_steady_state: core::default::Default::default(),
                alarms: core::default::Default::default(),
                capacity_provider_strategy: core::default::Default::default(),
                deployment_circuit_breaker: core::default::Default::default(),
                deployment_controller: core::default::Default::default(),
                load_balancer: core::default::Default::default(),
                network_configuration: core::default::Default::default(),
                ordered_placement_strategy: core::default::Default::default(),
                placement_constraints: core::default::Default::default(),
                service_connect_configuration: core::default::Default::default(),
                service_registries: core::default::Default::default(),
                timeouts: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `scheduling_strategy` after provisioning.\n"]
    pub fn scheduling_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduling_strategy", self.extract_ref()))
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
    pub fn alarms(&self) -> terrars::ListRef<EcsServiceAlarmsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alarms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_circuit_breaker` after provisioning.\n"]
    pub fn deployment_circuit_breaker(&self) -> terrars::ListRef<EcsServiceDeploymentCircuitBreakerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_circuit_breaker", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_controller` after provisioning.\n"]
    pub fn deployment_controller(&self) -> terrars::ListRef<EcsServiceDeploymentControllerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_controller", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<EcsServiceNetworkConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ordered_placement_strategy` after provisioning.\n"]
    pub fn ordered_placement_strategy(&self) -> terrars::ListRef<EcsServiceOrderedPlacementStrategyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ordered_placement_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_connect_configuration` after provisioning.\n"]
    pub fn service_connect_configuration(&self) -> terrars::ListRef<EcsServiceServiceConnectConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_connect_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_registries` after provisioning.\n"]
    pub fn service_registries(&self) -> terrars::ListRef<EcsServiceServiceRegistriesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_registries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EcsServiceTimeoutsElRef {
        EcsServiceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EcsServiceAlarmsEl {
    alarm_names: terrars::SetField<terrars::PrimField<std::string::String>>,
    enable: terrars::PrimField<bool>,
    rollback: terrars::PrimField<bool>,
}

impl EcsServiceAlarmsEl { }

impl terrars::ToListMappable for EcsServiceAlarmsEl {
    type O = terrars::BlockAssignable<EcsServiceAlarmsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceAlarmsEl {
    #[doc = ""]
    pub alarm_names: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub enable: terrars::PrimField<bool>,
    #[doc = ""]
    pub rollback: terrars::PrimField<bool>,
}

impl BuildEcsServiceAlarmsEl {
    pub fn build(self) -> EcsServiceAlarmsEl {
        EcsServiceAlarmsEl {
            alarm_names: self.alarm_names,
            enable: self.enable,
            rollback: self.rollback,
        }
    }
}

pub struct EcsServiceAlarmsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsServiceAlarmsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsServiceAlarmsElRef {
        EcsServiceAlarmsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceAlarmsElRef {
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
pub struct EcsServiceCapacityProviderStrategyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base: std::option::Option<terrars::PrimField<f64>>,
    capacity_provider: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl EcsServiceCapacityProviderStrategyEl {
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

impl terrars::ToListMappable for EcsServiceCapacityProviderStrategyEl {
    type O = terrars::BlockAssignable<EcsServiceCapacityProviderStrategyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceCapacityProviderStrategyEl {
    #[doc = ""]
    pub capacity_provider: terrars::PrimField<std::string::String>,
}

impl BuildEcsServiceCapacityProviderStrategyEl {
    pub fn build(self) -> EcsServiceCapacityProviderStrategyEl {
        EcsServiceCapacityProviderStrategyEl {
            base: core::default::Default::default(),
            capacity_provider: self.capacity_provider,
            weight: core::default::Default::default(),
        }
    }
}

pub struct EcsServiceCapacityProviderStrategyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsServiceCapacityProviderStrategyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsServiceCapacityProviderStrategyElRef {
        EcsServiceCapacityProviderStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceCapacityProviderStrategyElRef {
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
pub struct EcsServiceDeploymentCircuitBreakerEl {
    enable: terrars::PrimField<bool>,
    rollback: terrars::PrimField<bool>,
}

impl EcsServiceDeploymentCircuitBreakerEl { }

impl terrars::ToListMappable for EcsServiceDeploymentCircuitBreakerEl {
    type O = terrars::BlockAssignable<EcsServiceDeploymentCircuitBreakerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceDeploymentCircuitBreakerEl {
    #[doc = ""]
    pub enable: terrars::PrimField<bool>,
    #[doc = ""]
    pub rollback: terrars::PrimField<bool>,
}

impl BuildEcsServiceDeploymentCircuitBreakerEl {
    pub fn build(self) -> EcsServiceDeploymentCircuitBreakerEl {
        EcsServiceDeploymentCircuitBreakerEl {
            enable: self.enable,
            rollback: self.rollback,
        }
    }
}

pub struct EcsServiceDeploymentCircuitBreakerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsServiceDeploymentCircuitBreakerElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsServiceDeploymentCircuitBreakerElRef {
        EcsServiceDeploymentCircuitBreakerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceDeploymentCircuitBreakerElRef {
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
pub struct EcsServiceDeploymentControllerEl {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsServiceDeploymentControllerEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsServiceDeploymentControllerEl {
    type O = terrars::BlockAssignable<EcsServiceDeploymentControllerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceDeploymentControllerEl {}

impl BuildEcsServiceDeploymentControllerEl {
    pub fn build(self) -> EcsServiceDeploymentControllerEl {
        EcsServiceDeploymentControllerEl { type_: core::default::Default::default() }
    }
}

pub struct EcsServiceDeploymentControllerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsServiceDeploymentControllerElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsServiceDeploymentControllerElRef {
        EcsServiceDeploymentControllerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceDeploymentControllerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsServiceLoadBalancerEl {
    container_name: terrars::PrimField<std::string::String>,
    container_port: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elb_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsServiceLoadBalancerEl {
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
}

impl terrars::ToListMappable for EcsServiceLoadBalancerEl {
    type O = terrars::BlockAssignable<EcsServiceLoadBalancerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceLoadBalancerEl {
    #[doc = ""]
    pub container_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub container_port: terrars::PrimField<f64>,
}

impl BuildEcsServiceLoadBalancerEl {
    pub fn build(self) -> EcsServiceLoadBalancerEl {
        EcsServiceLoadBalancerEl {
            container_name: self.container_name,
            container_port: self.container_port,
            elb_name: core::default::Default::default(),
            target_group_arn: core::default::Default::default(),
        }
    }
}

pub struct EcsServiceLoadBalancerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsServiceLoadBalancerElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsServiceLoadBalancerElRef {
        EcsServiceLoadBalancerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceLoadBalancerElRef {
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
}

#[derive(Serialize)]
pub struct EcsServiceNetworkConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    assign_public_ip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl EcsServiceNetworkConfigurationEl {
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

impl terrars::ToListMappable for EcsServiceNetworkConfigurationEl {
    type O = terrars::BlockAssignable<EcsServiceNetworkConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceNetworkConfigurationEl {
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildEcsServiceNetworkConfigurationEl {
    pub fn build(self) -> EcsServiceNetworkConfigurationEl {
        EcsServiceNetworkConfigurationEl {
            assign_public_ip: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            subnets: self.subnets,
        }
    }
}

pub struct EcsServiceNetworkConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsServiceNetworkConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsServiceNetworkConfigurationElRef {
        EcsServiceNetworkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceNetworkConfigurationElRef {
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
pub struct EcsServiceOrderedPlacementStrategyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl EcsServiceOrderedPlacementStrategyEl {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsServiceOrderedPlacementStrategyEl {
    type O = terrars::BlockAssignable<EcsServiceOrderedPlacementStrategyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceOrderedPlacementStrategyEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildEcsServiceOrderedPlacementStrategyEl {
    pub fn build(self) -> EcsServiceOrderedPlacementStrategyEl {
        EcsServiceOrderedPlacementStrategyEl {
            field: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct EcsServiceOrderedPlacementStrategyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsServiceOrderedPlacementStrategyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsServiceOrderedPlacementStrategyElRef {
        EcsServiceOrderedPlacementStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceOrderedPlacementStrategyElRef {
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
pub struct EcsServicePlacementConstraintsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl EcsServicePlacementConstraintsEl {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsServicePlacementConstraintsEl {
    type O = terrars::BlockAssignable<EcsServicePlacementConstraintsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServicePlacementConstraintsEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildEcsServicePlacementConstraintsEl {
    pub fn build(self) -> EcsServicePlacementConstraintsEl {
        EcsServicePlacementConstraintsEl {
            expression: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct EcsServicePlacementConstraintsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsServicePlacementConstraintsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsServicePlacementConstraintsElRef {
        EcsServicePlacementConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServicePlacementConstraintsElRef {
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
pub struct EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionEl {
    name: terrars::PrimField<std::string::String>,
    value_from: terrars::PrimField<std::string::String>,
}

impl EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionEl { }

impl terrars::ToListMappable for EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionEl {
    type O = terrars::BlockAssignable<EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value_from: terrars::PrimField<std::string::String>,
}

impl BuildEcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionEl {
    pub fn build(self) -> EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionEl {
        EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionEl {
            name: self.name,
            value_from: self.value_from,
        }
    }
}

pub struct EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionElRef {
        EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionElRef {
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
struct EcsServiceServiceConnectConfigurationElLogConfigurationElDynamic {
    secret_option: std
    ::option
    ::Option<terrars::DynamicBlock<EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionEl>>,
}

#[derive(Serialize)]
pub struct EcsServiceServiceConnectConfigurationElLogConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_driver: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_option: std
    ::option
    ::Option<std::vec::Vec<EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionEl>>,
    dynamic: EcsServiceServiceConnectConfigurationElLogConfigurationElDynamic,
}

impl EcsServiceServiceConnectConfigurationElLogConfigurationEl {
    #[doc = "Set the field `log_driver`.\n"]
    pub fn set_log_driver(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_driver = std::option::Option::Some(v.into());
        self
    }

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
                        terrars
                        ::BlockAssignable<EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionEl>,
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

impl terrars::ToListMappable for EcsServiceServiceConnectConfigurationElLogConfigurationEl {
    type O = terrars::BlockAssignable<EcsServiceServiceConnectConfigurationElLogConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceServiceConnectConfigurationElLogConfigurationEl {}

impl BuildEcsServiceServiceConnectConfigurationElLogConfigurationEl {
    pub fn build(self) -> EcsServiceServiceConnectConfigurationElLogConfigurationEl {
        EcsServiceServiceConnectConfigurationElLogConfigurationEl {
            log_driver: core::default::Default::default(),
            options: core::default::Default::default(),
            secret_option: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EcsServiceServiceConnectConfigurationElLogConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsServiceServiceConnectConfigurationElLogConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsServiceServiceConnectConfigurationElLogConfigurationElRef {
        EcsServiceServiceConnectConfigurationElLogConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceServiceConnectConfigurationElLogConfigurationElRef {
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
    ) -> terrars::ListRef<EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_option", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsServiceServiceConnectConfigurationElServiceElClientAliasEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    port: terrars::PrimField<f64>,
}

impl EcsServiceServiceConnectConfigurationElServiceElClientAliasEl {
    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dns_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsServiceServiceConnectConfigurationElServiceElClientAliasEl {
    type O = terrars::BlockAssignable<EcsServiceServiceConnectConfigurationElServiceElClientAliasEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceServiceConnectConfigurationElServiceElClientAliasEl {
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildEcsServiceServiceConnectConfigurationElServiceElClientAliasEl {
    pub fn build(self) -> EcsServiceServiceConnectConfigurationElServiceElClientAliasEl {
        EcsServiceServiceConnectConfigurationElServiceElClientAliasEl {
            dns_name: core::default::Default::default(),
            port: self.port,
        }
    }
}

pub struct EcsServiceServiceConnectConfigurationElServiceElClientAliasElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsServiceServiceConnectConfigurationElServiceElClientAliasElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsServiceServiceConnectConfigurationElServiceElClientAliasElRef {
        EcsServiceServiceConnectConfigurationElServiceElClientAliasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceServiceConnectConfigurationElServiceElClientAliasElRef {
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
}

#[derive(Serialize, Default)]
struct EcsServiceServiceConnectConfigurationElServiceElDynamic {
    client_alias: std
    ::option
    ::Option<terrars::DynamicBlock<EcsServiceServiceConnectConfigurationElServiceElClientAliasEl>>,
}

#[derive(Serialize)]
pub struct EcsServiceServiceConnectConfigurationElServiceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discovery_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_port_override: std::option::Option<terrars::PrimField<f64>>,
    port_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_alias: std::option::Option<std::vec::Vec<EcsServiceServiceConnectConfigurationElServiceElClientAliasEl>>,
    dynamic: EcsServiceServiceConnectConfigurationElServiceElDynamic,
}

impl EcsServiceServiceConnectConfigurationElServiceEl {
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
                    ::Into<terrars::BlockAssignable<EcsServiceServiceConnectConfigurationElServiceElClientAliasEl>>,
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
}

impl terrars::ToListMappable for EcsServiceServiceConnectConfigurationElServiceEl {
    type O = terrars::BlockAssignable<EcsServiceServiceConnectConfigurationElServiceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceServiceConnectConfigurationElServiceEl {
    #[doc = ""]
    pub port_name: terrars::PrimField<std::string::String>,
}

impl BuildEcsServiceServiceConnectConfigurationElServiceEl {
    pub fn build(self) -> EcsServiceServiceConnectConfigurationElServiceEl {
        EcsServiceServiceConnectConfigurationElServiceEl {
            discovery_name: core::default::Default::default(),
            ingress_port_override: core::default::Default::default(),
            port_name: self.port_name,
            client_alias: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EcsServiceServiceConnectConfigurationElServiceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsServiceServiceConnectConfigurationElServiceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsServiceServiceConnectConfigurationElServiceElRef {
        EcsServiceServiceConnectConfigurationElServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceServiceConnectConfigurationElServiceElRef {
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
    pub fn client_alias(&self) -> terrars::ListRef<EcsServiceServiceConnectConfigurationElServiceElClientAliasElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_alias", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsServiceServiceConnectConfigurationElDynamic {
    log_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsServiceServiceConnectConfigurationElLogConfigurationEl>>,
    service: std::option::Option<terrars::DynamicBlock<EcsServiceServiceConnectConfigurationElServiceEl>>,
}

#[derive(Serialize)]
pub struct EcsServiceServiceConnectConfigurationEl {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_configuration: std::option::Option<std::vec::Vec<EcsServiceServiceConnectConfigurationElLogConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<std::vec::Vec<EcsServiceServiceConnectConfigurationElServiceEl>>,
    dynamic: EcsServiceServiceConnectConfigurationElDynamic,
}

impl EcsServiceServiceConnectConfigurationEl {
    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_configuration`.\n"]
    pub fn set_log_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EcsServiceServiceConnectConfigurationElLogConfigurationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<EcsServiceServiceConnectConfigurationElServiceEl>>,
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

impl terrars::ToListMappable for EcsServiceServiceConnectConfigurationEl {
    type O = terrars::BlockAssignable<EcsServiceServiceConnectConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceServiceConnectConfigurationEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildEcsServiceServiceConnectConfigurationEl {
    pub fn build(self) -> EcsServiceServiceConnectConfigurationEl {
        EcsServiceServiceConnectConfigurationEl {
            enabled: self.enabled,
            namespace: core::default::Default::default(),
            log_configuration: core::default::Default::default(),
            service: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EcsServiceServiceConnectConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsServiceServiceConnectConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsServiceServiceConnectConfigurationElRef {
        EcsServiceServiceConnectConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceServiceConnectConfigurationElRef {
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

    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(&self) -> terrars::ListRef<EcsServiceServiceConnectConfigurationElLogConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::ListRef<EcsServiceServiceConnectConfigurationElServiceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsServiceServiceRegistriesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    registry_arn: terrars::PrimField<std::string::String>,
}

impl EcsServiceServiceRegistriesEl {
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

impl terrars::ToListMappable for EcsServiceServiceRegistriesEl {
    type O = terrars::BlockAssignable<EcsServiceServiceRegistriesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceServiceRegistriesEl {
    #[doc = ""]
    pub registry_arn: terrars::PrimField<std::string::String>,
}

impl BuildEcsServiceServiceRegistriesEl {
    pub fn build(self) -> EcsServiceServiceRegistriesEl {
        EcsServiceServiceRegistriesEl {
            container_name: core::default::Default::default(),
            container_port: core::default::Default::default(),
            port: core::default::Default::default(),
            registry_arn: self.registry_arn,
        }
    }
}

pub struct EcsServiceServiceRegistriesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsServiceServiceRegistriesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsServiceServiceRegistriesElRef {
        EcsServiceServiceRegistriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceServiceRegistriesElRef {
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
pub struct EcsServiceTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsServiceTimeoutsEl {
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

impl terrars::ToListMappable for EcsServiceTimeoutsEl {
    type O = terrars::BlockAssignable<EcsServiceTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceTimeoutsEl {}

impl BuildEcsServiceTimeoutsEl {
    pub fn build(self) -> EcsServiceTimeoutsEl {
        EcsServiceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct EcsServiceTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsServiceTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsServiceTimeoutsElRef {
        EcsServiceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct EcsServiceDynamic {
    alarms: std::option::Option<terrars::DynamicBlock<EcsServiceAlarmsEl>>,
    capacity_provider_strategy: std::option::Option<terrars::DynamicBlock<EcsServiceCapacityProviderStrategyEl>>,
    deployment_circuit_breaker: std::option::Option<terrars::DynamicBlock<EcsServiceDeploymentCircuitBreakerEl>>,
    deployment_controller: std::option::Option<terrars::DynamicBlock<EcsServiceDeploymentControllerEl>>,
    load_balancer: std::option::Option<terrars::DynamicBlock<EcsServiceLoadBalancerEl>>,
    network_configuration: std::option::Option<terrars::DynamicBlock<EcsServiceNetworkConfigurationEl>>,
    ordered_placement_strategy: std::option::Option<terrars::DynamicBlock<EcsServiceOrderedPlacementStrategyEl>>,
    placement_constraints: std::option::Option<terrars::DynamicBlock<EcsServicePlacementConstraintsEl>>,
    service_connect_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsServiceServiceConnectConfigurationEl>>,
    service_registries: std::option::Option<terrars::DynamicBlock<EcsServiceServiceRegistriesEl>>,
}
