#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppmeshGatewayRouteData_ {
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
    virtual_gateway_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec>>,
    dynamic: AppmeshGatewayRouteDynamic,
}

struct AppmeshGatewayRoute_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppmeshGatewayRouteData_>,
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
    pub fn set_spec(self, v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec>>) -> Self {
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

    #[doc = "Get a reference to the value of field `virtual_gateway_name` after provisioning.\n"]
    pub fn virtual_gateway_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_gateway_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<AppmeshGatewayRoute_SpecRef> {
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
            data: std::cell::RefCell::new(AppmeshGatewayRouteData_ {
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

    #[doc = "Get a reference to the value of field `virtual_gateway_name` after provisioning.\n"]
    pub fn virtual_gateway_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_gateway_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<AppmeshGatewayRoute_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteDynamic {
    spec: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualService {
    virtual_service_name: terrars::PrimField<std::string::String>,
}

impl AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualService { }

impl terrars::ToListMappable for AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualService {
    type O = terrars::BlockAssignable<AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualService>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualService {
    #[doc = ""]
    pub virtual_service_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualService {
    pub fn build(self) -> AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualService {
        AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualService {
            virtual_service_name: self.virtual_service_name,
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualServiceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualServiceRef {
        AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualServiceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualServiceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_service_name` after provisioning.\n"]
    pub fn virtual_service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_service_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRoute_Spec_GrpcRoute_Action_TargetDynamic {
    virtual_service: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualService>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_service: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualService>>,
    dynamic: AppmeshGatewayRoute_Spec_GrpcRoute_Action_TargetDynamic,
}

impl AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target {
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
                    ::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualService>>,
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

impl terrars::ToListMappable for AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target {
    type O = terrars::BlockAssignable<AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target {}

impl BuildAppmeshGatewayRoute_Spec_GrpcRoute_Action_Target {
    pub fn build(self) -> AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target {
        AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target {
            port: core::default::Default::default(),
            virtual_service: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshGatewayRoute_Spec_GrpcRoute_Action_TargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRoute_Spec_GrpcRoute_Action_TargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRoute_Spec_GrpcRoute_Action_TargetRef {
        AppmeshGatewayRoute_Spec_GrpcRoute_Action_TargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRoute_Spec_GrpcRoute_Action_TargetRef {
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
    ) -> terrars::ListRef<AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target_VirtualServiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_service", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRoute_Spec_GrpcRoute_ActionDynamic {
    target: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRoute_Spec_GrpcRoute_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target>>,
    dynamic: AppmeshGatewayRoute_Spec_GrpcRoute_ActionDynamic,
}

impl AppmeshGatewayRoute_Spec_GrpcRoute_Action {
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target>>,
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

impl terrars::ToListMappable for AppmeshGatewayRoute_Spec_GrpcRoute_Action {
    type O = terrars::BlockAssignable<AppmeshGatewayRoute_Spec_GrpcRoute_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRoute_Spec_GrpcRoute_Action {}

impl BuildAppmeshGatewayRoute_Spec_GrpcRoute_Action {
    pub fn build(self) -> AppmeshGatewayRoute_Spec_GrpcRoute_Action {
        AppmeshGatewayRoute_Spec_GrpcRoute_Action {
            target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshGatewayRoute_Spec_GrpcRoute_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRoute_Spec_GrpcRoute_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshGatewayRoute_Spec_GrpcRoute_ActionRef {
        AppmeshGatewayRoute_Spec_GrpcRoute_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRoute_Spec_GrpcRoute_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::ListRef<AppmeshGatewayRoute_Spec_GrpcRoute_Action_TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRoute_Spec_GrpcRoute_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    service_name: terrars::PrimField<std::string::String>,
}

impl AppmeshGatewayRoute_Spec_GrpcRoute_Match {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshGatewayRoute_Spec_GrpcRoute_Match {
    type O = terrars::BlockAssignable<AppmeshGatewayRoute_Spec_GrpcRoute_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRoute_Spec_GrpcRoute_Match {
    #[doc = ""]
    pub service_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshGatewayRoute_Spec_GrpcRoute_Match {
    pub fn build(self) -> AppmeshGatewayRoute_Spec_GrpcRoute_Match {
        AppmeshGatewayRoute_Spec_GrpcRoute_Match {
            port: core::default::Default::default(),
            service_name: self.service_name,
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshGatewayRoute_Spec_GrpcRoute_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRoute_Spec_GrpcRoute_MatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshGatewayRoute_Spec_GrpcRoute_MatchRef {
        AppmeshGatewayRoute_Spec_GrpcRoute_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRoute_Spec_GrpcRoute_MatchRef {
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

#[derive(Serialize, Default)]
struct AppmeshGatewayRoute_Spec_GrpcRouteDynamic {
    action: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_GrpcRoute_Action>>,
    match_: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_GrpcRoute_Match>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRoute_Spec_GrpcRoute {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_GrpcRoute_Action>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_GrpcRoute_Match>>,
    dynamic: AppmeshGatewayRoute_Spec_GrpcRouteDynamic,
}

impl AppmeshGatewayRoute_Spec_GrpcRoute {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_GrpcRoute_Action>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_GrpcRoute_Match>>,
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

impl terrars::ToListMappable for AppmeshGatewayRoute_Spec_GrpcRoute {
    type O = terrars::BlockAssignable<AppmeshGatewayRoute_Spec_GrpcRoute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRoute_Spec_GrpcRoute {}

impl BuildAppmeshGatewayRoute_Spec_GrpcRoute {
    pub fn build(self) -> AppmeshGatewayRoute_Spec_GrpcRoute {
        AppmeshGatewayRoute_Spec_GrpcRoute {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshGatewayRoute_Spec_GrpcRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRoute_Spec_GrpcRouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshGatewayRoute_Spec_GrpcRouteRef {
        AppmeshGatewayRoute_Spec_GrpcRouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRoute_Spec_GrpcRouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<AppmeshGatewayRoute_Spec_GrpcRoute_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<AppmeshGatewayRoute_Spec_GrpcRoute_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Hostname {
    default_target_hostname: terrars::PrimField<std::string::String>,
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Hostname { }

impl terrars::ToListMappable for AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Hostname {
    type O = terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Hostname>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Hostname {
    #[doc = ""]
    pub default_target_hostname: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Hostname {
    pub fn build(self) -> AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Hostname {
        AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Hostname {
            default_target_hostname: self.default_target_hostname,
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_HostnameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_HostnameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_HostnameRef {
        AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_HostnameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_HostnameRef {
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
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Path {
    exact: terrars::PrimField<std::string::String>,
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Path { }

impl terrars::ToListMappable for AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Path {
    type O = terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Path>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Path {
    #[doc = ""]
    pub exact: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Path {
    pub fn build(self) -> AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Path {
        AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Path { exact: self.exact }
    }
}

#[allow(dead_code)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PathRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PathRef {
        AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PathRef {
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
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Prefix {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Prefix {
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

impl terrars::ToListMappable for AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Prefix {
    type O = terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Prefix>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Prefix {}

impl BuildAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Prefix {
    pub fn build(self) -> AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Prefix {
        AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Prefix {
            default_prefix: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PrefixRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PrefixRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PrefixRef {
        AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PrefixRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PrefixRef {
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

#[derive(Serialize, Default)]
struct AppmeshGatewayRoute_Spec_HttpRoute_Action_RewriteDynamic {
    hostname: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Hostname>>,
    path: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Path>>,
    prefix: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Prefix>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Hostname>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Path>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Prefix>>,
    dynamic: AppmeshGatewayRoute_Spec_HttpRoute_Action_RewriteDynamic,
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite {
    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Hostname>>,
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

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Path>>,
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

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_Prefix>>,
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

impl terrars::ToListMappable for AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite {
    type O = terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite {}

impl BuildAppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite {
    pub fn build(self) -> AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite {
        AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite {
            hostname: core::default::Default::default(),
            path: core::default::Default::default(),
            prefix: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Action_RewriteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRoute_Spec_HttpRoute_Action_RewriteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRoute_Spec_HttpRoute_Action_RewriteRef {
        AppmeshGatewayRoute_Spec_HttpRoute_Action_RewriteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Action_RewriteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::ListRef<AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_HostnameRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::ListRef<AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PathRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::ListRef<AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite_PrefixRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRoute_Spec_HttpRoute_ActionDynamic {
    rewrite: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite>>,
    target: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rewrite: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target>>,
    dynamic: AppmeshGatewayRoute_Spec_HttpRoute_ActionDynamic,
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Action {
    #[doc = "Set the field `rewrite`.\n"]
    pub fn set_rewrite(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Action_Rewrite>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_GrpcRoute_Action_Target>>,
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

impl terrars::ToListMappable for AppmeshGatewayRoute_Spec_HttpRoute_Action {
    type O = terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRoute_Spec_HttpRoute_Action {}

impl BuildAppmeshGatewayRoute_Spec_HttpRoute_Action {
    pub fn build(self) -> AppmeshGatewayRoute_Spec_HttpRoute_Action {
        AppmeshGatewayRoute_Spec_HttpRoute_Action {
            rewrite: core::default::Default::default(),
            target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRoute_Spec_HttpRoute_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshGatewayRoute_Spec_HttpRoute_ActionRef {
        AppmeshGatewayRoute_Spec_HttpRoute_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRoute_Spec_HttpRoute_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rewrite` after provisioning.\n"]
    pub fn rewrite(&self) -> terrars::ListRef<AppmeshGatewayRoute_Spec_HttpRoute_Action_RewriteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rewrite", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::ListRef<AppmeshGatewayRoute_Spec_GrpcRoute_Action_TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_Range {
    end: terrars::PrimField<f64>,
    start: terrars::PrimField<f64>,
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_Range { }

impl terrars::ToListMappable for AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_Range {
    type O = terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_Range>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_Range {
    #[doc = ""]
    pub end: terrars::PrimField<f64>,
    #[doc = ""]
    pub start: terrars::PrimField<f64>,
}

impl BuildAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_Range {
    pub fn build(self) -> AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_Range {
        AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_Range {
            end: self.end,
            start: self.start,
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_RangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_RangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_RangeRef {
        AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_RangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_RangeRef {
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
struct AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_MatchDynamic {
    range: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_Range>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_Range>>,
    dynamic: AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_MatchDynamic,
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match {
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
                    ::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_Range>>,
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

impl terrars::ToListMappable for AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match {
    type O = terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match {}

impl BuildAppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match {
    pub fn build(self) -> AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match {
        AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match {
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
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_MatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_MatchRef {
        AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_MatchRef {
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
    pub fn range(&self) -> terrars::ListRef<AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match_RangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRoute_Spec_HttpRoute_Match_HeaderDynamic {
    match_: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Match_Header {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match>>,
    dynamic: AppmeshGatewayRoute_Spec_HttpRoute_Match_HeaderDynamic,
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Match_Header {
    #[doc = "Set the field `invert`.\n"]
    pub fn set_invert(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_Match>>,
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

impl terrars::ToListMappable for AppmeshGatewayRoute_Spec_HttpRoute_Match_Header {
    type O = terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Match_Header>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRoute_Spec_HttpRoute_Match_Header {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshGatewayRoute_Spec_HttpRoute_Match_Header {
    pub fn build(self) -> AppmeshGatewayRoute_Spec_HttpRoute_Match_Header {
        AppmeshGatewayRoute_Spec_HttpRoute_Match_Header {
            invert: core::default::Default::default(),
            name: self.name,
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Match_HeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRoute_Spec_HttpRoute_Match_HeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRoute_Spec_HttpRoute_Match_HeaderRef {
        AppmeshGatewayRoute_Spec_HttpRoute_Match_HeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Match_HeaderRef {
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
    pub fn match_(&self) -> terrars::ListRef<AppmeshGatewayRoute_Spec_HttpRoute_Match_Header_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Match_Hostname {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Match_Hostname {
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

impl terrars::ToListMappable for AppmeshGatewayRoute_Spec_HttpRoute_Match_Hostname {
    type O = terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Match_Hostname>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRoute_Spec_HttpRoute_Match_Hostname {}

impl BuildAppmeshGatewayRoute_Spec_HttpRoute_Match_Hostname {
    pub fn build(self) -> AppmeshGatewayRoute_Spec_HttpRoute_Match_Hostname {
        AppmeshGatewayRoute_Spec_HttpRoute_Match_Hostname {
            exact: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Match_HostnameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRoute_Spec_HttpRoute_Match_HostnameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRoute_Spec_HttpRoute_Match_HostnameRef {
        AppmeshGatewayRoute_Spec_HttpRoute_Match_HostnameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Match_HostnameRef {
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
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Match_Path {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Match_Path {
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

impl terrars::ToListMappable for AppmeshGatewayRoute_Spec_HttpRoute_Match_Path {
    type O = terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Match_Path>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRoute_Spec_HttpRoute_Match_Path {}

impl BuildAppmeshGatewayRoute_Spec_HttpRoute_Match_Path {
    pub fn build(self) -> AppmeshGatewayRoute_Spec_HttpRoute_Match_Path {
        AppmeshGatewayRoute_Spec_HttpRoute_Match_Path {
            exact: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Match_PathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRoute_Spec_HttpRoute_Match_PathRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRoute_Spec_HttpRoute_Match_PathRef {
        AppmeshGatewayRoute_Spec_HttpRoute_Match_PathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Match_PathRef {
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
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_Match {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_Match {
    type O = terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_Match {}

impl BuildAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_Match {
    pub fn build(self) -> AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_Match {
        AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_Match { exact: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef {
        AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef {
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
struct AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameterDynamic {
    match_: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_Match>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter {
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_Match>>,
    dynamic: AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameterDynamic,
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter {
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_Match>>,
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

impl terrars::ToListMappable for AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter {
    type O = terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter {
    pub fn build(self) -> AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter {
        AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter {
            name: self.name,
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameterRef {
        AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRoute_Spec_HttpRoute_MatchDynamic {
    header: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_HttpRoute_Match_Header>>,
    hostname: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_HttpRoute_Match_Hostname>>,
    path: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_HttpRoute_Match_Path>>,
    query_parameter: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_HttpRoute_Match_Header>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_HttpRoute_Match_Hostname>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_HttpRoute_Match_Path>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_parameter: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter>>,
    dynamic: AppmeshGatewayRoute_Spec_HttpRoute_MatchDynamic,
}

impl AppmeshGatewayRoute_Spec_HttpRoute_Match {
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

    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Match_Header>>,
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

    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Match_Hostname>>,
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

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Match_Path>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Match_QueryParameter>>,
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

impl terrars::ToListMappable for AppmeshGatewayRoute_Spec_HttpRoute_Match {
    type O = terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRoute_Spec_HttpRoute_Match {}

impl BuildAppmeshGatewayRoute_Spec_HttpRoute_Match {
    pub fn build(self) -> AppmeshGatewayRoute_Spec_HttpRoute_Match {
        AppmeshGatewayRoute_Spec_HttpRoute_Match {
            port: core::default::Default::default(),
            prefix: core::default::Default::default(),
            header: core::default::Default::default(),
            hostname: core::default::Default::default(),
            path: core::default::Default::default(),
            query_parameter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRoute_Spec_HttpRoute_MatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshGatewayRoute_Spec_HttpRoute_MatchRef {
        AppmeshGatewayRoute_Spec_HttpRoute_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRoute_Spec_HttpRoute_MatchRef {
    #[allow(dead_code)]
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
    pub fn hostname(&self) -> terrars::ListRef<AppmeshGatewayRoute_Spec_HttpRoute_Match_HostnameRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::ListRef<AppmeshGatewayRoute_Spec_HttpRoute_Match_PathRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRoute_Spec_Http2RouteDynamic {
    action: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_HttpRoute_Action>>,
    match_: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_HttpRoute_Match>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRoute_Spec_Http2Route {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_HttpRoute_Action>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_HttpRoute_Match>>,
    dynamic: AppmeshGatewayRoute_Spec_Http2RouteDynamic,
}

impl AppmeshGatewayRoute_Spec_Http2Route {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Action>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Match>>,
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

impl terrars::ToListMappable for AppmeshGatewayRoute_Spec_Http2Route {
    type O = terrars::BlockAssignable<AppmeshGatewayRoute_Spec_Http2Route>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRoute_Spec_Http2Route {}

impl BuildAppmeshGatewayRoute_Spec_Http2Route {
    pub fn build(self) -> AppmeshGatewayRoute_Spec_Http2Route {
        AppmeshGatewayRoute_Spec_Http2Route {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshGatewayRoute_Spec_Http2RouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRoute_Spec_Http2RouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshGatewayRoute_Spec_Http2RouteRef {
        AppmeshGatewayRoute_Spec_Http2RouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRoute_Spec_Http2RouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<AppmeshGatewayRoute_Spec_HttpRoute_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<AppmeshGatewayRoute_Spec_HttpRoute_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRoute_Spec_HttpRouteDynamic {
    action: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_HttpRoute_Action>>,
    match_: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_HttpRoute_Match>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRoute_Spec_HttpRoute {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_HttpRoute_Action>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_HttpRoute_Match>>,
    dynamic: AppmeshGatewayRoute_Spec_HttpRouteDynamic,
}

impl AppmeshGatewayRoute_Spec_HttpRoute {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Action>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute_Match>>,
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

impl terrars::ToListMappable for AppmeshGatewayRoute_Spec_HttpRoute {
    type O = terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRoute_Spec_HttpRoute {}

impl BuildAppmeshGatewayRoute_Spec_HttpRoute {
    pub fn build(self) -> AppmeshGatewayRoute_Spec_HttpRoute {
        AppmeshGatewayRoute_Spec_HttpRoute {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshGatewayRoute_Spec_HttpRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRoute_Spec_HttpRouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshGatewayRoute_Spec_HttpRouteRef {
        AppmeshGatewayRoute_Spec_HttpRouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRoute_Spec_HttpRouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<AppmeshGatewayRoute_Spec_HttpRoute_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<AppmeshGatewayRoute_Spec_HttpRoute_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRoute_SpecDynamic {
    grpc_route: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_GrpcRoute>>,
    http2_route: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_Http2Route>>,
    http_route: std::option::Option<terrars::DynamicBlock<AppmeshGatewayRoute_Spec_HttpRoute>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRoute_Spec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc_route: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_GrpcRoute>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http2_route: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_Http2Route>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_route: std::option::Option<std::vec::Vec<AppmeshGatewayRoute_Spec_HttpRoute>>,
    dynamic: AppmeshGatewayRoute_SpecDynamic,
}

impl AppmeshGatewayRoute_Spec {
    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grpc_route`.\n"]
    pub fn set_grpc_route(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_GrpcRoute>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_Http2Route>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshGatewayRoute_Spec_HttpRoute>>,
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

impl terrars::ToListMappable for AppmeshGatewayRoute_Spec {
    type O = terrars::BlockAssignable<AppmeshGatewayRoute_Spec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRoute_Spec {}

impl BuildAppmeshGatewayRoute_Spec {
    pub fn build(self) -> AppmeshGatewayRoute_Spec {
        AppmeshGatewayRoute_Spec {
            priority: core::default::Default::default(),
            grpc_route: core::default::Default::default(),
            http2_route: core::default::Default::default(),
            http_route: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshGatewayRoute_SpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshGatewayRoute_SpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshGatewayRoute_SpecRef {
        AppmeshGatewayRoute_SpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRoute_SpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `grpc_route` after provisioning.\n"]
    pub fn grpc_route(&self) -> terrars::ListRef<AppmeshGatewayRoute_Spec_GrpcRouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc_route", self.base))
    }

    #[doc = "Get a reference to the value of field `http2_route` after provisioning.\n"]
    pub fn http2_route(&self) -> terrars::ListRef<AppmeshGatewayRoute_Spec_Http2RouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http2_route", self.base))
    }

    #[doc = "Get a reference to the value of field `http_route` after provisioning.\n"]
    pub fn http_route(&self) -> terrars::ListRef<AppmeshGatewayRoute_Spec_HttpRouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_route", self.base))
    }
}
