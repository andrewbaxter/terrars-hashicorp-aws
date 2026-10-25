#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct PrometheusAnomalyDetectorData_ {
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
    alias: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_interval_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    workspace_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<PrometheusAnomalyDetector_Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    missing_data_action: std::option::Option<std::vec::Vec<PrometheusAnomalyDetector_MissingDataAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<PrometheusAnomalyDetector_Timeouts>,
    dynamic: PrometheusAnomalyDetectorDynamic,
}

struct PrometheusAnomalyDetector_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PrometheusAnomalyDetectorData_>,
}

#[derive(Clone)]
pub struct PrometheusAnomalyDetector(std::rc::Rc<PrometheusAnomalyDetector_>);

impl PrometheusAnomalyDetector {
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

    #[doc = "Set the field `evaluation_interval_in_seconds`.\n"]
    pub fn set_evaluation_interval_in_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().evaluation_interval_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrometheusAnomalyDetector_Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `missing_data_action`.\n"]
    pub fn set_missing_data_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrometheusAnomalyDetector_MissingDataAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().missing_data_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.missing_data_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<PrometheusAnomalyDetector_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluation_interval_in_seconds` after provisioning.\n"]
    pub fn evaluation_interval_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.evaluation_interval_in_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_id` after provisioning.\n"]
    pub fn workspace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<PrometheusAnomalyDetector_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `missing_data_action` after provisioning.\n"]
    pub fn missing_data_action(&self) -> terrars::ListRef<PrometheusAnomalyDetector_MissingDataActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.missing_data_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrometheusAnomalyDetector_TimeoutsRef {
        PrometheusAnomalyDetector_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for PrometheusAnomalyDetector {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PrometheusAnomalyDetector { }

impl terrars::ToListMappable for PrometheusAnomalyDetector {
    type O = terrars::ListRef<PrometheusAnomalyDetectorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PrometheusAnomalyDetector_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_prometheus_anomaly_detector".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPrometheusAnomalyDetector {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub alias: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub workspace_id: terrars::PrimField<std::string::String>,
}

impl BuildPrometheusAnomalyDetector {
    pub fn build(self, stack: &mut terrars::Stack) -> PrometheusAnomalyDetector {
        let out = PrometheusAnomalyDetector(std::rc::Rc::new(PrometheusAnomalyDetector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PrometheusAnomalyDetectorData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                alias: self.alias,
                evaluation_interval_in_seconds: core::default::Default::default(),
                labels: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                workspace_id: self.workspace_id,
                configuration: core::default::Default::default(),
                missing_data_action: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PrometheusAnomalyDetectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusAnomalyDetectorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PrometheusAnomalyDetectorRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluation_interval_in_seconds` after provisioning.\n"]
    pub fn evaluation_interval_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.evaluation_interval_in_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_id` after provisioning.\n"]
    pub fn workspace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<PrometheusAnomalyDetector_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `missing_data_action` after provisioning.\n"]
    pub fn missing_data_action(&self) -> terrars::ListRef<PrometheusAnomalyDetector_MissingDataActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.missing_data_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrometheusAnomalyDetector_TimeoutsRef {
        PrometheusAnomalyDetector_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct PrometheusAnomalyDetectorDynamic {
    configuration: std::option::Option<terrars::DynamicBlock<PrometheusAnomalyDetector_Configuration>>,
    missing_data_action: std::option::Option<terrars::DynamicBlock<PrometheusAnomalyDetector_MissingDataAction>>,
}

#[derive(Serialize)]
pub struct PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromAbove {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ratio: std::option::Option<terrars::PrimField<f64>>,
}

impl PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromAbove {
    #[doc = "Set the field `amount`.\n"]
    pub fn set_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ratio`.\n"]
    pub fn set_ratio(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ratio = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromAbove {
    type O =
        terrars
        ::BlockAssignable<PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromAbove>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromAbove {}

impl BuildPrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromAbove {
    pub fn build(self) -> PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromAbove {
        PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromAbove {
            amount: core::default::Default::default(),
            ratio: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromAboveRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromAboveRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromAboveRef {
        PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromAboveRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromAboveRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amount` after provisioning.\n"]
    pub fn amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount", self.base))
    }

    #[doc = "Get a reference to the value of field `ratio` after provisioning.\n"]
    pub fn ratio(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ratio", self.base))
    }
}

#[derive(Serialize)]
pub struct PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromBelow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ratio: std::option::Option<terrars::PrimField<f64>>,
}

impl PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromBelow {
    #[doc = "Set the field `amount`.\n"]
    pub fn set_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ratio`.\n"]
    pub fn set_ratio(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ratio = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromBelow {
    type O =
        terrars
        ::BlockAssignable<PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromBelow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromBelow {}

impl BuildPrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromBelow {
    pub fn build(self) -> PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromBelow {
        PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromBelow {
            amount: core::default::Default::default(),
            ratio: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromBelowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromBelowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromBelowRef {
        PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromBelowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromBelowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amount` after provisioning.\n"]
    pub fn amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount", self.base))
    }

    #[doc = "Get a reference to the value of field `ratio` after provisioning.\n"]
    pub fn ratio(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ratio", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrometheusAnomalyDetector_Configuration_RandomCutForestDynamic {
    ignore_near_expected_from_above: std
    ::option
    ::Option<
        terrars::DynamicBlock<PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromAbove>,
    >,
    ignore_near_expected_from_below: std
    ::option
    ::Option<
        terrars::DynamicBlock<PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromBelow>,
    >,
}

#[derive(Serialize)]
pub struct PrometheusAnomalyDetector_Configuration_RandomCutForest {
    query: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shingle_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_near_expected_from_above: std
    ::option
    ::Option<std::vec::Vec<PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromAbove>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_near_expected_from_below: std
    ::option
    ::Option<std::vec::Vec<PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromBelow>>,
    dynamic: PrometheusAnomalyDetector_Configuration_RandomCutForestDynamic,
}

impl PrometheusAnomalyDetector_Configuration_RandomCutForest {
    #[doc = "Set the field `sample_size`.\n"]
    pub fn set_sample_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sample_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shingle_size`.\n"]
    pub fn set_shingle_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.shingle_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_near_expected_from_above`.\n"]
    pub fn set_ignore_near_expected_from_above(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromAbove,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ignore_near_expected_from_above = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ignore_near_expected_from_above = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ignore_near_expected_from_below`.\n"]
    pub fn set_ignore_near_expected_from_below(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromBelow,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ignore_near_expected_from_below = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ignore_near_expected_from_below = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrometheusAnomalyDetector_Configuration_RandomCutForest {
    type O = terrars::BlockAssignable<PrometheusAnomalyDetector_Configuration_RandomCutForest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusAnomalyDetector_Configuration_RandomCutForest {
    #[doc = ""]
    pub query: terrars::PrimField<std::string::String>,
}

impl BuildPrometheusAnomalyDetector_Configuration_RandomCutForest {
    pub fn build(self) -> PrometheusAnomalyDetector_Configuration_RandomCutForest {
        PrometheusAnomalyDetector_Configuration_RandomCutForest {
            query: self.query,
            sample_size: core::default::Default::default(),
            shingle_size: core::default::Default::default(),
            ignore_near_expected_from_above: core::default::Default::default(),
            ignore_near_expected_from_below: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrometheusAnomalyDetector_Configuration_RandomCutForestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusAnomalyDetector_Configuration_RandomCutForestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrometheusAnomalyDetector_Configuration_RandomCutForestRef {
        PrometheusAnomalyDetector_Configuration_RandomCutForestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusAnomalyDetector_Configuration_RandomCutForestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }

    #[doc = "Get a reference to the value of field `sample_size` after provisioning.\n"]
    pub fn sample_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sample_size", self.base))
    }

    #[doc = "Get a reference to the value of field `shingle_size` after provisioning.\n"]
    pub fn shingle_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shingle_size", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_near_expected_from_above` after provisioning.\n"]
    pub fn ignore_near_expected_from_above(
        &self,
    ) -> terrars::ListRef<PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromAboveRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ignore_near_expected_from_above", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_near_expected_from_below` after provisioning.\n"]
    pub fn ignore_near_expected_from_below(
        &self,
    ) -> terrars::ListRef<PrometheusAnomalyDetector_Configuration_RandomCutForest_IgnoreNearExpectedFromBelowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ignore_near_expected_from_below", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrometheusAnomalyDetector_ConfigurationDynamic {
    random_cut_forest: std
    ::option
    ::Option<terrars::DynamicBlock<PrometheusAnomalyDetector_Configuration_RandomCutForest>>,
}

#[derive(Serialize)]
pub struct PrometheusAnomalyDetector_Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    random_cut_forest: std::option::Option<std::vec::Vec<PrometheusAnomalyDetector_Configuration_RandomCutForest>>,
    dynamic: PrometheusAnomalyDetector_ConfigurationDynamic,
}

impl PrometheusAnomalyDetector_Configuration {
    #[doc = "Set the field `random_cut_forest`.\n"]
    pub fn set_random_cut_forest(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PrometheusAnomalyDetector_Configuration_RandomCutForest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.random_cut_forest = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.random_cut_forest = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrometheusAnomalyDetector_Configuration {
    type O = terrars::BlockAssignable<PrometheusAnomalyDetector_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusAnomalyDetector_Configuration {}

impl BuildPrometheusAnomalyDetector_Configuration {
    pub fn build(self) -> PrometheusAnomalyDetector_Configuration {
        PrometheusAnomalyDetector_Configuration {
            random_cut_forest: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrometheusAnomalyDetector_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusAnomalyDetector_ConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrometheusAnomalyDetector_ConfigurationRef {
        PrometheusAnomalyDetector_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusAnomalyDetector_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `random_cut_forest` after provisioning.\n"]
    pub fn random_cut_forest(&self) -> terrars::ListRef<PrometheusAnomalyDetector_Configuration_RandomCutForestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.random_cut_forest", self.base))
    }
}

#[derive(Serialize)]
pub struct PrometheusAnomalyDetector_MissingDataAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mark_as_anomaly: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip: std::option::Option<terrars::PrimField<bool>>,
}

impl PrometheusAnomalyDetector_MissingDataAction {
    #[doc = "Set the field `mark_as_anomaly`.\n"]
    pub fn set_mark_as_anomaly(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.mark_as_anomaly = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `skip`.\n"]
    pub fn set_skip(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.skip = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrometheusAnomalyDetector_MissingDataAction {
    type O = terrars::BlockAssignable<PrometheusAnomalyDetector_MissingDataAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusAnomalyDetector_MissingDataAction {}

impl BuildPrometheusAnomalyDetector_MissingDataAction {
    pub fn build(self) -> PrometheusAnomalyDetector_MissingDataAction {
        PrometheusAnomalyDetector_MissingDataAction {
            mark_as_anomaly: core::default::Default::default(),
            skip: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrometheusAnomalyDetector_MissingDataActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusAnomalyDetector_MissingDataActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrometheusAnomalyDetector_MissingDataActionRef {
        PrometheusAnomalyDetector_MissingDataActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusAnomalyDetector_MissingDataActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mark_as_anomaly` after provisioning.\n"]
    pub fn mark_as_anomaly(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mark_as_anomaly", self.base))
    }

    #[doc = "Get a reference to the value of field `skip` after provisioning.\n"]
    pub fn skip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip", self.base))
    }
}

#[derive(Serialize)]
pub struct PrometheusAnomalyDetector_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrometheusAnomalyDetector_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrometheusAnomalyDetector_Timeouts {
    type O = terrars::BlockAssignable<PrometheusAnomalyDetector_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusAnomalyDetector_Timeouts {}

impl BuildPrometheusAnomalyDetector_Timeouts {
    pub fn build(self) -> PrometheusAnomalyDetector_Timeouts {
        PrometheusAnomalyDetector_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrometheusAnomalyDetector_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusAnomalyDetector_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrometheusAnomalyDetector_TimeoutsRef {
        PrometheusAnomalyDetector_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusAnomalyDetector_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
