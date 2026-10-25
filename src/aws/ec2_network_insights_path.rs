#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Ec2NetworkInsightsPathData_ {
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
    destination: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    protocol: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    source: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_at_destination: std::option::Option<std::vec::Vec<Ec2NetworkInsightsPath_FilterAtDestination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_at_source: std::option::Option<std::vec::Vec<Ec2NetworkInsightsPath_FilterAtSource>>,
    dynamic: Ec2NetworkInsightsPathDynamic,
}

struct Ec2NetworkInsightsPath_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Ec2NetworkInsightsPathData_>,
}

#[derive(Clone)]
pub struct Ec2NetworkInsightsPath(std::rc::Rc<Ec2NetworkInsightsPath_>);

impl Ec2NetworkInsightsPath {
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

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_ip`.\n"]
    pub fn set_destination_ip(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().destination_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_port`.\n"]
    pub fn set_destination_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().destination_port = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `source_ip`.\n"]
    pub fn set_source_ip(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source_ip = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `filter_at_destination`.\n"]
    pub fn set_filter_at_destination(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2NetworkInsightsPath_FilterAtDestination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter_at_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter_at_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter_at_source`.\n"]
    pub fn set_filter_at_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2NetworkInsightsPath_FilterAtSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter_at_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter_at_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_arn` after provisioning.\n"]
    pub fn destination_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_ip` after provisioning.\n"]
    pub fn destination_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_port` after provisioning.\n"]
    pub fn destination_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_arn` after provisioning.\n"]
    pub fn source_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_ip` after provisioning.\n"]
    pub fn source_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_at_destination` after provisioning.\n"]
    pub fn filter_at_destination(&self) -> terrars::ListRef<Ec2NetworkInsightsPath_FilterAtDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_at_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_at_source` after provisioning.\n"]
    pub fn filter_at_source(&self) -> terrars::ListRef<Ec2NetworkInsightsPath_FilterAtSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_at_source", self.extract_ref()))
    }
}

impl terrars::Referable for Ec2NetworkInsightsPath {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Ec2NetworkInsightsPath { }

impl terrars::ToListMappable for Ec2NetworkInsightsPath {
    type O = terrars::ListRef<Ec2NetworkInsightsPathRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Ec2NetworkInsightsPath_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ec2_network_insights_path".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEc2NetworkInsightsPath {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub protocol: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source: terrars::PrimField<std::string::String>,
}

impl BuildEc2NetworkInsightsPath {
    pub fn build(self, stack: &mut terrars::Stack) -> Ec2NetworkInsightsPath {
        let out = Ec2NetworkInsightsPath(std::rc::Rc::new(Ec2NetworkInsightsPath_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Ec2NetworkInsightsPathData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                destination: core::default::Default::default(),
                destination_ip: core::default::Default::default(),
                destination_port: core::default::Default::default(),
                id: core::default::Default::default(),
                protocol: self.protocol,
                region: core::default::Default::default(),
                source: self.source,
                source_ip: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                filter_at_destination: core::default::Default::default(),
                filter_at_source: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Ec2NetworkInsightsPathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsPathRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Ec2NetworkInsightsPathRef {
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

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_arn` after provisioning.\n"]
    pub fn destination_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_ip` after provisioning.\n"]
    pub fn destination_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_port` after provisioning.\n"]
    pub fn destination_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_arn` after provisioning.\n"]
    pub fn source_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_ip` after provisioning.\n"]
    pub fn source_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_at_destination` after provisioning.\n"]
    pub fn filter_at_destination(&self) -> terrars::ListRef<Ec2NetworkInsightsPath_FilterAtDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_at_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_at_source` after provisioning.\n"]
    pub fn filter_at_source(&self) -> terrars::ListRef<Ec2NetworkInsightsPath_FilterAtSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_at_source", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Ec2NetworkInsightsPathDynamic {
    filter_at_destination: std::option::Option<terrars::DynamicBlock<Ec2NetworkInsightsPath_FilterAtDestination>>,
    filter_at_source: std::option::Option<terrars::DynamicBlock<Ec2NetworkInsightsPath_FilterAtSource>>,
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsPath_FilterAtSource_DestinationPortRange {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to_port: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsPath_FilterAtSource_DestinationPortRange {
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

impl terrars::ToListMappable for Ec2NetworkInsightsPath_FilterAtSource_DestinationPortRange {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsPath_FilterAtSource_DestinationPortRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsPath_FilterAtSource_DestinationPortRange {}

impl BuildEc2NetworkInsightsPath_FilterAtSource_DestinationPortRange {
    pub fn build(self) -> Ec2NetworkInsightsPath_FilterAtSource_DestinationPortRange {
        Ec2NetworkInsightsPath_FilterAtSource_DestinationPortRange {
            from_port: core::default::Default::default(),
            to_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsPath_FilterAtSource_DestinationPortRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsPath_FilterAtSource_DestinationPortRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsPath_FilterAtSource_DestinationPortRangeRef {
        Ec2NetworkInsightsPath_FilterAtSource_DestinationPortRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsPath_FilterAtSource_DestinationPortRangeRef {
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
pub struct Ec2NetworkInsightsPath_FilterAtSource_SourcePortRange {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to_port: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsPath_FilterAtSource_SourcePortRange {
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

impl terrars::ToListMappable for Ec2NetworkInsightsPath_FilterAtSource_SourcePortRange {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsPath_FilterAtSource_SourcePortRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsPath_FilterAtSource_SourcePortRange {}

impl BuildEc2NetworkInsightsPath_FilterAtSource_SourcePortRange {
    pub fn build(self) -> Ec2NetworkInsightsPath_FilterAtSource_SourcePortRange {
        Ec2NetworkInsightsPath_FilterAtSource_SourcePortRange {
            from_port: core::default::Default::default(),
            to_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsPath_FilterAtSource_SourcePortRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsPath_FilterAtSource_SourcePortRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsPath_FilterAtSource_SourcePortRangeRef {
        Ec2NetworkInsightsPath_FilterAtSource_SourcePortRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsPath_FilterAtSource_SourcePortRangeRef {
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

#[derive(Serialize, Default)]
struct Ec2NetworkInsightsPath_FilterAtDestinationDynamic {
    destination_port_range: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2NetworkInsightsPath_FilterAtSource_DestinationPortRange>>,
    source_port_range: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2NetworkInsightsPath_FilterAtSource_SourcePortRange>>,
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsPath_FilterAtDestination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_port_range: std
    ::option
    ::Option<std::vec::Vec<Ec2NetworkInsightsPath_FilterAtSource_DestinationPortRange>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_port_range: std::option::Option<std::vec::Vec<Ec2NetworkInsightsPath_FilterAtSource_SourcePortRange>>,
    dynamic: Ec2NetworkInsightsPath_FilterAtDestinationDynamic,
}

impl Ec2NetworkInsightsPath_FilterAtDestination {
    #[doc = "Set the field `destination_address`.\n"]
    pub fn set_destination_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.destination_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_address`.\n"]
    pub fn set_source_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_port_range`.\n"]
    pub fn set_destination_port_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Ec2NetworkInsightsPath_FilterAtSource_DestinationPortRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_port_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_port_range = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_port_range`.\n"]
    pub fn set_source_port_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2NetworkInsightsPath_FilterAtSource_SourcePortRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_port_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_port_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsPath_FilterAtDestination {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsPath_FilterAtDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsPath_FilterAtDestination {}

impl BuildEc2NetworkInsightsPath_FilterAtDestination {
    pub fn build(self) -> Ec2NetworkInsightsPath_FilterAtDestination {
        Ec2NetworkInsightsPath_FilterAtDestination {
            destination_address: core::default::Default::default(),
            source_address: core::default::Default::default(),
            destination_port_range: core::default::Default::default(),
            source_port_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsPath_FilterAtDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsPath_FilterAtDestinationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ec2NetworkInsightsPath_FilterAtDestinationRef {
        Ec2NetworkInsightsPath_FilterAtDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsPath_FilterAtDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_address` after provisioning.\n"]
    pub fn destination_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_address", self.base))
    }

    #[doc = "Get a reference to the value of field `source_address` after provisioning.\n"]
    pub fn source_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_address", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_port_range` after provisioning.\n"]
    pub fn destination_port_range(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsPath_FilterAtSource_DestinationPortRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_port_range", self.base))
    }

    #[doc = "Get a reference to the value of field `source_port_range` after provisioning.\n"]
    pub fn source_port_range(&self) -> terrars::ListRef<Ec2NetworkInsightsPath_FilterAtSource_SourcePortRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_port_range", self.base))
    }
}

#[derive(Serialize, Default)]
struct Ec2NetworkInsightsPath_FilterAtSourceDynamic {
    destination_port_range: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2NetworkInsightsPath_FilterAtSource_DestinationPortRange>>,
    source_port_range: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2NetworkInsightsPath_FilterAtSource_SourcePortRange>>,
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsPath_FilterAtSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_port_range: std
    ::option
    ::Option<std::vec::Vec<Ec2NetworkInsightsPath_FilterAtSource_DestinationPortRange>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_port_range: std::option::Option<std::vec::Vec<Ec2NetworkInsightsPath_FilterAtSource_SourcePortRange>>,
    dynamic: Ec2NetworkInsightsPath_FilterAtSourceDynamic,
}

impl Ec2NetworkInsightsPath_FilterAtSource {
    #[doc = "Set the field `destination_address`.\n"]
    pub fn set_destination_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.destination_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_address`.\n"]
    pub fn set_source_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_port_range`.\n"]
    pub fn set_destination_port_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Ec2NetworkInsightsPath_FilterAtSource_DestinationPortRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_port_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_port_range = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_port_range`.\n"]
    pub fn set_source_port_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2NetworkInsightsPath_FilterAtSource_SourcePortRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_port_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_port_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsPath_FilterAtSource {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsPath_FilterAtSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsPath_FilterAtSource {}

impl BuildEc2NetworkInsightsPath_FilterAtSource {
    pub fn build(self) -> Ec2NetworkInsightsPath_FilterAtSource {
        Ec2NetworkInsightsPath_FilterAtSource {
            destination_address: core::default::Default::default(),
            source_address: core::default::Default::default(),
            destination_port_range: core::default::Default::default(),
            source_port_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsPath_FilterAtSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsPath_FilterAtSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ec2NetworkInsightsPath_FilterAtSourceRef {
        Ec2NetworkInsightsPath_FilterAtSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsPath_FilterAtSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_address` after provisioning.\n"]
    pub fn destination_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_address", self.base))
    }

    #[doc = "Get a reference to the value of field `source_address` after provisioning.\n"]
    pub fn source_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_address", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_port_range` after provisioning.\n"]
    pub fn destination_port_range(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsPath_FilterAtSource_DestinationPortRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_port_range", self.base))
    }

    #[doc = "Get a reference to the value of field `source_port_range` after provisioning.\n"]
    pub fn source_port_range(&self) -> terrars::ListRef<Ec2NetworkInsightsPath_FilterAtSource_SourcePortRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_port_range", self.base))
    }
}
