use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcsServiceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_maximum_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_minimum_healthy_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_ecs_managed_tags: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_execute_command: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_new_deployment: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_grace_period_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_type: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    propagate_tags: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduling_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_definition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    triggers: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_steady_state: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alarms: Option<Vec<EcsServiceAlarmsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_provider_strategy: Option<Vec<EcsServiceCapacityProviderStrategyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_circuit_breaker: Option<Vec<EcsServiceDeploymentCircuitBreakerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_controller: Option<Vec<EcsServiceDeploymentControllerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer: Option<Vec<EcsServiceLoadBalancerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_configuration: Option<Vec<EcsServiceNetworkConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ordered_placement_strategy: Option<Vec<EcsServiceOrderedPlacementStrategyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_constraints: Option<Vec<EcsServicePlacementConstraintsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_connect_configuration: Option<Vec<EcsServiceServiceConnectConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_registries: Option<Vec<EcsServiceServiceRegistriesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<EcsServiceTimeoutsEl>,
    dynamic: EcsServiceDynamic,
}

struct EcsService_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EcsServiceData>,
}

#[derive(Clone)]
pub struct EcsService(Rc<EcsService_>);

impl EcsService {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `cluster`.\n"]
    pub fn set_cluster(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cluster = Some(v.into());
        self
    }

    #[doc= "Set the field `deployment_maximum_percent`.\n"]
    pub fn set_deployment_maximum_percent(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().deployment_maximum_percent = Some(v.into());
        self
    }

    #[doc= "Set the field `deployment_minimum_healthy_percent`.\n"]
    pub fn set_deployment_minimum_healthy_percent(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().deployment_minimum_healthy_percent = Some(v.into());
        self
    }

    #[doc= "Set the field `desired_count`.\n"]
    pub fn set_desired_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().desired_count = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_ecs_managed_tags`.\n"]
    pub fn set_enable_ecs_managed_tags(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_ecs_managed_tags = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_execute_command`.\n"]
    pub fn set_enable_execute_command(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_execute_command = Some(v.into());
        self
    }

    #[doc= "Set the field `force_new_deployment`.\n"]
    pub fn set_force_new_deployment(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_new_deployment = Some(v.into());
        self
    }

    #[doc= "Set the field `health_check_grace_period_seconds`.\n"]
    pub fn set_health_check_grace_period_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().health_check_grace_period_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `iam_role`.\n"]
    pub fn set_iam_role(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().iam_role = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `launch_type`.\n"]
    pub fn set_launch_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().launch_type = Some(v.into());
        self
    }

