#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ComputeoptimizerRecommendationPreferencesData_ {
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
    enhanced_infrastructure_metrics: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inferred_workload_types: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    look_back_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    savings_estimation_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_metrics_preference: std
    ::option
    ::Option<std::vec::Vec<ComputeoptimizerRecommendationPreferences_ExternalMetricsPreference>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_resource: std::option::Option<std::vec::Vec<ComputeoptimizerRecommendationPreferences_PreferredResource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<std::vec::Vec<ComputeoptimizerRecommendationPreferences_Scope>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    utilization_preference: std
    ::option
    ::Option<std::vec::Vec<ComputeoptimizerRecommendationPreferences_UtilizationPreference>>,
    dynamic: ComputeoptimizerRecommendationPreferencesDynamic,
}

struct ComputeoptimizerRecommendationPreferences_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeoptimizerRecommendationPreferencesData_>,
}

#[derive(Clone)]
pub struct ComputeoptimizerRecommendationPreferences(std::rc::Rc<ComputeoptimizerRecommendationPreferences_>);

impl ComputeoptimizerRecommendationPreferences {
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

    #[doc = "Set the field `enhanced_infrastructure_metrics`.\n"]
    pub fn set_enhanced_infrastructure_metrics(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().enhanced_infrastructure_metrics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inferred_workload_types`.\n"]
    pub fn set_inferred_workload_types(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().inferred_workload_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `look_back_period`.\n"]
    pub fn set_look_back_period(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().look_back_period = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `savings_estimation_mode`.\n"]
    pub fn set_savings_estimation_mode(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().savings_estimation_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_metrics_preference`.\n"]
    pub fn set_external_metrics_preference(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeoptimizerRecommendationPreferences_ExternalMetricsPreference>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().external_metrics_preference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.external_metrics_preference = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `preferred_resource`.\n"]
    pub fn set_preferred_resource(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeoptimizerRecommendationPreferences_PreferredResource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().preferred_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.preferred_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeoptimizerRecommendationPreferences_Scope>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scope = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scope = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `utilization_preference`.\n"]
    pub fn set_utilization_preference(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeoptimizerRecommendationPreferences_UtilizationPreference>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().utilization_preference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.utilization_preference = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `enhanced_infrastructure_metrics` after provisioning.\n"]
    pub fn enhanced_infrastructure_metrics(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enhanced_infrastructure_metrics", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inferred_workload_types` after provisioning.\n"]
    pub fn inferred_workload_types(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inferred_workload_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `look_back_period` after provisioning.\n"]
    pub fn look_back_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.look_back_period", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `savings_estimation_mode` after provisioning.\n"]
    pub fn savings_estimation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.savings_estimation_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_metrics_preference` after provisioning.\n"]
    pub fn external_metrics_preference(
        &self,
    ) -> terrars::ListRef<ComputeoptimizerRecommendationPreferences_ExternalMetricsPreferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_metrics_preference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_resource` after provisioning.\n"]
    pub fn preferred_resource(&self) -> terrars::ListRef<ComputeoptimizerRecommendationPreferences_PreferredResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferred_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::ListRef<ComputeoptimizerRecommendationPreferences_ScopeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `utilization_preference` after provisioning.\n"]
    pub fn utilization_preference(
        &self,
    ) -> terrars::ListRef<ComputeoptimizerRecommendationPreferences_UtilizationPreferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.utilization_preference", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeoptimizerRecommendationPreferences {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeoptimizerRecommendationPreferences { }

impl terrars::ToListMappable for ComputeoptimizerRecommendationPreferences {
    type O = terrars::ListRef<ComputeoptimizerRecommendationPreferencesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeoptimizerRecommendationPreferences_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_computeoptimizer_recommendation_preferences".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeoptimizerRecommendationPreferences {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub resource_type: terrars::PrimField<std::string::String>,
}

impl BuildComputeoptimizerRecommendationPreferences {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeoptimizerRecommendationPreferences {
        let out =
            ComputeoptimizerRecommendationPreferences(std::rc::Rc::new(ComputeoptimizerRecommendationPreferences_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(ComputeoptimizerRecommendationPreferencesData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    provisioner: core::default::Default::default(),
                    enhanced_infrastructure_metrics: core::default::Default::default(),
                    inferred_workload_types: core::default::Default::default(),
                    look_back_period: core::default::Default::default(),
                    region: core::default::Default::default(),
                    resource_type: self.resource_type,
                    savings_estimation_mode: core::default::Default::default(),
                    external_metrics_preference: core::default::Default::default(),
                    preferred_resource: core::default::Default::default(),
                    scope: core::default::Default::default(),
                    utilization_preference: core::default::Default::default(),
                    dynamic: core::default::Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeoptimizerRecommendationPreferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeoptimizerRecommendationPreferencesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeoptimizerRecommendationPreferencesRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enhanced_infrastructure_metrics` after provisioning.\n"]
    pub fn enhanced_infrastructure_metrics(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enhanced_infrastructure_metrics", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inferred_workload_types` after provisioning.\n"]
    pub fn inferred_workload_types(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inferred_workload_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `look_back_period` after provisioning.\n"]
    pub fn look_back_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.look_back_period", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `savings_estimation_mode` after provisioning.\n"]
    pub fn savings_estimation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.savings_estimation_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_metrics_preference` after provisioning.\n"]
    pub fn external_metrics_preference(
        &self,
    ) -> terrars::ListRef<ComputeoptimizerRecommendationPreferences_ExternalMetricsPreferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_metrics_preference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_resource` after provisioning.\n"]
    pub fn preferred_resource(&self) -> terrars::ListRef<ComputeoptimizerRecommendationPreferences_PreferredResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferred_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::ListRef<ComputeoptimizerRecommendationPreferences_ScopeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `utilization_preference` after provisioning.\n"]
    pub fn utilization_preference(
        &self,
    ) -> terrars::ListRef<ComputeoptimizerRecommendationPreferences_UtilizationPreferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.utilization_preference", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeoptimizerRecommendationPreferencesDynamic {
    external_metrics_preference: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeoptimizerRecommendationPreferences_ExternalMetricsPreference>>,
    preferred_resource: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeoptimizerRecommendationPreferences_PreferredResource>>,
    scope: std::option::Option<terrars::DynamicBlock<ComputeoptimizerRecommendationPreferences_Scope>>,
    utilization_preference: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeoptimizerRecommendationPreferences_UtilizationPreference>>,
}

#[derive(Serialize)]
pub struct ComputeoptimizerRecommendationPreferences_ExternalMetricsPreference {
    source: terrars::PrimField<std::string::String>,
}

impl ComputeoptimizerRecommendationPreferences_ExternalMetricsPreference { }

impl terrars::ToListMappable for ComputeoptimizerRecommendationPreferences_ExternalMetricsPreference {
    type O = terrars::BlockAssignable<ComputeoptimizerRecommendationPreferences_ExternalMetricsPreference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeoptimizerRecommendationPreferences_ExternalMetricsPreference {
    #[doc = ""]
    pub source: terrars::PrimField<std::string::String>,
}

impl BuildComputeoptimizerRecommendationPreferences_ExternalMetricsPreference {
    pub fn build(self) -> ComputeoptimizerRecommendationPreferences_ExternalMetricsPreference {
        ComputeoptimizerRecommendationPreferences_ExternalMetricsPreference { source: self.source }
    }
}

#[allow(dead_code)]
pub struct ComputeoptimizerRecommendationPreferences_ExternalMetricsPreferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeoptimizerRecommendationPreferences_ExternalMetricsPreferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeoptimizerRecommendationPreferences_ExternalMetricsPreferenceRef {
        ComputeoptimizerRecommendationPreferences_ExternalMetricsPreferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeoptimizerRecommendationPreferences_ExternalMetricsPreferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeoptimizerRecommendationPreferences_PreferredResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
}

impl ComputeoptimizerRecommendationPreferences_PreferredResource {
    #[doc = "Set the field `exclude_list`.\n"]
    pub fn set_exclude_list(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_list`.\n"]
    pub fn set_include_list(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_list = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeoptimizerRecommendationPreferences_PreferredResource {
    type O = terrars::BlockAssignable<ComputeoptimizerRecommendationPreferences_PreferredResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeoptimizerRecommendationPreferences_PreferredResource {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeoptimizerRecommendationPreferences_PreferredResource {
    pub fn build(self) -> ComputeoptimizerRecommendationPreferences_PreferredResource {
        ComputeoptimizerRecommendationPreferences_PreferredResource {
            exclude_list: core::default::Default::default(),
            include_list: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeoptimizerRecommendationPreferences_PreferredResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeoptimizerRecommendationPreferences_PreferredResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeoptimizerRecommendationPreferences_PreferredResourceRef {
        ComputeoptimizerRecommendationPreferences_PreferredResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeoptimizerRecommendationPreferences_PreferredResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclude_list` after provisioning.\n"]
    pub fn exclude_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exclude_list", self.base))
    }

    #[doc = "Get a reference to the value of field `include_list` after provisioning.\n"]
    pub fn include_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.include_list", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeoptimizerRecommendationPreferences_Scope {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl ComputeoptimizerRecommendationPreferences_Scope { }

impl terrars::ToListMappable for ComputeoptimizerRecommendationPreferences_Scope {
    type O = terrars::BlockAssignable<ComputeoptimizerRecommendationPreferences_Scope>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeoptimizerRecommendationPreferences_Scope {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildComputeoptimizerRecommendationPreferences_Scope {
    pub fn build(self) -> ComputeoptimizerRecommendationPreferences_Scope {
        ComputeoptimizerRecommendationPreferences_Scope {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeoptimizerRecommendationPreferences_ScopeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeoptimizerRecommendationPreferences_ScopeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeoptimizerRecommendationPreferences_ScopeRef {
        ComputeoptimizerRecommendationPreferences_ScopeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeoptimizerRecommendationPreferences_ScopeRef {
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

#[derive(Serialize)]
pub struct ComputeoptimizerRecommendationPreferences_UtilizationPreference_MetricParameters {
    headroom: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threshold: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeoptimizerRecommendationPreferences_UtilizationPreference_MetricParameters {
    #[doc = "Set the field `threshold`.\n"]
    pub fn set_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeoptimizerRecommendationPreferences_UtilizationPreference_MetricParameters {
    type O =
        terrars::BlockAssignable<ComputeoptimizerRecommendationPreferences_UtilizationPreference_MetricParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeoptimizerRecommendationPreferences_UtilizationPreference_MetricParameters {
    #[doc = ""]
    pub headroom: terrars::PrimField<std::string::String>,
}

impl BuildComputeoptimizerRecommendationPreferences_UtilizationPreference_MetricParameters {
    pub fn build(self) -> ComputeoptimizerRecommendationPreferences_UtilizationPreference_MetricParameters {
        ComputeoptimizerRecommendationPreferences_UtilizationPreference_MetricParameters {
            headroom: self.headroom,
            threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeoptimizerRecommendationPreferences_UtilizationPreference_MetricParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeoptimizerRecommendationPreferences_UtilizationPreference_MetricParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeoptimizerRecommendationPreferences_UtilizationPreference_MetricParametersRef {
        ComputeoptimizerRecommendationPreferences_UtilizationPreference_MetricParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeoptimizerRecommendationPreferences_UtilizationPreference_MetricParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `headroom` after provisioning.\n"]
    pub fn headroom(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.headroom", self.base))
    }

    #[doc = "Get a reference to the value of field `threshold` after provisioning.\n"]
    pub fn threshold(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeoptimizerRecommendationPreferences_UtilizationPreferenceDynamic {
    metric_parameters: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeoptimizerRecommendationPreferences_UtilizationPreference_MetricParameters>,
    >,
}

#[derive(Serialize)]
pub struct ComputeoptimizerRecommendationPreferences_UtilizationPreference {
    metric_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_parameters: std
    ::option
    ::Option<std::vec::Vec<ComputeoptimizerRecommendationPreferences_UtilizationPreference_MetricParameters>>,
    dynamic: ComputeoptimizerRecommendationPreferences_UtilizationPreferenceDynamic,
}

impl ComputeoptimizerRecommendationPreferences_UtilizationPreference {
    #[doc = "Set the field `metric_parameters`.\n"]
    pub fn set_metric_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeoptimizerRecommendationPreferences_UtilizationPreference_MetricParameters,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeoptimizerRecommendationPreferences_UtilizationPreference {
    type O = terrars::BlockAssignable<ComputeoptimizerRecommendationPreferences_UtilizationPreference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeoptimizerRecommendationPreferences_UtilizationPreference {
    #[doc = ""]
    pub metric_name: terrars::PrimField<std::string::String>,
}

impl BuildComputeoptimizerRecommendationPreferences_UtilizationPreference {
    pub fn build(self) -> ComputeoptimizerRecommendationPreferences_UtilizationPreference {
        ComputeoptimizerRecommendationPreferences_UtilizationPreference {
            metric_name: self.metric_name,
            metric_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeoptimizerRecommendationPreferences_UtilizationPreferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeoptimizerRecommendationPreferences_UtilizationPreferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeoptimizerRecommendationPreferences_UtilizationPreferenceRef {
        ComputeoptimizerRecommendationPreferences_UtilizationPreferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeoptimizerRecommendationPreferences_UtilizationPreferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_parameters` after provisioning.\n"]
    pub fn metric_parameters(
        &self,
    ) -> terrars::ListRef<ComputeoptimizerRecommendationPreferences_UtilizationPreference_MetricParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_parameters", self.base))
    }
}
