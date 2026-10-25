#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OsisPipelineData_ {
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
    max_units: terrars::PrimField<f64>,
    min_units: terrars::PrimField<f64>,
    pipeline_configuration_body: terrars::PrimField<std::string::String>,
    pipeline_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffer_options: std::option::Option<std::vec::Vec<OsisPipeline_BufferOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_at_rest_options: std::option::Option<std::vec::Vec<OsisPipeline_EncryptionAtRestOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_publishing_options: std::option::Option<std::vec::Vec<OsisPipeline_LogPublishingOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OsisPipeline_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_options: std::option::Option<std::vec::Vec<OsisPipeline_VpcOptions>>,
    dynamic: OsisPipelineDynamic,
}

struct OsisPipeline_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OsisPipelineData_>,
}

#[derive(Clone)]
pub struct OsisPipeline(std::rc::Rc<OsisPipeline_>);

impl OsisPipeline {
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

    #[doc = "Set the field `pipeline_role_arn`.\n"]
    pub fn set_pipeline_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().pipeline_role_arn = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `buffer_options`.\n"]
    pub fn set_buffer_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsisPipeline_BufferOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().buffer_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.buffer_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_at_rest_options`.\n"]
    pub fn set_encryption_at_rest_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsisPipeline_EncryptionAtRestOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_at_rest_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_at_rest_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_publishing_options`.\n"]
    pub fn set_log_publishing_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsisPipeline_LogPublishingOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_publishing_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_publishing_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<OsisPipeline_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_options`.\n"]
    pub fn set_vpc_options(self, v: impl std::convert::Into<terrars::BlockAssignable<OsisPipeline_VpcOptions>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingest_endpoint_urls` after provisioning.\n"]
    pub fn ingest_endpoint_urls(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ingest_endpoint_urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_units` after provisioning.\n"]
    pub fn max_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_units` after provisioning.\n"]
    pub fn min_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_arn` after provisioning.\n"]
    pub fn pipeline_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_configuration_body` after provisioning.\n"]
    pub fn pipeline_configuration_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_configuration_body", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_name` after provisioning.\n"]
    pub fn pipeline_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_role_arn` after provisioning.\n"]
    pub fn pipeline_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_role_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `buffer_options` after provisioning.\n"]
    pub fn buffer_options(&self) -> terrars::ListRef<OsisPipeline_BufferOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.buffer_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_at_rest_options` after provisioning.\n"]
    pub fn encryption_at_rest_options(&self) -> terrars::ListRef<OsisPipeline_EncryptionAtRestOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_at_rest_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_publishing_options` after provisioning.\n"]
    pub fn log_publishing_options(&self) -> terrars::ListRef<OsisPipeline_LogPublishingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_publishing_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OsisPipeline_TimeoutsRef {
        OsisPipeline_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_options` after provisioning.\n"]
    pub fn vpc_options(&self) -> terrars::ListRef<OsisPipeline_VpcOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_options", self.extract_ref()))
    }
}

