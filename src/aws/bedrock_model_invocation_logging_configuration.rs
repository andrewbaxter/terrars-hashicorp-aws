#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockModelInvocationLoggingConfigurationData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<BedrockModelInvocationLoggingConfiguration_LoggingConfig>>,
    dynamic: BedrockModelInvocationLoggingConfigurationDynamic,
}

struct BedrockModelInvocationLoggingConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockModelInvocationLoggingConfigurationData_>,
}

#[derive(Clone)]
pub struct BedrockModelInvocationLoggingConfiguration(std::rc::Rc<BedrockModelInvocationLoggingConfiguration_>);

impl BedrockModelInvocationLoggingConfiguration {
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

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockModelInvocationLoggingConfiguration_LoggingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
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

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<BedrockModelInvocationLoggingConfiguration_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }
}

impl terrars::Referable for BedrockModelInvocationLoggingConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockModelInvocationLoggingConfiguration { }

impl terrars::ToListMappable for BedrockModelInvocationLoggingConfiguration {
    type O = terrars::ListRef<BedrockModelInvocationLoggingConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockModelInvocationLoggingConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrock_model_invocation_logging_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockModelInvocationLoggingConfiguration {
    pub tf_id: std::string::String,
}

impl BuildBedrockModelInvocationLoggingConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockModelInvocationLoggingConfiguration {
        let out =
            BedrockModelInvocationLoggingConfiguration(std::rc::Rc::new(BedrockModelInvocationLoggingConfiguration_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(BedrockModelInvocationLoggingConfigurationData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    provisioner: core::default::Default::default(),
                    region: core::default::Default::default(),
                    logging_config: core::default::Default::default(),
                    dynamic: core::default::Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockModelInvocationLoggingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockModelInvocationLoggingConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockModelInvocationLoggingConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<BedrockModelInvocationLoggingConfiguration_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BedrockModelInvocationLoggingConfigurationDynamic {
    logging_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockModelInvocationLoggingConfiguration_LoggingConfig>>,
}

#[derive(Serialize)]
pub struct BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig_LargeDataDeliveryS3Config {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig_LargeDataDeliveryS3Config {
    #[doc = "Set the field `key_prefix`.\n"]
    pub fn set_key_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig_LargeDataDeliveryS3Config {
    type O =
        terrars
        ::BlockAssignable<
            BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig_LargeDataDeliveryS3Config,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig_LargeDataDeliveryS3Config {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig_LargeDataDeliveryS3Config {
    pub fn build(
        self,
    ) -> BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig_LargeDataDeliveryS3Config {
        BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig_LargeDataDeliveryS3Config {
            bucket_name: self.bucket_name,
            key_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig_LargeDataDeliveryS3ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig_LargeDataDeliveryS3ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig_LargeDataDeliveryS3ConfigRef {
        BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig_LargeDataDeliveryS3ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig_LargeDataDeliveryS3ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `key_prefix` after provisioning.\n"]
    pub fn key_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfigDynamic {
    large_data_delivery_s3_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig_LargeDataDeliveryS3Config,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig {
    log_group_name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    large_data_delivery_s3_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig_LargeDataDeliveryS3Config>,
    >,
    dynamic: BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfigDynamic,
}

impl BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig {
    #[doc = "Set the field `large_data_delivery_s3_config`.\n"]
    pub fn set_large_data_delivery_s3_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig_LargeDataDeliveryS3Config,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.large_data_delivery_s3_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.large_data_delivery_s3_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig {
    type O = terrars::BlockAssignable<BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig {
    #[doc = ""]
    pub log_group_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig {
    pub fn build(self) -> BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig {
        BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig {
            log_group_name: self.log_group_name,
            role_arn: self.role_arn,
            large_data_delivery_s3_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfigRef {
        BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `large_data_delivery_s3_config` after provisioning.\n"]
    pub fn large_data_delivery_s3_config(
        &self,
    ) -> terrars
    ::ListRef<BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig_LargeDataDeliveryS3ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.large_data_delivery_s3_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockModelInvocationLoggingConfiguration_LoggingConfig_S3Config {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockModelInvocationLoggingConfiguration_LoggingConfig_S3Config {
    #[doc = "Set the field `key_prefix`.\n"]
    pub fn set_key_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockModelInvocationLoggingConfiguration_LoggingConfig_S3Config {
    type O = terrars::BlockAssignable<BedrockModelInvocationLoggingConfiguration_LoggingConfig_S3Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockModelInvocationLoggingConfiguration_LoggingConfig_S3Config {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockModelInvocationLoggingConfiguration_LoggingConfig_S3Config {
    pub fn build(self) -> BedrockModelInvocationLoggingConfiguration_LoggingConfig_S3Config {
        BedrockModelInvocationLoggingConfiguration_LoggingConfig_S3Config {
            bucket_name: self.bucket_name,
            key_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockModelInvocationLoggingConfiguration_LoggingConfig_S3ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockModelInvocationLoggingConfiguration_LoggingConfig_S3ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockModelInvocationLoggingConfiguration_LoggingConfig_S3ConfigRef {
        BedrockModelInvocationLoggingConfiguration_LoggingConfig_S3ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockModelInvocationLoggingConfiguration_LoggingConfig_S3ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `key_prefix` after provisioning.\n"]
    pub fn key_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockModelInvocationLoggingConfiguration_LoggingConfigDynamic {
    cloudwatch_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig>>,
    s3_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockModelInvocationLoggingConfiguration_LoggingConfig_S3Config>>,
}

#[derive(Serialize)]
pub struct BedrockModelInvocationLoggingConfiguration_LoggingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    embedding_data_delivery_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_data_delivery_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_data_delivery_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video_data_delivery_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_config: std
    ::option
    ::Option<std::vec::Vec<BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_config: std::option::Option<std::vec::Vec<BedrockModelInvocationLoggingConfiguration_LoggingConfig_S3Config>>,
    dynamic: BedrockModelInvocationLoggingConfiguration_LoggingConfigDynamic,
}

impl BedrockModelInvocationLoggingConfiguration_LoggingConfig {
    #[doc = "Set the field `embedding_data_delivery_enabled`.\n"]
    pub fn set_embedding_data_delivery_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.embedding_data_delivery_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_data_delivery_enabled`.\n"]
    pub fn set_image_data_delivery_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.image_data_delivery_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text_data_delivery_enabled`.\n"]
    pub fn set_text_data_delivery_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.text_data_delivery_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `video_data_delivery_enabled`.\n"]
    pub fn set_video_data_delivery_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.video_data_delivery_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_config`.\n"]
    pub fn set_cloudwatch_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_config`.\n"]
    pub fn set_s3_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockModelInvocationLoggingConfiguration_LoggingConfig_S3Config>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockModelInvocationLoggingConfiguration_LoggingConfig {
    type O = terrars::BlockAssignable<BedrockModelInvocationLoggingConfiguration_LoggingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockModelInvocationLoggingConfiguration_LoggingConfig {}

impl BuildBedrockModelInvocationLoggingConfiguration_LoggingConfig {
    pub fn build(self) -> BedrockModelInvocationLoggingConfiguration_LoggingConfig {
        BedrockModelInvocationLoggingConfiguration_LoggingConfig {
            embedding_data_delivery_enabled: core::default::Default::default(),
            image_data_delivery_enabled: core::default::Default::default(),
            text_data_delivery_enabled: core::default::Default::default(),
            video_data_delivery_enabled: core::default::Default::default(),
            cloudwatch_config: core::default::Default::default(),
            s3_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockModelInvocationLoggingConfiguration_LoggingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockModelInvocationLoggingConfiguration_LoggingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockModelInvocationLoggingConfiguration_LoggingConfigRef {
        BedrockModelInvocationLoggingConfiguration_LoggingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockModelInvocationLoggingConfiguration_LoggingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `embedding_data_delivery_enabled` after provisioning.\n"]
    pub fn embedding_data_delivery_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.embedding_data_delivery_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `image_data_delivery_enabled` after provisioning.\n"]
    pub fn image_data_delivery_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_data_delivery_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `text_data_delivery_enabled` after provisioning.\n"]
    pub fn text_data_delivery_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text_data_delivery_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `video_data_delivery_enabled` after provisioning.\n"]
    pub fn video_data_delivery_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.video_data_delivery_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_config` after provisioning.\n"]
    pub fn cloudwatch_config(
        &self,
    ) -> terrars::ListRef<BedrockModelInvocationLoggingConfiguration_LoggingConfig_CloudwatchConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_config", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_config` after provisioning.\n"]
    pub fn s3_config(&self) -> terrars::ListRef<BedrockModelInvocationLoggingConfiguration_LoggingConfig_S3ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_config", self.base))
    }
}
