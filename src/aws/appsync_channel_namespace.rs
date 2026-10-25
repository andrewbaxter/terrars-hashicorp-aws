#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppsyncChannelNamespaceData_ {
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
    api_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_handlers: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    handler_configs: std::option::Option<std::vec::Vec<AppsyncChannelNamespace_HandlerConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_auth_mode: std::option::Option<std::vec::Vec<AppsyncChannelNamespace_PublishAuthMode>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscribe_auth_mode: std::option::Option<std::vec::Vec<AppsyncChannelNamespace_SubscribeAuthMode>>,
    dynamic: AppsyncChannelNamespaceDynamic,
}

struct AppsyncChannelNamespace_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppsyncChannelNamespaceData_>,
}

#[derive(Clone)]
pub struct AppsyncChannelNamespace(std::rc::Rc<AppsyncChannelNamespace_>);

impl AppsyncChannelNamespace {
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

    #[doc = "Set the field `code_handlers`.\n"]
    pub fn set_code_handlers(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().code_handlers = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `handler_configs`.\n"]
    pub fn set_handler_configs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncChannelNamespace_HandlerConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().handler_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.handler_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `publish_auth_mode`.\n"]
    pub fn set_publish_auth_mode(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncChannelNamespace_PublishAuthMode>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().publish_auth_mode = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.publish_auth_mode = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `subscribe_auth_mode`.\n"]
    pub fn set_subscribe_auth_mode(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncChannelNamespace_SubscribeAuthMode>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subscribe_auth_mode = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subscribe_auth_mode = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `channel_namespace_arn` after provisioning.\n"]
    pub fn channel_namespace_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel_namespace_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code_handlers` after provisioning.\n"]
    pub fn code_handlers(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_handlers", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `handler_configs` after provisioning.\n"]
    pub fn handler_configs(&self) -> terrars::ListRef<AppsyncChannelNamespace_HandlerConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.handler_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publish_auth_mode` after provisioning.\n"]
    pub fn publish_auth_mode(&self) -> terrars::ListRef<AppsyncChannelNamespace_PublishAuthModeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publish_auth_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subscribe_auth_mode` after provisioning.\n"]
    pub fn subscribe_auth_mode(&self) -> terrars::ListRef<AppsyncChannelNamespace_SubscribeAuthModeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subscribe_auth_mode", self.extract_ref()))
    }
}

impl terrars::Referable for AppsyncChannelNamespace {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppsyncChannelNamespace { }

impl terrars::ToListMappable for AppsyncChannelNamespace {
    type O = terrars::ListRef<AppsyncChannelNamespaceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppsyncChannelNamespace_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_appsync_channel_namespace".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppsyncChannelNamespace {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub api_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncChannelNamespace {
    pub fn build(self, stack: &mut terrars::Stack) -> AppsyncChannelNamespace {
        let out = AppsyncChannelNamespace(std::rc::Rc::new(AppsyncChannelNamespace_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppsyncChannelNamespaceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                api_id: self.api_id,
                code_handlers: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                handler_configs: core::default::Default::default(),
                publish_auth_mode: core::default::Default::default(),
                subscribe_auth_mode: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppsyncChannelNamespaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncChannelNamespaceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppsyncChannelNamespaceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `channel_namespace_arn` after provisioning.\n"]
    pub fn channel_namespace_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel_namespace_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code_handlers` after provisioning.\n"]
    pub fn code_handlers(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_handlers", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `handler_configs` after provisioning.\n"]
    pub fn handler_configs(&self) -> terrars::ListRef<AppsyncChannelNamespace_HandlerConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.handler_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publish_auth_mode` after provisioning.\n"]
    pub fn publish_auth_mode(&self) -> terrars::ListRef<AppsyncChannelNamespace_PublishAuthModeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publish_auth_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subscribe_auth_mode` after provisioning.\n"]
    pub fn subscribe_auth_mode(&self) -> terrars::ListRef<AppsyncChannelNamespace_SubscribeAuthModeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subscribe_auth_mode", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AppsyncChannelNamespaceDynamic {
    handler_configs: std::option::Option<terrars::DynamicBlock<AppsyncChannelNamespace_HandlerConfigs>>,
    publish_auth_mode: std::option::Option<terrars::DynamicBlock<AppsyncChannelNamespace_PublishAuthMode>>,
    subscribe_auth_mode: std::option::Option<terrars::DynamicBlock<AppsyncChannelNamespace_SubscribeAuthMode>>,
}

#[derive(Serialize)]
pub struct AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration_LambdaConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invoke_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration_LambdaConfig {
    #[doc = "Set the field `invoke_type`.\n"]
    pub fn set_invoke_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.invoke_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration_LambdaConfig {
    type O = terrars::BlockAssignable<AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration_LambdaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration_LambdaConfig {}

impl BuildAppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration_LambdaConfig {
    pub fn build(self) -> AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration_LambdaConfig {
        AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration_LambdaConfig {
            invoke_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration_LambdaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration_LambdaConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration_LambdaConfigRef {
        AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration_LambdaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration_LambdaConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `invoke_type` after provisioning.\n"]
    pub fn invoke_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invoke_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppsyncChannelNamespace_HandlerConfigs_OnPublish_IntegrationDynamic {
    lambda_config: std
    ::option
    ::Option<terrars::DynamicBlock<AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration_LambdaConfig>>,
}

#[derive(Serialize)]
pub struct AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration {
    data_source_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_config: std
    ::option
    ::Option<std::vec::Vec<AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration_LambdaConfig>>,
    dynamic: AppsyncChannelNamespace_HandlerConfigs_OnPublish_IntegrationDynamic,
}

impl AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration {
    #[doc = "Set the field `lambda_config`.\n"]
    pub fn set_lambda_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration_LambdaConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration {
    type O = terrars::BlockAssignable<AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration {
    #[doc = ""]
    pub data_source_name: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration {
    pub fn build(self) -> AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration {
        AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration {
            data_source_name: self.data_source_name,
            lambda_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncChannelNamespace_HandlerConfigs_OnPublish_IntegrationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncChannelNamespace_HandlerConfigs_OnPublish_IntegrationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppsyncChannelNamespace_HandlerConfigs_OnPublish_IntegrationRef {
        AppsyncChannelNamespace_HandlerConfigs_OnPublish_IntegrationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncChannelNamespace_HandlerConfigs_OnPublish_IntegrationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_source_name` after provisioning.\n"]
    pub fn data_source_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_name", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_config` after provisioning.\n"]
    pub fn lambda_config(
        &self,
    ) -> terrars::ListRef<AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration_LambdaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppsyncChannelNamespace_HandlerConfigs_OnPublishDynamic {
    integration: std
    ::option
    ::Option<terrars::DynamicBlock<AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration>>,
}

#[derive(Serialize)]
pub struct AppsyncChannelNamespace_HandlerConfigs_OnPublish {
    behavior: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integration: std::option::Option<std::vec::Vec<AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration>>,
    dynamic: AppsyncChannelNamespace_HandlerConfigs_OnPublishDynamic,
}

impl AppsyncChannelNamespace_HandlerConfigs_OnPublish {
    #[doc = "Set the field `integration`.\n"]
    pub fn set_integration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.integration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.integration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppsyncChannelNamespace_HandlerConfigs_OnPublish {
    type O = terrars::BlockAssignable<AppsyncChannelNamespace_HandlerConfigs_OnPublish>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncChannelNamespace_HandlerConfigs_OnPublish {
    #[doc = ""]
    pub behavior: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncChannelNamespace_HandlerConfigs_OnPublish {
    pub fn build(self) -> AppsyncChannelNamespace_HandlerConfigs_OnPublish {
        AppsyncChannelNamespace_HandlerConfigs_OnPublish {
            behavior: self.behavior,
            integration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncChannelNamespace_HandlerConfigs_OnPublishRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncChannelNamespace_HandlerConfigs_OnPublishRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppsyncChannelNamespace_HandlerConfigs_OnPublishRef {
        AppsyncChannelNamespace_HandlerConfigs_OnPublishRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncChannelNamespace_HandlerConfigs_OnPublishRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `behavior` after provisioning.\n"]
    pub fn behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `integration` after provisioning.\n"]
    pub fn integration(&self) -> terrars::ListRef<AppsyncChannelNamespace_HandlerConfigs_OnPublish_IntegrationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integration", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppsyncChannelNamespace_HandlerConfigs_OnSubscribeDynamic {
    integration: std
    ::option
    ::Option<terrars::DynamicBlock<AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration>>,
}

#[derive(Serialize)]
pub struct AppsyncChannelNamespace_HandlerConfigs_OnSubscribe {
    behavior: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integration: std::option::Option<std::vec::Vec<AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration>>,
    dynamic: AppsyncChannelNamespace_HandlerConfigs_OnSubscribeDynamic,
}

impl AppsyncChannelNamespace_HandlerConfigs_OnSubscribe {
    #[doc = "Set the field `integration`.\n"]
    pub fn set_integration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppsyncChannelNamespace_HandlerConfigs_OnPublish_Integration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.integration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.integration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppsyncChannelNamespace_HandlerConfigs_OnSubscribe {
    type O = terrars::BlockAssignable<AppsyncChannelNamespace_HandlerConfigs_OnSubscribe>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncChannelNamespace_HandlerConfigs_OnSubscribe {
    #[doc = ""]
    pub behavior: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncChannelNamespace_HandlerConfigs_OnSubscribe {
    pub fn build(self) -> AppsyncChannelNamespace_HandlerConfigs_OnSubscribe {
        AppsyncChannelNamespace_HandlerConfigs_OnSubscribe {
            behavior: self.behavior,
            integration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncChannelNamespace_HandlerConfigs_OnSubscribeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncChannelNamespace_HandlerConfigs_OnSubscribeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppsyncChannelNamespace_HandlerConfigs_OnSubscribeRef {
        AppsyncChannelNamespace_HandlerConfigs_OnSubscribeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncChannelNamespace_HandlerConfigs_OnSubscribeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `behavior` after provisioning.\n"]
    pub fn behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `integration` after provisioning.\n"]
    pub fn integration(&self) -> terrars::ListRef<AppsyncChannelNamespace_HandlerConfigs_OnPublish_IntegrationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integration", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppsyncChannelNamespace_HandlerConfigsDynamic {
    on_publish: std::option::Option<terrars::DynamicBlock<AppsyncChannelNamespace_HandlerConfigs_OnPublish>>,
    on_subscribe: std::option::Option<terrars::DynamicBlock<AppsyncChannelNamespace_HandlerConfigs_OnSubscribe>>,
}

#[derive(Serialize)]
pub struct AppsyncChannelNamespace_HandlerConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_publish: std::option::Option<std::vec::Vec<AppsyncChannelNamespace_HandlerConfigs_OnPublish>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_subscribe: std::option::Option<std::vec::Vec<AppsyncChannelNamespace_HandlerConfigs_OnSubscribe>>,
    dynamic: AppsyncChannelNamespace_HandlerConfigsDynamic,
}

impl AppsyncChannelNamespace_HandlerConfigs {
    #[doc = "Set the field `on_publish`.\n"]
    pub fn set_on_publish(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncChannelNamespace_HandlerConfigs_OnPublish>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.on_publish = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.on_publish = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `on_subscribe`.\n"]
    pub fn set_on_subscribe(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncChannelNamespace_HandlerConfigs_OnSubscribe>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.on_subscribe = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.on_subscribe = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppsyncChannelNamespace_HandlerConfigs {
    type O = terrars::BlockAssignable<AppsyncChannelNamespace_HandlerConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncChannelNamespace_HandlerConfigs {}

impl BuildAppsyncChannelNamespace_HandlerConfigs {
    pub fn build(self) -> AppsyncChannelNamespace_HandlerConfigs {
        AppsyncChannelNamespace_HandlerConfigs {
            on_publish: core::default::Default::default(),
            on_subscribe: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncChannelNamespace_HandlerConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncChannelNamespace_HandlerConfigsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncChannelNamespace_HandlerConfigsRef {
        AppsyncChannelNamespace_HandlerConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncChannelNamespace_HandlerConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `on_publish` after provisioning.\n"]
    pub fn on_publish(&self) -> terrars::ListRef<AppsyncChannelNamespace_HandlerConfigs_OnPublishRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_publish", self.base))
    }

    #[doc = "Get a reference to the value of field `on_subscribe` after provisioning.\n"]
    pub fn on_subscribe(&self) -> terrars::ListRef<AppsyncChannelNamespace_HandlerConfigs_OnSubscribeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_subscribe", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncChannelNamespace_PublishAuthMode {
    auth_type: terrars::PrimField<std::string::String>,
}

impl AppsyncChannelNamespace_PublishAuthMode { }

impl terrars::ToListMappable for AppsyncChannelNamespace_PublishAuthMode {
    type O = terrars::BlockAssignable<AppsyncChannelNamespace_PublishAuthMode>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncChannelNamespace_PublishAuthMode {
    #[doc = ""]
    pub auth_type: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncChannelNamespace_PublishAuthMode {
    pub fn build(self) -> AppsyncChannelNamespace_PublishAuthMode {
        AppsyncChannelNamespace_PublishAuthMode { auth_type: self.auth_type }
    }
}

#[allow(dead_code)]
pub struct AppsyncChannelNamespace_PublishAuthModeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncChannelNamespace_PublishAuthModeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncChannelNamespace_PublishAuthModeRef {
        AppsyncChannelNamespace_PublishAuthModeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncChannelNamespace_PublishAuthModeRef {
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
pub struct AppsyncChannelNamespace_SubscribeAuthMode {
    auth_type: terrars::PrimField<std::string::String>,
}

impl AppsyncChannelNamespace_SubscribeAuthMode { }

impl terrars::ToListMappable for AppsyncChannelNamespace_SubscribeAuthMode {
    type O = terrars::BlockAssignable<AppsyncChannelNamespace_SubscribeAuthMode>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncChannelNamespace_SubscribeAuthMode {
    #[doc = ""]
    pub auth_type: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncChannelNamespace_SubscribeAuthMode {
    pub fn build(self) -> AppsyncChannelNamespace_SubscribeAuthMode {
        AppsyncChannelNamespace_SubscribeAuthMode { auth_type: self.auth_type }
    }
}

#[allow(dead_code)]
pub struct AppsyncChannelNamespace_SubscribeAuthModeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncChannelNamespace_SubscribeAuthModeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncChannelNamespace_SubscribeAuthModeRef {
        AppsyncChannelNamespace_SubscribeAuthModeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncChannelNamespace_SubscribeAuthModeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }
}
