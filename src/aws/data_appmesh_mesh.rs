#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAppmeshMeshData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mesh_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataAppmeshMesh_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAppmeshMeshData_>,
}

#[derive(Clone)]
pub struct DataAppmeshMesh(std::rc::Rc<DataAppmeshMesh_>);

impl DataAppmeshMesh {
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
    pub fn spec(&self) -> terrars::ListRef<DataAppmeshMesh_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataAppmeshMesh {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAppmeshMesh { }

impl terrars::ToListMappable for DataAppmeshMesh {
    type O = terrars::ListRef<DataAppmeshMeshRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAppmeshMesh_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_appmesh_mesh".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAppmeshMesh {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataAppmeshMesh {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAppmeshMesh {
        let out = DataAppmeshMesh(std::rc::Rc::new(DataAppmeshMesh_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAppmeshMeshData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
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

pub struct DataAppmeshMeshRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshMeshRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAppmeshMeshRef {
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
    pub fn spec(&self) -> terrars::ListRef<DataAppmeshMesh_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshMesh_Spec_EgressFilter {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshMesh_Spec_EgressFilter {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshMesh_Spec_EgressFilter {
    type O = terrars::BlockAssignable<DataAppmeshMesh_Spec_EgressFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshMesh_Spec_EgressFilter {}

impl BuildDataAppmeshMesh_Spec_EgressFilter {
    pub fn build(self) -> DataAppmeshMesh_Spec_EgressFilter {
        DataAppmeshMesh_Spec_EgressFilter { type_: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshMesh_Spec_EgressFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshMesh_Spec_EgressFilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshMesh_Spec_EgressFilterRef {
        DataAppmeshMesh_Spec_EgressFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshMesh_Spec_EgressFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshMesh_Spec_ServiceDiscovery {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_preference: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshMesh_Spec_ServiceDiscovery {
    #[doc = "Set the field `ip_preference`.\n"]
    pub fn set_ip_preference(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_preference = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshMesh_Spec_ServiceDiscovery {
    type O = terrars::BlockAssignable<DataAppmeshMesh_Spec_ServiceDiscovery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshMesh_Spec_ServiceDiscovery {}

impl BuildDataAppmeshMesh_Spec_ServiceDiscovery {
    pub fn build(self) -> DataAppmeshMesh_Spec_ServiceDiscovery {
        DataAppmeshMesh_Spec_ServiceDiscovery { ip_preference: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshMesh_Spec_ServiceDiscoveryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshMesh_Spec_ServiceDiscoveryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshMesh_Spec_ServiceDiscoveryRef {
        DataAppmeshMesh_Spec_ServiceDiscoveryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshMesh_Spec_ServiceDiscoveryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_preference` after provisioning.\n"]
    pub fn ip_preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_preference", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshMesh_Spec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_filter: std::option::Option<terrars::ListField<DataAppmeshMesh_Spec_EgressFilter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_discovery: std::option::Option<terrars::ListField<DataAppmeshMesh_Spec_ServiceDiscovery>>,
}

impl DataAppmeshMesh_Spec {
    #[doc = "Set the field `egress_filter`.\n"]
    pub fn set_egress_filter(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshMesh_Spec_EgressFilter>>,
    ) -> Self {
        self.egress_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_discovery`.\n"]
    pub fn set_service_discovery(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshMesh_Spec_ServiceDiscovery>>,
    ) -> Self {
        self.service_discovery = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshMesh_Spec {
    type O = terrars::BlockAssignable<DataAppmeshMesh_Spec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshMesh_Spec {}

impl BuildDataAppmeshMesh_Spec {
    pub fn build(self) -> DataAppmeshMesh_Spec {
        DataAppmeshMesh_Spec {
            egress_filter: core::default::Default::default(),
            service_discovery: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshMesh_SpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshMesh_SpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshMesh_SpecRef {
        DataAppmeshMesh_SpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshMesh_SpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `egress_filter` after provisioning.\n"]
    pub fn egress_filter(&self) -> terrars::ListRef<DataAppmeshMesh_Spec_EgressFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_filter", self.base))
    }

    #[doc = "Get a reference to the value of field `service_discovery` after provisioning.\n"]
    pub fn service_discovery(&self) -> terrars::ListRef<DataAppmeshMesh_Spec_ServiceDiscoveryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_discovery", self.base))
    }
}
