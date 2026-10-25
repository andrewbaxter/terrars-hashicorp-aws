#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerWorkforceData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    workforce_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cognito_config: std::option::Option<std::vec::Vec<SagemakerWorkforce_CognitoConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc_config: std::option::Option<std::vec::Vec<SagemakerWorkforce_OidcConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_ip_config: std::option::Option<std::vec::Vec<SagemakerWorkforce_SourceIpConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workforce_vpc_config: std::option::Option<std::vec::Vec<SagemakerWorkforce_WorkforceVpcConfig>>,
    dynamic: SagemakerWorkforceDynamic,
}

struct SagemakerWorkforce_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerWorkforceData_>,
}

#[derive(Clone)]
pub struct SagemakerWorkforce(std::rc::Rc<SagemakerWorkforce_>);

impl SagemakerWorkforce {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cognito_config`.\n"]
    pub fn set_cognito_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerWorkforce_CognitoConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cognito_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cognito_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oidc_config`.\n"]
    pub fn set_oidc_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerWorkforce_OidcConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oidc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oidc_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_ip_config`.\n"]
    pub fn set_source_ip_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerWorkforce_SourceIpConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_ip_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_ip_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workforce_vpc_config`.\n"]
    pub fn set_workforce_vpc_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerWorkforce_WorkforceVpcConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workforce_vpc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workforce_vpc_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subdomain` after provisioning.\n"]
    pub fn subdomain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subdomain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workforce_name` after provisioning.\n"]
    pub fn workforce_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workforce_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cognito_config` after provisioning.\n"]
    pub fn cognito_config(&self) -> terrars::ListRef<SagemakerWorkforce_CognitoConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cognito_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oidc_config` after provisioning.\n"]
    pub fn oidc_config(&self) -> terrars::ListRef<SagemakerWorkforce_OidcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_ip_config` after provisioning.\n"]
    pub fn source_ip_config(&self) -> terrars::ListRef<SagemakerWorkforce_SourceIpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_ip_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workforce_vpc_config` after provisioning.\n"]
    pub fn workforce_vpc_config(&self) -> terrars::ListRef<SagemakerWorkforce_WorkforceVpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workforce_vpc_config", self.extract_ref()))
    }
}

