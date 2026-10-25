#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppmeshRouteData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    mesh_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mesh_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    virtual_router_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<std::vec::Vec<AppmeshRoute_Spec>>,
    dynamic: AppmeshRouteDynamic,
}

struct AppmeshRoute_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppmeshRouteData_>,
}

#[derive(Clone)]
pub struct AppmeshRoute(std::rc::Rc<AppmeshRoute_>);

impl AppmeshRoute {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mesh_owner`.\n"]
    pub fn set_mesh_owner(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().mesh_owner = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(self, v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_name` after provisioning.\n"]
    pub fn mesh_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mesh_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mesh_owner", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_router_name` after provisioning.\n"]
    pub fn virtual_router_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_router_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<AppmeshRoute_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }
}

impl terrars::Referable for AppmeshRoute {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppmeshRoute { }

impl terrars::ToListMappable for AppmeshRoute {
    type O = terrars::ListRef<AppmeshRouteRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppmeshRoute_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_appmesh_route".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppmeshRoute {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub mesh_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub virtual_router_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshRoute {
    pub fn build(self, stack: &mut terrars::Stack) -> AppmeshRoute {
        let out = AppmeshRoute(std::rc::Rc::new(AppmeshRoute_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppmeshRouteData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                mesh_name: self.mesh_name,
                mesh_owner: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                virtual_router_name: self.virtual_router_name,
                spec: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppmeshRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppmeshRouteRef {
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

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_name` after provisioning.\n"]
    pub fn mesh_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mesh_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mesh_owner", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_router_name` after provisioning.\n"]
    pub fn virtual_router_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_router_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<AppmeshRoute_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteDynamic {
    spec: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec>>,
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_TcpRoute_Action_WeightedTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    virtual_node: terrars::PrimField<std::string::String>,
    weight: terrars::PrimField<f64>,
}

impl AppmeshRoute_Spec_TcpRoute_Action_WeightedTarget {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshRoute_Spec_TcpRoute_Action_WeightedTarget {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_TcpRoute_Action_WeightedTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_TcpRoute_Action_WeightedTarget {
    #[doc = ""]
    pub virtual_node: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub weight: terrars::PrimField<f64>,
}

impl BuildAppmeshRoute_Spec_TcpRoute_Action_WeightedTarget {
    pub fn build(self) -> AppmeshRoute_Spec_TcpRoute_Action_WeightedTarget {
        AppmeshRoute_Spec_TcpRoute_Action_WeightedTarget {
            port: core::default::Default::default(),
            virtual_node: self.virtual_node,
            weight: self.weight,
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_TcpRoute_Action_WeightedTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_TcpRoute_Action_WeightedTargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRoute_Spec_TcpRoute_Action_WeightedTargetRef {
        AppmeshRoute_Spec_TcpRoute_Action_WeightedTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_TcpRoute_Action_WeightedTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_node` after provisioning.\n"]
    pub fn virtual_node(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_node", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRoute_Spec_TcpRoute_ActionDynamic {
    weighted_target: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_TcpRoute_Action_WeightedTarget>>,
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_TcpRoute_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_target: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_TcpRoute_Action_WeightedTarget>>,
    dynamic: AppmeshRoute_Spec_TcpRoute_ActionDynamic,
}

impl AppmeshRoute_Spec_TcpRoute_Action {
    #[doc = "Set the field `weighted_target`.\n"]
    pub fn set_weighted_target(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_TcpRoute_Action_WeightedTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.weighted_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.weighted_target = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshRoute_Spec_TcpRoute_Action {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_TcpRoute_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_TcpRoute_Action {}

impl BuildAppmeshRoute_Spec_TcpRoute_Action {
    pub fn build(self) -> AppmeshRoute_Spec_TcpRoute_Action {
        AppmeshRoute_Spec_TcpRoute_Action {
            weighted_target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_TcpRoute_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_TcpRoute_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRoute_Spec_TcpRoute_ActionRef {
        AppmeshRoute_Spec_TcpRoute_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_TcpRoute_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_HttpRoute_Match_Header_Match_Range {
    end: terrars::PrimField<f64>,
    start: terrars::PrimField<f64>,
}

impl AppmeshRoute_Spec_HttpRoute_Match_Header_Match_Range { }

impl terrars::ToListMappable for AppmeshRoute_Spec_HttpRoute_Match_Header_Match_Range {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_HttpRoute_Match_Header_Match_Range>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_HttpRoute_Match_Header_Match_Range {
    #[doc = ""]
    pub end: terrars::PrimField<f64>,
    #[doc = ""]
    pub start: terrars::PrimField<f64>,
}

impl BuildAppmeshRoute_Spec_HttpRoute_Match_Header_Match_Range {
    pub fn build(self) -> AppmeshRoute_Spec_HttpRoute_Match_Header_Match_Range {
        AppmeshRoute_Spec_HttpRoute_Match_Header_Match_Range {
            end: self.end,
            start: self.start,
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_HttpRoute_Match_Header_Match_RangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_HttpRoute_Match_Header_Match_RangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRoute_Spec_HttpRoute_Match_Header_Match_RangeRef {
        AppmeshRoute_Spec_HttpRoute_Match_Header_Match_RangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_HttpRoute_Match_Header_Match_RangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRoute_Spec_HttpRoute_Match_Header_MatchDynamic {
    range: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_HttpRoute_Match_Header_Match_Range>>,
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_HttpRoute_Match_Header_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_HttpRoute_Match_Header_Match_Range>>,
    dynamic: AppmeshRoute_Spec_HttpRoute_Match_Header_MatchDynamic,
}

impl AppmeshRoute_Spec_HttpRoute_Match_Header_Match {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `suffix`.\n"]
    pub fn set_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.suffix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `range`.\n"]
    pub fn set_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_HttpRoute_Match_Header_Match_Range>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshRoute_Spec_HttpRoute_Match_Header_Match {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_HttpRoute_Match_Header_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_HttpRoute_Match_Header_Match {}

impl BuildAppmeshRoute_Spec_HttpRoute_Match_Header_Match {
    pub fn build(self) -> AppmeshRoute_Spec_HttpRoute_Match_Header_Match {
        AppmeshRoute_Spec_HttpRoute_Match_Header_Match {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            regex: core::default::Default::default(),
            suffix: core::default::Default::default(),
            range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_HttpRoute_Match_Header_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_HttpRoute_Match_Header_MatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRoute_Spec_HttpRoute_Match_Header_MatchRef {
        AppmeshRoute_Spec_HttpRoute_Match_Header_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_HttpRoute_Match_Header_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc = "Get a reference to the value of field `suffix` after provisioning.\n"]
    pub fn suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }

    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(&self) -> terrars::ListRef<AppmeshRoute_Spec_HttpRoute_Match_Header_Match_RangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRoute_Spec_GrpcRoute_Match_MetadataDynamic {
    match_: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_HttpRoute_Match_Header_Match>>,
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_GrpcRoute_Match_Metadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_HttpRoute_Match_Header_Match>>,
    dynamic: AppmeshRoute_Spec_GrpcRoute_Match_MetadataDynamic,
}

impl AppmeshRoute_Spec_GrpcRoute_Match_Metadata {
    #[doc = "Set the field `invert`.\n"]
    pub fn set_invert(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_HttpRoute_Match_Header_Match>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshRoute_Spec_GrpcRoute_Match_Metadata {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_GrpcRoute_Match_Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_GrpcRoute_Match_Metadata {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshRoute_Spec_GrpcRoute_Match_Metadata {
    pub fn build(self) -> AppmeshRoute_Spec_GrpcRoute_Match_Metadata {
        AppmeshRoute_Spec_GrpcRoute_Match_Metadata {
            invert: core::default::Default::default(),
            name: self.name,
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_GrpcRoute_Match_MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_GrpcRoute_Match_MetadataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRoute_Spec_GrpcRoute_Match_MetadataRef {
        AppmeshRoute_Spec_GrpcRoute_Match_MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_GrpcRoute_Match_MetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `invert` after provisioning.\n"]
    pub fn invert(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<AppmeshRoute_Spec_HttpRoute_Match_Header_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRoute_Spec_GrpcRoute_MatchDynamic {
    metadata: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_GrpcRoute_Match_Metadata>>,
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_GrpcRoute_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_GrpcRoute_Match_Metadata>>,
    dynamic: AppmeshRoute_Spec_GrpcRoute_MatchDynamic,
}

impl AppmeshRoute_Spec_GrpcRoute_Match {
    #[doc = "Set the field `method_name`.\n"]
    pub fn set_method_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_name`.\n"]
    pub fn set_service_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_GrpcRoute_Match_Metadata>>,
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

impl terrars::ToListMappable for AppmeshRoute_Spec_GrpcRoute_Match {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_GrpcRoute_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_GrpcRoute_Match {}

impl BuildAppmeshRoute_Spec_GrpcRoute_Match {
    pub fn build(self) -> AppmeshRoute_Spec_GrpcRoute_Match {
        AppmeshRoute_Spec_GrpcRoute_Match {
            method_name: core::default::Default::default(),
            port: core::default::Default::default(),
            prefix: core::default::Default::default(),
            service_name: core::default::Default::default(),
            metadata: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_GrpcRoute_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_GrpcRoute_MatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRoute_Spec_GrpcRoute_MatchRef {
        AppmeshRoute_Spec_GrpcRoute_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_GrpcRoute_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `method_name` after provisioning.\n"]
    pub fn method_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method_name", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout { }

impl terrars::ToListMappable for AppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout {
    pub fn build(self) -> AppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout {
        AppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout {
            unit: self.unit,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeoutRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeoutRef {
        AppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeoutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRoute_Spec_GrpcRoute_RetryPolicyDynamic {
    per_retry_timeout: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout>>,
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_GrpcRoute_RetryPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc_retry_events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_retry_events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    max_retries: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_retry_events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_retry_timeout: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout>>,
    dynamic: AppmeshRoute_Spec_GrpcRoute_RetryPolicyDynamic,
}

impl AppmeshRoute_Spec_GrpcRoute_RetryPolicy {
    #[doc = "Set the field `grpc_retry_events`.\n"]
    pub fn set_grpc_retry_events(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.grpc_retry_events = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_retry_events`.\n"]
    pub fn set_http_retry_events(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.http_retry_events = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tcp_retry_events`.\n"]
    pub fn set_tcp_retry_events(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tcp_retry_events = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `per_retry_timeout`.\n"]
    pub fn set_per_retry_timeout(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.per_retry_timeout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.per_retry_timeout = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshRoute_Spec_GrpcRoute_RetryPolicy {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_GrpcRoute_RetryPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_GrpcRoute_RetryPolicy {
    #[doc = ""]
    pub max_retries: terrars::PrimField<f64>,
}

impl BuildAppmeshRoute_Spec_GrpcRoute_RetryPolicy {
    pub fn build(self) -> AppmeshRoute_Spec_GrpcRoute_RetryPolicy {
        AppmeshRoute_Spec_GrpcRoute_RetryPolicy {
            grpc_retry_events: core::default::Default::default(),
            http_retry_events: core::default::Default::default(),
            max_retries: self.max_retries,
            tcp_retry_events: core::default::Default::default(),
            per_retry_timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_GrpcRoute_RetryPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_GrpcRoute_RetryPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRoute_Spec_GrpcRoute_RetryPolicyRef {
        AppmeshRoute_Spec_GrpcRoute_RetryPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_GrpcRoute_RetryPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grpc_retry_events` after provisioning.\n"]
    pub fn grpc_retry_events(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.grpc_retry_events", self.base))
    }

    #[doc = "Get a reference to the value of field `http_retry_events` after provisioning.\n"]
    pub fn http_retry_events(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.http_retry_events", self.base))
    }

    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_retry_events` after provisioning.\n"]
    pub fn tcp_retry_events(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tcp_retry_events", self.base))
    }

    #[doc = "Get a reference to the value of field `per_retry_timeout` after provisioning.\n"]
    pub fn per_retry_timeout(&self) -> terrars::ListRef<AppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_retry_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_TcpRoute_Timeout_Idle {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshRoute_Spec_TcpRoute_Timeout_Idle { }

impl terrars::ToListMappable for AppmeshRoute_Spec_TcpRoute_Timeout_Idle {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_TcpRoute_Timeout_Idle>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_TcpRoute_Timeout_Idle {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshRoute_Spec_TcpRoute_Timeout_Idle {
    pub fn build(self) -> AppmeshRoute_Spec_TcpRoute_Timeout_Idle {
        AppmeshRoute_Spec_TcpRoute_Timeout_Idle {
            unit: self.unit,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_TcpRoute_Timeout_IdleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_TcpRoute_Timeout_IdleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRoute_Spec_TcpRoute_Timeout_IdleRef {
        AppmeshRoute_Spec_TcpRoute_Timeout_IdleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_TcpRoute_Timeout_IdleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_GrpcRoute_Timeout_PerRequest {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshRoute_Spec_GrpcRoute_Timeout_PerRequest { }

impl terrars::ToListMappable for AppmeshRoute_Spec_GrpcRoute_Timeout_PerRequest {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_GrpcRoute_Timeout_PerRequest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_GrpcRoute_Timeout_PerRequest {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshRoute_Spec_GrpcRoute_Timeout_PerRequest {
    pub fn build(self) -> AppmeshRoute_Spec_GrpcRoute_Timeout_PerRequest {
        AppmeshRoute_Spec_GrpcRoute_Timeout_PerRequest {
            unit: self.unit,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_GrpcRoute_Timeout_PerRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_GrpcRoute_Timeout_PerRequestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRoute_Spec_GrpcRoute_Timeout_PerRequestRef {
        AppmeshRoute_Spec_GrpcRoute_Timeout_PerRequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_GrpcRoute_Timeout_PerRequestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRoute_Spec_GrpcRoute_TimeoutDynamic {
    idle: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_TcpRoute_Timeout_Idle>>,
    per_request: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_GrpcRoute_Timeout_PerRequest>>,
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_GrpcRoute_Timeout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_TcpRoute_Timeout_Idle>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_request: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_GrpcRoute_Timeout_PerRequest>>,
    dynamic: AppmeshRoute_Spec_GrpcRoute_TimeoutDynamic,
}

impl AppmeshRoute_Spec_GrpcRoute_Timeout {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_TcpRoute_Timeout_Idle>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.idle = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.idle = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `per_request`.\n"]
    pub fn set_per_request(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_GrpcRoute_Timeout_PerRequest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.per_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.per_request = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshRoute_Spec_GrpcRoute_Timeout {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_GrpcRoute_Timeout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_GrpcRoute_Timeout {}

impl BuildAppmeshRoute_Spec_GrpcRoute_Timeout {
    pub fn build(self) -> AppmeshRoute_Spec_GrpcRoute_Timeout {
        AppmeshRoute_Spec_GrpcRoute_Timeout {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_GrpcRoute_TimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_GrpcRoute_TimeoutRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRoute_Spec_GrpcRoute_TimeoutRef {
        AppmeshRoute_Spec_GrpcRoute_TimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_GrpcRoute_TimeoutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> terrars::ListRef<AppmeshRoute_Spec_TcpRoute_Timeout_IdleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> terrars::ListRef<AppmeshRoute_Spec_GrpcRoute_Timeout_PerRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRoute_Spec_GrpcRouteDynamic {
    action: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_TcpRoute_Action>>,
    match_: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_GrpcRoute_Match>>,
    retry_policy: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_GrpcRoute_RetryPolicy>>,
    timeout: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_GrpcRoute_Timeout>>,
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_GrpcRoute {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_TcpRoute_Action>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_GrpcRoute_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_GrpcRoute_RetryPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_GrpcRoute_Timeout>>,
    dynamic: AppmeshRoute_Spec_GrpcRouteDynamic,
}

impl AppmeshRoute_Spec_GrpcRoute {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_TcpRoute_Action>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_GrpcRoute_Match>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_GrpcRoute_RetryPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retry_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_GrpcRoute_Timeout>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timeout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshRoute_Spec_GrpcRoute {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_GrpcRoute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_GrpcRoute {}

impl BuildAppmeshRoute_Spec_GrpcRoute {
    pub fn build(self) -> AppmeshRoute_Spec_GrpcRoute {
        AppmeshRoute_Spec_GrpcRoute {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_GrpcRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_GrpcRouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRoute_Spec_GrpcRouteRef {
        AppmeshRoute_Spec_GrpcRouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_GrpcRouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<AppmeshRoute_Spec_TcpRoute_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<AppmeshRoute_Spec_GrpcRoute_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<AppmeshRoute_Spec_GrpcRoute_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<AppmeshRoute_Spec_GrpcRoute_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRoute_Spec_HttpRoute_Match_HeaderDynamic {
    match_: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_HttpRoute_Match_Header_Match>>,
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_HttpRoute_Match_Header {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_HttpRoute_Match_Header_Match>>,
    dynamic: AppmeshRoute_Spec_HttpRoute_Match_HeaderDynamic,
}

impl AppmeshRoute_Spec_HttpRoute_Match_Header {
    #[doc = "Set the field `invert`.\n"]
    pub fn set_invert(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_HttpRoute_Match_Header_Match>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshRoute_Spec_HttpRoute_Match_Header {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_HttpRoute_Match_Header>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_HttpRoute_Match_Header {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshRoute_Spec_HttpRoute_Match_Header {
    pub fn build(self) -> AppmeshRoute_Spec_HttpRoute_Match_Header {
        AppmeshRoute_Spec_HttpRoute_Match_Header {
            invert: core::default::Default::default(),
            name: self.name,
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_HttpRoute_Match_HeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_HttpRoute_Match_HeaderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRoute_Spec_HttpRoute_Match_HeaderRef {
        AppmeshRoute_Spec_HttpRoute_Match_HeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_HttpRoute_Match_HeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `invert` after provisioning.\n"]
    pub fn invert(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<AppmeshRoute_Spec_HttpRoute_Match_Header_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_HttpRoute_Match_Path {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppmeshRoute_Spec_HttpRoute_Match_Path {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshRoute_Spec_HttpRoute_Match_Path {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_HttpRoute_Match_Path>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_HttpRoute_Match_Path {}

impl BuildAppmeshRoute_Spec_HttpRoute_Match_Path {
    pub fn build(self) -> AppmeshRoute_Spec_HttpRoute_Match_Path {
        AppmeshRoute_Spec_HttpRoute_Match_Path {
            exact: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_HttpRoute_Match_PathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_HttpRoute_Match_PathRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRoute_Spec_HttpRoute_Match_PathRef {
        AppmeshRoute_Spec_HttpRoute_Match_PathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_HttpRoute_Match_PathRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_HttpRoute_Match_QueryParameter_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppmeshRoute_Spec_HttpRoute_Match_QueryParameter_Match {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshRoute_Spec_HttpRoute_Match_QueryParameter_Match {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_HttpRoute_Match_QueryParameter_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_HttpRoute_Match_QueryParameter_Match {}

impl BuildAppmeshRoute_Spec_HttpRoute_Match_QueryParameter_Match {
    pub fn build(self) -> AppmeshRoute_Spec_HttpRoute_Match_QueryParameter_Match {
        AppmeshRoute_Spec_HttpRoute_Match_QueryParameter_Match { exact: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef {
        AppmeshRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRoute_Spec_HttpRoute_Match_QueryParameterDynamic {
    match_: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_HttpRoute_Match_QueryParameter_Match>>,
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_HttpRoute_Match_QueryParameter {
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_HttpRoute_Match_QueryParameter_Match>>,
    dynamic: AppmeshRoute_Spec_HttpRoute_Match_QueryParameterDynamic,
}

impl AppmeshRoute_Spec_HttpRoute_Match_QueryParameter {
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_HttpRoute_Match_QueryParameter_Match>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshRoute_Spec_HttpRoute_Match_QueryParameter {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_HttpRoute_Match_QueryParameter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_HttpRoute_Match_QueryParameter {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshRoute_Spec_HttpRoute_Match_QueryParameter {
    pub fn build(self) -> AppmeshRoute_Spec_HttpRoute_Match_QueryParameter {
        AppmeshRoute_Spec_HttpRoute_Match_QueryParameter {
            name: self.name,
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_HttpRoute_Match_QueryParameterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_HttpRoute_Match_QueryParameterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRoute_Spec_HttpRoute_Match_QueryParameterRef {
        AppmeshRoute_Spec_HttpRoute_Match_QueryParameterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_HttpRoute_Match_QueryParameterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<AppmeshRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRoute_Spec_HttpRoute_MatchDynamic {
    header: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_HttpRoute_Match_Header>>,
    path: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_HttpRoute_Match_Path>>,
    query_parameter: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_HttpRoute_Match_QueryParameter>>,
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_HttpRoute_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheme: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_HttpRoute_Match_Header>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_HttpRoute_Match_Path>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_parameter: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_HttpRoute_Match_QueryParameter>>,
    dynamic: AppmeshRoute_Spec_HttpRoute_MatchDynamic,
}

impl AppmeshRoute_Spec_HttpRoute_Match {
    #[doc = "Set the field `method`.\n"]
    pub fn set_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scheme`.\n"]
    pub fn set_scheme(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scheme = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_HttpRoute_Match_Header>>,
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

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_HttpRoute_Match_Path>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.path = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.path = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_parameter`.\n"]
    pub fn set_query_parameter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_HttpRoute_Match_QueryParameter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_parameter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_parameter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshRoute_Spec_HttpRoute_Match {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_HttpRoute_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_HttpRoute_Match {}

impl BuildAppmeshRoute_Spec_HttpRoute_Match {
    pub fn build(self) -> AppmeshRoute_Spec_HttpRoute_Match {
        AppmeshRoute_Spec_HttpRoute_Match {
            method: core::default::Default::default(),
            port: core::default::Default::default(),
            prefix: core::default::Default::default(),
            scheme: core::default::Default::default(),
            header: core::default::Default::default(),
            path: core::default::Default::default(),
            query_parameter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_HttpRoute_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_HttpRoute_MatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRoute_Spec_HttpRoute_MatchRef {
        AppmeshRoute_Spec_HttpRoute_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_HttpRoute_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `scheme` after provisioning.\n"]
    pub fn scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheme", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::ListRef<AppmeshRoute_Spec_HttpRoute_Match_PathRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRoute_Spec_HttpRoute_RetryPolicyDynamic {
    per_retry_timeout: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout>>,
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_HttpRoute_RetryPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_retry_events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    max_retries: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_retry_events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_retry_timeout: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout>>,
    dynamic: AppmeshRoute_Spec_HttpRoute_RetryPolicyDynamic,
}

impl AppmeshRoute_Spec_HttpRoute_RetryPolicy {
    #[doc = "Set the field `http_retry_events`.\n"]
    pub fn set_http_retry_events(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.http_retry_events = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tcp_retry_events`.\n"]
    pub fn set_tcp_retry_events(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tcp_retry_events = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `per_retry_timeout`.\n"]
    pub fn set_per_retry_timeout(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.per_retry_timeout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.per_retry_timeout = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshRoute_Spec_HttpRoute_RetryPolicy {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_HttpRoute_RetryPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_HttpRoute_RetryPolicy {
    #[doc = ""]
    pub max_retries: terrars::PrimField<f64>,
}

impl BuildAppmeshRoute_Spec_HttpRoute_RetryPolicy {
    pub fn build(self) -> AppmeshRoute_Spec_HttpRoute_RetryPolicy {
        AppmeshRoute_Spec_HttpRoute_RetryPolicy {
            http_retry_events: core::default::Default::default(),
            max_retries: self.max_retries,
            tcp_retry_events: core::default::Default::default(),
            per_retry_timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_HttpRoute_RetryPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_HttpRoute_RetryPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRoute_Spec_HttpRoute_RetryPolicyRef {
        AppmeshRoute_Spec_HttpRoute_RetryPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_HttpRoute_RetryPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_retry_events` after provisioning.\n"]
    pub fn http_retry_events(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.http_retry_events", self.base))
    }

    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_retry_events` after provisioning.\n"]
    pub fn tcp_retry_events(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tcp_retry_events", self.base))
    }

    #[doc = "Get a reference to the value of field `per_retry_timeout` after provisioning.\n"]
    pub fn per_retry_timeout(&self) -> terrars::ListRef<AppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_retry_timeout", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRoute_Spec_Http2RouteDynamic {
    action: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_TcpRoute_Action>>,
    match_: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_HttpRoute_Match>>,
    retry_policy: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_HttpRoute_RetryPolicy>>,
    timeout: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_GrpcRoute_Timeout>>,
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_Http2Route {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_TcpRoute_Action>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_HttpRoute_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_HttpRoute_RetryPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_GrpcRoute_Timeout>>,
    dynamic: AppmeshRoute_Spec_Http2RouteDynamic,
}

impl AppmeshRoute_Spec_Http2Route {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_TcpRoute_Action>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_HttpRoute_Match>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_HttpRoute_RetryPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retry_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_GrpcRoute_Timeout>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timeout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshRoute_Spec_Http2Route {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_Http2Route>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_Http2Route {}

impl BuildAppmeshRoute_Spec_Http2Route {
    pub fn build(self) -> AppmeshRoute_Spec_Http2Route {
        AppmeshRoute_Spec_Http2Route {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_Http2RouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_Http2RouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRoute_Spec_Http2RouteRef {
        AppmeshRoute_Spec_Http2RouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_Http2RouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<AppmeshRoute_Spec_TcpRoute_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<AppmeshRoute_Spec_HttpRoute_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<AppmeshRoute_Spec_HttpRoute_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<AppmeshRoute_Spec_GrpcRoute_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRoute_Spec_HttpRouteDynamic {
    action: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_TcpRoute_Action>>,
    match_: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_HttpRoute_Match>>,
    retry_policy: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_HttpRoute_RetryPolicy>>,
    timeout: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_GrpcRoute_Timeout>>,
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_HttpRoute {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_TcpRoute_Action>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_HttpRoute_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_HttpRoute_RetryPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_GrpcRoute_Timeout>>,
    dynamic: AppmeshRoute_Spec_HttpRouteDynamic,
}

impl AppmeshRoute_Spec_HttpRoute {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_TcpRoute_Action>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_HttpRoute_Match>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_HttpRoute_RetryPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retry_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_GrpcRoute_Timeout>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timeout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshRoute_Spec_HttpRoute {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_HttpRoute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_HttpRoute {}

impl BuildAppmeshRoute_Spec_HttpRoute {
    pub fn build(self) -> AppmeshRoute_Spec_HttpRoute {
        AppmeshRoute_Spec_HttpRoute {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_HttpRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_HttpRouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRoute_Spec_HttpRouteRef {
        AppmeshRoute_Spec_HttpRouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_HttpRouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<AppmeshRoute_Spec_TcpRoute_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<AppmeshRoute_Spec_HttpRoute_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<AppmeshRoute_Spec_HttpRoute_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<AppmeshRoute_Spec_GrpcRoute_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_TcpRoute_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl AppmeshRoute_Spec_TcpRoute_Match {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshRoute_Spec_TcpRoute_Match {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_TcpRoute_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_TcpRoute_Match {}

impl BuildAppmeshRoute_Spec_TcpRoute_Match {
    pub fn build(self) -> AppmeshRoute_Spec_TcpRoute_Match {
        AppmeshRoute_Spec_TcpRoute_Match { port: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_TcpRoute_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_TcpRoute_MatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRoute_Spec_TcpRoute_MatchRef {
        AppmeshRoute_Spec_TcpRoute_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_TcpRoute_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRoute_Spec_TcpRoute_TimeoutDynamic {
    idle: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_TcpRoute_Timeout_Idle>>,
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_TcpRoute_Timeout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_TcpRoute_Timeout_Idle>>,
    dynamic: AppmeshRoute_Spec_TcpRoute_TimeoutDynamic,
}

impl AppmeshRoute_Spec_TcpRoute_Timeout {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_TcpRoute_Timeout_Idle>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.idle = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.idle = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshRoute_Spec_TcpRoute_Timeout {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_TcpRoute_Timeout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_TcpRoute_Timeout {}

impl BuildAppmeshRoute_Spec_TcpRoute_Timeout {
    pub fn build(self) -> AppmeshRoute_Spec_TcpRoute_Timeout {
        AppmeshRoute_Spec_TcpRoute_Timeout {
            idle: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_TcpRoute_TimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_TcpRoute_TimeoutRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRoute_Spec_TcpRoute_TimeoutRef {
        AppmeshRoute_Spec_TcpRoute_TimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_TcpRoute_TimeoutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> terrars::ListRef<AppmeshRoute_Spec_TcpRoute_Timeout_IdleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRoute_Spec_TcpRouteDynamic {
    action: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_TcpRoute_Action>>,
    match_: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_TcpRoute_Match>>,
    timeout: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_TcpRoute_Timeout>>,
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec_TcpRoute {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_TcpRoute_Action>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_TcpRoute_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_TcpRoute_Timeout>>,
    dynamic: AppmeshRoute_Spec_TcpRouteDynamic,
}

impl AppmeshRoute_Spec_TcpRoute {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_TcpRoute_Action>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_TcpRoute_Match>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_TcpRoute_Timeout>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timeout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshRoute_Spec_TcpRoute {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec_TcpRoute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec_TcpRoute {}

impl BuildAppmeshRoute_Spec_TcpRoute {
    pub fn build(self) -> AppmeshRoute_Spec_TcpRoute {
        AppmeshRoute_Spec_TcpRoute {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_Spec_TcpRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_Spec_TcpRouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRoute_Spec_TcpRouteRef {
        AppmeshRoute_Spec_TcpRouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_Spec_TcpRouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<AppmeshRoute_Spec_TcpRoute_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<AppmeshRoute_Spec_TcpRoute_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<AppmeshRoute_Spec_TcpRoute_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRoute_SpecDynamic {
    grpc_route: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_GrpcRoute>>,
    http2_route: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_Http2Route>>,
    http_route: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_HttpRoute>>,
    tcp_route: std::option::Option<terrars::DynamicBlock<AppmeshRoute_Spec_TcpRoute>>,
}

#[derive(Serialize)]
pub struct AppmeshRoute_Spec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc_route: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_GrpcRoute>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http2_route: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_Http2Route>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_route: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_HttpRoute>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_route: std::option::Option<std::vec::Vec<AppmeshRoute_Spec_TcpRoute>>,
    dynamic: AppmeshRoute_SpecDynamic,
}

impl AppmeshRoute_Spec {
    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grpc_route`.\n"]
    pub fn set_grpc_route(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_GrpcRoute>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.grpc_route = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc_route = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http2_route`.\n"]
    pub fn set_http2_route(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_Http2Route>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http2_route = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http2_route = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_route`.\n"]
    pub fn set_http_route(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_HttpRoute>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_route = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_route = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tcp_route`.\n"]
    pub fn set_tcp_route(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRoute_Spec_TcpRoute>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tcp_route = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tcp_route = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshRoute_Spec {
    type O = terrars::BlockAssignable<AppmeshRoute_Spec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRoute_Spec {}

impl BuildAppmeshRoute_Spec {
    pub fn build(self) -> AppmeshRoute_Spec {
        AppmeshRoute_Spec {
            priority: core::default::Default::default(),
            grpc_route: core::default::Default::default(),
            http2_route: core::default::Default::default(),
            http_route: core::default::Default::default(),
            tcp_route: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshRoute_SpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRoute_SpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRoute_SpecRef {
        AppmeshRoute_SpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRoute_SpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `grpc_route` after provisioning.\n"]
    pub fn grpc_route(&self) -> terrars::ListRef<AppmeshRoute_Spec_GrpcRouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc_route", self.base))
    }

    #[doc = "Get a reference to the value of field `http2_route` after provisioning.\n"]
    pub fn http2_route(&self) -> terrars::ListRef<AppmeshRoute_Spec_Http2RouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http2_route", self.base))
    }

    #[doc = "Get a reference to the value of field `http_route` after provisioning.\n"]
    pub fn http_route(&self) -> terrars::ListRef<AppmeshRoute_Spec_HttpRouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_route", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_route` after provisioning.\n"]
    pub fn tcp_route(&self) -> terrars::ListRef<AppmeshRoute_Spec_TcpRouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tcp_route", self.base))
    }
}
