use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppautoscalingPolicyData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_type: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_id: terrars::PrimField<std::string::String>,
    scalable_dimension: terrars::PrimField<std::string::String>,
    service_namespace: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step_scaling_policy_configuration: std
    ::option
    ::Option<std::vec::Vec<AppautoscalingPolicyStepScalingPolicyConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_tracking_scaling_policy_configuration: std
    ::option
    ::Option<std::vec::Vec<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl>>,
    dynamic: AppautoscalingPolicyDynamic,
}

struct AppautoscalingPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppautoscalingPolicyData>,
}

#[derive(Clone)]
pub struct AppautoscalingPolicy(std::rc::Rc<AppautoscalingPolicy_>);

impl AppautoscalingPolicy {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_type`.\n"]
    pub fn set_policy_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().policy_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `step_scaling_policy_configuration`.\n"]
    pub fn set_step_scaling_policy_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppautoscalingPolicyStepScalingPolicyConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().step_scaling_policy_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.step_scaling_policy_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_tracking_scaling_policy_configuration`.\n"]
    pub fn set_target_tracking_scaling_policy_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_tracking_scaling_policy_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_tracking_scaling_policy_configuration =
                    std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `alarm_arns` after provisioning.\n"]
    pub fn alarm_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alarm_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_type` after provisioning.\n"]
    pub fn policy_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scalable_dimension` after provisioning.\n"]
    pub fn scalable_dimension(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scalable_dimension", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_namespace` after provisioning.\n"]
    pub fn service_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `step_scaling_policy_configuration` after provisioning.\n"]
    pub fn step_scaling_policy_configuration(
        &self,
    ) -> terrars::ListRef<AppautoscalingPolicyStepScalingPolicyConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.step_scaling_policy_configuration", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `target_tracking_scaling_policy_configuration` after provisioning.\n"]
    pub fn target_tracking_scaling_policy_configuration(
        &self,
    ) -> terrars::ListRef<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.target_tracking_scaling_policy_configuration", self.extract_ref()),
        )
    }
}

impl terrars::Referable for AppautoscalingPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppautoscalingPolicy { }

