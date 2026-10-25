#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppautoscalingPolicyData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_id: terrars::PrimField<std::string::String>,
    scalable_dimension: terrars::PrimField<std::string::String>,
    service_namespace: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predictive_scaling_policy_configuration: std
    ::option
    ::Option<std::vec::Vec<AppautoscalingPolicy_PredictiveScalingPolicyConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step_scaling_policy_configuration: std
    ::option
    ::Option<std::vec::Vec<AppautoscalingPolicy_StepScalingPolicyConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_tracking_scaling_policy_configuration: std
    ::option
    ::Option<std::vec::Vec<AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration>>,
    dynamic: AppautoscalingPolicyDynamic,
}

struct AppautoscalingPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppautoscalingPolicyData_>,
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `predictive_scaling_policy_configuration`.\n"]
    pub fn set_predictive_scaling_policy_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppautoscalingPolicy_PredictiveScalingPolicyConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().predictive_scaling_policy_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.predictive_scaling_policy_configuration =
                    std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `step_scaling_policy_configuration`.\n"]
    pub fn set_step_scaling_policy_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppautoscalingPolicy_StepScalingPolicyConfiguration>>,
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
                    ::Into<terrars::BlockAssignable<AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration>>,
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `predictive_scaling_policy_configuration` after provisioning.\n"]
    pub fn predictive_scaling_policy_configuration(
        &self,
    ) -> terrars::ListRef<AppautoscalingPolicy_PredictiveScalingPolicyConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.predictive_scaling_policy_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `step_scaling_policy_configuration` after provisioning.\n"]
    pub fn step_scaling_policy_configuration(
        &self,
    ) -> terrars::ListRef<AppautoscalingPolicy_StepScalingPolicyConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.step_scaling_policy_configuration", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `target_tracking_scaling_policy_configuration` after provisioning.\n"]
    pub fn target_tracking_scaling_policy_configuration(
        &self,
    ) -> terrars::ListRef<AppautoscalingPolicy_TargetTrackingScalingPolicyConfigurationRef> {
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
            data: std::cell::RefCell::new(AppautoscalingPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                policy_type: core::default::Default::default(),
                region: core::default::Default::default(),
                resource_id: self.resource_id,
                scalable_dimension: self.scalable_dimension,
                service_namespace: self.service_namespace,
                predictive_scaling_policy_configuration: core::default::Default::default(),
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `predictive_scaling_policy_configuration` after provisioning.\n"]
    pub fn predictive_scaling_policy_configuration(
        &self,
    ) -> terrars::ListRef<AppautoscalingPolicy_PredictiveScalingPolicyConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.predictive_scaling_policy_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `step_scaling_policy_configuration` after provisioning.\n"]
    pub fn step_scaling_policy_configuration(
        &self,
    ) -> terrars::ListRef<AppautoscalingPolicy_StepScalingPolicyConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.step_scaling_policy_configuration", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `target_tracking_scaling_policy_configuration` after provisioning.\n"]
    pub fn target_tracking_scaling_policy_configuration(
        &self,
    ) -> terrars::ListRef<AppautoscalingPolicy_TargetTrackingScalingPolicyConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.target_tracking_scaling_policy_configuration", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyDynamic {
    predictive_scaling_policy_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AppautoscalingPolicy_PredictiveScalingPolicyConfiguration>>,
    step_scaling_policy_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AppautoscalingPolicy_StepScalingPolicyConfiguration>>,
    target_tracking_scaling_policy_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration>>,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric_Dimension {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric_Dimension {

}

impl terrars
::ToListMappable for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric_Dimension {
    type O =
        terrars
        ::BlockAssignable<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric_Dimension,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric_Dimension {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric_Dimension {
    pub fn build(
        self,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric_Dimension {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric_Dimension {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric_DimensionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric_DimensionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric_DimensionRef {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric_DimensionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric_DimensionRef {
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
struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_MetricDynamic {
    dimension: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric_Dimension,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric_Dimension,
        >,
    >,
    dynamic: AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_MetricDynamic,
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric {
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

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric_Dimension,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dimension = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric {
    type O =
        terrars
        ::BlockAssignable<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric {}

impl BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric {
    pub fn build(
        self,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric {
            metric_name: core::default::Default::default(),
            namespace: core::default::Default::default(),
            dimension: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_MetricRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_MetricRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_MetricRef {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_MetricRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_MetricRef {
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
struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStatDynamic {
    metric: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat {
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
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric,
        >,
    >,
    dynamic: AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStatDynamic,
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat {
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
                            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_Metric,
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
::ToListMappable for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat {
    type O =
        terrars
        ::BlockAssignable<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat {
    #[doc = ""]
    pub stat: terrars::PrimField<std::string::String>,
}

impl BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat {
    pub fn build(
        self,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat {
            stat: self.stat,
            unit: core::default::Default::default(),
            metric: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStatRef {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStatRef {
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
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat_MetricRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueryDynamic {
    metric_stat: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery {
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
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat,
        >,
    >,
    dynamic: AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueryDynamic,
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery {
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
                            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStat,
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
::ToListMappable for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery {
    type O =
        terrars
        ::BlockAssignable<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery {
    pub fn build(
        self,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery {
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
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueryRef {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueryRef {
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
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery_MetricStatRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_stat", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedCapacityMetricSpecificationDynamic {
    metric_data_query: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_data_query: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery,
        >,
    >,
    dynamic: AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedCapacityMetricSpecificationDynamic,
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification {
    #[doc = "Set the field `metric_data_query`.\n"]
    pub fn set_metric_data_query(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric_data_query = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_data_query = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification {
    type O =
        terrars
        ::BlockAssignable<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification {}

impl BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification {
    pub fn build(
        self,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification {
            metric_data_query: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedCapacityMetricSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedCapacityMetricSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedCapacityMetricSpecificationRef {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedCapacityMetricSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedCapacityMetricSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_data_query` after provisioning.\n"]
    pub fn metric_data_query(
        &self,
    ) -> terrars
    ::ListRef<
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueryRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_data_query", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecificationDynamic {
    metric_data_query: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_data_query: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery,
        >,
    >,
    dynamic: AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecificationDynamic,
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification {
    #[doc = "Set the field `metric_data_query`.\n"]
    pub fn set_metric_data_query(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric_data_query = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_data_query = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification {
    type O =
        terrars
        ::BlockAssignable<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification {}

impl BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification {
    pub fn build(
        self,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification {
            metric_data_query: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecificationRef {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_data_query` after provisioning.\n"]
    pub fn metric_data_query(
        &self,
    ) -> terrars
    ::ListRef<
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueryRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_data_query", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedScalingMetricSpecificationDynamic {
    metric_data_query: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedScalingMetricSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_data_query: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery,
        >,
    >,
    dynamic: AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedScalingMetricSpecificationDynamic,
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedScalingMetricSpecification {
    #[doc = "Set the field `metric_data_query`.\n"]
    pub fn set_metric_data_query(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQuery,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric_data_query = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_data_query = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedScalingMetricSpecification {
    type O =
        terrars
        ::BlockAssignable<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedScalingMetricSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedScalingMetricSpecification {}

impl BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedScalingMetricSpecification {
    pub fn build(
        self,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedScalingMetricSpecification {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedScalingMetricSpecification {
            metric_data_query: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedScalingMetricSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedScalingMetricSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedScalingMetricSpecificationRef {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedScalingMetricSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedScalingMetricSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_data_query` after provisioning.\n"]
    pub fn metric_data_query(
        &self,
    ) -> terrars
    ::ListRef<
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification_MetricDataQueryRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_data_query", self.base))
    }
}

#[derive(Serialize)]
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedLoadMetricSpecification {
    predefined_metric_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_label: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedLoadMetricSpecification {
    #[doc = "Set the field `resource_label`.\n"]
    pub fn set_resource_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_label = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedLoadMetricSpecification {
    type O =
        terrars
        ::BlockAssignable<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedLoadMetricSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedLoadMetricSpecification {
    #[doc = ""]
    pub predefined_metric_type: terrars::PrimField<std::string::String>,
}

impl BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedLoadMetricSpecification {
    pub fn build(
        self,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedLoadMetricSpecification {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedLoadMetricSpecification {
            predefined_metric_type: self.predefined_metric_type,
            resource_label: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedLoadMetricSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedLoadMetricSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedLoadMetricSpecificationRef {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedLoadMetricSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedLoadMetricSpecificationRef {
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
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedMetricPairSpecification {
    predefined_metric_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_label: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedMetricPairSpecification {
    #[doc = "Set the field `resource_label`.\n"]
    pub fn set_resource_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_label = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedMetricPairSpecification {
    type O =
        terrars
        ::BlockAssignable<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedMetricPairSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedMetricPairSpecification {
    #[doc = ""]
    pub predefined_metric_type: terrars::PrimField<std::string::String>,
}

impl BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedMetricPairSpecification {
    pub fn build(
        self,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedMetricPairSpecification {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedMetricPairSpecification {
            predefined_metric_type: self.predefined_metric_type,
            resource_label: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedMetricPairSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedMetricPairSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedMetricPairSpecificationRef {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedMetricPairSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedMetricPairSpecificationRef {
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
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedScalingMetricSpecification {
    predefined_metric_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_label: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedScalingMetricSpecification {
    #[doc = "Set the field `resource_label`.\n"]
    pub fn set_resource_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_label = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedScalingMetricSpecification {
    type O =
        terrars
        ::BlockAssignable<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedScalingMetricSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedScalingMetricSpecification {
    #[doc = ""]
    pub predefined_metric_type: terrars::PrimField<std::string::String>,
}

impl BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedScalingMetricSpecification {
    pub fn build(
        self,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedScalingMetricSpecification {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedScalingMetricSpecification {
            predefined_metric_type: self.predefined_metric_type,
            resource_label: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedScalingMetricSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedScalingMetricSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedScalingMetricSpecificationRef {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedScalingMetricSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedScalingMetricSpecificationRef {
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
struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecificationDynamic {
    customized_capacity_metric_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification,
        >,
    >,
    customized_load_metric_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification,
        >,
    >,
    customized_scaling_metric_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedScalingMetricSpecification,
        >,
    >,
    predefined_load_metric_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedLoadMetricSpecification,
        >,
    >,
    predefined_metric_pair_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedMetricPairSpecification,
        >,
    >,
    predefined_scaling_metric_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedScalingMetricSpecification,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification {
    target_value: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customized_capacity_metric_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customized_load_metric_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customized_scaling_metric_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedScalingMetricSpecification,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predefined_load_metric_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedLoadMetricSpecification,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predefined_metric_pair_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedMetricPairSpecification,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predefined_scaling_metric_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedScalingMetricSpecification,
        >,
    >,
    dynamic: AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecificationDynamic,
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification {
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
                            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedCapacityMetricSpecification,
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
                            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecification,
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
                            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedScalingMetricSpecification,
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
                            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedLoadMetricSpecification,
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
                            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedMetricPairSpecification,
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
                            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedScalingMetricSpecification,
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

impl terrars::ToListMappable for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification {
    type O = terrars::BlockAssignable<AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification {
    #[doc = ""]
    pub target_value: terrars::PrimField<std::string::String>,
}

impl BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification {
    pub fn build(self) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification {
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
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecificationRef {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_value` after provisioning.\n"]
    pub fn target_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_value", self.base))
    }

    #[doc = "Get a reference to the value of field `customized_capacity_metric_specification` after provisioning.\n"]
    pub fn customized_capacity_metric_specification(
        &self,
    ) -> terrars
    ::ListRef<
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedCapacityMetricSpecificationRef,
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
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedLoadMetricSpecificationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customized_load_metric_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `customized_scaling_metric_specification` after provisioning.\n"]
    pub fn customized_scaling_metric_specification(
        &self,
    ) -> terrars
    ::ListRef<
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_CustomizedScalingMetricSpecificationRef,
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
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedLoadMetricSpecificationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predefined_load_metric_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `predefined_metric_pair_specification` after provisioning.\n"]
    pub fn predefined_metric_pair_specification(
        &self,
    ) -> terrars
    ::ListRef<
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedMetricPairSpecificationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predefined_metric_pair_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `predefined_scaling_metric_specification` after provisioning.\n"]
    pub fn predefined_scaling_metric_specification(
        &self,
    ) -> terrars
    ::ListRef<
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification_PredefinedScalingMetricSpecificationRef,
    > {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.predefined_scaling_metric_specification", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicy_PredictiveScalingPolicyConfigurationDynamic {
    metric_specification: std
    ::option
    ::Option<terrars::DynamicBlock<AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification>>,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_capacity_breach_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_capacity_buffer: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduling_buffer_time: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_specification: std
    ::option
    ::Option<std::vec::Vec<AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification>>,
    dynamic: AppautoscalingPolicy_PredictiveScalingPolicyConfigurationDynamic,
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfiguration {
    #[doc = "Set the field `max_capacity_breach_behavior`.\n"]
    pub fn set_max_capacity_breach_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.max_capacity_breach_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_capacity_buffer`.\n"]
    pub fn set_max_capacity_buffer(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_capacity_buffer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scheduling_buffer_time`.\n"]
    pub fn set_scheduling_buffer_time(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
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
                        ::BlockAssignable<
                            AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecification,
                        >,
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

impl terrars::ToListMappable for AppautoscalingPolicy_PredictiveScalingPolicyConfiguration {
    type O = terrars::BlockAssignable<AppautoscalingPolicy_PredictiveScalingPolicyConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration {}

impl BuildAppautoscalingPolicy_PredictiveScalingPolicyConfiguration {
    pub fn build(self) -> AppautoscalingPolicy_PredictiveScalingPolicyConfiguration {
        AppautoscalingPolicy_PredictiveScalingPolicyConfiguration {
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
pub struct AppautoscalingPolicy_PredictiveScalingPolicyConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppautoscalingPolicy_PredictiveScalingPolicyConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicy_PredictiveScalingPolicyConfigurationRef {
        AppautoscalingPolicy_PredictiveScalingPolicyConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicy_PredictiveScalingPolicyConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_capacity_breach_behavior` after provisioning.\n"]
    pub fn max_capacity_breach_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_capacity_breach_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `max_capacity_buffer` after provisioning.\n"]
    pub fn max_capacity_buffer(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_capacity_buffer", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduling_buffer_time` after provisioning.\n"]
    pub fn scheduling_buffer_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduling_buffer_time", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_specification` after provisioning.\n"]
    pub fn metric_specification(
        &self,
    ) -> terrars::ListRef<AppautoscalingPolicy_PredictiveScalingPolicyConfiguration_MetricSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_specification", self.base))
    }
}

#[derive(Serialize)]
pub struct AppautoscalingPolicy_StepScalingPolicyConfiguration_StepAdjustment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_interval_lower_bound: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_interval_upper_bound: std::option::Option<terrars::PrimField<std::string::String>>,
    scaling_adjustment: terrars::PrimField<f64>,
}

impl AppautoscalingPolicy_StepScalingPolicyConfiguration_StepAdjustment {
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

impl terrars::ToListMappable for AppautoscalingPolicy_StepScalingPolicyConfiguration_StepAdjustment {
    type O = terrars::BlockAssignable<AppautoscalingPolicy_StepScalingPolicyConfiguration_StepAdjustment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicy_StepScalingPolicyConfiguration_StepAdjustment {
    #[doc = ""]
    pub scaling_adjustment: terrars::PrimField<f64>,
}

impl BuildAppautoscalingPolicy_StepScalingPolicyConfiguration_StepAdjustment {
    pub fn build(self) -> AppautoscalingPolicy_StepScalingPolicyConfiguration_StepAdjustment {
        AppautoscalingPolicy_StepScalingPolicyConfiguration_StepAdjustment {
            metric_interval_lower_bound: core::default::Default::default(),
            metric_interval_upper_bound: core::default::Default::default(),
            scaling_adjustment: self.scaling_adjustment,
        }
    }
}

#[allow(dead_code)]
pub struct AppautoscalingPolicy_StepScalingPolicyConfiguration_StepAdjustmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppautoscalingPolicy_StepScalingPolicyConfiguration_StepAdjustmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicy_StepScalingPolicyConfiguration_StepAdjustmentRef {
        AppautoscalingPolicy_StepScalingPolicyConfiguration_StepAdjustmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicy_StepScalingPolicyConfiguration_StepAdjustmentRef {
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

#[derive(Serialize, Default)]
struct AppautoscalingPolicy_StepScalingPolicyConfigurationDynamic {
    step_adjustment: std
    ::option
    ::Option<terrars::DynamicBlock<AppautoscalingPolicy_StepScalingPolicyConfiguration_StepAdjustment>>,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicy_StepScalingPolicyConfiguration {
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
    ::Option<std::vec::Vec<AppautoscalingPolicy_StepScalingPolicyConfiguration_StepAdjustment>>,
    dynamic: AppautoscalingPolicy_StepScalingPolicyConfigurationDynamic,
}

impl AppautoscalingPolicy_StepScalingPolicyConfiguration {
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
                        terrars::BlockAssignable<AppautoscalingPolicy_StepScalingPolicyConfiguration_StepAdjustment>,
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

impl terrars::ToListMappable for AppautoscalingPolicy_StepScalingPolicyConfiguration {
    type O = terrars::BlockAssignable<AppautoscalingPolicy_StepScalingPolicyConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicy_StepScalingPolicyConfiguration {}

impl BuildAppautoscalingPolicy_StepScalingPolicyConfiguration {
    pub fn build(self) -> AppautoscalingPolicy_StepScalingPolicyConfiguration {
        AppautoscalingPolicy_StepScalingPolicyConfiguration {
            adjustment_type: core::default::Default::default(),
            cooldown: core::default::Default::default(),
            metric_aggregation_type: core::default::Default::default(),
            min_adjustment_magnitude: core::default::Default::default(),
            step_adjustment: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppautoscalingPolicy_StepScalingPolicyConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppautoscalingPolicy_StepScalingPolicyConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicy_StepScalingPolicyConfigurationRef {
        AppautoscalingPolicy_StepScalingPolicyConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicy_StepScalingPolicyConfigurationRef {
    #[allow(dead_code)]
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
pub struct AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Dimensions {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Dimensions { }

impl terrars
::ToListMappable for AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Dimensions {
    type O =
        terrars
        ::BlockAssignable<
            AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Dimensions,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Dimensions {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildAppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Dimensions {
    pub fn build(
        self,
    ) -> AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Dimensions {
        AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Dimensions {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_DimensionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_DimensionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_DimensionsRef {
        AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_DimensionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_DimensionsRef {
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
struct AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_MetricDynamic {
    dimensions: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Dimensions,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric {
    metric_name: terrars::PrimField<std::string::String>,
    namespace: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Dimensions>,
    >,
    dynamic: AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_MetricDynamic,
}

impl AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric {
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
                            AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Dimensions,
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
::ToListMappable for AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric {
    type O =
        terrars
        ::BlockAssignable<
            AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric {
    #[doc = ""]
    pub metric_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub namespace: terrars::PrimField<std::string::String>,
}

impl BuildAppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric {
    pub fn build(
        self,
    ) -> AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric {
        AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric {
            metric_name: self.metric_name,
            namespace: self.namespace,
            dimensions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_MetricRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_MetricRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_MetricRef {
        AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_MetricRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_MetricRef {
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
struct AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStatDynamic {
    metric: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat {
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
            AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric,
        >,
    >,
    dynamic: AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStatDynamic,
}

impl AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat {
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
                            AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_Metric,
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
::ToListMappable for AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat {
    type O =
        terrars
        ::BlockAssignable<
            AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat {
    #[doc = ""]
    pub stat: terrars::PrimField<std::string::String>,
}

impl BuildAppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat {
    pub fn build(
        self,
    ) -> AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat {
        AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat {
            stat: self.stat,
            unit: core::default::Default::default(),
            metric: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStatRef {
        AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStatRef {
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
        AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat_MetricRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_MetricsDynamic {
    metric_stat: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics {
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
            AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat,
        >,
    >,
    dynamic: AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_MetricsDynamic,
}

impl AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics {
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
                            AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStat,
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
::ToListMappable for AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics {
    type O =
        terrars
        ::BlockAssignable<
            AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildAppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics {
    pub fn build(
        self,
    ) -> AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics {
        AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics {
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
pub struct AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_MetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_MetricsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_MetricsRef {
        AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_MetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_MetricsRef {
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
        AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics_MetricStatRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_stat", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecificationDynamic {
    dimensions: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Dimensions,
        >,
    >,
    metrics: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statistic: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Dimensions>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metrics: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics>,
    >,
    dynamic: AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecificationDynamic,
}

impl AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification {
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
                            AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Dimensions,
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
                            AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification_Metrics,
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

impl terrars
::ToListMappable for AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification {
    type O =
        terrars
        ::BlockAssignable<
            AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification {}

impl BuildAppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification {
    pub fn build(
        self,
    ) -> AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification {
        AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification {
            metric_name: core::default::Default::default(),
            namespace: core::default::Default::default(),
            statistic: core::default::Default::default(),
            unit: core::default::Default::default(),
            dimensions: core::default::Default::default(),
            metrics: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecificationRef {
        AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecificationRef {
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
pub struct AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_PredefinedMetricSpecification {
    predefined_metric_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_label: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_PredefinedMetricSpecification {
    #[doc = "Set the field `resource_label`.\n"]
    pub fn set_resource_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_label = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_PredefinedMetricSpecification {
    type O =
        terrars
        ::BlockAssignable<
            AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_PredefinedMetricSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_PredefinedMetricSpecification {
    #[doc = ""]
    pub predefined_metric_type: terrars::PrimField<std::string::String>,
}

impl BuildAppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_PredefinedMetricSpecification {
    pub fn build(
        self,
    ) -> AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_PredefinedMetricSpecification {
        AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_PredefinedMetricSpecification {
            predefined_metric_type: self.predefined_metric_type,
            resource_label: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_PredefinedMetricSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_PredefinedMetricSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_PredefinedMetricSpecificationRef {
        AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_PredefinedMetricSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_PredefinedMetricSpecificationRef {
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
struct AppautoscalingPolicy_TargetTrackingScalingPolicyConfigurationDynamic {
    customized_metric_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification>,
    >,
    predefined_metric_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_PredefinedMetricSpecification>,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration {
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
        std::vec::Vec<AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predefined_metric_specification: std
    ::option
    ::Option<
        std::vec::Vec<AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_PredefinedMetricSpecification>,
    >,
    dynamic: AppautoscalingPolicy_TargetTrackingScalingPolicyConfigurationDynamic,
}

impl AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration {
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
                            AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecification,
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
                            AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_PredefinedMetricSpecification,
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

impl terrars::ToListMappable for AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration {
    type O = terrars::BlockAssignable<AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration {
    #[doc = ""]
    pub target_value: terrars::PrimField<f64>,
}

impl BuildAppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration {
    pub fn build(self) -> AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration {
        AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration {
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

#[allow(dead_code)]
pub struct AppautoscalingPolicy_TargetTrackingScalingPolicyConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppautoscalingPolicy_TargetTrackingScalingPolicyConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingPolicy_TargetTrackingScalingPolicyConfigurationRef {
        AppautoscalingPolicy_TargetTrackingScalingPolicyConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicy_TargetTrackingScalingPolicyConfigurationRef {
    #[allow(dead_code)]
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
    ::ListRef<AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_CustomizedMetricSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customized_metric_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `predefined_metric_specification` after provisioning.\n"]
    pub fn predefined_metric_specification(
        &self,
    ) -> terrars
    ::ListRef<AppautoscalingPolicy_TargetTrackingScalingPolicyConfiguration_PredefinedMetricSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predefined_metric_specification", self.base))
    }
}
