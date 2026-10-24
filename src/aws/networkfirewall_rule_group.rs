use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkfirewallRuleGroupData {
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
    rules: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroupEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_group: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroupRuleGroupEl>>,
    dynamic: NetworkfirewallRuleGroupDynamic,
}

struct NetworkfirewallRuleGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkfirewallRuleGroupData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkfirewallRuleGroupEncryptionConfigurationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupEl>>,
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
    pub fn encryption_configuration(&self) -> terrars::ListRef<NetworkfirewallRuleGroupEncryptionConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_group` after provisioning.\n"]
    pub fn rule_group(&self) -> terrars::ListRef<NetworkfirewallRuleGroupRuleGroupElRef> {
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
            data: std::cell::RefCell::new(NetworkfirewallRuleGroupData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                capacity: self.capacity,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
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
    pub fn encryption_configuration(&self) -> terrars::ListRef<NetworkfirewallRuleGroupEncryptionConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_group` after provisioning.\n"]
    pub fn rule_group(&self) -> terrars::ListRef<NetworkfirewallRuleGroupRuleGroupElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_group", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallRuleGroupEncryptionConfigurationEl {
    #[doc = "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallRuleGroupEncryptionConfigurationEl {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroupEncryptionConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupEncryptionConfigurationEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroupEncryptionConfigurationEl {
    pub fn build(self) -> NetworkfirewallRuleGroupEncryptionConfigurationEl {
        NetworkfirewallRuleGroupEncryptionConfigurationEl {
            key_id: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct NetworkfirewallRuleGroupEncryptionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroupEncryptionConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupEncryptionConfigurationElRef {
        NetworkfirewallRuleGroupEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupEncryptionConfigurationElRef {
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
pub struct NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl {
    reference_arn: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl { }

impl terrars::ToListMappable for NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl {
    type O =
        terrars
        ::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl {
    #[doc = ""]
    pub reference_arn: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl {
        NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl {
            reference_arn: self.reference_arn,
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceElRef {
        NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `reference_arn` after provisioning.\n"]
    pub fn reference_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reference_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElDynamic {
    ip_set_reference: std
    ::option
    ::Option<
        terrars::DynamicBlock<NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl>,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_set_reference: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl>>,
    dynamic: NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElDynamic,
}

impl NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl {
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
                            NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceEl,
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

impl terrars::ToListMappable for NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl {
        NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl {
            key: self.key,
            ip_set_reference: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElRef {
        NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElRef {
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
    ) -> terrars::ListRef<NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesElIpSetReferenceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_set_reference", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElReferenceSetsElDynamic {
    ip_set_references: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElReferenceSetsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_set_references: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl>>,
    dynamic: NetworkfirewallRuleGroupRuleGroupElReferenceSetsElDynamic,
}

impl NetworkfirewallRuleGroupRuleGroupElReferenceSetsEl {
    #[doc = "Set the field `ip_set_references`.\n"]
    pub fn set_ip_set_references(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElReferenceSetsElIpSetReferencesEl>,
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

impl terrars::ToListMappable for NetworkfirewallRuleGroupRuleGroupElReferenceSetsEl {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElReferenceSetsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElReferenceSetsEl {}

impl BuildNetworkfirewallRuleGroupRuleGroupElReferenceSetsEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElReferenceSetsEl {
        NetworkfirewallRuleGroupRuleGroupElReferenceSetsEl {
            ip_set_references: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElReferenceSetsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroupRuleGroupElReferenceSetsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElReferenceSetsElRef {
        NetworkfirewallRuleGroupRuleGroupElReferenceSetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElReferenceSetsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl {
    definition: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl { }

impl terrars::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl {
    #[doc = ""]
    pub definition: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl {
        NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl { definition: self.definition }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetElRef {
        NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.definition", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElDynamic {
    ip_set: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_set: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl>>,
    dynamic: NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElDynamic,
}

impl NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl {
    #[doc = "Set the field `ip_set`.\n"]
    pub fn set_ip_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetEl>,
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

impl terrars::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl {
        NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl {
            key: self.key,
            ip_set: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElRef {
        NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_set` after provisioning.\n"]
    pub fn ip_set(&self) -> terrars::ListRef<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsElIpSetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_set", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl {
    definition: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl { }

impl terrars::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl {
    #[doc = ""]
    pub definition: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl {
        NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl { definition: self.definition }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetElRef {
        NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.definition", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElDynamic {
    port_set: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_set: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl>>,
    dynamic: NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElDynamic,
}

impl NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl {
    #[doc = "Set the field `port_set`.\n"]
    pub fn set_port_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetEl>,
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

impl terrars::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl {
        NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl {
            key: self.key,
            port_set: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElRef {
        NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `port_set` after provisioning.\n"]
    pub fn port_set(&self) -> terrars::ListRef<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsElPortSetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_set", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElDynamic {
    ip_sets: std::option::Option<terrars::DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl>>,
    port_sets: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_sets: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_sets: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl>>,
    dynamic: NetworkfirewallRuleGroupRuleGroupElRuleVariablesElDynamic,
}

impl NetworkfirewallRuleGroupRuleGroupElRuleVariablesEl {
    #[doc = "Set the field `ip_sets`.\n"]
    pub fn set_ip_sets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElIpSetsEl>>,
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
                    ::Into<terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElPortSetsEl>>,
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

impl terrars::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRuleVariablesEl {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRuleVariablesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRuleVariablesEl {}

impl BuildNetworkfirewallRuleGroupRuleGroupElRuleVariablesEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElRuleVariablesEl {
        NetworkfirewallRuleGroupRuleGroupElRuleVariablesEl {
            ip_sets: core::default::Default::default(),
            port_sets: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRuleVariablesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroupRuleGroupElRuleVariablesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRuleVariablesElRef {
        NetworkfirewallRuleGroupRuleGroupElRuleVariablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRuleVariablesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl {
    generated_rules_type: terrars::PrimField<std::string::String>,
    target_types: terrars::SetField<terrars::PrimField<std::string::String>>,
    targets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl { }

impl terrars::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl {
    #[doc = ""]
    pub generated_rules_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_types: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub targets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl {
            generated_rules_type: self.generated_rules_type,
            target_types: self.target_types,
            targets: self.targets,
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListElRef {
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
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl {
    destination: terrars::PrimField<std::string::String>,
    destination_port: terrars::PrimField<std::string::String>,
    direction: terrars::PrimField<std::string::String>,
    protocol: terrars::PrimField<std::string::String>,
    source: terrars::PrimField<std::string::String>,
    source_port: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl { }

impl terrars::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl {
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

impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl {
            destination: self.destination,
            destination_port: self.destination_port,
            direction: self.direction,
            protocol: self.protocol,
            source: self.source,
            source_port: self.source_port,
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderElRef {
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
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl {
    keyword: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl {
    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.settings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl {
    #[doc = ""]
    pub keyword: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl {
            keyword: self.keyword,
            settings: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionElRef {
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
struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElDynamic {
    header: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl>>,
    rule_option: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl {
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_option: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl>>,
    dynamic: NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElDynamic,
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl {
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderEl>,
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
                        ::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRuleOptionEl>,
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

impl terrars::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl {
    #[doc = ""]
    pub action: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl {
            action: self.action,
            header: core::default::Default::default(),
            rule_option: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::ListRef<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {
    value: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {

}

impl terrars
::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {
            value: self.value,
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDynamic {
    dimension: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl,
        >,
    >,
    dynamic: NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDynamic,
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl {
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
                            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElDimensionEl,
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
::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl {}

impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl {
            dimension: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElDynamic {
    publish_metric_action: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_metric_action: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl,
        >,
    >,
    dynamic: NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElDynamic,
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl {
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
                            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionEl,
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
::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl {}

impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl {
            publish_metric_action: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `publish_metric_action` after provisioning.\n"]
    pub fn publish_metric_action(
        &self,
    ) -> terrars
    ::ListRef<
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElPublishMetricActionElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publish_metric_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElDynamic {
    action_definition: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl {
    action_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_definition: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl,
        >,
    >,
    dynamic: NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElDynamic,
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl {
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
                            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionEl,
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
::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl {
    #[doc = ""]
    pub action_name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl {
            action_name: self.action_name,
            action_definition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElRef {
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
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionElActionDefinitionElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_definition", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl {
    address_definition: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl {

}

impl terrars
::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl {
    #[doc = ""]
    pub address_definition: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl {
            address_definition: self.address_definition,
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_definition` after provisioning.\n"]
    pub fn address_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_definition", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl {
    from_port: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to_port: std::option::Option<terrars::PrimField<f64>>,
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl {
    #[doc = "Set the field `to_port`.\n"]
    pub fn set_to_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.to_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl {
    #[doc = ""]
    pub from_port: terrars::PrimField<f64>,
}

impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl {
            from_port: self.from_port,
            to_port: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortElRef {
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
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl {
    address_definition: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl {

}

impl terrars
::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl {
    #[doc = ""]
    pub address_definition: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl {
            address_definition: self.address_definition,
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_definition` after provisioning.\n"]
    pub fn address_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_definition", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl {
    from_port: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to_port: std::option::Option<terrars::PrimField<f64>>,
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl {
    #[doc = "Set the field `to_port`.\n"]
    pub fn set_to_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.to_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl {
    #[doc = ""]
    pub from_port: terrars::PrimField<f64>,
}

impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl {
            from_port: self.from_port,
            to_port: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortElRef {
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
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl {
    flags: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    masks: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl {
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
::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl {
    #[doc = ""]
    pub flags: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl {
            flags: self.flags,
            masks: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagElRef {
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
struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDynamic {
    destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl,
        >,
    >,
    destination_port: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl,
        >,
    >,
    source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl,
        >,
    >,
    source_port: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl,
        >,
    >,
    tcp_flag: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocols: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_port: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_port: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_flag: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl,
        >,
    >,
    dynamic: NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDynamic,
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl {
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
                            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationEl,
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
                            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElDestinationPortEl,
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
                            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourceEl,
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
                            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElSourcePortEl,
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
                            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElTcpFlagEl,
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
::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl {}

impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl {
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

pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `protocols` after provisioning.\n"]
    pub fn protocols(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.protocols", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElDynamic {
    match_attributes: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl {
    actions: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_attributes: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl,
        >,
    >,
    dynamic: NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElDynamic,
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl {
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
                            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesEl,
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
::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl {
    #[doc = ""]
    pub actions: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl {
            actions: self.actions,
            match_attributes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElRef {
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
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElMatchAttributesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_attributes", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElDynamic {
    rule_definition: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl {
    priority: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_definition: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl,
        >,
    >,
    dynamic: NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElDynamic,
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl {
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
                            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionEl,
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
::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl {
    #[doc = ""]
    pub priority: terrars::PrimField<f64>,
}

impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl {
    pub fn build(
        self,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl {
            priority: self.priority,
            rule_definition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRef {
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
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleElRuleDefinitionElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_definition", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElDynamic {
    custom_action: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl>,
    >,
    stateless_rule: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_action: std
    ::option
    ::Option<
        std::vec::Vec<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateless_rule: std
    ::option
    ::Option<
        std::vec::Vec<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl>,
    >,
    dynamic: NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElDynamic,
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl {
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
                            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElCustomActionEl,
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
                            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElStatelessRuleEl,
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

impl terrars::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl {
    type O =
        terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl {}

impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl {
            custom_action: core::default::Default::default(),
            stateless_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElDynamic {
    rules_source_list: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl>>,
    stateful_rule: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl>>,
    stateless_rules_and_custom_actions: std
    ::option
    ::Option<
        terrars::DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl>,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules_string: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules_source_list: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_rule: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateless_rules_and_custom_actions: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl>>,
    dynamic: NetworkfirewallRuleGroupRuleGroupElRulesSourceElDynamic,
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceEl {
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
                    ::Into<
                        terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListEl>,
                    >,
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
                    ::Into<terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleEl>>,
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
                            NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsEl,
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

impl terrars::ToListMappable for NetworkfirewallRuleGroupRuleGroupElRulesSourceEl {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRulesSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceEl {}

impl BuildNetworkfirewallRuleGroupRuleGroupElRulesSourceEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceEl {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceEl {
            rules_string: core::default::Default::default(),
            rules_source_list: core::default::Default::default(),
            stateful_rule: core::default::Default::default(),
            stateless_rules_and_custom_actions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRulesSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroupRuleGroupElRulesSourceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElRulesSourceElRef {
        NetworkfirewallRuleGroupRuleGroupElRulesSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRulesSourceElRef {
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
    ) -> terrars::ListRef<NetworkfirewallRuleGroupRuleGroupElRulesSourceElRulesSourceListElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules_source_list", self.base))
    }

    #[doc = "Get a reference to the value of field `stateful_rule` after provisioning.\n"]
    pub fn stateful_rule(&self) -> terrars::ListRef<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatefulRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `stateless_rules_and_custom_actions` after provisioning.\n"]
    pub fn stateless_rules_and_custom_actions(
        &self,
    ) -> terrars::ListRef<NetworkfirewallRuleGroupRuleGroupElRulesSourceElStatelessRulesAndCustomActionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateless_rules_and_custom_actions", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl {
    rule_order: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl { }

impl terrars::ToListMappable for NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl {
    #[doc = ""]
    pub rule_order: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl {
        NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl { rule_order: self.rule_order }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsElRef {
        NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule_order` after provisioning.\n"]
    pub fn rule_order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_order", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupRuleGroupElDynamic {
    reference_sets: std::option::Option<terrars::DynamicBlock<NetworkfirewallRuleGroupRuleGroupElReferenceSetsEl>>,
    rule_variables: std::option::Option<terrars::DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRuleVariablesEl>>,
    rules_source: std::option::Option<terrars::DynamicBlock<NetworkfirewallRuleGroupRuleGroupElRulesSourceEl>>,
    stateful_rule_options: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallRuleGroupRuleGroupEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reference_sets: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroupRuleGroupElReferenceSetsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_variables: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroupRuleGroupElRuleVariablesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules_source: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroupRuleGroupElRulesSourceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_rule_options: std::option::Option<std::vec::Vec<NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl>>,
    dynamic: NetworkfirewallRuleGroupRuleGroupElDynamic,
}

impl NetworkfirewallRuleGroupRuleGroupEl {
    #[doc = "Set the field `reference_sets`.\n"]
    pub fn set_reference_sets(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElReferenceSetsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRuleVariablesEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElRulesSourceEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsEl>>,
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

impl terrars::ToListMappable for NetworkfirewallRuleGroupRuleGroupEl {
    type O = terrars::BlockAssignable<NetworkfirewallRuleGroupRuleGroupEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallRuleGroupRuleGroupEl {}

impl BuildNetworkfirewallRuleGroupRuleGroupEl {
    pub fn build(self) -> NetworkfirewallRuleGroupRuleGroupEl {
        NetworkfirewallRuleGroupRuleGroupEl {
            reference_sets: core::default::Default::default(),
            rule_variables: core::default::Default::default(),
            rules_source: core::default::Default::default(),
            stateful_rule_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallRuleGroupRuleGroupElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallRuleGroupRuleGroupElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkfirewallRuleGroupRuleGroupElRef {
        NetworkfirewallRuleGroupRuleGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallRuleGroupRuleGroupElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `reference_sets` after provisioning.\n"]
    pub fn reference_sets(&self) -> terrars::ListRef<NetworkfirewallRuleGroupRuleGroupElReferenceSetsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reference_sets", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_variables` after provisioning.\n"]
    pub fn rule_variables(&self) -> terrars::ListRef<NetworkfirewallRuleGroupRuleGroupElRuleVariablesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `rules_source` after provisioning.\n"]
    pub fn rules_source(&self) -> terrars::ListRef<NetworkfirewallRuleGroupRuleGroupElRulesSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules_source", self.base))
    }

    #[doc = "Get a reference to the value of field `stateful_rule_options` after provisioning.\n"]
    pub fn stateful_rule_options(&self) -> terrars::ListRef<NetworkfirewallRuleGroupRuleGroupElStatefulRuleOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_rule_options", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallRuleGroupDynamic {
    encryption_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallRuleGroupEncryptionConfigurationEl>>,
    rule_group: std::option::Option<terrars::DynamicBlock<NetworkfirewallRuleGroupRuleGroupEl>>,
}
