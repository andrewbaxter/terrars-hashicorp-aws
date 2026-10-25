#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkfirewallRuleGroupData_ {
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
    capacity: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroup_EncryptionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_group: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroup_RuleGroup>>,
    dynamic: NetworkfirewallRuleGroupDynamic,
}

struct NetworkfirewallRuleGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkfirewallRuleGroupData_>,
}

#[derive(Clone)]
pub struct NetworkfirewallRuleGroup(std::rc::Rc<NetworkfirewallRuleGroup_>);

impl NetworkfirewallRuleGroup {
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

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().rules = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkfirewallRuleGroup_EncryptionConfiguration>>,
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

    #[doc = "Set the field `rule_group`.\n"]
    pub fn set_rule_group(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule_group = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<NetworkfirewallRuleGroup_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_group` after provisioning.\n"]
    pub fn rule_group(&self) -> terrars::ListRef<NetworkfirewallRuleGroup_RuleGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_group", self.extract_ref()))
    }
}

impl terrars::Referable for NetworkfirewallRuleGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkfirewallRuleGroup { }

impl terrars::ToListMappable for NetworkfirewallRuleGroup {
    type O = terrars::ListRef<NetworkfirewallRuleGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkfirewallRuleGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_networkfirewall_rule_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkfirewallRuleGroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub capacity: terrars::PrimField<f64>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkfirewallRuleGroup {
        let out = NetworkfirewallRuleGroup(std::rc::Rc::new(NetworkfirewallRuleGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkfirewallRuleGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                capacity: self.capacity,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                rules: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                encryption_configuration: core::default::Default::default(),
                rule_group: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkfirewallRuleGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkfirewallRuleGroupRef {
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

    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<NetworkfirewallRuleGroup_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_group` after provisioning.\n"]
    pub fn rule_group(&self) -> terrars::ListRef<NetworkfirewallRuleGroup_RuleGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_group", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupDynamic {
    encryption_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroup_EncryptionConfiguration>>,
    rule_group: std::option::Option<terrars::DynamicBlock<NetworkfirewallRuleGroup_RuleGroup>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_EncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallRuleGroup_EncryptionConfiguration {
    #[doc = "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallRuleGroup_EncryptionConfiguration {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroup_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_EncryptionConfiguration {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroup_EncryptionConfiguration {
    pub fn build(self) -> NetworkfirewallRuleGroup_EncryptionConfiguration {
        NetworkfirewallRuleGroup_EncryptionConfiguration {
            key_id: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroup_EncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_EncryptionConfigurationRef {
        NetworkfirewallRuleGroup_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_EncryptionConfigurationRef {
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
pub struct NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences_IpSetReference {
    reference_arn: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences_IpSetReference { }

impl terrars::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences_IpSetReference {
    type O =
        terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences_IpSetReference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences_IpSetReference {
    #[doc = ""]
    pub reference_arn: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences_IpSetReference {
    pub fn build(self) -> NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences_IpSetReference {
        NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences_IpSetReference {
            reference_arn: self.reference_arn,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences_IpSetReferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences_IpSetReferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences_IpSetReferenceRef {
        NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences_IpSetReferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences_IpSetReferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `reference_arn` after provisioning.\n"]
    pub fn reference_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reference_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferencesDynamic {
    ip_set_reference: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences_IpSetReference>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_set_reference: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences_IpSetReference>>,
    dynamic: NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferencesDynamic,
}

impl NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences {
    #[doc = "Set the field `ip_set_reference`.\n"]
    pub fn set_ip_set_reference(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences_IpSetReference,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_set_reference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_set_reference = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences {
    pub fn build(self) -> NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences {
        NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences {
            key: self.key,
            ip_set_reference: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferencesRef {
        NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_set_reference` after provisioning.\n"]
    pub fn ip_set_reference(
        &self,
    ) -> terrars::ListRef<NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences_IpSetReferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_set_reference", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroup_RuleGroup_ReferenceSetsDynamic {
    ip_set_references: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_ReferenceSets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_set_references: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences>>,
    dynamic: NetworkfirewallRuleGroup_RuleGroup_ReferenceSetsDynamic,
}

impl NetworkfirewallRuleGroup_RuleGroup_ReferenceSets {
    #[doc = "Set the field `ip_set_references`.\n"]
    pub fn set_ip_set_references(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_ReferenceSets_IpSetReferences>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_set_references = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_set_references = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_ReferenceSets {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_ReferenceSets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_ReferenceSets {}

impl BuildNetworkfirewallRuleGroup_RuleGroup_ReferenceSets {
    pub fn build(self) -> NetworkfirewallRuleGroup_RuleGroup_ReferenceSets {
        NetworkfirewallRuleGroup_RuleGroup_ReferenceSets {
            ip_set_references: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_ReferenceSetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroup_RuleGroup_ReferenceSetsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_ReferenceSetsRef {
        NetworkfirewallRuleGroup_RuleGroup_ReferenceSetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_ReferenceSetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets_IpSet {
    definition: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets_IpSet { }

impl terrars::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets_IpSet {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets_IpSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets_IpSet {
    #[doc = ""]
    pub definition: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets_IpSet {
    pub fn build(self) -> NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets_IpSet {
        NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets_IpSet { definition: self.definition }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets_IpSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets_IpSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets_IpSetRef {
        NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets_IpSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets_IpSetRef {
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
struct NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSetsDynamic {
    ip_set: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets_IpSet>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_set: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets_IpSet>>,
    dynamic: NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSetsDynamic,
}

impl NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets {
    #[doc = "Set the field `ip_set`.\n"]
    pub fn set_ip_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets_IpSet>>,
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

impl terrars::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets {
    pub fn build(self) -> NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets {
        NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets {
            key: self.key,
            ip_set: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSetsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSetsRef {
        NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_set` after provisioning.\n"]
    pub fn ip_set(&self) -> terrars::ListRef<NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets_IpSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_set", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets_PortSet {
    definition: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets_PortSet { }

impl terrars::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets_PortSet {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets_PortSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets_PortSet {
    #[doc = ""]
    pub definition: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets_PortSet {
    pub fn build(self) -> NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets_PortSet {
        NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets_PortSet { definition: self.definition }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets_PortSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets_PortSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets_PortSetRef {
        NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets_PortSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets_PortSetRef {
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
struct NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSetsDynamic {
    port_set: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets_PortSet>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_set: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets_PortSet>>,
    dynamic: NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSetsDynamic,
}

impl NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets {
    #[doc = "Set the field `port_set`.\n"]
    pub fn set_port_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets_PortSet>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.port_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.port_set = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets {
    pub fn build(self) -> NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets {
        NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets {
            key: self.key,
            port_set: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSetsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSetsRef {
        NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `port_set` after provisioning.\n"]
    pub fn port_set(&self) -> terrars::ListRef<NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets_PortSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_set", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroup_RuleGroup_RuleVariablesDynamic {
    ip_sets: std::option::Option<terrars::DynamicBlock<NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets>>,
    port_sets: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RuleVariables {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_sets: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_sets: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets>>,
    dynamic: NetworkfirewallRuleGroup_RuleGroup_RuleVariablesDynamic,
}

impl NetworkfirewallRuleGroup_RuleGroup_RuleVariables {
    #[doc = "Set the field `ip_sets`.\n"]
    pub fn set_ip_sets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RuleVariables_IpSets>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_sets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_sets = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `port_sets`.\n"]
    pub fn set_port_sets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RuleVariables_PortSets>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.port_sets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.port_sets = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RuleVariables {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RuleVariables>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RuleVariables {}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RuleVariables {
    pub fn build(self) -> NetworkfirewallRuleGroup_RuleGroup_RuleVariables {
        NetworkfirewallRuleGroup_RuleGroup_RuleVariables {
            ip_sets: core::default::Default::default(),
            port_sets: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RuleVariablesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroup_RuleGroup_RuleVariablesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RuleVariablesRef {
        NetworkfirewallRuleGroup_RuleGroup_RuleVariablesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RuleVariablesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_RulesSourceList {
    generated_rules_type: terrars::PrimField<std::string::String>,
    target_types: terrars::SetField<terrars::PrimField<std::string::String>>,
    targets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_RulesSourceList { }

impl terrars::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RulesSource_RulesSourceList {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RulesSource_RulesSourceList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_RulesSourceList {
    #[doc = ""]
    pub generated_rules_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_types: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub targets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_RulesSourceList {
    pub fn build(self) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_RulesSourceList {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_RulesSourceList {
            generated_rules_type: self.generated_rules_type,
            target_types: self.target_types,
            targets: self.targets,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_RulesSourceListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroup_RuleGroup_RulesSource_RulesSourceListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_RulesSourceListRef {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_RulesSourceListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_RulesSourceListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `generated_rules_type` after provisioning.\n"]
    pub fn generated_rules_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generated_rules_type", self.base))
    }

    #[doc = "Get a reference to the value of field `target_types` after provisioning.\n"]
    pub fn target_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_types", self.base))
    }

    #[doc = "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.targets", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_Header {
    destination: terrars::PrimField<std::string::String>,
    destination_port: terrars::PrimField<std::string::String>,
    direction: terrars::PrimField<std::string::String>,
    protocol: terrars::PrimField<std::string::String>,
    source: terrars::PrimField<std::string::String>,
    source_port: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_Header { }

impl terrars::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_Header {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_Header>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_Header {
    #[doc = ""]
    pub destination: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub destination_port: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub direction: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub protocol: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source_port: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_Header {
    pub fn build(self) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_Header {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_Header {
            destination: self.destination,
            destination_port: self.destination_port,
            direction: self.direction,
            protocol: self.protocol,
            source: self.source,
            source_port: self.source_port,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_HeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_HeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_HeaderRef {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_HeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_HeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_port` after provisioning.\n"]
    pub fn destination_port(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_port", self.base))
    }

    #[doc = "Get a reference to the value of field `direction` after provisioning.\n"]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `source_port` after provisioning.\n"]
    pub fn source_port(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_port", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_RuleOption {
    keyword: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_RuleOption {
    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.settings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_RuleOption {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_RuleOption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_RuleOption {
    #[doc = ""]
    pub keyword: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_RuleOption {
    pub fn build(self) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_RuleOption {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_RuleOption {
            keyword: self.keyword,
            settings: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_RuleOptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_RuleOptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_RuleOptionRef {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_RuleOptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_RuleOptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `keyword` after provisioning.\n"]
    pub fn keyword(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keyword", self.base))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRuleDynamic {
    header: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_Header>>,
    rule_option: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_RuleOption>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule {
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_Header>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_option: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_RuleOption>>,
    dynamic: NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRuleDynamic,
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule {
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_Header>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule_option`.\n"]
    pub fn set_rule_option(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_RuleOption>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rule_option = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_option = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule {
    #[doc = ""]
    pub action: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule {
    pub fn build(self) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule {
            action: self.action,
            header: core::default::Default::default(),
            rule_option: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRuleRef {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::ListRef<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule_HeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction_Dimension {
    value: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction_Dimension {

}

impl terrars
::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction_Dimension {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction_Dimension,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction_Dimension {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction_Dimension {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction_Dimension {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction_Dimension {
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction_DimensionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction_DimensionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction_DimensionRef {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction_DimensionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction_DimensionRef {
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
struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricActionDynamic {
    dimension: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction_Dimension,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction_Dimension,
        >,
    >,
    dynamic: NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricActionDynamic,
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction {
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
                            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction_Dimension,
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
::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction {}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction {
            dimension: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricActionRef {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinitionDynamic {
    publish_metric_action: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_metric_action: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction,
        >,
    >,
    dynamic: NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinitionDynamic,
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition {
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
                            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricAction,
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
::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition {}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition {
            publish_metric_action: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinitionRef {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `publish_metric_action` after provisioning.\n"]
    pub fn publish_metric_action(
        &self,
    ) -> terrars
    ::ListRef<
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition_PublishMetricActionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publish_metric_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomActionDynamic {
    action_definition: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction {
    action_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_definition: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition,
        >,
    >,
    dynamic: NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomActionDynamic,
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction {
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
                            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinition,
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

impl terrars
::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction {
    type O =
        terrars
        ::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction {
    #[doc = ""]
    pub action_name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction {
    pub fn build(self) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction {
            action_name: self.action_name,
            action_definition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomActionRef {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomActionRef {
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
    ) -> terrars
    ::ListRef<
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction_ActionDefinitionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_definition", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Destination {
    address_definition: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Destination {

}

impl terrars
::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Destination {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Destination,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Destination {
    #[doc = ""]
    pub address_definition: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Destination {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Destination {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Destination {
            address_definition: self.address_definition,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_DestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_DestinationRef {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_definition` after provisioning.\n"]
    pub fn address_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_definition", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_DestinationPort {
    from_port: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to_port: std::option::Option<terrars::PrimField<f64>>,
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_DestinationPort {
    #[doc = "Set the field `to_port`.\n"]
    pub fn set_to_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.to_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_DestinationPort {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_DestinationPort,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_DestinationPort {
    #[doc = ""]
    pub from_port: terrars::PrimField<f64>,
}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_DestinationPort {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_DestinationPort {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_DestinationPort {
            from_port: self.from_port,
            to_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_DestinationPortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_DestinationPortRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_DestinationPortRef {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_DestinationPortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_DestinationPortRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Source {
    address_definition: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Source {

}

impl terrars
::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Source {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Source,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Source {
    #[doc = ""]
    pub address_definition: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Source {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Source {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Source {
            address_definition: self.address_definition,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_SourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_SourceRef {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_definition` after provisioning.\n"]
    pub fn address_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_definition", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_SourcePort {
    from_port: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to_port: std::option::Option<terrars::PrimField<f64>>,
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_SourcePort {
    #[doc = "Set the field `to_port`.\n"]
    pub fn set_to_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.to_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_SourcePort {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_SourcePort,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_SourcePort {
    #[doc = ""]
    pub from_port: terrars::PrimField<f64>,
}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_SourcePort {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_SourcePort {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_SourcePort {
            from_port: self.from_port,
            to_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_SourcePortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_SourcePortRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_SourcePortRef {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_SourcePortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_SourcePortRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_TcpFlag {
    flags: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    masks: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_TcpFlag {
    #[doc = "Set the field `masks`.\n"]
    pub fn set_masks(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.masks = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_TcpFlag {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_TcpFlag,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_TcpFlag {
    #[doc = ""]
    pub flags: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_TcpFlag {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_TcpFlag {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_TcpFlag {
            flags: self.flags,
            masks: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_TcpFlagRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_TcpFlagRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_TcpFlagRef {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_TcpFlagRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_TcpFlagRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `flags` after provisioning.\n"]
    pub fn flags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.flags", self.base))
    }

    #[doc = "Get a reference to the value of field `masks` after provisioning.\n"]
    pub fn masks(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.masks", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributesDynamic {
    destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Destination,
        >,
    >,
    destination_port: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_DestinationPort,
        >,
    >,
    source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Source,
        >,
    >,
    source_port: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_SourcePort,
        >,
    >,
    tcp_flag: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_TcpFlag,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocols: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Destination,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_port: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_DestinationPort,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Source,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_port: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_SourcePort,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_flag: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_TcpFlag,
        >,
    >,
    dynamic: NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributesDynamic,
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes {
    #[doc = "Set the field `protocols`.\n"]
    pub fn set_protocols(mut self, v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>) -> Self {
        self.protocols = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Destination,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destination_port`.\n"]
    pub fn set_destination_port(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_DestinationPort,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_port = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_port = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_Source,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_port`.\n"]
    pub fn set_source_port(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_SourcePort,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_port = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_port = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tcp_flag`.\n"]
    pub fn set_tcp_flag(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes_TcpFlag,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tcp_flag = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tcp_flag = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes {}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes {
            protocols: core::default::Default::default(),
            destination: core::default::Default::default(),
            destination_port: core::default::Default::default(),
            source: core::default::Default::default(),
            source_port: core::default::Default::default(),
            tcp_flag: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributesRef {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `protocols` after provisioning.\n"]
    pub fn protocols(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.protocols", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinitionDynamic {
    match_attributes: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition {
    actions: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_attributes: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes,
        >,
    >,
    dynamic: NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinitionDynamic,
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition {
    #[doc = "Set the field `match_attributes`.\n"]
    pub fn set_match_attributes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_attributes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition {
    #[doc = ""]
    pub actions: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition {
            actions: self.actions,
            match_attributes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinitionRef {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }

    #[doc = "Get a reference to the value of field `match_attributes` after provisioning.\n"]
    pub fn match_attributes(
        &self,
    ) -> terrars
    ::ListRef<
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition_MatchAttributesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_attributes", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRuleDynamic {
    rule_definition: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule {
    priority: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_definition: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition,
        >,
    >,
    dynamic: NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRuleDynamic,
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule {
    #[doc = "Set the field `rule_definition`.\n"]
    pub fn set_rule_definition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rule_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_definition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule {
    #[doc = ""]
    pub priority: terrars::PrimField<f64>,
}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule {
            priority: self.priority,
            rule_definition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRuleRef {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_definition` after provisioning.\n"]
    pub fn rule_definition(
        &self,
    ) -> terrars
    ::ListRef<
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule_RuleDefinitionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_definition", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActionsDynamic {
    custom_action: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction>,
    >,
    stateless_rule: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule>,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_action: std
    ::option
    ::Option<
        std::vec::Vec<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateless_rule: std
    ::option
    ::Option<
        std::vec::Vec<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule>,
    >,
    dynamic: NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActionsDynamic,
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions {
    #[doc = "Set the field `custom_action`.\n"]
    pub fn set_custom_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_CustomAction,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stateless_rule`.\n"]
    pub fn set_stateless_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions_StatelessRule,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.stateless_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.stateless_rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions {}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions {
    pub fn build(self) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions {
            custom_action: core::default::Default::default(),
            stateless_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActionsRef {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroup_RuleGroup_RulesSourceDynamic {
    rules_source_list: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroup_RuleGroup_RulesSource_RulesSourceList>>,
    stateful_rule: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule>>,
    stateless_rules_and_custom_actions: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules_string: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules_source_list: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroup_RuleGroup_RulesSource_RulesSourceList>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_rule: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateless_rules_and_custom_actions: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions>>,
    dynamic: NetworkfirewallRuleGroup_RuleGroup_RulesSourceDynamic,
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSource {
    #[doc = "Set the field `rules_string`.\n"]
    pub fn set_rules_string(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rules_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rules_source_list`.\n"]
    pub fn set_rules_source_list(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RulesSource_RulesSourceList>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rules_source_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rules_source_list = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stateful_rule`.\n"]
    pub fn set_stateful_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.stateful_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.stateful_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stateless_rules_and_custom_actions`.\n"]
    pub fn set_stateless_rules_and_custom_actions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActions,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.stateless_rules_and_custom_actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.stateless_rules_and_custom_actions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_RulesSource {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RulesSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource {}

impl BuildNetworkfirewallRuleGroup_RuleGroup_RulesSource {
    pub fn build(self) -> NetworkfirewallRuleGroup_RuleGroup_RulesSource {
        NetworkfirewallRuleGroup_RuleGroup_RulesSource {
            rules_string: core::default::Default::default(),
            rules_source_list: core::default::Default::default(),
            stateful_rule: core::default::Default::default(),
            stateless_rules_and_custom_actions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_RulesSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroup_RuleGroup_RulesSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_RulesSourceRef {
        NetworkfirewallRuleGroup_RuleGroup_RulesSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_RulesSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rules_string` after provisioning.\n"]
    pub fn rules_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rules_string", self.base))
    }

    #[doc = "Get a reference to the value of field `rules_source_list` after provisioning.\n"]
    pub fn rules_source_list(
        &self,
    ) -> terrars::ListRef<NetworkfirewallRuleGroup_RuleGroup_RulesSource_RulesSourceListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules_source_list", self.base))
    }

    #[doc = "Get a reference to the value of field `stateful_rule` after provisioning.\n"]
    pub fn stateful_rule(&self) -> terrars::ListRef<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatefulRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `stateless_rules_and_custom_actions` after provisioning.\n"]
    pub fn stateless_rules_and_custom_actions(
        &self,
    ) -> terrars::ListRef<NetworkfirewallRuleGroup_RuleGroup_RulesSource_StatelessRulesAndCustomActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateless_rules_and_custom_actions", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup_StatefulRuleOptions {
    rule_order: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallRuleGroup_RuleGroup_StatefulRuleOptions { }

impl terrars::ToListMappable for NetworkfirewallRuleGroup_RuleGroup_StatefulRuleOptions {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_StatefulRuleOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup_StatefulRuleOptions {
    #[doc = ""]
    pub rule_order: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroup_RuleGroup_StatefulRuleOptions {
    pub fn build(self) -> NetworkfirewallRuleGroup_RuleGroup_StatefulRuleOptions {
        NetworkfirewallRuleGroup_RuleGroup_StatefulRuleOptions { rule_order: self.rule_order }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroup_StatefulRuleOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroup_RuleGroup_StatefulRuleOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroup_RuleGroup_StatefulRuleOptionsRef {
        NetworkfirewallRuleGroup_RuleGroup_StatefulRuleOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroup_StatefulRuleOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule_order` after provisioning.\n"]
    pub fn rule_order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_order", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroup_RuleGroupDynamic {
    reference_sets: std::option::Option<terrars::DynamicBlock<NetworkfirewallRuleGroup_RuleGroup_ReferenceSets>>,
    rule_variables: std::option::Option<terrars::DynamicBlock<NetworkfirewallRuleGroup_RuleGroup_RuleVariables>>,
    rules_source: std::option::Option<terrars::DynamicBlock<NetworkfirewallRuleGroup_RuleGroup_RulesSource>>,
    stateful_rule_options: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroup_RuleGroup_StatefulRuleOptions>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroup_RuleGroup {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reference_sets: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroup_RuleGroup_ReferenceSets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_variables: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroup_RuleGroup_RuleVariables>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules_source: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroup_RuleGroup_RulesSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_rule_options: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroup_RuleGroup_StatefulRuleOptions>>,
    dynamic: NetworkfirewallRuleGroup_RuleGroupDynamic,
}

impl NetworkfirewallRuleGroup_RuleGroup {
    #[doc = "Set the field `reference_sets`.\n"]
    pub fn set_reference_sets(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_ReferenceSets>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.reference_sets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.reference_sets = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule_variables`.\n"]
    pub fn set_rule_variables(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RuleVariables>>,
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

    #[doc = "Set the field `rules_source`.\n"]
    pub fn set_rules_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_RulesSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rules_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rules_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stateful_rule_options`.\n"]
    pub fn set_stateful_rule_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup_StatefulRuleOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.stateful_rule_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.stateful_rule_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallRuleGroup_RuleGroup {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroup_RuleGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroup_RuleGroup {}

impl BuildNetworkfirewallRuleGroup_RuleGroup {
    pub fn build(self) -> NetworkfirewallRuleGroup_RuleGroup {
        NetworkfirewallRuleGroup_RuleGroup {
            reference_sets: core::default::Default::default(),
            rule_variables: core::default::Default::default(),
            rules_source: core::default::Default::default(),
            stateful_rule_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallRuleGroup_RuleGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroup_RuleGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkfirewallRuleGroup_RuleGroupRef {
        NetworkfirewallRuleGroup_RuleGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroup_RuleGroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `reference_sets` after provisioning.\n"]
    pub fn reference_sets(&self) -> terrars::ListRef<NetworkfirewallRuleGroup_RuleGroup_ReferenceSetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reference_sets", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_variables` after provisioning.\n"]
    pub fn rule_variables(&self) -> terrars::ListRef<NetworkfirewallRuleGroup_RuleGroup_RuleVariablesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `rules_source` after provisioning.\n"]
    pub fn rules_source(&self) -> terrars::ListRef<NetworkfirewallRuleGroup_RuleGroup_RulesSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules_source", self.base))
    }

    #[doc = "Get a reference to the value of field `stateful_rule_options` after provisioning.\n"]
    pub fn stateful_rule_options(&self) -> terrars::ListRef<NetworkfirewallRuleGroup_RuleGroup_StatefulRuleOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_rule_options", self.base))
    }
}
