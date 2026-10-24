use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppmeshGatewayRouteData {
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
    virtual_gateway_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<std::vec::Vec<AppmeshGatewayRouteSpecEl>>,
    dynamic: AppmeshGatewayRouteDynamic,
}

struct AppmeshGatewayRoute_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppmeshGatewayRouteData>,
}

#[derive(Clone)]
pub struct AppmeshGatewayRoute(std::rc::Rc<AppmeshGatewayRoute_>);

impl AppmeshGatewayRoute {
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
    pub fn set_spec(self, v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRouteSpecEl>>) -> Self {
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

    #[doc = "Get a reference to the value of field `virtual_gateway_name` after provisioning.\n"]
    pub fn virtual_gateway_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_gateway_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }
}

impl terrars::Referable for AppmeshGatewayRoute {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppmeshGatewayRoute { }

impl terrars::ToListMappable for AppmeshGatewayRoute {
    type O = terrars::ListRef<AppmeshGatewayRouteRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppmeshGatewayRoute_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_appmesh_gateway_route".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppmeshGatewayRoute {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub mesh_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub virtual_gateway_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshGatewayRoute {
    pub fn build(self, stack: &mut terrars::Stack) -> AppmeshGatewayRoute {
        let out = AppmeshGatewayRoute(std::rc::Rc::new(AppmeshGatewayRoute_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppmeshGatewayRouteData {
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
                virtual_gateway_name: self.virtual_gateway_name,
                spec: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppmeshGatewayRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppmeshGatewayRouteRef {
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

    #[doc = "Get a reference to the value of field `virtual_gateway_name` after provisioning.\n"]
    pub fn virtual_gateway_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_gateway_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl {
    virtual_service_name: terrars::PrimField<std::string::String>,
}

impl AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl { }

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl {
    #[doc = ""]
    pub virtual_service_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl {
        AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl {
            virtual_service_name: self.virtual_service_name,
        }
    }
}

pub struct AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceElRef {
        AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_service_name` after provisioning.\n"]
    pub fn virtual_service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_service_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElDynamic {
    virtual_service: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_service: std
    ::option
    ::Option<std::vec::Vec<AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl>>,
    dynamic: AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElDynamic,
}

impl AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl {
    #[doc = "Set the field `virtual_service`.\n"]
    pub fn set_virtual_service(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.virtual_service = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.virtual_service = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl {}

impl BuildAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl {
        AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl {
            virtual_service: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElRef {
        AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_service` after provisioning.\n"]
    pub fn virtual_service(
        &self,
    ) -> terrars::ListRef<AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_service", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElGrpcRouteElActionElDynamic {
    target: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElGrpcRouteElActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<std::vec::Vec<AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl>>,
    dynamic: AppmeshGatewayRouteSpecElGrpcRouteElActionElDynamic,
}

impl AppmeshGatewayRouteSpecElGrpcRouteElActionEl {
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElGrpcRouteElActionEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElGrpcRouteElActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElGrpcRouteElActionEl {}

impl BuildAppmeshGatewayRouteSpecElGrpcRouteElActionEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElGrpcRouteElActionEl {
        AppmeshGatewayRouteSpecElGrpcRouteElActionEl {
            target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElGrpcRouteElActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElGrpcRouteElActionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRouteSpecElGrpcRouteElActionElRef {
        AppmeshGatewayRouteSpecElGrpcRouteElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElGrpcRouteElActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElGrpcRouteElMatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    service_name: terrars::PrimField<std::string::String>,
}

impl AppmeshGatewayRouteSpecElGrpcRouteElMatchEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElGrpcRouteElMatchEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElGrpcRouteElMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElGrpcRouteElMatchEl {
    #[doc = ""]
    pub service_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshGatewayRouteSpecElGrpcRouteElMatchEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElGrpcRouteElMatchEl {
        AppmeshGatewayRouteSpecElGrpcRouteElMatchEl {
            port: core::default::Default::default(),
            service_name: self.service_name,
        }
    }
}

pub struct AppmeshGatewayRouteSpecElGrpcRouteElMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElGrpcRouteElMatchElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRouteSpecElGrpcRouteElMatchElRef {
        AppmeshGatewayRouteSpecElGrpcRouteElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElGrpcRouteElMatchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElGrpcRouteElDynamic {
    action: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElGrpcRouteElActionEl>>,
    match_: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElGrpcRouteElMatchEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElGrpcRouteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<AppmeshGatewayRouteSpecElGrpcRouteElActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshGatewayRouteSpecElGrpcRouteElMatchEl>>,
    dynamic: AppmeshGatewayRouteSpecElGrpcRouteElDynamic,
}

impl AppmeshGatewayRouteSpecElGrpcRouteEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRouteSpecElGrpcRouteElActionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRouteSpecElGrpcRouteElMatchEl>>,
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

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElGrpcRouteEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElGrpcRouteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElGrpcRouteEl {}

impl BuildAppmeshGatewayRouteSpecElGrpcRouteEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElGrpcRouteEl {
        AppmeshGatewayRouteSpecElGrpcRouteEl {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElGrpcRouteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElGrpcRouteElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshGatewayRouteSpecElGrpcRouteElRef {
        AppmeshGatewayRouteSpecElGrpcRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElGrpcRouteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElGrpcRouteElActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElGrpcRouteElMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl {
    default_target_hostname: terrars::PrimField<std::string::String>,
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl { }

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl {
    #[doc = ""]
    pub default_target_hostname: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl {
            default_target_hostname: self.default_target_hostname,
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameElRef {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_target_hostname` after provisioning.\n"]
    pub fn default_target_hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_target_hostname", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl {
    #[doc = "Set the field `default_prefix`.\n"]
    pub fn set_default_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl {}

impl BuildAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl {
            default_prefix: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixElRef {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_prefix` after provisioning.\n"]
    pub fn default_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElDynamic {
    hostname: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl>>,
    prefix: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<std::vec::Vec<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<std::vec::Vec<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl>>,
    dynamic: AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElDynamic,
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl {
    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hostname = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hostname = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.prefix = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.prefix = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl {}

impl BuildAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl {
            hostname: core::default::Default::default(),
            prefix: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElRef {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl {
    virtual_service_name: terrars::PrimField<std::string::String>,
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl { }

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl {
    #[doc = ""]
    pub virtual_service_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl {
            virtual_service_name: self.virtual_service_name,
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceElRef {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_service_name` after provisioning.\n"]
    pub fn virtual_service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_service_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElDynamic {
    virtual_service: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_service: std
    ::option
    ::Option<std::vec::Vec<AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl>>,
    dynamic: AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElDynamic,
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl {
    #[doc = "Set the field `virtual_service`.\n"]
    pub fn set_virtual_service(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.virtual_service = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.virtual_service = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl {}

impl BuildAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl {
            virtual_service: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElRef {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_service` after provisioning.\n"]
    pub fn virtual_service(
        &self,
    ) -> terrars::ListRef<AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_service", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElHttp2RouteElActionElDynamic {
    rewrite: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl>>,
    target: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rewrite: std::option::Option<std::vec::Vec<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<std::vec::Vec<AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl>>,
    dynamic: AppmeshGatewayRouteSpecElHttp2RouteElActionElDynamic,
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionEl {
    #[doc = "Set the field `rewrite`.\n"]
    pub fn set_rewrite(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rewrite = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rewrite = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElHttp2RouteElActionEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttp2RouteElActionEl {}

impl BuildAppmeshGatewayRouteSpecElHttp2RouteElActionEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttp2RouteElActionEl {
        AppmeshGatewayRouteSpecElHttp2RouteElActionEl {
            rewrite: core::default::Default::default(),
            target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElHttp2RouteElActionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElRef {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rewrite` after provisioning.\n"]
    pub fn rewrite(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rewrite", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `suffix`.\n"]
    pub fn set_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.suffix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl {}

impl BuildAppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl {
        AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl {
            exact: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameElRef {
        AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `suffix` after provisioning.\n"]
    pub fn suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElHttp2RouteElMatchElDynamic {
    hostname: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttp2RouteElMatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<std::vec::Vec<AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl>>,
    dynamic: AppmeshGatewayRouteSpecElHttp2RouteElMatchElDynamic,
}

impl AppmeshGatewayRouteSpecElHttp2RouteElMatchEl {
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

    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hostname = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hostname = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElHttp2RouteElMatchEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttp2RouteElMatchEl {}

impl BuildAppmeshGatewayRouteSpecElHttp2RouteElMatchEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttp2RouteElMatchEl {
        AppmeshGatewayRouteSpecElHttp2RouteElMatchEl {
            port: core::default::Default::default(),
            prefix: core::default::Default::default(),
            hostname: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttp2RouteElMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElHttp2RouteElMatchElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRouteSpecElHttp2RouteElMatchElRef {
        AppmeshGatewayRouteSpecElHttp2RouteElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttp2RouteElMatchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElHttp2RouteElDynamic {
    action: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElHttp2RouteElActionEl>>,
    match_: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElHttp2RouteElMatchEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttp2RouteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<AppmeshGatewayRouteSpecElHttp2RouteElActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshGatewayRouteSpecElHttp2RouteElMatchEl>>,
    dynamic: AppmeshGatewayRouteSpecElHttp2RouteElDynamic,
}

impl AppmeshGatewayRouteSpecElHttp2RouteEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElMatchEl>>,
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

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElHttp2RouteEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttp2RouteEl {}

impl BuildAppmeshGatewayRouteSpecElHttp2RouteEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttp2RouteEl {
        AppmeshGatewayRouteSpecElHttp2RouteEl {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttp2RouteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElHttp2RouteElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshGatewayRouteSpecElHttp2RouteElRef {
        AppmeshGatewayRouteSpecElHttp2RouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttp2RouteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElHttp2RouteElActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElHttp2RouteElMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl {
    default_target_hostname: terrars::PrimField<std::string::String>,
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl { }

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl {
    #[doc = ""]
    pub default_target_hostname: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl {
        AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl {
            default_target_hostname: self.default_target_hostname,
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameElRef {
        AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_target_hostname` after provisioning.\n"]
    pub fn default_target_hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_target_hostname", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl {
    #[doc = "Set the field `default_prefix`.\n"]
    pub fn set_default_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl {}

impl BuildAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl {
        AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl {
            default_prefix: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixElRef {
        AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_prefix` after provisioning.\n"]
    pub fn default_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElDynamic {
    hostname: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl>>,
    prefix: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<std::vec::Vec<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<std::vec::Vec<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl>>,
    dynamic: AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElDynamic,
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl {
    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hostname = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hostname = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.prefix = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.prefix = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl {}

impl BuildAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl {
        AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl {
            hostname: core::default::Default::default(),
            prefix: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElRef {
        AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl {
    virtual_service_name: terrars::PrimField<std::string::String>,
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl { }

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl {
    #[doc = ""]
    pub virtual_service_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl {
        AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl {
            virtual_service_name: self.virtual_service_name,
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceElRef {
        AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_service_name` after provisioning.\n"]
    pub fn virtual_service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_service_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElDynamic {
    virtual_service: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_service: std
    ::option
    ::Option<std::vec::Vec<AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl>>,
    dynamic: AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElDynamic,
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl {
    #[doc = "Set the field `virtual_service`.\n"]
    pub fn set_virtual_service(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.virtual_service = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.virtual_service = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl {}

impl BuildAppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl {
        AppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl {
            virtual_service: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElRef {
        AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_service` after provisioning.\n"]
    pub fn virtual_service(
        &self,
    ) -> terrars::ListRef<AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_service", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElHttpRouteElActionElDynamic {
    rewrite: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl>>,
    target: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttpRouteElActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rewrite: std::option::Option<std::vec::Vec<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<std::vec::Vec<AppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl>>,
    dynamic: AppmeshGatewayRouteSpecElHttpRouteElActionElDynamic,
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionEl {
    #[doc = "Set the field `rewrite`.\n"]
    pub fn set_rewrite(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rewrite = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rewrite = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElHttpRouteElActionEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttpRouteElActionEl {}

impl BuildAppmeshGatewayRouteSpecElHttpRouteElActionEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttpRouteElActionEl {
        AppmeshGatewayRouteSpecElHttpRouteElActionEl {
            rewrite: core::default::Default::default(),
            target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElHttpRouteElActionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRouteSpecElHttpRouteElActionElRef {
        AppmeshGatewayRouteSpecElHttpRouteElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rewrite` after provisioning.\n"]
    pub fn rewrite(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rewrite", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `suffix`.\n"]
    pub fn set_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.suffix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl {}

impl BuildAppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl {
        AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl {
            exact: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameElRef {
        AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `suffix` after provisioning.\n"]
    pub fn suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElHttpRouteElMatchElDynamic {
    hostname: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttpRouteElMatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<std::vec::Vec<AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl>>,
    dynamic: AppmeshGatewayRouteSpecElHttpRouteElMatchElDynamic,
}

impl AppmeshGatewayRouteSpecElHttpRouteElMatchEl {
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

    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hostname = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hostname = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElHttpRouteElMatchEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttpRouteElMatchEl {}

impl BuildAppmeshGatewayRouteSpecElHttpRouteElMatchEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttpRouteElMatchEl {
        AppmeshGatewayRouteSpecElHttpRouteElMatchEl {
            port: core::default::Default::default(),
            prefix: core::default::Default::default(),
            hostname: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttpRouteElMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElHttpRouteElMatchElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRouteSpecElHttpRouteElMatchElRef {
        AppmeshGatewayRouteSpecElHttpRouteElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttpRouteElMatchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElHttpRouteElDynamic {
    action: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElHttpRouteElActionEl>>,
    match_: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElHttpRouteElMatchEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttpRouteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<AppmeshGatewayRouteSpecElHttpRouteElActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshGatewayRouteSpecElHttpRouteElMatchEl>>,
    dynamic: AppmeshGatewayRouteSpecElHttpRouteElDynamic,
}

impl AppmeshGatewayRouteSpecElHttpRouteEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElMatchEl>>,
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

impl terrars::ToListMappable for AppmeshGatewayRouteSpecElHttpRouteEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttpRouteEl {}

impl BuildAppmeshGatewayRouteSpecElHttpRouteEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttpRouteEl {
        AppmeshGatewayRouteSpecElHttpRouteEl {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttpRouteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElHttpRouteElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshGatewayRouteSpecElHttpRouteElRef {
        AppmeshGatewayRouteSpecElHttpRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttpRouteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElHttpRouteElActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElHttpRouteElMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElDynamic {
    grpc_route: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElGrpcRouteEl>>,
    http2_route: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElHttp2RouteEl>>,
    http_route: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecElHttpRouteEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc_route: std::option::Option<std::vec::Vec<AppmeshGatewayRouteSpecElGrpcRouteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http2_route: std::option::Option<std::vec::Vec<AppmeshGatewayRouteSpecElHttp2RouteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_route: std::option::Option<std::vec::Vec<AppmeshGatewayRouteSpecElHttpRouteEl>>,
    dynamic: AppmeshGatewayRouteSpecElDynamic,
}

impl AppmeshGatewayRouteSpecEl {
    #[doc = "Set the field `grpc_route`.\n"]
    pub fn set_grpc_route(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRouteSpecElGrpcRouteEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteEl>>,
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
}

impl terrars::ToListMappable for AppmeshGatewayRouteSpecEl {
    type O = terrars::BlockAssignable<AppmeshGatewayRouteSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecEl {}

impl BuildAppmeshGatewayRouteSpecEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecEl {
        AppmeshGatewayRouteSpecEl {
            grpc_route: core::default::Default::default(),
            http2_route: core::default::Default::default(),
            http_route: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRouteSpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshGatewayRouteSpecElRef {
        AppmeshGatewayRouteSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grpc_route` after provisioning.\n"]
    pub fn grpc_route(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElGrpcRouteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc_route", self.base))
    }

    #[doc = "Get a reference to the value of field `http2_route` after provisioning.\n"]
    pub fn http2_route(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElHttp2RouteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http2_route", self.base))
    }

    #[doc = "Get a reference to the value of field `http_route` after provisioning.\n"]
    pub fn http_route(&self) -> terrars::ListRef<AppmeshGatewayRouteSpecElHttpRouteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_route", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteDynamic {
    spec: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRouteSpecEl>>,
}
