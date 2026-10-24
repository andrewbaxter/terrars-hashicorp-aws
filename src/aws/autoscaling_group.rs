use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AutoscalingGroupData {
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
    availability_zones: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_rebalance: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    context: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_cooldown: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_instance_warmup: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_capacity_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled_metrics: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_delete: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_delete_warm_pool: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check_grace_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_configuration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_instance_lifetime: std::option::Option<terrars::PrimField<f64>>,
    max_size: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metrics_granularity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_elb_capacity: std::option::Option<terrars::PrimField<f64>>,
    min_size: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protect_from_scale_in: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_linked_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suspended_processes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::RecField<terrars::PrimField<std::string::String>>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    termination_policies: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_zone_identifier: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for_capacity_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for_elb_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_lifecycle_hook: std::option::Option<std::vec::Vec<AutoscalingGroupInitialLifecycleHookEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_refresh: std::option::Option<std::vec::Vec<AutoscalingGroupInstanceRefreshEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template: std::option::Option<std::vec::Vec<AutoscalingGroupLaunchTemplateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mixed_instances_policy: std::option::Option<std::vec::Vec<AutoscalingGroupMixedInstancesPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<std::vec::Vec<AutoscalingGroupTagEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AutoscalingGroupTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warm_pool: std::option::Option<std::vec::Vec<AutoscalingGroupWarmPoolEl>>,
    dynamic: AutoscalingGroupDynamic,
}

struct AutoscalingGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AutoscalingGroupData>,
}

#[derive(Clone)]
pub struct AutoscalingGroup(std::rc::Rc<AutoscalingGroup_>);

impl AutoscalingGroup {
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

