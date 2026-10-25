#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct PrometheusWorkspaceConfigurationData_ {
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
    out_of_order_time_window_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_period_in_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_query_offset_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    workspace_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits_per_label_set: std::option::Option<std::vec::Vec<PrometheusWorkspaceConfiguration_LimitsPerLabelSet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<PrometheusWorkspaceConfiguration_Timeouts>,
    dynamic: PrometheusWorkspaceConfigurationDynamic,
}

struct PrometheusWorkspaceConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PrometheusWorkspaceConfigurationData_>,
}

#[derive(Clone)]
pub struct PrometheusWorkspaceConfiguration(std::rc::Rc<PrometheusWorkspaceConfiguration_>);

impl PrometheusWorkspaceConfiguration {
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

    #[doc = "Set the field `out_of_order_time_window_in_seconds`.\n"]
    pub fn set_out_of_order_time_window_in_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().out_of_order_time_window_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention_period_in_days`.\n"]
    pub fn set_retention_period_in_days(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retention_period_in_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_query_offset_in_seconds`.\n"]
    pub fn set_rule_query_offset_in_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().rule_query_offset_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `limits_per_label_set`.\n"]
    pub fn set_limits_per_label_set(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrometheusWorkspaceConfiguration_LimitsPerLabelSet>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().limits_per_label_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.limits_per_label_set = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<PrometheusWorkspaceConfiguration_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `out_of_order_time_window_in_seconds` after provisioning.\n"]
    pub fn out_of_order_time_window_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.out_of_order_time_window_in_seconds", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_period_in_days` after provisioning.\n"]
    pub fn retention_period_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_period_in_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_query_offset_in_seconds` after provisioning.\n"]
    pub fn rule_query_offset_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_query_offset_in_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_id` after provisioning.\n"]
    pub fn workspace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `limits_per_label_set` after provisioning.\n"]
    pub fn limits_per_label_set(&self) -> terrars::ListRef<PrometheusWorkspaceConfiguration_LimitsPerLabelSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.limits_per_label_set", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrometheusWorkspaceConfiguration_TimeoutsRef {
        PrometheusWorkspaceConfiguration_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for PrometheusWorkspaceConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PrometheusWorkspaceConfiguration { }

impl terrars::ToListMappable for PrometheusWorkspaceConfiguration {
    type O = terrars::ListRef<PrometheusWorkspaceConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PrometheusWorkspaceConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_prometheus_workspace_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPrometheusWorkspaceConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub workspace_id: terrars::PrimField<std::string::String>,
}

impl BuildPrometheusWorkspaceConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> PrometheusWorkspaceConfiguration {
        let out = PrometheusWorkspaceConfiguration(std::rc::Rc::new(PrometheusWorkspaceConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PrometheusWorkspaceConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                out_of_order_time_window_in_seconds: core::default::Default::default(),
                region: core::default::Default::default(),
                retention_period_in_days: core::default::Default::default(),
                rule_query_offset_in_seconds: core::default::Default::default(),
                workspace_id: self.workspace_id,
                limits_per_label_set: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PrometheusWorkspaceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusWorkspaceConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PrometheusWorkspaceConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `out_of_order_time_window_in_seconds` after provisioning.\n"]
    pub fn out_of_order_time_window_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.out_of_order_time_window_in_seconds", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_period_in_days` after provisioning.\n"]
    pub fn retention_period_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_period_in_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_query_offset_in_seconds` after provisioning.\n"]
    pub fn rule_query_offset_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_query_offset_in_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_id` after provisioning.\n"]
    pub fn workspace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `limits_per_label_set` after provisioning.\n"]
    pub fn limits_per_label_set(&self) -> terrars::ListRef<PrometheusWorkspaceConfiguration_LimitsPerLabelSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.limits_per_label_set", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrometheusWorkspaceConfiguration_TimeoutsRef {
        PrometheusWorkspaceConfiguration_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct PrometheusWorkspaceConfigurationDynamic {
    limits_per_label_set: std
    ::option
    ::Option<terrars::DynamicBlock<PrometheusWorkspaceConfiguration_LimitsPerLabelSet>>,
}

#[derive(Serialize)]
pub struct PrometheusWorkspaceConfiguration_LimitsPerLabelSet_Limits {
    max_series: terrars::PrimField<f64>,
}

impl PrometheusWorkspaceConfiguration_LimitsPerLabelSet_Limits { }

impl terrars::ToListMappable for PrometheusWorkspaceConfiguration_LimitsPerLabelSet_Limits {
    type O = terrars::BlockAssignable<PrometheusWorkspaceConfiguration_LimitsPerLabelSet_Limits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusWorkspaceConfiguration_LimitsPerLabelSet_Limits {
    #[doc = ""]
    pub max_series: terrars::PrimField<f64>,
}

impl BuildPrometheusWorkspaceConfiguration_LimitsPerLabelSet_Limits {
    pub fn build(self) -> PrometheusWorkspaceConfiguration_LimitsPerLabelSet_Limits {
        PrometheusWorkspaceConfiguration_LimitsPerLabelSet_Limits { max_series: self.max_series }
    }
}

#[allow(dead_code)]
pub struct PrometheusWorkspaceConfiguration_LimitsPerLabelSet_LimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusWorkspaceConfiguration_LimitsPerLabelSet_LimitsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrometheusWorkspaceConfiguration_LimitsPerLabelSet_LimitsRef {
        PrometheusWorkspaceConfiguration_LimitsPerLabelSet_LimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusWorkspaceConfiguration_LimitsPerLabelSet_LimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_series` after provisioning.\n"]
    pub fn max_series(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_series", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrometheusWorkspaceConfiguration_LimitsPerLabelSetDynamic {
    limits: std::option::Option<terrars::DynamicBlock<PrometheusWorkspaceConfiguration_LimitsPerLabelSet_Limits>>,
}

#[derive(Serialize)]
pub struct PrometheusWorkspaceConfiguration_LimitsPerLabelSet {
    label_set: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<std::vec::Vec<PrometheusWorkspaceConfiguration_LimitsPerLabelSet_Limits>>,
    dynamic: PrometheusWorkspaceConfiguration_LimitsPerLabelSetDynamic,
}

impl PrometheusWorkspaceConfiguration_LimitsPerLabelSet {
    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PrometheusWorkspaceConfiguration_LimitsPerLabelSet_Limits>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.limits = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.limits = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrometheusWorkspaceConfiguration_LimitsPerLabelSet {
    type O = terrars::BlockAssignable<PrometheusWorkspaceConfiguration_LimitsPerLabelSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusWorkspaceConfiguration_LimitsPerLabelSet {
    #[doc = ""]
    pub label_set: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildPrometheusWorkspaceConfiguration_LimitsPerLabelSet {
    pub fn build(self) -> PrometheusWorkspaceConfiguration_LimitsPerLabelSet {
        PrometheusWorkspaceConfiguration_LimitsPerLabelSet {
            label_set: self.label_set,
            limits: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrometheusWorkspaceConfiguration_LimitsPerLabelSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusWorkspaceConfiguration_LimitsPerLabelSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrometheusWorkspaceConfiguration_LimitsPerLabelSetRef {
        PrometheusWorkspaceConfiguration_LimitsPerLabelSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusWorkspaceConfiguration_LimitsPerLabelSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `label_set` after provisioning.\n"]
    pub fn label_set(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.label_set", self.base))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> terrars::ListRef<PrometheusWorkspaceConfiguration_LimitsPerLabelSet_LimitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }
}

#[derive(Serialize)]
pub struct PrometheusWorkspaceConfiguration_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrometheusWorkspaceConfiguration_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrometheusWorkspaceConfiguration_Timeouts {
    type O = terrars::BlockAssignable<PrometheusWorkspaceConfiguration_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusWorkspaceConfiguration_Timeouts {}

impl BuildPrometheusWorkspaceConfiguration_Timeouts {
    pub fn build(self) -> PrometheusWorkspaceConfiguration_Timeouts {
        PrometheusWorkspaceConfiguration_Timeouts {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrometheusWorkspaceConfiguration_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusWorkspaceConfiguration_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrometheusWorkspaceConfiguration_TimeoutsRef {
        PrometheusWorkspaceConfiguration_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusWorkspaceConfiguration_TimeoutsRef {
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
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
