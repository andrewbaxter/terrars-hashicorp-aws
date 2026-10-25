#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerEndpointConfigurationData_ {
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
    execution_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    async_inference_config: std::option::Option<std::vec::Vec<SagemakerEndpointConfiguration_AsyncInferenceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_capture_config: std::option::Option<std::vec::Vec<SagemakerEndpointConfiguration_DataCaptureConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    production_variants: std::option::Option<std::vec::Vec<SagemakerEndpointConfiguration_ProductionVariants>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shadow_production_variants: std
    ::option
    ::Option<std::vec::Vec<SagemakerEndpointConfiguration_ShadowProductionVariants>>,
    dynamic: SagemakerEndpointConfigurationDynamic,
}

struct SagemakerEndpointConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerEndpointConfigurationData_>,
}

#[derive(Clone)]
pub struct SagemakerEndpointConfiguration(std::rc::Rc<SagemakerEndpointConfiguration_>);

impl SagemakerEndpointConfiguration {
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

    #[doc = "Set the field `execution_role_arn`.\n"]
    pub fn set_execution_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().execution_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `async_inference_config`.\n"]
    pub fn set_async_inference_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerEndpointConfiguration_AsyncInferenceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().async_inference_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.async_inference_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_capture_config`.\n"]
    pub fn set_data_capture_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerEndpointConfiguration_DataCaptureConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_capture_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_capture_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `production_variants`.\n"]
    pub fn set_production_variants(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerEndpointConfiguration_ProductionVariants>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().production_variants = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.production_variants = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `shadow_production_variants`.\n"]
    pub fn set_shadow_production_variants(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerEndpointConfiguration_ShadowProductionVariants>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().shadow_production_variants = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.shadow_production_variants = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `async_inference_config` after provisioning.\n"]
    pub fn async_inference_config(&self) -> terrars::ListRef<SagemakerEndpointConfiguration_AsyncInferenceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.async_inference_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_capture_config` after provisioning.\n"]
    pub fn data_capture_config(&self) -> terrars::ListRef<SagemakerEndpointConfiguration_DataCaptureConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_capture_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `production_variants` after provisioning.\n"]
    pub fn production_variants(&self) -> terrars::ListRef<SagemakerEndpointConfiguration_ProductionVariantsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.production_variants", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shadow_production_variants` after provisioning.\n"]
    pub fn shadow_production_variants(
        &self,
    ) -> terrars::ListRef<SagemakerEndpointConfiguration_ShadowProductionVariantsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shadow_production_variants", self.extract_ref()))
    }
}

impl terrars::Referable for SagemakerEndpointConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerEndpointConfiguration { }

