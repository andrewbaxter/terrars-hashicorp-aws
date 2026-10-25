#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct KinesisAnalyticsApplicationData_ {
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
    code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_application: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logging_options: std::option::Option<std::vec::Vec<KinesisAnalyticsApplication_CloudwatchLoggingOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inputs: std::option::Option<std::vec::Vec<KinesisAnalyticsApplication_Inputs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outputs: std::option::Option<std::vec::Vec<KinesisAnalyticsApplication_Outputs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reference_data_sources: std::option::Option<std::vec::Vec<KinesisAnalyticsApplication_ReferenceDataSources>>,
    dynamic: KinesisAnalyticsApplicationDynamic,
}

struct KinesisAnalyticsApplication_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<KinesisAnalyticsApplicationData_>,
}

#[derive(Clone)]
pub struct KinesisAnalyticsApplication(std::rc::Rc<KinesisAnalyticsApplication_>);

impl KinesisAnalyticsApplication {
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

    #[doc = "Set the field `code`.\n"]
    pub fn set_code(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `start_application`.\n"]
    pub fn set_start_application(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().start_application = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplication_CloudwatchLoggingOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloudwatch_logging_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloudwatch_logging_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inputs`.\n"]
    pub fn set_inputs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().inputs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.inputs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `outputs`.\n"]
    pub fn set_outputs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplication_Outputs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().outputs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.outputs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `reference_data_sources`.\n"]
    pub fn set_reference_data_sources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplication_ReferenceDataSources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().reference_data_sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.reference_data_sources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_timestamp` after provisioning.\n"]
    pub fn create_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_update_timestamp` after provisioning.\n"]
    pub fn last_update_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_update_timestamp", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `start_application` after provisioning.\n"]
    pub fn start_application(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_application", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> terrars::ListRef<KinesisAnalyticsApplication_CloudwatchLoggingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inputs` after provisioning.\n"]
    pub fn inputs(&self) -> terrars::ListRef<KinesisAnalyticsApplication_InputsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inputs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reference_data_sources` after provisioning.\n"]
    pub fn reference_data_sources(&self) -> terrars::ListRef<KinesisAnalyticsApplication_ReferenceDataSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reference_data_sources", self.extract_ref()))
    }
}

impl terrars::Referable for KinesisAnalyticsApplication {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for KinesisAnalyticsApplication { }

impl terrars::ToListMappable for KinesisAnalyticsApplication {
    type O = terrars::ListRef<KinesisAnalyticsApplicationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for KinesisAnalyticsApplication_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_kinesis_analytics_application".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKinesisAnalyticsApplication {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplication {
    pub fn build(self, stack: &mut terrars::Stack) -> KinesisAnalyticsApplication {
        let out = KinesisAnalyticsApplication(std::rc::Rc::new(KinesisAnalyticsApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(KinesisAnalyticsApplicationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                code: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                start_application: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                cloudwatch_logging_options: core::default::Default::default(),
                inputs: core::default::Default::default(),
                outputs: core::default::Default::default(),
                reference_data_sources: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct KinesisAnalyticsApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl KinesisAnalyticsApplicationRef {
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

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_timestamp` after provisioning.\n"]
    pub fn create_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_update_timestamp` after provisioning.\n"]
    pub fn last_update_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_update_timestamp", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `start_application` after provisioning.\n"]
    pub fn start_application(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_application", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> terrars::ListRef<KinesisAnalyticsApplication_CloudwatchLoggingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inputs` after provisioning.\n"]
    pub fn inputs(&self) -> terrars::ListRef<KinesisAnalyticsApplication_InputsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inputs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reference_data_sources` after provisioning.\n"]
    pub fn reference_data_sources(&self) -> terrars::ListRef<KinesisAnalyticsApplication_ReferenceDataSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reference_data_sources", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationDynamic {
    cloudwatch_logging_options: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_CloudwatchLoggingOptions>>,
    inputs: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_Inputs>>,
    outputs: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_Outputs>>,
    reference_data_sources: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_ReferenceDataSources>>,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplication_CloudwatchLoggingOptions {
    log_stream_arn: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplication_CloudwatchLoggingOptions { }

impl terrars::ToListMappable for KinesisAnalyticsApplication_CloudwatchLoggingOptions {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplication_CloudwatchLoggingOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplication_CloudwatchLoggingOptions {
    #[doc = ""]
    pub log_stream_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplication_CloudwatchLoggingOptions {
    pub fn build(self) -> KinesisAnalyticsApplication_CloudwatchLoggingOptions {
        KinesisAnalyticsApplication_CloudwatchLoggingOptions {
            log_stream_arn: self.log_stream_arn,
            role_arn: self.role_arn,
        }
    }
}

#[allow(dead_code)]
pub struct KinesisAnalyticsApplication_CloudwatchLoggingOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplication_CloudwatchLoggingOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplication_CloudwatchLoggingOptionsRef {
        KinesisAnalyticsApplication_CloudwatchLoggingOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplication_CloudwatchLoggingOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `log_stream_arn` after provisioning.\n"]
    pub fn log_stream_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_stream_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplication_Inputs_KinesisFirehose {
    resource_arn: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplication_Inputs_KinesisFirehose { }

impl terrars::ToListMappable for KinesisAnalyticsApplication_Inputs_KinesisFirehose {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_KinesisFirehose>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplication_Inputs_KinesisFirehose {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplication_Inputs_KinesisFirehose {
    pub fn build(self) -> KinesisAnalyticsApplication_Inputs_KinesisFirehose {
        KinesisAnalyticsApplication_Inputs_KinesisFirehose {
            resource_arn: self.resource_arn,
            role_arn: self.role_arn,
        }
    }
}

#[allow(dead_code)]
pub struct KinesisAnalyticsApplication_Inputs_KinesisFirehoseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplication_Inputs_KinesisFirehoseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplication_Inputs_KinesisFirehoseRef {
        KinesisAnalyticsApplication_Inputs_KinesisFirehoseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplication_Inputs_KinesisFirehoseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplication_Inputs_KinesisStream {
    resource_arn: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplication_Inputs_KinesisStream { }

impl terrars::ToListMappable for KinesisAnalyticsApplication_Inputs_KinesisStream {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_KinesisStream>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplication_Inputs_KinesisStream {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplication_Inputs_KinesisStream {
    pub fn build(self) -> KinesisAnalyticsApplication_Inputs_KinesisStream {
        KinesisAnalyticsApplication_Inputs_KinesisStream {
            resource_arn: self.resource_arn,
            role_arn: self.role_arn,
        }
    }
}

#[allow(dead_code)]
pub struct KinesisAnalyticsApplication_Inputs_KinesisStreamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplication_Inputs_KinesisStreamRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplication_Inputs_KinesisStreamRef {
        KinesisAnalyticsApplication_Inputs_KinesisStreamRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplication_Inputs_KinesisStreamRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplication_Inputs_Parallelism {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
}

impl KinesisAnalyticsApplication_Inputs_Parallelism {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KinesisAnalyticsApplication_Inputs_Parallelism {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_Parallelism>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplication_Inputs_Parallelism {}

impl BuildKinesisAnalyticsApplication_Inputs_Parallelism {
    pub fn build(self) -> KinesisAnalyticsApplication_Inputs_Parallelism {
        KinesisAnalyticsApplication_Inputs_Parallelism { count: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct KinesisAnalyticsApplication_Inputs_ParallelismRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplication_Inputs_ParallelismRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplication_Inputs_ParallelismRef {
        KinesisAnalyticsApplication_Inputs_ParallelismRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplication_Inputs_ParallelismRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplication_Outputs_Lambda {
    resource_arn: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplication_Outputs_Lambda { }

impl terrars::ToListMappable for KinesisAnalyticsApplication_Outputs_Lambda {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplication_Outputs_Lambda>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplication_Outputs_Lambda {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplication_Outputs_Lambda {
    pub fn build(self) -> KinesisAnalyticsApplication_Outputs_Lambda {
        KinesisAnalyticsApplication_Outputs_Lambda {
            resource_arn: self.resource_arn,
            role_arn: self.role_arn,
        }
    }
}

#[allow(dead_code)]
pub struct KinesisAnalyticsApplication_Outputs_LambdaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplication_Outputs_LambdaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KinesisAnalyticsApplication_Outputs_LambdaRef {
        KinesisAnalyticsApplication_Outputs_LambdaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplication_Outputs_LambdaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplication_Inputs_ProcessingConfigurationDynamic {
    lambda: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_Outputs_Lambda>>,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplication_Inputs_ProcessingConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda: std::option::Option<std::vec::Vec<KinesisAnalyticsApplication_Outputs_Lambda>>,
    dynamic: KinesisAnalyticsApplication_Inputs_ProcessingConfigurationDynamic,
}

impl KinesisAnalyticsApplication_Inputs_ProcessingConfiguration {
    #[doc = "Set the field `lambda`.\n"]
    pub fn set_lambda(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplication_Outputs_Lambda>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisAnalyticsApplication_Inputs_ProcessingConfiguration {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_ProcessingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplication_Inputs_ProcessingConfiguration {}

impl BuildKinesisAnalyticsApplication_Inputs_ProcessingConfiguration {
    pub fn build(self) -> KinesisAnalyticsApplication_Inputs_ProcessingConfiguration {
        KinesisAnalyticsApplication_Inputs_ProcessingConfiguration {
            lambda: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisAnalyticsApplication_Inputs_ProcessingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplication_Inputs_ProcessingConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplication_Inputs_ProcessingConfigurationRef {
        KinesisAnalyticsApplication_Inputs_ProcessingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplication_Inputs_ProcessingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda` after provisioning.\n"]
    pub fn lambda(&self) -> terrars::ListRef<KinesisAnalyticsApplication_Outputs_LambdaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplication_Inputs_Schema_RecordColumns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mapping: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    sql_type: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplication_Inputs_Schema_RecordColumns {
    #[doc = "Set the field `mapping`.\n"]
    pub fn set_mapping(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mapping = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KinesisAnalyticsApplication_Inputs_Schema_RecordColumns {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_Schema_RecordColumns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplication_Inputs_Schema_RecordColumns {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sql_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplication_Inputs_Schema_RecordColumns {
    pub fn build(self) -> KinesisAnalyticsApplication_Inputs_Schema_RecordColumns {
        KinesisAnalyticsApplication_Inputs_Schema_RecordColumns {
            mapping: core::default::Default::default(),
            name: self.name,
            sql_type: self.sql_type,
        }
    }
}

#[allow(dead_code)]
pub struct KinesisAnalyticsApplication_Inputs_Schema_RecordColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplication_Inputs_Schema_RecordColumnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplication_Inputs_Schema_RecordColumnsRef {
        KinesisAnalyticsApplication_Inputs_Schema_RecordColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplication_Inputs_Schema_RecordColumnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mapping` after provisioning.\n"]
    pub fn mapping(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_type` after provisioning.\n"]
    pub fn sql_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_type", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Csv {
    record_column_delimiter: terrars::PrimField<std::string::String>,
    record_row_delimiter: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Csv { }

impl terrars::ToListMappable for KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Csv {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Csv>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Csv {
    #[doc = ""]
    pub record_column_delimiter: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub record_row_delimiter: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Csv {
    pub fn build(self) -> KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Csv {
        KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Csv {
            record_column_delimiter: self.record_column_delimiter,
            record_row_delimiter: self.record_row_delimiter,
        }
    }
}

#[allow(dead_code)]
pub struct KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_CsvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_CsvRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_CsvRef {
        KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_CsvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_CsvRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_column_delimiter` after provisioning.\n"]
    pub fn record_column_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_column_delimiter", self.base))
    }

    #[doc = "Get a reference to the value of field `record_row_delimiter` after provisioning.\n"]
    pub fn record_row_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_row_delimiter", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Json {
    record_row_path: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Json { }

impl terrars::ToListMappable for KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Json {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Json>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Json {
    #[doc = ""]
    pub record_row_path: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Json {
    pub fn build(self) -> KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Json {
        KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Json {
            record_row_path: self.record_row_path,
        }
    }
}

#[allow(dead_code)]
pub struct KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_JsonRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_JsonRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_JsonRef {
        KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_JsonRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_JsonRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_row_path` after provisioning.\n"]
    pub fn record_row_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_row_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParametersDynamic {
    csv: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Csv>>,
    json: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Json>>,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv: std::option::Option<std::vec::Vec<KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Csv>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json: std
    ::option
    ::Option<std::vec::Vec<KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Json>>,
    dynamic: KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParametersDynamic,
}

impl KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters {
    #[doc = "Set the field `csv`.\n"]
    pub fn set_csv(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Csv,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.csv = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.csv = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `json`.\n"]
    pub fn set_json(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_Json,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.json = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.json = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters {}

impl BuildKinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters {
    pub fn build(self) -> KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters {
        KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters {
            csv: core::default::Default::default(),
            json: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParametersRef {
        KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `csv` after provisioning.\n"]
    pub fn csv(
        &self,
    ) -> terrars::ListRef<KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_CsvRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv", self.base))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(
        &self,
    ) -> terrars::ListRef<KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters_JsonRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplication_Inputs_Schema_RecordFormatDynamic {
    mapping_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters>>,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplication_Inputs_Schema_RecordFormat {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mapping_parameters: std
    ::option
    ::Option<std::vec::Vec<KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters>>,
    dynamic: KinesisAnalyticsApplication_Inputs_Schema_RecordFormatDynamic,
}

impl KinesisAnalyticsApplication_Inputs_Schema_RecordFormat {
    #[doc = "Set the field `mapping_parameters`.\n"]
    pub fn set_mapping_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParameters>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mapping_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mapping_parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisAnalyticsApplication_Inputs_Schema_RecordFormat {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_Schema_RecordFormat>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplication_Inputs_Schema_RecordFormat {}

impl BuildKinesisAnalyticsApplication_Inputs_Schema_RecordFormat {
    pub fn build(self) -> KinesisAnalyticsApplication_Inputs_Schema_RecordFormat {
        KinesisAnalyticsApplication_Inputs_Schema_RecordFormat {
            mapping_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisAnalyticsApplication_Inputs_Schema_RecordFormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplication_Inputs_Schema_RecordFormatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplication_Inputs_Schema_RecordFormatRef {
        KinesisAnalyticsApplication_Inputs_Schema_RecordFormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplication_Inputs_Schema_RecordFormatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_format_type` after provisioning.\n"]
    pub fn record_format_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_format_type", self.base))
    }

    #[doc = "Get a reference to the value of field `mapping_parameters` after provisioning.\n"]
    pub fn mapping_parameters(
        &self,
    ) -> terrars::ListRef<KinesisAnalyticsApplication_Inputs_Schema_RecordFormat_MappingParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mapping_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplication_Inputs_SchemaDynamic {
    record_columns: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_Inputs_Schema_RecordColumns>>,
    record_format: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_Inputs_Schema_RecordFormat>>,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplication_Inputs_Schema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_columns: std::option::Option<std::vec::Vec<KinesisAnalyticsApplication_Inputs_Schema_RecordColumns>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_format: std::option::Option<std::vec::Vec<KinesisAnalyticsApplication_Inputs_Schema_RecordFormat>>,
    dynamic: KinesisAnalyticsApplication_Inputs_SchemaDynamic,
}

impl KinesisAnalyticsApplication_Inputs_Schema {
    #[doc = "Set the field `record_encoding`.\n"]
    pub fn set_record_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.record_encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_columns`.\n"]
    pub fn set_record_columns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_Schema_RecordColumns>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.record_columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.record_columns = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `record_format`.\n"]
    pub fn set_record_format(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_Schema_RecordFormat>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.record_format = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.record_format = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisAnalyticsApplication_Inputs_Schema {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_Schema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplication_Inputs_Schema {}

impl BuildKinesisAnalyticsApplication_Inputs_Schema {
    pub fn build(self) -> KinesisAnalyticsApplication_Inputs_Schema {
        KinesisAnalyticsApplication_Inputs_Schema {
            record_encoding: core::default::Default::default(),
            record_columns: core::default::Default::default(),
            record_format: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisAnalyticsApplication_Inputs_SchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplication_Inputs_SchemaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KinesisAnalyticsApplication_Inputs_SchemaRef {
        KinesisAnalyticsApplication_Inputs_SchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplication_Inputs_SchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_encoding` after provisioning.\n"]
    pub fn record_encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_encoding", self.base))
    }

    #[doc = "Get a reference to the value of field `record_columns` after provisioning.\n"]
    pub fn record_columns(&self) -> terrars::ListRef<KinesisAnalyticsApplication_Inputs_Schema_RecordColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.record_columns", self.base))
    }

    #[doc = "Get a reference to the value of field `record_format` after provisioning.\n"]
    pub fn record_format(&self) -> terrars::ListRef<KinesisAnalyticsApplication_Inputs_Schema_RecordFormatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.record_format", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplication_Inputs_StartingPositionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    starting_position: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KinesisAnalyticsApplication_Inputs_StartingPositionConfiguration {
    #[doc = "Set the field `starting_position`.\n"]
    pub fn set_starting_position(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.starting_position = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KinesisAnalyticsApplication_Inputs_StartingPositionConfiguration {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_StartingPositionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplication_Inputs_StartingPositionConfiguration {}

impl BuildKinesisAnalyticsApplication_Inputs_StartingPositionConfiguration {
    pub fn build(self) -> KinesisAnalyticsApplication_Inputs_StartingPositionConfiguration {
        KinesisAnalyticsApplication_Inputs_StartingPositionConfiguration {
            starting_position: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisAnalyticsApplication_Inputs_StartingPositionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplication_Inputs_StartingPositionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplication_Inputs_StartingPositionConfigurationRef {
        KinesisAnalyticsApplication_Inputs_StartingPositionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplication_Inputs_StartingPositionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `starting_position` after provisioning.\n"]
    pub fn starting_position(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.starting_position", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplication_InputsDynamic {
    kinesis_firehose: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_Inputs_KinesisFirehose>>,
    kinesis_stream: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_Inputs_KinesisStream>>,
    parallelism: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_Inputs_Parallelism>>,
    processing_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_Inputs_ProcessingConfiguration>>,
    schema: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_Inputs_Schema>>,
    starting_position_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_Inputs_StartingPositionConfiguration>>,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplication_Inputs {
    name_prefix: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_firehose: std::option::Option<std::vec::Vec<KinesisAnalyticsApplication_Inputs_KinesisFirehose>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_stream: std::option::Option<std::vec::Vec<KinesisAnalyticsApplication_Inputs_KinesisStream>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallelism: std::option::Option<std::vec::Vec<KinesisAnalyticsApplication_Inputs_Parallelism>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processing_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisAnalyticsApplication_Inputs_ProcessingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<std::vec::Vec<KinesisAnalyticsApplication_Inputs_Schema>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    starting_position_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisAnalyticsApplication_Inputs_StartingPositionConfiguration>>,
    dynamic: KinesisAnalyticsApplication_InputsDynamic,
}

impl KinesisAnalyticsApplication_Inputs {
    #[doc = "Set the field `kinesis_firehose`.\n"]
    pub fn set_kinesis_firehose(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_KinesisFirehose>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis_firehose = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_firehose = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_stream`.\n"]
    pub fn set_kinesis_stream(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_KinesisStream>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis_stream = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_stream = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parallelism`.\n"]
    pub fn set_parallelism(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_Parallelism>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parallelism = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parallelism = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `processing_configuration`.\n"]
    pub fn set_processing_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_ProcessingConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.processing_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.processing_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_Schema>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `starting_position_configuration`.\n"]
    pub fn set_starting_position_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_StartingPositionConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.starting_position_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.starting_position_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisAnalyticsApplication_Inputs {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplication_Inputs {
    #[doc = ""]
    pub name_prefix: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplication_Inputs {
    pub fn build(self) -> KinesisAnalyticsApplication_Inputs {
        KinesisAnalyticsApplication_Inputs {
            name_prefix: self.name_prefix,
            kinesis_firehose: core::default::Default::default(),
            kinesis_stream: core::default::Default::default(),
            parallelism: core::default::Default::default(),
            processing_configuration: core::default::Default::default(),
            schema: core::default::Default::default(),
            starting_position_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisAnalyticsApplication_InputsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplication_InputsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KinesisAnalyticsApplication_InputsRef {
        KinesisAnalyticsApplication_InputsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplication_InputsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `stream_names` after provisioning.\n"]
    pub fn stream_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stream_names", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_firehose` after provisioning.\n"]
    pub fn kinesis_firehose(&self) -> terrars::ListRef<KinesisAnalyticsApplication_Inputs_KinesisFirehoseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_firehose", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_stream` after provisioning.\n"]
    pub fn kinesis_stream(&self) -> terrars::ListRef<KinesisAnalyticsApplication_Inputs_KinesisStreamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_stream", self.base))
    }

    #[doc = "Get a reference to the value of field `parallelism` after provisioning.\n"]
    pub fn parallelism(&self) -> terrars::ListRef<KinesisAnalyticsApplication_Inputs_ParallelismRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parallelism", self.base))
    }

    #[doc = "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> terrars::ListRef<KinesisAnalyticsApplication_Inputs_ProcessingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::ListRef<KinesisAnalyticsApplication_Inputs_SchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `starting_position_configuration` after provisioning.\n"]
    pub fn starting_position_configuration(
        &self,
    ) -> terrars::ListRef<KinesisAnalyticsApplication_Inputs_StartingPositionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.starting_position_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplication_Outputs_Schema {
    record_format_type: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplication_Outputs_Schema { }

impl terrars::ToListMappable for KinesisAnalyticsApplication_Outputs_Schema {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplication_Outputs_Schema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplication_Outputs_Schema {
    #[doc = ""]
    pub record_format_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplication_Outputs_Schema {
    pub fn build(self) -> KinesisAnalyticsApplication_Outputs_Schema {
        KinesisAnalyticsApplication_Outputs_Schema { record_format_type: self.record_format_type }
    }
}

#[allow(dead_code)]
pub struct KinesisAnalyticsApplication_Outputs_SchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplication_Outputs_SchemaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KinesisAnalyticsApplication_Outputs_SchemaRef {
        KinesisAnalyticsApplication_Outputs_SchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplication_Outputs_SchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_format_type` after provisioning.\n"]
    pub fn record_format_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_format_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplication_OutputsDynamic {
    kinesis_firehose: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_Inputs_KinesisFirehose>>,
    kinesis_stream: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_Inputs_KinesisStream>>,
    lambda: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_Outputs_Lambda>>,
    schema: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_Outputs_Schema>>,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplication_Outputs {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_firehose: std::option::Option<std::vec::Vec<KinesisAnalyticsApplication_Inputs_KinesisFirehose>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_stream: std::option::Option<std::vec::Vec<KinesisAnalyticsApplication_Inputs_KinesisStream>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda: std::option::Option<std::vec::Vec<KinesisAnalyticsApplication_Outputs_Lambda>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<std::vec::Vec<KinesisAnalyticsApplication_Outputs_Schema>>,
    dynamic: KinesisAnalyticsApplication_OutputsDynamic,
}

impl KinesisAnalyticsApplication_Outputs {
    #[doc = "Set the field `kinesis_firehose`.\n"]
    pub fn set_kinesis_firehose(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_KinesisFirehose>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis_firehose = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_firehose = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_stream`.\n"]
    pub fn set_kinesis_stream(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_KinesisStream>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis_stream = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_stream = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda`.\n"]
    pub fn set_lambda(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplication_Outputs_Lambda>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplication_Outputs_Schema>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisAnalyticsApplication_Outputs {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplication_Outputs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplication_Outputs {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplication_Outputs {
    pub fn build(self) -> KinesisAnalyticsApplication_Outputs {
        KinesisAnalyticsApplication_Outputs {
            name: self.name,
            kinesis_firehose: core::default::Default::default(),
            kinesis_stream: core::default::Default::default(),
            lambda: core::default::Default::default(),
            schema: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisAnalyticsApplication_OutputsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplication_OutputsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KinesisAnalyticsApplication_OutputsRef {
        KinesisAnalyticsApplication_OutputsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplication_OutputsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_firehose` after provisioning.\n"]
    pub fn kinesis_firehose(&self) -> terrars::ListRef<KinesisAnalyticsApplication_Inputs_KinesisFirehoseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_firehose", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_stream` after provisioning.\n"]
    pub fn kinesis_stream(&self) -> terrars::ListRef<KinesisAnalyticsApplication_Inputs_KinesisStreamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_stream", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda` after provisioning.\n"]
    pub fn lambda(&self) -> terrars::ListRef<KinesisAnalyticsApplication_Outputs_LambdaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::ListRef<KinesisAnalyticsApplication_Outputs_SchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplication_ReferenceDataSources_S3 {
    bucket_arn: terrars::PrimField<std::string::String>,
    file_key: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplication_ReferenceDataSources_S3 { }

impl terrars::ToListMappable for KinesisAnalyticsApplication_ReferenceDataSources_S3 {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplication_ReferenceDataSources_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplication_ReferenceDataSources_S3 {
    #[doc = ""]
    pub bucket_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub file_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplication_ReferenceDataSources_S3 {
    pub fn build(self) -> KinesisAnalyticsApplication_ReferenceDataSources_S3 {
        KinesisAnalyticsApplication_ReferenceDataSources_S3 {
            bucket_arn: self.bucket_arn,
            file_key: self.file_key,
            role_arn: self.role_arn,
        }
    }
}

#[allow(dead_code)]
pub struct KinesisAnalyticsApplication_ReferenceDataSources_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplication_ReferenceDataSources_S3Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplication_ReferenceDataSources_S3Ref {
        KinesisAnalyticsApplication_ReferenceDataSources_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplication_ReferenceDataSources_S3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `file_key` after provisioning.\n"]
    pub fn file_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_key", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplication_ReferenceDataSourcesDynamic {
    s3: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_ReferenceDataSources_S3>>,
    schema: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplication_Inputs_Schema>>,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplication_ReferenceDataSources {
    table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<KinesisAnalyticsApplication_ReferenceDataSources_S3>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<std::vec::Vec<KinesisAnalyticsApplication_Inputs_Schema>>,
    dynamic: KinesisAnalyticsApplication_ReferenceDataSourcesDynamic,
}

impl KinesisAnalyticsApplication_ReferenceDataSources {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplication_ReferenceDataSources_S3>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplication_Inputs_Schema>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisAnalyticsApplication_ReferenceDataSources {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplication_ReferenceDataSources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplication_ReferenceDataSources {
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplication_ReferenceDataSources {
    pub fn build(self) -> KinesisAnalyticsApplication_ReferenceDataSources {
        KinesisAnalyticsApplication_ReferenceDataSources {
            table_name: self.table_name,
            s3: core::default::Default::default(),
            schema: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisAnalyticsApplication_ReferenceDataSourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplication_ReferenceDataSourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplication_ReferenceDataSourcesRef {
        KinesisAnalyticsApplication_ReferenceDataSourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplication_ReferenceDataSourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<KinesisAnalyticsApplication_ReferenceDataSources_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::ListRef<KinesisAnalyticsApplication_Inputs_SchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}
