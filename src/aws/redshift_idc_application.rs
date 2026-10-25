#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RedshiftIdcApplicationData_ {
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
    application_type: std::option::Option<terrars::PrimField<std::string::String>>,
    iam_role_arn: terrars::PrimField<std::string::String>,
    idc_display_name: terrars::PrimField<std::string::String>,
    idc_instance_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    redshift_idc_application_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorized_token_issuer: std::option::Option<std::vec::Vec<RedshiftIdcApplication_AuthorizedTokenIssuer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_integration: std::option::Option<std::vec::Vec<RedshiftIdcApplication_ServiceIntegration>>,
    dynamic: RedshiftIdcApplicationDynamic,
}

struct RedshiftIdcApplication_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RedshiftIdcApplicationData_>,
}

#[derive(Clone)]
pub struct RedshiftIdcApplication(std::rc::Rc<RedshiftIdcApplication_>);

impl RedshiftIdcApplication {
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

    #[doc = "Set the field `application_type`.\n"]
    pub fn set_application_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().application_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_namespace`.\n"]
    pub fn set_identity_namespace(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().identity_namespace = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `authorized_token_issuer`.\n"]
    pub fn set_authorized_token_issuer(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RedshiftIdcApplication_AuthorizedTokenIssuer>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().authorized_token_issuer = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.authorized_token_issuer = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_integration`.\n"]
    pub fn set_service_integration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RedshiftIdcApplication_ServiceIntegration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_integration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_integration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `application_type` after provisioning.\n"]
    pub fn application_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idc_display_name` after provisioning.\n"]
    pub fn idc_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idc_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idc_instance_arn` after provisioning.\n"]
    pub fn idc_instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idc_instance_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idc_managed_application_arn` after provisioning.\n"]
    pub fn idc_managed_application_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idc_managed_application_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_namespace` after provisioning.\n"]
    pub fn identity_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redshift_idc_application_arn` after provisioning.\n"]
    pub fn redshift_idc_application_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redshift_idc_application_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redshift_idc_application_name` after provisioning.\n"]
    pub fn redshift_idc_application_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.redshift_idc_application_name", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `authorized_token_issuer` after provisioning.\n"]
    pub fn authorized_token_issuer(&self) -> terrars::ListRef<RedshiftIdcApplication_AuthorizedTokenIssuerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorized_token_issuer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_integration` after provisioning.\n"]
    pub fn service_integration(&self) -> terrars::ListRef<RedshiftIdcApplication_ServiceIntegrationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_integration", self.extract_ref()))
    }
}

impl terrars::Referable for RedshiftIdcApplication {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for RedshiftIdcApplication { }

impl terrars::ToListMappable for RedshiftIdcApplication {
    type O = terrars::ListRef<RedshiftIdcApplicationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for RedshiftIdcApplication_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_redshift_idc_application".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRedshiftIdcApplication {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub iam_role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub idc_display_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub idc_instance_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub redshift_idc_application_name: terrars::PrimField<std::string::String>,
}

impl BuildRedshiftIdcApplication {
    pub fn build(self, stack: &mut terrars::Stack) -> RedshiftIdcApplication {
        let out = RedshiftIdcApplication(std::rc::Rc::new(RedshiftIdcApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(RedshiftIdcApplicationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                application_type: core::default::Default::default(),
                iam_role_arn: self.iam_role_arn,
                idc_display_name: self.idc_display_name,
                idc_instance_arn: self.idc_instance_arn,
                identity_namespace: core::default::Default::default(),
                redshift_idc_application_name: self.redshift_idc_application_name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                authorized_token_issuer: core::default::Default::default(),
                service_integration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RedshiftIdcApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftIdcApplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RedshiftIdcApplicationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_type` after provisioning.\n"]
    pub fn application_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idc_display_name` after provisioning.\n"]
    pub fn idc_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idc_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idc_instance_arn` after provisioning.\n"]
    pub fn idc_instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idc_instance_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idc_managed_application_arn` after provisioning.\n"]
    pub fn idc_managed_application_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idc_managed_application_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_namespace` after provisioning.\n"]
    pub fn identity_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redshift_idc_application_arn` after provisioning.\n"]
    pub fn redshift_idc_application_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redshift_idc_application_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redshift_idc_application_name` after provisioning.\n"]
    pub fn redshift_idc_application_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.redshift_idc_application_name", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `authorized_token_issuer` after provisioning.\n"]
    pub fn authorized_token_issuer(&self) -> terrars::ListRef<RedshiftIdcApplication_AuthorizedTokenIssuerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorized_token_issuer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_integration` after provisioning.\n"]
    pub fn service_integration(&self) -> terrars::ListRef<RedshiftIdcApplication_ServiceIntegrationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_integration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct RedshiftIdcApplicationDynamic {
    authorized_token_issuer: std::option::Option<terrars::DynamicBlock<RedshiftIdcApplication_AuthorizedTokenIssuer>>,
    service_integration: std::option::Option<terrars::DynamicBlock<RedshiftIdcApplication_ServiceIntegration>>,
}

#[derive(Serialize)]
pub struct RedshiftIdcApplication_AuthorizedTokenIssuer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorized_audiences_list: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trusted_token_issuer_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedshiftIdcApplication_AuthorizedTokenIssuer {
    #[doc = "Set the field `authorized_audiences_list`.\n"]
    pub fn set_authorized_audiences_list(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.authorized_audiences_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trusted_token_issuer_arn`.\n"]
    pub fn set_trusted_token_issuer_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.trusted_token_issuer_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedshiftIdcApplication_AuthorizedTokenIssuer {
    type O = terrars::BlockAssignable<RedshiftIdcApplication_AuthorizedTokenIssuer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftIdcApplication_AuthorizedTokenIssuer {}

impl BuildRedshiftIdcApplication_AuthorizedTokenIssuer {
    pub fn build(self) -> RedshiftIdcApplication_AuthorizedTokenIssuer {
        RedshiftIdcApplication_AuthorizedTokenIssuer {
            authorized_audiences_list: core::default::Default::default(),
            trusted_token_issuer_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedshiftIdcApplication_AuthorizedTokenIssuerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftIdcApplication_AuthorizedTokenIssuerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedshiftIdcApplication_AuthorizedTokenIssuerRef {
        RedshiftIdcApplication_AuthorizedTokenIssuerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftIdcApplication_AuthorizedTokenIssuerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorized_audiences_list` after provisioning.\n"]
    pub fn authorized_audiences_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorized_audiences_list", self.base))
    }

    #[doc = "Get a reference to the value of field `trusted_token_issuer_arn` after provisioning.\n"]
    pub fn trusted_token_issuer_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trusted_token_issuer_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct RedshiftIdcApplication_ServiceIntegration_LakeFormation_LakeFormationQuery {
    authorization: terrars::PrimField<std::string::String>,
}

impl RedshiftIdcApplication_ServiceIntegration_LakeFormation_LakeFormationQuery { }

impl terrars::ToListMappable for RedshiftIdcApplication_ServiceIntegration_LakeFormation_LakeFormationQuery {
    type O = terrars::BlockAssignable<RedshiftIdcApplication_ServiceIntegration_LakeFormation_LakeFormationQuery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftIdcApplication_ServiceIntegration_LakeFormation_LakeFormationQuery {
    #[doc = ""]
    pub authorization: terrars::PrimField<std::string::String>,
}

impl BuildRedshiftIdcApplication_ServiceIntegration_LakeFormation_LakeFormationQuery {
    pub fn build(self) -> RedshiftIdcApplication_ServiceIntegration_LakeFormation_LakeFormationQuery {
        RedshiftIdcApplication_ServiceIntegration_LakeFormation_LakeFormationQuery {
            authorization: self.authorization,
        }
    }
}

#[allow(dead_code)]
pub struct RedshiftIdcApplication_ServiceIntegration_LakeFormation_LakeFormationQueryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftIdcApplication_ServiceIntegration_LakeFormation_LakeFormationQueryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedshiftIdcApplication_ServiceIntegration_LakeFormation_LakeFormationQueryRef {
        RedshiftIdcApplication_ServiceIntegration_LakeFormation_LakeFormationQueryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftIdcApplication_ServiceIntegration_LakeFormation_LakeFormationQueryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization", self.base))
    }
}

#[derive(Serialize, Default)]
struct RedshiftIdcApplication_ServiceIntegration_LakeFormationDynamic {
    lake_formation_query: std
    ::option
    ::Option<terrars::DynamicBlock<RedshiftIdcApplication_ServiceIntegration_LakeFormation_LakeFormationQuery>>,
}

#[derive(Serialize)]
pub struct RedshiftIdcApplication_ServiceIntegration_LakeFormation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lake_formation_query: std
    ::option
    ::Option<std::vec::Vec<RedshiftIdcApplication_ServiceIntegration_LakeFormation_LakeFormationQuery>>,
    dynamic: RedshiftIdcApplication_ServiceIntegration_LakeFormationDynamic,
}

impl RedshiftIdcApplication_ServiceIntegration_LakeFormation {
    #[doc = "Set the field `lake_formation_query`.\n"]
    pub fn set_lake_formation_query(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<RedshiftIdcApplication_ServiceIntegration_LakeFormation_LakeFormationQuery>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lake_formation_query = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lake_formation_query = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RedshiftIdcApplication_ServiceIntegration_LakeFormation {
    type O = terrars::BlockAssignable<RedshiftIdcApplication_ServiceIntegration_LakeFormation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftIdcApplication_ServiceIntegration_LakeFormation {}

impl BuildRedshiftIdcApplication_ServiceIntegration_LakeFormation {
    pub fn build(self) -> RedshiftIdcApplication_ServiceIntegration_LakeFormation {
        RedshiftIdcApplication_ServiceIntegration_LakeFormation {
            lake_formation_query: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedshiftIdcApplication_ServiceIntegration_LakeFormationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftIdcApplication_ServiceIntegration_LakeFormationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedshiftIdcApplication_ServiceIntegration_LakeFormationRef {
        RedshiftIdcApplication_ServiceIntegration_LakeFormationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftIdcApplication_ServiceIntegration_LakeFormationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lake_formation_query` after provisioning.\n"]
    pub fn lake_formation_query(
        &self,
    ) -> terrars::ListRef<RedshiftIdcApplication_ServiceIntegration_LakeFormation_LakeFormationQueryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lake_formation_query", self.base))
    }
}

#[derive(Serialize)]
pub struct RedshiftIdcApplication_ServiceIntegration_Redshift_Connect {
    authorization: terrars::PrimField<std::string::String>,
}

impl RedshiftIdcApplication_ServiceIntegration_Redshift_Connect { }

impl terrars::ToListMappable for RedshiftIdcApplication_ServiceIntegration_Redshift_Connect {
    type O = terrars::BlockAssignable<RedshiftIdcApplication_ServiceIntegration_Redshift_Connect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftIdcApplication_ServiceIntegration_Redshift_Connect {
    #[doc = ""]
    pub authorization: terrars::PrimField<std::string::String>,
}

impl BuildRedshiftIdcApplication_ServiceIntegration_Redshift_Connect {
    pub fn build(self) -> RedshiftIdcApplication_ServiceIntegration_Redshift_Connect {
        RedshiftIdcApplication_ServiceIntegration_Redshift_Connect { authorization: self.authorization }
    }
}

#[allow(dead_code)]
pub struct RedshiftIdcApplication_ServiceIntegration_Redshift_ConnectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftIdcApplication_ServiceIntegration_Redshift_ConnectRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedshiftIdcApplication_ServiceIntegration_Redshift_ConnectRef {
        RedshiftIdcApplication_ServiceIntegration_Redshift_ConnectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftIdcApplication_ServiceIntegration_Redshift_ConnectRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization", self.base))
    }
}

#[derive(Serialize, Default)]
struct RedshiftIdcApplication_ServiceIntegration_RedshiftDynamic {
    connect: std::option::Option<terrars::DynamicBlock<RedshiftIdcApplication_ServiceIntegration_Redshift_Connect>>,
}

#[derive(Serialize)]
pub struct RedshiftIdcApplication_ServiceIntegration_Redshift {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connect: std::option::Option<std::vec::Vec<RedshiftIdcApplication_ServiceIntegration_Redshift_Connect>>,
    dynamic: RedshiftIdcApplication_ServiceIntegration_RedshiftDynamic,
}

impl RedshiftIdcApplication_ServiceIntegration_Redshift {
    #[doc = "Set the field `connect`.\n"]
    pub fn set_connect(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<RedshiftIdcApplication_ServiceIntegration_Redshift_Connect>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.connect = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.connect = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RedshiftIdcApplication_ServiceIntegration_Redshift {
    type O = terrars::BlockAssignable<RedshiftIdcApplication_ServiceIntegration_Redshift>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftIdcApplication_ServiceIntegration_Redshift {}

impl BuildRedshiftIdcApplication_ServiceIntegration_Redshift {
    pub fn build(self) -> RedshiftIdcApplication_ServiceIntegration_Redshift {
        RedshiftIdcApplication_ServiceIntegration_Redshift {
            connect: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedshiftIdcApplication_ServiceIntegration_RedshiftRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftIdcApplication_ServiceIntegration_RedshiftRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedshiftIdcApplication_ServiceIntegration_RedshiftRef {
        RedshiftIdcApplication_ServiceIntegration_RedshiftRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftIdcApplication_ServiceIntegration_RedshiftRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connect` after provisioning.\n"]
    pub fn connect(&self) -> terrars::ListRef<RedshiftIdcApplication_ServiceIntegration_Redshift_ConnectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connect", self.base))
    }
}

#[derive(Serialize)]
pub struct RedshiftIdcApplication_ServiceIntegration_S3AccessGrants_ReadWriteAccess {
    authorization: terrars::PrimField<std::string::String>,
}

impl RedshiftIdcApplication_ServiceIntegration_S3AccessGrants_ReadWriteAccess { }

impl terrars::ToListMappable for RedshiftIdcApplication_ServiceIntegration_S3AccessGrants_ReadWriteAccess {
    type O = terrars::BlockAssignable<RedshiftIdcApplication_ServiceIntegration_S3AccessGrants_ReadWriteAccess>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftIdcApplication_ServiceIntegration_S3AccessGrants_ReadWriteAccess {
    #[doc = ""]
    pub authorization: terrars::PrimField<std::string::String>,
}

impl BuildRedshiftIdcApplication_ServiceIntegration_S3AccessGrants_ReadWriteAccess {
    pub fn build(self) -> RedshiftIdcApplication_ServiceIntegration_S3AccessGrants_ReadWriteAccess {
        RedshiftIdcApplication_ServiceIntegration_S3AccessGrants_ReadWriteAccess {
            authorization: self.authorization,
        }
    }
}

#[allow(dead_code)]
pub struct RedshiftIdcApplication_ServiceIntegration_S3AccessGrants_ReadWriteAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftIdcApplication_ServiceIntegration_S3AccessGrants_ReadWriteAccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedshiftIdcApplication_ServiceIntegration_S3AccessGrants_ReadWriteAccessRef {
        RedshiftIdcApplication_ServiceIntegration_S3AccessGrants_ReadWriteAccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftIdcApplication_ServiceIntegration_S3AccessGrants_ReadWriteAccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization", self.base))
    }
}

#[derive(Serialize, Default)]
struct RedshiftIdcApplication_ServiceIntegration_S3AccessGrantsDynamic {
    read_write_access: std
    ::option
    ::Option<terrars::DynamicBlock<RedshiftIdcApplication_ServiceIntegration_S3AccessGrants_ReadWriteAccess>>,
}

#[derive(Serialize)]
pub struct RedshiftIdcApplication_ServiceIntegration_S3AccessGrants {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_write_access: std
    ::option
    ::Option<std::vec::Vec<RedshiftIdcApplication_ServiceIntegration_S3AccessGrants_ReadWriteAccess>>,
    dynamic: RedshiftIdcApplication_ServiceIntegration_S3AccessGrantsDynamic,
}

impl RedshiftIdcApplication_ServiceIntegration_S3AccessGrants {
    #[doc = "Set the field `read_write_access`.\n"]
    pub fn set_read_write_access(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<RedshiftIdcApplication_ServiceIntegration_S3AccessGrants_ReadWriteAccess>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.read_write_access = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.read_write_access = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RedshiftIdcApplication_ServiceIntegration_S3AccessGrants {
    type O = terrars::BlockAssignable<RedshiftIdcApplication_ServiceIntegration_S3AccessGrants>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftIdcApplication_ServiceIntegration_S3AccessGrants {}

impl BuildRedshiftIdcApplication_ServiceIntegration_S3AccessGrants {
    pub fn build(self) -> RedshiftIdcApplication_ServiceIntegration_S3AccessGrants {
        RedshiftIdcApplication_ServiceIntegration_S3AccessGrants {
            read_write_access: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedshiftIdcApplication_ServiceIntegration_S3AccessGrantsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftIdcApplication_ServiceIntegration_S3AccessGrantsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedshiftIdcApplication_ServiceIntegration_S3AccessGrantsRef {
        RedshiftIdcApplication_ServiceIntegration_S3AccessGrantsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftIdcApplication_ServiceIntegration_S3AccessGrantsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read_write_access` after provisioning.\n"]
    pub fn read_write_access(
        &self,
    ) -> terrars::ListRef<RedshiftIdcApplication_ServiceIntegration_S3AccessGrants_ReadWriteAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.read_write_access", self.base))
    }
}

#[derive(Serialize, Default)]
struct RedshiftIdcApplication_ServiceIntegrationDynamic {
    lake_formation: std
    ::option
    ::Option<terrars::DynamicBlock<RedshiftIdcApplication_ServiceIntegration_LakeFormation>>,
    redshift: std::option::Option<terrars::DynamicBlock<RedshiftIdcApplication_ServiceIntegration_Redshift>>,
    s3_access_grants: std
    ::option
    ::Option<terrars::DynamicBlock<RedshiftIdcApplication_ServiceIntegration_S3AccessGrants>>,
}

#[derive(Serialize)]
pub struct RedshiftIdcApplication_ServiceIntegration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lake_formation: std::option::Option<std::vec::Vec<RedshiftIdcApplication_ServiceIntegration_LakeFormation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redshift: std::option::Option<std::vec::Vec<RedshiftIdcApplication_ServiceIntegration_Redshift>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_access_grants: std::option::Option<std::vec::Vec<RedshiftIdcApplication_ServiceIntegration_S3AccessGrants>>,
    dynamic: RedshiftIdcApplication_ServiceIntegrationDynamic,
}

impl RedshiftIdcApplication_ServiceIntegration {
    #[doc = "Set the field `lake_formation`.\n"]
    pub fn set_lake_formation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<RedshiftIdcApplication_ServiceIntegration_LakeFormation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lake_formation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lake_formation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redshift`.\n"]
    pub fn set_redshift(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RedshiftIdcApplication_ServiceIntegration_Redshift>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redshift = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redshift = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_access_grants`.\n"]
    pub fn set_s3_access_grants(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<RedshiftIdcApplication_ServiceIntegration_S3AccessGrants>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_access_grants = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_access_grants = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RedshiftIdcApplication_ServiceIntegration {
    type O = terrars::BlockAssignable<RedshiftIdcApplication_ServiceIntegration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftIdcApplication_ServiceIntegration {}

impl BuildRedshiftIdcApplication_ServiceIntegration {
    pub fn build(self) -> RedshiftIdcApplication_ServiceIntegration {
        RedshiftIdcApplication_ServiceIntegration {
            lake_formation: core::default::Default::default(),
            redshift: core::default::Default::default(),
            s3_access_grants: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedshiftIdcApplication_ServiceIntegrationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftIdcApplication_ServiceIntegrationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedshiftIdcApplication_ServiceIntegrationRef {
        RedshiftIdcApplication_ServiceIntegrationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftIdcApplication_ServiceIntegrationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lake_formation` after provisioning.\n"]
    pub fn lake_formation(&self) -> terrars::ListRef<RedshiftIdcApplication_ServiceIntegration_LakeFormationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lake_formation", self.base))
    }

    #[doc = "Get a reference to the value of field `redshift` after provisioning.\n"]
    pub fn redshift(&self) -> terrars::ListRef<RedshiftIdcApplication_ServiceIntegration_RedshiftRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_access_grants` after provisioning.\n"]
    pub fn s3_access_grants(&self) -> terrars::ListRef<RedshiftIdcApplication_ServiceIntegration_S3AccessGrantsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_access_grants", self.base))
    }
}
