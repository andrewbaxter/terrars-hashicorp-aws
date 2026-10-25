#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VerifiedaccessInstanceLoggingConfigurationData_ {
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
    verifiedaccess_instance_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_logs: std::option::Option<std::vec::Vec<VerifiedaccessInstanceLoggingConfiguration_AccessLogs>>,
    dynamic: VerifiedaccessInstanceLoggingConfigurationDynamic,
}

struct VerifiedaccessInstanceLoggingConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VerifiedaccessInstanceLoggingConfigurationData_>,
}

#[derive(Clone)]
pub struct VerifiedaccessInstanceLoggingConfiguration(std::rc::Rc<VerifiedaccessInstanceLoggingConfiguration_>);

impl VerifiedaccessInstanceLoggingConfiguration {
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

    #[doc = "Set the field `access_logs`.\n"]
    pub fn set_access_logs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VerifiedaccessInstanceLoggingConfiguration_AccessLogs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_logs = std::option::Option::Some(d);
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

    #[doc = "Get a reference to the value of field `verifiedaccess_instance_id` after provisioning.\n"]
    pub fn verifiedaccess_instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verifiedaccess_instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_logs` after provisioning.\n"]
    pub fn access_logs(&self) -> terrars::ListRef<VerifiedaccessInstanceLoggingConfiguration_AccessLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_logs", self.extract_ref()))
    }
}

impl terrars::Referable for VerifiedaccessInstanceLoggingConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VerifiedaccessInstanceLoggingConfiguration { }

impl terrars::ToListMappable for VerifiedaccessInstanceLoggingConfiguration {
    type O = terrars::ListRef<VerifiedaccessInstanceLoggingConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VerifiedaccessInstanceLoggingConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_verifiedaccess_instance_logging_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVerifiedaccessInstanceLoggingConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub verifiedaccess_instance_id: terrars::PrimField<std::string::String>,
}

impl BuildVerifiedaccessInstanceLoggingConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> VerifiedaccessInstanceLoggingConfiguration {
        let out =
            VerifiedaccessInstanceLoggingConfiguration(std::rc::Rc::new(VerifiedaccessInstanceLoggingConfiguration_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(VerifiedaccessInstanceLoggingConfigurationData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    provisioner: core::default::Default::default(),
                    id: core::default::Default::default(),
                    region: core::default::Default::default(),
                    verifiedaccess_instance_id: self.verifiedaccess_instance_id,
                    access_logs: core::default::Default::default(),
                    dynamic: core::default::Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VerifiedaccessInstanceLoggingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedaccessInstanceLoggingConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VerifiedaccessInstanceLoggingConfigurationRef {
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

    #[doc = "Get a reference to the value of field `verifiedaccess_instance_id` after provisioning.\n"]
    pub fn verifiedaccess_instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verifiedaccess_instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_logs` after provisioning.\n"]
    pub fn access_logs(&self) -> terrars::ListRef<VerifiedaccessInstanceLoggingConfiguration_AccessLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_logs", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct VerifiedaccessInstanceLoggingConfigurationDynamic {
    access_logs: std::option::Option<terrars::DynamicBlock<VerifiedaccessInstanceLoggingConfiguration_AccessLogs>>,
}