impl terrars::ToListMappable for SagemakerEndpointConfiguration {
    type O = terrars::ListRef<SagemakerEndpointConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerEndpointConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_endpoint_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerEndpointConfiguration {
    pub tf_id: std::string::String,
}

impl BuildSagemakerEndpointConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerEndpointConfiguration {
        let out = SagemakerEndpointConfiguration(std::rc::Rc::new(SagemakerEndpointConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerEndpointConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                execution_role_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                async_inference_config: core::default::Default::default(),
                data_capture_config: core::default::Default::default(),
                production_variants: core::default::Default::default(),
                shadow_production_variants: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerEndpointConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerEndpointConfigurationRef {
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

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `async_inference_config` after provisioning.\n"]
    pub fn async_inference_config(&self) -> terrars::ListRef<SagemakerEndpointConfiguration_AsyncInferenceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.async_inference_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_capture_config` after provisioning.\n"]
    pub fn data_capture_config(&self) -> terrars::ListRef<SagemakerEndpointConfiguration_DataCaptureConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_capture_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `production_variants` after provisioning.\n"]
    pub fn production_variants(&self) -> terrars::ListRef<SagemakerEndpointConfiguration_ProductionVariantsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.production_variants", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shadow_production_variants` after provisioning.\n"]
    pub fn shadow_production_variants(
        &self,
    ) -> terrars::ListRef<SagemakerEndpointConfiguration_ShadowProductionVariantsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shadow_production_variants", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointConfigurationDynamic {
    async_inference_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpointConfiguration_AsyncInferenceConfig>>,
    data_capture_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpointConfiguration_DataCaptureConfig>>,
    production_variants: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpointConfiguration_ProductionVariants>>,
    shadow_production_variants: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpointConfiguration_ShadowProductionVariants>>,
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfiguration_AsyncInferenceConfig_ClientConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_invocations_per_instance: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerEndpointConfiguration_AsyncInferenceConfig_ClientConfig {
    #[doc = "Set the field `max_concurrent_invocations_per_instance`.\n"]
    pub fn set_max_concurrent_invocations_per_instance(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.max_concurrent_invocations_per_instance = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerEndpointConfiguration_AsyncInferenceConfig_ClientConfig {
    type O = terrars::BlockAssignable<SagemakerEndpointConfiguration_AsyncInferenceConfig_ClientConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfiguration_AsyncInferenceConfig_ClientConfig {}

impl BuildSagemakerEndpointConfiguration_AsyncInferenceConfig_ClientConfig {
    pub fn build(self) -> SagemakerEndpointConfiguration_AsyncInferenceConfig_ClientConfig {
        SagemakerEndpointConfiguration_AsyncInferenceConfig_ClientConfig {
            max_concurrent_invocations_per_instance: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpointConfiguration_AsyncInferenceConfig_ClientConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointConfiguration_AsyncInferenceConfig_ClientConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpointConfiguration_AsyncInferenceConfig_ClientConfigRef {
        SagemakerEndpointConfiguration_AsyncInferenceConfig_ClientConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfiguration_AsyncInferenceConfig_ClientConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_concurrent_invocations_per_instance` after provisioning.\n"]
    pub fn max_concurrent_invocations_per_instance(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrent_invocations_per_instance", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig_NotificationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_topic: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_inference_response_in: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_topic: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig_NotificationConfig {
    #[doc = "Set the field `error_topic`.\n"]
    pub fn set_error_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.error_topic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_inference_response_in`.\n"]
    pub fn set_include_inference_response_in(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_inference_response_in = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `success_topic`.\n"]
    pub fn set_success_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.success_topic = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig_NotificationConfig {
    type O =
        terrars
        ::BlockAssignable<SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig_NotificationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig_NotificationConfig {}

impl BuildSagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig_NotificationConfig {
    pub fn build(self) -> SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig_NotificationConfig {
        SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig_NotificationConfig {
            error_topic: core::default::Default::default(),
            include_inference_response_in: core::default::Default::default(),
            success_topic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig_NotificationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig_NotificationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig_NotificationConfigRef {
        SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig_NotificationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig_NotificationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error_topic` after provisioning.\n"]
    pub fn error_topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_topic", self.base))
    }

    #[doc = "Get a reference to the value of field `include_inference_response_in` after provisioning.\n"]
    pub fn include_inference_response_in(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.include_inference_response_in", self.base))
    }

    #[doc = "Get a reference to the value of field `success_topic` after provisioning.\n"]
    pub fn success_topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_topic", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfigDynamic {
    notification_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig_NotificationConfig>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_failure_path: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_output_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig_NotificationConfig>>,
    dynamic: SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfigDynamic,
}

impl SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_failure_path`.\n"]
    pub fn set_s3_failure_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_failure_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `notification_config`.\n"]
    pub fn set_notification_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig_NotificationConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.notification_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.notification_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig {
    type O = terrars::BlockAssignable<SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig {
    #[doc = ""]
    pub s3_output_path: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig {
    pub fn build(self) -> SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig {
        SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig {
            kms_key_id: core::default::Default::default(),
            s3_failure_path: core::default::Default::default(),
            s3_output_path: self.s3_output_path,
            notification_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfigRef {
        SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_failure_path` after provisioning.\n"]
    pub fn s3_failure_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_failure_path", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_path` after provisioning.\n"]
    pub fn s3_output_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_output_path", self.base))
    }

    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(
        &self,
    ) -> terrars::ListRef<SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig_NotificationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointConfiguration_AsyncInferenceConfigDynamic {
    client_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpointConfiguration_AsyncInferenceConfig_ClientConfig>>,
    output_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfiguration_AsyncInferenceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_config: std::option::Option<std::vec::Vec<SagemakerEndpointConfiguration_AsyncInferenceConfig_ClientConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_config: std::option::Option<std::vec::Vec<SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig>>,
    dynamic: SagemakerEndpointConfiguration_AsyncInferenceConfigDynamic,
}

impl SagemakerEndpointConfiguration_AsyncInferenceConfig {
    #[doc = "Set the field `client_config`.\n"]
    pub fn set_client_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerEndpointConfiguration_AsyncInferenceConfig_ClientConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_config`.\n"]
    pub fn set_output_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerEndpointConfiguration_AsyncInferenceConfig {
    type O = terrars::BlockAssignable<SagemakerEndpointConfiguration_AsyncInferenceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfiguration_AsyncInferenceConfig {}

impl BuildSagemakerEndpointConfiguration_AsyncInferenceConfig {
    pub fn build(self) -> SagemakerEndpointConfiguration_AsyncInferenceConfig {
        SagemakerEndpointConfiguration_AsyncInferenceConfig {
            client_config: core::default::Default::default(),
            output_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpointConfiguration_AsyncInferenceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointConfiguration_AsyncInferenceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpointConfiguration_AsyncInferenceConfigRef {
        SagemakerEndpointConfiguration_AsyncInferenceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfiguration_AsyncInferenceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_config` after provisioning.\n"]
    pub fn client_config(&self) -> terrars::ListRef<SagemakerEndpointConfiguration_AsyncInferenceConfig_ClientConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_config", self.base))
    }

    #[doc = "Get a reference to the value of field `output_config` after provisioning.\n"]
    pub fn output_config(&self) -> terrars::ListRef<SagemakerEndpointConfiguration_AsyncInferenceConfig_OutputConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfiguration_DataCaptureConfig_CaptureContentTypeHeader {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_content_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_content_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl SagemakerEndpointConfiguration_DataCaptureConfig_CaptureContentTypeHeader {
    #[doc = "Set the field `csv_content_types`.\n"]
    pub fn set_csv_content_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.csv_content_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `json_content_types`.\n"]
    pub fn set_json_content_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.json_content_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerEndpointConfiguration_DataCaptureConfig_CaptureContentTypeHeader {
    type O = terrars::BlockAssignable<SagemakerEndpointConfiguration_DataCaptureConfig_CaptureContentTypeHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfiguration_DataCaptureConfig_CaptureContentTypeHeader {}

impl BuildSagemakerEndpointConfiguration_DataCaptureConfig_CaptureContentTypeHeader {
    pub fn build(self) -> SagemakerEndpointConfiguration_DataCaptureConfig_CaptureContentTypeHeader {
        SagemakerEndpointConfiguration_DataCaptureConfig_CaptureContentTypeHeader {
            csv_content_types: core::default::Default::default(),
            json_content_types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpointConfiguration_DataCaptureConfig_CaptureContentTypeHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointConfiguration_DataCaptureConfig_CaptureContentTypeHeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpointConfiguration_DataCaptureConfig_CaptureContentTypeHeaderRef {
        SagemakerEndpointConfiguration_DataCaptureConfig_CaptureContentTypeHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfiguration_DataCaptureConfig_CaptureContentTypeHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `csv_content_types` after provisioning.\n"]
    pub fn csv_content_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.csv_content_types", self.base))
    }

    #[doc = "Get a reference to the value of field `json_content_types` after provisioning.\n"]
    pub fn json_content_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.json_content_types", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfiguration_DataCaptureConfig_CaptureOptions {
    capture_mode: terrars::PrimField<std::string::String>,
}

impl SagemakerEndpointConfiguration_DataCaptureConfig_CaptureOptions { }

impl terrars::ToListMappable for SagemakerEndpointConfiguration_DataCaptureConfig_CaptureOptions {
    type O = terrars::BlockAssignable<SagemakerEndpointConfiguration_DataCaptureConfig_CaptureOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfiguration_DataCaptureConfig_CaptureOptions {
    #[doc = ""]
    pub capture_mode: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerEndpointConfiguration_DataCaptureConfig_CaptureOptions {
    pub fn build(self) -> SagemakerEndpointConfiguration_DataCaptureConfig_CaptureOptions {
        SagemakerEndpointConfiguration_DataCaptureConfig_CaptureOptions { capture_mode: self.capture_mode }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpointConfiguration_DataCaptureConfig_CaptureOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointConfiguration_DataCaptureConfig_CaptureOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpointConfiguration_DataCaptureConfig_CaptureOptionsRef {
        SagemakerEndpointConfiguration_DataCaptureConfig_CaptureOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfiguration_DataCaptureConfig_CaptureOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capture_mode` after provisioning.\n"]
    pub fn capture_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capture_mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointConfiguration_DataCaptureConfigDynamic {
    capture_content_type_header: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpointConfiguration_DataCaptureConfig_CaptureContentTypeHeader>>,
    capture_options: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpointConfiguration_DataCaptureConfig_CaptureOptions>>,
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfiguration_DataCaptureConfig {
    destination_s3_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_capture: std::option::Option<terrars::PrimField<bool>>,
    initial_sampling_percentage: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capture_content_type_header: std
    ::option
    ::Option<std::vec::Vec<SagemakerEndpointConfiguration_DataCaptureConfig_CaptureContentTypeHeader>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capture_options: std::option::Option<std::vec::Vec<SagemakerEndpointConfiguration_DataCaptureConfig_CaptureOptions>>,
    dynamic: SagemakerEndpointConfiguration_DataCaptureConfigDynamic,
}

impl SagemakerEndpointConfiguration_DataCaptureConfig {
    #[doc = "Set the field `enable_capture`.\n"]
    pub fn set_enable_capture(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_capture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capture_content_type_header`.\n"]
    pub fn set_capture_content_type_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerEndpointConfiguration_DataCaptureConfig_CaptureContentTypeHeader>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.capture_content_type_header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.capture_content_type_header = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `capture_options`.\n"]
    pub fn set_capture_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerEndpointConfiguration_DataCaptureConfig_CaptureOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.capture_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.capture_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerEndpointConfiguration_DataCaptureConfig {
    type O = terrars::BlockAssignable<SagemakerEndpointConfiguration_DataCaptureConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfiguration_DataCaptureConfig {
    #[doc = ""]
    pub destination_s3_uri: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub initial_sampling_percentage: terrars::PrimField<f64>,
}

impl BuildSagemakerEndpointConfiguration_DataCaptureConfig {
    pub fn build(self) -> SagemakerEndpointConfiguration_DataCaptureConfig {
        SagemakerEndpointConfiguration_DataCaptureConfig {
            destination_s3_uri: self.destination_s3_uri,
            enable_capture: core::default::Default::default(),
            initial_sampling_percentage: self.initial_sampling_percentage,
            kms_key_id: core::default::Default::default(),
            capture_content_type_header: core::default::Default::default(),
            capture_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpointConfiguration_DataCaptureConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointConfiguration_DataCaptureConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpointConfiguration_DataCaptureConfigRef {
        SagemakerEndpointConfiguration_DataCaptureConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfiguration_DataCaptureConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_s3_uri` after provisioning.\n"]
    pub fn destination_s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_capture` after provisioning.\n"]
    pub fn enable_capture(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_capture", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_sampling_percentage` after provisioning.\n"]
    pub fn initial_sampling_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_sampling_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `capture_content_type_header` after provisioning.\n"]
    pub fn capture_content_type_header(
        &self,
    ) -> terrars::ListRef<SagemakerEndpointConfiguration_DataCaptureConfig_CaptureContentTypeHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capture_content_type_header", self.base))
    }

    #[doc = "Get a reference to the value of field `capture_options` after provisioning.\n"]
    pub fn capture_options(
        &self,
    ) -> terrars::ListRef<SagemakerEndpointConfiguration_DataCaptureConfig_CaptureOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capture_options", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfiguration_ProductionVariants_CapacityReservationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_preference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ml_reservation_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerEndpointConfiguration_ProductionVariants_CapacityReservationConfig {
    #[doc = "Set the field `capacity_reservation_preference`.\n"]
    pub fn set_capacity_reservation_preference(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.capacity_reservation_preference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ml_reservation_arn`.\n"]
    pub fn set_ml_reservation_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ml_reservation_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerEndpointConfiguration_ProductionVariants_CapacityReservationConfig {
    type O = terrars::BlockAssignable<SagemakerEndpointConfiguration_ProductionVariants_CapacityReservationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfiguration_ProductionVariants_CapacityReservationConfig {}

impl BuildSagemakerEndpointConfiguration_ProductionVariants_CapacityReservationConfig {
    pub fn build(self) -> SagemakerEndpointConfiguration_ProductionVariants_CapacityReservationConfig {
        SagemakerEndpointConfiguration_ProductionVariants_CapacityReservationConfig {
            capacity_reservation_preference: core::default::Default::default(),
            ml_reservation_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpointConfiguration_ProductionVariants_CapacityReservationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointConfiguration_ProductionVariants_CapacityReservationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpointConfiguration_ProductionVariants_CapacityReservationConfigRef {
        SagemakerEndpointConfiguration_ProductionVariants_CapacityReservationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfiguration_ProductionVariants_CapacityReservationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_preference` after provisioning.\n"]
    pub fn capacity_reservation_preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_reservation_preference", self.base))
    }

    #[doc = "Get a reference to the value of field `ml_reservation_arn` after provisioning.\n"]
    pub fn ml_reservation_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ml_reservation_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfiguration_ProductionVariants_CoreDumpConfig {
    destination_s3_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerEndpointConfiguration_ProductionVariants_CoreDumpConfig {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerEndpointConfiguration_ProductionVariants_CoreDumpConfig {
    type O = terrars::BlockAssignable<SagemakerEndpointConfiguration_ProductionVariants_CoreDumpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfiguration_ProductionVariants_CoreDumpConfig {
    #[doc = ""]
    pub destination_s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerEndpointConfiguration_ProductionVariants_CoreDumpConfig {
    pub fn build(self) -> SagemakerEndpointConfiguration_ProductionVariants_CoreDumpConfig {
        SagemakerEndpointConfiguration_ProductionVariants_CoreDumpConfig {
            destination_s3_uri: self.destination_s3_uri,
            kms_key_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpointConfiguration_ProductionVariants_CoreDumpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointConfiguration_ProductionVariants_CoreDumpConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpointConfiguration_ProductionVariants_CoreDumpConfigRef {
        SagemakerEndpointConfiguration_ProductionVariants_CoreDumpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfiguration_ProductionVariants_CoreDumpConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_s3_uri` after provisioning.\n"]
    pub fn destination_s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfiguration_ProductionVariants_ManagedInstanceScaling {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerEndpointConfiguration_ProductionVariants_ManagedInstanceScaling {
    #[doc = "Set the field `max_instance_count`.\n"]
    pub fn set_max_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_instance_count`.\n"]
    pub fn set_min_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerEndpointConfiguration_ProductionVariants_ManagedInstanceScaling {
    type O = terrars::BlockAssignable<SagemakerEndpointConfiguration_ProductionVariants_ManagedInstanceScaling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfiguration_ProductionVariants_ManagedInstanceScaling {}

impl BuildSagemakerEndpointConfiguration_ProductionVariants_ManagedInstanceScaling {
    pub fn build(self) -> SagemakerEndpointConfiguration_ProductionVariants_ManagedInstanceScaling {
        SagemakerEndpointConfiguration_ProductionVariants_ManagedInstanceScaling {
            max_instance_count: core::default::Default::default(),
            min_instance_count: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpointConfiguration_ProductionVariants_ManagedInstanceScalingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointConfiguration_ProductionVariants_ManagedInstanceScalingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpointConfiguration_ProductionVariants_ManagedInstanceScalingRef {
        SagemakerEndpointConfiguration_ProductionVariants_ManagedInstanceScalingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfiguration_ProductionVariants_ManagedInstanceScalingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_instance_count` after provisioning.\n"]
    pub fn max_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `min_instance_count` after provisioning.\n"]
    pub fn min_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfiguration_ProductionVariants_RoutingConfig {
    routing_strategy: terrars::PrimField<std::string::String>,
}

impl SagemakerEndpointConfiguration_ProductionVariants_RoutingConfig { }

impl terrars::ToListMappable for SagemakerEndpointConfiguration_ProductionVariants_RoutingConfig {
    type O = terrars::BlockAssignable<SagemakerEndpointConfiguration_ProductionVariants_RoutingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfiguration_ProductionVariants_RoutingConfig {
    #[doc = ""]
    pub routing_strategy: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerEndpointConfiguration_ProductionVariants_RoutingConfig {
    pub fn build(self) -> SagemakerEndpointConfiguration_ProductionVariants_RoutingConfig {
        SagemakerEndpointConfiguration_ProductionVariants_RoutingConfig { routing_strategy: self.routing_strategy }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpointConfiguration_ProductionVariants_RoutingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointConfiguration_ProductionVariants_RoutingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpointConfiguration_ProductionVariants_RoutingConfigRef {
        SagemakerEndpointConfiguration_ProductionVariants_RoutingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfiguration_ProductionVariants_RoutingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `routing_strategy` after provisioning.\n"]
    pub fn routing_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfiguration_ProductionVariants_ServerlessConfig {
    max_concurrency: terrars::PrimField<f64>,
    memory_size_in_mb: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_concurrency: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerEndpointConfiguration_ProductionVariants_ServerlessConfig {
    #[doc = "Set the field `provisioned_concurrency`.\n"]
    pub fn set_provisioned_concurrency(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.provisioned_concurrency = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerEndpointConfiguration_ProductionVariants_ServerlessConfig {
    type O = terrars::BlockAssignable<SagemakerEndpointConfiguration_ProductionVariants_ServerlessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfiguration_ProductionVariants_ServerlessConfig {
    #[doc = ""]
    pub max_concurrency: terrars::PrimField<f64>,
    #[doc = ""]
    pub memory_size_in_mb: terrars::PrimField<f64>,
}

impl BuildSagemakerEndpointConfiguration_ProductionVariants_ServerlessConfig {
    pub fn build(self) -> SagemakerEndpointConfiguration_ProductionVariants_ServerlessConfig {
        SagemakerEndpointConfiguration_ProductionVariants_ServerlessConfig {
            max_concurrency: self.max_concurrency,
            memory_size_in_mb: self.memory_size_in_mb,
            provisioned_concurrency: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpointConfiguration_ProductionVariants_ServerlessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointConfiguration_ProductionVariants_ServerlessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpointConfiguration_ProductionVariants_ServerlessConfigRef {
        SagemakerEndpointConfiguration_ProductionVariants_ServerlessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfiguration_ProductionVariants_ServerlessConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_concurrency` after provisioning.\n"]
    pub fn max_concurrency(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrency", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_size_in_mb` after provisioning.\n"]
    pub fn memory_size_in_mb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_in_mb", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioned_concurrency` after provisioning.\n"]
    pub fn provisioned_concurrency(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_concurrency", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointConfiguration_ProductionVariantsDynamic {
    capacity_reservation_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpointConfiguration_ProductionVariants_CapacityReservationConfig>>,
    core_dump_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpointConfiguration_ProductionVariants_CoreDumpConfig>>,
    managed_instance_scaling: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpointConfiguration_ProductionVariants_ManagedInstanceScaling>>,
    routing_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpointConfiguration_ProductionVariants_RoutingConfig>>,
    serverless_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpointConfiguration_ProductionVariants_ServerlessConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfiguration_ProductionVariants {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_startup_health_check_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_ssm_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_ami_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_variant_weight: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_data_download_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    variant_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size_in_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerEndpointConfiguration_ProductionVariants_CapacityReservationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    core_dump_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerEndpointConfiguration_ProductionVariants_CoreDumpConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_instance_scaling: std
    ::option
    ::Option<std::vec::Vec<SagemakerEndpointConfiguration_ProductionVariants_ManagedInstanceScaling>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_config: std::option::Option<std::vec::Vec<SagemakerEndpointConfiguration_ProductionVariants_RoutingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serverless_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerEndpointConfiguration_ProductionVariants_ServerlessConfig>>,
    dynamic: SagemakerEndpointConfiguration_ProductionVariantsDynamic,
}

impl SagemakerEndpointConfiguration_ProductionVariants {
    #[doc = "Set the field `accelerator_type`.\n"]
    pub fn set_accelerator_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accelerator_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_startup_health_check_timeout_in_seconds`.\n"]
    pub fn set_container_startup_health_check_timeout_in_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.container_startup_health_check_timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_ssm_access`.\n"]
    pub fn set_enable_ssm_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_ssm_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inference_ami_version`.\n"]
    pub fn set_inference_ami_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.inference_ami_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initial_instance_count`.\n"]
    pub fn set_initial_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initial_variant_weight`.\n"]
    pub fn set_initial_variant_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_variant_weight = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_data_download_timeout_in_seconds`.\n"]
    pub fn set_model_data_download_timeout_in_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.model_data_download_timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_name`.\n"]
    pub fn set_model_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `variant_name`.\n"]
    pub fn set_variant_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.variant_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size_in_gb`.\n"]
    pub fn set_volume_size_in_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_size_in_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_reservation_config`.\n"]
    pub fn set_capacity_reservation_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerEndpointConfiguration_ProductionVariants_CapacityReservationConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.capacity_reservation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.capacity_reservation_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `core_dump_config`.\n"]
    pub fn set_core_dump_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerEndpointConfiguration_ProductionVariants_CoreDumpConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.core_dump_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.core_dump_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_instance_scaling`.\n"]
    pub fn set_managed_instance_scaling(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerEndpointConfiguration_ProductionVariants_ManagedInstanceScaling>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_instance_scaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_instance_scaling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `routing_config`.\n"]
    pub fn set_routing_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerEndpointConfiguration_ProductionVariants_RoutingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.routing_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.routing_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `serverless_config`.\n"]
    pub fn set_serverless_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerEndpointConfiguration_ProductionVariants_ServerlessConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.serverless_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.serverless_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerEndpointConfiguration_ProductionVariants {
    type O = terrars::BlockAssignable<SagemakerEndpointConfiguration_ProductionVariants>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfiguration_ProductionVariants {}

impl BuildSagemakerEndpointConfiguration_ProductionVariants {
    pub fn build(self) -> SagemakerEndpointConfiguration_ProductionVariants {
        SagemakerEndpointConfiguration_ProductionVariants {
            accelerator_type: core::default::Default::default(),
            container_startup_health_check_timeout_in_seconds: core::default::Default::default(),
            enable_ssm_access: core::default::Default::default(),
            inference_ami_version: core::default::Default::default(),
            initial_instance_count: core::default::Default::default(),
            initial_variant_weight: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            model_data_download_timeout_in_seconds: core::default::Default::default(),
            model_name: core::default::Default::default(),
            variant_name: core::default::Default::default(),
            volume_size_in_gb: core::default::Default::default(),
            capacity_reservation_config: core::default::Default::default(),
            core_dump_config: core::default::Default::default(),
            managed_instance_scaling: core::default::Default::default(),
            routing_config: core::default::Default::default(),
            serverless_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpointConfiguration_ProductionVariantsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointConfiguration_ProductionVariantsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpointConfiguration_ProductionVariantsRef {
        SagemakerEndpointConfiguration_ProductionVariantsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfiguration_ProductionVariantsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\n"]
    pub fn accelerator_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `container_startup_health_check_timeout_in_seconds` after provisioning.\n"]
    pub fn container_startup_health_check_timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_startup_health_check_timeout_in_seconds", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enable_ssm_access` after provisioning.\n"]
    pub fn enable_ssm_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_ssm_access", self.base))
    }

    #[doc = "Get a reference to the value of field `inference_ami_version` after provisioning.\n"]
    pub fn inference_ami_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inference_ami_version", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_instance_count` after provisioning.\n"]
    pub fn initial_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_variant_weight` after provisioning.\n"]
    pub fn initial_variant_weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_variant_weight", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `model_data_download_timeout_in_seconds` after provisioning.\n"]
    pub fn model_data_download_timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.model_data_download_timeout_in_seconds", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `model_name` after provisioning.\n"]
    pub fn model_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_name", self.base))
    }

    #[doc = "Get a reference to the value of field `variant_name` after provisioning.\n"]
    pub fn variant_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.variant_name", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size_in_gb` after provisioning.\n"]
    pub fn volume_size_in_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_size_in_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_config` after provisioning.\n"]
    pub fn capacity_reservation_config(
        &self,
    ) -> terrars::ListRef<SagemakerEndpointConfiguration_ProductionVariants_CapacityReservationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_reservation_config", self.base))
    }

    #[doc = "Get a reference to the value of field `core_dump_config` after provisioning.\n"]
    pub fn core_dump_config(
        &self,
    ) -> terrars::ListRef<SagemakerEndpointConfiguration_ProductionVariants_CoreDumpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.core_dump_config", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_instance_scaling` after provisioning.\n"]
    pub fn managed_instance_scaling(
        &self,
    ) -> terrars::ListRef<SagemakerEndpointConfiguration_ProductionVariants_ManagedInstanceScalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_instance_scaling", self.base))
    }

    #[doc = "Get a reference to the value of field `routing_config` after provisioning.\n"]
    pub fn routing_config(&self) -> terrars::ListRef<SagemakerEndpointConfiguration_ProductionVariants_RoutingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `serverless_config` after provisioning.\n"]
    pub fn serverless_config(
        &self,
    ) -> terrars::ListRef<SagemakerEndpointConfiguration_ProductionVariants_ServerlessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.serverless_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfiguration_ShadowProductionVariants_CoreDumpConfig {
    destination_s3_uri: terrars::PrimField<std::string::String>,
    kms_key_id: terrars::PrimField<std::string::String>,
}

impl SagemakerEndpointConfiguration_ShadowProductionVariants_CoreDumpConfig { }

impl terrars::ToListMappable for SagemakerEndpointConfiguration_ShadowProductionVariants_CoreDumpConfig {
    type O = terrars::BlockAssignable<SagemakerEndpointConfiguration_ShadowProductionVariants_CoreDumpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfiguration_ShadowProductionVariants_CoreDumpConfig {
    #[doc = ""]
    pub destination_s3_uri: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub kms_key_id: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerEndpointConfiguration_ShadowProductionVariants_CoreDumpConfig {
    pub fn build(self) -> SagemakerEndpointConfiguration_ShadowProductionVariants_CoreDumpConfig {
        SagemakerEndpointConfiguration_ShadowProductionVariants_CoreDumpConfig {
            destination_s3_uri: self.destination_s3_uri,
            kms_key_id: self.kms_key_id,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpointConfiguration_ShadowProductionVariants_CoreDumpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointConfiguration_ShadowProductionVariants_CoreDumpConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpointConfiguration_ShadowProductionVariants_CoreDumpConfigRef {
        SagemakerEndpointConfiguration_ShadowProductionVariants_CoreDumpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfiguration_ShadowProductionVariants_CoreDumpConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_s3_uri` after provisioning.\n"]
    pub fn destination_s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointConfiguration_ShadowProductionVariantsDynamic {
    capacity_reservation_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpointConfiguration_ProductionVariants_CapacityReservationConfig>>,
    core_dump_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpointConfiguration_ShadowProductionVariants_CoreDumpConfig>>,
    managed_instance_scaling: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpointConfiguration_ProductionVariants_ManagedInstanceScaling>>,
    routing_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpointConfiguration_ProductionVariants_RoutingConfig>>,
    serverless_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpointConfiguration_ProductionVariants_ServerlessConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerEndpointConfiguration_ShadowProductionVariants {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_startup_health_check_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_ssm_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_ami_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_variant_weight: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_data_download_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    variant_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size_in_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerEndpointConfiguration_ProductionVariants_CapacityReservationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    core_dump_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerEndpointConfiguration_ShadowProductionVariants_CoreDumpConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_instance_scaling: std
    ::option
    ::Option<std::vec::Vec<SagemakerEndpointConfiguration_ProductionVariants_ManagedInstanceScaling>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_config: std::option::Option<std::vec::Vec<SagemakerEndpointConfiguration_ProductionVariants_RoutingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serverless_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerEndpointConfiguration_ProductionVariants_ServerlessConfig>>,
    dynamic: SagemakerEndpointConfiguration_ShadowProductionVariantsDynamic,
}

impl SagemakerEndpointConfiguration_ShadowProductionVariants {
    #[doc = "Set the field `accelerator_type`.\n"]
    pub fn set_accelerator_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accelerator_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_startup_health_check_timeout_in_seconds`.\n"]
    pub fn set_container_startup_health_check_timeout_in_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.container_startup_health_check_timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_ssm_access`.\n"]
    pub fn set_enable_ssm_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_ssm_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inference_ami_version`.\n"]
    pub fn set_inference_ami_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.inference_ami_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initial_instance_count`.\n"]
    pub fn set_initial_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initial_variant_weight`.\n"]
    pub fn set_initial_variant_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_variant_weight = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_data_download_timeout_in_seconds`.\n"]
    pub fn set_model_data_download_timeout_in_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.model_data_download_timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_name`.\n"]
    pub fn set_model_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `variant_name`.\n"]
    pub fn set_variant_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.variant_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size_in_gb`.\n"]
    pub fn set_volume_size_in_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_size_in_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_reservation_config`.\n"]
    pub fn set_capacity_reservation_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerEndpointConfiguration_ProductionVariants_CapacityReservationConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.capacity_reservation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.capacity_reservation_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `core_dump_config`.\n"]
    pub fn set_core_dump_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerEndpointConfiguration_ShadowProductionVariants_CoreDumpConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.core_dump_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.core_dump_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_instance_scaling`.\n"]
    pub fn set_managed_instance_scaling(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerEndpointConfiguration_ProductionVariants_ManagedInstanceScaling>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_instance_scaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_instance_scaling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `routing_config`.\n"]
    pub fn set_routing_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerEndpointConfiguration_ProductionVariants_RoutingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.routing_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.routing_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `serverless_config`.\n"]
    pub fn set_serverless_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerEndpointConfiguration_ProductionVariants_ServerlessConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.serverless_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.serverless_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerEndpointConfiguration_ShadowProductionVariants {
    type O = terrars::BlockAssignable<SagemakerEndpointConfiguration_ShadowProductionVariants>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointConfiguration_ShadowProductionVariants {}

impl BuildSagemakerEndpointConfiguration_ShadowProductionVariants {
    pub fn build(self) -> SagemakerEndpointConfiguration_ShadowProductionVariants {
        SagemakerEndpointConfiguration_ShadowProductionVariants {
            accelerator_type: core::default::Default::default(),
            container_startup_health_check_timeout_in_seconds: core::default::Default::default(),
            enable_ssm_access: core::default::Default::default(),
            inference_ami_version: core::default::Default::default(),
            initial_instance_count: core::default::Default::default(),
            initial_variant_weight: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            model_data_download_timeout_in_seconds: core::default::Default::default(),
            model_name: core::default::Default::default(),
            variant_name: core::default::Default::default(),
            volume_size_in_gb: core::default::Default::default(),
            capacity_reservation_config: core::default::Default::default(),
            core_dump_config: core::default::Default::default(),
            managed_instance_scaling: core::default::Default::default(),
            routing_config: core::default::Default::default(),
            serverless_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpointConfiguration_ShadowProductionVariantsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointConfiguration_ShadowProductionVariantsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpointConfiguration_ShadowProductionVariantsRef {
        SagemakerEndpointConfiguration_ShadowProductionVariantsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointConfiguration_ShadowProductionVariantsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\n"]
    pub fn accelerator_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `container_startup_health_check_timeout_in_seconds` after provisioning.\n"]
    pub fn container_startup_health_check_timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_startup_health_check_timeout_in_seconds", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enable_ssm_access` after provisioning.\n"]
    pub fn enable_ssm_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_ssm_access", self.base))
    }

    #[doc = "Get a reference to the value of field `inference_ami_version` after provisioning.\n"]
    pub fn inference_ami_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inference_ami_version", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_instance_count` after provisioning.\n"]
    pub fn initial_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_variant_weight` after provisioning.\n"]
    pub fn initial_variant_weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_variant_weight", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `model_data_download_timeout_in_seconds` after provisioning.\n"]
    pub fn model_data_download_timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.model_data_download_timeout_in_seconds", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `model_name` after provisioning.\n"]
    pub fn model_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_name", self.base))
    }

    #[doc = "Get a reference to the value of field `variant_name` after provisioning.\n"]
    pub fn variant_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.variant_name", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size_in_gb` after provisioning.\n"]
    pub fn volume_size_in_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_size_in_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_config` after provisioning.\n"]
    pub fn capacity_reservation_config(
        &self,
    ) -> terrars::ListRef<SagemakerEndpointConfiguration_ProductionVariants_CapacityReservationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_reservation_config", self.base))
    }

    #[doc = "Get a reference to the value of field `core_dump_config` after provisioning.\n"]
    pub fn core_dump_config(
        &self,
    ) -> terrars::ListRef<SagemakerEndpointConfiguration_ShadowProductionVariants_CoreDumpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.core_dump_config", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_instance_scaling` after provisioning.\n"]
    pub fn managed_instance_scaling(
        &self,
    ) -> terrars::ListRef<SagemakerEndpointConfiguration_ProductionVariants_ManagedInstanceScalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_instance_scaling", self.base))
    }

    #[doc = "Get a reference to the value of field `routing_config` after provisioning.\n"]
    pub fn routing_config(&self) -> terrars::ListRef<SagemakerEndpointConfiguration_ProductionVariants_RoutingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `serverless_config` after provisioning.\n"]
    pub fn serverless_config(
        &self,
    ) -> terrars::ListRef<SagemakerEndpointConfiguration_ProductionVariants_ServerlessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.serverless_config", self.base))
    }
}
