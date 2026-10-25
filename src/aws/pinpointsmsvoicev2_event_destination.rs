#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Pinpointsmsvoicev2EventDestinationData_ {
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
    configuration_set_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    event_destination_name: terrars::PrimField<std::string::String>,
    matching_event_types: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs_destination: std
    ::option
    ::Option<std::vec::Vec<Pinpointsmsvoicev2EventDestination_CloudwatchLogsDestination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_firehose_destination: std
    ::option
    ::Option<std::vec::Vec<Pinpointsmsvoicev2EventDestination_KinesisFirehoseDestination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sns_destination: std::option::Option<std::vec::Vec<Pinpointsmsvoicev2EventDestination_SnsDestination>>,
    dynamic: Pinpointsmsvoicev2EventDestinationDynamic,
}

struct Pinpointsmsvoicev2EventDestination_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Pinpointsmsvoicev2EventDestinationData_>,
}

#[derive(Clone)]
pub struct Pinpointsmsvoicev2EventDestination(std::rc::Rc<Pinpointsmsvoicev2EventDestination_>);

impl Pinpointsmsvoicev2EventDestination {
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

    #[doc = "Set the field `enabled`.\nWhether the event destination is enabled. Defaults to `true`."]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_logs_destination`.\n"]
    pub fn set_cloudwatch_logs_destination(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Pinpointsmsvoicev2EventDestination_CloudwatchLogsDestination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloudwatch_logs_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloudwatch_logs_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_firehose_destination`.\n"]
    pub fn set_kinesis_firehose_destination(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Pinpointsmsvoicev2EventDestination_KinesisFirehoseDestination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kinesis_firehose_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kinesis_firehose_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sns_destination`.\n"]
    pub fn set_sns_destination(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Pinpointsmsvoicev2EventDestination_SnsDestination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sns_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sns_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `configuration_set_arn` after provisioning.\nARN of the parent configuration set."]
    pub fn configuration_set_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_set_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `configuration_set_name` after provisioning.\nName of the configuration set this event destination belongs to."]
    pub fn configuration_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_set_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the event destination is enabled. Defaults to `true`."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `event_destination_name` after provisioning.\nName of the event destination."]
    pub fn event_destination_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_destination_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `matching_event_types` after provisioning.\nEvent types for which the destination receives records."]
    pub fn matching_event_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.matching_event_types", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs_destination` after provisioning.\n"]
    pub fn cloudwatch_logs_destination(
        &self,
    ) -> terrars::ListRef<Pinpointsmsvoicev2EventDestination_CloudwatchLogsDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kinesis_firehose_destination` after provisioning.\n"]
    pub fn kinesis_firehose_destination(
        &self,
    ) -> terrars::ListRef<Pinpointsmsvoicev2EventDestination_KinesisFirehoseDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_firehose_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns_destination` after provisioning.\n"]
    pub fn sns_destination(&self) -> terrars::ListRef<Pinpointsmsvoicev2EventDestination_SnsDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sns_destination", self.extract_ref()))
    }
}

