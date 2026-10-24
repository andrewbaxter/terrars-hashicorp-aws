use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct KinesisAnalyticsApplicationData {
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
    start_application: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logging_options: std::option::Option<std::vec::Vec<KinesisAnalyticsApplicationCloudwatchLoggingOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inputs: std::option::Option<std::vec::Vec<KinesisAnalyticsApplicationInputsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outputs: std::option::Option<std::vec::Vec<KinesisAnalyticsApplicationOutputsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reference_data_sources: std::option::Option<std::vec::Vec<KinesisAnalyticsApplicationReferenceDataSourcesEl>>,
    dynamic: KinesisAnalyticsApplicationDynamic,
}

struct KinesisAnalyticsApplication_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<KinesisAnalyticsApplicationData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplicationCloudwatchLoggingOptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplicationInputsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplicationOutputsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplicationReferenceDataSourcesEl>>,
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
    ) -> terrars::ListRef<KinesisAnalyticsApplicationCloudwatchLoggingOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inputs` after provisioning.\n"]
    pub fn inputs(&self) -> terrars::ListRef<KinesisAnalyticsApplicationInputsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inputs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reference_data_sources` after provisioning.\n"]
    pub fn reference_data_sources(&self) -> terrars::ListRef<KinesisAnalyticsApplicationReferenceDataSourcesElRef> {
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
            data: std::cell::RefCell::new(KinesisAnalyticsApplicationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                code: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
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
    ) -> terrars::ListRef<KinesisAnalyticsApplicationCloudwatchLoggingOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inputs` after provisioning.\n"]
    pub fn inputs(&self) -> terrars::ListRef<KinesisAnalyticsApplicationInputsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inputs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reference_data_sources` after provisioning.\n"]
    pub fn reference_data_sources(&self) -> terrars::ListRef<KinesisAnalyticsApplicationReferenceDataSourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reference_data_sources", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationCloudwatchLoggingOptionsEl {
    log_stream_arn: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplicationCloudwatchLoggingOptionsEl { }

impl terrars::ToListMappable for KinesisAnalyticsApplicationCloudwatchLoggingOptionsEl {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationCloudwatchLoggingOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationCloudwatchLoggingOptionsEl {
    #[doc = ""]
    pub log_stream_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplicationCloudwatchLoggingOptionsEl {
    pub fn build(self) -> KinesisAnalyticsApplicationCloudwatchLoggingOptionsEl {
        KinesisAnalyticsApplicationCloudwatchLoggingOptionsEl {
            log_stream_arn: self.log_stream_arn,
            role_arn: self.role_arn,
        }
    }
}

pub struct KinesisAnalyticsApplicationCloudwatchLoggingOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationCloudwatchLoggingOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationCloudwatchLoggingOptionsElRef {
        KinesisAnalyticsApplicationCloudwatchLoggingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationCloudwatchLoggingOptionsElRef {
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
pub struct KinesisAnalyticsApplicationInputsElKinesisFirehoseEl {
    resource_arn: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplicationInputsElKinesisFirehoseEl { }

impl terrars::ToListMappable for KinesisAnalyticsApplicationInputsElKinesisFirehoseEl {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationInputsElKinesisFirehoseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElKinesisFirehoseEl {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplicationInputsElKinesisFirehoseEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsElKinesisFirehoseEl {
        KinesisAnalyticsApplicationInputsElKinesisFirehoseEl {
            resource_arn: self.resource_arn,
            role_arn: self.role_arn,
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElKinesisFirehoseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationInputsElKinesisFirehoseElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationInputsElKinesisFirehoseElRef {
        KinesisAnalyticsApplicationInputsElKinesisFirehoseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElKinesisFirehoseElRef {
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
pub struct KinesisAnalyticsApplicationInputsElKinesisStreamEl {
    resource_arn: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplicationInputsElKinesisStreamEl { }

impl terrars::ToListMappable for KinesisAnalyticsApplicationInputsElKinesisStreamEl {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationInputsElKinesisStreamEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElKinesisStreamEl {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplicationInputsElKinesisStreamEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsElKinesisStreamEl {
        KinesisAnalyticsApplicationInputsElKinesisStreamEl {
            resource_arn: self.resource_arn,
            role_arn: self.role_arn,
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElKinesisStreamElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationInputsElKinesisStreamElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationInputsElKinesisStreamElRef {
        KinesisAnalyticsApplicationInputsElKinesisStreamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElKinesisStreamElRef {
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
pub struct KinesisAnalyticsApplicationInputsElParallelismEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
}

impl KinesisAnalyticsApplicationInputsElParallelismEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KinesisAnalyticsApplicationInputsElParallelismEl {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationInputsElParallelismEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElParallelismEl {}

impl BuildKinesisAnalyticsApplicationInputsElParallelismEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsElParallelismEl {
        KinesisAnalyticsApplicationInputsElParallelismEl { count: core::default::Default::default() }
    }
}

pub struct KinesisAnalyticsApplicationInputsElParallelismElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationInputsElParallelismElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationInputsElParallelismElRef {
        KinesisAnalyticsApplicationInputsElParallelismElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElParallelismElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl {
    resource_arn: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl { }

impl terrars::ToListMappable for KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl {
        KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl {
            resource_arn: self.resource_arn,
            role_arn: self.role_arn,
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaElRef {
        KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaElRef {
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
struct KinesisAnalyticsApplicationInputsElProcessingConfigurationElDynamic {
    lambda: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl>>,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationInputsElProcessingConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda: std::option::Option<std::vec::Vec<KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl>>,
    dynamic: KinesisAnalyticsApplicationInputsElProcessingConfigurationElDynamic,
}

impl KinesisAnalyticsApplicationInputsElProcessingConfigurationEl {
    #[doc = "Set the field `lambda`.\n"]
    pub fn set_lambda(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl>,
                    >,
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

impl terrars::ToListMappable for KinesisAnalyticsApplicationInputsElProcessingConfigurationEl {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationInputsElProcessingConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElProcessingConfigurationEl {}

impl BuildKinesisAnalyticsApplicationInputsElProcessingConfigurationEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsElProcessingConfigurationEl {
        KinesisAnalyticsApplicationInputsElProcessingConfigurationEl {
            lambda: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElProcessingConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationInputsElProcessingConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationInputsElProcessingConfigurationElRef {
        KinesisAnalyticsApplicationInputsElProcessingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElProcessingConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda` after provisioning.\n"]
    pub fn lambda(&self) -> terrars::ListRef<KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mapping: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    sql_type: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl {
    #[doc = "Set the field `mapping`.\n"]
    pub fn set_mapping(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mapping = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sql_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl {
        KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl {
            mapping: core::default::Default::default(),
            name: self.name,
            sql_type: self.sql_type,
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsElRef {
        KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsElRef {
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
pub struct KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl {
    record_column_delimiter: terrars::PrimField<std::string::String>,
    record_row_delimiter: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl { }

impl terrars::ToListMappable for KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl {
    type O =
        terrars::BlockAssignable<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl {
    #[doc = ""]
    pub record_column_delimiter: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub record_row_delimiter: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl {
        KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl {
            record_column_delimiter: self.record_column_delimiter,
            record_row_delimiter: self.record_row_delimiter,
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvElRef {
        KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvElRef {
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
pub struct KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl {
    record_row_path: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl { }

impl terrars::ToListMappable for KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl {
    type O =
        terrars::BlockAssignable<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl {
    #[doc = ""]
    pub record_row_path: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl {
        KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl {
            record_row_path: self.record_row_path,
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonElRef {
        KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_row_path` after provisioning.\n"]
    pub fn record_row_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_row_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElDynamic {
    csv: std
    ::option
    ::Option<
        terrars::DynamicBlock<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl>,
    >,
    json: std
    ::option
    ::Option<
        terrars::DynamicBlock<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv: std
    ::option
    ::Option<std::vec::Vec<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json: std
    ::option
    ::Option<std::vec::Vec<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl>>,
    dynamic: KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElDynamic,
}

impl KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl {
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
                            KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl,
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
                            KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl,
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

impl terrars::ToListMappable for KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl {}

impl BuildKinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl {
        KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl {
            csv: core::default::Default::default(),
            json: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElRef {
        KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `csv` after provisioning.\n"]
    pub fn csv(
        &self,
    ) -> terrars::ListRef<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv", self.base))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(
        &self,
    ) -> terrars::ListRef<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElDynamic {
    mapping_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl>>,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mapping_parameters: std
    ::option
    ::Option<std::vec::Vec<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl>>,
    dynamic: KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElDynamic,
}

impl KinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl {
    #[doc = "Set the field `mapping_parameters`.\n"]
    pub fn set_mapping_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl,
                        >,
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

impl terrars::ToListMappable for KinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl {}

impl BuildKinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl {
        KinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl {
            mapping_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElRef {
        KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElRef {
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
    ) -> terrars::ListRef<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mapping_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationInputsElSchemaElDynamic {
    record_columns: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl>>,
    record_format: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl>>,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationInputsElSchemaEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_columns: std::option::Option<std::vec::Vec<KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_format: std::option::Option<std::vec::Vec<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl>>,
    dynamic: KinesisAnalyticsApplicationInputsElSchemaElDynamic,
}

impl KinesisAnalyticsApplicationInputsElSchemaEl {
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
                    ::Into<terrars::BlockAssignable<KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl>>,
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

impl terrars::ToListMappable for KinesisAnalyticsApplicationInputsElSchemaEl {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationInputsElSchemaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElSchemaEl {}

impl BuildKinesisAnalyticsApplicationInputsElSchemaEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsElSchemaEl {
        KinesisAnalyticsApplicationInputsElSchemaEl {
            record_encoding: core::default::Default::default(),
            record_columns: core::default::Default::default(),
            record_format: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElSchemaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationInputsElSchemaElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationInputsElSchemaElRef {
        KinesisAnalyticsApplicationInputsElSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElSchemaElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_encoding` after provisioning.\n"]
    pub fn record_encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_encoding", self.base))
    }

    #[doc = "Get a reference to the value of field `record_columns` after provisioning.\n"]
    pub fn record_columns(&self) -> terrars::ListRef<KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.record_columns", self.base))
    }

    #[doc = "Get a reference to the value of field `record_format` after provisioning.\n"]
    pub fn record_format(&self) -> terrars::ListRef<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.record_format", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    starting_position: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl {
    #[doc = "Set the field `starting_position`.\n"]
    pub fn set_starting_position(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.starting_position = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl {}

impl BuildKinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl {
        KinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl {
            starting_position: core::default::Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElStartingPositionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationInputsElStartingPositionConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationInputsElStartingPositionConfigurationElRef {
        KinesisAnalyticsApplicationInputsElStartingPositionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElStartingPositionConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `starting_position` after provisioning.\n"]
    pub fn starting_position(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.starting_position", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationInputsElDynamic {
    kinesis_firehose: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationInputsElKinesisFirehoseEl>>,
    kinesis_stream: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationInputsElKinesisStreamEl>>,
    parallelism: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationInputsElParallelismEl>>,
    processing_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationInputsElProcessingConfigurationEl>>,
    schema: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationInputsElSchemaEl>>,
    starting_position_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl>>,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationInputsEl {
    name_prefix: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_firehose: std::option::Option<std::vec::Vec<KinesisAnalyticsApplicationInputsElKinesisFirehoseEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_stream: std::option::Option<std::vec::Vec<KinesisAnalyticsApplicationInputsElKinesisStreamEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallelism: std::option::Option<std::vec::Vec<KinesisAnalyticsApplicationInputsElParallelismEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processing_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisAnalyticsApplicationInputsElProcessingConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<std::vec::Vec<KinesisAnalyticsApplicationInputsElSchemaEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    starting_position_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl>>,
    dynamic: KinesisAnalyticsApplicationInputsElDynamic,
}

impl KinesisAnalyticsApplicationInputsEl {
    #[doc = "Set the field `kinesis_firehose`.\n"]
    pub fn set_kinesis_firehose(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplicationInputsElKinesisFirehoseEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplicationInputsElKinesisStreamEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplicationInputsElParallelismEl>>,
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
                    ::Into<terrars::BlockAssignable<KinesisAnalyticsApplicationInputsElProcessingConfigurationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplicationInputsElSchemaEl>>,
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
                        terrars::BlockAssignable<KinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl>,
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

impl terrars::ToListMappable for KinesisAnalyticsApplicationInputsEl {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationInputsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsEl {
    #[doc = ""]
    pub name_prefix: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplicationInputsEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsEl {
        KinesisAnalyticsApplicationInputsEl {
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

pub struct KinesisAnalyticsApplicationInputsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationInputsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KinesisAnalyticsApplicationInputsElRef {
        KinesisAnalyticsApplicationInputsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElRef {
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
    pub fn kinesis_firehose(&self) -> terrars::ListRef<KinesisAnalyticsApplicationInputsElKinesisFirehoseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_firehose", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_stream` after provisioning.\n"]
    pub fn kinesis_stream(&self) -> terrars::ListRef<KinesisAnalyticsApplicationInputsElKinesisStreamElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_stream", self.base))
    }

    #[doc = "Get a reference to the value of field `parallelism` after provisioning.\n"]
    pub fn parallelism(&self) -> terrars::ListRef<KinesisAnalyticsApplicationInputsElParallelismElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parallelism", self.base))
    }

    #[doc = "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> terrars::ListRef<KinesisAnalyticsApplicationInputsElProcessingConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::ListRef<KinesisAnalyticsApplicationInputsElSchemaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `starting_position_configuration` after provisioning.\n"]
    pub fn starting_position_configuration(
        &self,
    ) -> terrars::ListRef<KinesisAnalyticsApplicationInputsElStartingPositionConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.starting_position_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationOutputsElKinesisFirehoseEl {
    resource_arn: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplicationOutputsElKinesisFirehoseEl { }

impl terrars::ToListMappable for KinesisAnalyticsApplicationOutputsElKinesisFirehoseEl {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationOutputsElKinesisFirehoseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationOutputsElKinesisFirehoseEl {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplicationOutputsElKinesisFirehoseEl {
    pub fn build(self) -> KinesisAnalyticsApplicationOutputsElKinesisFirehoseEl {
        KinesisAnalyticsApplicationOutputsElKinesisFirehoseEl {
            resource_arn: self.resource_arn,
            role_arn: self.role_arn,
        }
    }
}

pub struct KinesisAnalyticsApplicationOutputsElKinesisFirehoseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationOutputsElKinesisFirehoseElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationOutputsElKinesisFirehoseElRef {
        KinesisAnalyticsApplicationOutputsElKinesisFirehoseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationOutputsElKinesisFirehoseElRef {
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
pub struct KinesisAnalyticsApplicationOutputsElKinesisStreamEl {
    resource_arn: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplicationOutputsElKinesisStreamEl { }

impl terrars::ToListMappable for KinesisAnalyticsApplicationOutputsElKinesisStreamEl {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationOutputsElKinesisStreamEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationOutputsElKinesisStreamEl {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplicationOutputsElKinesisStreamEl {
    pub fn build(self) -> KinesisAnalyticsApplicationOutputsElKinesisStreamEl {
        KinesisAnalyticsApplicationOutputsElKinesisStreamEl {
            resource_arn: self.resource_arn,
            role_arn: self.role_arn,
        }
    }
}

pub struct KinesisAnalyticsApplicationOutputsElKinesisStreamElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationOutputsElKinesisStreamElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationOutputsElKinesisStreamElRef {
        KinesisAnalyticsApplicationOutputsElKinesisStreamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationOutputsElKinesisStreamElRef {
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
pub struct KinesisAnalyticsApplicationOutputsElLambdaEl {
    resource_arn: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplicationOutputsElLambdaEl { }

impl terrars::ToListMappable for KinesisAnalyticsApplicationOutputsElLambdaEl {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationOutputsElLambdaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationOutputsElLambdaEl {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplicationOutputsElLambdaEl {
    pub fn build(self) -> KinesisAnalyticsApplicationOutputsElLambdaEl {
        KinesisAnalyticsApplicationOutputsElLambdaEl {
            resource_arn: self.resource_arn,
            role_arn: self.role_arn,
        }
    }
}

pub struct KinesisAnalyticsApplicationOutputsElLambdaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationOutputsElLambdaElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationOutputsElLambdaElRef {
        KinesisAnalyticsApplicationOutputsElLambdaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationOutputsElLambdaElRef {
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
pub struct KinesisAnalyticsApplicationOutputsElSchemaEl {
    record_format_type: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplicationOutputsElSchemaEl { }

impl terrars::ToListMappable for KinesisAnalyticsApplicationOutputsElSchemaEl {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationOutputsElSchemaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationOutputsElSchemaEl {
    #[doc = ""]
    pub record_format_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplicationOutputsElSchemaEl {
    pub fn build(self) -> KinesisAnalyticsApplicationOutputsElSchemaEl {
        KinesisAnalyticsApplicationOutputsElSchemaEl { record_format_type: self.record_format_type }
    }
}

pub struct KinesisAnalyticsApplicationOutputsElSchemaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationOutputsElSchemaElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationOutputsElSchemaElRef {
        KinesisAnalyticsApplicationOutputsElSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationOutputsElSchemaElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_format_type` after provisioning.\n"]
    pub fn record_format_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_format_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationOutputsElDynamic {
    kinesis_firehose: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationOutputsElKinesisFirehoseEl>>,
    kinesis_stream: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationOutputsElKinesisStreamEl>>,
    lambda: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationOutputsElLambdaEl>>,
    schema: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationOutputsElSchemaEl>>,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationOutputsEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_firehose: std::option::Option<std::vec::Vec<KinesisAnalyticsApplicationOutputsElKinesisFirehoseEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_stream: std::option::Option<std::vec::Vec<KinesisAnalyticsApplicationOutputsElKinesisStreamEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda: std::option::Option<std::vec::Vec<KinesisAnalyticsApplicationOutputsElLambdaEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<std::vec::Vec<KinesisAnalyticsApplicationOutputsElSchemaEl>>,
    dynamic: KinesisAnalyticsApplicationOutputsElDynamic,
}

impl KinesisAnalyticsApplicationOutputsEl {
    #[doc = "Set the field `kinesis_firehose`.\n"]
    pub fn set_kinesis_firehose(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplicationOutputsElKinesisFirehoseEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplicationOutputsElKinesisStreamEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplicationOutputsElLambdaEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplicationOutputsElSchemaEl>>,
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

impl terrars::ToListMappable for KinesisAnalyticsApplicationOutputsEl {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationOutputsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationOutputsEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplicationOutputsEl {
    pub fn build(self) -> KinesisAnalyticsApplicationOutputsEl {
        KinesisAnalyticsApplicationOutputsEl {
            name: self.name,
            kinesis_firehose: core::default::Default::default(),
            kinesis_stream: core::default::Default::default(),
            lambda: core::default::Default::default(),
            schema: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationOutputsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationOutputsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KinesisAnalyticsApplicationOutputsElRef {
        KinesisAnalyticsApplicationOutputsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationOutputsElRef {
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
    pub fn kinesis_firehose(&self) -> terrars::ListRef<KinesisAnalyticsApplicationOutputsElKinesisFirehoseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_firehose", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_stream` after provisioning.\n"]
    pub fn kinesis_stream(&self) -> terrars::ListRef<KinesisAnalyticsApplicationOutputsElKinesisStreamElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_stream", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda` after provisioning.\n"]
    pub fn lambda(&self) -> terrars::ListRef<KinesisAnalyticsApplicationOutputsElLambdaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::ListRef<KinesisAnalyticsApplicationOutputsElSchemaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationReferenceDataSourcesElS3El {
    bucket_arn: terrars::PrimField<std::string::String>,
    file_key: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElS3El { }

impl terrars::ToListMappable for KinesisAnalyticsApplicationReferenceDataSourcesElS3El {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationReferenceDataSourcesElS3El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationReferenceDataSourcesElS3El {
    #[doc = ""]
    pub bucket_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub file_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplicationReferenceDataSourcesElS3El {
    pub fn build(self) -> KinesisAnalyticsApplicationReferenceDataSourcesElS3El {
        KinesisAnalyticsApplicationReferenceDataSourcesElS3El {
            bucket_arn: self.bucket_arn,
            file_key: self.file_key,
            role_arn: self.role_arn,
        }
    }
}

pub struct KinesisAnalyticsApplicationReferenceDataSourcesElS3ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationReferenceDataSourcesElS3ElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationReferenceDataSourcesElS3ElRef {
        KinesisAnalyticsApplicationReferenceDataSourcesElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElS3ElRef {
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

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mapping: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    sql_type: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl {
    #[doc = "Set the field `mapping`.\n"]
    pub fn set_mapping(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mapping = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sql_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl {
    pub fn build(self) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl {
            mapping: core::default::Default::default(),
            name: self.name,
            sql_type: self.sql_type,
        }
    }
}

pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsElRef {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsElRef {
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
pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl {
    record_column_delimiter: terrars::PrimField<std::string::String>,
    record_row_delimiter: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl { }

impl terrars
::ToListMappable for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl {
    #[doc = ""]
    pub record_column_delimiter: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub record_row_delimiter: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl {
    pub fn build(
        self,
    ) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl {
            record_column_delimiter: self.record_column_delimiter,
            record_row_delimiter: self.record_row_delimiter,
        }
    }
}

pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvElRef {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvElRef {
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
pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl {
    record_row_path: terrars::PrimField<std::string::String>,
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl { }

impl terrars
::ToListMappable for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl {
    #[doc = ""]
    pub record_row_path: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl {
    pub fn build(
        self,
    ) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl {
            record_row_path: self.record_row_path,
        }
    }
}

pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonElRef {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_row_path` after provisioning.\n"]
    pub fn record_row_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_row_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElDynamic {
    csv: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl,
        >,
    >,
    json: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv: std
    ::option
    ::Option<
        std::vec::Vec<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl>,
    >,
    dynamic: KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElDynamic,
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl {
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
                            KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl,
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
                            KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl,
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

impl terrars
::ToListMappable for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl {
    type O =
        terrars
        ::BlockAssignable<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl {}

impl BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl {
    pub fn build(self) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl {
            csv: core::default::Default::default(),
            json: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElRef {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `csv` after provisioning.\n"]
    pub fn csv(
        &self,
    ) -> terrars
    ::ListRef<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv", self.base))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(
        &self,
    ) -> terrars
    ::ListRef<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElDynamic {
    mapping_parameters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mapping_parameters: std
    ::option
    ::Option<
        std::vec::Vec<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl>,
    >,
    dynamic: KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElDynamic,
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl {
    #[doc = "Set the field `mapping_parameters`.\n"]
    pub fn set_mapping_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl,
                        >,
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

impl terrars::ToListMappable for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl {}

impl BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl {
    pub fn build(self) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl {
            mapping_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElRef {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElRef {
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
    ) -> terrars
    ::ListRef<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mapping_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElDynamic {
    record_columns: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl>>,
    record_format: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl>>,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_columns: std
    ::option
    ::Option<std::vec::Vec<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_format: std
    ::option
    ::Option<std::vec::Vec<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl>>,
    dynamic: KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElDynamic,
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl {
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
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl>,
                    >,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl>,
                    >,
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

impl terrars::ToListMappable for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl {}

impl BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl {
    pub fn build(self) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl {
            record_encoding: core::default::Default::default(),
            record_columns: core::default::Default::default(),
            record_format: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRef {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_encoding` after provisioning.\n"]
    pub fn record_encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_encoding", self.base))
    }

    #[doc = "Get a reference to the value of field `record_columns` after provisioning.\n"]
    pub fn record_columns(
        &self,
    ) -> terrars::ListRef<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.record_columns", self.base))
    }

    #[doc = "Get a reference to the value of field `record_format` after provisioning.\n"]
    pub fn record_format(
        &self,
    ) -> terrars::ListRef<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.record_format", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationReferenceDataSourcesElDynamic {
    s3: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationReferenceDataSourcesElS3El>>,
    schema: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl>>,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationReferenceDataSourcesEl {
    table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<KinesisAnalyticsApplicationReferenceDataSourcesElS3El>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<std::vec::Vec<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl>>,
    dynamic: KinesisAnalyticsApplicationReferenceDataSourcesElDynamic,
}

impl KinesisAnalyticsApplicationReferenceDataSourcesEl {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisAnalyticsApplicationReferenceDataSourcesElS3El>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl>>,
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

impl terrars::ToListMappable for KinesisAnalyticsApplicationReferenceDataSourcesEl {
    type O = terrars::BlockAssignable<KinesisAnalyticsApplicationReferenceDataSourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationReferenceDataSourcesEl {
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildKinesisAnalyticsApplicationReferenceDataSourcesEl {
    pub fn build(self) -> KinesisAnalyticsApplicationReferenceDataSourcesEl {
        KinesisAnalyticsApplicationReferenceDataSourcesEl {
            table_name: self.table_name,
            s3: core::default::Default::default(),
            schema: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationReferenceDataSourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisAnalyticsApplicationReferenceDataSourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisAnalyticsApplicationReferenceDataSourcesElRef {
        KinesisAnalyticsApplicationReferenceDataSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElRef {
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
    pub fn s3(&self) -> terrars::ListRef<KinesisAnalyticsApplicationReferenceDataSourcesElS3ElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::ListRef<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationDynamic {
    cloudwatch_logging_options: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationCloudwatchLoggingOptionsEl>>,
    inputs: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationInputsEl>>,
    outputs: std::option::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationOutputsEl>>,
    reference_data_sources: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisAnalyticsApplicationReferenceDataSourcesEl>>,
}
