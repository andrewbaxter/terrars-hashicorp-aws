#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAutoscalingGroupData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataAutoscalingGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAutoscalingGroupData_>,
}

#[derive(Clone)]
pub struct DataAutoscalingGroup(std::rc::Rc<DataAutoscalingGroup_>);

impl DataAutoscalingGroup {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `default_cooldown` after provisioning.\n"]
    pub fn default_cooldown(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_cooldown", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `instance_maintenance_policy` after provisioning.\n"]
    pub fn instance_maintenance_policy(&self) -> terrars::ListRef<DataAutoscalingGroup_InstanceMaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_configuration` after provisioning.\n"]
    pub fn launch_configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> terrars::ListRef<DataAutoscalingGroup_LaunchTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `min_size` after provisioning.\n"]
    pub fn min_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mixed_instances_policy` after provisioning.\n"]
    pub fn mixed_instances_policy(&self) -> terrars::ListRef<DataAutoscalingGroup_MixedInstancesPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mixed_instances_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `new_instances_protected_from_scale_in` after provisioning.\n"]
    pub fn new_instances_protected_from_scale_in(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.new_instances_protected_from_scale_in", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `placement_group` after provisioning.\n"]
    pub fn placement_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `predicted_capacity` after provisioning.\n"]
    pub fn predicted_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.predicted_capacity", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `suspended_processes` after provisioning.\n"]
    pub fn suspended_processes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.suspended_processes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> terrars::SetRef<DataAutoscalingGroup_TagRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_group_arns` after provisioning.\n"]
    pub fn target_group_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_group_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `termination_policies` after provisioning.\n"]
    pub fn termination_policies(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.termination_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_source` after provisioning.\n"]
    pub fn traffic_source(&self) -> terrars::SetRef<DataAutoscalingGroup_TrafficSourceRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.traffic_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_zone_identifier` after provisioning.\n"]
    pub fn vpc_zone_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_zone_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_pool` after provisioning.\n"]
    pub fn warm_pool(&self) -> terrars::ListRef<DataAutoscalingGroup_WarmPoolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warm_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_pool_size` after provisioning.\n"]
    pub fn warm_pool_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warm_pool_size", self.extract_ref()))
    }
}

impl terrars::Referable for DataAutoscalingGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAutoscalingGroup { }

impl terrars::ToListMappable for DataAutoscalingGroup {
    type O = terrars::ListRef<DataAutoscalingGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAutoscalingGroup_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_autoscaling_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAutoscalingGroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataAutoscalingGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAutoscalingGroup {
        let out = DataAutoscalingGroup(std::rc::Rc::new(DataAutoscalingGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAutoscalingGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAutoscalingGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAutoscalingGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAutoscalingGroupRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_cooldown` after provisioning.\n"]
    pub fn default_cooldown(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_cooldown", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `instance_maintenance_policy` after provisioning.\n"]
    pub fn instance_maintenance_policy(&self) -> terrars::ListRef<DataAutoscalingGroup_InstanceMaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_configuration` after provisioning.\n"]
    pub fn launch_configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> terrars::ListRef<DataAutoscalingGroup_LaunchTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `min_size` after provisioning.\n"]
    pub fn min_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mixed_instances_policy` after provisioning.\n"]
    pub fn mixed_instances_policy(&self) -> terrars::ListRef<DataAutoscalingGroup_MixedInstancesPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mixed_instances_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `new_instances_protected_from_scale_in` after provisioning.\n"]
    pub fn new_instances_protected_from_scale_in(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.new_instances_protected_from_scale_in", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `placement_group` after provisioning.\n"]
    pub fn placement_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `predicted_capacity` after provisioning.\n"]
    pub fn predicted_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.predicted_capacity", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `suspended_processes` after provisioning.\n"]
    pub fn suspended_processes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.suspended_processes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> terrars::SetRef<DataAutoscalingGroup_TagRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_group_arns` after provisioning.\n"]
    pub fn target_group_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_group_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `termination_policies` after provisioning.\n"]
    pub fn termination_policies(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.termination_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_source` after provisioning.\n"]
    pub fn traffic_source(&self) -> terrars::SetRef<DataAutoscalingGroup_TrafficSourceRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.traffic_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_zone_identifier` after provisioning.\n"]
    pub fn vpc_zone_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_zone_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_pool` after provisioning.\n"]
    pub fn warm_pool(&self) -> terrars::ListRef<DataAutoscalingGroup_WarmPoolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warm_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_pool_size` after provisioning.\n"]
    pub fn warm_pool_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warm_pool_size", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAutoscalingGroup_InstanceMaintenancePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_healthy_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_healthy_percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAutoscalingGroup_InstanceMaintenancePolicy {
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
}

impl terrars::ToListMappable for DataAutoscalingGroup_InstanceMaintenancePolicy {
    type O = terrars::BlockAssignable<DataAutoscalingGroup_InstanceMaintenancePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroup_InstanceMaintenancePolicy {}

impl BuildDataAutoscalingGroup_InstanceMaintenancePolicy {
    pub fn build(self) -> DataAutoscalingGroup_InstanceMaintenancePolicy {
        DataAutoscalingGroup_InstanceMaintenancePolicy {
            max_healthy_percentage: core::default::Default::default(),
            min_healthy_percentage: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAutoscalingGroup_InstanceMaintenancePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAutoscalingGroup_InstanceMaintenancePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAutoscalingGroup_InstanceMaintenancePolicyRef {
        DataAutoscalingGroup_InstanceMaintenancePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroup_InstanceMaintenancePolicyRef {
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
pub struct DataAutoscalingGroup_LaunchTemplate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAutoscalingGroup_LaunchTemplate {
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

impl terrars::ToListMappable for DataAutoscalingGroup_LaunchTemplate {
    type O = terrars::BlockAssignable<DataAutoscalingGroup_LaunchTemplate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroup_LaunchTemplate {}

impl BuildDataAutoscalingGroup_LaunchTemplate {
    pub fn build(self) -> DataAutoscalingGroup_LaunchTemplate {
        DataAutoscalingGroup_LaunchTemplate {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAutoscalingGroup_LaunchTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAutoscalingGroup_LaunchTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAutoscalingGroup_LaunchTemplateRef {
        DataAutoscalingGroup_LaunchTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroup_LaunchTemplateRef {
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
pub struct DataAutoscalingGroup_MixedInstancesPolicy_InstancesDistribution {
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

impl DataAutoscalingGroup_MixedInstancesPolicy_InstancesDistribution {
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

impl terrars::ToListMappable for DataAutoscalingGroup_MixedInstancesPolicy_InstancesDistribution {
    type O = terrars::BlockAssignable<DataAutoscalingGroup_MixedInstancesPolicy_InstancesDistribution>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroup_MixedInstancesPolicy_InstancesDistribution {}

impl BuildDataAutoscalingGroup_MixedInstancesPolicy_InstancesDistribution {
    pub fn build(self) -> DataAutoscalingGroup_MixedInstancesPolicy_InstancesDistribution {
        DataAutoscalingGroup_MixedInstancesPolicy_InstancesDistribution {
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
pub struct DataAutoscalingGroup_MixedInstancesPolicy_InstancesDistributionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAutoscalingGroup_MixedInstancesPolicy_InstancesDistributionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_InstancesDistributionRef {
        DataAutoscalingGroup_MixedInstancesPolicy_InstancesDistributionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroup_MixedInstancesPolicy_InstancesDistributionRef {
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
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification {
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

impl terrars::ToListMappable for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification {
    type O =
        terrars
        ::BlockAssignable<DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification {}

impl BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification {
    pub fn build(self) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification {
            launch_template_id: core::default::Default::default(),
            launch_template_name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecificationRef {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecificationRef {
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
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCount {
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
::ToListMappable for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCount {
    type O =
        terrars
        ::BlockAssignable<
            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCount,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCount {}

impl BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCount {
    pub fn build(
        self,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCount {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCount {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCountRef {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCountRef {
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
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMib {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMib {
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
::ToListMappable for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMib {
    type O =
        terrars
        ::BlockAssignable<
            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMib,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMib {}

impl BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMib {
    pub fn build(
        self,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMib {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMib {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMibRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMibRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMibRef {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMibRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMibRef {
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
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbps {
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
::ToListMappable for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbps {
    type O =
        terrars
        ::BlockAssignable<
            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbps,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbps {}

impl BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbps {
    pub fn build(
        self,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbps {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbps {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
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
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpu {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpu {
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
::ToListMappable for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpu {
    type O =
        terrars
        ::BlockAssignable<
            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpu,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpu {}

impl BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpu {
    pub fn build(
        self,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpu {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpu {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpuRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpuRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpuRef {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpuRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpuRef {
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
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMib {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMib {
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
::ToListMappable for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMib {
    type O =
        terrars
        ::BlockAssignable<
            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMib,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMib {}

impl BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMib {
    pub fn build(
        self,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMib {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMib {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMibRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMibRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMibRef {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMibRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMibRef {
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
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbps {
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
::ToListMappable for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbps {
    type O =
        terrars
        ::BlockAssignable<
            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbps,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbps {}

impl BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbps {
    pub fn build(
        self,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbps {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbps {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbpsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbpsRef {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbpsRef {
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
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCount {
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
::ToListMappable for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCount {
    type O =
        terrars
        ::BlockAssignable<
            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCount,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCount {}

impl BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCount {
    pub fn build(
        self,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCount {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCount {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCountRef {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCountRef {
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
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGb {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGb {
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
::ToListMappable for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGb {
    type O =
        terrars
        ::BlockAssignable<
            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGb,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGb {}

impl BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGb {
    pub fn build(
        self,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGb {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGb {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGbRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGbRef {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGbRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGbRef {
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
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCount {
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
::ToListMappable for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCount {
    type O =
        terrars
        ::BlockAssignable<
            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCount,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCount {}

impl BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCount {
    pub fn build(
        self,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCount {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCount {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCountRef {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCountRef {
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
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCount,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_manufacturers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_total_memory_mib: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMib,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_instance_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bare_metal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    baseline_ebs_bandwidth_mbps: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbps,
        >,
    >,
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
    memory_gib_per_vcpu: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpu,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_mib: std
    ::option
    ::Option<
        terrars
        ::ListField<DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMib>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_bandwidth_gbps: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbps,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface_count: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCount,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand_max_price_percentage_over_lowest_price: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_hibernate_support: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_max_price_percentage_over_lowest_price: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_local_storage_gb: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGb,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vcpu_count: std
    ::option
    ::Option<
        terrars
        ::ListField<DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCount>,
    >,
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements {
    #[doc = "Set the field `accelerator_count`.\n"]
    pub fn set_accelerator_count(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCount,
                        >,
                    >,
    ) -> Self {
        self.accelerator_count = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `accelerator_total_memory_mib`.\n"]
    pub fn set_accelerator_total_memory_mib(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMib,
                        >,
                    >,
    ) -> Self {
        self.accelerator_total_memory_mib = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `baseline_ebs_bandwidth_mbps`.\n"]
    pub fn set_baseline_ebs_bandwidth_mbps(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbps,
                        >,
                    >,
    ) -> Self {
        self.baseline_ebs_bandwidth_mbps = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `memory_gib_per_vcpu`.\n"]
    pub fn set_memory_gib_per_vcpu(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpu,
                        >,
                    >,
    ) -> Self {
        self.memory_gib_per_vcpu = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMib,
                        >,
                    >,
    ) -> Self {
        self.memory_mib = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbps,
                        >,
                    >,
    ) -> Self {
        self.network_bandwidth_gbps = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCount,
                        >,
                    >,
    ) -> Self {
        self.network_interface_count = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `total_local_storage_gb`.\n"]
    pub fn set_total_local_storage_gb(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGb,
                        >,
                    >,
    ) -> Self {
        self.total_local_storage_gb = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCount,
                        >,
                    >,
    ) -> Self {
        self.vcpu_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements {
    type O =
        terrars
        ::BlockAssignable<DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements {}

impl BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements {
    pub fn build(self) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements {
            accelerator_count: core::default::Default::default(),
            accelerator_manufacturers: core::default::Default::default(),
            accelerator_names: core::default::Default::default(),
            accelerator_total_memory_mib: core::default::Default::default(),
            accelerator_types: core::default::Default::default(),
            allowed_instance_types: core::default::Default::default(),
            bare_metal: core::default::Default::default(),
            baseline_ebs_bandwidth_mbps: core::default::Default::default(),
            burstable_performance: core::default::Default::default(),
            cpu_manufacturers: core::default::Default::default(),
            excluded_instance_types: core::default::Default::default(),
            instance_generations: core::default::Default::default(),
            local_storage: core::default::Default::default(),
            local_storage_types: core::default::Default::default(),
            max_spot_price_as_percentage_of_optimal_on_demand_price: core::default::Default::default(),
            memory_gib_per_vcpu: core::default::Default::default(),
            memory_mib: core::default::Default::default(),
            network_bandwidth_gbps: core::default::Default::default(),
            network_interface_count: core::default::Default::default(),
            on_demand_max_price_percentage_over_lowest_price: core::default::Default::default(),
            require_hibernate_support: core::default::Default::default(),
            spot_max_price_percentage_over_lowest_price: core::default::Default::default(),
            total_local_storage_gb: core::default::Default::default(),
            vcpu_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirementsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirementsRef {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirementsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\n"]
    pub fn accelerator_count(
        &self,
    ) -> terrars
    ::ListRef<
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorCountRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_manufacturers` after provisioning.\n"]
    pub fn accelerator_manufacturers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.accelerator_manufacturers", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_names` after provisioning.\n"]
    pub fn accelerator_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.accelerator_names", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_total_memory_mib` after provisioning.\n"]
    pub fn accelerator_total_memory_mib(
        &self,
    ) -> terrars
    ::ListRef<
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_AcceleratorTotalMemoryMibRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerator_total_memory_mib", self.base))
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

    #[doc = "Get a reference to the value of field `baseline_ebs_bandwidth_mbps` after provisioning.\n"]
    pub fn baseline_ebs_bandwidth_mbps(
        &self,
    ) -> terrars
    ::ListRef<
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_BaselineEbsBandwidthMbpsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.baseline_ebs_bandwidth_mbps", self.base))
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

    #[doc = "Get a reference to the value of field `memory_gib_per_vcpu` after provisioning.\n"]
    pub fn memory_gib_per_vcpu(
        &self,
    ) -> terrars
    ::ListRef<
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryGibPerVcpuRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_gib_per_vcpu", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_mib` after provisioning.\n"]
    pub fn memory_mib(
        &self,
    ) -> terrars
    ::ListRef<DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_MemoryMibRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_mib", self.base))
    }

    #[doc = "Get a reference to the value of field `network_bandwidth_gbps` after provisioning.\n"]
    pub fn network_bandwidth_gbps(
        &self,
    ) -> terrars
    ::ListRef<
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkBandwidthGbpsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_bandwidth_gbps", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_count` after provisioning.\n"]
    pub fn network_interface_count(
        &self,
    ) -> terrars
    ::ListRef<
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_NetworkInterfaceCountRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface_count", self.base))
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

    #[doc = "Get a reference to the value of field `total_local_storage_gb` after provisioning.\n"]
    pub fn total_local_storage_gb(
        &self,
    ) -> terrars
    ::ListRef<
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_TotalLocalStorageGbRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.total_local_storage_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `vcpu_count` after provisioning.\n"]
    pub fn vcpu_count(
        &self,
    ) -> terrars
    ::ListRef<DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements_VcpuCountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vcpu_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_requirements: std
    ::option
    ::Option<
        terrars::ListField<DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_specification: std
    ::option
    ::Option<
        terrars::ListField<DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_capacity: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override {
    #[doc = "Set the field `instance_requirements`.\n"]
    pub fn set_instance_requirements(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirements,
                        >,
                    >,
    ) -> Self {
        self.instance_requirements = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification,
                        >,
                    >,
    ) -> Self {
        self.launch_template_specification = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weighted_capacity`.\n"]
    pub fn set_weighted_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.weighted_capacity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override {
    type O = terrars::BlockAssignable<DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override {}

impl BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override {
    pub fn build(self) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override {
            instance_requirements: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            launch_template_specification: core::default::Default::default(),
            weighted_capacity: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_OverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_OverrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_OverrideRef {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_OverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_OverrideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_requirements` after provisioning.\n"]
    pub fn instance_requirements(
        &self,
    ) -> terrars::ListRef<DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override_InstanceRequirementsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_requirements", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template_specification` after provisioning.\n"]
    pub fn launch_template_specification(
        &self,
    ) -> terrars::ListRef<DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_capacity` after provisioning.\n"]
    pub fn weighted_capacity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weighted_capacity", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_specification: std
    ::option
    ::Option<
        terrars::ListField<DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification>,
    >,
    #[serde(rename = "override", skip_serializing_if = "std::option::Option::is_none")]
    override_: std
    ::option
    ::Option<terrars::ListField<DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override>>,
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate {
    #[doc = "Set the field `launch_template_specification`.\n"]
    pub fn set_launch_template_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecification,
                        >,
                    >,
    ) -> Self {
        self.launch_template_specification = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_Override>>,
    ) -> Self {
        self.override_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate {
    type O = terrars::BlockAssignable<DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate {}

impl BuildDataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate {
    pub fn build(self) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate {
            launch_template_specification: core::default::Default::default(),
            override_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplateRef {
        DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `launch_template_specification` after provisioning.\n"]
    pub fn launch_template_specification(
        &self,
    ) -> terrars::ListRef<DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_LaunchTemplateSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> terrars::ListRef<DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate_OverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.override", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAutoscalingGroup_MixedInstancesPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instances_distribution: std
    ::option
    ::Option<terrars::ListField<DataAutoscalingGroup_MixedInstancesPolicy_InstancesDistribution>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template: std::option::Option<terrars::ListField<DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate>>,
}

impl DataAutoscalingGroup_MixedInstancesPolicy {
    #[doc = "Set the field `instances_distribution`.\n"]
    pub fn set_instances_distribution(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAutoscalingGroup_MixedInstancesPolicy_InstancesDistribution>>,
    ) -> Self {
        self.instances_distribution = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_template`.\n"]
    pub fn set_launch_template(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplate>>,
    ) -> Self {
        self.launch_template = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAutoscalingGroup_MixedInstancesPolicy {
    type O = terrars::BlockAssignable<DataAutoscalingGroup_MixedInstancesPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroup_MixedInstancesPolicy {}

impl BuildDataAutoscalingGroup_MixedInstancesPolicy {
    pub fn build(self) -> DataAutoscalingGroup_MixedInstancesPolicy {
        DataAutoscalingGroup_MixedInstancesPolicy {
            instances_distribution: core::default::Default::default(),
            launch_template: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAutoscalingGroup_MixedInstancesPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAutoscalingGroup_MixedInstancesPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAutoscalingGroup_MixedInstancesPolicyRef {
        DataAutoscalingGroup_MixedInstancesPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroup_MixedInstancesPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instances_distribution` after provisioning.\n"]
    pub fn instances_distribution(
        &self,
    ) -> terrars::ListRef<DataAutoscalingGroup_MixedInstancesPolicy_InstancesDistributionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances_distribution", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> terrars::ListRef<DataAutoscalingGroup_MixedInstancesPolicy_LaunchTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAutoscalingGroup_Tag {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagate_at_launch: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAutoscalingGroup_Tag {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `propagate_at_launch`.\n"]
    pub fn set_propagate_at_launch(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.propagate_at_launch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAutoscalingGroup_Tag {
    type O = terrars::BlockAssignable<DataAutoscalingGroup_Tag>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroup_Tag {}

impl BuildDataAutoscalingGroup_Tag {
    pub fn build(self) -> DataAutoscalingGroup_Tag {
        DataAutoscalingGroup_Tag {
            key: core::default::Default::default(),
            propagate_at_launch: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAutoscalingGroup_TagRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAutoscalingGroup_TagRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAutoscalingGroup_TagRef {
        DataAutoscalingGroup_TagRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroup_TagRef {
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
pub struct DataAutoscalingGroup_TrafficSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAutoscalingGroup_TrafficSource {
    #[doc = "Set the field `identifier`.\n"]
    pub fn set_identifier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAutoscalingGroup_TrafficSource {
    type O = terrars::BlockAssignable<DataAutoscalingGroup_TrafficSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroup_TrafficSource {}

impl BuildDataAutoscalingGroup_TrafficSource {
    pub fn build(self) -> DataAutoscalingGroup_TrafficSource {
        DataAutoscalingGroup_TrafficSource {
            identifier: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAutoscalingGroup_TrafficSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAutoscalingGroup_TrafficSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAutoscalingGroup_TrafficSourceRef {
        DataAutoscalingGroup_TrafficSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroup_TrafficSourceRef {
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
pub struct DataAutoscalingGroup_WarmPool_InstanceReusePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reuse_on_scale_in: std::option::Option<terrars::PrimField<bool>>,
}

impl DataAutoscalingGroup_WarmPool_InstanceReusePolicy {
    #[doc = "Set the field `reuse_on_scale_in`.\n"]
    pub fn set_reuse_on_scale_in(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.reuse_on_scale_in = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAutoscalingGroup_WarmPool_InstanceReusePolicy {
    type O = terrars::BlockAssignable<DataAutoscalingGroup_WarmPool_InstanceReusePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroup_WarmPool_InstanceReusePolicy {}

impl BuildDataAutoscalingGroup_WarmPool_InstanceReusePolicy {
    pub fn build(self) -> DataAutoscalingGroup_WarmPool_InstanceReusePolicy {
        DataAutoscalingGroup_WarmPool_InstanceReusePolicy { reuse_on_scale_in: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAutoscalingGroup_WarmPool_InstanceReusePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAutoscalingGroup_WarmPool_InstanceReusePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAutoscalingGroup_WarmPool_InstanceReusePolicyRef {
        DataAutoscalingGroup_WarmPool_InstanceReusePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroup_WarmPool_InstanceReusePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `reuse_on_scale_in` after provisioning.\n"]
    pub fn reuse_on_scale_in(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reuse_on_scale_in", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAutoscalingGroup_WarmPool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_reuse_policy: std::option::Option<terrars::ListField<DataAutoscalingGroup_WarmPool_InstanceReusePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_group_prepared_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAutoscalingGroup_WarmPool {
    #[doc = "Set the field `instance_reuse_policy`.\n"]
    pub fn set_instance_reuse_policy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAutoscalingGroup_WarmPool_InstanceReusePolicy>>,
    ) -> Self {
        self.instance_reuse_policy = std::option::Option::Some(v.into());
        self
    }

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
}

impl terrars::ToListMappable for DataAutoscalingGroup_WarmPool {
    type O = terrars::BlockAssignable<DataAutoscalingGroup_WarmPool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroup_WarmPool {}

impl BuildDataAutoscalingGroup_WarmPool {
    pub fn build(self) -> DataAutoscalingGroup_WarmPool {
        DataAutoscalingGroup_WarmPool {
            instance_reuse_policy: core::default::Default::default(),
            max_group_prepared_capacity: core::default::Default::default(),
            min_size: core::default::Default::default(),
            pool_state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAutoscalingGroup_WarmPoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAutoscalingGroup_WarmPoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAutoscalingGroup_WarmPoolRef {
        DataAutoscalingGroup_WarmPoolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroup_WarmPoolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_reuse_policy` after provisioning.\n"]
    pub fn instance_reuse_policy(&self) -> terrars::ListRef<DataAutoscalingGroup_WarmPool_InstanceReusePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_reuse_policy", self.base))
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
}