impl terrars::Referable for SagemakerWorkforce {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerWorkforce { }

impl terrars::ToListMappable for SagemakerWorkforce {
    type O = terrars::ListRef<SagemakerWorkforceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerWorkforce_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_workforce".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerWorkforce {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub workforce_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerWorkforce {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerWorkforce {
        let out = SagemakerWorkforce(std::rc::Rc::new(SagemakerWorkforce_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerWorkforceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                workforce_name: self.workforce_name,
                cognito_config: core::default::Default::default(),
                oidc_config: core::default::Default::default(),
                source_ip_config: core::default::Default::default(),
                workforce_vpc_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerWorkforceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerWorkforceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerWorkforceRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subdomain` after provisioning.\n"]
    pub fn subdomain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subdomain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workforce_name` after provisioning.\n"]
    pub fn workforce_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workforce_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cognito_config` after provisioning.\n"]
    pub fn cognito_config(&self) -> terrars::ListRef<SagemakerWorkforce_CognitoConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cognito_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oidc_config` after provisioning.\n"]
    pub fn oidc_config(&self) -> terrars::ListRef<SagemakerWorkforce_OidcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_ip_config` after provisioning.\n"]
    pub fn source_ip_config(&self) -> terrars::ListRef<SagemakerWorkforce_SourceIpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_ip_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workforce_vpc_config` after provisioning.\n"]
    pub fn workforce_vpc_config(&self) -> terrars::ListRef<SagemakerWorkforce_WorkforceVpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workforce_vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SagemakerWorkforceDynamic {
    cognito_config: std::option::Option<terrars::DynamicBlock<SagemakerWorkforce_CognitoConfig>>,
    oidc_config: std::option::Option<terrars::DynamicBlock<SagemakerWorkforce_OidcConfig>>,
    source_ip_config: std::option::Option<terrars::DynamicBlock<SagemakerWorkforce_SourceIpConfig>>,
    workforce_vpc_config: std::option::Option<terrars::DynamicBlock<SagemakerWorkforce_WorkforceVpcConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerWorkforce_CognitoConfig {
    client_id: terrars::PrimField<std::string::String>,
    user_pool: terrars::PrimField<std::string::String>,
}

impl SagemakerWorkforce_CognitoConfig { }

impl terrars::ToListMappable for SagemakerWorkforce_CognitoConfig {
    type O = terrars::BlockAssignable<SagemakerWorkforce_CognitoConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerWorkforce_CognitoConfig {
    #[doc = ""]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerWorkforce_CognitoConfig {
    pub fn build(self) -> SagemakerWorkforce_CognitoConfig {
        SagemakerWorkforce_CognitoConfig {
            client_id: self.client_id,
            user_pool: self.user_pool,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerWorkforce_CognitoConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerWorkforce_CognitoConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerWorkforce_CognitoConfigRef {
        SagemakerWorkforce_CognitoConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerWorkforce_CognitoConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `user_pool` after provisioning.\n"]
    pub fn user_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerWorkforce_OidcConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_request_extra_params: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    authorization_endpoint: terrars::PrimField<std::string::String>,
    client_id: terrars::PrimField<std::string::String>,
    client_secret: terrars::PrimField<std::string::String>,
    issuer: terrars::PrimField<std::string::String>,
    jwks_uri: terrars::PrimField<std::string::String>,
    logout_endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    token_endpoint: terrars::PrimField<std::string::String>,
    user_info_endpoint: terrars::PrimField<std::string::String>,
}

impl SagemakerWorkforce_OidcConfig {
    #[doc = "Set the field `authentication_request_extra_params`.\n"]
    pub fn set_authentication_request_extra_params(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.authentication_request_extra_params = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerWorkforce_OidcConfig {
    type O = terrars::BlockAssignable<SagemakerWorkforce_OidcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerWorkforce_OidcConfig {
    #[doc = ""]
    pub authorization_endpoint: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub client_secret: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub issuer: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub jwks_uri: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub logout_endpoint: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub token_endpoint: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_info_endpoint: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerWorkforce_OidcConfig {
    pub fn build(self) -> SagemakerWorkforce_OidcConfig {
        SagemakerWorkforce_OidcConfig {
            authentication_request_extra_params: core::default::Default::default(),
            authorization_endpoint: self.authorization_endpoint,
            client_id: self.client_id,
            client_secret: self.client_secret,
            issuer: self.issuer,
            jwks_uri: self.jwks_uri,
            logout_endpoint: self.logout_endpoint,
            scope: core::default::Default::default(),
            token_endpoint: self.token_endpoint,
            user_info_endpoint: self.user_info_endpoint,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerWorkforce_OidcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerWorkforce_OidcConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerWorkforce_OidcConfigRef {
        SagemakerWorkforce_OidcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerWorkforce_OidcConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication_request_extra_params` after provisioning.\n"]
    pub fn authentication_request_extra_params(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.authentication_request_extra_params", self.base))
    }

    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `jwks_uri` after provisioning.\n"]
    pub fn jwks_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jwks_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `logout_endpoint` after provisioning.\n"]
    pub fn logout_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logout_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `user_info_endpoint` after provisioning.\n"]
    pub fn user_info_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_info_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerWorkforce_SourceIpConfig {
    cidrs: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl SagemakerWorkforce_SourceIpConfig { }

impl terrars::ToListMappable for SagemakerWorkforce_SourceIpConfig {
    type O = terrars::BlockAssignable<SagemakerWorkforce_SourceIpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerWorkforce_SourceIpConfig {
    #[doc = ""]
    pub cidrs: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildSagemakerWorkforce_SourceIpConfig {
    pub fn build(self) -> SagemakerWorkforce_SourceIpConfig {
        SagemakerWorkforce_SourceIpConfig { cidrs: self.cidrs }
    }
}

#[allow(dead_code)]
pub struct SagemakerWorkforce_SourceIpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerWorkforce_SourceIpConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerWorkforce_SourceIpConfigRef {
        SagemakerWorkforce_SourceIpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerWorkforce_SourceIpConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidrs` after provisioning.\n"]
    pub fn cidrs(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cidrs", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerWorkforce_WorkforceVpcConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnets: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerWorkforce_WorkforceVpcConfig {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnets`.\n"]
    pub fn set_subnets(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerWorkforce_WorkforceVpcConfig {
    type O = terrars::BlockAssignable<SagemakerWorkforce_WorkforceVpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerWorkforce_WorkforceVpcConfig {}

impl BuildSagemakerWorkforce_WorkforceVpcConfig {
    pub fn build(self) -> SagemakerWorkforce_WorkforceVpcConfig {
        SagemakerWorkforce_WorkforceVpcConfig {
            security_group_ids: core::default::Default::default(),
            subnets: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerWorkforce_WorkforceVpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerWorkforce_WorkforceVpcConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerWorkforce_WorkforceVpcConfigRef {
        SagemakerWorkforce_WorkforceVpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerWorkforce_WorkforceVpcConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
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
