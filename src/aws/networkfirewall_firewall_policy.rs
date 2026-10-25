#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkfirewallFirewallPolicyData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std::option::Option<std::vec::Vec<NetworkfirewallFirewallPolicy_EncryptionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firewall_policy: std::option::Option<std::vec::Vec<NetworkfirewallFirewallPolicy_FirewallPolicy>>,
    dynamic: NetworkfirewallFirewallPolicyDynamic,
}

struct NetworkfirewallFirewallPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkfirewallFirewallPolicyData_>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkfirewallFirewallPolicy_EncryptionConfiguration>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkfirewallFirewallPolicy_FirewallPolicy>>,
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

    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<NetworkfirewallFirewallPolicy_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_policy` after provisioning.\n"]
    pub fn firewall_policy(&self) -> terrars::ListRef<NetworkfirewallFirewallPolicy_FirewallPolicyRef> {
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
            data: std::cell::RefCell::new(NetworkfirewallFirewallPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<NetworkfirewallFirewallPolicy_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_policy` after provisioning.\n"]
    pub fn firewall_policy(&self) -> terrars::ListRef<NetworkfirewallFirewallPolicy_FirewallPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firewall_policy", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallPolicyDynamic {
    encryption_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallFirewallPolicy_EncryptionConfiguration>>,
    firewall_policy: std::option::Option<terrars::DynamicBlock<NetworkfirewallFirewallPolicy_FirewallPolicy>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicy_EncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallFirewallPolicy_EncryptionConfiguration {
    #[doc = "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallFirewallPolicy_EncryptionConfiguration {
    type O = terrars::BlockAssignable<NetworkfirewallFirewallPolicy_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicy_EncryptionConfiguration {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallFirewallPolicy_EncryptionConfiguration {
    pub fn build(self) -> NetworkfirewallFirewallPolicy_EncryptionConfiguration {
        NetworkfirewallFirewallPolicy_EncryptionConfiguration {
            key_id: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewallPolicy_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallPolicy_EncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicy_EncryptionConfigurationRef {
        NetworkfirewallFirewallPolicy_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicy_EncryptionConfigurationRef {
    #[allow(dead_code)]
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
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSet {
    definition: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSet { }

impl terrars::ToListMappable for NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSet {
    type O =
        terrars::BlockAssignable<NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSet {
    #[doc = ""]
    pub definition: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSet {
    pub fn build(self) -> NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSet {
        NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSet {
            definition: self.definition,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSetRef {
        NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.definition", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariablesDynamic {
    ip_set: std
    ::option
    ::Option<
        terrars::DynamicBlock<NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSet>,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_set: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSet>>,
    dynamic: NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariablesDynamic,
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables {
    #[doc = "Set the field `ip_set`.\n"]
    pub fn set_ip_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSet,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_set = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables {
    type O = terrars::BlockAssignable<NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables {
    pub fn build(self) -> NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables {
        NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables {
            key: self.key,
            ip_set: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariablesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariablesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariablesRef {
        NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariablesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariablesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_set` after provisioning.\n"]
    pub fn ip_set(
        &self,
    ) -> terrars::ListRef<NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_set", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariablesDynamic {
    rule_variables: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_variables: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables>>,
    dynamic: NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariablesDynamic,
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables {
    #[doc = "Set the field `rule_variables`.\n"]
    pub fn set_rule_variables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rule_variables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_variables = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables {
    type O = terrars::BlockAssignable<NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables {}

impl BuildNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables {
    pub fn build(self) -> NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables {
        NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables {
            rule_variables: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariablesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariablesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariablesRef {
        NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariablesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariablesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_idle_timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeouts {
    #[doc = "Set the field `tcp_idle_timeout_seconds`.\n"]
    pub fn set_tcp_idle_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tcp_idle_timeout_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeouts {
    type O =
        terrars::BlockAssignable<NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeouts {}

impl BuildNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeouts {
    pub fn build(self) -> NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeouts {
        NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeouts {
            tcp_idle_timeout_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeoutsRef {
        NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tcp_idle_timeout_seconds` after provisioning.\n"]
    pub fn tcp_idle_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_idle_timeout_seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptionsDynamic {
    flow_timeouts: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeouts>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_exception_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flow_timeouts: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeouts>>,
    dynamic: NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptionsDynamic,
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions {
    #[doc = "Set the field `rule_order`.\n"]
    pub fn set_rule_order(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule_order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stream_exception_policy`.\n"]
    pub fn set_stream_exception_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.stream_exception_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flow_timeouts`.\n"]
    pub fn set_flow_timeouts(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeouts,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.flow_timeouts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.flow_timeouts = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions {
    type O = terrars::BlockAssignable<NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions {}

impl BuildNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions {
    pub fn build(self) -> NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions {
        NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions {
            rule_order: core::default::Default::default(),
            stream_exception_policy: core::default::Default::default(),
            flow_timeouts: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptionsRef {
        NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule_order` after provisioning.\n"]
    pub fn rule_order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_order", self.base))
    }

    #[doc = "Get a reference to the value of field `stream_exception_policy` after provisioning.\n"]
    pub fn stream_exception_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_exception_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `flow_timeouts` after provisioning.\n"]
    pub fn flow_timeouts(
        &self,
    ) -> terrars::ListRef<NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeoutsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.flow_timeouts", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_Override {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_Override {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_Override {
    type O =
        terrars::BlockAssignable<NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_Override>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_Override {}

impl BuildNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_Override {
    pub fn build(self) -> NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_Override {
        NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_Override {
            action: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_OverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_OverrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_OverrideRef {
        NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_OverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_OverrideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReferenceDynamic {
    override_: std
    ::option
    ::Option<
        terrars::DynamicBlock<NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_Override>,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deep_threat_inspection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    resource_arn: terrars::PrimField<std::string::String>,
    #[serde(rename = "override", skip_serializing_if = "std::option::Option::is_none")]
    override_: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_Override>>,
    dynamic: NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReferenceDynamic,
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference {
    #[doc = "Set the field `deep_threat_inspection`.\n"]
    pub fn set_deep_threat_inspection(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.deep_threat_inspection = std::option::Option::Some(v.into());
        self
    }

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
                            NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_Override,
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

impl terrars::ToListMappable for NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference {
    type O = terrars::BlockAssignable<NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference {
    pub fn build(self) -> NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference {
        NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference {
            deep_threat_inspection: core::default::Default::default(),
            priority: core::default::Default::default(),
            resource_arn: self.resource_arn,
            override_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReferenceRef {
        NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deep_threat_inspection` after provisioning.\n"]
    pub fn deep_threat_inspection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deep_threat_inspection", self.base))
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
    ) -> terrars::ListRef<NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_OverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.override", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_Dimension {
    value: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_Dimension {

}

impl terrars
::ToListMappable for NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_Dimension {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_Dimension,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_Dimension {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_Dimension {
    pub fn build(
        self,
    ) -> NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_Dimension {
        NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_Dimension {
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_DimensionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_DimensionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_DimensionRef {
        NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_DimensionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_DimensionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricActionDynamic {
    dimension: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_Dimension,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_Dimension,
        >,
    >,
    dynamic: NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricActionDynamic,
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction {
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
                            NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_Dimension,
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
::ToListMappable for NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction {}

impl BuildNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction {
    pub fn build(
        self,
    ) -> NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction {
        NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction {
            dimension: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricActionRef {
        NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinitionDynamic {
    publish_metric_action: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_metric_action: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction>,
    >,
    dynamic: NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinitionDynamic,
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition {
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
                            NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction,
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

impl terrars::ToListMappable for NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition {
    type O =
        terrars
        ::BlockAssignable<NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition {}

impl BuildNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition {
    pub fn build(self) -> NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition {
        NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition {
            publish_metric_action: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinitionRef {
        NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `publish_metric_action` after provisioning.\n"]
    pub fn publish_metric_action(
        &self,
    ) -> terrars
    ::ListRef<
        NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricActionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publish_metric_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomActionDynamic {
    action_definition: std
    ::option
    ::Option<
        terrars::DynamicBlock<NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition>,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction {
    action_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_definition: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition>>,
    dynamic: NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomActionDynamic,
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction {
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
                            NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition,
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

impl terrars::ToListMappable for NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction {
    type O = terrars::BlockAssignable<NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction {
    #[doc = ""]
    pub action_name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction {
    pub fn build(self) -> NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction {
        NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction {
            action_name: self.action_name,
            action_definition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomActionRef {
        NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomActionRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_definition", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReference {
    priority: terrars::PrimField<f64>,
    resource_arn: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReference { }

impl terrars::ToListMappable for NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReference {
    type O = terrars::BlockAssignable<NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReference {
    #[doc = ""]
    pub priority: terrars::PrimField<f64>,
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReference {
    pub fn build(self) -> NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReference {
        NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReference {
            priority: self.priority,
            resource_arn: self.resource_arn,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReferenceRef {
        NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReferenceRef {
    #[allow(dead_code)]
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
struct NetworkfirewallFirewallPolicy_FirewallPolicyDynamic {
    policy_variables: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables>>,
    stateful_engine_options: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions>>,
    stateful_rule_group_reference: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference>>,
    stateless_custom_action: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction>>,
    stateless_rule_group_reference: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReference>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_tls_session_holding: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_default_actions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    stateless_default_actions: terrars::SetField<terrars::PrimField<std::string::String>>,
    stateless_fragment_default_actions: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_inspection_configuration_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_variables: std::option::Option<std::vec::Vec<NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_engine_options: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_rule_group_reference: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateless_custom_action: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateless_rule_group_reference: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReference>>,
    dynamic: NetworkfirewallFirewallPolicy_FirewallPolicyDynamic,
}

impl NetworkfirewallFirewallPolicy_FirewallPolicy {
    #[doc = "Set the field `enable_tls_session_holding`.\n"]
    pub fn set_enable_tls_session_holding(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_tls_session_holding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stateful_default_actions`.\n"]
    pub fn set_stateful_default_actions(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.stateful_default_actions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_inspection_configuration_arn`.\n"]
    pub fn set_tls_inspection_configuration_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.tls_inspection_configuration_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_variables`.\n"]
    pub fn set_policy_variables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.policy_variables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.policy_variables = std::option::Option::Some(d);
            },
        }
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
                        terrars::BlockAssignable<NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions>,
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
                        ::BlockAssignable<NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference>,
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
                        terrars::BlockAssignable<NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction>,
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
                        ::BlockAssignable<NetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReference>,
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

impl terrars::ToListMappable for NetworkfirewallFirewallPolicy_FirewallPolicy {
    type O = terrars::BlockAssignable<NetworkfirewallFirewallPolicy_FirewallPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicy_FirewallPolicy {
    #[doc = ""]
    pub stateless_default_actions: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub stateless_fragment_default_actions: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkfirewallFirewallPolicy_FirewallPolicy {
    pub fn build(self) -> NetworkfirewallFirewallPolicy_FirewallPolicy {
        NetworkfirewallFirewallPolicy_FirewallPolicy {
            enable_tls_session_holding: core::default::Default::default(),
            stateful_default_actions: core::default::Default::default(),
            stateless_default_actions: self.stateless_default_actions,
            stateless_fragment_default_actions: self.stateless_fragment_default_actions,
            tls_inspection_configuration_arn: core::default::Default::default(),
            policy_variables: core::default::Default::default(),
            stateful_engine_options: core::default::Default::default(),
            stateful_rule_group_reference: core::default::Default::default(),
            stateless_custom_action: core::default::Default::default(),
            stateless_rule_group_reference: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewallPolicy_FirewallPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallPolicy_FirewallPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallPolicy_FirewallPolicyRef {
        NetworkfirewallFirewallPolicy_FirewallPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicy_FirewallPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_tls_session_holding` after provisioning.\n"]
    pub fn enable_tls_session_holding(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_tls_session_holding", self.base))
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

    #[doc = "Get a reference to the value of field `tls_inspection_configuration_arn` after provisioning.\n"]
    pub fn tls_inspection_configuration_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_inspection_configuration_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_variables` after provisioning.\n"]
    pub fn policy_variables(&self) -> terrars::ListRef<NetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariablesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `stateful_engine_options` after provisioning.\n"]
    pub fn stateful_engine_options(
        &self,
    ) -> terrars::ListRef<NetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_engine_options", self.base))
    }
}
