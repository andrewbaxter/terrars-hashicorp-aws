#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct IvschatLoggingConfigurationData_ {
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
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_configuration: std::option::Option<std::vec::Vec<IvschatLoggingConfiguration_DestinationConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<IvschatLoggingConfiguration_Timeouts>,
    dynamic: IvschatLoggingConfigurationDynamic,
}

struct IvschatLoggingConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IvschatLoggingConfigurationData_>,
}

#[derive(Clone)]
pub struct IvschatLoggingConfiguration(std::rc::Rc<IvschatLoggingConfiguration_>);

impl IvschatLoggingConfiguration {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `destination_configuration`.\n"]
    pub fn set_destination_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IvschatLoggingConfiguration_DestinationConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<IvschatLoggingConfiguration_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_configuration` after provisioning.\n"]
    pub fn destination_configuration(&self) -> terrars::ListRef<IvschatLoggingConfiguration_DestinationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IvschatLoggingConfiguration_TimeoutsRef {
        IvschatLoggingConfiguration_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for IvschatLoggingConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IvschatLoggingConfiguration { }

impl terrars::ToListMappable for IvschatLoggingConfiguration {
    type O = terrars::ListRef<IvschatLoggingConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IvschatLoggingConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ivschat_logging_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIvschatLoggingConfiguration {
    pub tf_id: std::string::String,
}

impl BuildIvschatLoggingConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> IvschatLoggingConfiguration {
        let out = IvschatLoggingConfiguration(std::rc::Rc::new(IvschatLoggingConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IvschatLoggingConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                destination_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IvschatLoggingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IvschatLoggingConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IvschatLoggingConfigurationRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_configuration` after provisioning.\n"]
    pub fn destination_configuration(&self) -> terrars::ListRef<IvschatLoggingConfiguration_DestinationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IvschatLoggingConfiguration_TimeoutsRef {
        IvschatLoggingConfiguration_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct IvschatLoggingConfigurationDynamic {
    destination_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<IvschatLoggingConfiguration_DestinationConfiguration>>,
}

#[derive(Serialize)]
pub struct IvschatLoggingConfiguration_DestinationConfiguration_CloudwatchLogs {
    log_group_name: terrars::PrimField<std::string::String>,
}

impl IvschatLoggingConfiguration_DestinationConfiguration_CloudwatchLogs { }

impl terrars::ToListMappable for IvschatLoggingConfiguration_DestinationConfiguration_CloudwatchLogs {
    type O = terrars::BlockAssignable<IvschatLoggingConfiguration_DestinationConfiguration_CloudwatchLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIvschatLoggingConfiguration_DestinationConfiguration_CloudwatchLogs {
    #[doc = ""]
    pub log_group_name: terrars::PrimField<std::string::String>,
}

impl BuildIvschatLoggingConfiguration_DestinationConfiguration_CloudwatchLogs {
    pub fn build(self) -> IvschatLoggingConfiguration_DestinationConfiguration_CloudwatchLogs {
        IvschatLoggingConfiguration_DestinationConfiguration_CloudwatchLogs { log_group_name: self.log_group_name }
    }
}

#[allow(dead_code)]
pub struct IvschatLoggingConfiguration_DestinationConfiguration_CloudwatchLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IvschatLoggingConfiguration_DestinationConfiguration_CloudwatchLogsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IvschatLoggingConfiguration_DestinationConfiguration_CloudwatchLogsRef {
        IvschatLoggingConfiguration_DestinationConfiguration_CloudwatchLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IvschatLoggingConfiguration_DestinationConfiguration_CloudwatchLogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }
}

#[derive(Serialize)]
pub struct IvschatLoggingConfiguration_DestinationConfiguration_Firehose {
    delivery_stream_name: terrars::PrimField<std::string::String>,
}

impl IvschatLoggingConfiguration_DestinationConfiguration_Firehose { }

impl terrars::ToListMappable for IvschatLoggingConfiguration_DestinationConfiguration_Firehose {
    type O = terrars::BlockAssignable<IvschatLoggingConfiguration_DestinationConfiguration_Firehose>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIvschatLoggingConfiguration_DestinationConfiguration_Firehose {
    #[doc = ""]
    pub delivery_stream_name: terrars::PrimField<std::string::String>,
}

impl BuildIvschatLoggingConfiguration_DestinationConfiguration_Firehose {
    pub fn build(self) -> IvschatLoggingConfiguration_DestinationConfiguration_Firehose {
        IvschatLoggingConfiguration_DestinationConfiguration_Firehose {
            delivery_stream_name: self.delivery_stream_name,
        }
    }
}

#[allow(dead_code)]
pub struct IvschatLoggingConfiguration_DestinationConfiguration_FirehoseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IvschatLoggingConfiguration_DestinationConfiguration_FirehoseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IvschatLoggingConfiguration_DestinationConfiguration_FirehoseRef {
        IvschatLoggingConfiguration_DestinationConfiguration_FirehoseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IvschatLoggingConfiguration_DestinationConfiguration_FirehoseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delivery_stream_name` after provisioning.\n"]
    pub fn delivery_stream_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delivery_stream_name", self.base))
    }
}

#[derive(Serialize)]
pub struct IvschatLoggingConfiguration_DestinationConfiguration_S3 {
    bucket_name: terrars::PrimField<std::string::String>,
}

impl IvschatLoggingConfiguration_DestinationConfiguration_S3 { }

impl terrars::ToListMappable for IvschatLoggingConfiguration_DestinationConfiguration_S3 {
    type O = terrars::BlockAssignable<IvschatLoggingConfiguration_DestinationConfiguration_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIvschatLoggingConfiguration_DestinationConfiguration_S3 {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildIvschatLoggingConfiguration_DestinationConfiguration_S3 {
    pub fn build(self) -> IvschatLoggingConfiguration_DestinationConfiguration_S3 {
        IvschatLoggingConfiguration_DestinationConfiguration_S3 { bucket_name: self.bucket_name }
    }
}

#[allow(dead_code)]
pub struct IvschatLoggingConfiguration_DestinationConfiguration_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IvschatLoggingConfiguration_DestinationConfiguration_S3Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IvschatLoggingConfiguration_DestinationConfiguration_S3Ref {
        IvschatLoggingConfiguration_DestinationConfiguration_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IvschatLoggingConfiguration_DestinationConfiguration_S3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct IvschatLoggingConfiguration_DestinationConfigurationDynamic {
    cloudwatch_logs: std
    ::option
    ::Option<terrars::DynamicBlock<IvschatLoggingConfiguration_DestinationConfiguration_CloudwatchLogs>>,
    firehose: std
    ::option
    ::Option<terrars::DynamicBlock<IvschatLoggingConfiguration_DestinationConfiguration_Firehose>>,
    s3: std::option::Option<terrars::DynamicBlock<IvschatLoggingConfiguration_DestinationConfiguration_S3>>,
}

#[derive(Serialize)]
pub struct IvschatLoggingConfiguration_DestinationConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs: std
    ::option
    ::Option<std::vec::Vec<IvschatLoggingConfiguration_DestinationConfiguration_CloudwatchLogs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firehose: std::option::Option<std::vec::Vec<IvschatLoggingConfiguration_DestinationConfiguration_Firehose>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<IvschatLoggingConfiguration_DestinationConfiguration_S3>>,
    dynamic: IvschatLoggingConfiguration_DestinationConfigurationDynamic,
}

impl IvschatLoggingConfiguration_DestinationConfiguration {
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<IvschatLoggingConfiguration_DestinationConfiguration_CloudwatchLogs>,
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

    #[doc = "Set the field `firehose`.\n"]
    pub fn set_firehose(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IvschatLoggingConfiguration_DestinationConfiguration_Firehose>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.firehose = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.firehose = std::option::Option::Some(d);
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
                    ::Into<terrars::BlockAssignable<IvschatLoggingConfiguration_DestinationConfiguration_S3>>,
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

impl terrars::ToListMappable for IvschatLoggingConfiguration_DestinationConfiguration {
    type O = terrars::BlockAssignable<IvschatLoggingConfiguration_DestinationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIvschatLoggingConfiguration_DestinationConfiguration {}

impl BuildIvschatLoggingConfiguration_DestinationConfiguration {
    pub fn build(self) -> IvschatLoggingConfiguration_DestinationConfiguration {
        IvschatLoggingConfiguration_DestinationConfiguration {
            cloudwatch_logs: core::default::Default::default(),
            firehose: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IvschatLoggingConfiguration_DestinationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IvschatLoggingConfiguration_DestinationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IvschatLoggingConfiguration_DestinationConfigurationRef {
        IvschatLoggingConfiguration_DestinationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IvschatLoggingConfiguration_DestinationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(
        &self,
    ) -> terrars::ListRef<IvschatLoggingConfiguration_DestinationConfiguration_CloudwatchLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `firehose` after provisioning.\n"]
    pub fn firehose(&self) -> terrars::ListRef<IvschatLoggingConfiguration_DestinationConfiguration_FirehoseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firehose", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<IvschatLoggingConfiguration_DestinationConfiguration_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize)]
pub struct IvschatLoggingConfiguration_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IvschatLoggingConfiguration_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IvschatLoggingConfiguration_Timeouts {
    type O = terrars::BlockAssignable<IvschatLoggingConfiguration_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIvschatLoggingConfiguration_Timeouts {}

impl BuildIvschatLoggingConfiguration_Timeouts {
    pub fn build(self) -> IvschatLoggingConfiguration_Timeouts {
        IvschatLoggingConfiguration_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IvschatLoggingConfiguration_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IvschatLoggingConfiguration_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IvschatLoggingConfiguration_TimeoutsRef {
        IvschatLoggingConfiguration_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IvschatLoggingConfiguration_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
