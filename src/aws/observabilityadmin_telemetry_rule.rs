#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ObservabilityadminTelemetryRuleData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    rule_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<ObservabilityadminTelemetryRule_Rule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ObservabilityadminTelemetryRule_Timeouts>,
    dynamic: ObservabilityadminTelemetryRuleDynamic,
}

struct ObservabilityadminTelemetryRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ObservabilityadminTelemetryRuleData_>,
}

#[derive(Clone)]
pub struct ObservabilityadminTelemetryRule(std::rc::Rc<ObservabilityadminTelemetryRule_>);

impl ObservabilityadminTelemetryRule {
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

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ObservabilityadminTelemetryRule_Rule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ObservabilityadminTelemetryRule_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_arn` after provisioning.\n"]
    pub fn rule_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<ObservabilityadminTelemetryRule_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ObservabilityadminTelemetryRule_TimeoutsRef {
        ObservabilityadminTelemetryRule_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ObservabilityadminTelemetryRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ObservabilityadminTelemetryRule { }

impl terrars::ToListMappable for ObservabilityadminTelemetryRule {
    type O = terrars::ListRef<ObservabilityadminTelemetryRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ObservabilityadminTelemetryRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_observabilityadmin_telemetry_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildObservabilityadminTelemetryRule {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub rule_name: terrars::PrimField<std::string::String>,
}

impl BuildObservabilityadminTelemetryRule {
    pub fn build(self, stack: &mut terrars::Stack) -> ObservabilityadminTelemetryRule {
        let out = ObservabilityadminTelemetryRule(std::rc::Rc::new(ObservabilityadminTelemetryRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ObservabilityadminTelemetryRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                region: core::default::Default::default(),
                rule_name: self.rule_name,
                tags: core::default::Default::default(),
                rule: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ObservabilityadminTelemetryRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservabilityadminTelemetryRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ObservabilityadminTelemetryRuleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_arn` after provisioning.\n"]
    pub fn rule_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<ObservabilityadminTelemetryRule_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ObservabilityadminTelemetryRule_TimeoutsRef {
        ObservabilityadminTelemetryRule_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct ObservabilityadminTelemetryRuleDynamic {
    rule: std::option::Option<terrars::DynamicBlock<ObservabilityadminTelemetryRule_Rule>>,
}

#[derive(Serialize)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors_FieldSelectors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ends_with: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    equals: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    field: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_ends_with: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_equals: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_starts_with: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    starts_with: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors_FieldSelectors {
    #[doc = "Set the field `ends_with`.\n"]
    pub fn set_ends_with(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ends_with = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `equals`.\n"]
    pub fn set_equals(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.equals = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_ends_with`.\n"]
    pub fn set_not_ends_with(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.not_ends_with = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_equals`.\n"]
    pub fn set_not_equals(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.not_equals = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_starts_with`.\n"]
    pub fn set_not_starts_with(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.not_starts_with = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `starts_with`.\n"]
    pub fn set_starts_with(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.starts_with = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors_FieldSelectors {
    type O =
        terrars
        ::BlockAssignable<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors_FieldSelectors,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors_FieldSelectors {
    #[doc = ""]
    pub field: terrars::PrimField<std::string::String>,
}

impl BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors_FieldSelectors {
    pub fn build(
        self,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors_FieldSelectors {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors_FieldSelectors {
            ends_with: core::default::Default::default(),
            equals: core::default::Default::default(),
            field: self.field,
            not_ends_with: core::default::Default::default(),
            not_equals: core::default::Default::default(),
            not_starts_with: core::default::Default::default(),
            starts_with: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors_FieldSelectorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors_FieldSelectorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors_FieldSelectorsRef {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors_FieldSelectorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors_FieldSelectorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ends_with` after provisioning.\n"]
    pub fn ends_with(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ends_with", self.base))
    }

    #[doc = "Get a reference to the value of field `equals` after provisioning.\n"]
    pub fn equals(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.equals", self.base))
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `not_ends_with` after provisioning.\n"]
    pub fn not_ends_with(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_ends_with", self.base))
    }

    #[doc = "Get a reference to the value of field `not_equals` after provisioning.\n"]
    pub fn not_equals(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_equals", self.base))
    }

    #[doc = "Get a reference to the value of field `not_starts_with` after provisioning.\n"]
    pub fn not_starts_with(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_starts_with", self.base))
    }

    #[doc = "Get a reference to the value of field `starts_with` after provisioning.\n"]
    pub fn starts_with(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.starts_with", self.base))
    }
}

#[derive(Serialize, Default)]
struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectorsDynamic {
    field_selectors: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors_FieldSelectors,
        >,
    >,
}

#[derive(Serialize)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_selectors: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors_FieldSelectors,
        >,
    >,
    dynamic: ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectorsDynamic,
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field_selectors`.\n"]
    pub fn set_field_selectors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors_FieldSelectors,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_selectors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_selectors = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors {
    type O =
        terrars
        ::BlockAssignable<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors {}

impl BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors {
    pub fn build(
        self,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors {
            name: core::default::Default::default(),
            field_selectors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectorsRef {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `field_selectors` after provisioning.\n"]
    pub fn field_selectors(
        &self,
    ) -> terrars
    ::ListRef<
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors_FieldSelectorsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_selectors", self.base))
    }
}

#[derive(Serialize, Default)]
struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParametersDynamic {
    advanced_event_selectors: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors,
        >,
    >,
}

#[derive(Serialize)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_event_selectors: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors,
        >,
    >,
    dynamic: ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParametersDynamic,
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters {
    #[doc = "Set the field `advanced_event_selectors`.\n"]
    pub fn set_advanced_event_selectors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectors,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.advanced_event_selectors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_event_selectors = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters {
    type O =
        terrars::BlockAssignable<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters {}

impl BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters {
    pub fn build(self) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters {
            advanced_event_selectors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParametersRef {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `advanced_event_selectors` after provisioning.\n"]
    pub fn advanced_event_selectors(
        &self,
    ) -> terrars
    ::ListRef<
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters_AdvancedEventSelectorsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_event_selectors", self.base))
    }
}

#[derive(Serialize)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_ElbLoadBalancerLoggingParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_format: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_ElbLoadBalancerLoggingParameters {
    #[doc = "Set the field `field_delimiter`.\n"]
    pub fn set_field_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field_delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_format`.\n"]
    pub fn set_output_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_format = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_ElbLoadBalancerLoggingParameters {
    type O =
        terrars
        ::BlockAssignable<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_ElbLoadBalancerLoggingParameters,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_ElbLoadBalancerLoggingParameters {}

impl BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_ElbLoadBalancerLoggingParameters {
    pub fn build(
        self,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_ElbLoadBalancerLoggingParameters {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_ElbLoadBalancerLoggingParameters {
            field_delimiter: core::default::Default::default(),
            output_format: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_ElbLoadBalancerLoggingParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_ElbLoadBalancerLoggingParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_ElbLoadBalancerLoggingParametersRef {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_ElbLoadBalancerLoggingParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_ElbLoadBalancerLoggingParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field_delimiter` after provisioning.\n"]
    pub fn field_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_delimiter", self.base))
    }

    #[doc = "Get a reference to the value of field `output_format` after provisioning.\n"]
    pub fn output_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_format", self.base))
    }
}

#[derive(Serialize)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_LogDeliveryParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_LogDeliveryParameters {
    #[doc = "Set the field `log_types`.\n"]
    pub fn set_log_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.log_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_LogDeliveryParameters {
    type O =
        terrars
        ::BlockAssignable<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_LogDeliveryParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_LogDeliveryParameters {}

impl BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_LogDeliveryParameters {
    pub fn build(self) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_LogDeliveryParameters {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_LogDeliveryParameters {
            log_types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_LogDeliveryParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_LogDeliveryParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_LogDeliveryParametersRef {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_LogDeliveryParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_LogDeliveryParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_types` after provisioning.\n"]
    pub fn log_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_types", self.base))
    }
}

#[derive(Serialize)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_MskMonitoringParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enhanced_monitoring: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_MskMonitoringParameters {
    #[doc = "Set the field `enhanced_monitoring`.\n"]
    pub fn set_enhanced_monitoring(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.enhanced_monitoring = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_MskMonitoringParameters {
    type O =
        terrars
        ::BlockAssignable<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_MskMonitoringParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_MskMonitoringParameters {}

impl BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_MskMonitoringParameters {
    pub fn build(self) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_MskMonitoringParameters {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_MskMonitoringParameters {
            enhanced_monitoring: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_MskMonitoringParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_MskMonitoringParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_MskMonitoringParametersRef {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_MskMonitoringParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_MskMonitoringParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enhanced_monitoring` after provisioning.\n"]
    pub fn enhanced_monitoring(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enhanced_monitoring", self.base))
    }
}

#[derive(Serialize)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_VpcFlowLogParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_aggregation_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_VpcFlowLogParameters {
    #[doc = "Set the field `log_format`.\n"]
    pub fn set_log_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_aggregation_interval`.\n"]
    pub fn set_max_aggregation_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_aggregation_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `traffic_type`.\n"]
    pub fn set_traffic_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.traffic_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_VpcFlowLogParameters {
    type O =
        terrars::BlockAssignable<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_VpcFlowLogParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_VpcFlowLogParameters {}

impl BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_VpcFlowLogParameters {
    pub fn build(self) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_VpcFlowLogParameters {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_VpcFlowLogParameters {
            log_format: core::default::Default::default(),
            max_aggregation_interval: core::default::Default::default(),
            traffic_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_VpcFlowLogParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_VpcFlowLogParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_VpcFlowLogParametersRef {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_VpcFlowLogParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_VpcFlowLogParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_format` after provisioning.\n"]
    pub fn log_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_format", self.base))
    }

    #[doc = "Get a reference to the value of field `max_aggregation_interval` after provisioning.\n"]
    pub fn max_aggregation_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_aggregation_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `traffic_type` after provisioning.\n"]
    pub fn traffic_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_ActionCondition {
    action: terrars::PrimField<std::string::String>,
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_ActionCondition {

}

impl terrars
::ToListMappable for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_ActionCondition {
    type O =
        terrars
        ::BlockAssignable<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_ActionCondition,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_ActionCondition {
    #[doc = ""]
    pub action: terrars::PrimField<std::string::String>,
}

impl BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_ActionCondition {
    pub fn build(
        self,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_ActionCondition {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_ActionCondition {
            action: self.action,
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_ActionConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_ActionConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_ActionConditionRef {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_ActionConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_ActionConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
}

#[derive(Serialize)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_LabelNameCondition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_LabelNameCondition {
    #[doc = "Set the field `label_name`.\n"]
    pub fn set_label_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.label_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_LabelNameCondition {
    type O =
        terrars
        ::BlockAssignable<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_LabelNameCondition,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_LabelNameCondition {}

impl BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_LabelNameCondition {
    pub fn build(
        self,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_LabelNameCondition {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_LabelNameCondition {
            label_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_LabelNameConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_LabelNameConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_LabelNameConditionRef {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_LabelNameConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_LabelNameConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `label_name` after provisioning.\n"]
    pub fn label_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_ConditionsDynamic {
    action_condition: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_ActionCondition,
        >,
    >,
    label_name_condition: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_LabelNameCondition,
        >,
    >,
}

#[derive(Serialize)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_condition: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_ActionCondition,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label_name_condition: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_LabelNameCondition,
        >,
    >,
    dynamic: ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_ConditionsDynamic,
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions {
    #[doc = "Set the field `action_condition`.\n"]
    pub fn set_action_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_ActionCondition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action_condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `label_name_condition`.\n"]
    pub fn set_label_name_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_LabelNameCondition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.label_name_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.label_name_condition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions {
    type O =
        terrars
        ::BlockAssignable<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions {}

impl BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions {
    pub fn build(
        self,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions {
            action_condition: core::default::Default::default(),
            label_name_condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_ConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_ConditionsRef {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_ConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_condition` after provisioning.\n"]
    pub fn action_condition(
        &self,
    ) -> terrars
    ::ListRef<
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_ActionConditionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_condition", self.base))
    }

    #[doc = "Get a reference to the value of field `label_name_condition` after provisioning.\n"]
    pub fn label_name_condition(
        &self,
    ) -> terrars
    ::ListRef<
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions_LabelNameConditionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.label_name_condition", self.base))
    }
}

#[derive(Serialize, Default)]
struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_FiltersDynamic {
    conditions: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions,
        >,
    >,
}

#[derive(Serialize)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requirement: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions,
        >,
    >,
    dynamic: ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_FiltersDynamic,
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters {
    #[doc = "Set the field `behavior`.\n"]
    pub fn set_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requirement`.\n"]
    pub fn set_requirement(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.requirement = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_Conditions,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters {
    type O =
        terrars
        ::BlockAssignable<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters {}

impl BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters {
    pub fn build(
        self,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters {
            behavior: core::default::Default::default(),
            requirement: core::default::Default::default(),
            conditions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_FiltersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_FiltersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_FiltersRef {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_FiltersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_FiltersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `behavior` after provisioning.\n"]
    pub fn behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `requirement` after provisioning.\n"]
    pub fn requirement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requirement", self.base))
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(
        &self,
    ) -> terrars
    ::ListRef<
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters_ConditionsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }
}

#[derive(Serialize, Default)]
struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilterDynamic {
    filters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters,
        >,
    >,
}

#[derive(Serialize)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filters: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters,
        >,
    >,
    dynamic: ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilterDynamic,
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter {
    #[doc = "Set the field `default_behavior`.\n"]
    pub fn set_default_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filters`.\n"]
    pub fn set_filters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_Filters,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter {
    type O =
        terrars
        ::BlockAssignable<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter {}

impl BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter {
    pub fn build(
        self,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter {
            default_behavior: core::default::Default::default(),
            filters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilterRef {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_behavior` after provisioning.\n"]
    pub fn default_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(
        &self,
    ) -> terrars
    ::ListRef<
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter_FiltersRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters", self.base))
    }
}

#[derive(Serialize)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields_SingleHeader {
    name: terrars::PrimField<std::string::String>,
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields_SingleHeader {

}

impl terrars
::ToListMappable for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields_SingleHeader {
    type O =
        terrars
        ::BlockAssignable<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields_SingleHeader,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields_SingleHeader {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields_SingleHeader {
    pub fn build(
        self,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields_SingleHeader {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields_SingleHeader {
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields_SingleHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields_SingleHeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields_SingleHeaderRef {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields_SingleHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields_SingleHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFieldsDynamic {
    single_header: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields_SingleHeader,
        >,
    >,
}

#[derive(Serialize)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_header: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields_SingleHeader,
        >,
    >,
    dynamic: ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFieldsDynamic,
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields {
    #[doc = "Set the field `method`.\n"]
    pub fn set_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_string`.\n"]
    pub fn set_query_string(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.query_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri_path`.\n"]
    pub fn set_uri_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `single_header`.\n"]
    pub fn set_single_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields_SingleHeader,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.single_header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.single_header = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields {
    type O =
        terrars
        ::BlockAssignable<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields {}

impl BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields {
    pub fn build(
        self,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields {
            method: core::default::Default::default(),
            query_string: core::default::Default::default(),
            uri_path: core::default::Default::default(),
            single_header: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFieldsRef {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_string", self.base))
    }

    #[doc = "Get a reference to the value of field `uri_path` after provisioning.\n"]
    pub fn uri_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri_path", self.base))
    }

    #[doc = "Get a reference to the value of field `single_header` after provisioning.\n"]
    pub fn single_header(
        &self,
    ) -> terrars
    ::ListRef<
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields_SingleHeaderRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.single_header", self.base))
    }
}

#[derive(Serialize, Default)]
struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParametersDynamic {
    logging_filter: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter,
        >,
    >,
    redacted_fields: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields,
        >,
    >,
}

#[derive(Serialize)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_filter: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redacted_fields: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields>,
    >,
    dynamic: ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParametersDynamic,
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters {
    #[doc = "Set the field `log_type`.\n"]
    pub fn set_log_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_filter`.\n"]
    pub fn set_logging_filter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilter,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logging_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redacted_fields`.\n"]
    pub fn set_redacted_fields(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFields,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redacted_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redacted_fields = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters {
    type O =
        terrars::BlockAssignable<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters {}

impl BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters {
    pub fn build(self) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters {
            log_type: core::default::Default::default(),
            logging_filter: core::default::Default::default(),
            redacted_fields: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParametersRef {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_type` after provisioning.\n"]
    pub fn log_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_type", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_filter` after provisioning.\n"]
    pub fn logging_filter(
        &self,
    ) -> terrars
    ::ListRef<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_LoggingFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_filter", self.base))
    }

    #[doc = "Get a reference to the value of field `redacted_fields` after provisioning.\n"]
    pub fn redacted_fields(
        &self,
    ) -> terrars
    ::ListRef<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters_RedactedFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redacted_fields", self.base))
    }
}

#[derive(Serialize, Default)]
struct ObservabilityadminTelemetryRule_Rule_DestinationConfigurationDynamic {
    cloudtrail_parameters: std
    ::option
    ::Option<
        terrars::DynamicBlock<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters>,
    >,
    elb_load_balancer_logging_parameters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_ElbLoadBalancerLoggingParameters>,
    >,
    log_delivery_parameters: std
    ::option
    ::Option<
        terrars::DynamicBlock<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_LogDeliveryParameters>,
    >,
    msk_monitoring_parameters: std
    ::option
    ::Option<
        terrars::DynamicBlock<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_MskMonitoringParameters>,
    >,
    vpc_flow_log_parameters: std
    ::option
    ::Option<
        terrars::DynamicBlock<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_VpcFlowLogParameters>,
    >,
    waf_logging_parameters: std
    ::option
    ::Option<
        terrars::DynamicBlock<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters>,
    >,
}

#[derive(Serialize)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_pattern: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_in_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudtrail_parameters: std
    ::option
    ::Option<std::vec::Vec<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elb_load_balancer_logging_parameters: std
    ::option
    ::Option<
        std::vec::Vec<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_ElbLoadBalancerLoggingParameters>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_delivery_parameters: std
    ::option
    ::Option<std::vec::Vec<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_LogDeliveryParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    msk_monitoring_parameters: std
    ::option
    ::Option<std::vec::Vec<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_MskMonitoringParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_flow_log_parameters: std
    ::option
    ::Option<std::vec::Vec<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_VpcFlowLogParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    waf_logging_parameters: std
    ::option
    ::Option<std::vec::Vec<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters>>,
    dynamic: ObservabilityadminTelemetryRule_Rule_DestinationConfigurationDynamic,
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfiguration {
    #[doc = "Set the field `destination_pattern`.\n"]
    pub fn set_destination_pattern(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.destination_pattern = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_type`.\n"]
    pub fn set_destination_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.destination_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention_in_days`.\n"]
    pub fn set_retention_in_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retention_in_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudtrail_parameters`.\n"]
    pub fn set_cloudtrail_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParameters,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudtrail_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudtrail_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `elb_load_balancer_logging_parameters`.\n"]
    pub fn set_elb_load_balancer_logging_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_ElbLoadBalancerLoggingParameters,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.elb_load_balancer_logging_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.elb_load_balancer_logging_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_delivery_parameters`.\n"]
    pub fn set_log_delivery_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_LogDeliveryParameters,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.log_delivery_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.log_delivery_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `msk_monitoring_parameters`.\n"]
    pub fn set_msk_monitoring_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_MskMonitoringParameters,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.msk_monitoring_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.msk_monitoring_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_flow_log_parameters`.\n"]
    pub fn set_vpc_flow_log_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_VpcFlowLogParameters,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_flow_log_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_flow_log_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `waf_logging_parameters`.\n"]
    pub fn set_waf_logging_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParameters,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.waf_logging_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.waf_logging_parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ObservabilityadminTelemetryRule_Rule_DestinationConfiguration {
    type O = terrars::BlockAssignable<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration {}

impl BuildObservabilityadminTelemetryRule_Rule_DestinationConfiguration {
    pub fn build(self) -> ObservabilityadminTelemetryRule_Rule_DestinationConfiguration {
        ObservabilityadminTelemetryRule_Rule_DestinationConfiguration {
            destination_pattern: core::default::Default::default(),
            destination_type: core::default::Default::default(),
            retention_in_days: core::default::Default::default(),
            cloudtrail_parameters: core::default::Default::default(),
            elb_load_balancer_logging_parameters: core::default::Default::default(),
            log_delivery_parameters: core::default::Default::default(),
            msk_monitoring_parameters: core::default::Default::default(),
            vpc_flow_log_parameters: core::default::Default::default(),
            waf_logging_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminTelemetryRule_Rule_DestinationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservabilityadminTelemetryRule_Rule_DestinationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminTelemetryRule_Rule_DestinationConfigurationRef {
        ObservabilityadminTelemetryRule_Rule_DestinationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminTelemetryRule_Rule_DestinationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_pattern` after provisioning.\n"]
    pub fn destination_pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_pattern", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_type` after provisioning.\n"]
    pub fn destination_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_type", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_in_days` after provisioning.\n"]
    pub fn retention_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_in_days", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudtrail_parameters` after provisioning.\n"]
    pub fn cloudtrail_parameters(
        &self,
    ) -> terrars::ListRef<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_CloudtrailParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudtrail_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `elb_load_balancer_logging_parameters` after provisioning.\n"]
    pub fn elb_load_balancer_logging_parameters(
        &self,
    ) -> terrars
    ::ListRef<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_ElbLoadBalancerLoggingParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elb_load_balancer_logging_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `log_delivery_parameters` after provisioning.\n"]
    pub fn log_delivery_parameters(
        &self,
    ) -> terrars::ListRef<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_LogDeliveryParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_delivery_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `msk_monitoring_parameters` after provisioning.\n"]
    pub fn msk_monitoring_parameters(
        &self,
    ) -> terrars::ListRef<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_MskMonitoringParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.msk_monitoring_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_flow_log_parameters` after provisioning.\n"]
    pub fn vpc_flow_log_parameters(
        &self,
    ) -> terrars::ListRef<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_VpcFlowLogParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_flow_log_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `waf_logging_parameters` after provisioning.\n"]
    pub fn waf_logging_parameters(
        &self,
    ) -> terrars::ListRef<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration_WafLoggingParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.waf_logging_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct ObservabilityadminTelemetryRule_RuleDynamic {
    destination_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration>>,
}

#[derive(Serialize)]
pub struct ObservabilityadminTelemetryRule_Rule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_regions: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_field_updates: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selection_criteria: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    telemetry_source_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    telemetry_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_configuration: std
    ::option
    ::Option<std::vec::Vec<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration>>,
    dynamic: ObservabilityadminTelemetryRule_RuleDynamic,
}

impl ObservabilityadminTelemetryRule_Rule {
    #[doc = "Set the field `all_regions`.\n"]
    pub fn set_all_regions(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all_regions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_field_updates`.\n"]
    pub fn set_allow_field_updates(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_field_updates = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.regions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `selection_criteria`.\n"]
    pub fn set_selection_criteria(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.selection_criteria = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `telemetry_source_types`.\n"]
    pub fn set_telemetry_source_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.telemetry_source_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_configuration`.\n"]
    pub fn set_destination_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ObservabilityadminTelemetryRule_Rule_DestinationConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ObservabilityadminTelemetryRule_Rule {
    type O = terrars::BlockAssignable<ObservabilityadminTelemetryRule_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminTelemetryRule_Rule {
    #[doc = ""]
    pub telemetry_type: terrars::PrimField<std::string::String>,
}

impl BuildObservabilityadminTelemetryRule_Rule {
    pub fn build(self) -> ObservabilityadminTelemetryRule_Rule {
        ObservabilityadminTelemetryRule_Rule {
            all_regions: core::default::Default::default(),
            allow_field_updates: core::default::Default::default(),
            regions: core::default::Default::default(),
            resource_type: core::default::Default::default(),
            scope: core::default::Default::default(),
            selection_criteria: core::default::Default::default(),
            telemetry_source_types: core::default::Default::default(),
            telemetry_type: self.telemetry_type,
            destination_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminTelemetryRule_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservabilityadminTelemetryRule_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ObservabilityadminTelemetryRule_RuleRef {
        ObservabilityadminTelemetryRule_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminTelemetryRule_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all_regions` after provisioning.\n"]
    pub fn all_regions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all_regions", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_field_updates` after provisioning.\n"]
    pub fn allow_field_updates(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_field_updates", self.base))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `selection_criteria` after provisioning.\n"]
    pub fn selection_criteria(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.selection_criteria", self.base))
    }

    #[doc = "Get a reference to the value of field `telemetry_source_types` after provisioning.\n"]
    pub fn telemetry_source_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.telemetry_source_types", self.base))
    }

    #[doc = "Get a reference to the value of field `telemetry_type` after provisioning.\n"]
    pub fn telemetry_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.telemetry_type", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_configuration` after provisioning.\n"]
    pub fn destination_configuration(
        &self,
    ) -> terrars::ListRef<ObservabilityadminTelemetryRule_Rule_DestinationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct ObservabilityadminTelemetryRule_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ObservabilityadminTelemetryRule_Timeouts {
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

impl terrars::ToListMappable for ObservabilityadminTelemetryRule_Timeouts {
    type O = terrars::BlockAssignable<ObservabilityadminTelemetryRule_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminTelemetryRule_Timeouts {}

impl BuildObservabilityadminTelemetryRule_Timeouts {
    pub fn build(self) -> ObservabilityadminTelemetryRule_Timeouts {
        ObservabilityadminTelemetryRule_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminTelemetryRule_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservabilityadminTelemetryRule_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ObservabilityadminTelemetryRule_TimeoutsRef {
        ObservabilityadminTelemetryRule_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminTelemetryRule_TimeoutsRef {
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
