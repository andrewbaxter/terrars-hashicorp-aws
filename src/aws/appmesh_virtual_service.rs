use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppmeshVirtualServiceData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<std::vec::Vec<AppmeshVirtualServiceSpecEl>>,
    dynamic: AppmeshVirtualServiceDynamic,
}

struct AppmeshVirtualService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppmeshVirtualServiceData>,
}

#[derive(Clone)]
pub struct AppmeshVirtualService(std::rc::Rc<AppmeshVirtualService_>);

impl AppmeshVirtualService {
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
    pub fn set_spec(self, v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualServiceSpecEl>>) -> Self {
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

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<AppmeshVirtualServiceSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }
}

impl terrars::Referable for AppmeshVirtualService {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppmeshVirtualService { }

impl terrars::ToListMappable for AppmeshVirtualService {
    type O = terrars::ListRef<AppmeshVirtualServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppmeshVirtualService_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_appmesh_virtual_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppmeshVirtualService {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub mesh_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualService {
    pub fn build(self, stack: &mut terrars::Stack) -> AppmeshVirtualService {
        let out = AppmeshVirtualService(std::rc::Rc::new(AppmeshVirtualService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppmeshVirtualServiceData {
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
                spec: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppmeshVirtualServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppmeshVirtualServiceRef {
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

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<AppmeshVirtualServiceSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
    virtual_node_name: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualServiceSpecElProviderElVirtualNodeEl { }

impl terrars::ToListMappable for AppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
    type O = terrars::BlockAssignable<AppmeshVirtualServiceSpecElProviderElVirtualNodeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
    #[doc = ""]
    pub virtual_node_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
    pub fn build(self) -> AppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
        AppmeshVirtualServiceSpecElProviderElVirtualNodeEl { virtual_node_name: self.virtual_node_name }
    }
}

pub struct AppmeshVirtualServiceSpecElProviderElVirtualNodeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualServiceSpecElProviderElVirtualNodeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualServiceSpecElProviderElVirtualNodeElRef {
        AppmeshVirtualServiceSpecElProviderElVirtualNodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualServiceSpecElProviderElVirtualNodeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_node_name` after provisioning.\n"]
    pub fn virtual_node_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_node_name", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
    virtual_router_name: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualServiceSpecElProviderElVirtualRouterEl { }

impl terrars::ToListMappable for AppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
    type O = terrars::BlockAssignable<AppmeshVirtualServiceSpecElProviderElVirtualRouterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
    #[doc = ""]
    pub virtual_router_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
    pub fn build(self) -> AppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
        AppmeshVirtualServiceSpecElProviderElVirtualRouterEl { virtual_router_name: self.virtual_router_name }
    }
}

pub struct AppmeshVirtualServiceSpecElProviderElVirtualRouterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualServiceSpecElProviderElVirtualRouterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualServiceSpecElProviderElVirtualRouterElRef {
        AppmeshVirtualServiceSpecElProviderElVirtualRouterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualServiceSpecElProviderElVirtualRouterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_router_name` after provisioning.\n"]
    pub fn virtual_router_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_router_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualServiceSpecElProviderElDynamic {
    virtual_node: std::option::Option<terrars::DynamicBlock<AppmeshVirtualServiceSpecElProviderElVirtualNodeEl>>,
    virtual_router: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualServiceSpecElProviderElVirtualRouterEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualServiceSpecElProviderEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_node: std::option::Option<std::vec::Vec<AppmeshVirtualServiceSpecElProviderElVirtualNodeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_router: std::option::Option<std::vec::Vec<AppmeshVirtualServiceSpecElProviderElVirtualRouterEl>>,
    dynamic: AppmeshVirtualServiceSpecElProviderElDynamic,
}

impl AppmeshVirtualServiceSpecElProviderEl {
    #[doc = "Set the field `virtual_node`.\n"]
    pub fn set_virtual_node(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualServiceSpecElProviderElVirtualNodeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.virtual_node = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.virtual_node = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `virtual_router`.\n"]
    pub fn set_virtual_router(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualServiceSpecElProviderElVirtualRouterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.virtual_router = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.virtual_router = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualServiceSpecElProviderEl {
    type O = terrars::BlockAssignable<AppmeshVirtualServiceSpecElProviderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualServiceSpecElProviderEl {}

impl BuildAppmeshVirtualServiceSpecElProviderEl {
    pub fn build(self) -> AppmeshVirtualServiceSpecElProviderEl {
        AppmeshVirtualServiceSpecElProviderEl {
            virtual_node: core::default::Default::default(),
            virtual_router: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualServiceSpecElProviderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualServiceSpecElProviderElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualServiceSpecElProviderElRef {
        AppmeshVirtualServiceSpecElProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualServiceSpecElProviderElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_node` after provisioning.\n"]
    pub fn virtual_node(&self) -> terrars::ListRef<AppmeshVirtualServiceSpecElProviderElVirtualNodeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_node", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_router` after provisioning.\n"]
    pub fn virtual_router(&self) -> terrars::ListRef<AppmeshVirtualServiceSpecElProviderElVirtualRouterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_router", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualServiceSpecElDynamic {
    provider: std::option::Option<terrars::DynamicBlock<AppmeshVirtualServiceSpecElProviderEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualServiceSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::vec::Vec<AppmeshVirtualServiceSpecElProviderEl>>,
    dynamic: AppmeshVirtualServiceSpecElDynamic,
}

impl AppmeshVirtualServiceSpecEl {
    #[doc = "Set the field `provider`.\n"]
    pub fn set_provider(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualServiceSpecElProviderEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.provider = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.provider = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualServiceSpecEl {
    type O = terrars::BlockAssignable<AppmeshVirtualServiceSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualServiceSpecEl {}

impl BuildAppmeshVirtualServiceSpecEl {
    pub fn build(self) -> AppmeshVirtualServiceSpecEl {
        AppmeshVirtualServiceSpecEl {
            provider: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualServiceSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualServiceSpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualServiceSpecElRef {
        AppmeshVirtualServiceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualServiceSpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> terrars::ListRef<AppmeshVirtualServiceSpecElProviderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provider", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualServiceDynamic {
    spec: std::option::Option<terrars::DynamicBlock<AppmeshVirtualServiceSpecEl>>,
}
