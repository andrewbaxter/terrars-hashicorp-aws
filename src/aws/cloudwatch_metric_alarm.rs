#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudwatchMetricAlarmData_ {
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
    actions_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alarm_actions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alarm_description: std::option::Option<terrars::PrimField<std::string::String>>,
    alarm_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comparison_operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datapoints_to_alarm: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate_low_sample_count_percentiles: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_periods: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extended_statistic: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insufficient_data_actions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ok_actions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statistic: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threshold_metric_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    treat_missing_data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_criteria: std::option::Option<std::vec::Vec<CloudwatchMetricAlarm_EvaluationCriteria>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_window: std::option::Option<std::vec::Vec<CloudwatchMetricAlarm_EvaluationWindow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_query: std::option::Option<std::vec::Vec<CloudwatchMetricAlarm_MetricQuery>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warm_up_configuration: std::option::Option<std::vec::Vec<CloudwatchMetricAlarm_WarmUpConfiguration>>,
    dynamic: CloudwatchMetricAlarmDynamic,
}

struct CloudwatchMetricAlarm_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudwatchMetricAlarmData_>,
}

#[derive(Clone)]
pub struct CloudwatchMetricAlarm(std::rc::Rc<CloudwatchMetricAlarm_>);

impl CloudwatchMetricAlarm {
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

