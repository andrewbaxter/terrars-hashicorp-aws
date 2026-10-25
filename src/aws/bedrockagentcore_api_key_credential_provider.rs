#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentcoreApiKeyCredentialProviderData_ {
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
    api_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_secret_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_wo_version: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_secret_config: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretConfig>>,
    dynamic: BedrockagentcoreApiKeyCredentialProviderDynamic,
}

struct BedrockagentcoreApiKeyCredentialProvider_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockagentcoreApiKeyCredentialProviderData_>,
}

#[derive(Clone)]
pub struct BedrockagentcoreApiKeyCredentialProvider(std::rc::Rc<BedrockagentcoreApiKeyCredentialProvider_>);

impl BedrockagentcoreApiKeyCredentialProvider {
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

    #[doc = "Set the field `api_key`.\n"]
    pub fn set_api_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_key_secret_source`.\n"]
    pub fn set_api_key_secret_source(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().api_key_secret_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_key_wo`.\n"]
    pub fn set_api_key_wo(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().api_key_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_key_wo_version`.\n"]
    pub fn set_api_key_wo_version(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().api_key_wo_version = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `api_key_secret_config`.\n"]
    pub fn set_api_key_secret_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().api_key_secret_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.api_key_secret_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_key_secret_arn` after provisioning.\n"]
    pub fn api_key_secret_arn(&self) -> terrars::ListRef<BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretArnRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_secret_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_key_secret_source` after provisioning.\n"]
    pub fn api_key_secret_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_secret_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_key_wo` after provisioning.\n"]
    pub fn api_key_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_wo", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_key_wo_version` after provisioning.\n"]
    pub fn api_key_wo_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_wo_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credential_provider_arn` after provisioning.\n"]
    pub fn credential_provider_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credential_provider_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `api_key_secret_config` after provisioning.\n"]
    pub fn api_key_secret_config(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_secret_config", self.extract_ref()))
    }
}

impl terrars::Referable for BedrockagentcoreApiKeyCredentialProvider {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockagentcoreApiKeyCredentialProvider { }

impl terrars::ToListMappable for BedrockagentcoreApiKeyCredentialProvider {
    type O = terrars::ListRef<BedrockagentcoreApiKeyCredentialProviderRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockagentcoreApiKeyCredentialProvider_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrockagentcore_api_key_credential_provider".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentcoreApiKeyCredentialProvider {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreApiKeyCredentialProvider {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockagentcoreApiKeyCredentialProvider {
        let out =
            BedrockagentcoreApiKeyCredentialProvider(std::rc::Rc::new(BedrockagentcoreApiKeyCredentialProvider_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(BedrockagentcoreApiKeyCredentialProviderData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    provisioner: core::default::Default::default(),
                    api_key: core::default::Default::default(),
                    api_key_secret_source: core::default::Default::default(),
                    api_key_wo: core::default::Default::default(),
                    api_key_wo_version: core::default::Default::default(),
                    name: self.name,
                    region: core::default::Default::default(),
                    tags: core::default::Default::default(),
                    api_key_secret_config: core::default::Default::default(),
                    dynamic: core::default::Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentcoreApiKeyCredentialProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreApiKeyCredentialProviderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockagentcoreApiKeyCredentialProviderRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_key_secret_arn` after provisioning.\n"]
    pub fn api_key_secret_arn(&self) -> terrars::ListRef<BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretArnRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_secret_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_key_secret_source` after provisioning.\n"]
    pub fn api_key_secret_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_secret_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_key_wo` after provisioning.\n"]
    pub fn api_key_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_wo", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_key_wo_version` after provisioning.\n"]
    pub fn api_key_wo_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_wo_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credential_provider_arn` after provisioning.\n"]
    pub fn credential_provider_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credential_provider_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `api_key_secret_config` after provisioning.\n"]
    pub fn api_key_secret_config(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_secret_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreApiKeyCredentialProviderDynamic {
    api_key_secret_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretConfig>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretArn {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretArn {
    #[doc = "Set the field `secret_arn`.\n"]
    pub fn set_secret_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretArn {
    type O = terrars::BlockAssignable<BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretArn>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreApiKeyCredentialProvider_ApiKeySecretArn {}

impl BuildBedrockagentcoreApiKeyCredentialProvider_ApiKeySecretArn {
    pub fn build(self) -> BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretArn {
        BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretArn { secret_arn: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretArnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretArnRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretArnRef {
        BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretArnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretArnRef {
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
pub struct BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretConfig {
    json_key: terrars::PrimField<std::string::String>,
    secret_id: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretConfig { }

impl terrars::ToListMappable for BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretConfig {
    type O = terrars::BlockAssignable<BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreApiKeyCredentialProvider_ApiKeySecretConfig {
    #[doc = ""]
    pub json_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub secret_id: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreApiKeyCredentialProvider_ApiKeySecretConfig {
    pub fn build(self) -> BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretConfig {
        BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretConfig {
            json_key: self.json_key,
            secret_id: self.secret_id,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretConfigRef {
        BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreApiKeyCredentialProvider_ApiKeySecretConfigRef {
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
