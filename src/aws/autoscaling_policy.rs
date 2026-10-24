use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AutoscalingPolicyData {
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
    scaling_adjustment: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predictive_scaling_configuration: std
    ::option
    ::Option<std::vec::Vec<AutoscalingPolicyPredictiveScalingConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step_adjustment: std::option::Option<std::vec::Vec<AutoscalingPolicyStepAdjustmentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_tracking_configuration: std::option::Option<std::vec::Vec<AutoscalingPolicyTargetTrackingConfigurationEl>>,
    dynamic: AutoscalingPolicyDynamic,
}

struct AutoscalingPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AutoscalingPolicyData>,
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

    #[doc = "Set the field `scaling_adjustment`.\n"]
    pub fn set_scaling_adjustment(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().scaling_adjustment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `predictive_scaling_configuration`.\n"]
    pub fn set_predictive_scaling_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingPolicyPredictiveScalingConfigurationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingPolicyStepAdjustmentEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingPolicyTargetTrackingConfigurationEl>>,
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

    #[doc = "Get a reference to the value of field `scaling_adjustment` after provisioning.\n"]
    pub fn scaling_adjustment(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scaling_adjustment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `predictive_scaling_configuration` after provisioning.\n"]
    pub fn predictive_scaling_configuration(
        &self,
    ) -> terrars::ListRef<AutoscalingPolicyPredictiveScalingConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.predictive_scaling_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target_tracking_configuration` after provisioning.\n"]
    pub fn target_tracking_configuration(&self) -> terrars::ListRef<AutoscalingPolicyTargetTrackingConfigurationElRef> {
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
            data: std::cell::RefCell::new(AutoscalingPolicyData {
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

    #[doc = "Get a reference to the value of field `scaling_adjustment` after provisioning.\n"]
    pub fn scaling_adjustment(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scaling_adjustment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `predictive_scaling_configuration` after provisioning.\n"]
    pub fn predictive_scaling_configuration(
        &self,
    ) -> terrars::ListRef<AutoscalingPolicyPredictiveScalingConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.predictive_scaling_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target_tracking_configuration` after provisioning.\n"]
    pub fn target_tracking_configuration(&self) -> terrars::ListRef<AutoscalingPolicyTargetTrackingConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_tracking_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl {

}

impl terrars
::ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef {
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
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDynamic {
    dimensions: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl {
    metric_name: terrars::PrimField<std::string::String>,
    namespace: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl,
        >,
    >,
    dynamic: AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDynamic,
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl {
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
                            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl,
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
::ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl {
    #[doc = ""]
    pub metric_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub namespace: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl {
            metric_name: self.metric_name,
            namespace: self.namespace,
            dimensions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef {
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
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElDynamic {
    metric: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl {
    stat: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl,
        >,
    >,
    dynamic: AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElDynamic,
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl {
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
                            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl,
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
::ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl {
    #[doc = ""]
    pub stat: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl {
            stat: self.stat,
            unit: core::default::Default::default(),
            metric: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElRef {
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
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElDynamic {
    metric_stat: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl {
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
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl,
        >,
    >,
    dynamic: AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElDynamic,
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl {
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
                            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl,
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
::ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl {
            expression: core::default::Default::default(),
            id: self.id,
            label: core::default::Default::default(),
            return_data: core::default::Default::default(),
            metric_stat: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElRef {
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
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_stat", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElDynamic {
    metric_data_queries: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_data_queries: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl,
        >,
    >,
    dynamic: AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElDynamic,
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl {
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
                            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl,
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
::ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl {}

impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl {
            metric_data_queries: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_data_queries` after provisioning.\n"]
    pub fn metric_data_queries(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_data_queries", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl {

}

impl terrars
::ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef {
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
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDynamic {
    dimensions: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl {
    metric_name: terrars::PrimField<std::string::String>,
    namespace: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl,
        >,
    >,
    dynamic: AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDynamic,
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl {
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
                            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl,
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
::ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl {
    #[doc = ""]
    pub metric_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub namespace: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl {
            metric_name: self.metric_name,
            namespace: self.namespace,
            dimensions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef {
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
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElDynamic {
    metric: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl {
    stat: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl,
        >,
    >,
    dynamic: AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElDynamic,
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl {
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
                            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl,
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
::ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl {
    #[doc = ""]
    pub stat: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl {
            stat: self.stat,
            unit: core::default::Default::default(),
            metric: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElRef {
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
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElDynamic {
    metric_stat: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl {
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
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl,
        >,
    >,
    dynamic: AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElDynamic,
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl {
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
                            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl,
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
::ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl {
            expression: core::default::Default::default(),
            id: self.id,
            label: core::default::Default::default(),
            return_data: core::default::Default::default(),
            metric_stat: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElRef {
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
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_stat", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElDynamic {
    metric_data_queries: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_data_queries: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl,
        >,
    >,
    dynamic: AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElDynamic,
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl {
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
                            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl,
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
::ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl {}

impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl {
            metric_data_queries: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_data_queries` after provisioning.\n"]
    pub fn metric_data_queries(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_data_queries", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl {

}

impl terrars
::ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef {
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
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDynamic {
    dimensions: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl {
    metric_name: terrars::PrimField<std::string::String>,
    namespace: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl,
        >,
    >,
    dynamic: AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDynamic,
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl {
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
                            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl,
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
::ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl {
    #[doc = ""]
    pub metric_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub namespace: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl {
            metric_name: self.metric_name,
            namespace: self.namespace,
            dimensions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef {
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
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElDynamic {
    metric: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl {
    stat: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl,
        >,
    >,
    dynamic: AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElDynamic,
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl {
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
                            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl,
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
::ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl {
    #[doc = ""]
    pub stat: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl {
            stat: self.stat,
            unit: core::default::Default::default(),
            metric: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElRef {
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
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElDynamic {
    metric_stat: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl {
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
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl,
        >,
    >,
    dynamic: AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElDynamic,
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl {
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
                            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl,
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
::ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl {
            expression: core::default::Default::default(),
            id: self.id,
            label: core::default::Default::default(),
            return_data: core::default::Default::default(),
            metric_stat: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElRef {
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
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_stat", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElDynamic {
    metric_data_queries: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_data_queries: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl,
        >,
    >,
    dynamic: AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElDynamic,
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl {
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
                            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl,
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
::ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl {}

impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl {
            metric_data_queries: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_data_queries` after provisioning.\n"]
    pub fn metric_data_queries(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_data_queries", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl {
    predefined_metric_type: terrars::PrimField<std::string::String>,
    resource_label: terrars::PrimField<std::string::String>,
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl { }

impl terrars
::ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl {
    #[doc = ""]
    pub predefined_metric_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub resource_label: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl {
            predefined_metric_type: self.predefined_metric_type,
            resource_label: self.resource_label,
        }
    }
}

pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationElRef {
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
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl {
    predefined_metric_type: terrars::PrimField<std::string::String>,
    resource_label: terrars::PrimField<std::string::String>,
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl { }

impl terrars
::ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl {
    #[doc = ""]
    pub predefined_metric_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub resource_label: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl {
            predefined_metric_type: self.predefined_metric_type,
            resource_label: self.resource_label,
        }
    }
}

pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationElRef {
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
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl {
    predefined_metric_type: terrars::PrimField<std::string::String>,
    resource_label: terrars::PrimField<std::string::String>,
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl { }

impl terrars
::ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl {
    #[doc = ""]
    pub predefined_metric_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub resource_label: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl {
            predefined_metric_type: self.predefined_metric_type,
            resource_label: self.resource_label,
        }
    }
}

pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationElRef {
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
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElDynamic {
    customized_capacity_metric_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl,
        >,
    >,
    customized_load_metric_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl,
        >,
    >,
    customized_scaling_metric_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl,
        >,
    >,
    predefined_load_metric_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl,
        >,
    >,
    predefined_metric_pair_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl,
        >,
    >,
    predefined_scaling_metric_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl {
    target_value: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customized_capacity_metric_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customized_load_metric_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customized_scaling_metric_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predefined_load_metric_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predefined_metric_pair_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predefined_scaling_metric_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl,
        >,
    >,
    dynamic: AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElDynamic,
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl {
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
                            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl,
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
                            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl,
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
                            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl,
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
                            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl,
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
                            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl,
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
                            AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl,
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

impl terrars::ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl {
    type O = terrars::BlockAssignable<AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl {
    #[doc = ""]
    pub target_value: terrars::PrimField<f64>,
}

impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl {
    pub fn build(self) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl {
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

pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElRef {
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
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElRef,
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
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customized_load_metric_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `customized_scaling_metric_specification` after provisioning.\n"]
    pub fn customized_scaling_metric_specification(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElRef,
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
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predefined_load_metric_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `predefined_metric_pair_specification` after provisioning.\n"]
    pub fn predefined_metric_pair_specification(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predefined_metric_pair_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `predefined_scaling_metric_specification` after provisioning.\n"]
    pub fn predefined_scaling_metric_specification(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationElRef,
    > {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.predefined_scaling_metric_specification", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AutoscalingPolicyPredictiveScalingConfigurationElDynamic {
    metric_specification: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl>>,
}

#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationEl {
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
    ::Option<std::vec::Vec<AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl>>,
    dynamic: AutoscalingPolicyPredictiveScalingConfigurationElDynamic,
}

impl AutoscalingPolicyPredictiveScalingConfigurationEl {
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
                        terrars
                        ::BlockAssignable<AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl>,
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

impl terrars::ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationEl {
    type O = terrars::BlockAssignable<AutoscalingPolicyPredictiveScalingConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationEl {}

impl BuildAutoscalingPolicyPredictiveScalingConfigurationEl {
    pub fn build(self) -> AutoscalingPolicyPredictiveScalingConfigurationEl {
        AutoscalingPolicyPredictiveScalingConfigurationEl {
            max_capacity_breach_behavior: core::default::Default::default(),
            max_capacity_buffer: core::default::Default::default(),
            mode: core::default::Default::default(),
            scheduling_buffer_time: core::default::Default::default(),
            metric_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingPolicyPredictiveScalingConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingPolicyPredictiveScalingConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyPredictiveScalingConfigurationElRef {
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
    ) -> terrars::ListRef<AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_specification", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingPolicyStepAdjustmentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_interval_lower_bound: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_interval_upper_bound: std::option::Option<terrars::PrimField<std::string::String>>,
    scaling_adjustment: terrars::PrimField<f64>,
}

impl AutoscalingPolicyStepAdjustmentEl {
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

impl terrars::ToListMappable for AutoscalingPolicyStepAdjustmentEl {
    type O = terrars::BlockAssignable<AutoscalingPolicyStepAdjustmentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyStepAdjustmentEl {
    #[doc = ""]
    pub scaling_adjustment: terrars::PrimField<f64>,
}

impl BuildAutoscalingPolicyStepAdjustmentEl {
    pub fn build(self) -> AutoscalingPolicyStepAdjustmentEl {
        AutoscalingPolicyStepAdjustmentEl {
            metric_interval_lower_bound: core::default::Default::default(),
            metric_interval_upper_bound: core::default::Default::default(),
            scaling_adjustment: self.scaling_adjustment,
        }
    }
}

pub struct AutoscalingPolicyStepAdjustmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingPolicyStepAdjustmentElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AutoscalingPolicyStepAdjustmentElRef {
        AutoscalingPolicyStepAdjustmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyStepAdjustmentElRef {
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
pub struct AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl { }

impl terrars
::ToListMappable for AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl {
        AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionElRef {
        AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionElRef {
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
struct AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElDynamic {
    metric_dimension: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl>,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl {
    metric_name: terrars::PrimField<std::string::String>,
    namespace: terrars::PrimField<std::string::String>,
    statistic: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_dimension: std
    ::option
    ::Option<
        std::vec::Vec<AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl>,
    >,
    dynamic: AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElDynamic,
}

impl AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl {
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
                            AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl,
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
}

impl terrars::ToListMappable for AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl {
    type O = terrars::BlockAssignable<AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl {
    #[doc = ""]
    pub metric_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub namespace: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub statistic: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl {
    pub fn build(self) -> AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl {
        AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl {
            metric_name: self.metric_name,
            namespace: self.namespace,
            statistic: self.statistic,
            unit: core::default::Default::default(),
            metric_dimension: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElRef {
        AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElRef {
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
    ::ListRef<AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_dimension", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl {
    predefined_metric_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_label: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl {
    #[doc = "Set the field `resource_label`.\n"]
    pub fn set_resource_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_label = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl {
    type O = terrars::BlockAssignable<AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl {
    #[doc = ""]
    pub predefined_metric_type: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl {
    pub fn build(self) -> AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl {
        AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl {
            predefined_metric_type: self.predefined_metric_type,
            resource_label: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationElRef {
        AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationElRef {
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
struct AutoscalingPolicyTargetTrackingConfigurationElDynamic {
    customized_metric_specification: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl>>,
    predefined_metric_specification: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl>>,
}

#[derive(Serialize)]
pub struct AutoscalingPolicyTargetTrackingConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_scale_in: std::option::Option<terrars::PrimField<bool>>,
    target_value: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customized_metric_specification: std
    ::option
    ::Option<std::vec::Vec<AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predefined_metric_specification: std
    ::option
    ::Option<std::vec::Vec<AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl>>,
    dynamic: AutoscalingPolicyTargetTrackingConfigurationElDynamic,
}

impl AutoscalingPolicyTargetTrackingConfigurationEl {
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
                        ::BlockAssignable<
                            AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl,
                        >,
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
                        ::BlockAssignable<
                            AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl,
                        >,
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

impl terrars::ToListMappable for AutoscalingPolicyTargetTrackingConfigurationEl {
    type O = terrars::BlockAssignable<AutoscalingPolicyTargetTrackingConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingPolicyTargetTrackingConfigurationEl {
    #[doc = ""]
    pub target_value: terrars::PrimField<f64>,
}

impl BuildAutoscalingPolicyTargetTrackingConfigurationEl {
    pub fn build(self) -> AutoscalingPolicyTargetTrackingConfigurationEl {
        AutoscalingPolicyTargetTrackingConfigurationEl {
            disable_scale_in: core::default::Default::default(),
            target_value: self.target_value,
            customized_metric_specification: core::default::Default::default(),
            predefined_metric_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingPolicyTargetTrackingConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingPolicyTargetTrackingConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingPolicyTargetTrackingConfigurationElRef {
        AutoscalingPolicyTargetTrackingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingPolicyTargetTrackingConfigurationElRef {
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
    ) -> terrars::ListRef<AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customized_metric_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `predefined_metric_specification` after provisioning.\n"]
    pub fn predefined_metric_specification(
        &self,
    ) -> terrars::ListRef<AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predefined_metric_specification", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingPolicyDynamic {
    predictive_scaling_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingPolicyPredictiveScalingConfigurationEl>>,
    step_adjustment: std::option::Option<terrars::DynamicBlock<AutoscalingPolicyStepAdjustmentEl>>,
    target_tracking_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingPolicyTargetTrackingConfigurationEl>>,
}
