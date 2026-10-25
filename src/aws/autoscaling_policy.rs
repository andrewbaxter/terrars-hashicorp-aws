#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AutoscalingPolicyData_ {
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
    adjustment_type: std::option::Option<terrars::PrimField<std::string::String>>,
    autoscaling_group_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cooldown: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    estimated_instance_warmup: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_aggregation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_adjustment_magnitude: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_adjustment: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predictive_scaling_configuration: std
    ::option
    ::Option<std::vec::Vec<AutoscalingPolicy_PredictiveScalingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step_adjustment: std::option::Option<std::vec::Vec<AutoscalingPolicy_StepAdjustment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_tracking_configuration: std::option::Option<std::vec::Vec<AutoscalingPolicy_TargetTrackingConfiguration>>,
    dynamic: AutoscalingPolicyDynamic,
}

struct AutoscalingPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AutoscalingPolicyData_>,
}

#[derive(Clone)]
pub struct AutoscalingPolicy(std::rc::Rc<AutoscalingPolicy_>);

impl AutoscalingPolicy {
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

    #[doc = "Set the field `adjustment_type`.\n"]
    pub fn set_adjustment_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().adjustment_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cooldown`.\n"]
    pub fn set_cooldown(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().cooldown = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `estimated_instance_warmup`.\n"]
    pub fn set_estimated_instance_warmup(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().estimated_instance_warmup = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric_aggregation_type`.\n"]
    pub fn set_metric_aggregation_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().metric_aggregation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_adjustment_magnitude`.\n"]
    pub fn set_min_adjustment_magnitude(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_adjustment_magnitude = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_type`.\n"]
    pub fn set_policy_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().policy_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scaling_adjustment`.\n"]
    pub fn set_scaling_adjustment(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().scaling_adjustment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `predictive_scaling_configuration`.\n"]
    pub fn set_predictive_scaling_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingPolicy_PredictiveScalingConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().predictive_scaling_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.predictive_scaling_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `step_adjustment`.\n"]
    pub fn set_step_adjustment(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingPolicy_StepAdjustment>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().step_adjustment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.step_adjustment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_tracking_configuration`.\n"]
    pub fn set_target_tracking_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingPolicy_TargetTrackingConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_tracking_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_tracking_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `adjustment_type` after provisioning.\n"]
    pub fn adjustment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.adjustment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling_group_name` after provisioning.\n"]
    pub fn autoscaling_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autoscaling_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cooldown` after provisioning.\n"]
    pub fn cooldown(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cooldown", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `estimated_instance_warmup` after provisioning.\n"]
    pub fn estimated_instance_warmup(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.estimated_instance_warmup", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_aggregation_type` after provisioning.\n"]
    pub fn metric_aggregation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_aggregation_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_adjustment_magnitude` after provisioning.\n"]
    pub fn min_adjustment_magnitude(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_adjustment_magnitude", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_type` after provisioning.\n"]
    pub fn policy_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_adjustment` after provisioning.\n"]
    pub fn scaling_adjustment(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scaling_adjustment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `predictive_scaling_configuration` after provisioning.\n"]
    pub fn predictive_scaling_configuration(
        &self,
    ) -> terrars::ListRef<AutoscalingPolicy_PredictiveScalingConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.predictive_scaling_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target_tracking_configuration` after provisioning.\n"]
    pub fn target_tracking_configuration(&self) -> terrars::ListRef<AutoscalingPolicy_TargetTrackingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_tracking_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for AutoscalingPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AutoscalingPolicy { }

impl terrars::ToListMappable for AutoscalingPolicy {
    type O = terrars::ListRef<AutoscalingPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AutoscalingPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_autoscaling_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAutoscalingPolicy {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub autoscaling_group_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> AutoscalingPolicy {
        let out = AutoscalingPolicy(std::rc::Rc::new(AutoscalingPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AutoscalingPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                adjustment_type: core::default::Default::default(),
                autoscaling_group_name: self.autoscaling_group_name,
                cooldown: core::default::Default::default(),
                enabled: core::default::Default::default(),
                estimated_instance_warmup: core::default::Default::default(),
                id: core::default::Default::default(),
                metric_aggregation_type: core::default::Default::default(),
                min_adjustment_magnitude: core::default::Default::default(),
                name: self.name,
                policy_type: core::default::Default::default(),
                region: core::default::Default::default(),
                scaling_adjustment: core::default::Default::default(),
                predictive_scaling_configuration: core::default::Default::default(),
                step_adjustment: core::default::Default::default(),
                target_tracking_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AutoscalingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AutoscalingPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `adjustment_type` after provisioning.\n"]
    pub fn adjustment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.adjustment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling_group_name` after provisioning.\n"]
    pub fn autoscaling_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autoscaling_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cooldown` after provisioning.\n"]
    pub fn cooldown(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cooldown", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `estimated_instance_warmup` after provisioning.\n"]
    pub fn estimated_instance_warmup(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.estimated_instance_warmup", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_aggregation_type` after provisioning.\n"]
    pub fn metric_aggregation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_aggregation_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_adjustment_magnitude` after provisioning.\n"]
    pub fn min_adjustment_magnitude(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_adjustment_magnitude", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_type` after provisioning.\n"]
    pub fn policy_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_adjustment` after provisioning.\n"]
    pub fn scaling_adjustment(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scaling_adjustment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `predictive_scaling_configuration` after provisioning.\n"]
    pub fn predictive_scaling_configuration(
        &self,
    ) -> terrars::ListRef<AutoscalingPolicy_PredictiveScalingConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.predictive_scaling_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target_tracking_configuration` after provisioning.\n"]
    pub fn target_tracking_configuration(&self) -> terrars::ListRef<AutoscalingPolicy_TargetTrackingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_tracking_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingPolicyDynamic {
    predictive_scaling_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingPolicy_PredictiveScalingConfiguration>>,
    step_adjustment: std::option::Option<terrars::DynamicBlock<AutoscalingPolicy_StepAdjustment>>,
    target_tracking_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingPolicy_TargetTrackingConfiguration>>,
}

#[derive(Serialize)]
pub struct AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric_Dimensions {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric_Dimensions {

}

impl terrars
::ToListMappable for AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric_Dimensions {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric_Dimensions,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric_Dimensions {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric_Dimensions {
    pub fn build(
        self,
    ) -> AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric_Dimensions {
        AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric_Dimensions {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric_DimensionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric_DimensionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric_DimensionsRef {
        AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric_DimensionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric_DimensionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_MetricDynamic {
    dimensions: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric_Dimensions,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric {
    metric_name: terrars::PrimField<std::string::String>,
    namespace: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric_Dimensions,
        >,
    >,
    dynamic: AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_MetricDynamic,
}

impl AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric {
    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric_Dimensions,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dimensions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dimensions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric {
    #[doc = ""]
    pub metric_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub namespace: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric {
    pub fn build(
        self,
    ) -> AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric {
        AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric {
            metric_name: self.metric_name,
            namespace: self.namespace,
            dimensions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_MetricRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_MetricRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_MetricRef {
        AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_MetricRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_MetricRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries_MetricStatDynamic {
    metric: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries_MetricStat {
    stat: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric>,
    >,
    dynamic: AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries_MetricStatDynamic,
}

impl AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries_MetricStat {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric`.\n"]
    pub fn set_metric(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries_MetricStat {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries_MetricStat,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries_MetricStat {
    #[doc = ""]
    pub stat: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries_MetricStat {
    pub fn build(
        self,
    ) -> AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries_MetricStat {
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries_MetricStat {
            stat: self.stat,
            unit: core::default::Default::default(),
            metric: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries_MetricStatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries_MetricStatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries_MetricStatRef {
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries_MetricStatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries_MetricStatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stat` after provisioning.\n"]
    pub fn stat(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stat", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `metric` after provisioning.\n"]
    pub fn metric(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_MetricRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueriesDynamic {
    metric_stat: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries_MetricStat,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_data: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_stat: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries_MetricStat,
        >,
    >,
    dynamic: AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueriesDynamic,
}

impl AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `label`.\n"]
    pub fn set_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `return_data`.\n"]
    pub fn set_return_data(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.return_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric_stat`.\n"]
    pub fn set_metric_stat(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries_MetricStat,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric_stat = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_stat = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries {
    pub fn build(
        self,
    ) -> AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries {
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries {
            expression: core::default::Default::default(),
            id: self.id,
            label: core::default::Default::default(),
            return_data: core::default::Default::default(),
            metric_stat: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueriesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueriesRef {
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc = "Get a reference to the value of field `return_data` after provisioning.\n"]
    pub fn return_data(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return_data", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_stat` after provisioning.\n"]
    pub fn metric_stat(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries_MetricStatRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_stat", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedCapacityMetricSpecificationDynamic {
    metric_data_queries: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_data_queries: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries,
        >,
    >,
    dynamic: AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedCapacityMetricSpecificationDynamic,
}

impl AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification {
    #[doc = "Set the field `metric_data_queries`.\n"]
    pub fn set_metric_data_queries(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric_data_queries = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_data_queries = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification {}

impl BuildAutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification {
    pub fn build(
        self,
    ) -> AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification {
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification {
            metric_data_queries: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedCapacityMetricSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedCapacityMetricSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedCapacityMetricSpecificationRef {
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedCapacityMetricSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedCapacityMetricSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_data_queries` after provisioning.\n"]
    pub fn metric_data_queries(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueriesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_data_queries", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecificationDynamic {
    metric_data_queries: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_data_queries: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries,
        >,
    >,
    dynamic: AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecificationDynamic,
}

impl AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification {
    #[doc = "Set the field `metric_data_queries`.\n"]
    pub fn set_metric_data_queries(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric_data_queries = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_data_queries = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification {}

impl BuildAutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification {
    pub fn build(
        self,
    ) -> AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification {
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification {
            metric_data_queries: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecificationRef {
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_data_queries` after provisioning.\n"]
    pub fn metric_data_queries(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueriesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_data_queries", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedScalingMetricSpecificationDynamic {
    metric_data_queries: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedScalingMetricSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_data_queries: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries,
        >,
    >,
    dynamic: AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedScalingMetricSpecificationDynamic,
}

impl AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedScalingMetricSpecification {
    #[doc = "Set the field `metric_data_queries`.\n"]
    pub fn set_metric_data_queries(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueries,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric_data_queries = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_data_queries = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedScalingMetricSpecification {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedScalingMetricSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedScalingMetricSpecification {}

impl BuildAutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedScalingMetricSpecification {
    pub fn build(
        self,
    ) -> AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedScalingMetricSpecification {
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedScalingMetricSpecification {
            metric_data_queries: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedScalingMetricSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedScalingMetricSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedScalingMetricSpecificationRef {
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedScalingMetricSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedScalingMetricSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_data_queries` after provisioning.\n"]
    pub fn metric_data_queries(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueriesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_data_queries", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedLoadMetricSpecification {
    predefined_metric_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_label: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedLoadMetricSpecification {
    #[doc = "Set the field `resource_label`.\n"]
    pub fn set_resource_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_label = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedLoadMetricSpecification {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedLoadMetricSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedLoadMetricSpecification {
    #[doc = ""]
    pub predefined_metric_type: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedLoadMetricSpecification {
    pub fn build(
        self,
    ) -> AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedLoadMetricSpecification {
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedLoadMetricSpecification {
            predefined_metric_type: self.predefined_metric_type,
            resource_label: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedLoadMetricSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedLoadMetricSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedLoadMetricSpecificationRef {
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedLoadMetricSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedLoadMetricSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `predefined_metric_type` after provisioning.\n"]
    pub fn predefined_metric_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.predefined_metric_type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_label` after provisioning.\n"]
    pub fn resource_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_label", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedMetricPairSpecification {
    predefined_metric_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_label: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedMetricPairSpecification {
    #[doc = "Set the field `resource_label`.\n"]
    pub fn set_resource_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_label = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedMetricPairSpecification {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedMetricPairSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedMetricPairSpecification {
    #[doc = ""]
    pub predefined_metric_type: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedMetricPairSpecification {
    pub fn build(
        self,
    ) -> AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedMetricPairSpecification {
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedMetricPairSpecification {
            predefined_metric_type: self.predefined_metric_type,
            resource_label: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedMetricPairSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedMetricPairSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedMetricPairSpecificationRef {
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedMetricPairSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedMetricPairSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `predefined_metric_type` after provisioning.\n"]
    pub fn predefined_metric_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.predefined_metric_type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_label` after provisioning.\n"]
    pub fn resource_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_label", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedScalingMetricSpecification {
    predefined_metric_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_label: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedScalingMetricSpecification {
    #[doc = "Set the field `resource_label`.\n"]
    pub fn set_resource_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_label = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedScalingMetricSpecification {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedScalingMetricSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedScalingMetricSpecification {
    #[doc = ""]
    pub predefined_metric_type: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedScalingMetricSpecification {
    pub fn build(
        self,
    ) -> AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedScalingMetricSpecification {
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedScalingMetricSpecification {
            predefined_metric_type: self.predefined_metric_type,
            resource_label: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedScalingMetricSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedScalingMetricSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedScalingMetricSpecificationRef {
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedScalingMetricSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedScalingMetricSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `predefined_metric_type` after provisioning.\n"]
    pub fn predefined_metric_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.predefined_metric_type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_label` after provisioning.\n"]
    pub fn resource_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_label", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecificationDynamic {
    customized_capacity_metric_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification,
        >,
    >,
    customized_load_metric_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification,
        >,
    >,
    customized_scaling_metric_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedScalingMetricSpecification,
        >,
    >,
    predefined_load_metric_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedLoadMetricSpecification,
        >,
    >,
    predefined_metric_pair_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedMetricPairSpecification,
        >,
    >,
    predefined_scaling_metric_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedScalingMetricSpecification,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification {
    target_value: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customized_capacity_metric_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customized_load_metric_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customized_scaling_metric_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedScalingMetricSpecification,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predefined_load_metric_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedLoadMetricSpecification>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predefined_metric_pair_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedMetricPairSpecification>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predefined_scaling_metric_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedScalingMetricSpecification,
        >,
    >,
    dynamic: AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecificationDynamic,
}

impl AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification {
    #[doc = "Set the field `customized_capacity_metric_specification`.\n"]
    pub fn set_customized_capacity_metric_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.customized_capacity_metric_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.customized_capacity_metric_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `customized_load_metric_specification`.\n"]
    pub fn set_customized_load_metric_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.customized_load_metric_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.customized_load_metric_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `customized_scaling_metric_specification`.\n"]
    pub fn set_customized_scaling_metric_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedScalingMetricSpecification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.customized_scaling_metric_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.customized_scaling_metric_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `predefined_load_metric_specification`.\n"]
    pub fn set_predefined_load_metric_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedLoadMetricSpecification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.predefined_load_metric_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.predefined_load_metric_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `predefined_metric_pair_specification`.\n"]
    pub fn set_predefined_metric_pair_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedMetricPairSpecification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.predefined_metric_pair_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.predefined_metric_pair_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `predefined_scaling_metric_specification`.\n"]
    pub fn set_predefined_scaling_metric_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedScalingMetricSpecification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.predefined_scaling_metric_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.predefined_scaling_metric_specification = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification {
    type O = terrars::BlockAssignable<AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification {
    #[doc = ""]
    pub target_value: terrars::PrimField<f64>,
}

impl BuildAutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification {
    pub fn build(self) -> AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification {
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification {
            target_value: self.target_value,
            customized_capacity_metric_specification: core::default::Default::default(),
            customized_load_metric_specification: core::default::Default::default(),
            customized_scaling_metric_specification: core::default::Default::default(),
            predefined_load_metric_specification: core::default::Default::default(),
            predefined_metric_pair_specification: core::default::Default::default(),
            predefined_scaling_metric_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecificationRef {
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_value` after provisioning.\n"]
    pub fn target_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_value", self.base))
    }

    #[doc = "Get a reference to the value of field `customized_capacity_metric_specification` after provisioning.\n"]
    pub fn customized_capacity_metric_specification(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedCapacityMetricSpecificationRef,
    > {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.customized_capacity_metric_specification", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `customized_load_metric_specification` after provisioning.\n"]
    pub fn customized_load_metric_specification(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedLoadMetricSpecificationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customized_load_metric_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `customized_scaling_metric_specification` after provisioning.\n"]
    pub fn customized_scaling_metric_specification(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_CustomizedScalingMetricSpecificationRef,
    > {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.customized_scaling_metric_specification", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `predefined_load_metric_specification` after provisioning.\n"]
    pub fn predefined_load_metric_specification(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedLoadMetricSpecificationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predefined_load_metric_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `predefined_metric_pair_specification` after provisioning.\n"]
    pub fn predefined_metric_pair_specification(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedMetricPairSpecificationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predefined_metric_pair_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `predefined_scaling_metric_specification` after provisioning.\n"]
    pub fn predefined_scaling_metric_specification(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification_PredefinedScalingMetricSpecificationRef,
    > {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.predefined_scaling_metric_specification", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AutoscalingPolicy_PredictiveScalingConfigurationDynamic {
    metric_specification: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification>>,
}

#[derive(Serialize)]
pub struct AutoscalingPolicy_PredictiveScalingConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_capacity_breach_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_capacity_buffer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduling_buffer_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_specification: std
    ::option
    ::Option<std::vec::Vec<AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification>>,
    dynamic: AutoscalingPolicy_PredictiveScalingConfigurationDynamic,
}

impl AutoscalingPolicy_PredictiveScalingConfiguration {
    #[doc = "Set the field `max_capacity_breach_behavior`.\n"]
    pub fn set_max_capacity_breach_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.max_capacity_breach_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_capacity_buffer`.\n"]
    pub fn set_max_capacity_buffer(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.max_capacity_buffer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scheduling_buffer_time`.\n"]
    pub fn set_scheduling_buffer_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.scheduling_buffer_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric_specification`.\n"]
    pub fn set_metric_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecification>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_specification = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AutoscalingPolicy_PredictiveScalingConfiguration {
    type O = terrars::BlockAssignable<AutoscalingPolicy_PredictiveScalingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicy_PredictiveScalingConfiguration {}

impl BuildAutoscalingPolicy_PredictiveScalingConfiguration {
    pub fn build(self) -> AutoscalingPolicy_PredictiveScalingConfiguration {
        AutoscalingPolicy_PredictiveScalingConfiguration {
            max_capacity_breach_behavior: core::default::Default::default(),
            max_capacity_buffer: core::default::Default::default(),
            mode: core::default::Default::default(),
            scheduling_buffer_time: core::default::Default::default(),
            metric_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingPolicy_PredictiveScalingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingPolicy_PredictiveScalingConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicy_PredictiveScalingConfigurationRef {
        AutoscalingPolicy_PredictiveScalingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicy_PredictiveScalingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_capacity_breach_behavior` after provisioning.\n"]
    pub fn max_capacity_breach_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_capacity_breach_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `max_capacity_buffer` after provisioning.\n"]
    pub fn max_capacity_buffer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_capacity_buffer", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduling_buffer_time` after provisioning.\n"]
    pub fn scheduling_buffer_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduling_buffer_time", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_specification` after provisioning.\n"]
    pub fn metric_specification(
        &self,
    ) -> terrars::ListRef<AutoscalingPolicy_PredictiveScalingConfiguration_MetricSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_specification", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingPolicy_StepAdjustment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_interval_lower_bound: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_interval_upper_bound: std::option::Option<terrars::PrimField<std::string::String>>,
    scaling_adjustment: terrars::PrimField<f64>,
}

impl AutoscalingPolicy_StepAdjustment {
    #[doc = "Set the field `metric_interval_lower_bound`.\n"]
    pub fn set_metric_interval_lower_bound(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.metric_interval_lower_bound = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric_interval_upper_bound`.\n"]
    pub fn set_metric_interval_upper_bound(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.metric_interval_upper_bound = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AutoscalingPolicy_StepAdjustment {
    type O = terrars::BlockAssignable<AutoscalingPolicy_StepAdjustment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicy_StepAdjustment {
    #[doc = ""]
    pub scaling_adjustment: terrars::PrimField<f64>,
}

impl BuildAutoscalingPolicy_StepAdjustment {
    pub fn build(self) -> AutoscalingPolicy_StepAdjustment {
        AutoscalingPolicy_StepAdjustment {
            metric_interval_lower_bound: core::default::Default::default(),
            metric_interval_upper_bound: core::default::Default::default(),
            scaling_adjustment: self.scaling_adjustment,
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingPolicy_StepAdjustmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingPolicy_StepAdjustmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AutoscalingPolicy_StepAdjustmentRef {
        AutoscalingPolicy_StepAdjustmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicy_StepAdjustmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_interval_lower_bound` after provisioning.\n"]
    pub fn metric_interval_lower_bound(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_interval_lower_bound", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_interval_upper_bound` after provisioning.\n"]
    pub fn metric_interval_upper_bound(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_interval_upper_bound", self.base))
    }

    #[doc = "Get a reference to the value of field `scaling_adjustment` after provisioning.\n"]
    pub fn scaling_adjustment(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scaling_adjustment", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricDimension {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricDimension { }

impl terrars
::ToListMappable for AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricDimension {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricDimension,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricDimension {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricDimension {
    pub fn build(
        self,
    ) -> AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricDimension {
        AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricDimension {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricDimensionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricDimensionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricDimensionRef {
        AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricDimensionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricDimensionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStatDynamic {
    metric: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period: std::option::Option<terrars::PrimField<f64>>,
    stat: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric>,
    >,
    dynamic: AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStatDynamic,
}

impl AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat {
    #[doc = "Set the field `period`.\n"]
    pub fn set_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric`.\n"]
    pub fn set_metric(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat {
    #[doc = ""]
    pub stat: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat {
    pub fn build(
        self,
    ) -> AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat {
        AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat {
            period: core::default::Default::default(),
            stat: self.stat,
            unit: core::default::Default::default(),
            metric: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStatRef {
        AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }

    #[doc = "Get a reference to the value of field `stat` after provisioning.\n"]
    pub fn stat(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stat", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `metric` after provisioning.\n"]
    pub fn metric(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_MetricRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricsDynamic {
    metric_stat: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat>,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_data: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_stat: std
    ::option
    ::Option<
        std::vec::Vec<AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat>,
    >,
    dynamic: AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricsDynamic,
}

impl AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `label`.\n"]
    pub fn set_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `return_data`.\n"]
    pub fn set_return_data(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.return_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric_stat`.\n"]
    pub fn set_metric_stat(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStat,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric_stat = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_stat = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics {
    type O =
        terrars
        ::BlockAssignable<AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics {
    pub fn build(self) -> AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics {
        AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics {
            expression: core::default::Default::default(),
            id: self.id,
            label: core::default::Default::default(),
            return_data: core::default::Default::default(),
            metric_stat: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricsRef {
        AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc = "Get a reference to the value of field `return_data` after provisioning.\n"]
    pub fn return_data(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return_data", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_stat` after provisioning.\n"]
    pub fn metric_stat(
        &self,
    ) -> terrars
    ::ListRef<AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics_MetricStatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_stat", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecificationDynamic {
    metric_dimension: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricDimension>,
    >,
    metrics: std
    ::option
    ::Option<
        terrars::DynamicBlock<AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics>,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statistic: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_dimension: std
    ::option
    ::Option<
        std::vec::Vec<AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricDimension>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metrics: std
    ::option
    ::Option<std::vec::Vec<AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics>>,
    dynamic: AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecificationDynamic,
}

impl AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification {
    #[doc = "Set the field `metric_name`.\n"]
    pub fn set_metric_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metric_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `period`.\n"]
    pub fn set_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `statistic`.\n"]
    pub fn set_statistic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.statistic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric_dimension`.\n"]
    pub fn set_metric_dimension(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricDimension,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric_dimension = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_dimension = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metrics`.\n"]
    pub fn set_metrics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_Metrics,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metrics = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification {
    type O = terrars::BlockAssignable<AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification {}

impl BuildAutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification {
    pub fn build(self) -> AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification {
        AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification {
            metric_name: core::default::Default::default(),
            namespace: core::default::Default::default(),
            period: core::default::Default::default(),
            statistic: core::default::Default::default(),
            unit: core::default::Default::default(),
            metric_dimension: core::default::Default::default(),
            metrics: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecificationRef {
        AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }

    #[doc = "Get a reference to the value of field `statistic` after provisioning.\n"]
    pub fn statistic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statistic", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_dimension` after provisioning.\n"]
    pub fn metric_dimension(
        &self,
    ) -> terrars
    ::ListRef<AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification_MetricDimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_dimension", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingPolicy_TargetTrackingConfiguration_PredefinedMetricSpecification {
    predefined_metric_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_label: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AutoscalingPolicy_TargetTrackingConfiguration_PredefinedMetricSpecification {
    #[doc = "Set the field `resource_label`.\n"]
    pub fn set_resource_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_label = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AutoscalingPolicy_TargetTrackingConfiguration_PredefinedMetricSpecification {
    type O = terrars::BlockAssignable<AutoscalingPolicy_TargetTrackingConfiguration_PredefinedMetricSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicy_TargetTrackingConfiguration_PredefinedMetricSpecification {
    #[doc = ""]
    pub predefined_metric_type: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicy_TargetTrackingConfiguration_PredefinedMetricSpecification {
    pub fn build(self) -> AutoscalingPolicy_TargetTrackingConfiguration_PredefinedMetricSpecification {
        AutoscalingPolicy_TargetTrackingConfiguration_PredefinedMetricSpecification {
            predefined_metric_type: self.predefined_metric_type,
            resource_label: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingPolicy_TargetTrackingConfiguration_PredefinedMetricSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingPolicy_TargetTrackingConfiguration_PredefinedMetricSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicy_TargetTrackingConfiguration_PredefinedMetricSpecificationRef {
        AutoscalingPolicy_TargetTrackingConfiguration_PredefinedMetricSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicy_TargetTrackingConfiguration_PredefinedMetricSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `predefined_metric_type` after provisioning.\n"]
    pub fn predefined_metric_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.predefined_metric_type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_label` after provisioning.\n"]
    pub fn resource_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_label", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingPolicy_TargetTrackingConfigurationDynamic {
    customized_metric_specification: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification>>,
    predefined_metric_specification: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingPolicy_TargetTrackingConfiguration_PredefinedMetricSpecification>>,
}

#[derive(Serialize)]
pub struct AutoscalingPolicy_TargetTrackingConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_scale_in: std::option::Option<terrars::PrimField<bool>>,
    target_value: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customized_metric_specification: std
    ::option
    ::Option<std::vec::Vec<AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predefined_metric_specification: std
    ::option
    ::Option<std::vec::Vec<AutoscalingPolicy_TargetTrackingConfiguration_PredefinedMetricSpecification>>,
    dynamic: AutoscalingPolicy_TargetTrackingConfigurationDynamic,
}

impl AutoscalingPolicy_TargetTrackingConfiguration {
    #[doc = "Set the field `disable_scale_in`.\n"]
    pub fn set_disable_scale_in(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_scale_in = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customized_metric_specification`.\n"]
    pub fn set_customized_metric_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecification>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.customized_metric_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.customized_metric_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `predefined_metric_specification`.\n"]
    pub fn set_predefined_metric_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AutoscalingPolicy_TargetTrackingConfiguration_PredefinedMetricSpecification>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.predefined_metric_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.predefined_metric_specification = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AutoscalingPolicy_TargetTrackingConfiguration {
    type O = terrars::BlockAssignable<AutoscalingPolicy_TargetTrackingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicy_TargetTrackingConfiguration {
    #[doc = ""]
    pub target_value: terrars::PrimField<f64>,
}

impl BuildAutoscalingPolicy_TargetTrackingConfiguration {
    pub fn build(self) -> AutoscalingPolicy_TargetTrackingConfiguration {
        AutoscalingPolicy_TargetTrackingConfiguration {
            disable_scale_in: core::default::Default::default(),
            target_value: self.target_value,
            customized_metric_specification: core::default::Default::default(),
            predefined_metric_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingPolicy_TargetTrackingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingPolicy_TargetTrackingConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicy_TargetTrackingConfigurationRef {
        AutoscalingPolicy_TargetTrackingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicy_TargetTrackingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disable_scale_in` after provisioning.\n"]
    pub fn disable_scale_in(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_scale_in", self.base))
    }

    #[doc = "Get a reference to the value of field `target_value` after provisioning.\n"]
    pub fn target_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_value", self.base))
    }

    #[doc = "Get a reference to the value of field `customized_metric_specification` after provisioning.\n"]
    pub fn customized_metric_specification(
        &self,
    ) -> terrars::ListRef<AutoscalingPolicy_TargetTrackingConfiguration_CustomizedMetricSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customized_metric_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `predefined_metric_specification` after provisioning.\n"]
    pub fn predefined_metric_specification(
        &self,
    ) -> terrars::ListRef<AutoscalingPolicy_TargetTrackingConfiguration_PredefinedMetricSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predefined_metric_specification", self.base))
    }
}
