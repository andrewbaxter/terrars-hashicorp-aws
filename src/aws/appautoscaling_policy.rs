use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppautoscalingPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_type: Option<PrimField<String>>,
    resource_id: PrimField<String>,
    scalable_dimension: PrimField<String>,
    service_namespace: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    step_scaling_policy_configuration: Option<Vec<AppautoscalingPolicyStepScalingPolicyConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_tracking_scaling_policy_configuration: Option<
        Vec<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl>,
    >,
    dynamic: AppautoscalingPolicyDynamic,
}

struct AppautoscalingPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppautoscalingPolicyData>,
}

#[derive(Clone)]
pub struct AppautoscalingPolicy(Rc<AppautoscalingPolicy_>);

impl AppautoscalingPolicy {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `policy_type`.\n"]
    pub fn set_policy_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().policy_type = Some(v.into());
        self
    }

    #[doc= "Set the field `step_scaling_policy_configuration`.\n"]
    pub fn set_step_scaling_policy_configuration(
        self,
        v: impl Into<BlockAssignable<AppautoscalingPolicyStepScalingPolicyConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().step_scaling_policy_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.step_scaling_policy_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `target_tracking_scaling_policy_configuration`.\n"]
    pub fn set_target_tracking_scaling_policy_configuration(
        self,
        v: impl Into<BlockAssignable<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_tracking_scaling_policy_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_tracking_scaling_policy_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `alarm_arns` after provisioning.\n"]
    pub fn alarm_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.alarm_arns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `policy_type` after provisioning.\n"]
    pub fn policy_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scalable_dimension` after provisioning.\n"]
    pub fn scalable_dimension(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scalable_dimension", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_namespace` after provisioning.\n"]
    pub fn service_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_namespace", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `step_scaling_policy_configuration` after provisioning.\n"]
    pub fn step_scaling_policy_configuration(&self) -> ListRef<AppautoscalingPolicyStepScalingPolicyConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.step_scaling_policy_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_tracking_scaling_policy_configuration` after provisioning.\n"]
    pub fn target_tracking_scaling_policy_configuration(
        &self,
    ) -> ListRef<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_tracking_scaling_policy_configuration", self.extract_ref()),
        )
    }
}

impl Resource for AppautoscalingPolicy {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for AppautoscalingPolicy {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for AppautoscalingPolicy {
    type O = ListRef<AppautoscalingPolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for AppautoscalingPolicy_ {
    fn extract_resource_type(&self) -> String {
        "aws_appautoscaling_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppautoscalingPolicy {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub resource_id: PrimField<String>,
    #[doc= ""]
    pub scalable_dimension: PrimField<String>,
    #[doc= ""]
    pub service_namespace: PrimField<String>,
}

impl BuildAppautoscalingPolicy {
    pub fn build(self, stack: &mut Stack) -> AppautoscalingPolicy {
        let out = AppautoscalingPolicy(Rc::new(AppautoscalingPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppautoscalingPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                policy_type: core::default::Default::default(),
                resource_id: self.resource_id,
                scalable_dimension: self.scalable_dimension,
                service_namespace: self.service_namespace,
                step_scaling_policy_configuration: core::default::Default::default(),
                target_tracking_scaling_policy_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppautoscalingPolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppautoscalingPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `alarm_arns` after provisioning.\n"]
    pub fn alarm_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.alarm_arns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `policy_type` after provisioning.\n"]
    pub fn policy_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scalable_dimension` after provisioning.\n"]
    pub fn scalable_dimension(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scalable_dimension", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_namespace` after provisioning.\n"]
    pub fn service_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_namespace", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `step_scaling_policy_configuration` after provisioning.\n"]
    pub fn step_scaling_policy_configuration(&self) -> ListRef<AppautoscalingPolicyStepScalingPolicyConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.step_scaling_policy_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_tracking_scaling_policy_configuration` after provisioning.\n"]
    pub fn target_tracking_scaling_policy_configuration(
        &self,
    ) -> ListRef<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_tracking_scaling_policy_configuration", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_interval_lower_bound: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_interval_upper_bound: Option<PrimField<String>>,
    scaling_adjustment: PrimField<f64>,
}

impl AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl {
    #[doc= "Set the field `metric_interval_lower_bound`.\n"]
    pub fn set_metric_interval_lower_bound(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_interval_lower_bound = Some(v.into());
        self
    }

    #[doc= "Set the field `metric_interval_upper_bound`.\n"]
    pub fn set_metric_interval_upper_bound(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_interval_upper_bound = Some(v.into());
        self
    }
}

impl ToListMappable for AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl {
    type O = BlockAssignable<AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl {
    #[doc= ""]
    pub scaling_adjustment: PrimField<f64>,
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
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentElRef {
        AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `metric_interval_lower_bound` after provisioning.\n"]
    pub fn metric_interval_lower_bound(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_interval_lower_bound", self.base))
    }

    #[doc= "Get a reference to the value of field `metric_interval_upper_bound` after provisioning.\n"]
    pub fn metric_interval_upper_bound(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_interval_upper_bound", self.base))
    }

    #[doc= "Get a reference to the value of field `scaling_adjustment` after provisioning.\n"]
    pub fn scaling_adjustment(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scaling_adjustment", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyStepScalingPolicyConfigurationElDynamic {
    step_adjustment: Option<DynamicBlock<AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl>>,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyStepScalingPolicyConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    adjustment_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cooldown: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_aggregation_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_adjustment_magnitude: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    step_adjustment: Option<Vec<AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl>>,
    dynamic: AppautoscalingPolicyStepScalingPolicyConfigurationElDynamic,
}

impl AppautoscalingPolicyStepScalingPolicyConfigurationEl {
    #[doc= "Set the field `adjustment_type`.\n"]
    pub fn set_adjustment_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.adjustment_type = Some(v.into());
        self
    }

    #[doc= "Set the field `cooldown`.\n"]
    pub fn set_cooldown(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cooldown = Some(v.into());
        self
    }

    #[doc= "Set the field `metric_aggregation_type`.\n"]
    pub fn set_metric_aggregation_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_aggregation_type = Some(v.into());
        self
    }

    #[doc= "Set the field `min_adjustment_magnitude`.\n"]
    pub fn set_min_adjustment_magnitude(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_adjustment_magnitude = Some(v.into());
        self
    }

    #[doc= "Set the field `step_adjustment`.\n"]
    pub fn set_step_adjustment(
        mut self,
        v: impl Into<BlockAssignable<AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.step_adjustment = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.step_adjustment = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyStepScalingPolicyConfigurationEl {
    type O = BlockAssignable<AppautoscalingPolicyStepScalingPolicyConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyStepScalingPolicyConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyStepScalingPolicyConfigurationElRef {
    fn new(shared: StackShared, base: String) -> AppautoscalingPolicyStepScalingPolicyConfigurationElRef {
        AppautoscalingPolicyStepScalingPolicyConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyStepScalingPolicyConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `adjustment_type` after provisioning.\n"]
    pub fn adjustment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.adjustment_type", self.base))
    }

    #[doc= "Get a reference to the value of field `cooldown` after provisioning.\n"]
    pub fn cooldown(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cooldown", self.base))
    }

    #[doc= "Get a reference to the value of field `metric_aggregation_type` after provisioning.\n"]
    pub fn metric_aggregation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_aggregation_type", self.base))
    }

    #[doc= "Get a reference to the value of field `min_adjustment_magnitude` after provisioning.\n"]
    pub fn min_adjustment_magnitude(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_adjustment_magnitude", self.base))
    }
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl {
    name: PrimField<String>,
    value: PrimField<String>,
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl { }

impl ToListMappable for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl {
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub value: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsElRef {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDynamic {
    dimensions: Option<
        DynamicBlock<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl {
    metric_name: PrimField<String>,
    namespace: PrimField<String>,
    statistic: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<
        Vec<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl>,
    >,
    dynamic: AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDynamic,
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl {
    #[doc= "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc= "Set the field `dimensions`.\n"]
    pub fn set_dimensions(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimensions = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimensions = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl {
    #[doc= ""]
    pub metric_name: PrimField<String>,
    #[doc= ""]
    pub namespace: PrimField<String>,
    #[doc= ""]
    pub statistic: PrimField<String>,
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
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElRef {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc= "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc= "Get a reference to the value of field `statistic` after provisioning.\n"]
    pub fn statistic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.statistic", self.base))
    }

    #[doc= "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl {
    predefined_metric_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_label: Option<PrimField<String>>,
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl {
    #[doc= "Set the field `resource_label`.\n"]
    pub fn set_resource_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_label = Some(v.into());
        self
    }
}

impl ToListMappable for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl {
    #[doc= ""]
    pub predefined_metric_type: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationElRef {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `predefined_metric_type` after provisioning.\n"]
    pub fn predefined_metric_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.predefined_metric_type", self.base))
    }

    #[doc= "Get a reference to the value of field `resource_label` after provisioning.\n"]
    pub fn resource_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_label", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElDynamic {
    customized_metric_specification: Option<
        DynamicBlock<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl>,
    >,
    predefined_metric_specification: Option<
        DynamicBlock<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl>,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_scale_in: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_in_cooldown: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_out_cooldown: Option<PrimField<f64>>,
    target_value: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customized_metric_specification: Option<
        Vec<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    predefined_metric_specification: Option<
        Vec<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl>,
    >,
    dynamic: AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElDynamic,
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl {
    #[doc= "Set the field `disable_scale_in`.\n"]
    pub fn set_disable_scale_in(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_scale_in = Some(v.into());
        self
    }

    #[doc= "Set the field `scale_in_cooldown`.\n"]
    pub fn set_scale_in_cooldown(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scale_in_cooldown = Some(v.into());
        self
    }

    #[doc= "Set the field `scale_out_cooldown`.\n"]
    pub fn set_scale_out_cooldown(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scale_out_cooldown = Some(v.into());
        self
    }

    #[doc= "Set the field `customized_metric_specification`.\n"]
    pub fn set_customized_metric_specification(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.customized_metric_specification = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.customized_metric_specification = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `predefined_metric_specification`.\n"]
    pub fn set_predefined_metric_specification(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.predefined_metric_specification = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.predefined_metric_specification = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl {
    type O = BlockAssignable<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl {
    #[doc= ""]
    pub target_value: PrimField<f64>,
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
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElRef {
    fn new(shared: StackShared, base: String) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElRef {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disable_scale_in` after provisioning.\n"]
    pub fn disable_scale_in(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_scale_in", self.base))
    }

    #[doc= "Get a reference to the value of field `scale_in_cooldown` after provisioning.\n"]
    pub fn scale_in_cooldown(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scale_in_cooldown", self.base))
    }

    #[doc= "Get a reference to the value of field `scale_out_cooldown` after provisioning.\n"]
    pub fn scale_out_cooldown(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scale_out_cooldown", self.base))
    }

    #[doc= "Get a reference to the value of field `target_value` after provisioning.\n"]
    pub fn target_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_value", self.base))
    }

    #[doc= "Get a reference to the value of field `customized_metric_specification` after provisioning.\n"]
    pub fn customized_metric_specification(
        &self,
    ) -> ListRef<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.customized_metric_specification", self.base))
    }

    #[doc= "Get a reference to the value of field `predefined_metric_specification` after provisioning.\n"]
    pub fn predefined_metric_specification(
        &self,
    ) -> ListRef<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.predefined_metric_specification", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyDynamic {
    step_scaling_policy_configuration: Option<DynamicBlock<AppautoscalingPolicyStepScalingPolicyConfigurationEl>>,
    target_tracking_scaling_policy_configuration: Option<
        DynamicBlock<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl>,
    >,
}
