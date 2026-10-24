use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ConfigConfigRuleData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_parameters: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_execution_frequency: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<std::vec::Vec<ConfigConfigRuleScopeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<ConfigConfigRuleSourceEl>>,
    dynamic: ConfigConfigRuleDynamic,
}

struct ConfigConfigRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ConfigConfigRuleData>,
}

#[derive(Clone)]
pub struct ConfigConfigRule(std::rc::Rc<ConfigConfigRule_>);

impl ConfigConfigRule {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_parameters`.\n"]
    pub fn set_input_parameters(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().input_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_execution_frequency`.\n"]
    pub fn set_maximum_execution_frequency(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().maximum_execution_frequency = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(self, v: impl std::convert::Into<terrars::BlockAssignable<ConfigConfigRuleScopeEl>>) -> Self {
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

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(self, v: impl std::convert::Into<terrars::BlockAssignable<ConfigConfigRuleSourceEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_parameters` after provisioning.\n"]
    pub fn input_parameters(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_execution_frequency` after provisioning.\n"]
    pub fn maximum_execution_frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_execution_frequency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\n"]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::ListRef<ConfigConfigRuleScopeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<ConfigConfigRuleSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

impl terrars::Referable for ConfigConfigRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ConfigConfigRule { }

impl terrars::ToListMappable for ConfigConfigRule {
    type O = terrars::ListRef<ConfigConfigRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ConfigConfigRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_config_config_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConfigConfigRule {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildConfigConfigRule {
    pub fn build(self, stack: &mut terrars::Stack) -> ConfigConfigRule {
        let out = ConfigConfigRule(std::rc::Rc::new(ConfigConfigRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ConfigConfigRuleData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                input_parameters: core::default::Default::default(),
                maximum_execution_frequency: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                scope: core::default::Default::default(),
                source: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConfigConfigRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConfigConfigRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ConfigConfigRuleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_parameters` after provisioning.\n"]
    pub fn input_parameters(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_execution_frequency` after provisioning.\n"]
    pub fn maximum_execution_frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_execution_frequency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\n"]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::ListRef<ConfigConfigRuleScopeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<ConfigConfigRuleSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ConfigConfigRuleScopeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_resource_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_resource_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ConfigConfigRuleScopeEl {
    #[doc = "Set the field `compliance_resource_id`.\n"]
    pub fn set_compliance_resource_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.compliance_resource_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compliance_resource_types`.\n"]
    pub fn set_compliance_resource_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.compliance_resource_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_key`.\n"]
    pub fn set_tag_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_value`.\n"]
    pub fn set_tag_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ConfigConfigRuleScopeEl {
    type O = terrars::BlockAssignable<ConfigConfigRuleScopeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConfigConfigRuleScopeEl {}

impl BuildConfigConfigRuleScopeEl {
    pub fn build(self) -> ConfigConfigRuleScopeEl {
        ConfigConfigRuleScopeEl {
            compliance_resource_id: core::default::Default::default(),
            compliance_resource_types: core::default::Default::default(),
            tag_key: core::default::Default::default(),
            tag_value: core::default::Default::default(),
        }
    }
}

pub struct ConfigConfigRuleScopeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConfigConfigRuleScopeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ConfigConfigRuleScopeElRef {
        ConfigConfigRuleScopeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConfigConfigRuleScopeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compliance_resource_id` after provisioning.\n"]
    pub fn compliance_resource_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compliance_resource_id", self.base))
    }

    #[doc = "Get a reference to the value of field `compliance_resource_types` after provisioning.\n"]
    pub fn compliance_resource_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.compliance_resource_types", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_key` after provisioning.\n"]
    pub fn tag_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_key", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_value` after provisioning.\n"]
    pub fn tag_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_value", self.base))
    }
}

#[derive(Serialize)]
pub struct ConfigConfigRuleSourceElCustomPolicyDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_debug_log_delivery: std::option::Option<terrars::PrimField<bool>>,
    policy_runtime: terrars::PrimField<std::string::String>,
    policy_text: terrars::PrimField<std::string::String>,
}

impl ConfigConfigRuleSourceElCustomPolicyDetailsEl {
    #[doc = "Set the field `enable_debug_log_delivery`.\n"]
    pub fn set_enable_debug_log_delivery(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_debug_log_delivery = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ConfigConfigRuleSourceElCustomPolicyDetailsEl {
    type O = terrars::BlockAssignable<ConfigConfigRuleSourceElCustomPolicyDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConfigConfigRuleSourceElCustomPolicyDetailsEl {
    #[doc = ""]
    pub policy_runtime: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub policy_text: terrars::PrimField<std::string::String>,
}

impl BuildConfigConfigRuleSourceElCustomPolicyDetailsEl {
    pub fn build(self) -> ConfigConfigRuleSourceElCustomPolicyDetailsEl {
        ConfigConfigRuleSourceElCustomPolicyDetailsEl {
            enable_debug_log_delivery: core::default::Default::default(),
            policy_runtime: self.policy_runtime,
            policy_text: self.policy_text,
        }
    }
}

pub struct ConfigConfigRuleSourceElCustomPolicyDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConfigConfigRuleSourceElCustomPolicyDetailsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConfigConfigRuleSourceElCustomPolicyDetailsElRef {
        ConfigConfigRuleSourceElCustomPolicyDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConfigConfigRuleSourceElCustomPolicyDetailsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_debug_log_delivery` after provisioning.\n"]
    pub fn enable_debug_log_delivery(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_debug_log_delivery", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_runtime` after provisioning.\n"]
    pub fn policy_runtime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_runtime", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_text` after provisioning.\n"]
    pub fn policy_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_text", self.base))
    }
}

#[derive(Serialize)]
pub struct ConfigConfigRuleSourceElSourceDetailEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_execution_frequency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ConfigConfigRuleSourceElSourceDetailEl {
    #[doc = "Set the field `event_source`.\n"]
    pub fn set_event_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.event_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_execution_frequency`.\n"]
    pub fn set_maximum_execution_frequency(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.maximum_execution_frequency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message_type`.\n"]
    pub fn set_message_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ConfigConfigRuleSourceElSourceDetailEl {
    type O = terrars::BlockAssignable<ConfigConfigRuleSourceElSourceDetailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConfigConfigRuleSourceElSourceDetailEl {}

impl BuildConfigConfigRuleSourceElSourceDetailEl {
    pub fn build(self) -> ConfigConfigRuleSourceElSourceDetailEl {
        ConfigConfigRuleSourceElSourceDetailEl {
            event_source: core::default::Default::default(),
            maximum_execution_frequency: core::default::Default::default(),
            message_type: core::default::Default::default(),
        }
    }
}

pub struct ConfigConfigRuleSourceElSourceDetailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConfigConfigRuleSourceElSourceDetailElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ConfigConfigRuleSourceElSourceDetailElRef {
        ConfigConfigRuleSourceElSourceDetailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConfigConfigRuleSourceElSourceDetailElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_source` after provisioning.\n"]
    pub fn event_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_source", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_execution_frequency` after provisioning.\n"]
    pub fn maximum_execution_frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_execution_frequency", self.base))
    }

    #[doc = "Get a reference to the value of field `message_type` after provisioning.\n"]
    pub fn message_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ConfigConfigRuleSourceElDynamic {
    custom_policy_details: std::option::Option<terrars::DynamicBlock<ConfigConfigRuleSourceElCustomPolicyDetailsEl>>,
    source_detail: std::option::Option<terrars::DynamicBlock<ConfigConfigRuleSourceElSourceDetailEl>>,
}

#[derive(Serialize)]
pub struct ConfigConfigRuleSourceEl {
    owner: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_policy_details: std::option::Option<std::vec::Vec<ConfigConfigRuleSourceElCustomPolicyDetailsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_detail: std::option::Option<std::vec::Vec<ConfigConfigRuleSourceElSourceDetailEl>>,
    dynamic: ConfigConfigRuleSourceElDynamic,
}

impl ConfigConfigRuleSourceEl {
    #[doc = "Set the field `source_identifier`.\n"]
    pub fn set_source_identifier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_policy_details`.\n"]
    pub fn set_custom_policy_details(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ConfigConfigRuleSourceElCustomPolicyDetailsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_policy_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_policy_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_detail`.\n"]
    pub fn set_source_detail(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ConfigConfigRuleSourceElSourceDetailEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_detail = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_detail = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ConfigConfigRuleSourceEl {
    type O = terrars::BlockAssignable<ConfigConfigRuleSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConfigConfigRuleSourceEl {
    #[doc = ""]
    pub owner: terrars::PrimField<std::string::String>,
}

impl BuildConfigConfigRuleSourceEl {
    pub fn build(self) -> ConfigConfigRuleSourceEl {
        ConfigConfigRuleSourceEl {
            owner: self.owner,
            source_identifier: core::default::Default::default(),
            custom_policy_details: core::default::Default::default(),
            source_detail: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ConfigConfigRuleSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConfigConfigRuleSourceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ConfigConfigRuleSourceElRef {
        ConfigConfigRuleSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConfigConfigRuleSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }

    #[doc = "Get a reference to the value of field `source_identifier` after provisioning.\n"]
    pub fn source_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_policy_details` after provisioning.\n"]
    pub fn custom_policy_details(&self) -> terrars::ListRef<ConfigConfigRuleSourceElCustomPolicyDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_policy_details", self.base))
    }
}

#[derive(Serialize, Default)]
struct ConfigConfigRuleDynamic {
    scope: std::option::Option<terrars::DynamicBlock<ConfigConfigRuleScopeEl>>,
    source: std::option::Option<terrars::DynamicBlock<ConfigConfigRuleSourceEl>>,
}