    #[doc = "Set the field `availability_zones`.\n"]
    pub fn set_availability_zones(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().availability_zones = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_rebalance`.\n"]
    pub fn set_capacity_rebalance(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().capacity_rebalance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `context`.\n"]
    pub fn set_context(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_cooldown`.\n"]
    pub fn set_default_cooldown(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().default_cooldown = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_instance_warmup`.\n"]
    pub fn set_default_instance_warmup(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().default_instance_warmup = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `desired_capacity`.\n"]
    pub fn set_desired_capacity(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().desired_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `desired_capacity_type`.\n"]
    pub fn set_desired_capacity_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().desired_capacity_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled_metrics`.\n"]
    pub fn set_enabled_metrics(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().enabled_metrics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `force_delete`.\n"]
    pub fn set_force_delete(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `force_delete_warm_pool`.\n"]
    pub fn set_force_delete_warm_pool(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_delete_warm_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check_grace_period`.\n"]
    pub fn set_health_check_grace_period(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().health_check_grace_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check_type`.\n"]
    pub fn set_health_check_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().health_check_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_configuration`.\n"]
    pub fn set_launch_configuration(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().launch_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancers`.\n"]
    pub fn set_load_balancers(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().load_balancers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_instance_lifetime`.\n"]
    pub fn set_max_instance_lifetime(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_instance_lifetime = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metrics_granularity`.\n"]
    pub fn set_metrics_granularity(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().metrics_granularity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_elb_capacity`.\n"]
    pub fn set_min_elb_capacity(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_elb_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `placement_group`.\n"]
    pub fn set_placement_group(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().placement_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protect_from_scale_in`.\n"]
    pub fn set_protect_from_scale_in(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().protect_from_scale_in = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_linked_role_arn`.\n"]
    pub fn set_service_linked_role_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().service_linked_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `suspended_processes`.\n"]
    pub fn set_suspended_processes(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().suspended_processes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::RecField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_group_arns`.\n"]
    pub fn set_target_group_arns(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().target_group_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `termination_policies`.\n"]
    pub fn set_termination_policies(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().termination_policies = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_zone_identifier`.\n"]
    pub fn set_vpc_zone_identifier(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().vpc_zone_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wait_for_capacity_timeout`.\n"]
    pub fn set_wait_for_capacity_timeout(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().wait_for_capacity_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wait_for_elb_capacity`.\n"]
    pub fn set_wait_for_elb_capacity(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().wait_for_elb_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initial_lifecycle_hook`.\n"]
    pub fn set_initial_lifecycle_hook(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroupInitialLifecycleHookEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().initial_lifecycle_hook = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.initial_lifecycle_hook = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_refresh`.\n"]
    pub fn set_instance_refresh(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroupInstanceRefreshEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_refresh = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_refresh = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `launch_template`.\n"]
    pub fn set_launch_template(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroupLaunchTemplateEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().launch_template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.launch_template = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mixed_instances_policy`.\n"]
    pub fn set_mixed_instances_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroupMixedInstancesPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mixed_instances_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mixed_instances_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(self, v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroupTagEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tag = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tag = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AutoscalingGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `warm_pool`.\n"]
    pub fn set_warm_pool(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroupWarmPoolEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().warm_pool = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.warm_pool = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_rebalance` after provisioning.\n"]
    pub fn capacity_rebalance(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_rebalance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `context` after provisioning.\n"]
    pub fn context(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_cooldown` after provisioning.\n"]
    pub fn default_cooldown(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_cooldown", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_instance_warmup` after provisioning.\n"]
    pub fn default_instance_warmup(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_instance_warmup", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `desired_capacity` after provisioning.\n"]
    pub fn desired_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `desired_capacity_type` after provisioning.\n"]
    pub fn desired_capacity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_capacity_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled_metrics` after provisioning.\n"]
    pub fn enabled_metrics(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.enabled_metrics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\n"]
    pub fn force_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_delete", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_delete_warm_pool` after provisioning.\n"]
    pub fn force_delete_warm_pool(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_delete_warm_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_grace_period` after provisioning.\n"]
    pub fn health_check_grace_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_grace_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_type` after provisioning.\n"]
    pub fn health_check_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_configuration` after provisioning.\n"]
    pub fn launch_configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancers` after provisioning.\n"]
    pub fn load_balancers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.load_balancers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_instance_lifetime` after provisioning.\n"]
    pub fn max_instance_lifetime(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instance_lifetime", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_size` after provisioning.\n"]
    pub fn max_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metrics_granularity` after provisioning.\n"]
    pub fn metrics_granularity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metrics_granularity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_elb_capacity` after provisioning.\n"]
    pub fn min_elb_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_elb_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_size` after provisioning.\n"]
    pub fn min_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_group` after provisioning.\n"]
    pub fn placement_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protect_from_scale_in` after provisioning.\n"]
    pub fn protect_from_scale_in(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protect_from_scale_in", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_linked_role_arn` after provisioning.\n"]
    pub fn service_linked_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_linked_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `suspended_processes` after provisioning.\n"]
    pub fn suspended_processes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.suspended_processes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_group_arns` after provisioning.\n"]
    pub fn target_group_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_group_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `termination_policies` after provisioning.\n"]
    pub fn termination_policies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.termination_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_zone_identifier` after provisioning.\n"]
    pub fn vpc_zone_identifier(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_zone_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_capacity_timeout` after provisioning.\n"]
    pub fn wait_for_capacity_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_capacity_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_elb_capacity` after provisioning.\n"]
    pub fn wait_for_elb_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_elb_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_refresh` after provisioning.\n"]
    pub fn instance_refresh(&self) -> terrars::ListRef<AutoscalingGroupInstanceRefreshElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_refresh", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> terrars::ListRef<AutoscalingGroupLaunchTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mixed_instances_policy` after provisioning.\n"]
    pub fn mixed_instances_policy(&self) -> terrars::ListRef<AutoscalingGroupMixedInstancesPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mixed_instances_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AutoscalingGroupTimeoutsElRef {
        AutoscalingGroupTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_pool` after provisioning.\n"]
    pub fn warm_pool(&self) -> terrars::ListRef<AutoscalingGroupWarmPoolElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warm_pool", self.extract_ref()))
    }
}

impl terrars::Referable for AutoscalingGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AutoscalingGroup { }

impl terrars::ToListMappable for AutoscalingGroup {
    type O = terrars::ListRef<AutoscalingGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AutoscalingGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_autoscaling_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAutoscalingGroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub max_size: terrars::PrimField<f64>,
    #[doc = ""]
    pub min_size: terrars::PrimField<f64>,
}

impl BuildAutoscalingGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> AutoscalingGroup {
        let out = AutoscalingGroup(std::rc::Rc::new(AutoscalingGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AutoscalingGroupData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                availability_zones: core::default::Default::default(),
                capacity_rebalance: core::default::Default::default(),
                context: core::default::Default::default(),
                default_cooldown: core::default::Default::default(),
                default_instance_warmup: core::default::Default::default(),
                desired_capacity: core::default::Default::default(),
                desired_capacity_type: core::default::Default::default(),
                enabled_metrics: core::default::Default::default(),
                force_delete: core::default::Default::default(),
                force_delete_warm_pool: core::default::Default::default(),
                health_check_grace_period: core::default::Default::default(),
                health_check_type: core::default::Default::default(),
                id: core::default::Default::default(),
                launch_configuration: core::default::Default::default(),
                load_balancers: core::default::Default::default(),
                max_instance_lifetime: core::default::Default::default(),
                max_size: self.max_size,
                metrics_granularity: core::default::Default::default(),
                min_elb_capacity: core::default::Default::default(),
                min_size: self.min_size,
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                placement_group: core::default::Default::default(),
                protect_from_scale_in: core::default::Default::default(),
                service_linked_role_arn: core::default::Default::default(),
                suspended_processes: core::default::Default::default(),
                tags: core::default::Default::default(),
                target_group_arns: core::default::Default::default(),
                termination_policies: core::default::Default::default(),
                vpc_zone_identifier: core::default::Default::default(),
                wait_for_capacity_timeout: core::default::Default::default(),
                wait_for_elb_capacity: core::default::Default::default(),
                initial_lifecycle_hook: core::default::Default::default(),
                instance_refresh: core::default::Default::default(),
                launch_template: core::default::Default::default(),
                mixed_instances_policy: core::default::Default::default(),
                tag: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                warm_pool: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AutoscalingGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AutoscalingGroupRef {
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

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_rebalance` after provisioning.\n"]
    pub fn capacity_rebalance(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_rebalance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `context` after provisioning.\n"]
    pub fn context(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_cooldown` after provisioning.\n"]
    pub fn default_cooldown(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_cooldown", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_instance_warmup` after provisioning.\n"]
    pub fn default_instance_warmup(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_instance_warmup", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `desired_capacity` after provisioning.\n"]
    pub fn desired_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `desired_capacity_type` after provisioning.\n"]
    pub fn desired_capacity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_capacity_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled_metrics` after provisioning.\n"]
    pub fn enabled_metrics(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.enabled_metrics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\n"]
    pub fn force_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_delete", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_delete_warm_pool` after provisioning.\n"]
    pub fn force_delete_warm_pool(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_delete_warm_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_grace_period` after provisioning.\n"]
    pub fn health_check_grace_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_grace_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_type` after provisioning.\n"]
    pub fn health_check_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_configuration` after provisioning.\n"]
    pub fn launch_configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancers` after provisioning.\n"]
    pub fn load_balancers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.load_balancers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_instance_lifetime` after provisioning.\n"]
    pub fn max_instance_lifetime(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instance_lifetime", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_size` after provisioning.\n"]
    pub fn max_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metrics_granularity` after provisioning.\n"]
    pub fn metrics_granularity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metrics_granularity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_elb_capacity` after provisioning.\n"]
    pub fn min_elb_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_elb_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_size` after provisioning.\n"]
    pub fn min_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_group` after provisioning.\n"]
    pub fn placement_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protect_from_scale_in` after provisioning.\n"]
    pub fn protect_from_scale_in(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protect_from_scale_in", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_linked_role_arn` after provisioning.\n"]
    pub fn service_linked_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_linked_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `suspended_processes` after provisioning.\n"]
    pub fn suspended_processes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.suspended_processes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_group_arns` after provisioning.\n"]
    pub fn target_group_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_group_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `termination_policies` after provisioning.\n"]
    pub fn termination_policies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.termination_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_zone_identifier` after provisioning.\n"]
    pub fn vpc_zone_identifier(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_zone_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_capacity_timeout` after provisioning.\n"]
    pub fn wait_for_capacity_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_capacity_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_elb_capacity` after provisioning.\n"]
    pub fn wait_for_elb_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_elb_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_refresh` after provisioning.\n"]
    pub fn instance_refresh(&self) -> terrars::ListRef<AutoscalingGroupInstanceRefreshElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_refresh", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> terrars::ListRef<AutoscalingGroupLaunchTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mixed_instances_policy` after provisioning.\n"]
    pub fn mixed_instances_policy(&self) -> terrars::ListRef<AutoscalingGroupMixedInstancesPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mixed_instances_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AutoscalingGroupTimeoutsElRef {
        AutoscalingGroupTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_pool` after provisioning.\n"]
    pub fn warm_pool(&self) -> terrars::ListRef<AutoscalingGroupWarmPoolElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warm_pool", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupInitialLifecycleHookEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_result: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    heartbeat_timeout: std::option::Option<terrars::PrimField<f64>>,
    lifecycle_transition: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_metadata: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_target_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AutoscalingGroupInitialLifecycleHookEl {
    #[doc = "Set the field `default_result`.\n"]
    pub fn set_default_result(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_result = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `heartbeat_timeout`.\n"]
    pub fn set_heartbeat_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.heartbeat_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `notification_metadata`.\n"]
    pub fn set_notification_metadata(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.notification_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `notification_target_arn`.\n"]
    pub fn set_notification_target_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.notification_target_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AutoscalingGroupInitialLifecycleHookEl {
    type O = terrars::BlockAssignable<AutoscalingGroupInitialLifecycleHookEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupInitialLifecycleHookEl {
    #[doc = ""]
    pub lifecycle_transition: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingGroupInitialLifecycleHookEl {
    pub fn build(self) -> AutoscalingGroupInitialLifecycleHookEl {
        AutoscalingGroupInitialLifecycleHookEl {
            default_result: core::default::Default::default(),
            heartbeat_timeout: core::default::Default::default(),
            lifecycle_transition: self.lifecycle_transition,
            name: self.name,
            notification_metadata: core::default::Default::default(),
            notification_target_arn: core::default::Default::default(),
            role_arn: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupInitialLifecycleHookElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroupInitialLifecycleHookElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AutoscalingGroupInitialLifecycleHookElRef {
        AutoscalingGroupInitialLifecycleHookElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupInitialLifecycleHookElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_result` after provisioning.\n"]
    pub fn default_result(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_result", self.base))
    }

    #[doc = "Get a reference to the value of field `heartbeat_timeout` after provisioning.\n"]
    pub fn heartbeat_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.heartbeat_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_transition` after provisioning.\n"]
    pub fn lifecycle_transition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_transition", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `notification_metadata` after provisioning.\n"]
    pub fn notification_metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notification_metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `notification_target_arn` after provisioning.\n"]
    pub fn notification_target_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notification_target_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupInstanceRefreshElPreferencesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    checkpoint_delay: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    checkpoint_percentages: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_warmup: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_healthy_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_matching: std::option::Option<terrars::PrimField<bool>>,
}

impl AutoscalingGroupInstanceRefreshElPreferencesEl {
    #[doc = "Set the field `checkpoint_delay`.\n"]
    pub fn set_checkpoint_delay(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.checkpoint_delay = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `checkpoint_percentages`.\n"]
    pub fn set_checkpoint_percentages(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.checkpoint_percentages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_warmup`.\n"]
    pub fn set_instance_warmup(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_warmup = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_healthy_percentage`.\n"]
    pub fn set_min_healthy_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_healthy_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `skip_matching`.\n"]
    pub fn set_skip_matching(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.skip_matching = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AutoscalingGroupInstanceRefreshElPreferencesEl {
    type O = terrars::BlockAssignable<AutoscalingGroupInstanceRefreshElPreferencesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupInstanceRefreshElPreferencesEl {}

impl BuildAutoscalingGroupInstanceRefreshElPreferencesEl {
    pub fn build(self) -> AutoscalingGroupInstanceRefreshElPreferencesEl {
        AutoscalingGroupInstanceRefreshElPreferencesEl {
            checkpoint_delay: core::default::Default::default(),
            checkpoint_percentages: core::default::Default::default(),
            instance_warmup: core::default::Default::default(),
            min_healthy_percentage: core::default::Default::default(),
            skip_matching: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupInstanceRefreshElPreferencesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroupInstanceRefreshElPreferencesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroupInstanceRefreshElPreferencesElRef {
        AutoscalingGroupInstanceRefreshElPreferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupInstanceRefreshElPreferencesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `checkpoint_delay` after provisioning.\n"]
    pub fn checkpoint_delay(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.checkpoint_delay", self.base))
    }

    #[doc = "Get a reference to the value of field `checkpoint_percentages` after provisioning.\n"]
    pub fn checkpoint_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.checkpoint_percentages", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_warmup` after provisioning.\n"]
    pub fn instance_warmup(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_warmup", self.base))
    }

    #[doc = "Get a reference to the value of field `min_healthy_percentage` after provisioning.\n"]
    pub fn min_healthy_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_healthy_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `skip_matching` after provisioning.\n"]
    pub fn skip_matching(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_matching", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroupInstanceRefreshElDynamic {
    preferences: std::option::Option<terrars::DynamicBlock<AutoscalingGroupInstanceRefreshElPreferencesEl>>,
}

#[derive(Serialize)]
pub struct AutoscalingGroupInstanceRefreshEl {
    strategy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    triggers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferences: std::option::Option<std::vec::Vec<AutoscalingGroupInstanceRefreshElPreferencesEl>>,
    dynamic: AutoscalingGroupInstanceRefreshElDynamic,
}

impl AutoscalingGroupInstanceRefreshEl {
    #[doc = "Set the field `triggers`.\n"]
    pub fn set_triggers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.triggers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preferences`.\n"]
    pub fn set_preferences(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroupInstanceRefreshElPreferencesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.preferences = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.preferences = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AutoscalingGroupInstanceRefreshEl {
    type O = terrars::BlockAssignable<AutoscalingGroupInstanceRefreshEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupInstanceRefreshEl {
    #[doc = ""]
    pub strategy: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingGroupInstanceRefreshEl {
    pub fn build(self) -> AutoscalingGroupInstanceRefreshEl {
        AutoscalingGroupInstanceRefreshEl {
            strategy: self.strategy,
            triggers: core::default::Default::default(),
            preferences: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupInstanceRefreshElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroupInstanceRefreshElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AutoscalingGroupInstanceRefreshElRef {
        AutoscalingGroupInstanceRefreshElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupInstanceRefreshElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `strategy` after provisioning.\n"]
    pub fn strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `triggers` after provisioning.\n"]
    pub fn triggers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.triggers", self.base))
    }

    #[doc = "Get a reference to the value of field `preferences` after provisioning.\n"]
    pub fn preferences(&self) -> terrars::ListRef<AutoscalingGroupInstanceRefreshElPreferencesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferences", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupLaunchTemplateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AutoscalingGroupLaunchTemplateEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AutoscalingGroupLaunchTemplateEl {
    type O = terrars::BlockAssignable<AutoscalingGroupLaunchTemplateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupLaunchTemplateEl {}

impl BuildAutoscalingGroupLaunchTemplateEl {
    pub fn build(self) -> AutoscalingGroupLaunchTemplateEl {
        AutoscalingGroupLaunchTemplateEl {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupLaunchTemplateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroupLaunchTemplateElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AutoscalingGroupLaunchTemplateElRef {
        AutoscalingGroupLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupLaunchTemplateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand_allocation_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand_base_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand_percentage_above_base_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_allocation_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_instance_pools: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_max_price: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl {
    #[doc = "Set the field `on_demand_allocation_strategy`.\n"]
    pub fn set_on_demand_allocation_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.on_demand_allocation_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_demand_base_capacity`.\n"]
    pub fn set_on_demand_base_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.on_demand_base_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_demand_percentage_above_base_capacity`.\n"]
    pub fn set_on_demand_percentage_above_base_capacity(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.on_demand_percentage_above_base_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spot_allocation_strategy`.\n"]
    pub fn set_spot_allocation_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.spot_allocation_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spot_instance_pools`.\n"]
    pub fn set_spot_instance_pools(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.spot_instance_pools = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spot_max_price`.\n"]
    pub fn set_spot_max_price(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.spot_max_price = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl {
    type O = terrars::BlockAssignable<AutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl {}

impl BuildAutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl {
    pub fn build(self) -> AutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl {
        AutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl {
            on_demand_allocation_strategy: core::default::Default::default(),
            on_demand_base_capacity: core::default::Default::default(),
            on_demand_percentage_above_base_capacity: core::default::Default::default(),
            spot_allocation_strategy: core::default::Default::default(),
            spot_instance_pools: core::default::Default::default(),
            spot_max_price: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElInstancesDistributionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroupMixedInstancesPolicyElInstancesDistributionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroupMixedInstancesPolicyElInstancesDistributionElRef {
        AutoscalingGroupMixedInstancesPolicyElInstancesDistributionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElInstancesDistributionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `on_demand_allocation_strategy` after provisioning.\n"]
    pub fn on_demand_allocation_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_demand_allocation_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `on_demand_base_capacity` after provisioning.\n"]
    pub fn on_demand_base_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_demand_base_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `on_demand_percentage_above_base_capacity` after provisioning.\n"]
    pub fn on_demand_percentage_above_base_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_demand_percentage_above_base_capacity", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `spot_allocation_strategy` after provisioning.\n"]
    pub fn spot_allocation_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_allocation_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_instance_pools` after provisioning.\n"]
    pub fn spot_instance_pools(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_instance_pools", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_max_price` after provisioning.\n"]
    pub fn spot_max_price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_max_price", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl {
    #[doc = "Set the field `launch_template_id`.\n"]
    pub fn set_launch_template_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.launch_template_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_template_name`.\n"]
    pub fn set_launch_template_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.launch_template_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl {
    type O =
        terrars
        ::BlockAssignable<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl {}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl {
    pub fn build(self) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl {
            launch_template_id: core::default::Default::default(),
            launch_template_name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `launch_template_id` after provisioning.\n"]
    pub fn launch_template_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_template_id", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template_name` after provisioning.\n"]
    pub fn launch_template_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_template_name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl {}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl {}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl {}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl {}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl {}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElDynamic {
    accelerator_count: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl,
        >,
    >,
    accelerator_total_memory_mib: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl,
        >,
    >,
    baseline_ebs_bandwidth_mbps: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl,
        >,
    >,
    memory_gib_per_vcpu: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl,
        >,
    >,
    memory_mib: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl,
        >,
    >,
    network_interface_count: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl,
        >,
    >,
    total_local_storage_gb: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl,
        >,
    >,
    vcpu_count: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_manufacturers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bare_metal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    burstable_performance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_manufacturers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_instance_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_generations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_storage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_storage_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand_max_price_percentage_over_lowest_price: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_hibernate_support: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_max_price_percentage_over_lowest_price: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_total_memory_mib: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    baseline_ebs_bandwidth_mbps: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_gib_per_vcpu: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_mib: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface_count: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_local_storage_gb: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vcpu_count: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl>,
    >,
    dynamic: AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElDynamic,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl {
    #[doc = "Set the field `accelerator_manufacturers`.\n"]
    pub fn set_accelerator_manufacturers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.accelerator_manufacturers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_names`.\n"]
    pub fn set_accelerator_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.accelerator_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_types`.\n"]
    pub fn set_accelerator_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.accelerator_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bare_metal`.\n"]
    pub fn set_bare_metal(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bare_metal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `burstable_performance`.\n"]
    pub fn set_burstable_performance(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.burstable_performance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_manufacturers`.\n"]
    pub fn set_cpu_manufacturers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cpu_manufacturers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `excluded_instance_types`.\n"]
    pub fn set_excluded_instance_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_instance_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_generations`.\n"]
    pub fn set_instance_generations(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.instance_generations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_storage`.\n"]
    pub fn set_local_storage(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_storage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_storage_types`.\n"]
    pub fn set_local_storage_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.local_storage_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_demand_max_price_percentage_over_lowest_price`.\n"]
    pub fn set_on_demand_max_price_percentage_over_lowest_price(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.on_demand_max_price_percentage_over_lowest_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_hibernate_support`.\n"]
    pub fn set_require_hibernate_support(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_hibernate_support = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spot_max_price_percentage_over_lowest_price`.\n"]
    pub fn set_spot_max_price_percentage_over_lowest_price(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.spot_max_price_percentage_over_lowest_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_count`.\n"]
    pub fn set_accelerator_count(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.accelerator_count = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerator_count = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `accelerator_total_memory_mib`.\n"]
    pub fn set_accelerator_total_memory_mib(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.accelerator_total_memory_mib = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerator_total_memory_mib = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `baseline_ebs_bandwidth_mbps`.\n"]
    pub fn set_baseline_ebs_bandwidth_mbps(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.baseline_ebs_bandwidth_mbps = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.baseline_ebs_bandwidth_mbps = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `memory_gib_per_vcpu`.\n"]
    pub fn set_memory_gib_per_vcpu(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.memory_gib_per_vcpu = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.memory_gib_per_vcpu = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `memory_mib`.\n"]
    pub fn set_memory_mib(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.memory_mib = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.memory_mib = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_interface_count`.\n"]
    pub fn set_network_interface_count(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_interface_count = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_interface_count = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `total_local_storage_gb`.\n"]
    pub fn set_total_local_storage_gb(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.total_local_storage_gb = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.total_local_storage_gb = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vcpu_count`.\n"]
    pub fn set_vcpu_count(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vcpu_count = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vcpu_count = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl {
    type O =
        terrars
        ::BlockAssignable<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl {}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl {
    pub fn build(self) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl {
            accelerator_manufacturers: core::default::Default::default(),
            accelerator_names: core::default::Default::default(),
            accelerator_types: core::default::Default::default(),
            bare_metal: core::default::Default::default(),
            burstable_performance: core::default::Default::default(),
            cpu_manufacturers: core::default::Default::default(),
            excluded_instance_types: core::default::Default::default(),
            instance_generations: core::default::Default::default(),
            local_storage: core::default::Default::default(),
            local_storage_types: core::default::Default::default(),
            on_demand_max_price_percentage_over_lowest_price: core::default::Default::default(),
            require_hibernate_support: core::default::Default::default(),
            spot_max_price_percentage_over_lowest_price: core::default::Default::default(),
            accelerator_count: core::default::Default::default(),
            accelerator_total_memory_mib: core::default::Default::default(),
            baseline_ebs_bandwidth_mbps: core::default::Default::default(),
            memory_gib_per_vcpu: core::default::Default::default(),
            memory_mib: core::default::Default::default(),
            network_interface_count: core::default::Default::default(),
            total_local_storage_gb: core::default::Default::default(),
            vcpu_count: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerator_manufacturers` after provisioning.\n"]
    pub fn accelerator_manufacturers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.accelerator_manufacturers", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_names` after provisioning.\n"]
    pub fn accelerator_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.accelerator_names", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_types` after provisioning.\n"]
    pub fn accelerator_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.accelerator_types", self.base))
    }

    #[doc = "Get a reference to the value of field `bare_metal` after provisioning.\n"]
    pub fn bare_metal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bare_metal", self.base))
    }

    #[doc = "Get a reference to the value of field `burstable_performance` after provisioning.\n"]
    pub fn burstable_performance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.burstable_performance", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_manufacturers` after provisioning.\n"]
    pub fn cpu_manufacturers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cpu_manufacturers", self.base))
    }

    #[doc = "Get a reference to the value of field `excluded_instance_types` after provisioning.\n"]
    pub fn excluded_instance_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.excluded_instance_types", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_generations` after provisioning.\n"]
    pub fn instance_generations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.instance_generations", self.base))
    }

    #[doc = "Get a reference to the value of field `local_storage` after provisioning.\n"]
    pub fn local_storage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_storage", self.base))
    }

    #[doc = "Get a reference to the value of field `local_storage_types` after provisioning.\n"]
    pub fn local_storage_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.local_storage_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `on_demand_max_price_percentage_over_lowest_price` after provisioning.\n"]
    pub fn on_demand_max_price_percentage_over_lowest_price(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_demand_max_price_percentage_over_lowest_price", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `require_hibernate_support` after provisioning.\n"]
    pub fn require_hibernate_support(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_hibernate_support", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_max_price_percentage_over_lowest_price` after provisioning.\n"]
    pub fn spot_max_price_percentage_over_lowest_price(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.spot_max_price_percentage_over_lowest_price", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\n"]
    pub fn accelerator_count(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorCountElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_total_memory_mib` after provisioning.\n"]
    pub fn accelerator_total_memory_mib(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElAcceleratorTotalMemoryMibElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerator_total_memory_mib", self.base))
    }

    #[doc = "Get a reference to the value of field `baseline_ebs_bandwidth_mbps` after provisioning.\n"]
    pub fn baseline_ebs_bandwidth_mbps(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElBaselineEbsBandwidthMbpsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.baseline_ebs_bandwidth_mbps", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_gib_per_vcpu` after provisioning.\n"]
    pub fn memory_gib_per_vcpu(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryGibPerVcpuElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_gib_per_vcpu", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_mib` after provisioning.\n"]
    pub fn memory_mib(
        &self,
    ) -> terrars
    ::ListRef<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElMemoryMibElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_mib", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_count` after provisioning.\n"]
    pub fn network_interface_count(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElNetworkInterfaceCountElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface_count", self.base))
    }

    #[doc = "Get a reference to the value of field `total_local_storage_gb` after provisioning.\n"]
    pub fn total_local_storage_gb(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElTotalLocalStorageGbElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.total_local_storage_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `vcpu_count` after provisioning.\n"]
    pub fn vcpu_count(
        &self,
    ) -> terrars
    ::ListRef<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElVcpuCountElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vcpu_count", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl {
    #[doc = "Set the field `launch_template_id`.\n"]
    pub fn set_launch_template_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.launch_template_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_template_name`.\n"]
    pub fn set_launch_template_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.launch_template_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl {}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl {
    pub fn build(
        self,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl {
            launch_template_id: core::default::Default::default(),
            launch_template_name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `launch_template_id` after provisioning.\n"]
    pub fn launch_template_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_template_id", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template_name` after provisioning.\n"]
    pub fn launch_template_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_template_name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElDynamic {
    instance_requirements: std
    ::option
    ::Option<
        terrars::DynamicBlock<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl>,
    >,
    launch_template_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl>,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_capacity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_requirements: std
    ::option
    ::Option<std::vec::Vec<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_specification: std
    ::option
    ::Option<
        std::vec::Vec<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl>,
    >,
    dynamic: AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElDynamic,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weighted_capacity`.\n"]
    pub fn set_weighted_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.weighted_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_requirements`.\n"]
    pub fn set_instance_requirements(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_requirements = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_requirements = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `launch_template_specification`.\n"]
    pub fn set_launch_template_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.launch_template_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_template_specification = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl {
    type O = terrars::BlockAssignable<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl {}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl {
    pub fn build(self) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl {
            instance_type: core::default::Default::default(),
            weighted_capacity: core::default::Default::default(),
            instance_requirements: core::default::Default::default(),
            launch_template_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_capacity` after provisioning.\n"]
    pub fn weighted_capacity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weighted_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_requirements` after provisioning.\n"]
    pub fn instance_requirements(
        &self,
    ) -> terrars::ListRef<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElInstanceRequirementsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_requirements", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template_specification` after provisioning.\n"]
    pub fn launch_template_specification(
        &self,
    ) -> terrars
    ::ListRef<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElLaunchTemplateSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template_specification", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElDynamic {
    launch_template_specification: std
    ::option
    ::Option<
        terrars::DynamicBlock<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl>,
    >,
    override_: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl>>,
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_specification: std
    ::option
    ::Option<std::vec::Vec<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl>>,
    #[serde(rename = "override", skip_serializing_if = "std::option::Option::is_none")]
    override_: std::option::Option<std::vec::Vec<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl>>,
    dynamic: AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElDynamic,
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl {
    #[doc = "Set the field `launch_template_specification`.\n"]
    pub fn set_launch_template_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.launch_template_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_template_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.override_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.override_ = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl {
    type O = terrars::BlockAssignable<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl {}

impl BuildAutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl {
    pub fn build(self) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl {
            launch_template_specification: core::default::Default::default(),
            override_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElRef {
        AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `launch_template_specification` after provisioning.\n"]
    pub fn launch_template_specification(
        &self,
    ) -> terrars::ListRef<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElLaunchTemplateSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> terrars::ListRef<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElOverrideElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.override", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroupMixedInstancesPolicyElDynamic {
    instances_distribution: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl>>,
    launch_template: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl>>,
}

#[derive(Serialize)]
pub struct AutoscalingGroupMixedInstancesPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instances_distribution: std
    ::option
    ::Option<std::vec::Vec<AutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template: std::option::Option<std::vec::Vec<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl>>,
    dynamic: AutoscalingGroupMixedInstancesPolicyElDynamic,
}

impl AutoscalingGroupMixedInstancesPolicyEl {
    #[doc = "Set the field `instances_distribution`.\n"]
    pub fn set_instances_distribution(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AutoscalingGroupMixedInstancesPolicyElInstancesDistributionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instances_distribution = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instances_distribution = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `launch_template`.\n"]
    pub fn set_launch_template(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.launch_template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_template = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AutoscalingGroupMixedInstancesPolicyEl {
    type O = terrars::BlockAssignable<AutoscalingGroupMixedInstancesPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupMixedInstancesPolicyEl {}

impl BuildAutoscalingGroupMixedInstancesPolicyEl {
    pub fn build(self) -> AutoscalingGroupMixedInstancesPolicyEl {
        AutoscalingGroupMixedInstancesPolicyEl {
            instances_distribution: core::default::Default::default(),
            launch_template: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupMixedInstancesPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroupMixedInstancesPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AutoscalingGroupMixedInstancesPolicyElRef {
        AutoscalingGroupMixedInstancesPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupMixedInstancesPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instances_distribution` after provisioning.\n"]
    pub fn instances_distribution(
        &self,
    ) -> terrars::ListRef<AutoscalingGroupMixedInstancesPolicyElInstancesDistributionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances_distribution", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> terrars::ListRef<AutoscalingGroupMixedInstancesPolicyElLaunchTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupTagEl {
    key: terrars::PrimField<std::string::String>,
    propagate_at_launch: terrars::PrimField<bool>,
    value: terrars::PrimField<std::string::String>,
}

impl AutoscalingGroupTagEl { }

impl terrars::ToListMappable for AutoscalingGroupTagEl {
    type O = terrars::BlockAssignable<AutoscalingGroupTagEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupTagEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub propagate_at_launch: terrars::PrimField<bool>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingGroupTagEl {
    pub fn build(self) -> AutoscalingGroupTagEl {
        AutoscalingGroupTagEl {
            key: self.key,
            propagate_at_launch: self.propagate_at_launch,
            value: self.value,
        }
    }
}

pub struct AutoscalingGroupTagElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroupTagElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AutoscalingGroupTagElRef {
        AutoscalingGroupTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupTagElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `propagate_at_launch` after provisioning.\n"]
    pub fn propagate_at_launch(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagate_at_launch", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroupTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AutoscalingGroupTimeoutsEl {
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

impl terrars::ToListMappable for AutoscalingGroupTimeoutsEl {
    type O = terrars::BlockAssignable<AutoscalingGroupTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupTimeoutsEl {}

impl BuildAutoscalingGroupTimeoutsEl {
    pub fn build(self) -> AutoscalingGroupTimeoutsEl {
        AutoscalingGroupTimeoutsEl {
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroupTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AutoscalingGroupTimeoutsElRef {
        AutoscalingGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
pub struct AutoscalingGroupWarmPoolElInstanceReusePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reuse_on_scale_in: std::option::Option<terrars::PrimField<bool>>,
}

impl AutoscalingGroupWarmPoolElInstanceReusePolicyEl {
    #[doc = "Set the field `reuse_on_scale_in`.\n"]
    pub fn set_reuse_on_scale_in(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.reuse_on_scale_in = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AutoscalingGroupWarmPoolElInstanceReusePolicyEl {
    type O = terrars::BlockAssignable<AutoscalingGroupWarmPoolElInstanceReusePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupWarmPoolElInstanceReusePolicyEl {}

impl BuildAutoscalingGroupWarmPoolElInstanceReusePolicyEl {
    pub fn build(self) -> AutoscalingGroupWarmPoolElInstanceReusePolicyEl {
        AutoscalingGroupWarmPoolElInstanceReusePolicyEl { reuse_on_scale_in: core::default::Default::default() }
    }
}

pub struct AutoscalingGroupWarmPoolElInstanceReusePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroupWarmPoolElInstanceReusePolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroupWarmPoolElInstanceReusePolicyElRef {
        AutoscalingGroupWarmPoolElInstanceReusePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupWarmPoolElInstanceReusePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `reuse_on_scale_in` after provisioning.\n"]
    pub fn reuse_on_scale_in(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reuse_on_scale_in", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroupWarmPoolElDynamic {
    instance_reuse_policy: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingGroupWarmPoolElInstanceReusePolicyEl>>,
}

#[derive(Serialize)]
pub struct AutoscalingGroupWarmPoolEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_group_prepared_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_reuse_policy: std::option::Option<std::vec::Vec<AutoscalingGroupWarmPoolElInstanceReusePolicyEl>>,
    dynamic: AutoscalingGroupWarmPoolElDynamic,
}

impl AutoscalingGroupWarmPoolEl {
    #[doc = "Set the field `max_group_prepared_capacity`.\n"]
    pub fn set_max_group_prepared_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_group_prepared_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_size`.\n"]
    pub fn set_min_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pool_state`.\n"]
    pub fn set_pool_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pool_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_reuse_policy`.\n"]
    pub fn set_instance_reuse_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroupWarmPoolElInstanceReusePolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_reuse_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_reuse_policy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AutoscalingGroupWarmPoolEl {
    type O = terrars::BlockAssignable<AutoscalingGroupWarmPoolEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroupWarmPoolEl {}

impl BuildAutoscalingGroupWarmPoolEl {
    pub fn build(self) -> AutoscalingGroupWarmPoolEl {
        AutoscalingGroupWarmPoolEl {
            max_group_prepared_capacity: core::default::Default::default(),
            min_size: core::default::Default::default(),
            pool_state: core::default::Default::default(),
            instance_reuse_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingGroupWarmPoolElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroupWarmPoolElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AutoscalingGroupWarmPoolElRef {
        AutoscalingGroupWarmPoolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroupWarmPoolElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_group_prepared_capacity` after provisioning.\n"]
    pub fn max_group_prepared_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_group_prepared_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `min_size` after provisioning.\n"]
    pub fn min_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_size", self.base))
    }

    #[doc = "Get a reference to the value of field `pool_state` after provisioning.\n"]
    pub fn pool_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_state", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_reuse_policy` after provisioning.\n"]
    pub fn instance_reuse_policy(&self) -> terrars::ListRef<AutoscalingGroupWarmPoolElInstanceReusePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_reuse_policy", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroupDynamic {
    initial_lifecycle_hook: std::option::Option<terrars::DynamicBlock<AutoscalingGroupInitialLifecycleHookEl>>,
    instance_refresh: std::option::Option<terrars::DynamicBlock<AutoscalingGroupInstanceRefreshEl>>,
    launch_template: std::option::Option<terrars::DynamicBlock<AutoscalingGroupLaunchTemplateEl>>,
    mixed_instances_policy: std::option::Option<terrars::DynamicBlock<AutoscalingGroupMixedInstancesPolicyEl>>,
    tag: std::option::Option<terrars::DynamicBlock<AutoscalingGroupTagEl>>,
    warm_pool: std::option::Option<terrars::DynamicBlock<AutoscalingGroupWarmPoolEl>>,
}