impl terrars::Referable for OsisPipeline {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OsisPipeline { }

impl terrars::ToListMappable for OsisPipeline {
    type O = terrars::ListRef<OsisPipelineRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OsisPipeline_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_osis_pipeline".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOsisPipeline {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub max_units: terrars::PrimField<f64>,
    #[doc = ""]
    pub min_units: terrars::PrimField<f64>,
    #[doc = ""]
    pub pipeline_configuration_body: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub pipeline_name: terrars::PrimField<std::string::String>,
}

impl BuildOsisPipeline {
    pub fn build(self, stack: &mut terrars::Stack) -> OsisPipeline {
        let out = OsisPipeline(std::rc::Rc::new(OsisPipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OsisPipelineData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                max_units: self.max_units,
                min_units: self.min_units,
                pipeline_configuration_body: self.pipeline_configuration_body,
                pipeline_name: self.pipeline_name,
                pipeline_role_arn: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                buffer_options: core::default::Default::default(),
                encryption_at_rest_options: core::default::Default::default(),
                log_publishing_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_options: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OsisPipelineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsisPipelineRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OsisPipelineRef {
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

    #[doc = "Get a reference to the value of field `ingest_endpoint_urls` after provisioning.\n"]
    pub fn ingest_endpoint_urls(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ingest_endpoint_urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_units` after provisioning.\n"]
    pub fn max_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_units` after provisioning.\n"]
    pub fn min_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_arn` after provisioning.\n"]
    pub fn pipeline_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_configuration_body` after provisioning.\n"]
    pub fn pipeline_configuration_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_configuration_body", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_name` after provisioning.\n"]
    pub fn pipeline_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_role_arn` after provisioning.\n"]
    pub fn pipeline_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_role_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `buffer_options` after provisioning.\n"]
    pub fn buffer_options(&self) -> terrars::ListRef<OsisPipeline_BufferOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.buffer_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_at_rest_options` after provisioning.\n"]
    pub fn encryption_at_rest_options(&self) -> terrars::ListRef<OsisPipeline_EncryptionAtRestOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_at_rest_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_publishing_options` after provisioning.\n"]
    pub fn log_publishing_options(&self) -> terrars::ListRef<OsisPipeline_LogPublishingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_publishing_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OsisPipeline_TimeoutsRef {
        OsisPipeline_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_options` after provisioning.\n"]
    pub fn vpc_options(&self) -> terrars::ListRef<OsisPipeline_VpcOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_options", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct OsisPipelineDynamic {
    buffer_options: std::option::Option<terrars::DynamicBlock<OsisPipeline_BufferOptions>>,
    encryption_at_rest_options: std::option::Option<terrars::DynamicBlock<OsisPipeline_EncryptionAtRestOptions>>,
    log_publishing_options: std::option::Option<terrars::DynamicBlock<OsisPipeline_LogPublishingOptions>>,
    vpc_options: std::option::Option<terrars::DynamicBlock<OsisPipeline_VpcOptions>>,
}

#[derive(Serialize)]
pub struct OsisPipeline_BufferOptions {
    persistent_buffer_enabled: terrars::PrimField<bool>,
}

impl OsisPipeline_BufferOptions { }

impl terrars::ToListMappable for OsisPipeline_BufferOptions {
    type O = terrars::BlockAssignable<OsisPipeline_BufferOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsisPipeline_BufferOptions {
    #[doc = ""]
    pub persistent_buffer_enabled: terrars::PrimField<bool>,
}

impl BuildOsisPipeline_BufferOptions {
    pub fn build(self) -> OsisPipeline_BufferOptions {
        OsisPipeline_BufferOptions { persistent_buffer_enabled: self.persistent_buffer_enabled }
    }
}

#[allow(dead_code)]
pub struct OsisPipeline_BufferOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsisPipeline_BufferOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OsisPipeline_BufferOptionsRef {
        OsisPipeline_BufferOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsisPipeline_BufferOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `persistent_buffer_enabled` after provisioning.\n"]
    pub fn persistent_buffer_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.persistent_buffer_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct OsisPipeline_EncryptionAtRestOptions {
    kms_key_arn: terrars::PrimField<std::string::String>,
}

impl OsisPipeline_EncryptionAtRestOptions { }

impl terrars::ToListMappable for OsisPipeline_EncryptionAtRestOptions {
    type O = terrars::BlockAssignable<OsisPipeline_EncryptionAtRestOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsisPipeline_EncryptionAtRestOptions {
    #[doc = ""]
    pub kms_key_arn: terrars::PrimField<std::string::String>,
}

impl BuildOsisPipeline_EncryptionAtRestOptions {
    pub fn build(self) -> OsisPipeline_EncryptionAtRestOptions {
        OsisPipeline_EncryptionAtRestOptions { kms_key_arn: self.kms_key_arn }
    }
}

#[allow(dead_code)]
pub struct OsisPipeline_EncryptionAtRestOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsisPipeline_EncryptionAtRestOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OsisPipeline_EncryptionAtRestOptionsRef {
        OsisPipeline_EncryptionAtRestOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsisPipeline_EncryptionAtRestOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct OsisPipeline_LogPublishingOptions_CloudwatchLogDestination {
    log_group: terrars::PrimField<std::string::String>,
}

impl OsisPipeline_LogPublishingOptions_CloudwatchLogDestination { }

impl terrars::ToListMappable for OsisPipeline_LogPublishingOptions_CloudwatchLogDestination {
    type O = terrars::BlockAssignable<OsisPipeline_LogPublishingOptions_CloudwatchLogDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsisPipeline_LogPublishingOptions_CloudwatchLogDestination {
    #[doc = ""]
    pub log_group: terrars::PrimField<std::string::String>,
}

impl BuildOsisPipeline_LogPublishingOptions_CloudwatchLogDestination {
    pub fn build(self) -> OsisPipeline_LogPublishingOptions_CloudwatchLogDestination {
        OsisPipeline_LogPublishingOptions_CloudwatchLogDestination { log_group: self.log_group }
    }
}

#[allow(dead_code)]
pub struct OsisPipeline_LogPublishingOptions_CloudwatchLogDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsisPipeline_LogPublishingOptions_CloudwatchLogDestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsisPipeline_LogPublishingOptions_CloudwatchLogDestinationRef {
        OsisPipeline_LogPublishingOptions_CloudwatchLogDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsisPipeline_LogPublishingOptions_CloudwatchLogDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group` after provisioning.\n"]
    pub fn log_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsisPipeline_LogPublishingOptionsDynamic {
    cloudwatch_log_destination: std
    ::option
    ::Option<terrars::DynamicBlock<OsisPipeline_LogPublishingOptions_CloudwatchLogDestination>>,
}

#[derive(Serialize)]
pub struct OsisPipeline_LogPublishingOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_logging_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_log_destination: std
    ::option
    ::Option<std::vec::Vec<OsisPipeline_LogPublishingOptions_CloudwatchLogDestination>>,
    dynamic: OsisPipeline_LogPublishingOptionsDynamic,
}

impl OsisPipeline_LogPublishingOptions {
    #[doc = "Set the field `is_logging_enabled`.\n"]
    pub fn set_is_logging_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_logging_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_log_destination`.\n"]
    pub fn set_cloudwatch_log_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OsisPipeline_LogPublishingOptions_CloudwatchLogDestination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_log_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_log_destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsisPipeline_LogPublishingOptions {
    type O = terrars::BlockAssignable<OsisPipeline_LogPublishingOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsisPipeline_LogPublishingOptions {}

impl BuildOsisPipeline_LogPublishingOptions {
    pub fn build(self) -> OsisPipeline_LogPublishingOptions {
        OsisPipeline_LogPublishingOptions {
            is_logging_enabled: core::default::Default::default(),
            cloudwatch_log_destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsisPipeline_LogPublishingOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsisPipeline_LogPublishingOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OsisPipeline_LogPublishingOptionsRef {
        OsisPipeline_LogPublishingOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsisPipeline_LogPublishingOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_logging_enabled` after provisioning.\n"]
    pub fn is_logging_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_logging_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_log_destination` after provisioning.\n"]
    pub fn cloudwatch_log_destination(
        &self,
    ) -> terrars::ListRef<OsisPipeline_LogPublishingOptions_CloudwatchLogDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_log_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct OsisPipeline_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OsisPipeline_Timeouts {
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

impl terrars::ToListMappable for OsisPipeline_Timeouts {
    type O = terrars::BlockAssignable<OsisPipeline_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsisPipeline_Timeouts {}

impl BuildOsisPipeline_Timeouts {
    pub fn build(self) -> OsisPipeline_Timeouts {
        OsisPipeline_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsisPipeline_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsisPipeline_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OsisPipeline_TimeoutsRef {
        OsisPipeline_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsisPipeline_TimeoutsRef {
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

#[derive(Serialize)]
pub struct OsisPipeline_VpcOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_endpoint_management: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OsisPipeline_VpcOptions {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_endpoint_management`.\n"]
    pub fn set_vpc_endpoint_management(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vpc_endpoint_management = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsisPipeline_VpcOptions {
    type O = terrars::BlockAssignable<OsisPipeline_VpcOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsisPipeline_VpcOptions {
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildOsisPipeline_VpcOptions {
    pub fn build(self) -> OsisPipeline_VpcOptions {
        OsisPipeline_VpcOptions {
            security_group_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
            vpc_endpoint_management: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OsisPipeline_VpcOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsisPipeline_VpcOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OsisPipeline_VpcOptionsRef {
        OsisPipeline_VpcOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsisPipeline_VpcOptionsRef {
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

    #[doc = "Get a reference to the value of field `vpc_endpoint_management` after provisioning.\n"]
    pub fn vpc_endpoint_management(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_management", self.base))
    }
}
