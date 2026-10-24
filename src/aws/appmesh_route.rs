use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppmeshRouteData {
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
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    virtual_router_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<std::vec::Vec<AppmeshRouteSpecEl>>,
    dynamic: AppmeshRouteDynamic,
}

struct AppmeshRoute_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppmeshRouteData>,
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
    pub fn set_spec(self, v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecEl>>) -> Self {
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
    pub fn spec(&self) -> terrars::ListRef<AppmeshRouteSpecElRef> {
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
            data: std::cell::RefCell::new(AppmeshRouteData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                mesh_name: self.mesh_name,
                mesh_owner: core::default::Default::default(),
                name: self.name,
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
    pub fn spec(&self) -> terrars::ListRef<AppmeshRouteSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    virtual_node: terrars::PrimField<std::string::String>,
    weight: terrars::PrimField<f64>,
}

impl AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl {
    #[doc = ""]
    pub virtual_node: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub weight: terrars::PrimField<f64>,
}

impl BuildAppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl {
        AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl {
            port: core::default::Default::default(),
            virtual_node: self.virtual_node,
            weight: self.weight,
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetElRef {
        AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetElRef {
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
struct AppmeshRouteSpecElGrpcRouteElActionElDynamic {
    weighted_target: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteElActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_target: std::option::Option<std::vec::Vec<AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl>>,
    dynamic: AppmeshRouteSpecElGrpcRouteElActionElDynamic,
}

impl AppmeshRouteSpecElGrpcRouteElActionEl {
    #[doc = "Set the field `weighted_target`.\n"]
    pub fn set_weighted_target(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElGrpcRouteElActionEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElActionEl {}

impl BuildAppmeshRouteSpecElGrpcRouteElActionEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElActionEl {
        AppmeshRouteSpecElGrpcRouteElActionEl {
            weighted_target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElGrpcRouteElActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRouteSpecElGrpcRouteElActionElRef {
        AppmeshRouteSpecElGrpcRouteElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl {
    end: terrars::PrimField<f64>,
    start: terrars::PrimField<f64>,
}

impl AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl { }

impl terrars::ToListMappable for AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl {
    #[doc = ""]
    pub end: terrars::PrimField<f64>,
    #[doc = ""]
    pub start: terrars::PrimField<f64>,
}

impl BuildAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl {
        AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl {
            end: self.end,
            start: self.start,
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeElRef {
        AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeElRef {
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
struct AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElDynamic {
    range: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range: std::option::Option<std::vec::Vec<AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl>>,
    dynamic: AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElDynamic,
}

impl AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl {
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl {}

impl BuildAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl {
        AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            regex: core::default::Default::default(),
            suffix: core::default::Default::default(),
            range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRef {
        AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRef {
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
    pub fn range(&self) -> terrars::ListRef<AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElGrpcRouteElMatchElMetadataElDynamic {
    match_: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteElMatchElMetadataEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl>>,
    dynamic: AppmeshRouteSpecElGrpcRouteElMatchElMetadataElDynamic,
}

impl AppmeshRouteSpecElGrpcRouteElMatchElMetadataEl {
    #[doc = "Set the field `invert`.\n"]
    pub fn set_invert(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElGrpcRouteElMatchElMetadataEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElMatchElMetadataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElMatchElMetadataEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshRouteSpecElGrpcRouteElMatchElMetadataEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElMatchElMetadataEl {
        AppmeshRouteSpecElGrpcRouteElMatchElMetadataEl {
            invert: core::default::Default::default(),
            name: self.name,
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElMatchElMetadataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElGrpcRouteElMatchElMetadataElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElGrpcRouteElMatchElMetadataElRef {
        AppmeshRouteSpecElGrpcRouteElMatchElMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElMatchElMetadataElRef {
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
    pub fn match_(&self) -> terrars::ListRef<AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElGrpcRouteElMatchElDynamic {
    metadata: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElGrpcRouteElMatchElMetadataEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteElMatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<std::vec::Vec<AppmeshRouteSpecElGrpcRouteElMatchElMetadataEl>>,
    dynamic: AppmeshRouteSpecElGrpcRouteElMatchElDynamic,
}

impl AppmeshRouteSpecElGrpcRouteElMatchEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElMatchElMetadataEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElGrpcRouteElMatchEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElMatchEl {}

impl BuildAppmeshRouteSpecElGrpcRouteElMatchEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElMatchEl {
        AppmeshRouteSpecElGrpcRouteElMatchEl {
            method_name: core::default::Default::default(),
            port: core::default::Default::default(),
            prefix: core::default::Default::default(),
            service_name: core::default::Default::default(),
            metadata: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElGrpcRouteElMatchElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRouteSpecElGrpcRouteElMatchElRef {
        AppmeshRouteSpecElGrpcRouteElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElMatchElRef {
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
pub struct AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl { }

impl terrars::ToListMappable for AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl {
        AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutElRef {
        AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutElRef {
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
struct AppmeshRouteSpecElGrpcRouteElRetryPolicyElDynamic {
    per_retry_timeout: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteElRetryPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc_retry_events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_retry_events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    max_retries: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_retry_events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_retry_timeout: std::option::Option<std::vec::Vec<AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl>>,
    dynamic: AppmeshRouteSpecElGrpcRouteElRetryPolicyElDynamic,
}

impl AppmeshRouteSpecElGrpcRouteElRetryPolicyEl {
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
                    ::Into<terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElGrpcRouteElRetryPolicyEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElRetryPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElRetryPolicyEl {
    #[doc = ""]
    pub max_retries: terrars::PrimField<f64>,
}

impl BuildAppmeshRouteSpecElGrpcRouteElRetryPolicyEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElRetryPolicyEl {
        AppmeshRouteSpecElGrpcRouteElRetryPolicyEl {
            grpc_retry_events: core::default::Default::default(),
            http_retry_events: core::default::Default::default(),
            max_retries: self.max_retries,
            tcp_retry_events: core::default::Default::default(),
            per_retry_timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElRetryPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElGrpcRouteElRetryPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRouteSpecElGrpcRouteElRetryPolicyElRef {
        AppmeshRouteSpecElGrpcRouteElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElRetryPolicyElRef {
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
    pub fn per_retry_timeout(&self) -> terrars::ListRef<AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_retry_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl { }

impl terrars::ToListMappable for AppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl {
        AppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElTimeoutElIdleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElGrpcRouteElTimeoutElIdleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElGrpcRouteElTimeoutElIdleElRef {
        AppmeshRouteSpecElGrpcRouteElTimeoutElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElTimeoutElIdleElRef {
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
pub struct AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl { }

impl terrars::ToListMappable for AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl {
        AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestElRef {
        AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestElRef {
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
struct AppmeshRouteSpecElGrpcRouteElTimeoutElDynamic {
    idle: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl>>,
    per_request: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteElTimeoutEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle: std::option::Option<std::vec::Vec<AppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_request: std::option::Option<std::vec::Vec<AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl>>,
    dynamic: AppmeshRouteSpecElGrpcRouteElTimeoutElDynamic,
}

impl AppmeshRouteSpecElGrpcRouteElTimeoutEl {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElGrpcRouteElTimeoutEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElTimeoutEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElTimeoutEl {}

impl BuildAppmeshRouteSpecElGrpcRouteElTimeoutEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElTimeoutEl {
        AppmeshRouteSpecElGrpcRouteElTimeoutEl {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElTimeoutElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElGrpcRouteElTimeoutElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRouteSpecElGrpcRouteElTimeoutElRef {
        AppmeshRouteSpecElGrpcRouteElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElTimeoutElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> terrars::ListRef<AppmeshRouteSpecElGrpcRouteElTimeoutElIdleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> terrars::ListRef<AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElGrpcRouteElDynamic {
    action: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElGrpcRouteElActionEl>>,
    match_: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElGrpcRouteElMatchEl>>,
    retry_policy: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElGrpcRouteElRetryPolicyEl>>,
    timeout: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElGrpcRouteElTimeoutEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<AppmeshRouteSpecElGrpcRouteElActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshRouteSpecElGrpcRouteElMatchEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<AppmeshRouteSpecElGrpcRouteElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<AppmeshRouteSpecElGrpcRouteElTimeoutEl>>,
    dynamic: AppmeshRouteSpecElGrpcRouteElDynamic,
}

impl AppmeshRouteSpecElGrpcRouteEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElActionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElMatchEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElRetryPolicyEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteElTimeoutEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElGrpcRouteEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteEl {}

impl BuildAppmeshRouteSpecElGrpcRouteEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteEl {
        AppmeshRouteSpecElGrpcRouteEl {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElGrpcRouteElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRouteSpecElGrpcRouteElRef {
        AppmeshRouteSpecElGrpcRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<AppmeshRouteSpecElGrpcRouteElActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<AppmeshRouteSpecElGrpcRouteElMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<AppmeshRouteSpecElGrpcRouteElRetryPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<AppmeshRouteSpecElGrpcRouteElTimeoutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    virtual_node: terrars::PrimField<std::string::String>,
    weight: terrars::PrimField<f64>,
}

impl AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl {
    #[doc = ""]
    pub virtual_node: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub weight: terrars::PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl {
        AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl {
            port: core::default::Default::default(),
            virtual_node: self.virtual_node,
            weight: self.weight,
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetElRef {
        AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetElRef {
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
struct AppmeshRouteSpecElHttp2RouteElActionElDynamic {
    weighted_target: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_target: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl>>,
    dynamic: AppmeshRouteSpecElHttp2RouteElActionElDynamic,
}

impl AppmeshRouteSpecElHttp2RouteElActionEl {
    #[doc = "Set the field `weighted_target`.\n"]
    pub fn set_weighted_target(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElHttp2RouteElActionEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElActionEl {}

impl BuildAppmeshRouteSpecElHttp2RouteElActionEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElActionEl {
        AppmeshRouteSpecElHttp2RouteElActionEl {
            weighted_target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttp2RouteElActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRouteSpecElHttp2RouteElActionElRef {
        AppmeshRouteSpecElHttp2RouteElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
    end: terrars::PrimField<f64>,
    start: terrars::PrimField<f64>,
}

impl AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl { }

impl terrars::ToListMappable for AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
    #[doc = ""]
    pub end: terrars::PrimField<f64>,
    #[doc = ""]
    pub start: terrars::PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
        AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
            end: self.end,
            start: self.start,
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef {
        AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef {
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
struct AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElDynamic {
    range: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl>>,
    dynamic: AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElDynamic,
}

impl AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {}

impl BuildAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
        AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            regex: core::default::Default::default(),
            suffix: core::default::Default::default(),
            range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef {
        AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef {
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
    pub fn range(&self) -> terrars::ListRef<AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttp2RouteElMatchElHeaderElDynamic {
    match_: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElMatchElHeaderEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl>>,
    dynamic: AppmeshRouteSpecElHttp2RouteElMatchElHeaderElDynamic,
}

impl AppmeshRouteSpecElHttp2RouteElMatchElHeaderEl {
    #[doc = "Set the field `invert`.\n"]
    pub fn set_invert(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElHttp2RouteElMatchElHeaderEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchElHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElMatchElHeaderEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshRouteSpecElHttp2RouteElMatchElHeaderEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElMatchElHeaderEl {
        AppmeshRouteSpecElHttp2RouteElMatchElHeaderEl {
            invert: core::default::Default::default(),
            name: self.name,
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElMatchElHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttp2RouteElMatchElHeaderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElHttp2RouteElMatchElHeaderElRef {
        AppmeshRouteSpecElHttp2RouteElMatchElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElMatchElHeaderElRef {
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
    pub fn match_(&self) -> terrars::ListRef<AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttp2RouteElMatchElDynamic {
    header: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttp2RouteElMatchElHeaderEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElMatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    prefix: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheme: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttp2RouteElMatchElHeaderEl>>,
    dynamic: AppmeshRouteSpecElHttp2RouteElMatchElDynamic,
}

impl AppmeshRouteSpecElHttp2RouteElMatchEl {
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

    #[doc = "Set the field `scheme`.\n"]
    pub fn set_scheme(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scheme = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchElHeaderEl>>,
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
}

impl terrars::ToListMappable for AppmeshRouteSpecElHttp2RouteElMatchEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElMatchEl {
    #[doc = ""]
    pub prefix: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshRouteSpecElHttp2RouteElMatchEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElMatchEl {
        AppmeshRouteSpecElHttp2RouteElMatchEl {
            method: core::default::Default::default(),
            port: core::default::Default::default(),
            prefix: self.prefix,
            scheme: core::default::Default::default(),
            header: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttp2RouteElMatchElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRouteSpecElHttp2RouteElMatchElRef {
        AppmeshRouteSpecElHttp2RouteElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElMatchElRef {
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
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl { }

impl terrars::ToListMappable for AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl {
        AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutElRef {
        AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutElRef {
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
struct AppmeshRouteSpecElHttp2RouteElRetryPolicyElDynamic {
    per_retry_timeout: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElRetryPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_retry_events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    max_retries: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_retry_events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_retry_timeout: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl>>,
    dynamic: AppmeshRouteSpecElHttp2RouteElRetryPolicyElDynamic,
}

impl AppmeshRouteSpecElHttp2RouteElRetryPolicyEl {
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
                    ::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElHttp2RouteElRetryPolicyEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElRetryPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElRetryPolicyEl {
    #[doc = ""]
    pub max_retries: terrars::PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttp2RouteElRetryPolicyEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElRetryPolicyEl {
        AppmeshRouteSpecElHttp2RouteElRetryPolicyEl {
            http_retry_events: core::default::Default::default(),
            max_retries: self.max_retries,
            tcp_retry_events: core::default::Default::default(),
            per_retry_timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElRetryPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttp2RouteElRetryPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElHttp2RouteElRetryPolicyElRef {
        AppmeshRouteSpecElHttp2RouteElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElRetryPolicyElRef {
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
    pub fn per_retry_timeout(&self) -> terrars::ListRef<AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_retry_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl { }

impl terrars::ToListMappable for AppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl {
        AppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElTimeoutElIdleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttp2RouteElTimeoutElIdleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElHttp2RouteElTimeoutElIdleElRef {
        AppmeshRouteSpecElHttp2RouteElTimeoutElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElTimeoutElIdleElRef {
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
pub struct AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl { }

impl terrars::ToListMappable for AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl {
        AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestElRef {
        AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestElRef {
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
struct AppmeshRouteSpecElHttp2RouteElTimeoutElDynamic {
    idle: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl>>,
    per_request: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElTimeoutEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_request: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl>>,
    dynamic: AppmeshRouteSpecElHttp2RouteElTimeoutElDynamic,
}

impl AppmeshRouteSpecElHttp2RouteElTimeoutEl {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElHttp2RouteElTimeoutEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElTimeoutEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElTimeoutEl {}

impl BuildAppmeshRouteSpecElHttp2RouteElTimeoutEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElTimeoutEl {
        AppmeshRouteSpecElHttp2RouteElTimeoutEl {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElTimeoutElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttp2RouteElTimeoutElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRouteSpecElHttp2RouteElTimeoutElRef {
        AppmeshRouteSpecElHttp2RouteElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElTimeoutElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> terrars::ListRef<AppmeshRouteSpecElHttp2RouteElTimeoutElIdleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> terrars::ListRef<AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttp2RouteElDynamic {
    action: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttp2RouteElActionEl>>,
    match_: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttp2RouteElMatchEl>>,
    retry_policy: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttp2RouteElRetryPolicyEl>>,
    timeout: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttp2RouteElTimeoutEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttp2RouteElActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttp2RouteElMatchEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttp2RouteElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttp2RouteElTimeoutEl>>,
    dynamic: AppmeshRouteSpecElHttp2RouteElDynamic,
}

impl AppmeshRouteSpecElHttp2RouteEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElActionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElRetryPolicyEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteElTimeoutEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElHttp2RouteEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteEl {}

impl BuildAppmeshRouteSpecElHttp2RouteEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteEl {
        AppmeshRouteSpecElHttp2RouteEl {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttp2RouteElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRouteSpecElHttp2RouteElRef {
        AppmeshRouteSpecElHttp2RouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<AppmeshRouteSpecElHttp2RouteElActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<AppmeshRouteSpecElHttp2RouteElMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<AppmeshRouteSpecElHttp2RouteElRetryPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<AppmeshRouteSpecElHttp2RouteElTimeoutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    virtual_node: terrars::PrimField<std::string::String>,
    weight: terrars::PrimField<f64>,
}

impl AppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl {
    #[doc = ""]
    pub virtual_node: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub weight: terrars::PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl {
        AppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl {
            port: core::default::Default::default(),
            virtual_node: self.virtual_node,
            weight: self.weight,
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElActionElWeightedTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttpRouteElActionElWeightedTargetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElHttpRouteElActionElWeightedTargetElRef {
        AppmeshRouteSpecElHttpRouteElActionElWeightedTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElActionElWeightedTargetElRef {
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
struct AppmeshRouteSpecElHttpRouteElActionElDynamic {
    weighted_target: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_target: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl>>,
    dynamic: AppmeshRouteSpecElHttpRouteElActionElDynamic,
}

impl AppmeshRouteSpecElHttpRouteElActionEl {
    #[doc = "Set the field `weighted_target`.\n"]
    pub fn set_weighted_target(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElHttpRouteElActionEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElActionEl {}

impl BuildAppmeshRouteSpecElHttpRouteElActionEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElActionEl {
        AppmeshRouteSpecElHttpRouteElActionEl {
            weighted_target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttpRouteElActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRouteSpecElHttpRouteElActionElRef {
        AppmeshRouteSpecElHttpRouteElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
    end: terrars::PrimField<f64>,
    start: terrars::PrimField<f64>,
}

impl AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl { }

impl terrars::ToListMappable for AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
    #[doc = ""]
    pub end: terrars::PrimField<f64>,
    #[doc = ""]
    pub start: terrars::PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
        AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
            end: self.end,
            start: self.start,
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef {
        AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef {
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
struct AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElDynamic {
    range: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl>>,
    dynamic: AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElDynamic,
}

impl AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl {}

impl BuildAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
        AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            regex: core::default::Default::default(),
            suffix: core::default::Default::default(),
            range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRef {
        AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRef {
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
    pub fn range(&self) -> terrars::ListRef<AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttpRouteElMatchElHeaderElDynamic {
    match_: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElMatchElHeaderEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl>>,
    dynamic: AppmeshRouteSpecElHttpRouteElMatchElHeaderElDynamic,
}

impl AppmeshRouteSpecElHttpRouteElMatchElHeaderEl {
    #[doc = "Set the field `invert`.\n"]
    pub fn set_invert(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElHttpRouteElMatchElHeaderEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchElHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElMatchElHeaderEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshRouteSpecElHttpRouteElMatchElHeaderEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElMatchElHeaderEl {
        AppmeshRouteSpecElHttpRouteElMatchElHeaderEl {
            invert: core::default::Default::default(),
            name: self.name,
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElMatchElHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttpRouteElMatchElHeaderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElHttpRouteElMatchElHeaderElRef {
        AppmeshRouteSpecElHttpRouteElMatchElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElMatchElHeaderElRef {
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
    pub fn match_(&self) -> terrars::ListRef<AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttpRouteElMatchElDynamic {
    header: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttpRouteElMatchElHeaderEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElMatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    prefix: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheme: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttpRouteElMatchElHeaderEl>>,
    dynamic: AppmeshRouteSpecElHttpRouteElMatchElDynamic,
}

impl AppmeshRouteSpecElHttpRouteElMatchEl {
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

    #[doc = "Set the field `scheme`.\n"]
    pub fn set_scheme(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scheme = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchElHeaderEl>>,
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
}

impl terrars::ToListMappable for AppmeshRouteSpecElHttpRouteElMatchEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElMatchEl {
    #[doc = ""]
    pub prefix: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshRouteSpecElHttpRouteElMatchEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElMatchEl {
        AppmeshRouteSpecElHttpRouteElMatchEl {
            method: core::default::Default::default(),
            port: core::default::Default::default(),
            prefix: self.prefix,
            scheme: core::default::Default::default(),
            header: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttpRouteElMatchElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRouteSpecElHttpRouteElMatchElRef {
        AppmeshRouteSpecElHttpRouteElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElMatchElRef {
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
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl { }

impl terrars::ToListMappable for AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl {
        AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutElRef {
        AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutElRef {
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
struct AppmeshRouteSpecElHttpRouteElRetryPolicyElDynamic {
    per_retry_timeout: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElRetryPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_retry_events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    max_retries: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_retry_events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_retry_timeout: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl>>,
    dynamic: AppmeshRouteSpecElHttpRouteElRetryPolicyElDynamic,
}

impl AppmeshRouteSpecElHttpRouteElRetryPolicyEl {
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
                    ::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElHttpRouteElRetryPolicyEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElRetryPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElRetryPolicyEl {
    #[doc = ""]
    pub max_retries: terrars::PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttpRouteElRetryPolicyEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElRetryPolicyEl {
        AppmeshRouteSpecElHttpRouteElRetryPolicyEl {
            http_retry_events: core::default::Default::default(),
            max_retries: self.max_retries,
            tcp_retry_events: core::default::Default::default(),
            per_retry_timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElRetryPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttpRouteElRetryPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRouteSpecElHttpRouteElRetryPolicyElRef {
        AppmeshRouteSpecElHttpRouteElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElRetryPolicyElRef {
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
    pub fn per_retry_timeout(&self) -> terrars::ListRef<AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_retry_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElTimeoutElIdleEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshRouteSpecElHttpRouteElTimeoutElIdleEl { }

impl terrars::ToListMappable for AppmeshRouteSpecElHttpRouteElTimeoutElIdleEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElTimeoutElIdleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElTimeoutElIdleEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttpRouteElTimeoutElIdleEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElTimeoutElIdleEl {
        AppmeshRouteSpecElHttpRouteElTimeoutElIdleEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElTimeoutElIdleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttpRouteElTimeoutElIdleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElHttpRouteElTimeoutElIdleElRef {
        AppmeshRouteSpecElHttpRouteElTimeoutElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElTimeoutElIdleElRef {
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
pub struct AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl { }

impl terrars::ToListMappable for AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl {
        AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestElRef {
        AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestElRef {
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
struct AppmeshRouteSpecElHttpRouteElTimeoutElDynamic {
    idle: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttpRouteElTimeoutElIdleEl>>,
    per_request: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElTimeoutEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttpRouteElTimeoutElIdleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_request: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl>>,
    dynamic: AppmeshRouteSpecElHttpRouteElTimeoutElDynamic,
}

impl AppmeshRouteSpecElHttpRouteElTimeoutEl {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElTimeoutElIdleEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElHttpRouteElTimeoutEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElTimeoutEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElTimeoutEl {}

impl BuildAppmeshRouteSpecElHttpRouteElTimeoutEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElTimeoutEl {
        AppmeshRouteSpecElHttpRouteElTimeoutEl {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElTimeoutElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttpRouteElTimeoutElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRouteSpecElHttpRouteElTimeoutElRef {
        AppmeshRouteSpecElHttpRouteElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElTimeoutElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> terrars::ListRef<AppmeshRouteSpecElHttpRouteElTimeoutElIdleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> terrars::ListRef<AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttpRouteElDynamic {
    action: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttpRouteElActionEl>>,
    match_: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttpRouteElMatchEl>>,
    retry_policy: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttpRouteElRetryPolicyEl>>,
    timeout: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttpRouteElTimeoutEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttpRouteElActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttpRouteElMatchEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttpRouteElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttpRouteElTimeoutEl>>,
    dynamic: AppmeshRouteSpecElHttpRouteElDynamic,
}

impl AppmeshRouteSpecElHttpRouteEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElActionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElRetryPolicyEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteElTimeoutEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElHttpRouteEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteEl {}

impl BuildAppmeshRouteSpecElHttpRouteEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteEl {
        AppmeshRouteSpecElHttpRouteEl {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElHttpRouteElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRouteSpecElHttpRouteElRef {
        AppmeshRouteSpecElHttpRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<AppmeshRouteSpecElHttpRouteElActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<AppmeshRouteSpecElHttpRouteElMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<AppmeshRouteSpecElHttpRouteElRetryPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<AppmeshRouteSpecElHttpRouteElTimeoutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    virtual_node: terrars::PrimField<std::string::String>,
    weight: terrars::PrimField<f64>,
}

impl AppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl {
    #[doc = ""]
    pub virtual_node: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub weight: terrars::PrimField<f64>,
}

impl BuildAppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl {
    pub fn build(self) -> AppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl {
        AppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl {
            port: core::default::Default::default(),
            virtual_node: self.virtual_node,
            weight: self.weight,
        }
    }
}

pub struct AppmeshRouteSpecElTcpRouteElActionElWeightedTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElTcpRouteElActionElWeightedTargetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElTcpRouteElActionElWeightedTargetElRef {
        AppmeshRouteSpecElTcpRouteElActionElWeightedTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElTcpRouteElActionElWeightedTargetElRef {
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
struct AppmeshRouteSpecElTcpRouteElActionElDynamic {
    weighted_target: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElTcpRouteElActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_target: std::option::Option<std::vec::Vec<AppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl>>,
    dynamic: AppmeshRouteSpecElTcpRouteElActionElDynamic,
}

impl AppmeshRouteSpecElTcpRouteElActionEl {
    #[doc = "Set the field `weighted_target`.\n"]
    pub fn set_weighted_target(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElTcpRouteElActionEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElTcpRouteElActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElTcpRouteElActionEl {}

impl BuildAppmeshRouteSpecElTcpRouteElActionEl {
    pub fn build(self) -> AppmeshRouteSpecElTcpRouteElActionEl {
        AppmeshRouteSpecElTcpRouteElActionEl {
            weighted_target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElTcpRouteElActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElTcpRouteElActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRouteSpecElTcpRouteElActionElRef {
        AppmeshRouteSpecElTcpRouteElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElTcpRouteElActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElTcpRouteElMatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl AppmeshRouteSpecElTcpRouteElMatchEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshRouteSpecElTcpRouteElMatchEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElTcpRouteElMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElTcpRouteElMatchEl {}

impl BuildAppmeshRouteSpecElTcpRouteElMatchEl {
    pub fn build(self) -> AppmeshRouteSpecElTcpRouteElMatchEl {
        AppmeshRouteSpecElTcpRouteElMatchEl { port: core::default::Default::default() }
    }
}

pub struct AppmeshRouteSpecElTcpRouteElMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElTcpRouteElMatchElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRouteSpecElTcpRouteElMatchElRef {
        AppmeshRouteSpecElTcpRouteElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElTcpRouteElMatchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElTcpRouteElTimeoutElIdleEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshRouteSpecElTcpRouteElTimeoutElIdleEl { }

impl terrars::ToListMappable for AppmeshRouteSpecElTcpRouteElTimeoutElIdleEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElTcpRouteElTimeoutElIdleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElTcpRouteElTimeoutElIdleEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshRouteSpecElTcpRouteElTimeoutElIdleEl {
    pub fn build(self) -> AppmeshRouteSpecElTcpRouteElTimeoutElIdleEl {
        AppmeshRouteSpecElTcpRouteElTimeoutElIdleEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshRouteSpecElTcpRouteElTimeoutElIdleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElTcpRouteElTimeoutElIdleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshRouteSpecElTcpRouteElTimeoutElIdleElRef {
        AppmeshRouteSpecElTcpRouteElTimeoutElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElTcpRouteElTimeoutElIdleElRef {
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
struct AppmeshRouteSpecElTcpRouteElTimeoutElDynamic {
    idle: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElTcpRouteElTimeoutElIdleEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElTcpRouteElTimeoutEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle: std::option::Option<std::vec::Vec<AppmeshRouteSpecElTcpRouteElTimeoutElIdleEl>>,
    dynamic: AppmeshRouteSpecElTcpRouteElTimeoutElDynamic,
}

impl AppmeshRouteSpecElTcpRouteElTimeoutEl {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElTcpRouteElTimeoutElIdleEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElTcpRouteElTimeoutEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElTcpRouteElTimeoutEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElTcpRouteElTimeoutEl {}

impl BuildAppmeshRouteSpecElTcpRouteElTimeoutEl {
    pub fn build(self) -> AppmeshRouteSpecElTcpRouteElTimeoutEl {
        AppmeshRouteSpecElTcpRouteElTimeoutEl {
            idle: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElTcpRouteElTimeoutElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElTcpRouteElTimeoutElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRouteSpecElTcpRouteElTimeoutElRef {
        AppmeshRouteSpecElTcpRouteElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElTcpRouteElTimeoutElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> terrars::ListRef<AppmeshRouteSpecElTcpRouteElTimeoutElIdleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElTcpRouteElDynamic {
    action: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElTcpRouteElActionEl>>,
    match_: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElTcpRouteElMatchEl>>,
    timeout: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElTcpRouteElTimeoutEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElTcpRouteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<AppmeshRouteSpecElTcpRouteElActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshRouteSpecElTcpRouteElMatchEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<AppmeshRouteSpecElTcpRouteElTimeoutEl>>,
    dynamic: AppmeshRouteSpecElTcpRouteElDynamic,
}

impl AppmeshRouteSpecElTcpRouteEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElTcpRouteElActionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElTcpRouteElMatchEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElTcpRouteElTimeoutEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecElTcpRouteEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecElTcpRouteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElTcpRouteEl {}

impl BuildAppmeshRouteSpecElTcpRouteEl {
    pub fn build(self) -> AppmeshRouteSpecElTcpRouteEl {
        AppmeshRouteSpecElTcpRouteEl {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElTcpRouteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElTcpRouteElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRouteSpecElTcpRouteElRef {
        AppmeshRouteSpecElTcpRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElTcpRouteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<AppmeshRouteSpecElTcpRouteElActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<AppmeshRouteSpecElTcpRouteElMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<AppmeshRouteSpecElTcpRouteElTimeoutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElDynamic {
    grpc_route: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElGrpcRouteEl>>,
    http2_route: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttp2RouteEl>>,
    http_route: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElHttpRouteEl>>,
    tcp_route: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecElTcpRouteEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc_route: std::option::Option<std::vec::Vec<AppmeshRouteSpecElGrpcRouteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http2_route: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttp2RouteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_route: std::option::Option<std::vec::Vec<AppmeshRouteSpecElHttpRouteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_route: std::option::Option<std::vec::Vec<AppmeshRouteSpecElTcpRouteEl>>,
    dynamic: AppmeshRouteSpecElDynamic,
}

impl AppmeshRouteSpecEl {
    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grpc_route`.\n"]
    pub fn set_grpc_route(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElGrpcRouteEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttp2RouteEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElHttpRouteEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshRouteSpecElTcpRouteEl>>,
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

impl terrars::ToListMappable for AppmeshRouteSpecEl {
    type O = terrars::BlockAssignable<AppmeshRouteSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecEl {}

impl BuildAppmeshRouteSpecEl {
    pub fn build(self) -> AppmeshRouteSpecEl {
        AppmeshRouteSpecEl {
            priority: core::default::Default::default(),
            grpc_route: core::default::Default::default(),
            http2_route: core::default::Default::default(),
            http_route: core::default::Default::default(),
            tcp_route: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshRouteSpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshRouteSpecElRef {
        AppmeshRouteSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `grpc_route` after provisioning.\n"]
    pub fn grpc_route(&self) -> terrars::ListRef<AppmeshRouteSpecElGrpcRouteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc_route", self.base))
    }

    #[doc = "Get a reference to the value of field `http2_route` after provisioning.\n"]
    pub fn http2_route(&self) -> terrars::ListRef<AppmeshRouteSpecElHttp2RouteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http2_route", self.base))
    }

    #[doc = "Get a reference to the value of field `http_route` after provisioning.\n"]
    pub fn http_route(&self) -> terrars::ListRef<AppmeshRouteSpecElHttpRouteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_route", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_route` after provisioning.\n"]
    pub fn tcp_route(&self) -> terrars::ListRef<AppmeshRouteSpecElTcpRouteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tcp_route", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteDynamic {
    spec: std::option::Option<terrars::DynamicBlock<AppmeshRouteSpecEl>>,
}
