use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkfirewallFirewallPolicyData {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std::option::Option<std::vec::Vec<NetworkfirewallFirewallPolicyEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firewall_policy: std::option::Option<std::vec::Vec<NetworkfirewallFirewallPolicyFirewallPolicyEl>>,
    dynamic: NetworkfirewallFirewallPolicyDynamic,
}

struct NetworkfirewallFirewallPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkfirewallFirewallPolicyData>,
}

#[derive(Clone)]
pub struct NetworkfirewallFirewallPolicy(std::rc::Rc<NetworkfirewallFirewallPolicy_>);

impl NetworkfirewallFirewallPolicy {
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

    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkfirewallFirewallPolicyEncryptionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `firewall_policy`.\n"]
    pub fn set_firewall_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().firewall_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.firewall_policy = std::option::Option::Some(d);
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> terrars::ListRef<NetworkfirewallFirewallPolicyEncryptionConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_policy` after provisioning.\n"]
    pub fn firewall_policy(&self) -> terrars::ListRef<NetworkfirewallFirewallPolicyFirewallPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firewall_policy", self.extract_ref()))
    }
}

impl terrars::Referable for NetworkfirewallFirewallPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkfirewallFirewallPolicy { }

impl terrars::ToListMappable for NetworkfirewallFirewallPolicy {
    type O = terrars::ListRef<NetworkfirewallFirewallPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkfirewallFirewallPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_networkfirewall_firewall_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkfirewallFirewallPolicy {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallFirewallPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkfirewallFirewallPolicy {
        let out = NetworkfirewallFirewallPolicy(std::rc::Rc::new(NetworkfirewallFirewallPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkfirewallFirewallPolicyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                encryption_configuration: core::default::Default::default(),
                firewall_policy: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkfirewallFirewallPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkfirewallFirewallPolicyRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> terrars::ListRef<NetworkfirewallFirewallPolicyEncryptionConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_policy` after provisioning.\n"]
    pub fn firewall_policy(&self) -> terrars::ListRef<NetworkfirewallFirewallPolicyFirewallPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firewall_policy", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicyEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallFirewallPolicyEncryptionConfigurationEl {
    #[doc = "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallFirewallPolicyEncryptionConfigurationEl {
    type O = terrars::BlockAssignable<NetworkfirewallFirewallPolicyEncryptionConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicyEncryptionConfigurationEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallFirewallPolicyEncryptionConfigurationEl {
    pub fn build(self) -> NetworkfirewallFirewallPolicyEncryptionConfigurationEl {
        NetworkfirewallFirewallPolicyEncryptionConfigurationEl {
            key_id: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct NetworkfirewallFirewallPolicyEncryptionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallPolicyEncryptionConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicyEncryptionConfigurationElRef {
        NetworkfirewallFirewallPolicyEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicyEncryptionConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
    rule_order: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl { }

impl terrars::ToListMappable for NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
    type O = terrars::BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
    #[doc = ""]
    pub rule_order: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
    pub fn build(self) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl { rule_order: self.rule_order }
    }
}

pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule_order` after provisioning.\n"]
    pub fn rule_order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_order", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl {
    type O =
        terrars
        ::BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl {}

impl BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl {
    pub fn build(self) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl {
            action: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideElRef {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElDynamic {
    override_: std
    ::option
    ::Option<
        terrars::DynamicBlock<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl>,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    resource_arn: terrars::PrimField<std::string::String>,
    #[serde(rename = "override", skip_serializing_if = "std::option::Option::is_none")]
    override_: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl>>,
    dynamic: NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElDynamic,
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.override_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.override_ = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
    type O = terrars::BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
    pub fn build(self) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
            priority: core::default::Default::default(),
            resource_arn: self.resource_arn,
            override_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(
        &self,
    ) -> terrars::ListRef<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.override", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {
    value: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {

}

impl terrars
::ToListMappable for NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {
    pub fn build(
        self,
    ) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {
            value: self.value,
        }
    }
}

pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDynamic {
    dimension: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl,
        >,
    >,
    dynamic: NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDynamic,
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl {
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
                            NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl,
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
::ToListMappable for NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl {}

impl BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl {
    pub fn build(
        self,
    ) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl {
            dimension: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElDynamic {
    publish_metric_action: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_metric_action: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl,
        >,
    >,
    dynamic: NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElDynamic,
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl {
    #[doc = "Set the field `publish_metric_action`.\n"]
    pub fn set_publish_metric_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.publish_metric_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.publish_metric_action = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl {
    type O =
        terrars
        ::BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl {}

impl BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl {
    pub fn build(self) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl {
            publish_metric_action: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `publish_metric_action` after provisioning.\n"]
    pub fn publish_metric_action(
        &self,
    ) -> terrars
    ::ListRef<
        NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publish_metric_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElDynamic {
    action_definition: std
    ::option
    ::Option<
        terrars::DynamicBlock<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl>,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
    action_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_definition: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl>>,
    dynamic: NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElDynamic,
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
    #[doc = "Set the field `action_definition`.\n"]
    pub fn set_action_definition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action_definition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
    type O = terrars::BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
    #[doc = ""]
    pub action_name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
    pub fn build(self) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
            action_name: self.action_name,
            action_definition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_name` after provisioning.\n"]
    pub fn action_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_name", self.base))
    }

    #[doc = "Get a reference to the value of field `action_definition` after provisioning.\n"]
    pub fn action_definition(
        &self,
    ) -> terrars::ListRef<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_definition", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {
    priority: terrars::PrimField<f64>,
    resource_arn: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl { }

impl terrars::ToListMappable for NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {
    type O = terrars::BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {
    #[doc = ""]
    pub priority: terrars::PrimField<f64>,
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {
    pub fn build(self) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {
            priority: self.priority,
            resource_arn: self.resource_arn,
        }
    }
}

pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceElRef {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallPolicyFirewallPolicyElDynamic {
    stateful_engine_options: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl>>,
    stateful_rule_group_reference: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl>>,
    stateless_custom_action: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl>>,
    stateless_rule_group_reference: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicyFirewallPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_default_actions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    stateless_default_actions: terrars::SetField<terrars::PrimField<std::string::String>>,
    stateless_fragment_default_actions: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_engine_options: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_rule_group_reference: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateless_custom_action: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateless_rule_group_reference: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl>>,
    dynamic: NetworkfirewallFirewallPolicyFirewallPolicyElDynamic,
}

impl NetworkfirewallFirewallPolicyFirewallPolicyEl {
    #[doc = "Set the field `stateful_default_actions`.\n"]
    pub fn set_stateful_default_actions(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.stateful_default_actions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stateful_engine_options`.\n"]
    pub fn set_stateful_engine_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.stateful_engine_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.stateful_engine_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stateful_rule_group_reference`.\n"]
    pub fn set_stateful_rule_group_reference(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.stateful_rule_group_reference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.stateful_rule_group_reference = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stateless_custom_action`.\n"]
    pub fn set_stateless_custom_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.stateless_custom_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.stateless_custom_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stateless_rule_group_reference`.\n"]
    pub fn set_stateless_rule_group_reference(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.stateless_rule_group_reference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.stateless_rule_group_reference = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallFirewallPolicyFirewallPolicyEl {
    type O = terrars::BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicyFirewallPolicyEl {
    #[doc = ""]
    pub stateless_default_actions: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub stateless_fragment_default_actions: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkfirewallFirewallPolicyFirewallPolicyEl {
    pub fn build(self) -> NetworkfirewallFirewallPolicyFirewallPolicyEl {
        NetworkfirewallFirewallPolicyFirewallPolicyEl {
            stateful_default_actions: core::default::Default::default(),
            stateless_default_actions: self.stateless_default_actions,
            stateless_fragment_default_actions: self.stateless_fragment_default_actions,
            stateful_engine_options: core::default::Default::default(),
            stateful_rule_group_reference: core::default::Default::default(),
            stateless_custom_action: core::default::Default::default(),
            stateless_rule_group_reference: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallFirewallPolicyFirewallPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallPolicyFirewallPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicyFirewallPolicyElRef {
        NetworkfirewallFirewallPolicyFirewallPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stateful_default_actions` after provisioning.\n"]
    pub fn stateful_default_actions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.stateful_default_actions", self.base))
    }

    #[doc = "Get a reference to the value of field `stateless_default_actions` after provisioning.\n"]
    pub fn stateless_default_actions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.stateless_default_actions", self.base))
    }

    #[doc = "Get a reference to the value of field `stateless_fragment_default_actions` after provisioning.\n"]
    pub fn stateless_fragment_default_actions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.stateless_fragment_default_actions", self.base))
    }

    #[doc = "Get a reference to the value of field `stateful_engine_options` after provisioning.\n"]
    pub fn stateful_engine_options(
        &self,
    ) -> terrars::ListRef<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_engine_options", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallPolicyDynamic {
    encryption_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallFirewallPolicyEncryptionConfigurationEl>>,
    firewall_policy: std::option::Option<terrars::DynamicBlock<NetworkfirewallFirewallPolicyFirewallPolicyEl>>,
}
