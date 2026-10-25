#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentcoreOauth2CredentialProviderData_ {
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
    credential_provider_vendor: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_provider_config: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockagentcoreOauth2CredentialProvider_Timeouts>,
    dynamic: BedrockagentcoreOauth2CredentialProviderDynamic,
}

struct BedrockagentcoreOauth2CredentialProvider_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockagentcoreOauth2CredentialProviderData_>,
}

#[derive(Clone)]
pub struct BedrockagentcoreOauth2CredentialProvider(std::rc::Rc<BedrockagentcoreOauth2CredentialProvider_>);

impl BedrockagentcoreOauth2CredentialProvider {
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

    #[doc = "Set the field `oauth2_provider_config`.\n"]
    pub fn set_oauth2_provider_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oauth2_provider_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oauth2_provider_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockagentcoreOauth2CredentialProvider_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `callback_url` after provisioning.\n"]
    pub fn callback_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.callback_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_secret_arn` after provisioning.\n"]
    pub fn client_secret_arn(&self) -> terrars::ListRef<BedrockagentcoreOauth2CredentialProvider_ClientSecretArnRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_secret_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credential_provider_arn` after provisioning.\n"]
    pub fn credential_provider_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credential_provider_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credential_provider_vendor` after provisioning.\n"]
    pub fn credential_provider_vendor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credential_provider_vendor", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `oauth2_provider_config` after provisioning.\n"]
    pub fn oauth2_provider_config(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2_provider_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreOauth2CredentialProvider_TimeoutsRef {
        BedrockagentcoreOauth2CredentialProvider_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for BedrockagentcoreOauth2CredentialProvider {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockagentcoreOauth2CredentialProvider { }

impl terrars::ToListMappable for BedrockagentcoreOauth2CredentialProvider {
    type O = terrars::ListRef<BedrockagentcoreOauth2CredentialProviderRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockagentcoreOauth2CredentialProvider_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrockagentcore_oauth2_credential_provider".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub credential_provider_vendor: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreOauth2CredentialProvider {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockagentcoreOauth2CredentialProvider {
        let out =
            BedrockagentcoreOauth2CredentialProvider(std::rc::Rc::new(BedrockagentcoreOauth2CredentialProvider_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(BedrockagentcoreOauth2CredentialProviderData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    provisioner: core::default::Default::default(),
                    credential_provider_vendor: self.credential_provider_vendor,
                    name: self.name,
                    region: core::default::Default::default(),
                    tags: core::default::Default::default(),
                    oauth2_provider_config: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: core::default::Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOauth2CredentialProviderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockagentcoreOauth2CredentialProviderRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `callback_url` after provisioning.\n"]
    pub fn callback_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.callback_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_secret_arn` after provisioning.\n"]
    pub fn client_secret_arn(&self) -> terrars::ListRef<BedrockagentcoreOauth2CredentialProvider_ClientSecretArnRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_secret_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credential_provider_arn` after provisioning.\n"]
    pub fn credential_provider_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credential_provider_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credential_provider_vendor` after provisioning.\n"]
    pub fn credential_provider_vendor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credential_provider_vendor", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `oauth2_provider_config` after provisioning.\n"]
    pub fn oauth2_provider_config(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2_provider_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreOauth2CredentialProvider_TimeoutsRef {
        BedrockagentcoreOauth2CredentialProvider_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOauth2CredentialProviderDynamic {
    oauth2_provider_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_ClientSecretArn {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreOauth2CredentialProvider_ClientSecretArn {
    #[doc = "Set the field `secret_arn`.\n"]
    pub fn set_secret_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreOauth2CredentialProvider_ClientSecretArn {
    type O = terrars::BlockAssignable<BedrockagentcoreOauth2CredentialProvider_ClientSecretArn>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_ClientSecretArn {}

impl BuildBedrockagentcoreOauth2CredentialProvider_ClientSecretArn {
    pub fn build(self) -> BedrockagentcoreOauth2CredentialProvider_ClientSecretArn {
        BedrockagentcoreOauth2CredentialProvider_ClientSecretArn { secret_arn: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_ClientSecretArnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOauth2CredentialProvider_ClientSecretArnRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_ClientSecretArnRef {
        BedrockagentcoreOauth2CredentialProvider_ClientSecretArnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_ClientSecretArnRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_endpoint_auth_methods: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadata {
    #[doc = "Set the field `authorization_endpoint`.\n"]
    pub fn set_authorization_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authorization_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_types`.\n"]
    pub fn set_response_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.response_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_endpoint`.\n"]
    pub fn set_token_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_endpoint_auth_methods`.\n"]
    pub fn set_token_endpoint_auth_methods(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.token_endpoint_auth_methods = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadata {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadata,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadata {}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadata {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadata {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadata {
            authorization_endpoint: core::default::Default::default(),
            issuer: core::default::Default::default(),
            response_types: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
            token_endpoint_auth_methods: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadataRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `response_types` after provisioning.\n"]
    pub fn response_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.response_types", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint_auth_methods` after provisioning.\n"]
    pub fn token_endpoint_auth_methods(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.token_endpoint_auth_methods", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_server_metadata: std
    ::option
    ::Option<
        terrars
        ::ListField<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadata,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discovery_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery {
    #[doc = "Set the field `authorization_server_metadata`.\n"]
    pub fn set_authorization_server_metadata(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadata,
                        >,
                    >,
    ) -> Self {
        self.authorization_server_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discovery_url`.\n"]
    pub fn set_discovery_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.discovery_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery {}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery {
            authorization_server_metadata: core::default::Default::default(),
            discovery_url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscoveryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscoveryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscoveryRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscoveryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscoveryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_server_metadata` after provisioning.\n"]
    pub fn authorization_server_metadata(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadataRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization_server_metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `discovery_url` after provisioning.\n"]
    pub fn discovery_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discovery_url", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig {
    json_key: terrars::PrimField<std::string::String>,
    secret_id: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig { }

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig {
    #[doc = ""]
    pub json_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub secret_id: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig {
            json_key: self.json_key,
            secret_id: self.secret_id,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfigRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `json_key` after provisioning.\n"]
    pub fn json_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json_key", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_AtlassianOauth2ProviderConfigDynamic {
    client_secret_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_AtlassianOauth2ProviderConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_credentials_wo_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
        >,
    >,
    dynamic: BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_AtlassianOauth2ProviderConfigDynamic,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_AtlassianOauth2ProviderConfig {
    #[doc = "Set the field `client_credentials_wo_version`.\n"]
    pub fn set_client_credentials_wo_version(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.client_credentials_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id_wo`.\n"]
    pub fn set_client_id_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_source`.\n"]
    pub fn set_client_secret_source(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_secret_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_wo`.\n"]
    pub fn set_client_secret_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_config`.\n"]
    pub fn set_client_secret_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_secret_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_secret_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_AtlassianOauth2ProviderConfig {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_AtlassianOauth2ProviderConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_AtlassianOauth2ProviderConfig {}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_AtlassianOauth2ProviderConfig {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_AtlassianOauth2ProviderConfig {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_AtlassianOauth2ProviderConfig {
            client_credentials_wo_version: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_id_wo: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            client_secret_source: core::default::Default::default(),
            client_secret_wo: core::default::Default::default(),
            client_secret_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_AtlassianOauth2ProviderConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_AtlassianOauth2ProviderConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_AtlassianOauth2ProviderConfigRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_AtlassianOauth2ProviderConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_AtlassianOauth2ProviderConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_credentials_wo_version` after provisioning.\n"]
    pub fn client_credentials_wo_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_credentials_wo_version", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id_wo` after provisioning.\n"]
    pub fn client_id_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_source` after provisioning.\n"]
    pub fn client_secret_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_source", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_wo` after provisioning.\n"]
    pub fn client_secret_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_discovery` after provisioning.\n"]
    pub fn oauth_discovery(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscoveryRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_discovery", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_config` after provisioning.\n"]
    pub fn client_secret_config(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_secret_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadata {
    authorization_endpoint: terrars::PrimField<std::string::String>,
    issuer: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    token_endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_endpoint_auth_methods: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadata {
    #[doc = "Set the field `response_types`.\n"]
    pub fn set_response_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.response_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_endpoint_auth_methods`.\n"]
    pub fn set_token_endpoint_auth_methods(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.token_endpoint_auth_methods = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadata {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadata,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadata {
    #[doc = ""]
    pub authorization_endpoint: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub issuer: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub token_endpoint: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadata {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadata {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadata {
            authorization_endpoint: self.authorization_endpoint,
            issuer: self.issuer,
            response_types: core::default::Default::default(),
            token_endpoint: self.token_endpoint,
            token_endpoint_auth_methods: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadataRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `response_types` after provisioning.\n"]
    pub fn response_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.response_types", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint_auth_methods` after provisioning.\n"]
    pub fn token_endpoint_auth_methods(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.token_endpoint_auth_methods", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscoveryDynamic {
    authorization_server_metadata: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadata,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discovery_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_server_metadata: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadata,
        >,
    >,
    dynamic: BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscoveryDynamic,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery {
    #[doc = "Set the field `discovery_url`.\n"]
    pub fn set_discovery_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.discovery_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorization_server_metadata`.\n"]
    pub fn set_authorization_server_metadata(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadata,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authorization_server_metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authorization_server_metadata = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery {}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery {
            discovery_url: core::default::Default::default(),
            authorization_server_metadata: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscoveryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscoveryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscoveryRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscoveryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscoveryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `discovery_url` after provisioning.\n"]
    pub fn discovery_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discovery_url", self.base))
    }

    #[doc = "Get a reference to the value of field `authorization_server_metadata` after provisioning.\n"]
    pub fn authorization_server_metadata(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery_AuthorizationServerMetadataRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization_server_metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig_TokenExchangeGrantTypeConfig {
    actor_token_content: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actor_token_scopes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig_TokenExchangeGrantTypeConfig {
    #[doc = "Set the field `actor_token_scopes`.\n"]
    pub fn set_actor_token_scopes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.actor_token_scopes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig_TokenExchangeGrantTypeConfig {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig_TokenExchangeGrantTypeConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig_TokenExchangeGrantTypeConfig {
    #[doc = ""]
    pub actor_token_content: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig_TokenExchangeGrantTypeConfig {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig_TokenExchangeGrantTypeConfig {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig_TokenExchangeGrantTypeConfig {
            actor_token_content: self.actor_token_content,
            actor_token_scopes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig_TokenExchangeGrantTypeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig_TokenExchangeGrantTypeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig_TokenExchangeGrantTypeConfigRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig_TokenExchangeGrantTypeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig_TokenExchangeGrantTypeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `actor_token_content` after provisioning.\n"]
    pub fn actor_token_content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.actor_token_content", self.base))
    }

    #[doc = "Get a reference to the value of field `actor_token_scopes` after provisioning.\n"]
    pub fn actor_token_scopes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.actor_token_scopes", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfigDynamic {
    token_exchange_grant_type_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig_TokenExchangeGrantTypeConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig {
    grant_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_exchange_grant_type_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig_TokenExchangeGrantTypeConfig,
        >,
    >,
    dynamic: BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfigDynamic,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig {
    #[doc = "Set the field `token_exchange_grant_type_config`.\n"]
    pub fn set_token_exchange_grant_type_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig_TokenExchangeGrantTypeConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.token_exchange_grant_type_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.token_exchange_grant_type_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig {
    #[doc = ""]
    pub grant_type: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig {
            grant_type: self.grant_type,
            token_exchange_grant_type_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfigRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grant_type` after provisioning.\n"]
    pub fn grant_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grant_type", self.base))
    }

    #[doc = "Get a reference to the value of field `token_exchange_grant_type_config` after provisioning.\n"]
    pub fn token_exchange_grant_type_config(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig_TokenExchangeGrantTypeConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.token_exchange_grant_type_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_ManagedVpcResource {
    endpoint_ip_address_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    vpc_identifier: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_ManagedVpcResource {
    #[doc = "Set the field `routing_domain`.\n"]
    pub fn set_routing_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.routing_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_ManagedVpcResource {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_ManagedVpcResource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_ManagedVpcResource {
    #[doc = ""]
    pub endpoint_ip_address_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub vpc_identifier: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_ManagedVpcResource {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_ManagedVpcResource {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_ManagedVpcResource {
            endpoint_ip_address_type: self.endpoint_ip_address_type,
            routing_domain: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
            tags: core::default::Default::default(),
            vpc_identifier: self.vpc_identifier,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_ManagedVpcResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_ManagedVpcResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_ManagedVpcResourceRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_ManagedVpcResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_ManagedVpcResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_ip_address_type` after provisioning.\n"]
    pub fn endpoint_ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_ip_address_type", self.base))
    }

    #[doc = "Get a reference to the value of field `routing_domain` after provisioning.\n"]
    pub fn routing_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_domain", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_identifier` after provisioning.\n"]
    pub fn vpc_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_SelfManagedLatticeResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_configuration_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_SelfManagedLatticeResource {
    #[doc = "Set the field `resource_configuration_identifier`.\n"]
    pub fn set_resource_configuration_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.resource_configuration_identifier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_SelfManagedLatticeResource {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_SelfManagedLatticeResource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_SelfManagedLatticeResource {}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_SelfManagedLatticeResource {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_SelfManagedLatticeResource {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_SelfManagedLatticeResource {
            resource_configuration_identifier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_SelfManagedLatticeResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_SelfManagedLatticeResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_SelfManagedLatticeResourceRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_SelfManagedLatticeResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_SelfManagedLatticeResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_configuration_identifier` after provisioning.\n"]
    pub fn resource_configuration_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_configuration_identifier", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointDynamic {
    managed_vpc_resource: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_ManagedVpcResource,
        >,
    >,
    self_managed_lattice_resource: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_SelfManagedLatticeResource,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_vpc_resource: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_ManagedVpcResource,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_managed_lattice_resource: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_SelfManagedLatticeResource,
        >,
    >,
    dynamic: BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointDynamic,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint {
    #[doc = "Set the field `managed_vpc_resource`.\n"]
    pub fn set_managed_vpc_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_ManagedVpcResource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_vpc_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_vpc_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `self_managed_lattice_resource`.\n"]
    pub fn set_self_managed_lattice_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_SelfManagedLatticeResource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.self_managed_lattice_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.self_managed_lattice_resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint {}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint {
            managed_vpc_resource: core::default::Default::default(),
            self_managed_lattice_resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `managed_vpc_resource` after provisioning.\n"]
    pub fn managed_vpc_resource(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_ManagedVpcResourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_vpc_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `self_managed_lattice_resource` after provisioning.\n"]
    pub fn self_managed_lattice_resource(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint_SelfManagedLatticeResourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_managed_lattice_resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointOverrideDynamic {
    private_endpoint: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointOverride {
    domain: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint,
        >,
    >,
    dynamic: BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointOverrideDynamic,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointOverride {
    #[doc = "Set the field `private_endpoint`.\n"]
    pub fn set_private_endpoint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_endpoint = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointOverride {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointOverride,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointOverride {
    #[doc = ""]
    pub domain: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointOverride {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointOverride {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointOverride {
            domain: self.domain,
            private_endpoint: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointOverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointOverrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointOverrideRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointOverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointOverrideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `private_endpoint` after provisioning.\n"]
    pub fn private_endpoint(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource_KmsKeySource {
    kms_key_arn: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource_KmsKeySource {

}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource_KmsKeySource {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource_KmsKeySource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource_KmsKeySource {
    #[doc = ""]
    pub kms_key_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource_KmsKeySource {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource_KmsKeySource {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource_KmsKeySource {
            kms_key_arn: self.kms_key_arn,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource_KmsKeySourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource_KmsKeySourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource_KmsKeySourceRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource_KmsKeySourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource_KmsKeySourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySourceDynamic {
    kms_key_source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource_KmsKeySource,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource_KmsKeySource,
        >,
    >,
    dynamic: BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySourceDynamic,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource {
    #[doc = "Set the field `kms_key_source`.\n"]
    pub fn set_kms_key_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource_KmsKeySource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kms_key_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kms_key_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource {}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource {
            kms_key_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySourceRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_source` after provisioning.\n"]
    pub fn kms_key_source(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource_KmsKeySourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kms_key_source", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfigDynamic {
    private_key_source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_header_claims: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_payload_claims: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signing_algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key_source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource,
        >,
    >,
    dynamic: BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfigDynamic,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig {
    #[doc = "Set the field `additional_header_claims`.\n"]
    pub fn set_additional_header_claims(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.additional_header_claims = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_payload_claims`.\n"]
    pub fn set_additional_payload_claims(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.additional_payload_claims = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `signing_algorithm`.\n"]
    pub fn set_signing_algorithm(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.signing_algorithm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_key_source`.\n"]
    pub fn set_private_key_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_key_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_key_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig {}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig {
            additional_header_claims: core::default::Default::default(),
            additional_payload_claims: core::default::Default::default(),
            signing_algorithm: core::default::Default::default(),
            private_key_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfigRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_header_claims` after provisioning.\n"]
    pub fn additional_header_claims(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.additional_header_claims", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_payload_claims` after provisioning.\n"]
    pub fn additional_payload_claims(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.additional_payload_claims", self.base))
    }

    #[doc = "Get a reference to the value of field `signing_algorithm` after provisioning.\n"]
    pub fn signing_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signing_algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `private_key_source` after provisioning.\n"]
    pub fn private_key_source(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig_PrivateKeySourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_key_source", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfigDynamic {
    client_secret_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
        >,
    >,
    oauth_discovery: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery,
        >,
    >,
    on_behalf_of_token_exchange_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig,
        >,
    >,
    private_endpoint: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint,
        >,
    >,
    private_endpoint_override: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointOverride,
        >,
    >,
    private_key_jwt_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_authentication_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_credentials_wo_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_discovery: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_behalf_of_token_exchange_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint_override: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointOverride,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key_jwt_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig,
        >,
    >,
    dynamic: BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfigDynamic,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig {
    #[doc = "Set the field `client_authentication_method`.\n"]
    pub fn set_client_authentication_method(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_authentication_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_credentials_wo_version`.\n"]
    pub fn set_client_credentials_wo_version(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.client_credentials_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id_wo`.\n"]
    pub fn set_client_id_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_source`.\n"]
    pub fn set_client_secret_source(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_secret_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_wo`.\n"]
    pub fn set_client_secret_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_config`.\n"]
    pub fn set_client_secret_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_secret_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_secret_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth_discovery`.\n"]
    pub fn set_oauth_discovery(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscovery,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth_discovery = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_discovery = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `on_behalf_of_token_exchange_config`.\n"]
    pub fn set_on_behalf_of_token_exchange_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.on_behalf_of_token_exchange_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.on_behalf_of_token_exchange_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_endpoint`.\n"]
    pub fn set_private_endpoint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpoint,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_endpoint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_endpoint_override`.\n"]
    pub fn set_private_endpoint_override(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointOverride,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_endpoint_override = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_endpoint_override = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_key_jwt_config`.\n"]
    pub fn set_private_key_jwt_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_key_jwt_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_key_jwt_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig {
    type O =
        terrars
        ::BlockAssignable<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig {}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig {
    pub fn build(self) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig {
            client_authentication_method: core::default::Default::default(),
            client_credentials_wo_version: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_id_wo: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            client_secret_source: core::default::Default::default(),
            client_secret_wo: core::default::Default::default(),
            client_secret_config: core::default::Default::default(),
            oauth_discovery: core::default::Default::default(),
            on_behalf_of_token_exchange_config: core::default::Default::default(),
            private_endpoint: core::default::Default::default(),
            private_endpoint_override: core::default::Default::default(),
            private_key_jwt_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfigRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_authentication_method` after provisioning.\n"]
    pub fn client_authentication_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_authentication_method", self.base))
    }

    #[doc = "Get a reference to the value of field `client_credentials_wo_version` after provisioning.\n"]
    pub fn client_credentials_wo_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_credentials_wo_version", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id_wo` after provisioning.\n"]
    pub fn client_id_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_source` after provisioning.\n"]
    pub fn client_secret_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_source", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_wo` after provisioning.\n"]
    pub fn client_secret_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_config` after provisioning.\n"]
    pub fn client_secret_config(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_secret_config", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_discovery` after provisioning.\n"]
    pub fn oauth_discovery(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OauthDiscoveryRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_discovery", self.base))
    }

    #[doc = "Get a reference to the value of field `on_behalf_of_token_exchange_config` after provisioning.\n"]
    pub fn on_behalf_of_token_exchange_config(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_OnBehalfOfTokenExchangeConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_behalf_of_token_exchange_config", self.base))
    }

    #[doc = "Get a reference to the value of field `private_endpoint` after provisioning.\n"]
    pub fn private_endpoint(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `private_endpoint_override` after provisioning.\n"]
    pub fn private_endpoint_override(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateEndpointOverrideRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoint_override", self.base))
    }

    #[doc = "Get a reference to the value of field `private_key_jwt_config` after provisioning.\n"]
    pub fn private_key_jwt_config(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig_PrivateKeyJwtConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_key_jwt_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GithubOauth2ProviderConfigDynamic {
    client_secret_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GithubOauth2ProviderConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_credentials_wo_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
        >,
    >,
    dynamic: BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GithubOauth2ProviderConfigDynamic,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GithubOauth2ProviderConfig {
    #[doc = "Set the field `client_credentials_wo_version`.\n"]
    pub fn set_client_credentials_wo_version(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.client_credentials_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id_wo`.\n"]
    pub fn set_client_id_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_source`.\n"]
    pub fn set_client_secret_source(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_secret_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_wo`.\n"]
    pub fn set_client_secret_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_config`.\n"]
    pub fn set_client_secret_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_secret_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_secret_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GithubOauth2ProviderConfig {
    type O =
        terrars
        ::BlockAssignable<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GithubOauth2ProviderConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GithubOauth2ProviderConfig {}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GithubOauth2ProviderConfig {
    pub fn build(self) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GithubOauth2ProviderConfig {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GithubOauth2ProviderConfig {
            client_credentials_wo_version: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_id_wo: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            client_secret_source: core::default::Default::default(),
            client_secret_wo: core::default::Default::default(),
            client_secret_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GithubOauth2ProviderConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GithubOauth2ProviderConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GithubOauth2ProviderConfigRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GithubOauth2ProviderConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GithubOauth2ProviderConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_credentials_wo_version` after provisioning.\n"]
    pub fn client_credentials_wo_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_credentials_wo_version", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id_wo` after provisioning.\n"]
    pub fn client_id_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_source` after provisioning.\n"]
    pub fn client_secret_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_source", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_wo` after provisioning.\n"]
    pub fn client_secret_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_discovery` after provisioning.\n"]
    pub fn oauth_discovery(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscoveryRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_discovery", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_config` after provisioning.\n"]
    pub fn client_secret_config(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_secret_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GoogleOauth2ProviderConfigDynamic {
    client_secret_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GoogleOauth2ProviderConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_credentials_wo_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
        >,
    >,
    dynamic: BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GoogleOauth2ProviderConfigDynamic,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GoogleOauth2ProviderConfig {
    #[doc = "Set the field `client_credentials_wo_version`.\n"]
    pub fn set_client_credentials_wo_version(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.client_credentials_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id_wo`.\n"]
    pub fn set_client_id_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_source`.\n"]
    pub fn set_client_secret_source(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_secret_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_wo`.\n"]
    pub fn set_client_secret_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_config`.\n"]
    pub fn set_client_secret_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_secret_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_secret_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GoogleOauth2ProviderConfig {
    type O =
        terrars
        ::BlockAssignable<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GoogleOauth2ProviderConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GoogleOauth2ProviderConfig {}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GoogleOauth2ProviderConfig {
    pub fn build(self) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GoogleOauth2ProviderConfig {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GoogleOauth2ProviderConfig {
            client_credentials_wo_version: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_id_wo: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            client_secret_source: core::default::Default::default(),
            client_secret_wo: core::default::Default::default(),
            client_secret_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GoogleOauth2ProviderConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GoogleOauth2ProviderConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GoogleOauth2ProviderConfigRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GoogleOauth2ProviderConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GoogleOauth2ProviderConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_credentials_wo_version` after provisioning.\n"]
    pub fn client_credentials_wo_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_credentials_wo_version", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id_wo` after provisioning.\n"]
    pub fn client_id_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_source` after provisioning.\n"]
    pub fn client_secret_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_source", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_wo` after provisioning.\n"]
    pub fn client_secret_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_discovery` after provisioning.\n"]
    pub fn oauth_discovery(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscoveryRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_discovery", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_config` after provisioning.\n"]
    pub fn client_secret_config(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_secret_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_IncludedOauth2ProviderConfigDynamic {
    client_secret_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_IncludedOauth2ProviderConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_credentials_wo_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
        >,
    >,
    dynamic: BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_IncludedOauth2ProviderConfigDynamic,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_IncludedOauth2ProviderConfig {
    #[doc = "Set the field `authorization_endpoint`.\n"]
    pub fn set_authorization_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authorization_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_credentials_wo_version`.\n"]
    pub fn set_client_credentials_wo_version(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.client_credentials_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id_wo`.\n"]
    pub fn set_client_id_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_source`.\n"]
    pub fn set_client_secret_source(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_secret_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_wo`.\n"]
    pub fn set_client_secret_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_endpoint`.\n"]
    pub fn set_token_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_config`.\n"]
    pub fn set_client_secret_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_secret_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_secret_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_IncludedOauth2ProviderConfig {
    type O =
        terrars
        ::BlockAssignable<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_IncludedOauth2ProviderConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_IncludedOauth2ProviderConfig {}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_IncludedOauth2ProviderConfig {
    pub fn build(self) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_IncludedOauth2ProviderConfig {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_IncludedOauth2ProviderConfig {
            authorization_endpoint: core::default::Default::default(),
            client_credentials_wo_version: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_id_wo: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            client_secret_source: core::default::Default::default(),
            client_secret_wo: core::default::Default::default(),
            issuer: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
            client_secret_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_IncludedOauth2ProviderConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_IncludedOauth2ProviderConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_IncludedOauth2ProviderConfigRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_IncludedOauth2ProviderConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_IncludedOauth2ProviderConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `client_credentials_wo_version` after provisioning.\n"]
    pub fn client_credentials_wo_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_credentials_wo_version", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id_wo` after provisioning.\n"]
    pub fn client_id_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_source` after provisioning.\n"]
    pub fn client_secret_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_source", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_wo` after provisioning.\n"]
    pub fn client_secret_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_discovery` after provisioning.\n"]
    pub fn oauth_discovery(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscoveryRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_discovery", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_config` after provisioning.\n"]
    pub fn client_secret_config(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_secret_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_LinkedinOauth2ProviderConfigDynamic {
    client_secret_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_LinkedinOauth2ProviderConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_credentials_wo_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
        >,
    >,
    dynamic: BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_LinkedinOauth2ProviderConfigDynamic,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_LinkedinOauth2ProviderConfig {
    #[doc = "Set the field `client_credentials_wo_version`.\n"]
    pub fn set_client_credentials_wo_version(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.client_credentials_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id_wo`.\n"]
    pub fn set_client_id_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_source`.\n"]
    pub fn set_client_secret_source(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_secret_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_wo`.\n"]
    pub fn set_client_secret_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_config`.\n"]
    pub fn set_client_secret_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_secret_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_secret_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_LinkedinOauth2ProviderConfig {
    type O =
        terrars
        ::BlockAssignable<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_LinkedinOauth2ProviderConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_LinkedinOauth2ProviderConfig {}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_LinkedinOauth2ProviderConfig {
    pub fn build(self) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_LinkedinOauth2ProviderConfig {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_LinkedinOauth2ProviderConfig {
            client_credentials_wo_version: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_id_wo: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            client_secret_source: core::default::Default::default(),
            client_secret_wo: core::default::Default::default(),
            client_secret_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_LinkedinOauth2ProviderConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_LinkedinOauth2ProviderConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_LinkedinOauth2ProviderConfigRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_LinkedinOauth2ProviderConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_LinkedinOauth2ProviderConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_credentials_wo_version` after provisioning.\n"]
    pub fn client_credentials_wo_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_credentials_wo_version", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id_wo` after provisioning.\n"]
    pub fn client_id_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_source` after provisioning.\n"]
    pub fn client_secret_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_source", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_wo` after provisioning.\n"]
    pub fn client_secret_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_discovery` after provisioning.\n"]
    pub fn oauth_discovery(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscoveryRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_discovery", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_config` after provisioning.\n"]
    pub fn client_secret_config(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_secret_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_MicrosoftOauth2ProviderConfigDynamic {
    client_secret_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_MicrosoftOauth2ProviderConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_credentials_wo_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_id_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_id_wo_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
        >,
    >,
    dynamic: BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_MicrosoftOauth2ProviderConfigDynamic,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_MicrosoftOauth2ProviderConfig {
    #[doc = "Set the field `client_credentials_wo_version`.\n"]
    pub fn set_client_credentials_wo_version(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.client_credentials_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id_wo`.\n"]
    pub fn set_client_id_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_source`.\n"]
    pub fn set_client_secret_source(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_secret_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_wo`.\n"]
    pub fn set_client_secret_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenant_id`.\n"]
    pub fn set_tenant_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenant_id_wo`.\n"]
    pub fn set_tenant_id_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant_id_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenant_id_wo_version`.\n"]
    pub fn set_tenant_id_wo_version(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tenant_id_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_config`.\n"]
    pub fn set_client_secret_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_secret_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_secret_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_MicrosoftOauth2ProviderConfig {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_MicrosoftOauth2ProviderConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_MicrosoftOauth2ProviderConfig {}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_MicrosoftOauth2ProviderConfig {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_MicrosoftOauth2ProviderConfig {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_MicrosoftOauth2ProviderConfig {
            client_credentials_wo_version: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_id_wo: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            client_secret_source: core::default::Default::default(),
            client_secret_wo: core::default::Default::default(),
            tenant_id: core::default::Default::default(),
            tenant_id_wo: core::default::Default::default(),
            tenant_id_wo_version: core::default::Default::default(),
            client_secret_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_MicrosoftOauth2ProviderConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_MicrosoftOauth2ProviderConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_MicrosoftOauth2ProviderConfigRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_MicrosoftOauth2ProviderConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_MicrosoftOauth2ProviderConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_credentials_wo_version` after provisioning.\n"]
    pub fn client_credentials_wo_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_credentials_wo_version", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id_wo` after provisioning.\n"]
    pub fn client_id_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_source` after provisioning.\n"]
    pub fn client_secret_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_source", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_wo` after provisioning.\n"]
    pub fn client_secret_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_discovery` after provisioning.\n"]
    pub fn oauth_discovery(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscoveryRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_discovery", self.base))
    }

    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\n"]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }

    #[doc = "Get a reference to the value of field `tenant_id_wo` after provisioning.\n"]
    pub fn tenant_id_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `tenant_id_wo_version` after provisioning.\n"]
    pub fn tenant_id_wo_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id_wo_version", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_config` after provisioning.\n"]
    pub fn client_secret_config(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_secret_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SalesforceOauth2ProviderConfigDynamic {
    client_secret_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SalesforceOauth2ProviderConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_credentials_wo_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
        >,
    >,
    dynamic: BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SalesforceOauth2ProviderConfigDynamic,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SalesforceOauth2ProviderConfig {
    #[doc = "Set the field `client_credentials_wo_version`.\n"]
    pub fn set_client_credentials_wo_version(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.client_credentials_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id_wo`.\n"]
    pub fn set_client_id_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_source`.\n"]
    pub fn set_client_secret_source(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_secret_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_wo`.\n"]
    pub fn set_client_secret_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_config`.\n"]
    pub fn set_client_secret_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_secret_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_secret_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SalesforceOauth2ProviderConfig {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SalesforceOauth2ProviderConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SalesforceOauth2ProviderConfig {}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SalesforceOauth2ProviderConfig {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SalesforceOauth2ProviderConfig {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SalesforceOauth2ProviderConfig {
            client_credentials_wo_version: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_id_wo: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            client_secret_source: core::default::Default::default(),
            client_secret_wo: core::default::Default::default(),
            client_secret_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SalesforceOauth2ProviderConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SalesforceOauth2ProviderConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SalesforceOauth2ProviderConfigRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SalesforceOauth2ProviderConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SalesforceOauth2ProviderConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_credentials_wo_version` after provisioning.\n"]
    pub fn client_credentials_wo_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_credentials_wo_version", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id_wo` after provisioning.\n"]
    pub fn client_id_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_source` after provisioning.\n"]
    pub fn client_secret_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_source", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_wo` after provisioning.\n"]
    pub fn client_secret_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_discovery` after provisioning.\n"]
    pub fn oauth_discovery(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscoveryRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_discovery", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_config` after provisioning.\n"]
    pub fn client_secret_config(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_secret_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfigDynamic {
    client_secret_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_credentials_wo_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
        >,
    >,
    dynamic: BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfigDynamic,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig {
    #[doc = "Set the field `client_credentials_wo_version`.\n"]
    pub fn set_client_credentials_wo_version(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.client_credentials_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id_wo`.\n"]
    pub fn set_client_id_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_source`.\n"]
    pub fn set_client_secret_source(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_secret_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_wo`.\n"]
    pub fn set_client_secret_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_config`.\n"]
    pub fn set_client_secret_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_secret_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_secret_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig {
    type O =
        terrars
        ::BlockAssignable<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig {}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig {
    pub fn build(self) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig {
            client_credentials_wo_version: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_id_wo: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            client_secret_source: core::default::Default::default(),
            client_secret_wo: core::default::Default::default(),
            client_secret_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfigRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_credentials_wo_version` after provisioning.\n"]
    pub fn client_credentials_wo_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_credentials_wo_version", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id_wo` after provisioning.\n"]
    pub fn client_id_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_source` after provisioning.\n"]
    pub fn client_secret_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_source", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_wo` after provisioning.\n"]
    pub fn client_secret_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_discovery` after provisioning.\n"]
    pub fn oauth_discovery(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_OauthDiscoveryRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_discovery", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_config` after provisioning.\n"]
    pub fn client_secret_config(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig_ClientSecretConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_secret_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfigDynamic {
    atlassian_oauth2_provider_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_AtlassianOauth2ProviderConfig>,
    >,
    custom_oauth2_provider_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig>,
    >,
    github_oauth2_provider_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GithubOauth2ProviderConfig>,
    >,
    google_oauth2_provider_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GoogleOauth2ProviderConfig>,
    >,
    included_oauth2_provider_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_IncludedOauth2ProviderConfig>,
    >,
    linkedin_oauth2_provider_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_LinkedinOauth2ProviderConfig>,
    >,
    microsoft_oauth2_provider_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_MicrosoftOauth2ProviderConfig>,
    >,
    salesforce_oauth2_provider_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SalesforceOauth2ProviderConfig>,
    >,
    slack_oauth2_provider_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    atlassian_oauth2_provider_config: std
    ::option
    ::Option<
        std::vec::Vec<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_AtlassianOauth2ProviderConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_oauth2_provider_config: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github_oauth2_provider_config: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GithubOauth2ProviderConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_oauth2_provider_config: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GoogleOauth2ProviderConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_oauth2_provider_config: std
    ::option
    ::Option<
        std::vec::Vec<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_IncludedOauth2ProviderConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linkedin_oauth2_provider_config: std
    ::option
    ::Option<
        std::vec::Vec<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_LinkedinOauth2ProviderConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    microsoft_oauth2_provider_config: std
    ::option
    ::Option<
        std::vec::Vec<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_MicrosoftOauth2ProviderConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    salesforce_oauth2_provider_config: std
    ::option
    ::Option<
        std::vec::Vec<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SalesforceOauth2ProviderConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slack_oauth2_provider_config: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig>>,
    dynamic: BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfigDynamic,
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig {
    #[doc = "Set the field `atlassian_oauth2_provider_config`.\n"]
    pub fn set_atlassian_oauth2_provider_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_AtlassianOauth2ProviderConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.atlassian_oauth2_provider_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.atlassian_oauth2_provider_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_oauth2_provider_config`.\n"]
    pub fn set_custom_oauth2_provider_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_oauth2_provider_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_oauth2_provider_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `github_oauth2_provider_config`.\n"]
    pub fn set_github_oauth2_provider_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GithubOauth2ProviderConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.github_oauth2_provider_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.github_oauth2_provider_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_oauth2_provider_config`.\n"]
    pub fn set_google_oauth2_provider_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GoogleOauth2ProviderConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_oauth2_provider_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_oauth2_provider_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `included_oauth2_provider_config`.\n"]
    pub fn set_included_oauth2_provider_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_IncludedOauth2ProviderConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.included_oauth2_provider_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.included_oauth2_provider_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `linkedin_oauth2_provider_config`.\n"]
    pub fn set_linkedin_oauth2_provider_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_LinkedinOauth2ProviderConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.linkedin_oauth2_provider_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.linkedin_oauth2_provider_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `microsoft_oauth2_provider_config`.\n"]
    pub fn set_microsoft_oauth2_provider_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_MicrosoftOauth2ProviderConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.microsoft_oauth2_provider_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.microsoft_oauth2_provider_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `salesforce_oauth2_provider_config`.\n"]
    pub fn set_salesforce_oauth2_provider_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SalesforceOauth2ProviderConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.salesforce_oauth2_provider_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.salesforce_oauth2_provider_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `slack_oauth2_provider_config`.\n"]
    pub fn set_slack_oauth2_provider_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.slack_oauth2_provider_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.slack_oauth2_provider_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig {
    type O = terrars::BlockAssignable<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig {}

impl BuildBedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig {
    pub fn build(self) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig {
            atlassian_oauth2_provider_config: core::default::Default::default(),
            custom_oauth2_provider_config: core::default::Default::default(),
            github_oauth2_provider_config: core::default::Default::default(),
            google_oauth2_provider_config: core::default::Default::default(),
            included_oauth2_provider_config: core::default::Default::default(),
            linkedin_oauth2_provider_config: core::default::Default::default(),
            microsoft_oauth2_provider_config: core::default::Default::default(),
            salesforce_oauth2_provider_config: core::default::Default::default(),
            slack_oauth2_provider_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfigRef {
        BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `atlassian_oauth2_provider_config` after provisioning.\n"]
    pub fn atlassian_oauth2_provider_config(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_AtlassianOauth2ProviderConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.atlassian_oauth2_provider_config", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_oauth2_provider_config` after provisioning.\n"]
    pub fn custom_oauth2_provider_config(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_CustomOauth2ProviderConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_oauth2_provider_config", self.base))
    }

    #[doc = "Get a reference to the value of field `github_oauth2_provider_config` after provisioning.\n"]
    pub fn github_oauth2_provider_config(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GithubOauth2ProviderConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github_oauth2_provider_config", self.base))
    }

    #[doc = "Get a reference to the value of field `google_oauth2_provider_config` after provisioning.\n"]
    pub fn google_oauth2_provider_config(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_GoogleOauth2ProviderConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_oauth2_provider_config", self.base))
    }

    #[doc = "Get a reference to the value of field `included_oauth2_provider_config` after provisioning.\n"]
    pub fn included_oauth2_provider_config(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_IncludedOauth2ProviderConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_oauth2_provider_config", self.base))
    }

    #[doc = "Get a reference to the value of field `linkedin_oauth2_provider_config` after provisioning.\n"]
    pub fn linkedin_oauth2_provider_config(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_LinkedinOauth2ProviderConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linkedin_oauth2_provider_config", self.base))
    }

    #[doc = "Get a reference to the value of field `microsoft_oauth2_provider_config` after provisioning.\n"]
    pub fn microsoft_oauth2_provider_config(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_MicrosoftOauth2ProviderConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.microsoft_oauth2_provider_config", self.base))
    }

    #[doc = "Get a reference to the value of field `salesforce_oauth2_provider_config` after provisioning.\n"]
    pub fn salesforce_oauth2_provider_config(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SalesforceOauth2ProviderConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.salesforce_oauth2_provider_config", self.base))
    }

    #[doc = "Get a reference to the value of field `slack_oauth2_provider_config` after provisioning.\n"]
    pub fn slack_oauth2_provider_config(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreOauth2CredentialProvider_Oauth2ProviderConfig_SlackOauth2ProviderConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slack_oauth2_provider_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProvider_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreOauth2CredentialProvider_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreOauth2CredentialProvider_Timeouts {
    type O = terrars::BlockAssignable<BedrockagentcoreOauth2CredentialProvider_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider_Timeouts {}

impl BuildBedrockagentcoreOauth2CredentialProvider_Timeouts {
    pub fn build(self) -> BedrockagentcoreOauth2CredentialProvider_Timeouts {
        BedrockagentcoreOauth2CredentialProvider_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOauth2CredentialProvider_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOauth2CredentialProvider_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOauth2CredentialProvider_TimeoutsRef {
        BedrockagentcoreOauth2CredentialProvider_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProvider_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