    #[doc = "Set the field `actions_enabled`.\n"]
    pub fn set_actions_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().actions_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alarm_actions`.\n"]
    pub fn set_alarm_actions(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().alarm_actions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alarm_description`.\n"]
    pub fn set_alarm_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().alarm_description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `comparison_operator`.\n"]
    pub fn set_comparison_operator(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().comparison_operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `datapoints_to_alarm`.\n"]
    pub fn set_datapoints_to_alarm(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().datapoints_to_alarm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().dimensions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `evaluate_low_sample_count_percentiles`.\n"]
    pub fn set_evaluate_low_sample_count_percentiles(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().evaluate_low_sample_count_percentiles = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `evaluation_interval`.\n"]
    pub fn set_evaluation_interval(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().evaluation_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `evaluation_periods`.\n"]
    pub fn set_evaluation_periods(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().evaluation_periods = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `extended_statistic`.\n"]
    pub fn set_extended_statistic(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().extended_statistic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `insufficient_data_actions`.\n"]
    pub fn set_insufficient_data_actions(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().insufficient_data_actions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric_name`.\n"]
    pub fn set_metric_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().metric_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ok_actions`.\n"]
    pub fn set_ok_actions(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().ok_actions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `period`.\n"]
    pub fn set_period(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().period = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `statistic`.\n"]
    pub fn set_statistic(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().statistic = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `threshold`.\n"]
    pub fn set_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `threshold_metric_id`.\n"]
    pub fn set_threshold_metric_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().threshold_metric_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `treat_missing_data`.\n"]
    pub fn set_treat_missing_data(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().treat_missing_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `evaluation_criteria`.\n"]
    pub fn set_evaluation_criteria(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchMetricAlarm_EvaluationCriteria>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().evaluation_criteria = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.evaluation_criteria = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `evaluation_window`.\n"]
    pub fn set_evaluation_window(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchMetricAlarm_EvaluationWindow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().evaluation_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.evaluation_window = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metric_query`.\n"]
    pub fn set_metric_query(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchMetricAlarm_MetricQuery>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metric_query = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metric_query = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `warm_up_configuration`.\n"]
    pub fn set_warm_up_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchMetricAlarm_WarmUpConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().warm_up_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.warm_up_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `actions_enabled` after provisioning.\n"]
    pub fn actions_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.actions_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alarm_actions` after provisioning.\n"]
    pub fn alarm_actions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.alarm_actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alarm_description` after provisioning.\n"]
    pub fn alarm_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alarm_description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alarm_name` after provisioning.\n"]
    pub fn alarm_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alarm_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comparison_operator` after provisioning.\n"]
    pub fn comparison_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison_operator", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `datapoints_to_alarm` after provisioning.\n"]
    pub fn datapoints_to_alarm(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.datapoints_to_alarm", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.dimensions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluate_low_sample_count_percentiles` after provisioning.\n"]
    pub fn evaluate_low_sample_count_percentiles(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.evaluate_low_sample_count_percentiles", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `evaluation_interval` after provisioning.\n"]
    pub fn evaluation_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluation_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluation_periods` after provisioning.\n"]
    pub fn evaluation_periods(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluation_periods", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `extended_statistic` after provisioning.\n"]
    pub fn extended_statistic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.extended_statistic", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `insufficient_data_actions` after provisioning.\n"]
    pub fn insufficient_data_actions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.insufficient_data_actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ok_actions` after provisioning.\n"]
    pub fn ok_actions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ok_actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `statistic` after provisioning.\n"]
    pub fn statistic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statistic", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `threshold` after provisioning.\n"]
    pub fn threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `threshold_metric_id` after provisioning.\n"]
    pub fn threshold_metric_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold_metric_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `treat_missing_data` after provisioning.\n"]
    pub fn treat_missing_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.treat_missing_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluation_criteria` after provisioning.\n"]
    pub fn evaluation_criteria(&self) -> terrars::ListRef<CloudwatchMetricAlarm_EvaluationCriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluation_criteria", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluation_window` after provisioning.\n"]
    pub fn evaluation_window(&self) -> terrars::ListRef<CloudwatchMetricAlarm_EvaluationWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluation_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_up_configuration` after provisioning.\n"]
    pub fn warm_up_configuration(&self) -> terrars::ListRef<CloudwatchMetricAlarm_WarmUpConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warm_up_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for CloudwatchMetricAlarm {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudwatchMetricAlarm { }

impl terrars::ToListMappable for CloudwatchMetricAlarm {
    type O = terrars::ListRef<CloudwatchMetricAlarmRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudwatchMetricAlarm_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudwatch_metric_alarm".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudwatchMetricAlarm {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub alarm_name: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchMetricAlarm {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudwatchMetricAlarm {
        let out = CloudwatchMetricAlarm(std::rc::Rc::new(CloudwatchMetricAlarm_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudwatchMetricAlarmData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                actions_enabled: core::default::Default::default(),
                alarm_actions: core::default::Default::default(),
                alarm_description: core::default::Default::default(),
                alarm_name: self.alarm_name,
                comparison_operator: core::default::Default::default(),
                datapoints_to_alarm: core::default::Default::default(),
                dimensions: core::default::Default::default(),
                evaluate_low_sample_count_percentiles: core::default::Default::default(),
                evaluation_interval: core::default::Default::default(),
                evaluation_periods: core::default::Default::default(),
                extended_statistic: core::default::Default::default(),
                id: core::default::Default::default(),
                insufficient_data_actions: core::default::Default::default(),
                metric_name: core::default::Default::default(),
                namespace: core::default::Default::default(),
                ok_actions: core::default::Default::default(),
                period: core::default::Default::default(),
                region: core::default::Default::default(),
                statistic: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                threshold: core::default::Default::default(),
                threshold_metric_id: core::default::Default::default(),
                treat_missing_data: core::default::Default::default(),
                unit: core::default::Default::default(),
                evaluation_criteria: core::default::Default::default(),
                evaluation_window: core::default::Default::default(),
                metric_query: core::default::Default::default(),
                warm_up_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudwatchMetricAlarmRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchMetricAlarmRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudwatchMetricAlarmRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `actions_enabled` after provisioning.\n"]
    pub fn actions_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.actions_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alarm_actions` after provisioning.\n"]
    pub fn alarm_actions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.alarm_actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alarm_description` after provisioning.\n"]
    pub fn alarm_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alarm_description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alarm_name` after provisioning.\n"]
    pub fn alarm_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alarm_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comparison_operator` after provisioning.\n"]
    pub fn comparison_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison_operator", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `datapoints_to_alarm` after provisioning.\n"]
    pub fn datapoints_to_alarm(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.datapoints_to_alarm", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.dimensions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluate_low_sample_count_percentiles` after provisioning.\n"]
    pub fn evaluate_low_sample_count_percentiles(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.evaluate_low_sample_count_percentiles", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `evaluation_interval` after provisioning.\n"]
    pub fn evaluation_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluation_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluation_periods` after provisioning.\n"]
    pub fn evaluation_periods(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluation_periods", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `extended_statistic` after provisioning.\n"]
    pub fn extended_statistic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.extended_statistic", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `insufficient_data_actions` after provisioning.\n"]
    pub fn insufficient_data_actions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.insufficient_data_actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ok_actions` after provisioning.\n"]
    pub fn ok_actions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ok_actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `statistic` after provisioning.\n"]
    pub fn statistic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statistic", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `threshold` after provisioning.\n"]
    pub fn threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `threshold_metric_id` after provisioning.\n"]
    pub fn threshold_metric_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold_metric_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `treat_missing_data` after provisioning.\n"]
    pub fn treat_missing_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.treat_missing_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluation_criteria` after provisioning.\n"]
    pub fn evaluation_criteria(&self) -> terrars::ListRef<CloudwatchMetricAlarm_EvaluationCriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluation_criteria", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluation_window` after provisioning.\n"]
    pub fn evaluation_window(&self) -> terrars::ListRef<CloudwatchMetricAlarm_EvaluationWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluation_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_up_configuration` after provisioning.\n"]
    pub fn warm_up_configuration(&self) -> terrars::ListRef<CloudwatchMetricAlarm_WarmUpConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warm_up_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchMetricAlarmDynamic {
    evaluation_criteria: std::option::Option<terrars::DynamicBlock<CloudwatchMetricAlarm_EvaluationCriteria>>,
    evaluation_window: std::option::Option<terrars::DynamicBlock<CloudwatchMetricAlarm_EvaluationWindow>>,
    metric_query: std::option::Option<terrars::DynamicBlock<CloudwatchMetricAlarm_MetricQuery>>,
    warm_up_configuration: std::option::Option<terrars::DynamicBlock<CloudwatchMetricAlarm_WarmUpConfiguration>>,
}

#[derive(Serialize)]
pub struct CloudwatchMetricAlarm_EvaluationCriteria_PromqlCriteria {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pending_period: std::option::Option<terrars::PrimField<f64>>,
    query: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recovery_period: std::option::Option<terrars::PrimField<f64>>,
}

impl CloudwatchMetricAlarm_EvaluationCriteria_PromqlCriteria {
    #[doc = "Set the field `pending_period`.\n"]
    pub fn set_pending_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pending_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `recovery_period`.\n"]
    pub fn set_recovery_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.recovery_period = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchMetricAlarm_EvaluationCriteria_PromqlCriteria {
    type O = terrars::BlockAssignable<CloudwatchMetricAlarm_EvaluationCriteria_PromqlCriteria>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchMetricAlarm_EvaluationCriteria_PromqlCriteria {
    #[doc = ""]
    pub query: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchMetricAlarm_EvaluationCriteria_PromqlCriteria {
    pub fn build(self) -> CloudwatchMetricAlarm_EvaluationCriteria_PromqlCriteria {
        CloudwatchMetricAlarm_EvaluationCriteria_PromqlCriteria {
            pending_period: core::default::Default::default(),
            query: self.query,
            recovery_period: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchMetricAlarm_EvaluationCriteria_PromqlCriteriaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchMetricAlarm_EvaluationCriteria_PromqlCriteriaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchMetricAlarm_EvaluationCriteria_PromqlCriteriaRef {
        CloudwatchMetricAlarm_EvaluationCriteria_PromqlCriteriaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchMetricAlarm_EvaluationCriteria_PromqlCriteriaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pending_period` after provisioning.\n"]
    pub fn pending_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pending_period", self.base))
    }

    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }

    #[doc = "Get a reference to the value of field `recovery_period` after provisioning.\n"]
    pub fn recovery_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recovery_period", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchMetricAlarm_EvaluationCriteriaDynamic {
    promql_criteria: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchMetricAlarm_EvaluationCriteria_PromqlCriteria>>,
}

#[derive(Serialize)]
pub struct CloudwatchMetricAlarm_EvaluationCriteria {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    promql_criteria: std::option::Option<std::vec::Vec<CloudwatchMetricAlarm_EvaluationCriteria_PromqlCriteria>>,
    dynamic: CloudwatchMetricAlarm_EvaluationCriteriaDynamic,
}

impl CloudwatchMetricAlarm_EvaluationCriteria {
    #[doc = "Set the field `promql_criteria`.\n"]
    pub fn set_promql_criteria(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchMetricAlarm_EvaluationCriteria_PromqlCriteria>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.promql_criteria = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.promql_criteria = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchMetricAlarm_EvaluationCriteria {
    type O = terrars::BlockAssignable<CloudwatchMetricAlarm_EvaluationCriteria>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchMetricAlarm_EvaluationCriteria {}

impl BuildCloudwatchMetricAlarm_EvaluationCriteria {
    pub fn build(self) -> CloudwatchMetricAlarm_EvaluationCriteria {
        CloudwatchMetricAlarm_EvaluationCriteria {
            promql_criteria: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchMetricAlarm_EvaluationCriteriaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchMetricAlarm_EvaluationCriteriaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchMetricAlarm_EvaluationCriteriaRef {
        CloudwatchMetricAlarm_EvaluationCriteriaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchMetricAlarm_EvaluationCriteriaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `promql_criteria` after provisioning.\n"]
    pub fn promql_criteria(&self) -> terrars::ListRef<CloudwatchMetricAlarm_EvaluationCriteria_PromqlCriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.promql_criteria", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchMetricAlarm_EvaluationWindow_WallClockWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timezone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchMetricAlarm_EvaluationWindow_WallClockWindow {
    #[doc = "Set the field `timezone`.\n"]
    pub fn set_timezone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timezone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchMetricAlarm_EvaluationWindow_WallClockWindow {
    type O = terrars::BlockAssignable<CloudwatchMetricAlarm_EvaluationWindow_WallClockWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchMetricAlarm_EvaluationWindow_WallClockWindow {}

impl BuildCloudwatchMetricAlarm_EvaluationWindow_WallClockWindow {
    pub fn build(self) -> CloudwatchMetricAlarm_EvaluationWindow_WallClockWindow {
        CloudwatchMetricAlarm_EvaluationWindow_WallClockWindow { timezone: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudwatchMetricAlarm_EvaluationWindow_WallClockWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchMetricAlarm_EvaluationWindow_WallClockWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchMetricAlarm_EvaluationWindow_WallClockWindowRef {
        CloudwatchMetricAlarm_EvaluationWindow_WallClockWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchMetricAlarm_EvaluationWindow_WallClockWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchMetricAlarm_EvaluationWindowDynamic {
    wall_clock_window: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchMetricAlarm_EvaluationWindow_WallClockWindow>>,
}

#[derive(Serialize)]
pub struct CloudwatchMetricAlarm_EvaluationWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wall_clock_window: std::option::Option<std::vec::Vec<CloudwatchMetricAlarm_EvaluationWindow_WallClockWindow>>,
    dynamic: CloudwatchMetricAlarm_EvaluationWindowDynamic,
}

impl CloudwatchMetricAlarm_EvaluationWindow {
    #[doc = "Set the field `wall_clock_window`.\n"]
    pub fn set_wall_clock_window(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchMetricAlarm_EvaluationWindow_WallClockWindow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.wall_clock_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.wall_clock_window = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchMetricAlarm_EvaluationWindow {
    type O = terrars::BlockAssignable<CloudwatchMetricAlarm_EvaluationWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchMetricAlarm_EvaluationWindow {}

impl BuildCloudwatchMetricAlarm_EvaluationWindow {
    pub fn build(self) -> CloudwatchMetricAlarm_EvaluationWindow {
        CloudwatchMetricAlarm_EvaluationWindow {
            wall_clock_window: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchMetricAlarm_EvaluationWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchMetricAlarm_EvaluationWindowRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchMetricAlarm_EvaluationWindowRef {
        CloudwatchMetricAlarm_EvaluationWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchMetricAlarm_EvaluationWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `wall_clock_window` after provisioning.\n"]
    pub fn wall_clock_window(&self) -> terrars::ListRef<CloudwatchMetricAlarm_EvaluationWindow_WallClockWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.wall_clock_window", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchMetricAlarm_MetricQuery_Metric {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    metric_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    period: terrars::PrimField<f64>,
    stat: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchMetricAlarm_MetricQuery_Metric {
    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dimensions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchMetricAlarm_MetricQuery_Metric {
    type O = terrars::BlockAssignable<CloudwatchMetricAlarm_MetricQuery_Metric>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchMetricAlarm_MetricQuery_Metric {
    #[doc = ""]
    pub metric_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub period: terrars::PrimField<f64>,
    #[doc = ""]
    pub stat: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchMetricAlarm_MetricQuery_Metric {
    pub fn build(self) -> CloudwatchMetricAlarm_MetricQuery_Metric {
        CloudwatchMetricAlarm_MetricQuery_Metric {
            dimensions: core::default::Default::default(),
            metric_name: self.metric_name,
            namespace: core::default::Default::default(),
            period: self.period,
            stat: self.stat,
            unit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchMetricAlarm_MetricQuery_MetricRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchMetricAlarm_MetricQuery_MetricRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchMetricAlarm_MetricQuery_MetricRef {
        CloudwatchMetricAlarm_MetricQuery_MetricRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchMetricAlarm_MetricQuery_MetricRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
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

    #[doc = "Get a reference to the value of field `stat` after provisioning.\n"]
    pub fn stat(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stat", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchMetricAlarm_MetricQueryDynamic {
    metric: std::option::Option<terrars::DynamicBlock<CloudwatchMetricAlarm_MetricQuery_Metric>>,
}

#[derive(Serialize)]
pub struct CloudwatchMetricAlarm_MetricQuery {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_data: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric: std::option::Option<std::vec::Vec<CloudwatchMetricAlarm_MetricQuery_Metric>>,
    dynamic: CloudwatchMetricAlarm_MetricQueryDynamic,
}

impl CloudwatchMetricAlarm_MetricQuery {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_id = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `period`.\n"]
    pub fn set_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `return_data`.\n"]
    pub fn set_return_data(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.return_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric`.\n"]
    pub fn set_metric(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchMetricAlarm_MetricQuery_Metric>>,
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

impl terrars::ToListMappable for CloudwatchMetricAlarm_MetricQuery {
    type O = terrars::BlockAssignable<CloudwatchMetricAlarm_MetricQuery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchMetricAlarm_MetricQuery {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchMetricAlarm_MetricQuery {
    pub fn build(self) -> CloudwatchMetricAlarm_MetricQuery {
        CloudwatchMetricAlarm_MetricQuery {
            account_id: core::default::Default::default(),
            expression: core::default::Default::default(),
            id: self.id,
            label: core::default::Default::default(),
            period: core::default::Default::default(),
            return_data: core::default::Default::default(),
            metric: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchMetricAlarm_MetricQueryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchMetricAlarm_MetricQueryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchMetricAlarm_MetricQueryRef {
        CloudwatchMetricAlarm_MetricQueryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchMetricAlarm_MetricQueryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
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

    #[doc = "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }

    #[doc = "Get a reference to the value of field `return_data` after provisioning.\n"]
    pub fn return_data(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return_data", self.base))
    }

    #[doc = "Get a reference to the value of field `metric` after provisioning.\n"]
    pub fn metric(&self) -> terrars::ListRef<CloudwatchMetricAlarm_MetricQuery_MetricRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchMetricAlarm_WarmUpConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    only_start_evaluating_after_warm_up_period_ends: std::option::Option<terrars::PrimField<bool>>,
    warm_up_period_duration_in_minutes: terrars::PrimField<f64>,
}

impl CloudwatchMetricAlarm_WarmUpConfiguration {
    #[doc = "Set the field `only_start_evaluating_after_warm_up_period_ends`.\n"]
    pub fn set_only_start_evaluating_after_warm_up_period_ends(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.only_start_evaluating_after_warm_up_period_ends = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchMetricAlarm_WarmUpConfiguration {
    type O = terrars::BlockAssignable<CloudwatchMetricAlarm_WarmUpConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchMetricAlarm_WarmUpConfiguration {
    #[doc = ""]
    pub warm_up_period_duration_in_minutes: terrars::PrimField<f64>,
}

impl BuildCloudwatchMetricAlarm_WarmUpConfiguration {
    pub fn build(self) -> CloudwatchMetricAlarm_WarmUpConfiguration {
        CloudwatchMetricAlarm_WarmUpConfiguration {
            only_start_evaluating_after_warm_up_period_ends: core::default::Default::default(),
            warm_up_period_duration_in_minutes: self.warm_up_period_duration_in_minutes,
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchMetricAlarm_WarmUpConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchMetricAlarm_WarmUpConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchMetricAlarm_WarmUpConfigurationRef {
        CloudwatchMetricAlarm_WarmUpConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchMetricAlarm_WarmUpConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `only_start_evaluating_after_warm_up_period_ends` after provisioning.\n"]
    pub fn only_start_evaluating_after_warm_up_period_ends(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.only_start_evaluating_after_warm_up_period_ends", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `warm_up_period_duration_in_minutes` after provisioning.\n"]
    pub fn warm_up_period_duration_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warm_up_period_duration_in_minutes", self.base))
    }
}
