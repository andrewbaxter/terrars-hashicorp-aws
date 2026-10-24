use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAppmeshVirtualServiceData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    mesh_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mesh_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataAppmeshVirtualService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAppmeshVirtualServiceData>,
}

#[derive(Clone)]
pub struct DataAppmeshVirtualService(std::rc::Rc<DataAppmeshVirtualService_>);

impl DataAppmeshVirtualService {
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

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<DataAppmeshVirtualServiceSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataAppmeshVirtualService {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAppmeshVirtualService { }

impl terrars::ToListMappable for DataAppmeshVirtualService {
    type O = terrars::ListRef<DataAppmeshVirtualServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAppmeshVirtualService_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_appmesh_virtual_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAppmeshVirtualService {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub mesh_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataAppmeshVirtualService {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAppmeshVirtualService {
        let out = DataAppmeshVirtualService(std::rc::Rc::new(DataAppmeshVirtualService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAppmeshVirtualServiceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                mesh_name: self.mesh_name,
                mesh_owner: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAppmeshVirtualServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAppmeshVirtualServiceRef {
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

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<DataAppmeshVirtualServiceSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_node_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
    #[doc = "Set the field `virtual_node_name`.\n"]
    pub fn set_virtual_node_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.virtual_node_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
    type O = terrars::BlockAssignable<DataAppmeshVirtualServiceSpecElProviderElVirtualNodeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualServiceSpecElProviderElVirtualNodeEl {}

impl BuildDataAppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
    pub fn build(self) -> DataAppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
        DataAppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
            virtual_node_name: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualServiceSpecElProviderElVirtualNodeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualServiceSpecElProviderElVirtualNodeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualServiceSpecElProviderElVirtualNodeElRef {
        DataAppmeshVirtualServiceSpecElProviderElVirtualNodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualServiceSpecElProviderElVirtualNodeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_node_name` after provisioning.\n"]
    pub fn virtual_node_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_node_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_router_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
    #[doc = "Set the field `virtual_router_name`.\n"]
    pub fn set_virtual_router_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.virtual_router_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
    type O = terrars::BlockAssignable<DataAppmeshVirtualServiceSpecElProviderElVirtualRouterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualServiceSpecElProviderElVirtualRouterEl {}

impl BuildDataAppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
    pub fn build(self) -> DataAppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
        DataAppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
            virtual_router_name: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualServiceSpecElProviderElVirtualRouterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualServiceSpecElProviderElVirtualRouterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualServiceSpecElProviderElVirtualRouterElRef {
        DataAppmeshVirtualServiceSpecElProviderElVirtualRouterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualServiceSpecElProviderElVirtualRouterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_router_name` after provisioning.\n"]
    pub fn virtual_router_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_router_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualServiceSpecElProviderEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_node: std::option::Option<terrars::ListField<DataAppmeshVirtualServiceSpecElProviderElVirtualNodeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_router: std::option::Option<terrars::ListField<DataAppmeshVirtualServiceSpecElProviderElVirtualRouterEl>>,
}

impl DataAppmeshVirtualServiceSpecElProviderEl {
    #[doc = "Set the field `virtual_node`.\n"]
    pub fn set_virtual_node(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualServiceSpecElProviderElVirtualNodeEl>>,
    ) -> Self {
        self.virtual_node = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_router`.\n"]
    pub fn set_virtual_router(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualServiceSpecElProviderElVirtualRouterEl>>,
    ) -> Self {
        self.virtual_router = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualServiceSpecElProviderEl {
    type O = terrars::BlockAssignable<DataAppmeshVirtualServiceSpecElProviderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualServiceSpecElProviderEl {}

impl BuildDataAppmeshVirtualServiceSpecElProviderEl {
    pub fn build(self) -> DataAppmeshVirtualServiceSpecElProviderEl {
        DataAppmeshVirtualServiceSpecElProviderEl {
            virtual_node: core::default::Default::default(),
            virtual_router: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualServiceSpecElProviderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualServiceSpecElProviderElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshVirtualServiceSpecElProviderElRef {
        DataAppmeshVirtualServiceSpecElProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualServiceSpecElProviderElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_node` after provisioning.\n"]
    pub fn virtual_node(&self) -> terrars::ListRef<DataAppmeshVirtualServiceSpecElProviderElVirtualNodeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_node", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_router` after provisioning.\n"]
    pub fn virtual_router(&self) -> terrars::ListRef<DataAppmeshVirtualServiceSpecElProviderElVirtualRouterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_router", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualServiceSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<terrars::ListField<DataAppmeshVirtualServiceSpecElProviderEl>>,
}

impl DataAppmeshVirtualServiceSpecEl {
    #[doc = "Set the field `provider`.\n"]
    pub fn set_provider(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualServiceSpecElProviderEl>>,
    ) -> Self {
        self.provider = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualServiceSpecEl {
    type O = terrars::BlockAssignable<DataAppmeshVirtualServiceSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualServiceSpecEl {}

impl BuildDataAppmeshVirtualServiceSpecEl {
    pub fn build(self) -> DataAppmeshVirtualServiceSpecEl {
        DataAppmeshVirtualServiceSpecEl { provider: core::default::Default::default() }
    }
}

pub struct DataAppmeshVirtualServiceSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualServiceSpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshVirtualServiceSpecElRef {
        DataAppmeshVirtualServiceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualServiceSpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> terrars::ListRef<DataAppmeshVirtualServiceSpecElProviderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provider", self.base))
    }
}
