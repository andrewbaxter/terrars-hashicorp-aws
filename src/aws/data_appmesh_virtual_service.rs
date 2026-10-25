#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAppmeshVirtualServiceData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataAppmeshVirtualService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAppmeshVirtualServiceData_>,
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

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<DataAppmeshVirtualService_SpecRef> {
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
            data: std::cell::RefCell::new(DataAppmeshVirtualServiceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                mesh_name: self.mesh_name,
                mesh_owner: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<DataAppmeshVirtualService_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualService_Spec_Provider_VirtualNode {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_node_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshVirtualService_Spec_Provider_VirtualNode {
    #[doc = "Set the field `virtual_node_name`.\n"]
    pub fn set_virtual_node_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.virtual_node_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualService_Spec_Provider_VirtualNode {
    type O = terrars::BlockAssignable<DataAppmeshVirtualService_Spec_Provider_VirtualNode>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualService_Spec_Provider_VirtualNode {}

impl BuildDataAppmeshVirtualService_Spec_Provider_VirtualNode {
    pub fn build(self) -> DataAppmeshVirtualService_Spec_Provider_VirtualNode {
        DataAppmeshVirtualService_Spec_Provider_VirtualNode { virtual_node_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualService_Spec_Provider_VirtualNodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualService_Spec_Provider_VirtualNodeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualService_Spec_Provider_VirtualNodeRef {
        DataAppmeshVirtualService_Spec_Provider_VirtualNodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualService_Spec_Provider_VirtualNodeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_node_name` after provisioning.\n"]
    pub fn virtual_node_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_node_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualService_Spec_Provider_VirtualRouter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_router_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshVirtualService_Spec_Provider_VirtualRouter {
    #[doc = "Set the field `virtual_router_name`.\n"]
    pub fn set_virtual_router_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.virtual_router_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualService_Spec_Provider_VirtualRouter {
    type O = terrars::BlockAssignable<DataAppmeshVirtualService_Spec_Provider_VirtualRouter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualService_Spec_Provider_VirtualRouter {}

impl BuildDataAppmeshVirtualService_Spec_Provider_VirtualRouter {
    pub fn build(self) -> DataAppmeshVirtualService_Spec_Provider_VirtualRouter {
        DataAppmeshVirtualService_Spec_Provider_VirtualRouter {
            virtual_router_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualService_Spec_Provider_VirtualRouterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualService_Spec_Provider_VirtualRouterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualService_Spec_Provider_VirtualRouterRef {
        DataAppmeshVirtualService_Spec_Provider_VirtualRouterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualService_Spec_Provider_VirtualRouterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_router_name` after provisioning.\n"]
    pub fn virtual_router_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_router_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualService_Spec_Provider {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_node: std::option::Option<terrars::ListField<DataAppmeshVirtualService_Spec_Provider_VirtualNode>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_router: std::option::Option<terrars::ListField<DataAppmeshVirtualService_Spec_Provider_VirtualRouter>>,
}

impl DataAppmeshVirtualService_Spec_Provider {
    #[doc = "Set the field `virtual_node`.\n"]
    pub fn set_virtual_node(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualService_Spec_Provider_VirtualNode>>,
    ) -> Self {
        self.virtual_node = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_router`.\n"]
    pub fn set_virtual_router(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualService_Spec_Provider_VirtualRouter>>,
    ) -> Self {
        self.virtual_router = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualService_Spec_Provider {
    type O = terrars::BlockAssignable<DataAppmeshVirtualService_Spec_Provider>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualService_Spec_Provider {}

impl BuildDataAppmeshVirtualService_Spec_Provider {
    pub fn build(self) -> DataAppmeshVirtualService_Spec_Provider {
        DataAppmeshVirtualService_Spec_Provider {
            virtual_node: core::default::Default::default(),
            virtual_router: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualService_Spec_ProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualService_Spec_ProviderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshVirtualService_Spec_ProviderRef {
        DataAppmeshVirtualService_Spec_ProviderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualService_Spec_ProviderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_node` after provisioning.\n"]
    pub fn virtual_node(&self) -> terrars::ListRef<DataAppmeshVirtualService_Spec_Provider_VirtualNodeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_node", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_router` after provisioning.\n"]
    pub fn virtual_router(&self) -> terrars::ListRef<DataAppmeshVirtualService_Spec_Provider_VirtualRouterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_router", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualService_Spec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<terrars::ListField<DataAppmeshVirtualService_Spec_Provider>>,
}

impl DataAppmeshVirtualService_Spec {
    #[doc = "Set the field `provider`.\n"]
    pub fn set_provider(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualService_Spec_Provider>>,
    ) -> Self {
        self.provider = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualService_Spec {
    type O = terrars::BlockAssignable<DataAppmeshVirtualService_Spec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualService_Spec {}

impl BuildDataAppmeshVirtualService_Spec {
    pub fn build(self) -> DataAppmeshVirtualService_Spec {
        DataAppmeshVirtualService_Spec { provider: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualService_SpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualService_SpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshVirtualService_SpecRef {
        DataAppmeshVirtualService_SpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualService_SpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> terrars::ListRef<DataAppmeshVirtualService_Spec_ProviderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provider", self.base))
    }
}