#[derive(Serialize)]
pub struct VerifiedaccessInstanceLoggingConfiguration_AccessLogs_CloudwatchLogs {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VerifiedaccessInstanceLoggingConfiguration_AccessLogs_CloudwatchLogs {
    #[doc = "Set the field `log_group`.\n"]
    pub fn set_log_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_group = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VerifiedaccessInstanceLoggingConfiguration_AccessLogs_CloudwatchLogs {
    type O = terrars::BlockAssignable<VerifiedaccessInstanceLoggingConfiguration_AccessLogs_CloudwatchLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessInstanceLoggingConfiguration_AccessLogs_CloudwatchLogs {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildVerifiedaccessInstanceLoggingConfiguration_AccessLogs_CloudwatchLogs {
    pub fn build(self) -> VerifiedaccessInstanceLoggingConfiguration_AccessLogs_CloudwatchLogs {
        VerifiedaccessInstanceLoggingConfiguration_AccessLogs_CloudwatchLogs {
            enabled: self.enabled,
            log_group: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedaccessInstanceLoggingConfiguration_AccessLogs_CloudwatchLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedaccessInstanceLoggingConfiguration_AccessLogs_CloudwatchLogsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VerifiedaccessInstanceLoggingConfiguration_AccessLogs_CloudwatchLogsRef {
        VerifiedaccessInstanceLoggingConfiguration_AccessLogs_CloudwatchLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessInstanceLoggingConfiguration_AccessLogs_CloudwatchLogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_group` after provisioning.\n"]
    pub fn log_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedaccessInstanceLoggingConfiguration_AccessLogs_KinesisDataFirehose {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delivery_stream: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
}

impl VerifiedaccessInstanceLoggingConfiguration_AccessLogs_KinesisDataFirehose {
    #[doc = "Set the field `delivery_stream`.\n"]
    pub fn set_delivery_stream(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delivery_stream = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VerifiedaccessInstanceLoggingConfiguration_AccessLogs_KinesisDataFirehose {
    type O = terrars::BlockAssignable<VerifiedaccessInstanceLoggingConfiguration_AccessLogs_KinesisDataFirehose>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessInstanceLoggingConfiguration_AccessLogs_KinesisDataFirehose {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildVerifiedaccessInstanceLoggingConfiguration_AccessLogs_KinesisDataFirehose {
    pub fn build(self) -> VerifiedaccessInstanceLoggingConfiguration_AccessLogs_KinesisDataFirehose {
        VerifiedaccessInstanceLoggingConfiguration_AccessLogs_KinesisDataFirehose {
            delivery_stream: core::default::Default::default(),
            enabled: self.enabled,
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedaccessInstanceLoggingConfiguration_AccessLogs_KinesisDataFirehoseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedaccessInstanceLoggingConfiguration_AccessLogs_KinesisDataFirehoseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VerifiedaccessInstanceLoggingConfiguration_AccessLogs_KinesisDataFirehoseRef {
        VerifiedaccessInstanceLoggingConfiguration_AccessLogs_KinesisDataFirehoseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessInstanceLoggingConfiguration_AccessLogs_KinesisDataFirehoseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delivery_stream` after provisioning.\n"]
    pub fn delivery_stream(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delivery_stream", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedaccessInstanceLoggingConfiguration_AccessLogs_S3 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VerifiedaccessInstanceLoggingConfiguration_AccessLogs_S3 {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_owner`.\n"]
    pub fn set_bucket_owner(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_owner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VerifiedaccessInstanceLoggingConfiguration_AccessLogs_S3 {
    type O = terrars::BlockAssignable<VerifiedaccessInstanceLoggingConfiguration_AccessLogs_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessInstanceLoggingConfiguration_AccessLogs_S3 {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildVerifiedaccessInstanceLoggingConfiguration_AccessLogs_S3 {
    pub fn build(self) -> VerifiedaccessInstanceLoggingConfiguration_AccessLogs_S3 {
        VerifiedaccessInstanceLoggingConfiguration_AccessLogs_S3 {
            bucket_name: core::default::Default::default(),
            bucket_owner: core::default::Default::default(),
            enabled: self.enabled,
            prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedaccessInstanceLoggingConfiguration_AccessLogs_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedaccessInstanceLoggingConfiguration_AccessLogs_S3Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VerifiedaccessInstanceLoggingConfiguration_AccessLogs_S3Ref {
        VerifiedaccessInstanceLoggingConfiguration_AccessLogs_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessInstanceLoggingConfiguration_AccessLogs_S3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_owner` after provisioning.\n"]
    pub fn bucket_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_owner", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct VerifiedaccessInstanceLoggingConfiguration_AccessLogsDynamic {
    cloudwatch_logs: std
    ::option
    ::Option<terrars::DynamicBlock<VerifiedaccessInstanceLoggingConfiguration_AccessLogs_CloudwatchLogs>>,
    kinesis_data_firehose: std
    ::option
    ::Option<terrars::DynamicBlock<VerifiedaccessInstanceLoggingConfiguration_AccessLogs_KinesisDataFirehose>>,
    s3: std::option::Option<terrars::DynamicBlock<VerifiedaccessInstanceLoggingConfiguration_AccessLogs_S3>>,
}

#[derive(Serialize)]
pub struct VerifiedaccessInstanceLoggingConfiguration_AccessLogs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_trust_context: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs: std
    ::option
    ::Option<std::vec::Vec<VerifiedaccessInstanceLoggingConfiguration_AccessLogs_CloudwatchLogs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_data_firehose: std
    ::option
    ::Option<std::vec::Vec<VerifiedaccessInstanceLoggingConfiguration_AccessLogs_KinesisDataFirehose>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<VerifiedaccessInstanceLoggingConfiguration_AccessLogs_S3>>,
    dynamic: VerifiedaccessInstanceLoggingConfiguration_AccessLogsDynamic,
}

impl VerifiedaccessInstanceLoggingConfiguration_AccessLogs {
    #[doc = "Set the field `include_trust_context`.\n"]
    pub fn set_include_trust_context(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_trust_context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_version`.\n"]
    pub fn set_log_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<VerifiedaccessInstanceLoggingConfiguration_AccessLogs_CloudwatchLogs>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_data_firehose`.\n"]
    pub fn set_kinesis_data_firehose(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VerifiedaccessInstanceLoggingConfiguration_AccessLogs_KinesisDataFirehose>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis_data_firehose = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_data_firehose = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VerifiedaccessInstanceLoggingConfiguration_AccessLogs_S3>>,
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
}

impl terrars::ToListMappable for VerifiedaccessInstanceLoggingConfiguration_AccessLogs {
    type O = terrars::BlockAssignable<VerifiedaccessInstanceLoggingConfiguration_AccessLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessInstanceLoggingConfiguration_AccessLogs {}

impl BuildVerifiedaccessInstanceLoggingConfiguration_AccessLogs {
    pub fn build(self) -> VerifiedaccessInstanceLoggingConfiguration_AccessLogs {
        VerifiedaccessInstanceLoggingConfiguration_AccessLogs {
            include_trust_context: core::default::Default::default(),
            log_version: core::default::Default::default(),
            cloudwatch_logs: core::default::Default::default(),
            kinesis_data_firehose: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedaccessInstanceLoggingConfiguration_AccessLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedaccessInstanceLoggingConfiguration_AccessLogsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VerifiedaccessInstanceLoggingConfiguration_AccessLogsRef {
        VerifiedaccessInstanceLoggingConfiguration_AccessLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessInstanceLoggingConfiguration_AccessLogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_trust_context` after provisioning.\n"]
    pub fn include_trust_context(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_trust_context", self.base))
    }

    #[doc = "Get a reference to the value of field `log_version` after provisioning.\n"]
    pub fn log_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_version", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(
        &self,
    ) -> terrars::ListRef<VerifiedaccessInstanceLoggingConfiguration_AccessLogs_CloudwatchLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_data_firehose` after provisioning.\n"]
    pub fn kinesis_data_firehose(
        &self,
    ) -> terrars::ListRef<VerifiedaccessInstanceLoggingConfiguration_AccessLogs_KinesisDataFirehoseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_data_firehose", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<VerifiedaccessInstanceLoggingConfiguration_AccessLogs_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}
