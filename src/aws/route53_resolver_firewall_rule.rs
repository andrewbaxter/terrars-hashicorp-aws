use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Route53ResolverFirewallRuleData {
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
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_override_dns_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_override_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_override_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_response: std::option::Option<terrars::PrimField<std::string::String>>,
    firewall_domain_list_id: terrars::PrimField<std::string::String>,
    firewall_rule_group_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    priority: terrars::PrimField<f64>,
}

struct Route53ResolverFirewallRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Route53ResolverFirewallRuleData>,
}

#[derive(Clone)]
pub struct Route53ResolverFirewallRule(std::rc::Rc<Route53ResolverFirewallRule_>);

impl Route53ResolverFirewallRule {
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

    #[doc = "Set the field `block_override_dns_type`.\n"]
    pub fn set_block_override_dns_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().block_override_dns_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `block_override_domain`.\n"]
    pub fn set_block_override_domain(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().block_override_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `block_override_ttl`.\n"]
    pub fn set_block_override_ttl(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().block_override_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `block_response`.\n"]
    pub fn set_block_response(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().block_response = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_override_dns_type` after provisioning.\n"]
    pub fn block_override_dns_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_override_dns_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_override_domain` after provisioning.\n"]
    pub fn block_override_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_override_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_override_ttl` after provisioning.\n"]
    pub fn block_override_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_override_ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_response` after provisioning.\n"]
    pub fn block_response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_response", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_domain_list_id` after provisioning.\n"]
    pub fn firewall_domain_list_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_domain_list_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_rule_group_id` after provisioning.\n"]
    pub fn firewall_rule_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_rule_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }
}

impl terrars::Referable for Route53ResolverFirewallRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Route53ResolverFirewallRule { }

impl terrars::ToListMappable for Route53ResolverFirewallRule {
    type O = terrars::ListRef<Route53ResolverFirewallRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Route53ResolverFirewallRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_route53_resolver_firewall_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRoute53ResolverFirewallRule {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub firewall_domain_list_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub firewall_rule_group_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub priority: terrars::PrimField<f64>,
}

impl BuildRoute53ResolverFirewallRule {
    pub fn build(self, stack: &mut terrars::Stack) -> Route53ResolverFirewallRule {
        let out = Route53ResolverFirewallRule(std::rc::Rc::new(Route53ResolverFirewallRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Route53ResolverFirewallRuleData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                action: self.action,
                block_override_dns_type: core::default::Default::default(),
                block_override_domain: core::default::Default::default(),
                block_override_ttl: core::default::Default::default(),
                block_response: core::default::Default::default(),
                firewall_domain_list_id: self.firewall_domain_list_id,
                firewall_rule_group_id: self.firewall_rule_group_id,
                id: core::default::Default::default(),
                name: self.name,
                priority: self.priority,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Route53ResolverFirewallRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53ResolverFirewallRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Route53ResolverFirewallRuleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_override_dns_type` after provisioning.\n"]
    pub fn block_override_dns_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_override_dns_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_override_domain` after provisioning.\n"]
    pub fn block_override_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_override_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_override_ttl` after provisioning.\n"]
    pub fn block_override_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_override_ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_response` after provisioning.\n"]
    pub fn block_response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_response", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_domain_list_id` after provisioning.\n"]
    pub fn firewall_domain_list_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_domain_list_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_rule_group_id` after provisioning.\n"]
    pub fn firewall_rule_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_rule_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }
}