impl terrars::Referable for Pinpointsmsvoicev2EventDestination {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Pinpointsmsvoicev2EventDestination { }

impl terrars::ToListMappable for Pinpointsmsvoicev2EventDestination {
    type O = terrars::ListRef<Pinpointsmsvoicev2EventDestinationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Pinpointsmsvoicev2EventDestination_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_pinpointsmsvoicev2_event_destination".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPinpointsmsvoicev2EventDestination {
    pub tf_id: std::string::String,
    #[doc = "Name of the configuration set this event destination belongs to."]
    pub configuration_set_name: terrars::PrimField<std::string::String>,
    #[doc = "Name of the event destination."]
    pub event_destination_name: terrars::PrimField<std::string::String>,
    #[doc = "Event types for which the destination receives records."]
    pub matching_event_types: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildPinpointsmsvoicev2EventDestination {
    pub fn build(self, stack: &mut terrars::Stack) -> Pinpointsmsvoicev2EventDestination {
        let out = Pinpointsmsvoicev2EventDestination(std::rc::Rc::new(Pinpointsmsvoicev2EventDestination_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Pinpointsmsvoicev2EventDestinationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                configuration_set_name: self.configuration_set_name,
                enabled: core::default::Default::default(),
                event_destination_name: self.event_destination_name,
                matching_event_types: self.matching_event_types,
                region: core::default::Default::default(),
                cloudwatch_logs_destination: core::default::Default::default(),
                kinesis_firehose_destination: core::default::Default::default(),
                sns_destination: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Pinpointsmsvoicev2EventDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Pinpointsmsvoicev2EventDestinationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Pinpointsmsvoicev2EventDestinationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `configuration_set_arn` after provisioning.\nARN of the parent configuration set."]
    pub fn configuration_set_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_set_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `configuration_set_name` after provisioning.\nName of the configuration set this event destination belongs to."]
    pub fn configuration_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_set_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the event destination is enabled. Defaults to `true`."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `event_destination_name` after provisioning.\nName of the event destination."]
    pub fn event_destination_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_destination_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `matching_event_types` after provisioning.\nEvent types for which the destination receives records."]
    pub fn matching_event_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.matching_event_types", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs_destination` after provisioning.\n"]
    pub fn cloudwatch_logs_destination(
        &self,
    ) -> terrars::ListRef<Pinpointsmsvoicev2EventDestination_CloudwatchLogsDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kinesis_firehose_destination` after provisioning.\n"]
    pub fn kinesis_firehose_destination(
        &self,
    ) -> terrars::ListRef<Pinpointsmsvoicev2EventDestination_KinesisFirehoseDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_firehose_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns_destination` after provisioning.\n"]
    pub fn sns_destination(&self) -> terrars::ListRef<Pinpointsmsvoicev2EventDestination_SnsDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sns_destination", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Pinpointsmsvoicev2EventDestinationDynamic {
    cloudwatch_logs_destination: std
    ::option
    ::Option<terrars::DynamicBlock<Pinpointsmsvoicev2EventDestination_CloudwatchLogsDestination>>,
    kinesis_firehose_destination: std
    ::option
    ::Option<terrars::DynamicBlock<Pinpointsmsvoicev2EventDestination_KinesisFirehoseDestination>>,
    sns_destination: std::option::Option<terrars::DynamicBlock<Pinpointsmsvoicev2EventDestination_SnsDestination>>,
}

#[derive(Serialize)]
pub struct Pinpointsmsvoicev2EventDestination_CloudwatchLogsDestination {
    iam_role_arn: terrars::PrimField<std::string::String>,
    log_group_arn: terrars::PrimField<std::string::String>,
}

impl Pinpointsmsvoicev2EventDestination_CloudwatchLogsDestination { }

impl terrars::ToListMappable for Pinpointsmsvoicev2EventDestination_CloudwatchLogsDestination {
    type O = terrars::BlockAssignable<Pinpointsmsvoicev2EventDestination_CloudwatchLogsDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPinpointsmsvoicev2EventDestination_CloudwatchLogsDestination {
    #[doc = "ARN of the IAM role that End User Messaging SMS assumes to write to the log group."]
    pub iam_role_arn: terrars::PrimField<std::string::String>,
    #[doc = "ARN of the Amazon CloudWatch log group that receives the events."]
    pub log_group_arn: terrars::PrimField<std::string::String>,
}

impl BuildPinpointsmsvoicev2EventDestination_CloudwatchLogsDestination {
    pub fn build(self) -> Pinpointsmsvoicev2EventDestination_CloudwatchLogsDestination {
        Pinpointsmsvoicev2EventDestination_CloudwatchLogsDestination {
            iam_role_arn: self.iam_role_arn,
            log_group_arn: self.log_group_arn,
        }
    }
}

#[allow(dead_code)]
pub struct Pinpointsmsvoicev2EventDestination_CloudwatchLogsDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Pinpointsmsvoicev2EventDestination_CloudwatchLogsDestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Pinpointsmsvoicev2EventDestination_CloudwatchLogsDestinationRef {
        Pinpointsmsvoicev2EventDestination_CloudwatchLogsDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Pinpointsmsvoicev2EventDestination_CloudwatchLogsDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `iam_role_arn` after provisioning.\nARN of the IAM role that End User Messaging SMS assumes to write to the log group."]
    pub fn iam_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `log_group_arn` after provisioning.\nARN of the Amazon CloudWatch log group that receives the events."]
    pub fn log_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Pinpointsmsvoicev2EventDestination_KinesisFirehoseDestination {
    delivery_stream_arn: terrars::PrimField<std::string::String>,
    iam_role_arn: terrars::PrimField<std::string::String>,
}

impl Pinpointsmsvoicev2EventDestination_KinesisFirehoseDestination { }

impl terrars::ToListMappable for Pinpointsmsvoicev2EventDestination_KinesisFirehoseDestination {
    type O = terrars::BlockAssignable<Pinpointsmsvoicev2EventDestination_KinesisFirehoseDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPinpointsmsvoicev2EventDestination_KinesisFirehoseDestination {
    #[doc = "ARN of the Amazon Data Firehose delivery stream that receives the events."]
    pub delivery_stream_arn: terrars::PrimField<std::string::String>,
    #[doc = "ARN of the IAM role that End User Messaging SMS assumes to write to the delivery stream."]
    pub iam_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildPinpointsmsvoicev2EventDestination_KinesisFirehoseDestination {
    pub fn build(self) -> Pinpointsmsvoicev2EventDestination_KinesisFirehoseDestination {
        Pinpointsmsvoicev2EventDestination_KinesisFirehoseDestination {
            delivery_stream_arn: self.delivery_stream_arn,
            iam_role_arn: self.iam_role_arn,
        }
    }
}

#[allow(dead_code)]
pub struct Pinpointsmsvoicev2EventDestination_KinesisFirehoseDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Pinpointsmsvoicev2EventDestination_KinesisFirehoseDestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Pinpointsmsvoicev2EventDestination_KinesisFirehoseDestinationRef {
        Pinpointsmsvoicev2EventDestination_KinesisFirehoseDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Pinpointsmsvoicev2EventDestination_KinesisFirehoseDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `delivery_stream_arn` after provisioning.\nARN of the Amazon Data Firehose delivery stream that receives the events."]
    pub fn delivery_stream_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delivery_stream_arn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `iam_role_arn` after provisioning.\nARN of the IAM role that End User Messaging SMS assumes to write to the delivery stream."]
    pub fn iam_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Pinpointsmsvoicev2EventDestination_SnsDestination {
    topic_arn: terrars::PrimField<std::string::String>,
}

impl Pinpointsmsvoicev2EventDestination_SnsDestination { }

impl terrars::ToListMappable for Pinpointsmsvoicev2EventDestination_SnsDestination {
    type O = terrars::BlockAssignable<Pinpointsmsvoicev2EventDestination_SnsDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPinpointsmsvoicev2EventDestination_SnsDestination {
    #[doc = "ARN of the Amazon SNS topic that receives the events."]
    pub topic_arn: terrars::PrimField<std::string::String>,
}

impl BuildPinpointsmsvoicev2EventDestination_SnsDestination {
    pub fn build(self) -> Pinpointsmsvoicev2EventDestination_SnsDestination {
        Pinpointsmsvoicev2EventDestination_SnsDestination { topic_arn: self.topic_arn }
    }
}

#[allow(dead_code)]
pub struct Pinpointsmsvoicev2EventDestination_SnsDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Pinpointsmsvoicev2EventDestination_SnsDestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Pinpointsmsvoicev2EventDestination_SnsDestinationRef {
        Pinpointsmsvoicev2EventDestination_SnsDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Pinpointsmsvoicev2EventDestination_SnsDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `topic_arn` after provisioning.\nARN of the Amazon SNS topic that receives the events."]
    pub fn topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}
