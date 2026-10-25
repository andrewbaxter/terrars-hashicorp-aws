#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AutoscalingGroupData_ {
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
    ignore_failed_scaling_activities: std::option::Option<terrars::PrimField<bool>>,
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_linked_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suspended_processes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
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
    availability_zone_distribution: std::option::Option<std::vec::Vec<AutoscalingGroup_AvailabilityZoneDistribution>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_specification: std
    ::option
    ::Option<std::vec::Vec<AutoscalingGroup_CapacityReservationSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_lifecycle_hook: std::option::Option<std::vec::Vec<AutoscalingGroup_InitialLifecycleHook>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_lifecycle_policy: std::option::Option<std::vec::Vec<AutoscalingGroup_InstanceLifecyclePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_maintenance_policy: std::option::Option<std::vec::Vec<AutoscalingGroup_InstanceMaintenancePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_refresh: std::option::Option<std::vec::Vec<AutoscalingGroup_InstanceRefresh>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template: std::option::Option<std::vec::Vec<AutoscalingGroup_LaunchTemplate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mixed_instances_policy: std::option::Option<std::vec::Vec<AutoscalingGroup_MixedInstancesPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<std::vec::Vec<AutoscalingGroup_Tag>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AutoscalingGroup_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic_source: std::option::Option<std::vec::Vec<AutoscalingGroup_TrafficSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warm_pool: std::option::Option<std::vec::Vec<AutoscalingGroup_WarmPool>>,
    dynamic: AutoscalingGroupDynamic,
}

