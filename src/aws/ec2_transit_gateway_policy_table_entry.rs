#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Ec2TransitGatewayPolicyTableEntryData_ {
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
    policy_rule_number: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    target_route_table_id: terrars::PrimField<std::string::String>,
    transit_gateway_policy_table_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_rule: std::option::Option<std::vec::Vec<Ec2TransitGatewayPolicyTableEntry_PolicyRule>>,
    dynamic: Ec2TransitGatewayPolicyTableEntryDynamic,
}

struct Ec2TransitGatewayPolicyTableEntry_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Ec2TransitGatewayPolicyTableEntryData_>,
}

#[derive(Clone)]
pub struct Ec2TransitGatewayPolicyTableEntry(std::rc::Rc<Ec2TransitGatewayPolicyTableEntry_>);

impl Ec2TransitGatewayPolicyTableEntry {
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

    #[doc = "Set the field `policy_rule`.\n"]
    pub fn set_policy_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2TransitGatewayPolicyTableEntry_PolicyRule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().policy_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.policy_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `policy_rule_number` after provisioning.\n"]
    pub fn policy_rule_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_rule_number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_route_table_id` after provisioning.\n"]
    pub fn target_route_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_route_table_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_policy_table_id` after provisioning.\n"]
    pub fn transit_gateway_policy_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_policy_table_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `policy_rule` after provisioning.\n"]
    pub fn policy_rule(&self) -> terrars::ListRef<Ec2TransitGatewayPolicyTableEntry_PolicyRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_rule", self.extract_ref()))
    }
}

impl terrars::Referable for Ec2TransitGatewayPolicyTableEntry {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Ec2TransitGatewayPolicyTableEntry { }

impl terrars::ToListMappable for Ec2TransitGatewayPolicyTableEntry {
    type O = terrars::ListRef<Ec2TransitGatewayPolicyTableEntryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Ec2TransitGatewayPolicyTableEntry_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ec2_transit_gateway_policy_table_entry".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEc2TransitGatewayPolicyTableEntry {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub policy_rule_number: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_route_table_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub transit_gateway_policy_table_id: terrars::PrimField<std::string::String>,
}

impl BuildEc2TransitGatewayPolicyTableEntry {
    pub fn build(self, stack: &mut terrars::Stack) -> Ec2TransitGatewayPolicyTableEntry {
        let out = Ec2TransitGatewayPolicyTableEntry(std::rc::Rc::new(Ec2TransitGatewayPolicyTableEntry_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Ec2TransitGatewayPolicyTableEntryData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                policy_rule_number: self.policy_rule_number,
                region: core::default::Default::default(),
                target_route_table_id: self.target_route_table_id,
                transit_gateway_policy_table_id: self.transit_gateway_policy_table_id,
                policy_rule: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Ec2TransitGatewayPolicyTableEntryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2TransitGatewayPolicyTableEntryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Ec2TransitGatewayPolicyTableEntryRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `policy_rule_number` after provisioning.\n"]
    pub fn policy_rule_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_rule_number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_route_table_id` after provisioning.\n"]
    pub fn target_route_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_route_table_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_policy_table_id` after provisioning.\n"]
    pub fn transit_gateway_policy_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_policy_table_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `policy_rule` after provisioning.\n"]
    pub fn policy_rule(&self) -> terrars::ListRef<Ec2TransitGatewayPolicyTableEntry_PolicyRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_rule", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Ec2TransitGatewayPolicyTableEntryDynamic {
    policy_rule: std::option::Option<terrars::DynamicBlock<Ec2TransitGatewayPolicyTableEntry_PolicyRule>>,
}

#[derive(Serialize)]
pub struct Ec2TransitGatewayPolicyTableEntry_PolicyRule_Metadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2TransitGatewayPolicyTableEntry_PolicyRule_Metadata {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2TransitGatewayPolicyTableEntry_PolicyRule_Metadata {
    type O = terrars::BlockAssignable<Ec2TransitGatewayPolicyTableEntry_PolicyRule_Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2TransitGatewayPolicyTableEntry_PolicyRule_Metadata {}

impl BuildEc2TransitGatewayPolicyTableEntry_PolicyRule_Metadata {
    pub fn build(self) -> Ec2TransitGatewayPolicyTableEntry_PolicyRule_Metadata {
        Ec2TransitGatewayPolicyTableEntry_PolicyRule_Metadata {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2TransitGatewayPolicyTableEntry_PolicyRule_MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2TransitGatewayPolicyTableEntry_PolicyRule_MetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2TransitGatewayPolicyTableEntry_PolicyRule_MetadataRef {
        Ec2TransitGatewayPolicyTableEntry_PolicyRule_MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2TransitGatewayPolicyTableEntry_PolicyRule_MetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct Ec2TransitGatewayPolicyTableEntry_PolicyRuleDynamic {
    metadata: std::option::Option<terrars::DynamicBlock<Ec2TransitGatewayPolicyTableEntry_PolicyRule_Metadata>>,
}

#[derive(Serialize)]
pub struct Ec2TransitGatewayPolicyTableEntry_PolicyRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_port_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_port_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<std::vec::Vec<Ec2TransitGatewayPolicyTableEntry_PolicyRule_Metadata>>,
    dynamic: Ec2TransitGatewayPolicyTableEntry_PolicyRuleDynamic,
}

impl Ec2TransitGatewayPolicyTableEntry_PolicyRule {
    #[doc = "Set the field `destination_cidr_block`.\n"]
    pub fn set_destination_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.destination_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_port_range`.\n"]
    pub fn set_destination_port_range(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.destination_port_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_cidr_block`.\n"]
    pub fn set_source_cidr_block(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_port_range`.\n"]
    pub fn set_source_port_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_port_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2TransitGatewayPolicyTableEntry_PolicyRule_Metadata>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Ec2TransitGatewayPolicyTableEntry_PolicyRule {
    type O = terrars::BlockAssignable<Ec2TransitGatewayPolicyTableEntry_PolicyRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2TransitGatewayPolicyTableEntry_PolicyRule {}

impl BuildEc2TransitGatewayPolicyTableEntry_PolicyRule {
    pub fn build(self) -> Ec2TransitGatewayPolicyTableEntry_PolicyRule {
        Ec2TransitGatewayPolicyTableEntry_PolicyRule {
            destination_cidr_block: core::default::Default::default(),
            destination_port_range: core::default::Default::default(),
            protocol: core::default::Default::default(),
            source_cidr_block: core::default::Default::default(),
            source_port_range: core::default::Default::default(),
            metadata: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2TransitGatewayPolicyTableEntry_PolicyRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2TransitGatewayPolicyTableEntry_PolicyRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2TransitGatewayPolicyTableEntry_PolicyRuleRef {
        Ec2TransitGatewayPolicyTableEntry_PolicyRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2TransitGatewayPolicyTableEntry_PolicyRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_cidr_block` after provisioning.\n"]
    pub fn destination_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_port_range` after provisioning.\n"]
    pub fn destination_port_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_port_range", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `source_cidr_block` after provisioning.\n"]
    pub fn source_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `source_port_range` after provisioning.\n"]
    pub fn source_port_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_port_range", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::ListRef<Ec2TransitGatewayPolicyTableEntry_PolicyRule_MetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}
