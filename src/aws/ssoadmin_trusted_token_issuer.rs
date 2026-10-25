#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SsoadminTrustedTokenIssuerData_ {
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
    client_token: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_arn: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    trusted_token_issuer_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trusted_token_issuer_configuration: std
    ::option
    ::Option<std::vec::Vec<SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration>>,
    dynamic: SsoadminTrustedTokenIssuerDynamic,
}

struct SsoadminTrustedTokenIssuer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SsoadminTrustedTokenIssuerData_>,
}

#[derive(Clone)]
pub struct SsoadminTrustedTokenIssuer(std::rc::Rc<SsoadminTrustedTokenIssuer_>);

impl SsoadminTrustedTokenIssuer {
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

    #[doc = "Set the field `client_token`.\n"]
    pub fn set_client_token(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().client_token = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `trusted_token_issuer_configuration`.\n"]
    pub fn set_trusted_token_issuer_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().trusted_token_issuer_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.trusted_token_issuer_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_token` after provisioning.\n"]
    pub fn client_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trusted_token_issuer_type` after provisioning.\n"]
    pub fn trusted_token_issuer_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trusted_token_issuer_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trusted_token_issuer_configuration` after provisioning.\n"]
    pub fn trusted_token_issuer_configuration(
        &self,
    ) -> terrars::ListRef<SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.trusted_token_issuer_configuration", self.extract_ref()),
        )
    }
}

impl terrars::Referable for SsoadminTrustedTokenIssuer {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SsoadminTrustedTokenIssuer { }

impl terrars::ToListMappable for SsoadminTrustedTokenIssuer {
    type O = terrars::ListRef<SsoadminTrustedTokenIssuerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SsoadminTrustedTokenIssuer_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ssoadmin_trusted_token_issuer".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsoadminTrustedTokenIssuer {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub instance_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub trusted_token_issuer_type: terrars::PrimField<std::string::String>,
}

impl BuildSsoadminTrustedTokenIssuer {
    pub fn build(self, stack: &mut terrars::Stack) -> SsoadminTrustedTokenIssuer {
        let out = SsoadminTrustedTokenIssuer(std::rc::Rc::new(SsoadminTrustedTokenIssuer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SsoadminTrustedTokenIssuerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                client_token: core::default::Default::default(),
                instance_arn: self.instance_arn,
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                trusted_token_issuer_type: self.trusted_token_issuer_type,
                trusted_token_issuer_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsoadminTrustedTokenIssuerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsoadminTrustedTokenIssuerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SsoadminTrustedTokenIssuerRef {
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

    #[doc = "Get a reference to the value of field `client_token` after provisioning.\n"]
    pub fn client_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trusted_token_issuer_type` after provisioning.\n"]
    pub fn trusted_token_issuer_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trusted_token_issuer_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trusted_token_issuer_configuration` after provisioning.\n"]
    pub fn trusted_token_issuer_configuration(
        &self,
    ) -> terrars::ListRef<SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.trusted_token_issuer_configuration", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct SsoadminTrustedTokenIssuerDynamic {
    trusted_token_issuer_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration>>,
}

#[derive(Serialize)]
pub struct SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration_OidcJwtConfiguration {
    claim_attribute_path: terrars::PrimField<std::string::String>,
    identity_store_attribute_path: terrars::PrimField<std::string::String>,
    issuer_url: terrars::PrimField<std::string::String>,
    jwks_retrieval_option: terrars::PrimField<std::string::String>,
}

impl SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration_OidcJwtConfiguration { }

impl terrars::ToListMappable for SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration_OidcJwtConfiguration {
    type O =
        terrars::BlockAssignable<SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration_OidcJwtConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration_OidcJwtConfiguration {
    #[doc = ""]
    pub claim_attribute_path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub identity_store_attribute_path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub issuer_url: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub jwks_retrieval_option: terrars::PrimField<std::string::String>,
}

impl BuildSsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration_OidcJwtConfiguration {
    pub fn build(self) -> SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration_OidcJwtConfiguration {
        SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration_OidcJwtConfiguration {
            claim_attribute_path: self.claim_attribute_path,
            identity_store_attribute_path: self.identity_store_attribute_path,
            issuer_url: self.issuer_url,
            jwks_retrieval_option: self.jwks_retrieval_option,
        }
    }
}

#[allow(dead_code)]
pub struct SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration_OidcJwtConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration_OidcJwtConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration_OidcJwtConfigurationRef {
        SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration_OidcJwtConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration_OidcJwtConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `claim_attribute_path` after provisioning.\n"]
    pub fn claim_attribute_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.claim_attribute_path", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_store_attribute_path` after provisioning.\n"]
    pub fn identity_store_attribute_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_store_attribute_path", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer_url` after provisioning.\n"]
    pub fn issuer_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer_url", self.base))
    }

    #[doc = "Get a reference to the value of field `jwks_retrieval_option` after provisioning.\n"]
    pub fn jwks_retrieval_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jwks_retrieval_option", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfigurationDynamic {
    oidc_jwt_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration_OidcJwtConfiguration>>,
}

#[derive(Serialize)]
pub struct SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc_jwt_configuration: std
    ::option
    ::Option<std::vec::Vec<SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration_OidcJwtConfiguration>>,
    dynamic: SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfigurationDynamic,
}

impl SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration {
    #[doc = "Set the field `oidc_jwt_configuration`.\n"]
    pub fn set_oidc_jwt_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration_OidcJwtConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oidc_jwt_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oidc_jwt_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration {
    type O = terrars::BlockAssignable<SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration {}

impl BuildSsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration {
    pub fn build(self) -> SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration {
        SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration {
            oidc_jwt_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfigurationRef {
        SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `oidc_jwt_configuration` after provisioning.\n"]
    pub fn oidc_jwt_configuration(
        &self,
    ) -> terrars::ListRef<SsoadminTrustedTokenIssuer_TrustedTokenIssuerConfiguration_OidcJwtConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc_jwt_configuration", self.base))
    }
}
