use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkfirewallFirewallPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_configuration: Option<Vec<NetworkfirewallFirewallPolicyEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    firewall_policy: Option<Vec<NetworkfirewallFirewallPolicyFirewallPolicyEl>>,
    dynamic: NetworkfirewallFirewallPolicyDynamic,
}

struct NetworkfirewallFirewallPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkfirewallFirewallPolicyData>,
}

#[derive(Clone)]
pub struct NetworkfirewallFirewallPolicy(Rc<NetworkfirewallFirewallPolicy_>);

impl NetworkfirewallFirewallPolicy {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl Into<BlockAssignable<NetworkfirewallFirewallPolicyEncryptionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `firewall_policy`.\n"]
    pub fn set_firewall_policy(
        self,
        v: impl Into<BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().firewall_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.firewall_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> ListRef<NetworkfirewallFirewallPolicyEncryptionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `firewall_policy` after provisioning.\n"]
    pub fn firewall_policy(&self) -> ListRef<NetworkfirewallFirewallPolicyFirewallPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.firewall_policy", self.extract_ref()))
    }
}

impl Resource for NetworkfirewallFirewallPolicy {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for NetworkfirewallFirewallPolicy {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for NetworkfirewallFirewallPolicy {
    type O = ListRef<NetworkfirewallFirewallPolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for NetworkfirewallFirewallPolicy_ {
    fn extract_resource_type(&self) -> String {
        "aws_networkfirewall_firewall_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkfirewallFirewallPolicy {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildNetworkfirewallFirewallPolicy {
    pub fn build(self, stack: &mut Stack) -> NetworkfirewallFirewallPolicy {
        let out = NetworkfirewallFirewallPolicy(Rc::new(NetworkfirewallFirewallPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkfirewallFirewallPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                encryption_configuration: core::default::Default::default(),
                firewall_policy: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkfirewallFirewallPolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallFirewallPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkfirewallFirewallPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> ListRef<NetworkfirewallFirewallPolicyEncryptionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `firewall_policy` after provisioning.\n"]
    pub fn firewall_policy(&self) -> ListRef<NetworkfirewallFirewallPolicyFirewallPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.firewall_policy", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicyEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key_id: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl NetworkfirewallFirewallPolicyEncryptionConfigurationEl {
    #[doc= "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_id = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkfirewallFirewallPolicyEncryptionConfigurationEl {
    type O = BlockAssignable<NetworkfirewallFirewallPolicyEncryptionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicyEncryptionConfigurationEl {
    #[doc= ""]
    pub type_: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallFirewallPolicyEncryptionConfigurationElRef {
    fn new(shared: StackShared, base: String) -> NetworkfirewallFirewallPolicyEncryptionConfigurationElRef {
        NetworkfirewallFirewallPolicyEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicyEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
    rule_order: PrimField<String>,
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl { }

impl ToListMappable for NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
    type O = BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
    #[doc= ""]
    pub rule_order: PrimField<String>,
}

impl BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
    pub fn build(self) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl { rule_order: self.rule_order }
    }
}

pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `rule_order` after provisioning.\n"]
    pub fn rule_order(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_order", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl {
    #[doc= "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl {
    type O = BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideElRef {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElDynamic {
    override_: Option<
        DynamicBlock<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl>,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    resource_arn: PrimField<String>,
    #[serde(rename = "override", skip_serializing_if = "Option::is_none")]
    override_: Option<Vec<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl>>,
    dynamic: NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElDynamic,
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
    #[doc= "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }

    #[doc= "Set the field `override_`.\n"]
    pub fn set_override(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.override_ = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.override_ = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
    type O = BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
    #[doc= ""]
    pub resource_arn: PrimField<String>,
}

impl BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
    pub fn build(self) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
            priority: core::default::Default::default(),
            resource_arn: self.resource_arn,
            override_: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc= "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(
        &self,
    ) -> ListRef<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElOverrideElRef> {
        ListRef::new(self.shared().clone(), format!("{}.override", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {
    value: PrimField<String>,
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {

}

impl ToListMappable for NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {
    type O =
        BlockAssignable<
            NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {
    #[doc= ""]
    pub value: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDynamic {
    dimension: Option<
        DynamicBlock<
            NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<
        Vec<
            NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl,
        >,
    >,
    dynamic: NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDynamic,
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl {
    #[doc= "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimension = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl {
    type O =
        BlockAssignable<
            NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElDynamic {
    publish_metric_action: Option<
        DynamicBlock<
            NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    publish_metric_action: Option<
        Vec<
            NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl,
        >,
    >,
    dynamic: NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElDynamic,
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl {
    #[doc= "Set the field `publish_metric_action`.\n"]
    pub fn set_publish_metric_action(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.publish_metric_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.publish_metric_action = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl {
    type O = BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `publish_metric_action` after provisioning.\n"]
    pub fn publish_metric_action(
        &self,
    ) -> ListRef<
        NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.publish_metric_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElDynamic {
    action_definition: Option<
        DynamicBlock<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl>,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
    action_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_definition: Option<
        Vec<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl>,
    >,
    dynamic: NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElDynamic,
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
    #[doc= "Set the field `action_definition`.\n"]
    pub fn set_action_definition(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action_definition = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action_definition = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
    type O = BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
    #[doc= ""]
    pub action_name: PrimField<String>,
}

impl BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
    pub fn build(self) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
            action_name: self.action_name,
            action_definition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `action_name` after provisioning.\n"]
    pub fn action_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_name", self.base))
    }

    #[doc= "Get a reference to the value of field `action_definition` after provisioning.\n"]
    pub fn action_definition(
        &self,
    ) -> ListRef<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action_definition", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {
    priority: PrimField<f64>,
    resource_arn: PrimField<String>,
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl { }

impl ToListMappable for NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {
    type O = BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {
    #[doc= ""]
    pub priority: PrimField<f64>,
    #[doc= ""]
    pub resource_arn: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceElRef {
        NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc= "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallPolicyFirewallPolicyElDynamic {
    stateful_engine_options: Option<
        DynamicBlock<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl>,
    >,
    stateful_rule_group_reference: Option<
        DynamicBlock<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl>,
    >,
    stateless_custom_action: Option<
        DynamicBlock<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl>,
    >,
    stateless_rule_group_reference: Option<
        DynamicBlock<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl>,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallPolicyFirewallPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    stateful_default_actions: Option<SetField<PrimField<String>>>,
    stateless_default_actions: SetField<PrimField<String>>,
    stateless_fragment_default_actions: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stateful_engine_options: Option<Vec<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stateful_rule_group_reference: Option<Vec<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stateless_custom_action: Option<Vec<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stateless_rule_group_reference: Option<
        Vec<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl>,
    >,
    dynamic: NetworkfirewallFirewallPolicyFirewallPolicyElDynamic,
}

impl NetworkfirewallFirewallPolicyFirewallPolicyEl {
    #[doc= "Set the field `stateful_default_actions`.\n"]
    pub fn set_stateful_default_actions(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.stateful_default_actions = Some(v.into());
        self
    }

    #[doc= "Set the field `stateful_engine_options`.\n"]
    pub fn set_stateful_engine_options(
        mut self,
        v: impl Into<BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.stateful_engine_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.stateful_engine_options = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `stateful_rule_group_reference`.\n"]
    pub fn set_stateful_rule_group_reference(
        mut self,
        v: impl Into<BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.stateful_rule_group_reference = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.stateful_rule_group_reference = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `stateless_custom_action`.\n"]
    pub fn set_stateless_custom_action(
        mut self,
        v: impl Into<BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.stateless_custom_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.stateless_custom_action = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `stateless_rule_group_reference`.\n"]
    pub fn set_stateless_rule_group_reference(
        mut self,
        v: impl Into<BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.stateless_rule_group_reference = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.stateless_rule_group_reference = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for NetworkfirewallFirewallPolicyFirewallPolicyEl {
    type O = BlockAssignable<NetworkfirewallFirewallPolicyFirewallPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallPolicyFirewallPolicyEl {
    #[doc= ""]
    pub stateless_default_actions: SetField<PrimField<String>>,
    #[doc= ""]
    pub stateless_fragment_default_actions: SetField<PrimField<String>>,
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
            dynamic: Default::default(),
        }
    }
}

pub struct NetworkfirewallFirewallPolicyFirewallPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallFirewallPolicyFirewallPolicyElRef {
    fn new(shared: StackShared, base: String) -> NetworkfirewallFirewallPolicyFirewallPolicyElRef {
        NetworkfirewallFirewallPolicyFirewallPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallPolicyFirewallPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `stateful_default_actions` after provisioning.\n"]
    pub fn stateful_default_actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.stateful_default_actions", self.base))
    }

    #[doc= "Get a reference to the value of field `stateless_default_actions` after provisioning.\n"]
    pub fn stateless_default_actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.stateless_default_actions", self.base))
    }

    #[doc= "Get a reference to the value of field `stateless_fragment_default_actions` after provisioning.\n"]
    pub fn stateless_fragment_default_actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.stateless_fragment_default_actions", self.base))
    }

    #[doc= "Get a reference to the value of field `stateful_engine_options` after provisioning.\n"]
    pub fn stateful_engine_options(
        &self,
    ) -> ListRef<NetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.stateful_engine_options", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallPolicyDynamic {
    encryption_configuration: Option<DynamicBlock<NetworkfirewallFirewallPolicyEncryptionConfigurationEl>>,
    firewall_policy: Option<DynamicBlock<NetworkfirewallFirewallPolicyFirewallPolicyEl>>,
}