struct AutoscalingGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AutoscalingGroupData_>,
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

    #[doc = "Set the field `ignore_failed_scaling_activities`.\n"]
    pub fn set_ignore_failed_scaling_activities(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ignore_failed_scaling_activities = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `availability_zone_distribution`.\n"]
    pub fn set_availability_zone_distribution(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroup_AvailabilityZoneDistribution>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().availability_zone_distribution = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.availability_zone_distribution = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `capacity_reservation_specification`.\n"]
    pub fn set_capacity_reservation_specification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroup_CapacityReservationSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity_reservation_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capacity_reservation_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `initial_lifecycle_hook`.\n"]
    pub fn set_initial_lifecycle_hook(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroup_InitialLifecycleHook>>,
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

    #[doc = "Set the field `instance_lifecycle_policy`.\n"]
    pub fn set_instance_lifecycle_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroup_InstanceLifecyclePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_lifecycle_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_lifecycle_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_maintenance_policy`.\n"]
    pub fn set_instance_maintenance_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroup_InstanceMaintenancePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_maintenance_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_maintenance_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_refresh`.\n"]
    pub fn set_instance_refresh(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroup_InstanceRefresh>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroup_LaunchTemplate>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroup_MixedInstancesPolicy>>,
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
    pub fn set_tag(self, v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroup_Tag>>) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<AutoscalingGroup_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `traffic_source`.\n"]
    pub fn set_traffic_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroup_TrafficSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().traffic_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.traffic_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `warm_pool`.\n"]
    pub fn set_warm_pool(self, v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroup_WarmPool>>) -> Self {
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

    #[doc = "Get a reference to the value of field `ignore_failed_scaling_activities` after provisioning.\n"]
    pub fn ignore_failed_scaling_activities(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_failed_scaling_activities", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `predicted_capacity` after provisioning.\n"]
    pub fn predicted_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.predicted_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protect_from_scale_in` after provisioning.\n"]
    pub fn protect_from_scale_in(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protect_from_scale_in", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_linked_role_arn` after provisioning.\n"]
    pub fn service_linked_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_linked_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `suspended_processes` after provisioning.\n"]
    pub fn suspended_processes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.suspended_processes", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `warm_pool_size` after provisioning.\n"]
    pub fn warm_pool_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warm_pool_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_distribution` after provisioning.\n"]
    pub fn availability_zone_distribution(&self) -> terrars::ListRef<AutoscalingGroup_AvailabilityZoneDistributionRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.availability_zone_distribution", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_specification` after provisioning.\n"]
    pub fn capacity_reservation_specification(
        &self,
    ) -> terrars::ListRef<AutoscalingGroup_CapacityReservationSpecificationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_lifecycle_policy` after provisioning.\n"]
    pub fn instance_lifecycle_policy(&self) -> terrars::ListRef<AutoscalingGroup_InstanceLifecyclePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_lifecycle_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_maintenance_policy` after provisioning.\n"]
    pub fn instance_maintenance_policy(&self) -> terrars::ListRef<AutoscalingGroup_InstanceMaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_refresh` after provisioning.\n"]
    pub fn instance_refresh(&self) -> terrars::ListRef<AutoscalingGroup_InstanceRefreshRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_refresh", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> terrars::ListRef<AutoscalingGroup_LaunchTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mixed_instances_policy` after provisioning.\n"]
    pub fn mixed_instances_policy(&self) -> terrars::ListRef<AutoscalingGroup_MixedInstancesPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mixed_instances_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AutoscalingGroup_TimeoutsRef {
        AutoscalingGroup_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_pool` after provisioning.\n"]
    pub fn warm_pool(&self) -> terrars::ListRef<AutoscalingGroup_WarmPoolRef> {
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
            data: std::cell::RefCell::new(AutoscalingGroupData_ {
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
                ignore_failed_scaling_activities: core::default::Default::default(),
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
                region: core::default::Default::default(),
                service_linked_role_arn: core::default::Default::default(),
                suspended_processes: core::default::Default::default(),
                target_group_arns: core::default::Default::default(),
                termination_policies: core::default::Default::default(),
                vpc_zone_identifier: core::default::Default::default(),
                wait_for_capacity_timeout: core::default::Default::default(),
                wait_for_elb_capacity: core::default::Default::default(),
                availability_zone_distribution: core::default::Default::default(),
                capacity_reservation_specification: core::default::Default::default(),
                initial_lifecycle_hook: core::default::Default::default(),
                instance_lifecycle_policy: core::default::Default::default(),
                instance_maintenance_policy: core::default::Default::default(),
                instance_refresh: core::default::Default::default(),
                launch_template: core::default::Default::default(),
                mixed_instances_policy: core::default::Default::default(),
                tag: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                traffic_source: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `ignore_failed_scaling_activities` after provisioning.\n"]
    pub fn ignore_failed_scaling_activities(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_failed_scaling_activities", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `predicted_capacity` after provisioning.\n"]
    pub fn predicted_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.predicted_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protect_from_scale_in` after provisioning.\n"]
    pub fn protect_from_scale_in(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protect_from_scale_in", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_linked_role_arn` after provisioning.\n"]
    pub fn service_linked_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_linked_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `suspended_processes` after provisioning.\n"]
    pub fn suspended_processes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.suspended_processes", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `warm_pool_size` after provisioning.\n"]
    pub fn warm_pool_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warm_pool_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_distribution` after provisioning.\n"]
    pub fn availability_zone_distribution(&self) -> terrars::ListRef<AutoscalingGroup_AvailabilityZoneDistributionRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.availability_zone_distribution", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_specification` after provisioning.\n"]
    pub fn capacity_reservation_specification(
        &self,
    ) -> terrars::ListRef<AutoscalingGroup_CapacityReservationSpecificationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_lifecycle_policy` after provisioning.\n"]
    pub fn instance_lifecycle_policy(&self) -> terrars::ListRef<AutoscalingGroup_InstanceLifecyclePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_lifecycle_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_maintenance_policy` after provisioning.\n"]
    pub fn instance_maintenance_policy(&self) -> terrars::ListRef<AutoscalingGroup_InstanceMaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_refresh` after provisioning.\n"]
    pub fn instance_refresh(&self) -> terrars::ListRef<AutoscalingGroup_InstanceRefreshRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_refresh", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> terrars::ListRef<AutoscalingGroup_LaunchTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mixed_instances_policy` after provisioning.\n"]
    pub fn mixed_instances_policy(&self) -> terrars::ListRef<AutoscalingGroup_MixedInstancesPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mixed_instances_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AutoscalingGroup_TimeoutsRef {
        AutoscalingGroup_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_pool` after provisioning.\n"]
    pub fn warm_pool(&self) -> terrars::ListRef<AutoscalingGroup_WarmPoolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warm_pool", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroupDynamic {
    availability_zone_distribution: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingGroup_AvailabilityZoneDistribution>>,
    capacity_reservation_specification: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingGroup_CapacityReservationSpecification>>,
    initial_lifecycle_hook: std::option::Option<terrars::DynamicBlock<AutoscalingGroup_InitialLifecycleHook>>,
    instance_lifecycle_policy: std::option::Option<terrars::DynamicBlock<AutoscalingGroup_InstanceLifecyclePolicy>>,
    instance_maintenance_policy: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingGroup_InstanceMaintenancePolicy>>,
    instance_refresh: std::option::Option<terrars::DynamicBlock<AutoscalingGroup_InstanceRefresh>>,
    launch_template: std::option::Option<terrars::DynamicBlock<AutoscalingGroup_LaunchTemplate>>,
    mixed_instances_policy: std::option::Option<terrars::DynamicBlock<AutoscalingGroup_MixedInstancesPolicy>>,
    tag: std::option::Option<terrars::DynamicBlock<AutoscalingGroup_Tag>>,
    traffic_source: std::option::Option<terrars::DynamicBlock<AutoscalingGroup_TrafficSource>>,
    warm_pool: std::option::Option<terrars::DynamicBlock<AutoscalingGroup_WarmPool>>,
}

#[derive(Serialize)]
pub struct AutoscalingGroup_AvailabilityZoneDistribution {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_distribution_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AutoscalingGroup_AvailabilityZoneDistribution {
    #[doc = "Set the field `capacity_distribution_strategy`.\n"]
    pub fn set_capacity_distribution_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.capacity_distribution_strategy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AutoscalingGroup_AvailabilityZoneDistribution {
    type O = terrars::BlockAssignable<AutoscalingGroup_AvailabilityZoneDistribution>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_AvailabilityZoneDistribution {}

impl BuildAutoscalingGroup_AvailabilityZoneDistribution {
    pub fn build(self) -> AutoscalingGroup_AvailabilityZoneDistribution {
        AutoscalingGroup_AvailabilityZoneDistribution {
            capacity_distribution_strategy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_AvailabilityZoneDistributionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_AvailabilityZoneDistributionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroup_AvailabilityZoneDistributionRef {
        AutoscalingGroup_AvailabilityZoneDistributionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_AvailabilityZoneDistributionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_distribution_strategy` after provisioning.\n"]
    pub fn capacity_distribution_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_distribution_strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroup_CapacityReservationSpecification_CapacityReservationTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_resource_group_arns: std
    ::option
    ::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl AutoscalingGroup_CapacityReservationSpecification_CapacityReservationTarget {
    #[doc = "Set the field `capacity_reservation_ids`.\n"]
    pub fn set_capacity_reservation_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.capacity_reservation_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_reservation_resource_group_arns`.\n"]
    pub fn set_capacity_reservation_resource_group_arns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.capacity_reservation_resource_group_arns = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AutoscalingGroup_CapacityReservationSpecification_CapacityReservationTarget {
    type O = terrars::BlockAssignable<AutoscalingGroup_CapacityReservationSpecification_CapacityReservationTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_CapacityReservationSpecification_CapacityReservationTarget {}

impl BuildAutoscalingGroup_CapacityReservationSpecification_CapacityReservationTarget {
    pub fn build(self) -> AutoscalingGroup_CapacityReservationSpecification_CapacityReservationTarget {
        AutoscalingGroup_CapacityReservationSpecification_CapacityReservationTarget {
            capacity_reservation_ids: core::default::Default::default(),
            capacity_reservation_resource_group_arns: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_CapacityReservationSpecification_CapacityReservationTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_CapacityReservationSpecification_CapacityReservationTargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroup_CapacityReservationSpecification_CapacityReservationTargetRef {
        AutoscalingGroup_CapacityReservationSpecification_CapacityReservationTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_CapacityReservationSpecification_CapacityReservationTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_ids` after provisioning.\n"]
    pub fn capacity_reservation_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_reservation_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_resource_group_arns` after provisioning.\n"]
    pub fn capacity_reservation_resource_group_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_resource_group_arns", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroup_CapacityReservationSpecificationDynamic {
    capacity_reservation_target: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingGroup_CapacityReservationSpecification_CapacityReservationTarget>>,
}

#[derive(Serialize)]
pub struct AutoscalingGroup_CapacityReservationSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_preference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_target: std
    ::option
    ::Option<std::vec::Vec<AutoscalingGroup_CapacityReservationSpecification_CapacityReservationTarget>>,
    dynamic: AutoscalingGroup_CapacityReservationSpecificationDynamic,
}

impl AutoscalingGroup_CapacityReservationSpecification {
    #[doc = "Set the field `capacity_reservation_preference`.\n"]
    pub fn set_capacity_reservation_preference(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.capacity_reservation_preference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_reservation_target`.\n"]
    pub fn set_capacity_reservation_target(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AutoscalingGroup_CapacityReservationSpecification_CapacityReservationTarget>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.capacity_reservation_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.capacity_reservation_target = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AutoscalingGroup_CapacityReservationSpecification {
    type O = terrars::BlockAssignable<AutoscalingGroup_CapacityReservationSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_CapacityReservationSpecification {}

impl BuildAutoscalingGroup_CapacityReservationSpecification {
    pub fn build(self) -> AutoscalingGroup_CapacityReservationSpecification {
        AutoscalingGroup_CapacityReservationSpecification {
            capacity_reservation_preference: core::default::Default::default(),
            capacity_reservation_target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_CapacityReservationSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_CapacityReservationSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroup_CapacityReservationSpecificationRef {
        AutoscalingGroup_CapacityReservationSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_CapacityReservationSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_preference` after provisioning.\n"]
    pub fn capacity_reservation_preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_reservation_preference", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_target` after provisioning.\n"]
    pub fn capacity_reservation_target(
        &self,
    ) -> terrars::ListRef<AutoscalingGroup_CapacityReservationSpecification_CapacityReservationTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_reservation_target", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroup_InitialLifecycleHook {
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

impl AutoscalingGroup_InitialLifecycleHook {
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

impl terrars::ToListMappable for AutoscalingGroup_InitialLifecycleHook {
    type O = terrars::BlockAssignable<AutoscalingGroup_InitialLifecycleHook>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_InitialLifecycleHook {
    #[doc = ""]
    pub lifecycle_transition: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingGroup_InitialLifecycleHook {
    pub fn build(self) -> AutoscalingGroup_InitialLifecycleHook {
        AutoscalingGroup_InitialLifecycleHook {
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

#[allow(dead_code)]
pub struct AutoscalingGroup_InitialLifecycleHookRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_InitialLifecycleHookRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AutoscalingGroup_InitialLifecycleHookRef {
        AutoscalingGroup_InitialLifecycleHookRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_InitialLifecycleHookRef {
    #[allow(dead_code)]
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
pub struct AutoscalingGroup_InstanceLifecyclePolicy_RetentionTriggers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terminate_hook_abandon: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AutoscalingGroup_InstanceLifecyclePolicy_RetentionTriggers {
    #[doc = "Set the field `terminate_hook_abandon`.\n"]
    pub fn set_terminate_hook_abandon(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.terminate_hook_abandon = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AutoscalingGroup_InstanceLifecyclePolicy_RetentionTriggers {
    type O = terrars::BlockAssignable<AutoscalingGroup_InstanceLifecyclePolicy_RetentionTriggers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_InstanceLifecyclePolicy_RetentionTriggers {}

impl BuildAutoscalingGroup_InstanceLifecyclePolicy_RetentionTriggers {
    pub fn build(self) -> AutoscalingGroup_InstanceLifecyclePolicy_RetentionTriggers {
        AutoscalingGroup_InstanceLifecyclePolicy_RetentionTriggers {
            terminate_hook_abandon: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_InstanceLifecyclePolicy_RetentionTriggersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_InstanceLifecyclePolicy_RetentionTriggersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroup_InstanceLifecyclePolicy_RetentionTriggersRef {
        AutoscalingGroup_InstanceLifecyclePolicy_RetentionTriggersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_InstanceLifecyclePolicy_RetentionTriggersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `terminate_hook_abandon` after provisioning.\n"]
    pub fn terminate_hook_abandon(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.terminate_hook_abandon", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroup_InstanceLifecyclePolicyDynamic {
    retention_triggers: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingGroup_InstanceLifecyclePolicy_RetentionTriggers>>,
}

#[derive(Serialize)]
pub struct AutoscalingGroup_InstanceLifecyclePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_triggers: std::option::Option<std::vec::Vec<AutoscalingGroup_InstanceLifecyclePolicy_RetentionTriggers>>,
    dynamic: AutoscalingGroup_InstanceLifecyclePolicyDynamic,
}

impl AutoscalingGroup_InstanceLifecyclePolicy {
    #[doc = "Set the field `retention_triggers`.\n"]
    pub fn set_retention_triggers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AutoscalingGroup_InstanceLifecyclePolicy_RetentionTriggers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retention_triggers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retention_triggers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AutoscalingGroup_InstanceLifecyclePolicy {
    type O = terrars::BlockAssignable<AutoscalingGroup_InstanceLifecyclePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_InstanceLifecyclePolicy {}

impl BuildAutoscalingGroup_InstanceLifecyclePolicy {
    pub fn build(self) -> AutoscalingGroup_InstanceLifecyclePolicy {
        AutoscalingGroup_InstanceLifecyclePolicy {
            retention_triggers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_InstanceLifecyclePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_InstanceLifecyclePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AutoscalingGroup_InstanceLifecyclePolicyRef {
        AutoscalingGroup_InstanceLifecyclePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_InstanceLifecyclePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `retention_triggers` after provisioning.\n"]
    pub fn retention_triggers(&self) -> terrars::ListRef<AutoscalingGroup_InstanceLifecyclePolicy_RetentionTriggersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_triggers", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroup_InstanceMaintenancePolicy {
    max_healthy_percentage: terrars::PrimField<f64>,
    min_healthy_percentage: terrars::PrimField<f64>,
}

impl AutoscalingGroup_InstanceMaintenancePolicy { }

impl terrars::ToListMappable for AutoscalingGroup_InstanceMaintenancePolicy {
    type O = terrars::BlockAssignable<AutoscalingGroup_InstanceMaintenancePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_InstanceMaintenancePolicy {
    #[doc = ""]
    pub max_healthy_percentage: terrars::PrimField<f64>,
    #[doc = ""]
    pub min_healthy_percentage: terrars::PrimField<f64>,
}

impl BuildAutoscalingGroup_InstanceMaintenancePolicy {
    pub fn build(self) -> AutoscalingGroup_InstanceMaintenancePolicy {
        AutoscalingGroup_InstanceMaintenancePolicy {
            max_healthy_percentage: self.max_healthy_percentage,
            min_healthy_percentage: self.min_healthy_percentage,
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_InstanceMaintenancePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_InstanceMaintenancePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AutoscalingGroup_InstanceMaintenancePolicyRef {
        AutoscalingGroup_InstanceMaintenancePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_InstanceMaintenancePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_healthy_percentage` after provisioning.\n"]
    pub fn max_healthy_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_healthy_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `min_healthy_percentage` after provisioning.\n"]
    pub fn min_healthy_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_healthy_percentage", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroup_InstanceRefresh_Preferences_AlarmSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alarms: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl AutoscalingGroup_InstanceRefresh_Preferences_AlarmSpecification {
    #[doc = "Set the field `alarms`.\n"]
    pub fn set_alarms(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.alarms = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AutoscalingGroup_InstanceRefresh_Preferences_AlarmSpecification {
    type O = terrars::BlockAssignable<AutoscalingGroup_InstanceRefresh_Preferences_AlarmSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_InstanceRefresh_Preferences_AlarmSpecification {}

impl BuildAutoscalingGroup_InstanceRefresh_Preferences_AlarmSpecification {
    pub fn build(self) -> AutoscalingGroup_InstanceRefresh_Preferences_AlarmSpecification {
        AutoscalingGroup_InstanceRefresh_Preferences_AlarmSpecification { alarms: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_InstanceRefresh_Preferences_AlarmSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_InstanceRefresh_Preferences_AlarmSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroup_InstanceRefresh_Preferences_AlarmSpecificationRef {
        AutoscalingGroup_InstanceRefresh_Preferences_AlarmSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_InstanceRefresh_Preferences_AlarmSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alarms` after provisioning.\n"]
    pub fn alarms(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alarms", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroup_InstanceRefresh_PreferencesDynamic {
    alarm_specification: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingGroup_InstanceRefresh_Preferences_AlarmSpecification>>,
}

#[derive(Serialize)]
pub struct AutoscalingGroup_InstanceRefresh_Preferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_rollback: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    checkpoint_delay: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    checkpoint_percentages: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_warmup: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_healthy_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_healthy_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_in_protected_instances: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_matching: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standby_instances: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alarm_specification: std
    ::option
    ::Option<std::vec::Vec<AutoscalingGroup_InstanceRefresh_Preferences_AlarmSpecification>>,
    dynamic: AutoscalingGroup_InstanceRefresh_PreferencesDynamic,
}

impl AutoscalingGroup_InstanceRefresh_Preferences {
    #[doc = "Set the field `auto_rollback`.\n"]
    pub fn set_auto_rollback(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_rollback = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `max_healthy_percentage`.\n"]
    pub fn set_max_healthy_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_healthy_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_healthy_percentage`.\n"]
    pub fn set_min_healthy_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_healthy_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_in_protected_instances`.\n"]
    pub fn set_scale_in_protected_instances(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.scale_in_protected_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `skip_matching`.\n"]
    pub fn set_skip_matching(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.skip_matching = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `standby_instances`.\n"]
    pub fn set_standby_instances(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.standby_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alarm_specification`.\n"]
    pub fn set_alarm_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AutoscalingGroup_InstanceRefresh_Preferences_AlarmSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.alarm_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.alarm_specification = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AutoscalingGroup_InstanceRefresh_Preferences {
    type O = terrars::BlockAssignable<AutoscalingGroup_InstanceRefresh_Preferences>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_InstanceRefresh_Preferences {}

impl BuildAutoscalingGroup_InstanceRefresh_Preferences {
    pub fn build(self) -> AutoscalingGroup_InstanceRefresh_Preferences {
        AutoscalingGroup_InstanceRefresh_Preferences {
            auto_rollback: core::default::Default::default(),
            checkpoint_delay: core::default::Default::default(),
            checkpoint_percentages: core::default::Default::default(),
            instance_warmup: core::default::Default::default(),
            max_healthy_percentage: core::default::Default::default(),
            min_healthy_percentage: core::default::Default::default(),
            scale_in_protected_instances: core::default::Default::default(),
            skip_matching: core::default::Default::default(),
            standby_instances: core::default::Default::default(),
            alarm_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_InstanceRefresh_PreferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_InstanceRefresh_PreferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroup_InstanceRefresh_PreferencesRef {
        AutoscalingGroup_InstanceRefresh_PreferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_InstanceRefresh_PreferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_rollback` after provisioning.\n"]
    pub fn auto_rollback(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_rollback", self.base))
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

    #[doc = "Get a reference to the value of field `max_healthy_percentage` after provisioning.\n"]
    pub fn max_healthy_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_healthy_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `min_healthy_percentage` after provisioning.\n"]
    pub fn min_healthy_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_healthy_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_in_protected_instances` after provisioning.\n"]
    pub fn scale_in_protected_instances(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_in_protected_instances", self.base))
    }

    #[doc = "Get a reference to the value of field `skip_matching` after provisioning.\n"]
    pub fn skip_matching(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_matching", self.base))
    }

    #[doc = "Get a reference to the value of field `standby_instances` after provisioning.\n"]
    pub fn standby_instances(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standby_instances", self.base))
    }

    #[doc = "Get a reference to the value of field `alarm_specification` after provisioning.\n"]
    pub fn alarm_specification(
        &self,
    ) -> terrars::ListRef<AutoscalingGroup_InstanceRefresh_Preferences_AlarmSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alarm_specification", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroup_InstanceRefreshDynamic {
    preferences: std::option::Option<terrars::DynamicBlock<AutoscalingGroup_InstanceRefresh_Preferences>>,
}

#[derive(Serialize)]
pub struct AutoscalingGroup_InstanceRefresh {
    strategy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    triggers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferences: std::option::Option<std::vec::Vec<AutoscalingGroup_InstanceRefresh_Preferences>>,
    dynamic: AutoscalingGroup_InstanceRefreshDynamic,
}

impl AutoscalingGroup_InstanceRefresh {
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
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroup_InstanceRefresh_Preferences>>,
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

impl terrars::ToListMappable for AutoscalingGroup_InstanceRefresh {
    type O = terrars::BlockAssignable<AutoscalingGroup_InstanceRefresh>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_InstanceRefresh {
    #[doc = ""]
    pub strategy: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingGroup_InstanceRefresh {
    pub fn build(self) -> AutoscalingGroup_InstanceRefresh {
        AutoscalingGroup_InstanceRefresh {
            strategy: self.strategy,
            triggers: core::default::Default::default(),
            preferences: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_InstanceRefreshRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_InstanceRefreshRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AutoscalingGroup_InstanceRefreshRef {
        AutoscalingGroup_InstanceRefreshRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_InstanceRefreshRef {
    #[allow(dead_code)]
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
    pub fn preferences(&self) -> terrars::ListRef<AutoscalingGroup_InstanceRefresh_PreferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferences", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroup_LaunchTemplate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AutoscalingGroup_LaunchTemplate {
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

impl terrars::ToListMappable for AutoscalingGroup_LaunchTemplate {
    type O = terrars::BlockAssignable<AutoscalingGroup_LaunchTemplate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_LaunchTemplate {}

impl BuildAutoscalingGroup_LaunchTemplate {
    pub fn build(self) -> AutoscalingGroup_LaunchTemplate {
        AutoscalingGroup_LaunchTemplate {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_LaunchTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_LaunchTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AutoscalingGroup_LaunchTemplateRef {
        AutoscalingGroup_LaunchTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_LaunchTemplateRef {
    #[allow(dead_code)]
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
pub struct AutoscalingGroup_MixedInstancesPolicy_InstancesDistribution {
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

impl AutoscalingGroup_MixedInstancesPolicy_InstancesDistribution {
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

impl terrars::ToListMappable for AutoscalingGroup_MixedInstancesPolicy_InstancesDistribution {
    type O = terrars::BlockAssignable<AutoscalingGroup_MixedInstancesPolicy_InstancesDistribution>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_MixedInstancesPolicy_InstancesDistribution {}

impl BuildAutoscalingGroup_MixedInstancesPolicy_InstancesDistribution {
    pub fn build(self) -> AutoscalingGroup_MixedInstancesPolicy_InstancesDistribution {
        AutoscalingGroup_MixedInstancesPolicy_InstancesDistribution {
            on_demand_allocation_strategy: core::default::Default::default(),
            on_demand_base_capacity: core::default::Default::default(),
            on_demand_percentage_above_base_capacity: core::default::Default::default(),
            spot_allocation_strategy: core::default::Default::default(),
            spot_instance_pools: core::default::Default::default(),
            spot_max_price: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_MixedInstancesPolicy_InstancesDistributionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_MixedInstancesPolicy_InstancesDistributionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroup_MixedInstancesPolicy_InstancesDistributionRef {
        AutoscalingGroup_MixedInstancesPolicy_InstancesDistributionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_MixedInstancesPolicy_InstancesDistributionRef {
    #[allow(dead_code)]
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
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification {
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

impl terrars::ToListMappable for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification {
    type O =
        terrars::BlockAssignable<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification {}

impl BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification {
    pub fn build(self) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification {
            launch_template_id: core::default::Default::default(),
            launch_template_name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecificationRef {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecificationRef {
    #[allow(dead_code)]
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
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCount {
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
::ToListMappable for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCount {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCount,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCount {}

impl BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCount {
    pub fn build(
        self,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCount {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCount {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCountRef {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCountRef {
    #[allow(dead_code)]
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
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMib {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMib {
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
::ToListMappable for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMib {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMib,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMib {}

impl BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMib {
    pub fn build(
        self,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMib {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMib {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMibRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMibRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMibRef {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMibRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMibRef {
    #[allow(dead_code)]
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
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbps {
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
::ToListMappable for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbps {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbps,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbps {}

impl BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbps {
    pub fn build(
        self,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbps {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbps {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
    #[allow(dead_code)]
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
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpu {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpu {
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
::ToListMappable for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpu {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpu,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpu {}

impl BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpu {
    pub fn build(
        self,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpu {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpu {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpuRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpuRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpuRef {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpuRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpuRef {
    #[allow(dead_code)]
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
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMib {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMib {
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
::ToListMappable for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMib {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMib,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMib {}

impl BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMib {
    pub fn build(
        self,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMib {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMib {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMibRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMibRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMibRef {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMibRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMibRef {
    #[allow(dead_code)]
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
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbps {
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
::ToListMappable for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbps {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbps,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbps {}

impl BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbps {
    pub fn build(
        self,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbps {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbps {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbpsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbpsRef {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbpsRef {
    #[allow(dead_code)]
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
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCount {
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
::ToListMappable for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCount {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCount,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCount {}

impl BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCount {
    pub fn build(
        self,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCount {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCount {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCountRef {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCountRef {
    #[allow(dead_code)]
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
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGb {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGb {
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
::ToListMappable for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGb {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGb,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGb {}

impl BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGb {
    pub fn build(
        self,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGb {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGb {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGbRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGbRef {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGbRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGbRef {
    #[allow(dead_code)]
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
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCount {
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
::ToListMappable for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCount {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCount,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCount {}

impl BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCount {
    pub fn build(
        self,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCount {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCount {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCountRef {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCountRef {
    #[allow(dead_code)]
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
struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirementsDynamic {
    accelerator_count: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCount,
        >,
    >,
    accelerator_total_memory_mib: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMib,
        >,
    >,
    baseline_ebs_bandwidth_mbps: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbps,
        >,
    >,
    memory_gib_per_vcpu: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpu,
        >,
    >,
    memory_mib: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMib>,
    >,
    network_bandwidth_gbps: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbps,
        >,
    >,
    network_interface_count: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCount,
        >,
    >,
    total_local_storage_gb: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGb,
        >,
    >,
    vcpu_count: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCount>,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_manufacturers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_instance_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
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
    max_spot_price_as_percentage_of_optimal_on_demand_price: std::option::Option<terrars::PrimField<f64>>,
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
        ::Vec<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCount>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_total_memory_mib: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMib,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    baseline_ebs_bandwidth_mbps: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbps,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_gib_per_vcpu: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpu>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_mib: std
    ::option
    ::Option<
        std::vec::Vec<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMib>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_bandwidth_gbps: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbps>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface_count: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCount,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_local_storage_gb: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGb>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vcpu_count: std
    ::option
    ::Option<
        std::vec::Vec<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCount>,
    >,
    dynamic: AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirementsDynamic,
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements {
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

    #[doc = "Set the field `allowed_instance_types`.\n"]
    pub fn set_allowed_instance_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_instance_types = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `max_spot_price_as_percentage_of_optimal_on_demand_price`.\n"]
    pub fn set_max_spot_price_as_percentage_of_optimal_on_demand_price(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.max_spot_price_as_percentage_of_optimal_on_demand_price = std::option::Option::Some(v.into());
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
                            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCount,
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
                            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMib,
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
                            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbps,
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
                            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpu,
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
                            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMib,
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

    #[doc = "Set the field `network_bandwidth_gbps`.\n"]
    pub fn set_network_bandwidth_gbps(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbps,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_bandwidth_gbps = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_bandwidth_gbps = std::option::Option::Some(d);
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
                            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCount,
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
                            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGb,
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
                            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCount,
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

impl terrars::ToListMappable for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements {
    type O =
        terrars::BlockAssignable<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements {}

impl BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements {
    pub fn build(self) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements {
            accelerator_manufacturers: core::default::Default::default(),
            accelerator_names: core::default::Default::default(),
            accelerator_types: core::default::Default::default(),
            allowed_instance_types: core::default::Default::default(),
            bare_metal: core::default::Default::default(),
            burstable_performance: core::default::Default::default(),
            cpu_manufacturers: core::default::Default::default(),
            excluded_instance_types: core::default::Default::default(),
            instance_generations: core::default::Default::default(),
            local_storage: core::default::Default::default(),
            local_storage_types: core::default::Default::default(),
            max_spot_price_as_percentage_of_optimal_on_demand_price: core::default::Default::default(),
            on_demand_max_price_percentage_over_lowest_price: core::default::Default::default(),
            require_hibernate_support: core::default::Default::default(),
            spot_max_price_percentage_over_lowest_price: core::default::Default::default(),
            accelerator_count: core::default::Default::default(),
            accelerator_total_memory_mib: core::default::Default::default(),
            baseline_ebs_bandwidth_mbps: core::default::Default::default(),
            memory_gib_per_vcpu: core::default::Default::default(),
            memory_mib: core::default::Default::default(),
            network_bandwidth_gbps: core::default::Default::default(),
            network_interface_count: core::default::Default::default(),
            total_local_storage_gb: core::default::Default::default(),
            vcpu_count: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirementsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirementsRef {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirementsRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `allowed_instance_types` after provisioning.\n"]
    pub fn allowed_instance_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_instance_types", self.base))
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
        "Get a reference to the value of field `max_spot_price_as_percentage_of_optimal_on_demand_price` after provisioning.\n"]
    pub fn max_spot_price_as_percentage_of_optimal_on_demand_price(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_spot_price_as_percentage_of_optimal_on_demand_price", self.base),
        )
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
    ::ListRef<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_total_memory_mib` after provisioning.\n"]
    pub fn accelerator_total_memory_mib(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMibRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerator_total_memory_mib", self.base))
    }

    #[doc = "Get a reference to the value of field `baseline_ebs_bandwidth_mbps` after provisioning.\n"]
    pub fn baseline_ebs_bandwidth_mbps(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbpsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.baseline_ebs_bandwidth_mbps", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_gib_per_vcpu` after provisioning.\n"]
    pub fn memory_gib_per_vcpu(
        &self,
    ) -> terrars
    ::ListRef<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpuRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_gib_per_vcpu", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_mib` after provisioning.\n"]
    pub fn memory_mib(
        &self,
    ) -> terrars
    ::ListRef<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMibRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_mib", self.base))
    }

    #[doc = "Get a reference to the value of field `network_bandwidth_gbps` after provisioning.\n"]
    pub fn network_bandwidth_gbps(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbpsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_bandwidth_gbps", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_count` after provisioning.\n"]
    pub fn network_interface_count(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCountRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface_count", self.base))
    }

    #[doc = "Get a reference to the value of field `total_local_storage_gb` after provisioning.\n"]
    pub fn total_local_storage_gb(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGbRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.total_local_storage_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `vcpu_count` after provisioning.\n"]
    pub fn vcpu_count(
        &self,
    ) -> terrars
    ::ListRef<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vcpu_count", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_OverrideDynamic {
    instance_requirements: std
    ::option
    ::Option<
        terrars::DynamicBlock<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements>,
    >,
    launch_template_specification: std
    ::option
    ::Option<
        terrars::DynamicBlock<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification>,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_capacity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_requirements: std
    ::option
    ::Option<std::vec::Vec<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_specification: std
    ::option
    ::Option<std::vec::Vec<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification>>,
    dynamic: AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_OverrideDynamic,
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override {
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
                            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements,
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
                            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification,
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

impl terrars::ToListMappable for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override {
    type O = terrars::BlockAssignable<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override {}

impl BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override {
    pub fn build(self) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override {
            instance_type: core::default::Default::default(),
            weighted_capacity: core::default::Default::default(),
            instance_requirements: core::default::Default::default(),
            launch_template_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_OverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_OverrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_OverrideRef {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_OverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_OverrideRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirementsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_requirements", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template_specification` after provisioning.\n"]
    pub fn launch_template_specification(
        &self,
    ) -> terrars::ListRef<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template_specification", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplateDynamic {
    launch_template_specification: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification>>,
    override_: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override>>,
}

#[derive(Serialize)]
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_specification: std
    ::option
    ::Option<std::vec::Vec<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification>>,
    #[serde(rename = "override", skip_serializing_if = "std::option::Option::is_none")]
    override_: std::option::Option<std::vec::Vec<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override>>,
    dynamic: AutoscalingGroup_MixedInstancesPolicy_LaunchTemplateDynamic,
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate {
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
                            AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification,
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
                    ::Into<terrars::BlockAssignable<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override>>,
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

impl terrars::ToListMappable for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate {
    type O = terrars::BlockAssignable<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate {}

impl BuildAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate {
    pub fn build(self) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate {
            launch_template_specification: core::default::Default::default(),
            override_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_MixedInstancesPolicy_LaunchTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_MixedInstancesPolicy_LaunchTemplateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroup_MixedInstancesPolicy_LaunchTemplateRef {
        AutoscalingGroup_MixedInstancesPolicy_LaunchTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_MixedInstancesPolicy_LaunchTemplateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `launch_template_specification` after provisioning.\n"]
    pub fn launch_template_specification(
        &self,
    ) -> terrars::ListRef<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> terrars::ListRef<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_OverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.override", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroup_MixedInstancesPolicyDynamic {
    instances_distribution: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingGroup_MixedInstancesPolicy_InstancesDistribution>>,
    launch_template: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate>>,
}

#[derive(Serialize)]
pub struct AutoscalingGroup_MixedInstancesPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instances_distribution: std
    ::option
    ::Option<std::vec::Vec<AutoscalingGroup_MixedInstancesPolicy_InstancesDistribution>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template: std::option::Option<std::vec::Vec<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate>>,
    dynamic: AutoscalingGroup_MixedInstancesPolicyDynamic,
}

impl AutoscalingGroup_MixedInstancesPolicy {
    #[doc = "Set the field `instances_distribution`.\n"]
    pub fn set_instances_distribution(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AutoscalingGroup_MixedInstancesPolicy_InstancesDistribution>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplate>>,
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

impl terrars::ToListMappable for AutoscalingGroup_MixedInstancesPolicy {
    type O = terrars::BlockAssignable<AutoscalingGroup_MixedInstancesPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_MixedInstancesPolicy {}

impl BuildAutoscalingGroup_MixedInstancesPolicy {
    pub fn build(self) -> AutoscalingGroup_MixedInstancesPolicy {
        AutoscalingGroup_MixedInstancesPolicy {
            instances_distribution: core::default::Default::default(),
            launch_template: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_MixedInstancesPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_MixedInstancesPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AutoscalingGroup_MixedInstancesPolicyRef {
        AutoscalingGroup_MixedInstancesPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_MixedInstancesPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instances_distribution` after provisioning.\n"]
    pub fn instances_distribution(
        &self,
    ) -> terrars::ListRef<AutoscalingGroup_MixedInstancesPolicy_InstancesDistributionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances_distribution", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> terrars::ListRef<AutoscalingGroup_MixedInstancesPolicy_LaunchTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroup_Tag {
    key: terrars::PrimField<std::string::String>,
    propagate_at_launch: terrars::PrimField<bool>,
    value: terrars::PrimField<std::string::String>,
}

impl AutoscalingGroup_Tag { }

impl terrars::ToListMappable for AutoscalingGroup_Tag {
    type O = terrars::BlockAssignable<AutoscalingGroup_Tag>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_Tag {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub propagate_at_launch: terrars::PrimField<bool>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingGroup_Tag {
    pub fn build(self) -> AutoscalingGroup_Tag {
        AutoscalingGroup_Tag {
            key: self.key,
            propagate_at_launch: self.propagate_at_launch,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_TagRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_TagRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AutoscalingGroup_TagRef {
        AutoscalingGroup_TagRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_TagRef {
    #[allow(dead_code)]
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
pub struct AutoscalingGroup_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AutoscalingGroup_Timeouts {
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

impl terrars::ToListMappable for AutoscalingGroup_Timeouts {
    type O = terrars::BlockAssignable<AutoscalingGroup_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_Timeouts {}

impl BuildAutoscalingGroup_Timeouts {
    pub fn build(self) -> AutoscalingGroup_Timeouts {
        AutoscalingGroup_Timeouts {
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AutoscalingGroup_TimeoutsRef {
        AutoscalingGroup_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_TimeoutsRef {
    #[allow(dead_code)]
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
pub struct AutoscalingGroup_TrafficSource {
    identifier: terrars::PrimField<std::string::String>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AutoscalingGroup_TrafficSource {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AutoscalingGroup_TrafficSource {
    type O = terrars::BlockAssignable<AutoscalingGroup_TrafficSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_TrafficSource {
    #[doc = ""]
    pub identifier: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingGroup_TrafficSource {
    pub fn build(self) -> AutoscalingGroup_TrafficSource {
        AutoscalingGroup_TrafficSource {
            identifier: self.identifier,
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_TrafficSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_TrafficSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AutoscalingGroup_TrafficSourceRef {
        AutoscalingGroup_TrafficSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_TrafficSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingGroup_WarmPool_InstanceReusePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reuse_on_scale_in: std::option::Option<terrars::PrimField<bool>>,
}

impl AutoscalingGroup_WarmPool_InstanceReusePolicy {
    #[doc = "Set the field `reuse_on_scale_in`.\n"]
    pub fn set_reuse_on_scale_in(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.reuse_on_scale_in = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AutoscalingGroup_WarmPool_InstanceReusePolicy {
    type O = terrars::BlockAssignable<AutoscalingGroup_WarmPool_InstanceReusePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_WarmPool_InstanceReusePolicy {}

impl BuildAutoscalingGroup_WarmPool_InstanceReusePolicy {
    pub fn build(self) -> AutoscalingGroup_WarmPool_InstanceReusePolicy {
        AutoscalingGroup_WarmPool_InstanceReusePolicy { reuse_on_scale_in: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_WarmPool_InstanceReusePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_WarmPool_InstanceReusePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingGroup_WarmPool_InstanceReusePolicyRef {
        AutoscalingGroup_WarmPool_InstanceReusePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_WarmPool_InstanceReusePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `reuse_on_scale_in` after provisioning.\n"]
    pub fn reuse_on_scale_in(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reuse_on_scale_in", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingGroup_WarmPoolDynamic {
    instance_reuse_policy: std::option::Option<terrars::DynamicBlock<AutoscalingGroup_WarmPool_InstanceReusePolicy>>,
}

#[derive(Serialize)]
pub struct AutoscalingGroup_WarmPool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_group_prepared_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_reuse_policy: std::option::Option<std::vec::Vec<AutoscalingGroup_WarmPool_InstanceReusePolicy>>,
    dynamic: AutoscalingGroup_WarmPoolDynamic,
}

impl AutoscalingGroup_WarmPool {
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
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingGroup_WarmPool_InstanceReusePolicy>>,
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

impl terrars::ToListMappable for AutoscalingGroup_WarmPool {
    type O = terrars::BlockAssignable<AutoscalingGroup_WarmPool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingGroup_WarmPool {}

impl BuildAutoscalingGroup_WarmPool {
    pub fn build(self) -> AutoscalingGroup_WarmPool {
        AutoscalingGroup_WarmPool {
            max_group_prepared_capacity: core::default::Default::default(),
            min_size: core::default::Default::default(),
            pool_state: core::default::Default::default(),
            instance_reuse_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingGroup_WarmPoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingGroup_WarmPoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AutoscalingGroup_WarmPoolRef {
        AutoscalingGroup_WarmPoolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingGroup_WarmPoolRef {
    #[allow(dead_code)]
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
    pub fn instance_reuse_policy(&self) -> terrars::ListRef<AutoscalingGroup_WarmPool_InstanceReusePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_reuse_policy", self.base))
    }
}
