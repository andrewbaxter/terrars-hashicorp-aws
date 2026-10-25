#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CognitoLogDeliveryConfigurationData_ {
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
    user_pool_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_configurations: std::option::Option<std::vec::Vec<CognitoLogDeliveryConfiguration_LogConfigurations>>,
    dynamic: CognitoLogDeliveryConfigurationDynamic,
}

struct CognitoLogDeliveryConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CognitoLogDeliveryConfigurationData_>,
}

#[derive(Clone)]
pub struct CognitoLogDeliveryConfiguration(std::rc::Rc<CognitoLogDeliveryConfiguration_>);

impl CognitoLogDeliveryConfiguration {
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

    #[doc = "Set the field `log_configurations`.\n"]
    pub fn set_log_configurations(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoLogDeliveryConfiguration_LogConfigurations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_configurations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_configurations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_configurations` after provisioning.\n"]
    pub fn log_configurations(&self) -> terrars::ListRef<CognitoLogDeliveryConfiguration_LogConfigurationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_configurations", self.extract_ref()))
    }
}

impl terrars::Referable for CognitoLogDeliveryConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CognitoLogDeliveryConfiguration { }

impl terrars::ToListMappable for CognitoLogDeliveryConfiguration {
    type O = terrars::ListRef<CognitoLogDeliveryConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CognitoLogDeliveryConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cognito_log_delivery_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCognitoLogDeliveryConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub user_pool_id: terrars::PrimField<std::string::String>,
}

impl BuildCognitoLogDeliveryConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> CognitoLogDeliveryConfiguration {
        let out = CognitoLogDeliveryConfiguration(std::rc::Rc::new(CognitoLogDeliveryConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CognitoLogDeliveryConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                region: core::default::Default::default(),
                user_pool_id: self.user_pool_id,
                log_configurations: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CognitoLogDeliveryConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoLogDeliveryConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CognitoLogDeliveryConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_configurations` after provisioning.\n"]
    pub fn log_configurations(&self) -> terrars::ListRef<CognitoLogDeliveryConfiguration_LogConfigurationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_configurations", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CognitoLogDeliveryConfigurationDynamic {
    log_configurations: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoLogDeliveryConfiguration_LogConfigurations>>,
}

#[derive(Serialize)]
pub struct CognitoLogDeliveryConfiguration_LogConfigurations_CloudWatchLogsConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CognitoLogDeliveryConfiguration_LogConfigurations_CloudWatchLogsConfiguration {
    #[doc = "Set the field `log_group_arn`.\n"]
    pub fn set_log_group_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_group_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoLogDeliveryConfiguration_LogConfigurations_CloudWatchLogsConfiguration {
    type O = terrars::BlockAssignable<CognitoLogDeliveryConfiguration_LogConfigurations_CloudWatchLogsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoLogDeliveryConfiguration_LogConfigurations_CloudWatchLogsConfiguration {}

impl BuildCognitoLogDeliveryConfiguration_LogConfigurations_CloudWatchLogsConfiguration {
    pub fn build(self) -> CognitoLogDeliveryConfiguration_LogConfigurations_CloudWatchLogsConfiguration {
        CognitoLogDeliveryConfiguration_LogConfigurations_CloudWatchLogsConfiguration {
            log_group_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoLogDeliveryConfiguration_LogConfigurations_CloudWatchLogsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoLogDeliveryConfiguration_LogConfigurations_CloudWatchLogsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoLogDeliveryConfiguration_LogConfigurations_CloudWatchLogsConfigurationRef {
        CognitoLogDeliveryConfiguration_LogConfigurations_CloudWatchLogsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoLogDeliveryConfiguration_LogConfigurations_CloudWatchLogsConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group_arn` after provisioning.\n"]
    pub fn log_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoLogDeliveryConfiguration_LogConfigurations_FirehoseConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CognitoLogDeliveryConfiguration_LogConfigurations_FirehoseConfiguration {
    #[doc = "Set the field `stream_arn`.\n"]
    pub fn set_stream_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stream_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoLogDeliveryConfiguration_LogConfigurations_FirehoseConfiguration {
    type O = terrars::BlockAssignable<CognitoLogDeliveryConfiguration_LogConfigurations_FirehoseConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoLogDeliveryConfiguration_LogConfigurations_FirehoseConfiguration {}

impl BuildCognitoLogDeliveryConfiguration_LogConfigurations_FirehoseConfiguration {
    pub fn build(self) -> CognitoLogDeliveryConfiguration_LogConfigurations_FirehoseConfiguration {
        CognitoLogDeliveryConfiguration_LogConfigurations_FirehoseConfiguration {
            stream_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoLogDeliveryConfiguration_LogConfigurations_FirehoseConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoLogDeliveryConfiguration_LogConfigurations_FirehoseConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoLogDeliveryConfiguration_LogConfigurations_FirehoseConfigurationRef {
        CognitoLogDeliveryConfiguration_LogConfigurations_FirehoseConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoLogDeliveryConfiguration_LogConfigurations_FirehoseConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoLogDeliveryConfiguration_LogConfigurations_S3Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CognitoLogDeliveryConfiguration_LogConfigurations_S3Configuration {
    #[doc = "Set the field `bucket_arn`.\n"]
    pub fn set_bucket_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoLogDeliveryConfiguration_LogConfigurations_S3Configuration {
    type O = terrars::BlockAssignable<CognitoLogDeliveryConfiguration_LogConfigurations_S3Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoLogDeliveryConfiguration_LogConfigurations_S3Configuration {}

impl BuildCognitoLogDeliveryConfiguration_LogConfigurations_S3Configuration {
    pub fn build(self) -> CognitoLogDeliveryConfiguration_LogConfigurations_S3Configuration {
        CognitoLogDeliveryConfiguration_LogConfigurations_S3Configuration {
            bucket_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoLogDeliveryConfiguration_LogConfigurations_S3ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoLogDeliveryConfiguration_LogConfigurations_S3ConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoLogDeliveryConfiguration_LogConfigurations_S3ConfigurationRef {
        CognitoLogDeliveryConfiguration_LogConfigurations_S3ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoLogDeliveryConfiguration_LogConfigurations_S3ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoLogDeliveryConfiguration_LogConfigurationsDynamic {
    cloud_watch_logs_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoLogDeliveryConfiguration_LogConfigurations_CloudWatchLogsConfiguration>>,
    firehose_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoLogDeliveryConfiguration_LogConfigurations_FirehoseConfiguration>>,
    s3_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoLogDeliveryConfiguration_LogConfigurations_S3Configuration>>,
}

#[derive(Serialize)]
pub struct CognitoLogDeliveryConfiguration_LogConfigurations {
    event_source: terrars::PrimField<std::string::String>,
    log_level: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_watch_logs_configuration: std
    ::option
    ::Option<std::vec::Vec<CognitoLogDeliveryConfiguration_LogConfigurations_CloudWatchLogsConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firehose_configuration: std
    ::option
    ::Option<std::vec::Vec<CognitoLogDeliveryConfiguration_LogConfigurations_FirehoseConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_configuration: std
    ::option
    ::Option<std::vec::Vec<CognitoLogDeliveryConfiguration_LogConfigurations_S3Configuration>>,
    dynamic: CognitoLogDeliveryConfiguration_LogConfigurationsDynamic,
}

impl CognitoLogDeliveryConfiguration_LogConfigurations {
    #[doc = "Set the field `cloud_watch_logs_configuration`.\n"]
    pub fn set_cloud_watch_logs_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CognitoLogDeliveryConfiguration_LogConfigurations_CloudWatchLogsConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_watch_logs_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_watch_logs_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `firehose_configuration`.\n"]
    pub fn set_firehose_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CognitoLogDeliveryConfiguration_LogConfigurations_FirehoseConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.firehose_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.firehose_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CognitoLogDeliveryConfiguration_LogConfigurations_S3Configuration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CognitoLogDeliveryConfiguration_LogConfigurations {
    type O = terrars::BlockAssignable<CognitoLogDeliveryConfiguration_LogConfigurations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoLogDeliveryConfiguration_LogConfigurations {
    #[doc = ""]
    pub event_source: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub log_level: terrars::PrimField<std::string::String>,
}

impl BuildCognitoLogDeliveryConfiguration_LogConfigurations {
    pub fn build(self) -> CognitoLogDeliveryConfiguration_LogConfigurations {
        CognitoLogDeliveryConfiguration_LogConfigurations {
            event_source: self.event_source,
            log_level: self.log_level,
            cloud_watch_logs_configuration: core::default::Default::default(),
            firehose_configuration: core::default::Default::default(),
            s3_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoLogDeliveryConfiguration_LogConfigurationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoLogDeliveryConfiguration_LogConfigurationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoLogDeliveryConfiguration_LogConfigurationsRef {
        CognitoLogDeliveryConfiguration_LogConfigurationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoLogDeliveryConfiguration_LogConfigurationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_source` after provisioning.\n"]
    pub fn event_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_source", self.base))
    }

    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_watch_logs_configuration` after provisioning.\n"]
    pub fn cloud_watch_logs_configuration(
        &self,
    ) -> terrars::ListRef<CognitoLogDeliveryConfiguration_LogConfigurations_CloudWatchLogsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_watch_logs_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `firehose_configuration` after provisioning.\n"]
    pub fn firehose_configuration(
        &self,
    ) -> terrars::ListRef<CognitoLogDeliveryConfiguration_LogConfigurations_FirehoseConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firehose_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> terrars::ListRef<CognitoLogDeliveryConfiguration_LogConfigurations_S3ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.base))
    }
}
