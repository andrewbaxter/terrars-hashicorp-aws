#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct TransferWebAppData_ {
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
    access_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_app_endpoint_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_app_units: std::option::Option<terrars::ListField<TransferWebApp_WebAppUnits>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_details: std::option::Option<std::vec::Vec<TransferWebApp_EndpointDetails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider_details: std::option::Option<std::vec::Vec<TransferWebApp_IdentityProviderDetails>>,
    dynamic: TransferWebAppDynamic,
}

struct TransferWebApp_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<TransferWebAppData_>,
}

#[derive(Clone)]
pub struct TransferWebApp(std::rc::Rc<TransferWebApp_>);

impl TransferWebApp {
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

    #[doc = "Set the field `access_endpoint`.\n"]
    pub fn set_access_endpoint(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().access_endpoint = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `web_app_endpoint_policy`.\n"]
    pub fn set_web_app_endpoint_policy(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().web_app_endpoint_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `web_app_units`.\n"]
    pub fn set_web_app_units(self, v: impl std::convert::Into<terrars::ListField<TransferWebApp_WebAppUnits>>) -> Self {
        self.0.data.borrow_mut().web_app_units = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_details`.\n"]
    pub fn set_endpoint_details(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWebApp_EndpointDetails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `identity_provider_details`.\n"]
    pub fn set_identity_provider_details(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWebApp_IdentityProviderDetails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().identity_provider_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.identity_provider_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `access_endpoint` after provisioning.\n"]
    pub fn access_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `web_app_endpoint_policy` after provisioning.\n"]
    pub fn web_app_endpoint_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_app_endpoint_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_app_id` after provisioning.\n"]
    pub fn web_app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_app_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_app_units` after provisioning.\n"]
    pub fn web_app_units(&self) -> terrars::ListRef<TransferWebApp_WebAppUnitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_app_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_details` after provisioning.\n"]
    pub fn endpoint_details(&self) -> terrars::ListRef<TransferWebApp_EndpointDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_provider_details` after provisioning.\n"]
    pub fn identity_provider_details(&self) -> terrars::ListRef<TransferWebApp_IdentityProviderDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_provider_details", self.extract_ref()))
    }
}

impl terrars::Referable for TransferWebApp {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for TransferWebApp { }

impl terrars::ToListMappable for TransferWebApp {
    type O = terrars::ListRef<TransferWebAppRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for TransferWebApp_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_transfer_web_app".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTransferWebApp {
    pub tf_id: std::string::String,
}

impl BuildTransferWebApp {
    pub fn build(self, stack: &mut terrars::Stack) -> TransferWebApp {
        let out = TransferWebApp(std::rc::Rc::new(TransferWebApp_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(TransferWebAppData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                access_endpoint: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                web_app_endpoint_policy: core::default::Default::default(),
                web_app_units: core::default::Default::default(),
                endpoint_details: core::default::Default::default(),
                identity_provider_details: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TransferWebAppRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWebAppRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TransferWebAppRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_endpoint` after provisioning.\n"]
    pub fn access_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `web_app_endpoint_policy` after provisioning.\n"]
    pub fn web_app_endpoint_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_app_endpoint_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_app_id` after provisioning.\n"]
    pub fn web_app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_app_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_app_units` after provisioning.\n"]
    pub fn web_app_units(&self) -> terrars::ListRef<TransferWebApp_WebAppUnitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_app_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_details` after provisioning.\n"]
    pub fn endpoint_details(&self) -> terrars::ListRef<TransferWebApp_EndpointDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_provider_details` after provisioning.\n"]
    pub fn identity_provider_details(&self) -> terrars::ListRef<TransferWebApp_IdentityProviderDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_provider_details", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct TransferWebAppDynamic {
    endpoint_details: std::option::Option<terrars::DynamicBlock<TransferWebApp_EndpointDetails>>,
    identity_provider_details: std::option::Option<terrars::DynamicBlock<TransferWebApp_IdentityProviderDetails>>,
}

#[derive(Serialize)]
pub struct TransferWebApp_WebAppUnits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned: std::option::Option<terrars::PrimField<f64>>,
}

impl TransferWebApp_WebAppUnits {
    #[doc = "Set the field `provisioned`.\n"]
    pub fn set_provisioned(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.provisioned = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TransferWebApp_WebAppUnits {
    type O = terrars::BlockAssignable<TransferWebApp_WebAppUnits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWebApp_WebAppUnits {}

impl BuildTransferWebApp_WebAppUnits {
    pub fn build(self) -> TransferWebApp_WebAppUnits {
        TransferWebApp_WebAppUnits { provisioned: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct TransferWebApp_WebAppUnitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWebApp_WebAppUnitsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferWebApp_WebAppUnitsRef {
        TransferWebApp_WebAppUnitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWebApp_WebAppUnitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `provisioned` after provisioning.\n"]
    pub fn provisioned(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned", self.base))
    }
}

#[derive(Serialize)]
pub struct TransferWebApp_EndpointDetails_Vpc {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    vpc_id: terrars::PrimField<std::string::String>,
}

impl TransferWebApp_EndpointDetails_Vpc {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TransferWebApp_EndpointDetails_Vpc {
    type O = terrars::BlockAssignable<TransferWebApp_EndpointDetails_Vpc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWebApp_EndpointDetails_Vpc {
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub vpc_id: terrars::PrimField<std::string::String>,
}

impl BuildTransferWebApp_EndpointDetails_Vpc {
    pub fn build(self) -> TransferWebApp_EndpointDetails_Vpc {
        TransferWebApp_EndpointDetails_Vpc {
            security_group_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
            vpc_id: self.vpc_id,
        }
    }
}

#[allow(dead_code)]
pub struct TransferWebApp_EndpointDetails_VpcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWebApp_EndpointDetails_VpcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferWebApp_EndpointDetails_VpcRef {
        TransferWebApp_EndpointDetails_VpcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWebApp_EndpointDetails_VpcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\n"]
    pub fn vpc_endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_id", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferWebApp_EndpointDetailsDynamic {
    vpc: std::option::Option<terrars::DynamicBlock<TransferWebApp_EndpointDetails_Vpc>>,
}

#[derive(Serialize)]
pub struct TransferWebApp_EndpointDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<std::vec::Vec<TransferWebApp_EndpointDetails_Vpc>>,
    dynamic: TransferWebApp_EndpointDetailsDynamic,
}

impl TransferWebApp_EndpointDetails {
    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWebApp_EndpointDetails_Vpc>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TransferWebApp_EndpointDetails {
    type O = terrars::BlockAssignable<TransferWebApp_EndpointDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWebApp_EndpointDetails {}

impl BuildTransferWebApp_EndpointDetails {
    pub fn build(self) -> TransferWebApp_EndpointDetails {
        TransferWebApp_EndpointDetails {
            vpc: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TransferWebApp_EndpointDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWebApp_EndpointDetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferWebApp_EndpointDetailsRef {
        TransferWebApp_EndpointDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWebApp_EndpointDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<TransferWebApp_EndpointDetails_VpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }
}

#[derive(Serialize)]
pub struct TransferWebApp_IdentityProviderDetails_IdentityCenterConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TransferWebApp_IdentityProviderDetails_IdentityCenterConfig {
    #[doc = "Set the field `instance_arn`.\n"]
    pub fn set_instance_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role`.\n"]
    pub fn set_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TransferWebApp_IdentityProviderDetails_IdentityCenterConfig {
    type O = terrars::BlockAssignable<TransferWebApp_IdentityProviderDetails_IdentityCenterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWebApp_IdentityProviderDetails_IdentityCenterConfig {}

impl BuildTransferWebApp_IdentityProviderDetails_IdentityCenterConfig {
    pub fn build(self) -> TransferWebApp_IdentityProviderDetails_IdentityCenterConfig {
        TransferWebApp_IdentityProviderDetails_IdentityCenterConfig {
            instance_arn: core::default::Default::default(),
            role: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TransferWebApp_IdentityProviderDetails_IdentityCenterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWebApp_IdentityProviderDetails_IdentityCenterConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TransferWebApp_IdentityProviderDetails_IdentityCenterConfigRef {
        TransferWebApp_IdentityProviderDetails_IdentityCenterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWebApp_IdentityProviderDetails_IdentityCenterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_arn` after provisioning.\n"]
    pub fn application_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferWebApp_IdentityProviderDetailsDynamic {
    identity_center_config: std
    ::option
    ::Option<terrars::DynamicBlock<TransferWebApp_IdentityProviderDetails_IdentityCenterConfig>>,
}

#[derive(Serialize)]
pub struct TransferWebApp_IdentityProviderDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_center_config: std
    ::option
    ::Option<std::vec::Vec<TransferWebApp_IdentityProviderDetails_IdentityCenterConfig>>,
    dynamic: TransferWebApp_IdentityProviderDetailsDynamic,
}

impl TransferWebApp_IdentityProviderDetails {
    #[doc = "Set the field `identity_center_config`.\n"]
    pub fn set_identity_center_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<TransferWebApp_IdentityProviderDetails_IdentityCenterConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.identity_center_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.identity_center_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TransferWebApp_IdentityProviderDetails {
    type O = terrars::BlockAssignable<TransferWebApp_IdentityProviderDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWebApp_IdentityProviderDetails {}

impl BuildTransferWebApp_IdentityProviderDetails {
    pub fn build(self) -> TransferWebApp_IdentityProviderDetails {
        TransferWebApp_IdentityProviderDetails {
            identity_center_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TransferWebApp_IdentityProviderDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWebApp_IdentityProviderDetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferWebApp_IdentityProviderDetailsRef {
        TransferWebApp_IdentityProviderDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWebApp_IdentityProviderDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identity_center_config` after provisioning.\n"]
    pub fn identity_center_config(
        &self,
    ) -> terrars::ListRef<TransferWebApp_IdentityProviderDetails_IdentityCenterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_center_config", self.base))
    }
}
