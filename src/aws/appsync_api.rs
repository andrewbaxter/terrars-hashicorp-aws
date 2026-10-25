#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppsyncApiData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner_contact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_config: std::option::Option<std::vec::Vec<AppsyncApi_EventConfig>>,
    dynamic: AppsyncApiDynamic,
}

struct AppsyncApi_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppsyncApiData_>,
}

#[derive(Clone)]
pub struct AppsyncApi(std::rc::Rc<AppsyncApi_>);

impl AppsyncApi {
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

    #[doc = "Set the field `owner_contact`.\n"]
    pub fn set_owner_contact(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().owner_contact = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `event_config`.\n"]
    pub fn set_event_config(self, v: impl std::convert::Into<terrars::BlockAssignable<AppsyncApi_EventConfig>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `api_arn` after provisioning.\n"]
    pub fn api_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns` after provisioning.\n"]
    pub fn dns(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_contact` after provisioning.\n"]
    pub fn owner_contact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_contact", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `waf_web_acl_arn` after provisioning.\n"]
    pub fn waf_web_acl_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.waf_web_acl_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xray_enabled` after provisioning.\n"]
    pub fn xray_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.xray_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_config` after provisioning.\n"]
    pub fn event_config(&self) -> terrars::ListRef<AppsyncApi_EventConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_config", self.extract_ref()))
    }
}

