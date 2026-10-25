#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAppmeshGatewayRouteData_ {
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
    virtual_gateway_name: terrars::PrimField<std::string::String>,
}

struct DataAppmeshGatewayRoute_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAppmeshGatewayRouteData_>,
}

#[derive(Clone)]
pub struct DataAppmeshGatewayRoute(std::rc::Rc<DataAppmeshGatewayRoute_>);

impl DataAppmeshGatewayRoute {
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
    pub fn spec(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_gateway_name` after provisioning.\n"]
    pub fn virtual_gateway_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_gateway_name", self.extract_ref()))
    }
}

impl terrars::Referable for DataAppmeshGatewayRoute {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAppmeshGatewayRoute { }

impl terrars::ToListMappable for DataAppmeshGatewayRoute {
    type O = terrars::ListRef<DataAppmeshGatewayRouteRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAppmeshGatewayRoute_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_appmesh_gateway_route".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAppmeshGatewayRoute {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub mesh_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub virtual_gateway_name: terrars::PrimField<std::string::String>,
}

impl BuildDataAppmeshGatewayRoute {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAppmeshGatewayRoute {
        let out = DataAppmeshGatewayRoute(std::rc::Rc::new(DataAppmeshGatewayRoute_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAppmeshGatewayRouteData_ {
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
                virtual_gateway_name: self.virtual_gateway_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAppmeshGatewayRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAppmeshGatewayRouteRef {
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
    pub fn spec(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_gateway_name` after provisioning.\n"]
    pub fn virtual_gateway_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_gateway_name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualService {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_service_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualService {
    #[doc = "Set the field `virtual_service_name`.\n"]
    pub fn set_virtual_service_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.virtual_service_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualService {
    type O = terrars::BlockAssignable<DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualService>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualService {}

impl BuildDataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualService {
    pub fn build(self) -> DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualService {
        DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualService {
            virtual_service_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualServiceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualServiceRef {
        DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualServiceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualServiceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_service_name` after provisioning.\n"]
    pub fn virtual_service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_service_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_service: std
    ::option
    ::Option<terrars::ListField<DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualService>>,
}

impl DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_service`.\n"]
    pub fn set_virtual_service(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualService>>,
    ) -> Self {
        self.virtual_service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target {
    type O = terrars::BlockAssignable<DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target {}

impl BuildDataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target {
    pub fn build(self) -> DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target {
        DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target {
            port: core::default::Default::default(),
            virtual_service: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_TargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_TargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_TargetRef {
        DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_TargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_TargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_service` after provisioning.\n"]
    pub fn virtual_service(
        &self,
    ) -> terrars::ListRef<DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualServiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_service", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshGatewayRoute_Spec_GrpcRoute_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::ListField<DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target>>,
}

impl DataAppmeshGatewayRoute_Spec_GrpcRoute_Action {
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target>>,
    ) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshGatewayRoute_Spec_GrpcRoute_Action {
    type O = terrars::BlockAssignable<DataAppmeshGatewayRoute_Spec_GrpcRoute_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshGatewayRoute_Spec_GrpcRoute_Action {}

impl BuildDataAppmeshGatewayRoute_Spec_GrpcRoute_Action {
    pub fn build(self) -> DataAppmeshGatewayRoute_Spec_GrpcRoute_Action {
        DataAppmeshGatewayRoute_Spec_GrpcRoute_Action { target: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshGatewayRoute_Spec_GrpcRoute_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRoute_Spec_GrpcRoute_ActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshGatewayRoute_Spec_GrpcRoute_ActionRef {
        DataAppmeshGatewayRoute_Spec_GrpcRoute_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshGatewayRoute_Spec_GrpcRoute_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshGatewayRoute_Spec_GrpcRoute_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshGatewayRoute_Spec_GrpcRoute_Match {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_name`.\n"]
    pub fn set_service_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshGatewayRoute_Spec_GrpcRoute_Match {
    type O = terrars::BlockAssignable<DataAppmeshGatewayRoute_Spec_GrpcRoute_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshGatewayRoute_Spec_GrpcRoute_Match {}

impl BuildDataAppmeshGatewayRoute_Spec_GrpcRoute_Match {
    pub fn build(self) -> DataAppmeshGatewayRoute_Spec_GrpcRoute_Match {
        DataAppmeshGatewayRoute_Spec_GrpcRoute_Match {
            port: core::default::Default::default(),
            service_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshGatewayRoute_Spec_GrpcRoute_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRoute_Spec_GrpcRoute_MatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshGatewayRoute_Spec_GrpcRoute_MatchRef {
        DataAppmeshGatewayRoute_Spec_GrpcRoute_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshGatewayRoute_Spec_GrpcRoute_MatchRef {
    #[allow(dead_code)]
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

#[derive(Serialize)]
pub struct DataAppmeshGatewayRoute_Spec_GrpcRoute {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::ListField<DataAppmeshGatewayRoute_Spec_GrpcRoute_Action>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::ListField<DataAppmeshGatewayRoute_Spec_GrpcRoute_Match>>,
}

impl DataAppmeshGatewayRoute_Spec_GrpcRoute {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshGatewayRoute_Spec_GrpcRoute_Action>>,
    ) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshGatewayRoute_Spec_GrpcRoute_Match>>,
    ) -> Self {
        self.match_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshGatewayRoute_Spec_GrpcRoute {
    type O = terrars::BlockAssignable<DataAppmeshGatewayRoute_Spec_GrpcRoute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshGatewayRoute_Spec_GrpcRoute {}

impl BuildDataAppmeshGatewayRoute_Spec_GrpcRoute {
    pub fn build(self) -> DataAppmeshGatewayRoute_Spec_GrpcRoute {
        DataAppmeshGatewayRoute_Spec_GrpcRoute {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshGatewayRoute_Spec_GrpcRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRoute_Spec_GrpcRouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshGatewayRoute_Spec_GrpcRouteRef {
        DataAppmeshGatewayRoute_Spec_GrpcRouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshGatewayRoute_Spec_GrpcRouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_Spec_GrpcRoute_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_Spec_GrpcRoute_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Hostname {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_target_hostname: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Hostname {
    #[doc = "Set the field `default_target_hostname`.\n"]
    pub fn set_default_target_hostname(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_target_hostname = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Hostname {
    type O = terrars::BlockAssignable<DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Hostname>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Hostname {}

impl BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Hostname {
    pub fn build(self) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Hostname {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Hostname {
            default_target_hostname: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_HostnameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_HostnameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_HostnameRef {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_HostnameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_HostnameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_target_hostname` after provisioning.\n"]
    pub fn default_target_hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_target_hostname", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Path {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Path {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Path {
    type O = terrars::BlockAssignable<DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Path>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Path {}

impl BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Path {
    pub fn build(self) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Path {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Path { exact: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PathRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PathRef {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PathRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Prefix {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Prefix {
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

impl terrars::ToListMappable for DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Prefix {
    type O = terrars::BlockAssignable<DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Prefix>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Prefix {}

impl BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Prefix {
    pub fn build(self) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Prefix {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Prefix {
            default_prefix: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PrefixRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PrefixRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PrefixRef {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PrefixRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PrefixRef {
    #[allow(dead_code)]
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

#[derive(Serialize)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Hostname>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Path>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Prefix>>,
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite {
    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Hostname>>,
    ) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Path>>,
    ) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Prefix>>,
    ) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite {
    type O = terrars::BlockAssignable<DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite {}

impl BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite {
    pub fn build(self) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite {
            hostname: core::default::Default::default(),
            path: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Action_RewriteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRoute_Spec_HttpRoute_Action_RewriteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Action_RewriteRef {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Action_RewriteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Action_RewriteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_HostnameRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PathRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PrefixRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rewrite: std::option::Option<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::ListField<DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target>>,
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Action {
    #[doc = "Set the field `rewrite`.\n"]
    pub fn set_rewrite(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite>>,
    ) -> Self {
        self.rewrite = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target>>,
    ) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshGatewayRoute_Spec_HttpRoute_Action {
    type O = terrars::BlockAssignable<DataAppmeshGatewayRoute_Spec_HttpRoute_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Action {}

impl BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Action {
    pub fn build(self) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Action {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Action {
            rewrite: core::default::Default::default(),
            target: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRoute_Spec_HttpRoute_ActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshGatewayRoute_Spec_HttpRoute_ActionRef {
        DataAppmeshGatewayRoute_Spec_HttpRoute_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rewrite` after provisioning.\n"]
    pub fn rewrite(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_Spec_HttpRoute_Action_RewriteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rewrite", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_Spec_GrpcRoute_Action_TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_Range {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_Range {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_Range {
    type O = terrars::BlockAssignable<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_Range>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_Range {}

impl BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_Range {
    pub fn build(self) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_Range {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_Range {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_RangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_RangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_RangeRef {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_RangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_RangeRef {
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

#[derive(Serialize)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range: std::option::Option<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_Range>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match {
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

    #[doc = "Set the field `range`.\n"]
    pub fn set_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_Range>>,
    ) -> Self {
        self.range = std::option::Option::Some(v.into());
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
}

impl terrars::ToListMappable for DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match {
    type O = terrars::BlockAssignable<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match {}

impl BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match {
    pub fn build(self) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            range: core::default::Default::default(),
            regex: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_MatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_MatchRef {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_MatchRef {
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

    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_RangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc = "Get a reference to the value of field `suffix` after provisioning.\n"]
    pub fn suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header {
    #[doc = "Set the field `invert`.\n"]
    pub fn set_invert(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match>>,
    ) -> Self {
        self.match_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header {
    type O = terrars::BlockAssignable<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header {}

impl BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header {
    pub fn build(self) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header {
            invert: core::default::Default::default(),
            match_: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Match_HeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRoute_Spec_HttpRoute_Match_HeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Match_HeaderRef {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Match_HeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Match_HeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `invert` after provisioning.\n"]
    pub fn invert(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Hostname {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Hostname {
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

impl terrars::ToListMappable for DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Hostname {
    type O = terrars::BlockAssignable<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Hostname>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Match_Hostname {}

impl BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Match_Hostname {
    pub fn build(self) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Hostname {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Hostname {
            exact: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Match_HostnameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRoute_Spec_HttpRoute_Match_HostnameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Match_HostnameRef {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Match_HostnameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Match_HostnameRef {
    #[allow(dead_code)]
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

#[derive(Serialize)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Path {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Path {
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

impl terrars::ToListMappable for DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Path {
    type O = terrars::BlockAssignable<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Path>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Match_Path {}

impl BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Match_Path {
    pub fn build(self) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Path {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Path {
            exact: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Match_PathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRoute_Spec_HttpRoute_Match_PathRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Match_PathRef {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Match_PathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Match_PathRef {
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
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_Match {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_Match {
    type O = terrars::BlockAssignable<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_Match {}

impl BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_Match {
    pub fn build(self) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_Match {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_Match {
            exact: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter {
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter {
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_Match>>,
    ) -> Self {
        self.match_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter {
    type O = terrars::BlockAssignable<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter {}

impl BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter {
    pub fn build(self) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter {
            match_: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameterRef {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<terrars::SetField<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Hostname>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Path>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_parameter: std::option::Option<terrars::SetField<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter>>,
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_Match {
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Header>>,
    ) -> Self {
        self.header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Hostname>>,
    ) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_Path>>,
    ) -> Self {
        self.path = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `query_parameter`.\n"]
    pub fn set_query_parameter(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter>>,
    ) -> Self {
        self.query_parameter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshGatewayRoute_Spec_HttpRoute_Match {
    type O = terrars::BlockAssignable<DataAppmeshGatewayRoute_Spec_HttpRoute_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Match {}

impl BuildDataAppmeshGatewayRoute_Spec_HttpRoute_Match {
    pub fn build(self) -> DataAppmeshGatewayRoute_Spec_HttpRoute_Match {
        DataAppmeshGatewayRoute_Spec_HttpRoute_Match {
            header: core::default::Default::default(),
            hostname: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            prefix: core::default::Default::default(),
            query_parameter: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRoute_Spec_HttpRoute_MatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshGatewayRoute_Spec_HttpRoute_MatchRef {
        DataAppmeshGatewayRoute_Spec_HttpRoute_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::SetRef<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_HeaderRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_HostnameRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_PathRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `query_parameter` after provisioning.\n"]
    pub fn query_parameter(&self) -> terrars::SetRef<DataAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameterRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.query_parameter", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshGatewayRoute_Spec_Http2Route {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Action>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Match>>,
}

impl DataAppmeshGatewayRoute_Spec_Http2Route {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Action>>,
    ) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Match>>,
    ) -> Self {
        self.match_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshGatewayRoute_Spec_Http2Route {
    type O = terrars::BlockAssignable<DataAppmeshGatewayRoute_Spec_Http2Route>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshGatewayRoute_Spec_Http2Route {}

impl BuildDataAppmeshGatewayRoute_Spec_Http2Route {
    pub fn build(self) -> DataAppmeshGatewayRoute_Spec_Http2Route {
        DataAppmeshGatewayRoute_Spec_Http2Route {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshGatewayRoute_Spec_Http2RouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRoute_Spec_Http2RouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshGatewayRoute_Spec_Http2RouteRef {
        DataAppmeshGatewayRoute_Spec_Http2RouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshGatewayRoute_Spec_Http2RouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_Spec_HttpRoute_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_Spec_HttpRoute_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRoute {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Action>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Match>>,
}

impl DataAppmeshGatewayRoute_Spec_HttpRoute {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Action>>,
    ) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute_Match>>,
    ) -> Self {
        self.match_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshGatewayRoute_Spec_HttpRoute {
    type O = terrars::BlockAssignable<DataAppmeshGatewayRoute_Spec_HttpRoute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshGatewayRoute_Spec_HttpRoute {}

impl BuildDataAppmeshGatewayRoute_Spec_HttpRoute {
    pub fn build(self) -> DataAppmeshGatewayRoute_Spec_HttpRoute {
        DataAppmeshGatewayRoute_Spec_HttpRoute {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshGatewayRoute_Spec_HttpRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRoute_Spec_HttpRouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshGatewayRoute_Spec_HttpRouteRef {
        DataAppmeshGatewayRoute_Spec_HttpRouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshGatewayRoute_Spec_HttpRouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_Spec_HttpRoute_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_Spec_HttpRoute_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshGatewayRoute_Spec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc_route: std::option::Option<terrars::ListField<DataAppmeshGatewayRoute_Spec_GrpcRoute>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http2_route: std::option::Option<terrars::ListField<DataAppmeshGatewayRoute_Spec_Http2Route>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_route: std::option::Option<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAppmeshGatewayRoute_Spec {
    #[doc = "Set the field `grpc_route`.\n"]
    pub fn set_grpc_route(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshGatewayRoute_Spec_GrpcRoute>>,
    ) -> Self {
        self.grpc_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http2_route`.\n"]
    pub fn set_http2_route(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshGatewayRoute_Spec_Http2Route>>,
    ) -> Self {
        self.http2_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_route`.\n"]
    pub fn set_http_route(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshGatewayRoute_Spec_HttpRoute>>,
    ) -> Self {
        self.http_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshGatewayRoute_Spec {
    type O = terrars::BlockAssignable<DataAppmeshGatewayRoute_Spec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshGatewayRoute_Spec {}

impl BuildDataAppmeshGatewayRoute_Spec {
    pub fn build(self) -> DataAppmeshGatewayRoute_Spec {
        DataAppmeshGatewayRoute_Spec {
            grpc_route: core::default::Default::default(),
            http2_route: core::default::Default::default(),
            http_route: core::default::Default::default(),
            priority: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshGatewayRoute_SpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshGatewayRoute_SpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshGatewayRoute_SpecRef {
        DataAppmeshGatewayRoute_SpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshGatewayRoute_SpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grpc_route` after provisioning.\n"]
    pub fn grpc_route(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_Spec_GrpcRouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc_route", self.base))
    }

    #[doc = "Get a reference to the value of field `http2_route` after provisioning.\n"]
    pub fn http2_route(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_Spec_Http2RouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http2_route", self.base))
    }

    #[doc = "Get a reference to the value of field `http_route` after provisioning.\n"]
    pub fn http_route(&self) -> terrars::ListRef<DataAppmeshGatewayRoute_Spec_HttpRouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_route", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
}