impl terrars::ToListMappable for AppautoscalingPolicy {
    type O = terrars::ListRef<AppautoscalingPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppautoscalingPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_appautoscaling_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppautoscalingPolicy {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub resource_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub scalable_dimension: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_namespace: terrars::PrimField<std::string::String>,
}

impl BuildAppautoscalingPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> AppautoscalingPolicy {
        let out = AppautoscalingPolicy(std::rc::Rc::new(AppautoscalingPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppautoscalingPolicyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                policy_type: core::default::Default::default(),
                resource_id: self.resource_id,
                scalable_dimension: self.scalable_dimension,
                service_namespace: self.service_namespace,
                step_scaling_policy_configuration: core::default::Default::default(),
                target_tracking_scaling_policy_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppautoscalingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppautoscalingPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppautoscalingPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alarm_arns` after provisioning.\n"]
    pub fn alarm_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alarm_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_type` after provisioning.\n"]
    pub fn policy_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scalable_dimension` after provisioning.\n"]
    pub fn scalable_dimension(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scalable_dimension", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_namespace` after provisioning.\n"]
    pub fn service_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `step_scaling_policy_configuration` after provisioning.\n"]
    pub fn step_scaling_policy_configuration(
        &self,
    ) -> terrars::ListRef<AppautoscalingPolicyStepScalingPolicyConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.step_scaling_policy_configuration", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `target_tracking_scaling_policy_configuration` after provisioning.\n"]
    pub fn target_tracking_scaling_policy_configuration(
        &self,
    ) -> terrars::ListRef<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.target_tracking_scaling_policy_configuration", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_interval_lower_bound: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_interval_upper_bound: std::option::Option<terrars::PrimField<std::string::String>>,
    scaling_adjustment: terrars::PrimField<f64>,
}

impl AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl {
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

impl terrars::ToListMappable for AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl {
    type O = terrars::BlockAssignable<AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl {
    #[doc = ""]
    pub scaling_adjustment: terrars::PrimField<f64>,
}

impl BuildAppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl {
    pub fn build(self) -> AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl {
        AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl {
            metric_interval_lower_bound: core::default::Default::default(),
            metric_interval_upper_bound: core::default::Default::default(),
            scaling_adjustment: self.scaling_adjustment,
        }
    }
}

pub struct AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentElRef {
        AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentElRef {
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

#[derive(Serialize, Default)]
struct AppautoscalingPolicyStepScalingPolicyConfigurationElDynamic {
    step_adjustment: std
    ::option
    ::Option<terrars::DynamicBlock<AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl>>,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyStepScalingPolicyConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    adjustment_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cooldown: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_aggregation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_adjustment_magnitude: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step_adjustment: std
    ::option
    ::Option<std::vec::Vec<AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl>>,
    dynamic: AppautoscalingPolicyStepScalingPolicyConfigurationElDynamic,
}

impl AppautoscalingPolicyStepScalingPolicyConfigurationEl {
    #[doc = "Set the field `adjustment_type`.\n"]
    pub fn set_adjustment_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.adjustment_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cooldown`.\n"]
    pub fn set_cooldown(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cooldown = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric_aggregation_type`.\n"]
    pub fn set_metric_aggregation_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.metric_aggregation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_adjustment_magnitude`.\n"]
    pub fn set_min_adjustment_magnitude(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_adjustment_magnitude = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `step_adjustment`.\n"]
    pub fn set_step_adjustment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.step_adjustment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.step_adjustment = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppautoscalingPolicyStepScalingPolicyConfigurationEl {
    type O = terrars::BlockAssignable<AppautoscalingPolicyStepScalingPolicyConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyStepScalingPolicyConfigurationEl {}

impl BuildAppautoscalingPolicyStepScalingPolicyConfigurationEl {
    pub fn build(self) -> AppautoscalingPolicyStepScalingPolicyConfigurationEl {
        AppautoscalingPolicyStepScalingPolicyConfigurationEl {
            adjustment_type: core::default::Default::default(),
            cooldown: core::default::Default::default(),
            metric_aggregation_type: core::default::Default::default(),
            min_adjustment_magnitude: core::default::Default::default(),
            step_adjustment: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyStepScalingPolicyConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppautoscalingPolicyStepScalingPolicyConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicyStepScalingPolicyConfigurationElRef {
        AppautoscalingPolicyStepScalingPolicyConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyStepScalingPolicyConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `adjustment_type` after provisioning.\n"]
    pub fn adjustment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.adjustment_type", self.base))
    }

    #[doc = "Get a reference to the value of field `cooldown` after provisioning.\n"]
    pub fn cooldown(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cooldown", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_aggregation_type` after provisioning.\n"]
    pub fn metric_aggregation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_aggregation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `min_adjustment_magnitude` after provisioning.\n"]
    pub fn min_adjustment_magnitude(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_adjustment_magnitude", self.base))
    }
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl { }

impl terrars
::ToListMappable for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl {
    type O =
        terrars
        ::BlockAssignable<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsElRef {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsElRef {
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
struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDynamic {
    dimensions: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl {
    metric_name: terrars::PrimField<std::string::String>,
    namespace: terrars::PrimField<std::string::String>,
    statistic: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl,
        >,
    >,
    dynamic: AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDynamic,
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit = std::option::Option::Some(v.into());
        self
    }

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
                            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl,
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
::ToListMappable for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl {
    type O =
        terrars
        ::BlockAssignable<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl {
    #[doc = ""]
    pub metric_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub namespace: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub statistic: terrars::PrimField<std::string::String>,
}

impl BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl {
            metric_name: self.metric_name,
            namespace: self.namespace,
            statistic: self.statistic,
            unit: core::default::Default::default(),
            dimensions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElRef {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElRef {
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
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl {
    predefined_metric_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_label: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl {
    #[doc = "Set the field `resource_label`.\n"]
    pub fn set_resource_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_label = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl {
    type O =
        terrars
        ::BlockAssignable<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl {
    #[doc = ""]
    pub predefined_metric_type: terrars::PrimField<std::string::String>,
}

impl BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl {
            predefined_metric_type: self.predefined_metric_type,
            resource_label: core::default::Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationElRef {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationElRef {
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
struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElDynamic {
    customized_metric_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl>,
    >,
    predefined_metric_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl>,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_scale_in: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_in_cooldown: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_out_cooldown: std::option::Option<terrars::PrimField<f64>>,
    target_value: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customized_metric_specification: std
    ::option
    ::Option<
        std::vec::Vec<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predefined_metric_specification: std
    ::option
    ::Option<
        std::vec::Vec<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl>,
    >,
    dynamic: AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElDynamic,
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl {
    #[doc = "Set the field `disable_scale_in`.\n"]
    pub fn set_disable_scale_in(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_scale_in = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_in_cooldown`.\n"]
    pub fn set_scale_in_cooldown(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_in_cooldown = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_out_cooldown`.\n"]
    pub fn set_scale_out_cooldown(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_out_cooldown = std::option::Option::Some(v.into());
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
                            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl,
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
                            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl,
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

impl terrars::ToListMappable for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl {
    type O = terrars::BlockAssignable<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl {
    #[doc = ""]
    pub target_value: terrars::PrimField<f64>,
}

impl BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl {
    pub fn build(self) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl {
            disable_scale_in: core::default::Default::default(),
            scale_in_cooldown: core::default::Default::default(),
            scale_out_cooldown: core::default::Default::default(),
            target_value: self.target_value,
            customized_metric_specification: core::default::Default::default(),
            predefined_metric_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElRef {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disable_scale_in` after provisioning.\n"]
    pub fn disable_scale_in(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_scale_in", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_in_cooldown` after provisioning.\n"]
    pub fn scale_in_cooldown(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_in_cooldown", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_out_cooldown` after provisioning.\n"]
    pub fn scale_out_cooldown(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_out_cooldown", self.base))
    }

    #[doc = "Get a reference to the value of field `target_value` after provisioning.\n"]
    pub fn target_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_value", self.base))
    }

    #[doc = "Get a reference to the value of field `customized_metric_specification` after provisioning.\n"]
    pub fn customized_metric_specification(
        &self,
    ) -> terrars
    ::ListRef<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customized_metric_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `predefined_metric_specification` after provisioning.\n"]
    pub fn predefined_metric_specification(
        &self,
    ) -> terrars
    ::ListRef<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predefined_metric_specification", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyDynamic {
    step_scaling_policy_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AppautoscalingPolicyStepScalingPolicyConfigurationEl>>,
    target_tracking_scaling_policy_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl>>,
}