    #[doc= "Set the field `platform_version`.\n"]
    pub fn set_platform_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().platform_version = Some(v.into());
        self
    }

    #[doc= "Set the field `propagate_tags`.\n"]
    pub fn set_propagate_tags(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().propagate_tags = Some(v.into());
        self
    }

    #[doc= "Set the field `scheduling_strategy`.\n"]
    pub fn set_scheduling_strategy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().scheduling_strategy = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `task_definition`.\n"]
    pub fn set_task_definition(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().task_definition = Some(v.into());
        self
    }

    #[doc= "Set the field `triggers`.\n"]
    pub fn set_triggers(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().triggers = Some(v.into());
        self
    }

    #[doc= "Set the field `wait_for_steady_state`.\n"]
    pub fn set_wait_for_steady_state(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_steady_state = Some(v.into());
        self
    }

    #[doc= "Set the field `alarms`.\n"]
    pub fn set_alarms(self, v: impl Into<BlockAssignable<EcsServiceAlarmsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().alarms = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.alarms = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `capacity_provider_strategy`.\n"]
    pub fn set_capacity_provider_strategy(
        self,
        v: impl Into<BlockAssignable<EcsServiceCapacityProviderStrategyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity_provider_strategy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capacity_provider_strategy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `deployment_circuit_breaker`.\n"]
    pub fn set_deployment_circuit_breaker(
        self,
        v: impl Into<BlockAssignable<EcsServiceDeploymentCircuitBreakerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deployment_circuit_breaker = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deployment_circuit_breaker = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `deployment_controller`.\n"]
    pub fn set_deployment_controller(self, v: impl Into<BlockAssignable<EcsServiceDeploymentControllerEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deployment_controller = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deployment_controller = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `load_balancer`.\n"]
    pub fn set_load_balancer(self, v: impl Into<BlockAssignable<EcsServiceLoadBalancerEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().load_balancer = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.load_balancer = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(self, v: impl Into<BlockAssignable<EcsServiceNetworkConfigurationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `ordered_placement_strategy`.\n"]
    pub fn set_ordered_placement_strategy(
        self,
        v: impl Into<BlockAssignable<EcsServiceOrderedPlacementStrategyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ordered_placement_strategy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ordered_placement_strategy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `placement_constraints`.\n"]
    pub fn set_placement_constraints(self, v: impl Into<BlockAssignable<EcsServicePlacementConstraintsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().placement_constraints = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.placement_constraints = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `service_connect_configuration`.\n"]
    pub fn set_service_connect_configuration(
        self,
        v: impl Into<BlockAssignable<EcsServiceServiceConnectConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_connect_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_connect_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `service_registries`.\n"]
    pub fn set_service_registries(self, v: impl Into<BlockAssignable<EcsServiceServiceRegistriesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_registries = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_registries = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<EcsServiceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployment_maximum_percent` after provisioning.\n"]
    pub fn deployment_maximum_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_maximum_percent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployment_minimum_healthy_percent` after provisioning.\n"]
    pub fn deployment_minimum_healthy_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_minimum_healthy_percent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `desired_count` after provisioning.\n"]
    pub fn desired_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.desired_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_ecs_managed_tags` after provisioning.\n"]
    pub fn enable_ecs_managed_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ecs_managed_tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_execute_command` after provisioning.\n"]
    pub fn enable_execute_command(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_execute_command", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `force_new_deployment` after provisioning.\n"]
    pub fn force_new_deployment(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_new_deployment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `health_check_grace_period_seconds` after provisioning.\n"]
    pub fn health_check_grace_period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check_grace_period_seconds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `iam_role` after provisioning.\n"]
    pub fn iam_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_role", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `launch_type` after provisioning.\n"]
    pub fn launch_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.launch_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scheduling_strategy` after provisioning.\n"]
    pub fn scheduling_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scheduling_strategy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_definition", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `triggers` after provisioning.\n"]
    pub fn triggers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.triggers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wait_for_steady_state` after provisioning.\n"]
    pub fn wait_for_steady_state(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait_for_steady_state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `alarms` after provisioning.\n"]
    pub fn alarms(&self) -> ListRef<EcsServiceAlarmsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alarms", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployment_circuit_breaker` after provisioning.\n"]
    pub fn deployment_circuit_breaker(&self) -> ListRef<EcsServiceDeploymentCircuitBreakerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deployment_circuit_breaker", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployment_controller` after provisioning.\n"]
    pub fn deployment_controller(&self) -> ListRef<EcsServiceDeploymentControllerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deployment_controller", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> ListRef<EcsServiceNetworkConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ordered_placement_strategy` after provisioning.\n"]
    pub fn ordered_placement_strategy(&self) -> ListRef<EcsServiceOrderedPlacementStrategyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ordered_placement_strategy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_connect_configuration` after provisioning.\n"]
    pub fn service_connect_configuration(&self) -> ListRef<EcsServiceServiceConnectConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service_connect_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_registries` after provisioning.\n"]
    pub fn service_registries(&self) -> ListRef<EcsServiceServiceRegistriesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service_registries", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EcsServiceTimeoutsElRef {
        EcsServiceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Resource for EcsService {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for EcsService {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for EcsService {
    type O = ListRef<EcsServiceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(&self))
    }
}

impl Resource_ for EcsService_ {
    fn extract_resource_type(&self) -> String {
        "aws_ecs_service".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEcsService {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildEcsService {
    pub fn build(self, stack: &mut Stack) -> EcsService {
        let out = EcsService(Rc::new(EcsService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EcsServiceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EcsServiceRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EcsServiceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployment_maximum_percent` after provisioning.\n"]
    pub fn deployment_maximum_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_maximum_percent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployment_minimum_healthy_percent` after provisioning.\n"]
    pub fn deployment_minimum_healthy_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_minimum_healthy_percent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `desired_count` after provisioning.\n"]
    pub fn desired_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.desired_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_ecs_managed_tags` after provisioning.\n"]
    pub fn enable_ecs_managed_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ecs_managed_tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_execute_command` after provisioning.\n"]
    pub fn enable_execute_command(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_execute_command", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `force_new_deployment` after provisioning.\n"]
    pub fn force_new_deployment(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_new_deployment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `health_check_grace_period_seconds` after provisioning.\n"]
    pub fn health_check_grace_period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check_grace_period_seconds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `iam_role` after provisioning.\n"]
    pub fn iam_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_role", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `launch_type` after provisioning.\n"]
    pub fn launch_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.launch_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scheduling_strategy` after provisioning.\n"]
    pub fn scheduling_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scheduling_strategy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_definition", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `triggers` after provisioning.\n"]
    pub fn triggers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.triggers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wait_for_steady_state` after provisioning.\n"]
    pub fn wait_for_steady_state(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait_for_steady_state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `alarms` after provisioning.\n"]
    pub fn alarms(&self) -> ListRef<EcsServiceAlarmsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alarms", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployment_circuit_breaker` after provisioning.\n"]
    pub fn deployment_circuit_breaker(&self) -> ListRef<EcsServiceDeploymentCircuitBreakerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deployment_circuit_breaker", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployment_controller` after provisioning.\n"]
    pub fn deployment_controller(&self) -> ListRef<EcsServiceDeploymentControllerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deployment_controller", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> ListRef<EcsServiceNetworkConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ordered_placement_strategy` after provisioning.\n"]
    pub fn ordered_placement_strategy(&self) -> ListRef<EcsServiceOrderedPlacementStrategyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ordered_placement_strategy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_connect_configuration` after provisioning.\n"]
    pub fn service_connect_configuration(&self) -> ListRef<EcsServiceServiceConnectConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service_connect_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_registries` after provisioning.\n"]
    pub fn service_registries(&self) -> ListRef<EcsServiceServiceRegistriesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service_registries", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EcsServiceTimeoutsElRef {
        EcsServiceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EcsServiceAlarmsEl {
    alarm_names: SetField<PrimField<String>>,
    enable: PrimField<bool>,
    rollback: PrimField<bool>,
}

impl EcsServiceAlarmsEl { }

impl ToListMappable for EcsServiceAlarmsEl {
    type O = BlockAssignable<EcsServiceAlarmsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceAlarmsEl {
    #[doc= ""]
    pub alarm_names: SetField<PrimField<String>>,
    #[doc= ""]
    pub enable: PrimField<bool>,
    #[doc= ""]
    pub rollback: PrimField<bool>,
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
    shared: StackShared,
    base: String,
}

impl Ref for EcsServiceAlarmsElRef {
    fn new(shared: StackShared, base: String) -> EcsServiceAlarmsElRef {
        EcsServiceAlarmsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceAlarmsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `alarm_names` after provisioning.\n"]
    pub fn alarm_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.alarm_names", self.base))
    }

    #[doc= "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc= "Get a reference to the value of field `rollback` after provisioning.\n"]
    pub fn rollback(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.rollback", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsServiceCapacityProviderStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<PrimField<f64>>,
    capacity_provider: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}

impl EcsServiceCapacityProviderStrategyEl {
    #[doc= "Set the field `base`.\n"]
    pub fn set_base(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.base = Some(v.into());
        self
    }

    #[doc= "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}

impl ToListMappable for EcsServiceCapacityProviderStrategyEl {
    type O = BlockAssignable<EcsServiceCapacityProviderStrategyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceCapacityProviderStrategyEl {
    #[doc= ""]
    pub capacity_provider: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for EcsServiceCapacityProviderStrategyElRef {
    fn new(shared: StackShared, base: String) -> EcsServiceCapacityProviderStrategyElRef {
        EcsServiceCapacityProviderStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceCapacityProviderStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `base` after provisioning.\n"]
    pub fn base(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.base", self.base))
    }

    #[doc= "Get a reference to the value of field `capacity_provider` after provisioning.\n"]
    pub fn capacity_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity_provider", self.base))
    }

    #[doc= "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsServiceDeploymentCircuitBreakerEl {
    enable: PrimField<bool>,
    rollback: PrimField<bool>,
}

impl EcsServiceDeploymentCircuitBreakerEl { }

impl ToListMappable for EcsServiceDeploymentCircuitBreakerEl {
    type O = BlockAssignable<EcsServiceDeploymentCircuitBreakerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceDeploymentCircuitBreakerEl {
    #[doc= ""]
    pub enable: PrimField<bool>,
    #[doc= ""]
    pub rollback: PrimField<bool>,
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
    shared: StackShared,
    base: String,
}

impl Ref for EcsServiceDeploymentCircuitBreakerElRef {
    fn new(shared: StackShared, base: String) -> EcsServiceDeploymentCircuitBreakerElRef {
        EcsServiceDeploymentCircuitBreakerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceDeploymentCircuitBreakerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc= "Get a reference to the value of field `rollback` after provisioning.\n"]
    pub fn rollback(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.rollback", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsServiceDeploymentControllerEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl EcsServiceDeploymentControllerEl {
    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for EcsServiceDeploymentControllerEl {
    type O = BlockAssignable<EcsServiceDeploymentControllerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for EcsServiceDeploymentControllerElRef {
    fn new(shared: StackShared, base: String) -> EcsServiceDeploymentControllerElRef {
        EcsServiceDeploymentControllerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceDeploymentControllerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsServiceLoadBalancerEl {
    container_name: PrimField<String>,
    container_port: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elb_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group_arn: Option<PrimField<String>>,
}

impl EcsServiceLoadBalancerEl {
    #[doc= "Set the field `elb_name`.\n"]
    pub fn set_elb_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.elb_name = Some(v.into());
        self
    }

    #[doc= "Set the field `target_group_arn`.\n"]
    pub fn set_target_group_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_group_arn = Some(v.into());
        self
    }
}

impl ToListMappable for EcsServiceLoadBalancerEl {
    type O = BlockAssignable<EcsServiceLoadBalancerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceLoadBalancerEl {
    #[doc= ""]
    pub container_name: PrimField<String>,
    #[doc= ""]
    pub container_port: PrimField<f64>,
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
    shared: StackShared,
    base: String,
}

impl Ref for EcsServiceLoadBalancerElRef {
    fn new(shared: StackShared, base: String) -> EcsServiceLoadBalancerElRef {
        EcsServiceLoadBalancerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceLoadBalancerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `container_name` after provisioning.\n"]
    pub fn container_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_name", self.base))
    }

    #[doc= "Get a reference to the value of field `container_port` after provisioning.\n"]
    pub fn container_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_port", self.base))
    }

    #[doc= "Get a reference to the value of field `elb_name` after provisioning.\n"]
    pub fn elb_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.elb_name", self.base))
    }

    #[doc= "Get a reference to the value of field `target_group_arn` after provisioning.\n"]
    pub fn target_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_group_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsServiceNetworkConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    assign_public_ip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    subnets: SetField<PrimField<String>>,
}

impl EcsServiceNetworkConfigurationEl {
    #[doc= "Set the field `assign_public_ip`.\n"]
    pub fn set_assign_public_ip(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.assign_public_ip = Some(v.into());
        self
    }

    #[doc= "Set the field `security_groups`.\n"]
    pub fn set_security_groups(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_groups = Some(v.into());
        self
    }
}

impl ToListMappable for EcsServiceNetworkConfigurationEl {
    type O = BlockAssignable<EcsServiceNetworkConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceNetworkConfigurationEl {
    #[doc= ""]
    pub subnets: SetField<PrimField<String>>,
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
    shared: StackShared,
    base: String,
}

impl Ref for EcsServiceNetworkConfigurationElRef {
    fn new(shared: StackShared, base: String) -> EcsServiceNetworkConfigurationElRef {
        EcsServiceNetworkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceNetworkConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `assign_public_ip` after provisioning.\n"]
    pub fn assign_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.assign_public_ip", self.base))
    }

    #[doc= "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc= "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsServiceOrderedPlacementStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl EcsServiceOrderedPlacementStrategyEl {
    #[doc= "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field = Some(v.into());
        self
    }
}

impl ToListMappable for EcsServiceOrderedPlacementStrategyEl {
    type O = BlockAssignable<EcsServiceOrderedPlacementStrategyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceOrderedPlacementStrategyEl {
    #[doc= ""]
    pub type_: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for EcsServiceOrderedPlacementStrategyElRef {
    fn new(shared: StackShared, base: String) -> EcsServiceOrderedPlacementStrategyElRef {
        EcsServiceOrderedPlacementStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceOrderedPlacementStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsServicePlacementConstraintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl EcsServicePlacementConstraintsEl {
    #[doc= "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }
}

impl ToListMappable for EcsServicePlacementConstraintsEl {
    type O = BlockAssignable<EcsServicePlacementConstraintsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServicePlacementConstraintsEl {
    #[doc= ""]
    pub type_: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for EcsServicePlacementConstraintsElRef {
    fn new(shared: StackShared, base: String) -> EcsServicePlacementConstraintsElRef {
        EcsServicePlacementConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServicePlacementConstraintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionEl {
    name: PrimField<String>,
    value_from: PrimField<String>,
}

impl EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionEl { }

impl ToListMappable for EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionEl {
    type O = BlockAssignable<EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionEl {
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub value_from: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionElRef {
        EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `value_from` after provisioning.\n"]
    pub fn value_from(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value_from", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsServiceServiceConnectConfigurationElLogConfigurationElDynamic {
    secret_option: Option<DynamicBlock<EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionEl>>,
}

#[derive(Serialize)]
pub struct EcsServiceServiceConnectConfigurationElLogConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    log_driver: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_option: Option<Vec<EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionEl>>,
    dynamic: EcsServiceServiceConnectConfigurationElLogConfigurationElDynamic,
}

impl EcsServiceServiceConnectConfigurationElLogConfigurationEl {
    #[doc= "Set the field `log_driver`.\n"]
    pub fn set_log_driver(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_driver = Some(v.into());
        self
    }

    #[doc= "Set the field `options`.\n"]
    pub fn set_options(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.options = Some(v.into());
        self
    }

    #[doc= "Set the field `secret_option`.\n"]
    pub fn set_secret_option(
        mut self,
        v: impl Into<BlockAssignable<EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secret_option = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_option = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EcsServiceServiceConnectConfigurationElLogConfigurationEl {
    type O = BlockAssignable<EcsServiceServiceConnectConfigurationElLogConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct EcsServiceServiceConnectConfigurationElLogConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsServiceServiceConnectConfigurationElLogConfigurationElRef {
    fn new(shared: StackShared, base: String) -> EcsServiceServiceConnectConfigurationElLogConfigurationElRef {
        EcsServiceServiceConnectConfigurationElLogConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceServiceConnectConfigurationElLogConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `log_driver` after provisioning.\n"]
    pub fn log_driver(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_driver", self.base))
    }

    #[doc= "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.options", self.base))
    }

    #[doc= "Get a reference to the value of field `secret_option` after provisioning.\n"]
    pub fn secret_option(&self) -> ListRef<EcsServiceServiceConnectConfigurationElLogConfigurationElSecretOptionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.secret_option", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsServiceServiceConnectConfigurationElServiceElClientAliasEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_name: Option<PrimField<String>>,
    port: PrimField<f64>,
}

impl EcsServiceServiceConnectConfigurationElServiceElClientAliasEl {
    #[doc= "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dns_name = Some(v.into());
        self
    }
}

impl ToListMappable for EcsServiceServiceConnectConfigurationElServiceElClientAliasEl {
    type O = BlockAssignable<EcsServiceServiceConnectConfigurationElServiceElClientAliasEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceServiceConnectConfigurationElServiceElClientAliasEl {
    #[doc= ""]
    pub port: PrimField<f64>,
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
    shared: StackShared,
    base: String,
}

impl Ref for EcsServiceServiceConnectConfigurationElServiceElClientAliasElRef {
    fn new(shared: StackShared, base: String) -> EcsServiceServiceConnectConfigurationElServiceElClientAliasElRef {
        EcsServiceServiceConnectConfigurationElServiceElClientAliasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceServiceConnectConfigurationElServiceElClientAliasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsServiceServiceConnectConfigurationElServiceElDynamic {
    client_alias: Option<DynamicBlock<EcsServiceServiceConnectConfigurationElServiceElClientAliasEl>>,
}

#[derive(Serialize)]
pub struct EcsServiceServiceConnectConfigurationElServiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    discovery_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_port_override: Option<PrimField<f64>>,
    port_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_alias: Option<Vec<EcsServiceServiceConnectConfigurationElServiceElClientAliasEl>>,
    dynamic: EcsServiceServiceConnectConfigurationElServiceElDynamic,
}

impl EcsServiceServiceConnectConfigurationElServiceEl {
    #[doc= "Set the field `discovery_name`.\n"]
    pub fn set_discovery_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.discovery_name = Some(v.into());
        self
    }

    #[doc= "Set the field `ingress_port_override`.\n"]
    pub fn set_ingress_port_override(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ingress_port_override = Some(v.into());
        self
    }

    #[doc= "Set the field `client_alias`.\n"]
    pub fn set_client_alias(
        mut self,
        v: impl Into<BlockAssignable<EcsServiceServiceConnectConfigurationElServiceElClientAliasEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.client_alias = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.client_alias = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EcsServiceServiceConnectConfigurationElServiceEl {
    type O = BlockAssignable<EcsServiceServiceConnectConfigurationElServiceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceServiceConnectConfigurationElServiceEl {
    #[doc= ""]
    pub port_name: PrimField<String>,
}

impl BuildEcsServiceServiceConnectConfigurationElServiceEl {
    pub fn build(self) -> EcsServiceServiceConnectConfigurationElServiceEl {
        EcsServiceServiceConnectConfigurationElServiceEl {
            discovery_name: core::default::Default::default(),
            ingress_port_override: core::default::Default::default(),
            port_name: self.port_name,
            client_alias: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EcsServiceServiceConnectConfigurationElServiceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsServiceServiceConnectConfigurationElServiceElRef {
    fn new(shared: StackShared, base: String) -> EcsServiceServiceConnectConfigurationElServiceElRef {
        EcsServiceServiceConnectConfigurationElServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceServiceConnectConfigurationElServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `discovery_name` after provisioning.\n"]
    pub fn discovery_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.discovery_name", self.base))
    }

    #[doc= "Get a reference to the value of field `ingress_port_override` after provisioning.\n"]
    pub fn ingress_port_override(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ingress_port_override", self.base))
    }

    #[doc= "Get a reference to the value of field `port_name` after provisioning.\n"]
    pub fn port_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }

    #[doc= "Get a reference to the value of field `client_alias` after provisioning.\n"]
    pub fn client_alias(&self) -> ListRef<EcsServiceServiceConnectConfigurationElServiceElClientAliasElRef> {
        ListRef::new(self.shared().clone(), format!("{}.client_alias", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsServiceServiceConnectConfigurationElDynamic {
    log_configuration: Option<DynamicBlock<EcsServiceServiceConnectConfigurationElLogConfigurationEl>>,
    service: Option<DynamicBlock<EcsServiceServiceConnectConfigurationElServiceEl>>,
}

#[derive(Serialize)]
pub struct EcsServiceServiceConnectConfigurationEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_configuration: Option<Vec<EcsServiceServiceConnectConfigurationElLogConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<Vec<EcsServiceServiceConnectConfigurationElServiceEl>>,
    dynamic: EcsServiceServiceConnectConfigurationElDynamic,
}

impl EcsServiceServiceConnectConfigurationEl {
    #[doc= "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }

    #[doc= "Set the field `log_configuration`.\n"]
    pub fn set_log_configuration(
        mut self,
        v: impl Into<BlockAssignable<EcsServiceServiceConnectConfigurationElLogConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.log_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.log_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `service`.\n"]
    pub fn set_service(
        mut self,
        v: impl Into<BlockAssignable<EcsServiceServiceConnectConfigurationElServiceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EcsServiceServiceConnectConfigurationEl {
    type O = BlockAssignable<EcsServiceServiceConnectConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceServiceConnectConfigurationEl {
    #[doc= ""]
    pub enabled: PrimField<bool>,
}

impl BuildEcsServiceServiceConnectConfigurationEl {
    pub fn build(self) -> EcsServiceServiceConnectConfigurationEl {
        EcsServiceServiceConnectConfigurationEl {
            enabled: self.enabled,
            namespace: core::default::Default::default(),
            log_configuration: core::default::Default::default(),
            service: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EcsServiceServiceConnectConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsServiceServiceConnectConfigurationElRef {
    fn new(shared: StackShared, base: String) -> EcsServiceServiceConnectConfigurationElRef {
        EcsServiceServiceConnectConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceServiceConnectConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc= "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(&self) -> ListRef<EcsServiceServiceConnectConfigurationElLogConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.log_configuration", self.base))
    }

    #[doc= "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> ListRef<EcsServiceServiceConnectConfigurationElServiceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsServiceServiceRegistriesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    registry_arn: PrimField<String>,
}

impl EcsServiceServiceRegistriesEl {
    #[doc= "Set the field `container_name`.\n"]
    pub fn set_container_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_name = Some(v.into());
        self
    }

    #[doc= "Set the field `container_port`.\n"]
    pub fn set_container_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.container_port = Some(v.into());
        self
    }

    #[doc= "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}

impl ToListMappable for EcsServiceServiceRegistriesEl {
    type O = BlockAssignable<EcsServiceServiceRegistriesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsServiceServiceRegistriesEl {
    #[doc= ""]
    pub registry_arn: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for EcsServiceServiceRegistriesElRef {
    fn new(shared: StackShared, base: String) -> EcsServiceServiceRegistriesElRef {
        EcsServiceServiceRegistriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceServiceRegistriesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `container_name` after provisioning.\n"]
    pub fn container_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_name", self.base))
    }

    #[doc= "Get a reference to the value of field `container_port` after provisioning.\n"]
    pub fn container_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_port", self.base))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc= "Get a reference to the value of field `registry_arn` after provisioning.\n"]
    pub fn registry_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsServiceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl EcsServiceTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for EcsServiceTimeoutsEl {
    type O = BlockAssignable<EcsServiceTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for EcsServiceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> EcsServiceTimeoutsElRef {
        EcsServiceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsServiceTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsServiceDynamic {
    alarms: Option<DynamicBlock<EcsServiceAlarmsEl>>,
    capacity_provider_strategy: Option<DynamicBlock<EcsServiceCapacityProviderStrategyEl>>,
    deployment_circuit_breaker: Option<DynamicBlock<EcsServiceDeploymentCircuitBreakerEl>>,
    deployment_controller: Option<DynamicBlock<EcsServiceDeploymentControllerEl>>,
    load_balancer: Option<DynamicBlock<EcsServiceLoadBalancerEl>>,
    network_configuration: Option<DynamicBlock<EcsServiceNetworkConfigurationEl>>,
    ordered_placement_strategy: Option<DynamicBlock<EcsServiceOrderedPlacementStrategyEl>>,
    placement_constraints: Option<DynamicBlock<EcsServicePlacementConstraintsEl>>,
    service_connect_configuration: Option<DynamicBlock<EcsServiceServiceConnectConfigurationEl>>,
    service_registries: Option<DynamicBlock<EcsServiceServiceRegistriesEl>>,
}