impl terrars::Referable for AppsyncApi {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppsyncApi { }

impl terrars::ToListMappable for AppsyncApi {
    type O = terrars::ListRef<AppsyncApiRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppsyncApi_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_appsync_api".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppsyncApi {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncApi {
    pub fn build(self, stack: &mut terrars::Stack) -> AppsyncApi {
        let out = AppsyncApi(std::rc::Rc::new(AppsyncApi_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppsyncApiData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                name: self.name,
                owner_contact: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                event_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppsyncApiRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncApiRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppsyncApiRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_arn` after provisioning.\n"]
    pub fn api_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns` after provisioning.\n"]
    pub fn dns(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_contact` after provisioning.\n"]
    pub fn owner_contact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_contact", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `waf_web_acl_arn` after provisioning.\n"]
    pub fn waf_web_acl_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.waf_web_acl_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xray_enabled` after provisioning.\n"]
    pub fn xray_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.xray_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_config` after provisioning.\n"]
    pub fn event_config(&self) -> terrars::ListRef<AppsyncApi_EventConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AppsyncApiDynamic {
    event_config: std::option::Option<terrars::DynamicBlock<AppsyncApi_EventConfig>>,
}

#[derive(Serialize)]
pub struct AppsyncApi_EventConfig_AuthProvider_CognitoConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_id_client_regex: std::option::Option<terrars::PrimField<std::string::String>>,
    aws_region: terrars::PrimField<std::string::String>,
    user_pool_id: terrars::PrimField<std::string::String>,
}

impl AppsyncApi_EventConfig_AuthProvider_CognitoConfig {
    #[doc = "Set the field `app_id_client_regex`.\n"]
    pub fn set_app_id_client_regex(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.app_id_client_regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppsyncApi_EventConfig_AuthProvider_CognitoConfig {
    type O = terrars::BlockAssignable<AppsyncApi_EventConfig_AuthProvider_CognitoConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncApi_EventConfig_AuthProvider_CognitoConfig {
    #[doc = ""]
    pub aws_region: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool_id: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncApi_EventConfig_AuthProvider_CognitoConfig {
    pub fn build(self) -> AppsyncApi_EventConfig_AuthProvider_CognitoConfig {
        AppsyncApi_EventConfig_AuthProvider_CognitoConfig {
            app_id_client_regex: core::default::Default::default(),
            aws_region: self.aws_region,
            user_pool_id: self.user_pool_id,
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncApi_EventConfig_AuthProvider_CognitoConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncApi_EventConfig_AuthProvider_CognitoConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppsyncApi_EventConfig_AuthProvider_CognitoConfigRef {
        AppsyncApi_EventConfig_AuthProvider_CognitoConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncApi_EventConfig_AuthProvider_CognitoConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_id_client_regex` after provisioning.\n"]
    pub fn app_id_client_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id_client_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_region` after provisioning.\n"]
    pub fn aws_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_region", self.base))
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncApi_EventConfig_AuthProvider_LambdaAuthorizerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizer_result_ttl_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    authorizer_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_validation_expression: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppsyncApi_EventConfig_AuthProvider_LambdaAuthorizerConfig {
    #[doc = "Set the field `authorizer_result_ttl_in_seconds`.\n"]
    pub fn set_authorizer_result_ttl_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.authorizer_result_ttl_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_validation_expression`.\n"]
    pub fn set_identity_validation_expression(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.identity_validation_expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppsyncApi_EventConfig_AuthProvider_LambdaAuthorizerConfig {
    type O = terrars::BlockAssignable<AppsyncApi_EventConfig_AuthProvider_LambdaAuthorizerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncApi_EventConfig_AuthProvider_LambdaAuthorizerConfig {
    #[doc = ""]
    pub authorizer_uri: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncApi_EventConfig_AuthProvider_LambdaAuthorizerConfig {
    pub fn build(self) -> AppsyncApi_EventConfig_AuthProvider_LambdaAuthorizerConfig {
        AppsyncApi_EventConfig_AuthProvider_LambdaAuthorizerConfig {
            authorizer_result_ttl_in_seconds: core::default::Default::default(),
            authorizer_uri: self.authorizer_uri,
            identity_validation_expression: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncApi_EventConfig_AuthProvider_LambdaAuthorizerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncApi_EventConfig_AuthProvider_LambdaAuthorizerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppsyncApi_EventConfig_AuthProvider_LambdaAuthorizerConfigRef {
        AppsyncApi_EventConfig_AuthProvider_LambdaAuthorizerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncApi_EventConfig_AuthProvider_LambdaAuthorizerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorizer_result_ttl_in_seconds` after provisioning.\n"]
    pub fn authorizer_result_ttl_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorizer_result_ttl_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `authorizer_uri` after provisioning.\n"]
    pub fn authorizer_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorizer_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_validation_expression` after provisioning.\n"]
    pub fn identity_validation_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_validation_expression", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncApi_EventConfig_AuthProvider_OpenidConnectConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iat_ttl: std::option::Option<terrars::PrimField<f64>>,
    issuer: terrars::PrimField<std::string::String>,
}

impl AppsyncApi_EventConfig_AuthProvider_OpenidConnectConfig {
    #[doc = "Set the field `auth_ttl`.\n"]
    pub fn set_auth_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.auth_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iat_ttl`.\n"]
    pub fn set_iat_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.iat_ttl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppsyncApi_EventConfig_AuthProvider_OpenidConnectConfig {
    type O = terrars::BlockAssignable<AppsyncApi_EventConfig_AuthProvider_OpenidConnectConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncApi_EventConfig_AuthProvider_OpenidConnectConfig {
    #[doc = ""]
    pub issuer: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncApi_EventConfig_AuthProvider_OpenidConnectConfig {
    pub fn build(self) -> AppsyncApi_EventConfig_AuthProvider_OpenidConnectConfig {
        AppsyncApi_EventConfig_AuthProvider_OpenidConnectConfig {
            auth_ttl: core::default::Default::default(),
            client_id: core::default::Default::default(),
            iat_ttl: core::default::Default::default(),
            issuer: self.issuer,
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncApi_EventConfig_AuthProvider_OpenidConnectConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncApi_EventConfig_AuthProvider_OpenidConnectConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppsyncApi_EventConfig_AuthProvider_OpenidConnectConfigRef {
        AppsyncApi_EventConfig_AuthProvider_OpenidConnectConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncApi_EventConfig_AuthProvider_OpenidConnectConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_ttl` after provisioning.\n"]
    pub fn auth_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `iat_ttl` after provisioning.\n"]
    pub fn iat_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iat_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppsyncApi_EventConfig_AuthProviderDynamic {
    cognito_config: std::option::Option<terrars::DynamicBlock<AppsyncApi_EventConfig_AuthProvider_CognitoConfig>>,
    lambda_authorizer_config: std
    ::option
    ::Option<terrars::DynamicBlock<AppsyncApi_EventConfig_AuthProvider_LambdaAuthorizerConfig>>,
    openid_connect_config: std
    ::option
    ::Option<terrars::DynamicBlock<AppsyncApi_EventConfig_AuthProvider_OpenidConnectConfig>>,
}

#[derive(Serialize)]
pub struct AppsyncApi_EventConfig_AuthProvider {
    auth_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cognito_config: std::option::Option<std::vec::Vec<AppsyncApi_EventConfig_AuthProvider_CognitoConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_authorizer_config: std
    ::option
    ::Option<std::vec::Vec<AppsyncApi_EventConfig_AuthProvider_LambdaAuthorizerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    openid_connect_config: std::option::Option<std::vec::Vec<AppsyncApi_EventConfig_AuthProvider_OpenidConnectConfig>>,
    dynamic: AppsyncApi_EventConfig_AuthProviderDynamic,
}

impl AppsyncApi_EventConfig_AuthProvider {
    #[doc = "Set the field `cognito_config`.\n"]
    pub fn set_cognito_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncApi_EventConfig_AuthProvider_CognitoConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cognito_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cognito_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_authorizer_config`.\n"]
    pub fn set_lambda_authorizer_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppsyncApi_EventConfig_AuthProvider_LambdaAuthorizerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda_authorizer_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_authorizer_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `openid_connect_config`.\n"]
    pub fn set_openid_connect_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppsyncApi_EventConfig_AuthProvider_OpenidConnectConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.openid_connect_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.openid_connect_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppsyncApi_EventConfig_AuthProvider {
    type O = terrars::BlockAssignable<AppsyncApi_EventConfig_AuthProvider>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncApi_EventConfig_AuthProvider {
    #[doc = ""]
    pub auth_type: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncApi_EventConfig_AuthProvider {
    pub fn build(self) -> AppsyncApi_EventConfig_AuthProvider {
        AppsyncApi_EventConfig_AuthProvider {
            auth_type: self.auth_type,
            cognito_config: core::default::Default::default(),
            lambda_authorizer_config: core::default::Default::default(),
            openid_connect_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncApi_EventConfig_AuthProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncApi_EventConfig_AuthProviderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncApi_EventConfig_AuthProviderRef {
        AppsyncApi_EventConfig_AuthProviderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncApi_EventConfig_AuthProviderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }

    #[doc = "Get a reference to the value of field `cognito_config` after provisioning.\n"]
    pub fn cognito_config(&self) -> terrars::ListRef<AppsyncApi_EventConfig_AuthProvider_CognitoConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cognito_config", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_authorizer_config` after provisioning.\n"]
    pub fn lambda_authorizer_config(
        &self,
    ) -> terrars::ListRef<AppsyncApi_EventConfig_AuthProvider_LambdaAuthorizerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_authorizer_config", self.base))
    }

    #[doc = "Get a reference to the value of field `openid_connect_config` after provisioning.\n"]
    pub fn openid_connect_config(&self) -> terrars::ListRef<AppsyncApi_EventConfig_AuthProvider_OpenidConnectConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.openid_connect_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncApi_EventConfig_ConnectionAuthMode {
    auth_type: terrars::PrimField<std::string::String>,
}

impl AppsyncApi_EventConfig_ConnectionAuthMode { }

impl terrars::ToListMappable for AppsyncApi_EventConfig_ConnectionAuthMode {
    type O = terrars::BlockAssignable<AppsyncApi_EventConfig_ConnectionAuthMode>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncApi_EventConfig_ConnectionAuthMode {
    #[doc = ""]
    pub auth_type: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncApi_EventConfig_ConnectionAuthMode {
    pub fn build(self) -> AppsyncApi_EventConfig_ConnectionAuthMode {
        AppsyncApi_EventConfig_ConnectionAuthMode { auth_type: self.auth_type }
    }
}

#[allow(dead_code)]
pub struct AppsyncApi_EventConfig_ConnectionAuthModeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncApi_EventConfig_ConnectionAuthModeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncApi_EventConfig_ConnectionAuthModeRef {
        AppsyncApi_EventConfig_ConnectionAuthModeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncApi_EventConfig_ConnectionAuthModeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncApi_EventConfig_DefaultPublishAuthMode {
    auth_type: terrars::PrimField<std::string::String>,
}

impl AppsyncApi_EventConfig_DefaultPublishAuthMode { }

impl terrars::ToListMappable for AppsyncApi_EventConfig_DefaultPublishAuthMode {
    type O = terrars::BlockAssignable<AppsyncApi_EventConfig_DefaultPublishAuthMode>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncApi_EventConfig_DefaultPublishAuthMode {
    #[doc = ""]
    pub auth_type: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncApi_EventConfig_DefaultPublishAuthMode {
    pub fn build(self) -> AppsyncApi_EventConfig_DefaultPublishAuthMode {
        AppsyncApi_EventConfig_DefaultPublishAuthMode { auth_type: self.auth_type }
    }
}

#[allow(dead_code)]
pub struct AppsyncApi_EventConfig_DefaultPublishAuthModeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncApi_EventConfig_DefaultPublishAuthModeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppsyncApi_EventConfig_DefaultPublishAuthModeRef {
        AppsyncApi_EventConfig_DefaultPublishAuthModeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncApi_EventConfig_DefaultPublishAuthModeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncApi_EventConfig_DefaultSubscribeAuthMode {
    auth_type: terrars::PrimField<std::string::String>,
}

impl AppsyncApi_EventConfig_DefaultSubscribeAuthMode { }

impl terrars::ToListMappable for AppsyncApi_EventConfig_DefaultSubscribeAuthMode {
    type O = terrars::BlockAssignable<AppsyncApi_EventConfig_DefaultSubscribeAuthMode>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncApi_EventConfig_DefaultSubscribeAuthMode {
    #[doc = ""]
    pub auth_type: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncApi_EventConfig_DefaultSubscribeAuthMode {
    pub fn build(self) -> AppsyncApi_EventConfig_DefaultSubscribeAuthMode {
        AppsyncApi_EventConfig_DefaultSubscribeAuthMode { auth_type: self.auth_type }
    }
}

#[allow(dead_code)]
pub struct AppsyncApi_EventConfig_DefaultSubscribeAuthModeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncApi_EventConfig_DefaultSubscribeAuthModeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppsyncApi_EventConfig_DefaultSubscribeAuthModeRef {
        AppsyncApi_EventConfig_DefaultSubscribeAuthModeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncApi_EventConfig_DefaultSubscribeAuthModeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncApi_EventConfig_LogConfig {
    cloudwatch_logs_role_arn: terrars::PrimField<std::string::String>,
    log_level: terrars::PrimField<std::string::String>,
}

impl AppsyncApi_EventConfig_LogConfig { }

impl terrars::ToListMappable for AppsyncApi_EventConfig_LogConfig {
    type O = terrars::BlockAssignable<AppsyncApi_EventConfig_LogConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncApi_EventConfig_LogConfig {
    #[doc = ""]
    pub cloudwatch_logs_role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub log_level: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncApi_EventConfig_LogConfig {
    pub fn build(self) -> AppsyncApi_EventConfig_LogConfig {
        AppsyncApi_EventConfig_LogConfig {
            cloudwatch_logs_role_arn: self.cloudwatch_logs_role_arn,
            log_level: self.log_level,
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncApi_EventConfig_LogConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncApi_EventConfig_LogConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncApi_EventConfig_LogConfigRef {
        AppsyncApi_EventConfig_LogConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncApi_EventConfig_LogConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs_role_arn` after provisioning.\n"]
    pub fn cloudwatch_logs_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_logs_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppsyncApi_EventConfigDynamic {
    auth_provider: std::option::Option<terrars::DynamicBlock<AppsyncApi_EventConfig_AuthProvider>>,
    connection_auth_mode: std::option::Option<terrars::DynamicBlock<AppsyncApi_EventConfig_ConnectionAuthMode>>,
    default_publish_auth_mode: std
    ::option
    ::Option<terrars::DynamicBlock<AppsyncApi_EventConfig_DefaultPublishAuthMode>>,
    default_subscribe_auth_mode: std
    ::option
    ::Option<terrars::DynamicBlock<AppsyncApi_EventConfig_DefaultSubscribeAuthMode>>,
    log_config: std::option::Option<terrars::DynamicBlock<AppsyncApi_EventConfig_LogConfig>>,
}

#[derive(Serialize)]
pub struct AppsyncApi_EventConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_provider: std::option::Option<std::vec::Vec<AppsyncApi_EventConfig_AuthProvider>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_auth_mode: std::option::Option<std::vec::Vec<AppsyncApi_EventConfig_ConnectionAuthMode>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_publish_auth_mode: std::option::Option<std::vec::Vec<AppsyncApi_EventConfig_DefaultPublishAuthMode>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_subscribe_auth_mode: std::option::Option<std::vec::Vec<AppsyncApi_EventConfig_DefaultSubscribeAuthMode>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_config: std::option::Option<std::vec::Vec<AppsyncApi_EventConfig_LogConfig>>,
    dynamic: AppsyncApi_EventConfigDynamic,
}

impl AppsyncApi_EventConfig {
    #[doc = "Set the field `auth_provider`.\n"]
    pub fn set_auth_provider(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncApi_EventConfig_AuthProvider>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auth_provider = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auth_provider = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `connection_auth_mode`.\n"]
    pub fn set_connection_auth_mode(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncApi_EventConfig_ConnectionAuthMode>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.connection_auth_mode = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.connection_auth_mode = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_publish_auth_mode`.\n"]
    pub fn set_default_publish_auth_mode(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncApi_EventConfig_DefaultPublishAuthMode>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_publish_auth_mode = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_publish_auth_mode = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_subscribe_auth_mode`.\n"]
    pub fn set_default_subscribe_auth_mode(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncApi_EventConfig_DefaultSubscribeAuthMode>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_subscribe_auth_mode = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_subscribe_auth_mode = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_config`.\n"]
    pub fn set_log_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncApi_EventConfig_LogConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.log_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.log_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppsyncApi_EventConfig {
    type O = terrars::BlockAssignable<AppsyncApi_EventConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncApi_EventConfig {}

impl BuildAppsyncApi_EventConfig {
    pub fn build(self) -> AppsyncApi_EventConfig {
        AppsyncApi_EventConfig {
            auth_provider: core::default::Default::default(),
            connection_auth_mode: core::default::Default::default(),
            default_publish_auth_mode: core::default::Default::default(),
            default_subscribe_auth_mode: core::default::Default::default(),
            log_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncApi_EventConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncApi_EventConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncApi_EventConfigRef {
        AppsyncApi_EventConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncApi_EventConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_provider` after provisioning.\n"]
    pub fn auth_provider(&self) -> terrars::ListRef<AppsyncApi_EventConfig_AuthProviderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_provider", self.base))
    }

    #[doc = "Get a reference to the value of field `connection_auth_mode` after provisioning.\n"]
    pub fn connection_auth_mode(&self) -> terrars::ListRef<AppsyncApi_EventConfig_ConnectionAuthModeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_auth_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `default_publish_auth_mode` after provisioning.\n"]
    pub fn default_publish_auth_mode(&self) -> terrars::ListRef<AppsyncApi_EventConfig_DefaultPublishAuthModeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_publish_auth_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `default_subscribe_auth_mode` after provisioning.\n"]
    pub fn default_subscribe_auth_mode(&self) -> terrars::ListRef<AppsyncApi_EventConfig_DefaultSubscribeAuthModeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_subscribe_auth_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> terrars::ListRef<AppsyncApi_EventConfig_LogConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.base))
    }
}
