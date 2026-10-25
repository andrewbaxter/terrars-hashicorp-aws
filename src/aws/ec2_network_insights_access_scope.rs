#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Ec2NetworkInsightsAccessScopeData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_paths: std::option::Option<std::vec::Vec<Ec2NetworkInsightsAccessScope_ExcludePaths>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_paths: std::option::Option<std::vec::Vec<Ec2NetworkInsightsAccessScope_MatchPaths>>,
    dynamic: Ec2NetworkInsightsAccessScopeDynamic,
}

struct Ec2NetworkInsightsAccessScope_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Ec2NetworkInsightsAccessScopeData_>,
}

#[derive(Clone)]
pub struct Ec2NetworkInsightsAccessScope(std::rc::Rc<Ec2NetworkInsightsAccessScope_>);

impl Ec2NetworkInsightsAccessScope {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_paths`.\n"]
    pub fn set_exclude_paths(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2NetworkInsightsAccessScope_ExcludePaths>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().exclude_paths = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.exclude_paths = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `match_paths`.\n"]
    pub fn set_match_paths(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2NetworkInsightsAccessScope_MatchPaths>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().match_paths = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.match_paths = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `exclude_paths` after provisioning.\n"]
    pub fn exclude_paths(&self) -> terrars::ListRef<Ec2NetworkInsightsAccessScope_ExcludePathsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_paths", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `match_paths` after provisioning.\n"]
    pub fn match_paths(&self) -> terrars::ListRef<Ec2NetworkInsightsAccessScope_MatchPathsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_paths", self.extract_ref()))
    }
}

impl terrars::Referable for Ec2NetworkInsightsAccessScope {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Ec2NetworkInsightsAccessScope { }

impl terrars::ToListMappable for Ec2NetworkInsightsAccessScope {
    type O = terrars::ListRef<Ec2NetworkInsightsAccessScopeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Ec2NetworkInsightsAccessScope_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ec2_network_insights_access_scope".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEc2NetworkInsightsAccessScope {
    pub tf_id: std::string::String,
}

impl BuildEc2NetworkInsightsAccessScope {
    pub fn build(self, stack: &mut terrars::Stack) -> Ec2NetworkInsightsAccessScope {
        let out = Ec2NetworkInsightsAccessScope(std::rc::Rc::new(Ec2NetworkInsightsAccessScope_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Ec2NetworkInsightsAccessScopeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                exclude_paths: core::default::Default::default(),
                match_paths: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Ec2NetworkInsightsAccessScopeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAccessScopeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Ec2NetworkInsightsAccessScopeRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `exclude_paths` after provisioning.\n"]
    pub fn exclude_paths(&self) -> terrars::ListRef<Ec2NetworkInsightsAccessScope_ExcludePathsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_paths", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `match_paths` after provisioning.\n"]
    pub fn match_paths(&self) -> terrars::ListRef<Ec2NetworkInsightsAccessScope_MatchPathsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_paths", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Ec2NetworkInsightsAccessScopeDynamic {
    exclude_paths: std::option::Option<terrars::DynamicBlock<Ec2NetworkInsightsAccessScope_ExcludePaths>>,
    match_paths: std::option::Option<terrars::DynamicBlock<Ec2NetworkInsightsAccessScope_MatchPaths>>,
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAccessScope_MatchPaths_Source_PacketHeaderStatement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_ports: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_prefix_lists: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocols: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_ports: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_prefix_lists: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Ec2NetworkInsightsAccessScope_MatchPaths_Source_PacketHeaderStatement {
    #[doc = "Set the field `destination_addresses`.\n"]
    pub fn set_destination_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.destination_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_ports`.\n"]
    pub fn set_destination_ports(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.destination_ports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_prefix_lists`.\n"]
    pub fn set_destination_prefix_lists(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.destination_prefix_lists = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocols`.\n"]
    pub fn set_protocols(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.protocols = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_addresses`.\n"]
    pub fn set_source_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.source_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_ports`.\n"]
    pub fn set_source_ports(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.source_ports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_prefix_lists`.\n"]
    pub fn set_source_prefix_lists(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.source_prefix_lists = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAccessScope_MatchPaths_Source_PacketHeaderStatement {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAccessScope_MatchPaths_Source_PacketHeaderStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAccessScope_MatchPaths_Source_PacketHeaderStatement {}

impl BuildEc2NetworkInsightsAccessScope_MatchPaths_Source_PacketHeaderStatement {
    pub fn build(self) -> Ec2NetworkInsightsAccessScope_MatchPaths_Source_PacketHeaderStatement {
        Ec2NetworkInsightsAccessScope_MatchPaths_Source_PacketHeaderStatement {
            destination_addresses: core::default::Default::default(),
            destination_ports: core::default::Default::default(),
            destination_prefix_lists: core::default::Default::default(),
            protocols: core::default::Default::default(),
            source_addresses: core::default::Default::default(),
            source_ports: core::default::Default::default(),
            source_prefix_lists: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAccessScope_MatchPaths_Source_PacketHeaderStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAccessScope_MatchPaths_Source_PacketHeaderStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAccessScope_MatchPaths_Source_PacketHeaderStatementRef {
        Ec2NetworkInsightsAccessScope_MatchPaths_Source_PacketHeaderStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAccessScope_MatchPaths_Source_PacketHeaderStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_addresses` after provisioning.\n"]
    pub fn destination_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_ports` after provisioning.\n"]
    pub fn destination_ports(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_ports", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_prefix_lists` after provisioning.\n"]
    pub fn destination_prefix_lists(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_prefix_lists", self.base))
    }

    #[doc = "Get a reference to the value of field `protocols` after provisioning.\n"]
    pub fn protocols(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protocols", self.base))
    }

    #[doc = "Get a reference to the value of field `source_addresses` after provisioning.\n"]
    pub fn source_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `source_ports` after provisioning.\n"]
    pub fn source_ports(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_ports", self.base))
    }

    #[doc = "Get a reference to the value of field `source_prefix_lists` after provisioning.\n"]
    pub fn source_prefix_lists(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_prefix_lists", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatement {
    #[doc = "Set the field `resource_types`.\n"]
    pub fn set_resource_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatement {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatement {}

impl BuildEc2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatement {
    pub fn build(self) -> Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatement {
        Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatement {
            resource_types: core::default::Default::default(),
            resources: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatementRef {
        Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_types` after provisioning.\n"]
    pub fn resource_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_types", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
}

#[derive(Serialize, Default)]
struct Ec2NetworkInsightsAccessScope_MatchPaths_DestinationDynamic {
    packet_header_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2NetworkInsightsAccessScope_MatchPaths_Source_PacketHeaderStatement>>,
    resource_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatement>>,
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAccessScope_MatchPaths_Destination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    packet_header_statement: std
    ::option
    ::Option<std::vec::Vec<Ec2NetworkInsightsAccessScope_MatchPaths_Source_PacketHeaderStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_statement: std
    ::option
    ::Option<std::vec::Vec<Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatement>>,
    dynamic: Ec2NetworkInsightsAccessScope_MatchPaths_DestinationDynamic,
}

impl Ec2NetworkInsightsAccessScope_MatchPaths_Destination {
    #[doc = "Set the field `packet_header_statement`.\n"]
    pub fn set_packet_header_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Ec2NetworkInsightsAccessScope_MatchPaths_Source_PacketHeaderStatement>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.packet_header_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.packet_header_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_statement`.\n"]
    pub fn set_resource_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatement>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_statement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAccessScope_MatchPaths_Destination {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAccessScope_MatchPaths_Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAccessScope_MatchPaths_Destination {}

impl BuildEc2NetworkInsightsAccessScope_MatchPaths_Destination {
    pub fn build(self) -> Ec2NetworkInsightsAccessScope_MatchPaths_Destination {
        Ec2NetworkInsightsAccessScope_MatchPaths_Destination {
            packet_header_statement: core::default::Default::default(),
            resource_statement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAccessScope_MatchPaths_DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAccessScope_MatchPaths_DestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAccessScope_MatchPaths_DestinationRef {
        Ec2NetworkInsightsAccessScope_MatchPaths_DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAccessScope_MatchPaths_DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `packet_header_statement` after provisioning.\n"]
    pub fn packet_header_statement(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAccessScope_MatchPaths_Source_PacketHeaderStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.packet_header_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_statement` after provisioning.\n"]
    pub fn resource_statement(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_statement", self.base))
    }
}

#[derive(Serialize, Default)]
struct Ec2NetworkInsightsAccessScope_MatchPaths_SourceDynamic {
    packet_header_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2NetworkInsightsAccessScope_MatchPaths_Source_PacketHeaderStatement>>,
    resource_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatement>>,
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAccessScope_MatchPaths_Source {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    packet_header_statement: std
    ::option
    ::Option<std::vec::Vec<Ec2NetworkInsightsAccessScope_MatchPaths_Source_PacketHeaderStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_statement: std
    ::option
    ::Option<std::vec::Vec<Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatement>>,
    dynamic: Ec2NetworkInsightsAccessScope_MatchPaths_SourceDynamic,
}

impl Ec2NetworkInsightsAccessScope_MatchPaths_Source {
    #[doc = "Set the field `packet_header_statement`.\n"]
    pub fn set_packet_header_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Ec2NetworkInsightsAccessScope_MatchPaths_Source_PacketHeaderStatement>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.packet_header_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.packet_header_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_statement`.\n"]
    pub fn set_resource_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatement>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_statement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAccessScope_MatchPaths_Source {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAccessScope_MatchPaths_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAccessScope_MatchPaths_Source {}

impl BuildEc2NetworkInsightsAccessScope_MatchPaths_Source {
    pub fn build(self) -> Ec2NetworkInsightsAccessScope_MatchPaths_Source {
        Ec2NetworkInsightsAccessScope_MatchPaths_Source {
            packet_header_statement: core::default::Default::default(),
            resource_statement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAccessScope_MatchPaths_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAccessScope_MatchPaths_SourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAccessScope_MatchPaths_SourceRef {
        Ec2NetworkInsightsAccessScope_MatchPaths_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAccessScope_MatchPaths_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `packet_header_statement` after provisioning.\n"]
    pub fn packet_header_statement(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAccessScope_MatchPaths_Source_PacketHeaderStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.packet_header_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_statement` after provisioning.\n"]
    pub fn resource_statement(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_statement", self.base))
    }
}

#[derive(Serialize, Default)]
struct Ec2NetworkInsightsAccessScope_ExcludePaths_ThroughResourcesDynamic {
    resource_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatement>>,
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAccessScope_ExcludePaths_ThroughResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_statement: std
    ::option
    ::Option<std::vec::Vec<Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatement>>,
    dynamic: Ec2NetworkInsightsAccessScope_ExcludePaths_ThroughResourcesDynamic,
}

impl Ec2NetworkInsightsAccessScope_ExcludePaths_ThroughResources {
    #[doc = "Set the field `resource_statement`.\n"]
    pub fn set_resource_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatement>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_statement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAccessScope_ExcludePaths_ThroughResources {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAccessScope_ExcludePaths_ThroughResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAccessScope_ExcludePaths_ThroughResources {}

impl BuildEc2NetworkInsightsAccessScope_ExcludePaths_ThroughResources {
    pub fn build(self) -> Ec2NetworkInsightsAccessScope_ExcludePaths_ThroughResources {
        Ec2NetworkInsightsAccessScope_ExcludePaths_ThroughResources {
            resource_statement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAccessScope_ExcludePaths_ThroughResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAccessScope_ExcludePaths_ThroughResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAccessScope_ExcludePaths_ThroughResourcesRef {
        Ec2NetworkInsightsAccessScope_ExcludePaths_ThroughResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAccessScope_ExcludePaths_ThroughResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_statement` after provisioning.\n"]
    pub fn resource_statement(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAccessScope_MatchPaths_Source_ResourceStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_statement", self.base))
    }
}

#[derive(Serialize, Default)]
struct Ec2NetworkInsightsAccessScope_ExcludePathsDynamic {
    destination: std::option::Option<terrars::DynamicBlock<Ec2NetworkInsightsAccessScope_MatchPaths_Destination>>,
    source: std::option::Option<terrars::DynamicBlock<Ec2NetworkInsightsAccessScope_MatchPaths_Source>>,
    through_resources: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2NetworkInsightsAccessScope_ExcludePaths_ThroughResources>>,
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAccessScope_ExcludePaths {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<std::vec::Vec<Ec2NetworkInsightsAccessScope_MatchPaths_Destination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<Ec2NetworkInsightsAccessScope_MatchPaths_Source>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    through_resources: std::option::Option<std::vec::Vec<Ec2NetworkInsightsAccessScope_ExcludePaths_ThroughResources>>,
    dynamic: Ec2NetworkInsightsAccessScope_ExcludePathsDynamic,
}

impl Ec2NetworkInsightsAccessScope_ExcludePaths {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2NetworkInsightsAccessScope_MatchPaths_Destination>>,
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

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2NetworkInsightsAccessScope_MatchPaths_Source>>,
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

    #[doc = "Set the field `through_resources`.\n"]
    pub fn set_through_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Ec2NetworkInsightsAccessScope_ExcludePaths_ThroughResources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.through_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.through_resources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAccessScope_ExcludePaths {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAccessScope_ExcludePaths>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAccessScope_ExcludePaths {}

impl BuildEc2NetworkInsightsAccessScope_ExcludePaths {
    pub fn build(self) -> Ec2NetworkInsightsAccessScope_ExcludePaths {
        Ec2NetworkInsightsAccessScope_ExcludePaths {
            destination: core::default::Default::default(),
            source: core::default::Default::default(),
            through_resources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAccessScope_ExcludePathsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAccessScope_ExcludePathsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ec2NetworkInsightsAccessScope_ExcludePathsRef {
        Ec2NetworkInsightsAccessScope_ExcludePathsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAccessScope_ExcludePathsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<Ec2NetworkInsightsAccessScope_MatchPaths_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<Ec2NetworkInsightsAccessScope_MatchPaths_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `through_resources` after provisioning.\n"]
    pub fn through_resources(&self) -> terrars::ListRef<Ec2NetworkInsightsAccessScope_ExcludePaths_ThroughResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.through_resources", self.base))
    }
}

#[derive(Serialize, Default)]
struct Ec2NetworkInsightsAccessScope_MatchPathsDynamic {
    destination: std::option::Option<terrars::DynamicBlock<Ec2NetworkInsightsAccessScope_MatchPaths_Destination>>,
    source: std::option::Option<terrars::DynamicBlock<Ec2NetworkInsightsAccessScope_MatchPaths_Source>>,
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAccessScope_MatchPaths {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<std::vec::Vec<Ec2NetworkInsightsAccessScope_MatchPaths_Destination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<Ec2NetworkInsightsAccessScope_MatchPaths_Source>>,
    dynamic: Ec2NetworkInsightsAccessScope_MatchPathsDynamic,
}

impl Ec2NetworkInsightsAccessScope_MatchPaths {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2NetworkInsightsAccessScope_MatchPaths_Destination>>,
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

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2NetworkInsightsAccessScope_MatchPaths_Source>>,
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
}

impl terrars::ToListMappable for Ec2NetworkInsightsAccessScope_MatchPaths {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAccessScope_MatchPaths>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAccessScope_MatchPaths {}

impl BuildEc2NetworkInsightsAccessScope_MatchPaths {
    pub fn build(self) -> Ec2NetworkInsightsAccessScope_MatchPaths {
        Ec2NetworkInsightsAccessScope_MatchPaths {
            destination: core::default::Default::default(),
            source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAccessScope_MatchPathsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAccessScope_MatchPathsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ec2NetworkInsightsAccessScope_MatchPathsRef {
        Ec2NetworkInsightsAccessScope_MatchPathsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAccessScope_MatchPathsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<Ec2NetworkInsightsAccessScope_MatchPaths_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<Ec2NetworkInsightsAccessScope_MatchPaths_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}
