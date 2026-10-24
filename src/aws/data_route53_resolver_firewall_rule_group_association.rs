use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataRoute53ResolverFirewallRuleGroupAssociationData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    firewall_rule_group_association_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataRoute53ResolverFirewallRuleGroupAssociation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataRoute53ResolverFirewallRuleGroupAssociationData>,
}

#[derive(Clone)]
pub struct DataRoute53ResolverFirewallRuleGroupAssociation(
    std::rc::Rc<DataRoute53ResolverFirewallRuleGroupAssociation_>,
);

impl DataRoute53ResolverFirewallRuleGroupAssociation {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creator_request_id` after provisioning.\n"]
    pub fn creator_request_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator_request_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_rule_group_association_id` after provisioning.\n"]
    pub fn firewall_rule_group_association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.firewall_rule_group_association_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `firewall_rule_group_id` after provisioning.\n"]
    pub fn firewall_rule_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_rule_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_owner_name` after provisioning.\n"]
    pub fn managed_owner_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_owner_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modification_time` after provisioning.\n"]
    pub fn modification_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modification_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mutation_protection` after provisioning.\n"]
    pub fn mutation_protection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mutation_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataRoute53ResolverFirewallRuleGroupAssociation {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataRoute53ResolverFirewallRuleGroupAssociation { }

impl terrars::ToListMappable for DataRoute53ResolverFirewallRuleGroupAssociation {
    type O = terrars::ListRef<DataRoute53ResolverFirewallRuleGroupAssociationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataRoute53ResolverFirewallRuleGroupAssociation_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_route53_resolver_firewall_rule_group_association".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRoute53ResolverFirewallRuleGroupAssociation {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub firewall_rule_group_association_id: terrars::PrimField<std::string::String>,
}

impl BuildDataRoute53ResolverFirewallRuleGroupAssociation {
    pub fn build(self, stack: &mut terrars::Stack) -> DataRoute53ResolverFirewallRuleGroupAssociation {
        let out =
            DataRoute53ResolverFirewallRuleGroupAssociation(
                std::rc::Rc::new(DataRoute53ResolverFirewallRuleGroupAssociation_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(DataRoute53ResolverFirewallRuleGroupAssociationData {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        firewall_rule_group_association_id: self.firewall_rule_group_association_id,
                        id: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRoute53ResolverFirewallRuleGroupAssociationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRoute53ResolverFirewallRuleGroupAssociationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRoute53ResolverFirewallRuleGroupAssociationRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creator_request_id` after provisioning.\n"]
    pub fn creator_request_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator_request_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_rule_group_association_id` after provisioning.\n"]
    pub fn firewall_rule_group_association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.firewall_rule_group_association_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `firewall_rule_group_id` after provisioning.\n"]
    pub fn firewall_rule_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_rule_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_owner_name` after provisioning.\n"]
    pub fn managed_owner_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_owner_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modification_time` after provisioning.\n"]
    pub fn modification_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modification_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mutation_protection` after provisioning.\n"]
    pub fn mutation_protection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mutation_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }
}
