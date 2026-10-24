use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataRoute53ResolverFirewallRulesData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    firewall_rule_group_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
}

struct DataRoute53ResolverFirewallRules_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataRoute53ResolverFirewallRulesData>,
}

#[derive(Clone)]
pub struct DataRoute53ResolverFirewallRules(std::rc::Rc<DataRoute53ResolverFirewallRules_>);

impl DataRoute53ResolverFirewallRules {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_rule_group_id` after provisioning.\n"]
    pub fn firewall_rule_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_rule_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_rules` after provisioning.\n"]
    pub fn firewall_rules(&self) -> terrars::ListRef<DataRoute53ResolverFirewallRulesFirewallRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firewall_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }
}

impl terrars::Referable for DataRoute53ResolverFirewallRules {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataRoute53ResolverFirewallRules { }

impl terrars::ToListMappable for DataRoute53ResolverFirewallRules {
    type O = terrars::ListRef<DataRoute53ResolverFirewallRulesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataRoute53ResolverFirewallRules_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_route53_resolver_firewall_rules".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRoute53ResolverFirewallRules {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub firewall_rule_group_id: terrars::PrimField<std::string::String>,
}

impl BuildDataRoute53ResolverFirewallRules {
    pub fn build(self, stack: &mut terrars::Stack) -> DataRoute53ResolverFirewallRules {
        let out = DataRoute53ResolverFirewallRules(std::rc::Rc::new(DataRoute53ResolverFirewallRules_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataRoute53ResolverFirewallRulesData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                action: core::default::Default::default(),
                firewall_rule_group_id: self.firewall_rule_group_id,
                id: core::default::Default::default(),
                priority: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRoute53ResolverFirewallRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRoute53ResolverFirewallRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRoute53ResolverFirewallRulesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_rule_group_id` after provisioning.\n"]
    pub fn firewall_rule_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_rule_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_rules` after provisioning.\n"]
    pub fn firewall_rules(&self) -> terrars::ListRef<DataRoute53ResolverFirewallRulesFirewallRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firewall_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRoute53ResolverFirewallRulesFirewallRulesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_override_dns_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_override_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_override_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_response: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    creation_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    creator_request_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firewall_domain_list_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firewall_rule_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    modification_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
}

impl DataRoute53ResolverFirewallRulesFirewallRulesEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `block_override_dns_type`.\n"]
    pub fn set_block_override_dns_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.block_override_dns_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `block_override_domain`.\n"]
    pub fn set_block_override_domain(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.block_override_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `block_override_ttl`.\n"]
    pub fn set_block_override_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.block_override_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `block_response`.\n"]
    pub fn set_block_response(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.block_response = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `creation_time`.\n"]
    pub fn set_creation_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.creation_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `creator_request_id`.\n"]
    pub fn set_creator_request_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.creator_request_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `firewall_domain_list_id`.\n"]
    pub fn set_firewall_domain_list_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.firewall_domain_list_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `firewall_rule_group_id`.\n"]
    pub fn set_firewall_rule_group_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.firewall_rule_group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `modification_time`.\n"]
    pub fn set_modification_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.modification_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRoute53ResolverFirewallRulesFirewallRulesEl {
    type O = terrars::BlockAssignable<DataRoute53ResolverFirewallRulesFirewallRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRoute53ResolverFirewallRulesFirewallRulesEl {}

impl BuildDataRoute53ResolverFirewallRulesFirewallRulesEl {
    pub fn build(self) -> DataRoute53ResolverFirewallRulesFirewallRulesEl {
        DataRoute53ResolverFirewallRulesFirewallRulesEl {
            action: core::default::Default::default(),
            block_override_dns_type: core::default::Default::default(),
            block_override_domain: core::default::Default::default(),
            block_override_ttl: core::default::Default::default(),
            block_response: core::default::Default::default(),
            creation_time: core::default::Default::default(),
            creator_request_id: core::default::Default::default(),
            firewall_domain_list_id: core::default::Default::default(),
            firewall_rule_group_id: core::default::Default::default(),
            modification_time: core::default::Default::default(),
            name: core::default::Default::default(),
            priority: core::default::Default::default(),
        }
    }
}

pub struct DataRoute53ResolverFirewallRulesFirewallRulesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRoute53ResolverFirewallRulesFirewallRulesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRoute53ResolverFirewallRulesFirewallRulesElRef {
        DataRoute53ResolverFirewallRulesFirewallRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRoute53ResolverFirewallRulesFirewallRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `block_override_dns_type` after provisioning.\n"]
    pub fn block_override_dns_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_override_dns_type", self.base))
    }

    #[doc = "Get a reference to the value of field `block_override_domain` after provisioning.\n"]
    pub fn block_override_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_override_domain", self.base))
    }

    #[doc = "Get a reference to the value of field `block_override_ttl` after provisioning.\n"]
    pub fn block_override_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_override_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `block_response` after provisioning.\n"]
    pub fn block_response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_response", self.base))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.base))
    }

    #[doc = "Get a reference to the value of field `creator_request_id` after provisioning.\n"]
    pub fn creator_request_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator_request_id", self.base))
    }

    #[doc = "Get a reference to the value of field `firewall_domain_list_id` after provisioning.\n"]
    pub fn firewall_domain_list_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_domain_list_id", self.base))
    }

    #[doc = "Get a reference to the value of field `firewall_rule_group_id` after provisioning.\n"]
    pub fn firewall_rule_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_rule_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `modification_time` after provisioning.\n"]
    pub fn modification_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modification_time", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
}
