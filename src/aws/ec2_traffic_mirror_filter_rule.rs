#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Ec2TrafficMirrorFilterRuleData_ {
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
    destination_cidr_block: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    rule_action: terrars::PrimField<std::string::String>,
    rule_number: terrars::PrimField<f64>,
    source_cidr_block: terrars::PrimField<std::string::String>,
    traffic_direction: terrars::PrimField<std::string::String>,
    traffic_mirror_filter_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_port_range: std::option::Option<std::vec::Vec<Ec2TrafficMirrorFilterRule_DestinationPortRange>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_port_range: std::option::Option<std::vec::Vec<Ec2TrafficMirrorFilterRule_SourcePortRange>>,
    dynamic: Ec2TrafficMirrorFilterRuleDynamic,
}

struct Ec2TrafficMirrorFilterRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Ec2TrafficMirrorFilterRuleData_>,
}

#[derive(Clone)]
pub struct Ec2TrafficMirrorFilterRule(std::rc::Rc<Ec2TrafficMirrorFilterRule_>);

impl Ec2TrafficMirrorFilterRule {
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

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_port_range`.\n"]
    pub fn set_destination_port_range(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2TrafficMirrorFilterRule_DestinationPortRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_port_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_port_range = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_port_range`.\n"]
    pub fn set_source_port_range(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2TrafficMirrorFilterRule_SourcePortRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_port_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_port_range = std::option::Option::Some(d);
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

    #[doc = "Get a reference to the value of field `destination_cidr_block` after provisioning.\n"]
    pub fn destination_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_action` after provisioning.\n"]
    pub fn rule_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_number` after provisioning.\n"]
    pub fn rule_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_cidr_block` after provisioning.\n"]
    pub fn source_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_direction` after provisioning.\n"]
    pub fn traffic_direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_mirror_filter_id` after provisioning.\n"]
    pub fn traffic_mirror_filter_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_mirror_filter_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_port_range` after provisioning.\n"]
    pub fn destination_port_range(&self) -> terrars::ListRef<Ec2TrafficMirrorFilterRule_DestinationPortRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_port_range", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_port_range` after provisioning.\n"]
    pub fn source_port_range(&self) -> terrars::ListRef<Ec2TrafficMirrorFilterRule_SourcePortRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_port_range", self.extract_ref()))
    }
}

impl terrars::Referable for Ec2TrafficMirrorFilterRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Ec2TrafficMirrorFilterRule { }

impl terrars::ToListMappable for Ec2TrafficMirrorFilterRule {
    type O = terrars::ListRef<Ec2TrafficMirrorFilterRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Ec2TrafficMirrorFilterRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ec2_traffic_mirror_filter_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEc2TrafficMirrorFilterRule {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub destination_cidr_block: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule_action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule_number: terrars::PrimField<f64>,
    #[doc = ""]
    pub source_cidr_block: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub traffic_direction: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub traffic_mirror_filter_id: terrars::PrimField<std::string::String>,
}

impl BuildEc2TrafficMirrorFilterRule {
    pub fn build(self, stack: &mut terrars::Stack) -> Ec2TrafficMirrorFilterRule {
        let out = Ec2TrafficMirrorFilterRule(std::rc::Rc::new(Ec2TrafficMirrorFilterRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Ec2TrafficMirrorFilterRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                destination_cidr_block: self.destination_cidr_block,
                id: core::default::Default::default(),
                protocol: core::default::Default::default(),
                region: core::default::Default::default(),
                rule_action: self.rule_action,
                rule_number: self.rule_number,
                source_cidr_block: self.source_cidr_block,
                traffic_direction: self.traffic_direction,
                traffic_mirror_filter_id: self.traffic_mirror_filter_id,
                destination_port_range: core::default::Default::default(),
                source_port_range: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Ec2TrafficMirrorFilterRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2TrafficMirrorFilterRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Ec2TrafficMirrorFilterRuleRef {
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

    #[doc = "Get a reference to the value of field `destination_cidr_block` after provisioning.\n"]
    pub fn destination_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_action` after provisioning.\n"]
    pub fn rule_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_number` after provisioning.\n"]
    pub fn rule_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_cidr_block` after provisioning.\n"]
    pub fn source_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_direction` after provisioning.\n"]
    pub fn traffic_direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_mirror_filter_id` after provisioning.\n"]
    pub fn traffic_mirror_filter_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_mirror_filter_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_port_range` after provisioning.\n"]
    pub fn destination_port_range(&self) -> terrars::ListRef<Ec2TrafficMirrorFilterRule_DestinationPortRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_port_range", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_port_range` after provisioning.\n"]
    pub fn source_port_range(&self) -> terrars::ListRef<Ec2TrafficMirrorFilterRule_SourcePortRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_port_range", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Ec2TrafficMirrorFilterRuleDynamic {
    destination_port_range: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2TrafficMirrorFilterRule_DestinationPortRange>>,
    source_port_range: std::option::Option<terrars::DynamicBlock<Ec2TrafficMirrorFilterRule_SourcePortRange>>,
}

#[derive(Serialize)]
pub struct Ec2TrafficMirrorFilterRule_DestinationPortRange {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to_port: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2TrafficMirrorFilterRule_DestinationPortRange {
    #[doc = "Set the field `from_port`.\n"]
    pub fn set_from_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.from_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `to_port`.\n"]
    pub fn set_to_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.to_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2TrafficMirrorFilterRule_DestinationPortRange {
    type O = terrars::BlockAssignable<Ec2TrafficMirrorFilterRule_DestinationPortRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2TrafficMirrorFilterRule_DestinationPortRange {}

impl BuildEc2TrafficMirrorFilterRule_DestinationPortRange {
    pub fn build(self) -> Ec2TrafficMirrorFilterRule_DestinationPortRange {
        Ec2TrafficMirrorFilterRule_DestinationPortRange {
            from_port: core::default::Default::default(),
            to_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2TrafficMirrorFilterRule_DestinationPortRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2TrafficMirrorFilterRule_DestinationPortRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2TrafficMirrorFilterRule_DestinationPortRangeRef {
        Ec2TrafficMirrorFilterRule_DestinationPortRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2TrafficMirrorFilterRule_DestinationPortRangeRef {
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
pub struct Ec2TrafficMirrorFilterRule_SourcePortRange {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to_port: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2TrafficMirrorFilterRule_SourcePortRange {
    #[doc = "Set the field `from_port`.\n"]
    pub fn set_from_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.from_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `to_port`.\n"]
    pub fn set_to_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.to_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2TrafficMirrorFilterRule_SourcePortRange {
    type O = terrars::BlockAssignable<Ec2TrafficMirrorFilterRule_SourcePortRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2TrafficMirrorFilterRule_SourcePortRange {}

impl BuildEc2TrafficMirrorFilterRule_SourcePortRange {
    pub fn build(self) -> Ec2TrafficMirrorFilterRule_SourcePortRange {
        Ec2TrafficMirrorFilterRule_SourcePortRange {
            from_port: core::default::Default::default(),
            to_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2TrafficMirrorFilterRule_SourcePortRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2TrafficMirrorFilterRule_SourcePortRangeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ec2TrafficMirrorFilterRule_SourcePortRangeRef {
        Ec2TrafficMirrorFilterRule_SourcePortRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2TrafficMirrorFilterRule_SourcePortRangeRef {
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
