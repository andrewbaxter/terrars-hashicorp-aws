#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct IotTopicRuleData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    sql: terrars::PrimField<std::string::String>,
    sql_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_alarm: std::option::Option<std::vec::Vec<IotTopicRule_CloudwatchAlarm>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs: std::option::Option<std::vec::Vec<IotTopicRule_CloudwatchLogs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_metric: std::option::Option<std::vec::Vec<IotTopicRule_CloudwatchMetric>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynamodb: std::option::Option<std::vec::Vec<IotTopicRule_Dynamodb>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynamodbv2: std::option::Option<std::vec::Vec<IotTopicRule_Dynamodbv2>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elasticsearch: std::option::Option<std::vec::Vec<IotTopicRule_Elasticsearch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_action: std::option::Option<std::vec::Vec<IotTopicRule_ErrorAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firehose: std::option::Option<std::vec::Vec<IotTopicRule_Firehose>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http: std::option::Option<std::vec::Vec<IotTopicRule_Http>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iot_analytics: std::option::Option<std::vec::Vec<IotTopicRule_IotAnalytics>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iot_events: std::option::Option<std::vec::Vec<IotTopicRule_IotEvents>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kafka: std::option::Option<std::vec::Vec<IotTopicRule_Kafka>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis: std::option::Option<std::vec::Vec<IotTopicRule_Kinesis>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda: std::option::Option<std::vec::Vec<IotTopicRule_Lambda>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    republish: std::option::Option<std::vec::Vec<IotTopicRule_Republish>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<IotTopicRule_S3>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sns: std::option::Option<std::vec::Vec<IotTopicRule_Sns>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sqs: std::option::Option<std::vec::Vec<IotTopicRule_Sqs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step_functions: std::option::Option<std::vec::Vec<IotTopicRule_StepFunctions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestream: std::option::Option<std::vec::Vec<IotTopicRule_Timestream>>,
    dynamic: IotTopicRuleDynamic,
}

struct IotTopicRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IotTopicRuleData_>,
}

#[derive(Clone)]
pub struct IotTopicRule(std::rc::Rc<IotTopicRule_>);

impl IotTopicRule {
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

    #[doc = "Set the field `cloudwatch_alarm`.\n"]
    pub fn set_cloudwatch_alarm(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_CloudwatchAlarm>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloudwatch_alarm = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloudwatch_alarm = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_CloudwatchLogs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloudwatch_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloudwatch_logs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloudwatch_metric`.\n"]
    pub fn set_cloudwatch_metric(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_CloudwatchMetric>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloudwatch_metric = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloudwatch_metric = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dynamodb`.\n"]
    pub fn set_dynamodb(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Dynamodb>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dynamodb = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dynamodb = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dynamodbv2`.\n"]
    pub fn set_dynamodbv2(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Dynamodbv2>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dynamodbv2 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dynamodbv2 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `elasticsearch`.\n"]
    pub fn set_elasticsearch(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Elasticsearch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().elasticsearch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.elasticsearch = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `error_action`.\n"]
    pub fn set_error_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_ErrorAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().error_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.error_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `firehose`.\n"]
    pub fn set_firehose(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Firehose>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().firehose = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.firehose = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http`.\n"]
    pub fn set_http(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Http>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().http = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.http = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `iot_analytics`.\n"]
    pub fn set_iot_analytics(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_IotAnalytics>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().iot_analytics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.iot_analytics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `iot_events`.\n"]
    pub fn set_iot_events(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_IotEvents>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().iot_events = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.iot_events = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kafka`.\n"]
    pub fn set_kafka(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Kafka>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kafka = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kafka = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis`.\n"]
    pub fn set_kinesis(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Kinesis>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kinesis = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kinesis = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda`.\n"]
    pub fn set_lambda(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Lambda>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lambda = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lambda = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `republish`.\n"]
    pub fn set_republish(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Republish>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().republish = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.republish = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_S3>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sns`.\n"]
    pub fn set_sns(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Sns>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sns = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sqs`.\n"]
    pub fn set_sqs(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Sqs>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sqs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sqs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `step_functions`.\n"]
    pub fn set_step_functions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_StepFunctions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().step_functions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.step_functions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timestream`.\n"]
    pub fn set_timestream(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Timestream>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().timestream = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.timestream = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `sql` after provisioning.\n"]
    pub fn sql(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sql_version` after provisioning.\n"]
    pub fn sql_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_action` after provisioning.\n"]
    pub fn error_action(&self) -> terrars::ListRef<IotTopicRule_ErrorActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_action", self.extract_ref()))
    }
}

impl terrars::Referable for IotTopicRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IotTopicRule { }

impl terrars::ToListMappable for IotTopicRule {
    type O = terrars::ListRef<IotTopicRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IotTopicRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_iot_topic_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIotTopicRule {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sql: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sql_version: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule {
    pub fn build(self, stack: &mut terrars::Stack) -> IotTopicRule {
        let out = IotTopicRule(std::rc::Rc::new(IotTopicRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IotTopicRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                enabled: self.enabled,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                sql: self.sql,
                sql_version: self.sql_version,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                cloudwatch_alarm: core::default::Default::default(),
                cloudwatch_logs: core::default::Default::default(),
                cloudwatch_metric: core::default::Default::default(),
                dynamodb: core::default::Default::default(),
                dynamodbv2: core::default::Default::default(),
                elasticsearch: core::default::Default::default(),
                error_action: core::default::Default::default(),
                firehose: core::default::Default::default(),
                http: core::default::Default::default(),
                iot_analytics: core::default::Default::default(),
                iot_events: core::default::Default::default(),
                kafka: core::default::Default::default(),
                kinesis: core::default::Default::default(),
                lambda: core::default::Default::default(),
                republish: core::default::Default::default(),
                s3: core::default::Default::default(),
                sns: core::default::Default::default(),
                sqs: core::default::Default::default(),
                step_functions: core::default::Default::default(),
                timestream: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IotTopicRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IotTopicRuleRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `sql` after provisioning.\n"]
    pub fn sql(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sql_version` after provisioning.\n"]
    pub fn sql_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_action` after provisioning.\n"]
    pub fn error_action(&self) -> terrars::ListRef<IotTopicRule_ErrorActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_action", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct IotTopicRuleDynamic {
    cloudwatch_alarm: std::option::Option<terrars::DynamicBlock<IotTopicRule_CloudwatchAlarm>>,
    cloudwatch_logs: std::option::Option<terrars::DynamicBlock<IotTopicRule_CloudwatchLogs>>,
    cloudwatch_metric: std::option::Option<terrars::DynamicBlock<IotTopicRule_CloudwatchMetric>>,
    dynamodb: std::option::Option<terrars::DynamicBlock<IotTopicRule_Dynamodb>>,
    dynamodbv2: std::option::Option<terrars::DynamicBlock<IotTopicRule_Dynamodbv2>>,
    elasticsearch: std::option::Option<terrars::DynamicBlock<IotTopicRule_Elasticsearch>>,
    error_action: std::option::Option<terrars::DynamicBlock<IotTopicRule_ErrorAction>>,
    firehose: std::option::Option<terrars::DynamicBlock<IotTopicRule_Firehose>>,
    http: std::option::Option<terrars::DynamicBlock<IotTopicRule_Http>>,
    iot_analytics: std::option::Option<terrars::DynamicBlock<IotTopicRule_IotAnalytics>>,
    iot_events: std::option::Option<terrars::DynamicBlock<IotTopicRule_IotEvents>>,
    kafka: std::option::Option<terrars::DynamicBlock<IotTopicRule_Kafka>>,
    kinesis: std::option::Option<terrars::DynamicBlock<IotTopicRule_Kinesis>>,
    lambda: std::option::Option<terrars::DynamicBlock<IotTopicRule_Lambda>>,
    republish: std::option::Option<terrars::DynamicBlock<IotTopicRule_Republish>>,
    s3: std::option::Option<terrars::DynamicBlock<IotTopicRule_S3>>,
    sns: std::option::Option<terrars::DynamicBlock<IotTopicRule_Sns>>,
    sqs: std::option::Option<terrars::DynamicBlock<IotTopicRule_Sqs>>,
    step_functions: std::option::Option<terrars::DynamicBlock<IotTopicRule_StepFunctions>>,
    timestream: std::option::Option<terrars::DynamicBlock<IotTopicRule_Timestream>>,
}

#[derive(Serialize)]
pub struct IotTopicRule_CloudwatchAlarm {
    alarm_name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    state_reason: terrars::PrimField<std::string::String>,
    state_value: terrars::PrimField<std::string::String>,
}

impl IotTopicRule_CloudwatchAlarm { }

impl terrars::ToListMappable for IotTopicRule_CloudwatchAlarm {
    type O = terrars::BlockAssignable<IotTopicRule_CloudwatchAlarm>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_CloudwatchAlarm {
    #[doc = ""]
    pub alarm_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub state_reason: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub state_value: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_CloudwatchAlarm {
    pub fn build(self) -> IotTopicRule_CloudwatchAlarm {
        IotTopicRule_CloudwatchAlarm {
            alarm_name: self.alarm_name,
            role_arn: self.role_arn,
            state_reason: self.state_reason,
            state_value: self.state_value,
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_CloudwatchAlarmRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_CloudwatchAlarmRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_CloudwatchAlarmRef {
        IotTopicRule_CloudwatchAlarmRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_CloudwatchAlarmRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alarm_name` after provisioning.\n"]
    pub fn alarm_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alarm_name", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `state_reason` after provisioning.\n"]
    pub fn state_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_reason", self.base))
    }

    #[doc = "Get a reference to the value of field `state_value` after provisioning.\n"]
    pub fn state_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_value", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRule_CloudwatchLogs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_mode: std::option::Option<terrars::PrimField<bool>>,
    log_group_name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl IotTopicRule_CloudwatchLogs {
    #[doc = "Set the field `batch_mode`.\n"]
    pub fn set_batch_mode(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.batch_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRule_CloudwatchLogs {
    type O = terrars::BlockAssignable<IotTopicRule_CloudwatchLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_CloudwatchLogs {
    #[doc = ""]
    pub log_group_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_CloudwatchLogs {
    pub fn build(self) -> IotTopicRule_CloudwatchLogs {
        IotTopicRule_CloudwatchLogs {
            batch_mode: core::default::Default::default(),
            log_group_name: self.log_group_name,
            role_arn: self.role_arn,
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_CloudwatchLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_CloudwatchLogsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_CloudwatchLogsRef {
        IotTopicRule_CloudwatchLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_CloudwatchLogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `batch_mode` after provisioning.\n"]
    pub fn batch_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRule_CloudwatchMetric {
    metric_name: terrars::PrimField<std::string::String>,
    metric_namespace: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_timestamp: std::option::Option<terrars::PrimField<std::string::String>>,
    metric_unit: terrars::PrimField<std::string::String>,
    metric_value: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl IotTopicRule_CloudwatchMetric {
    #[doc = "Set the field `metric_timestamp`.\n"]
    pub fn set_metric_timestamp(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metric_timestamp = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRule_CloudwatchMetric {
    type O = terrars::BlockAssignable<IotTopicRule_CloudwatchMetric>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_CloudwatchMetric {
    #[doc = ""]
    pub metric_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub metric_namespace: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub metric_unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub metric_value: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_CloudwatchMetric {
    pub fn build(self) -> IotTopicRule_CloudwatchMetric {
        IotTopicRule_CloudwatchMetric {
            metric_name: self.metric_name,
            metric_namespace: self.metric_namespace,
            metric_timestamp: core::default::Default::default(),
            metric_unit: self.metric_unit,
            metric_value: self.metric_value,
            role_arn: self.role_arn,
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_CloudwatchMetricRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_CloudwatchMetricRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_CloudwatchMetricRef {
        IotTopicRule_CloudwatchMetricRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_CloudwatchMetricRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_namespace` after provisioning.\n"]
    pub fn metric_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_timestamp` after provisioning.\n"]
    pub fn metric_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_timestamp", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_unit` after provisioning.\n"]
    pub fn metric_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_unit", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_value` after provisioning.\n"]
    pub fn metric_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_value", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRule_Dynamodb {
    hash_key_field: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hash_key_type: std::option::Option<terrars::PrimField<std::string::String>>,
    hash_key_value: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payload_field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range_key_field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range_key_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range_key_value: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    table_name: terrars::PrimField<std::string::String>,
}

impl IotTopicRule_Dynamodb {
    #[doc = "Set the field `hash_key_type`.\n"]
    pub fn set_hash_key_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hash_key_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operation`.\n"]
    pub fn set_operation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payload_field`.\n"]
    pub fn set_payload_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.payload_field = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `range_key_field`.\n"]
    pub fn set_range_key_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.range_key_field = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `range_key_type`.\n"]
    pub fn set_range_key_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.range_key_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `range_key_value`.\n"]
    pub fn set_range_key_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.range_key_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRule_Dynamodb {
    type O = terrars::BlockAssignable<IotTopicRule_Dynamodb>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_Dynamodb {
    #[doc = ""]
    pub hash_key_field: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub hash_key_value: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_Dynamodb {
    pub fn build(self) -> IotTopicRule_Dynamodb {
        IotTopicRule_Dynamodb {
            hash_key_field: self.hash_key_field,
            hash_key_type: core::default::Default::default(),
            hash_key_value: self.hash_key_value,
            operation: core::default::Default::default(),
            payload_field: core::default::Default::default(),
            range_key_field: core::default::Default::default(),
            range_key_type: core::default::Default::default(),
            range_key_value: core::default::Default::default(),
            role_arn: self.role_arn,
            table_name: self.table_name,
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_DynamodbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_DynamodbRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_DynamodbRef {
        IotTopicRule_DynamodbRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_DynamodbRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hash_key_field` after provisioning.\n"]
    pub fn hash_key_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hash_key_field", self.base))
    }

    #[doc = "Get a reference to the value of field `hash_key_type` after provisioning.\n"]
    pub fn hash_key_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hash_key_type", self.base))
    }

    #[doc = "Get a reference to the value of field `hash_key_value` after provisioning.\n"]
    pub fn hash_key_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hash_key_value", self.base))
    }

    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc = "Get a reference to the value of field `payload_field` after provisioning.\n"]
    pub fn payload_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload_field", self.base))
    }

    #[doc = "Get a reference to the value of field `range_key_field` after provisioning.\n"]
    pub fn range_key_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.range_key_field", self.base))
    }

    #[doc = "Get a reference to the value of field `range_key_type` after provisioning.\n"]
    pub fn range_key_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.range_key_type", self.base))
    }

    #[doc = "Get a reference to the value of field `range_key_value` after provisioning.\n"]
    pub fn range_key_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.range_key_value", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRule_Dynamodbv2_PutItem {
    table_name: terrars::PrimField<std::string::String>,
}

impl IotTopicRule_Dynamodbv2_PutItem { }

impl terrars::ToListMappable for IotTopicRule_Dynamodbv2_PutItem {
    type O = terrars::BlockAssignable<IotTopicRule_Dynamodbv2_PutItem>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_Dynamodbv2_PutItem {
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_Dynamodbv2_PutItem {
    pub fn build(self) -> IotTopicRule_Dynamodbv2_PutItem {
        IotTopicRule_Dynamodbv2_PutItem { table_name: self.table_name }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_Dynamodbv2_PutItemRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_Dynamodbv2_PutItemRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_Dynamodbv2_PutItemRef {
        IotTopicRule_Dynamodbv2_PutItemRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_Dynamodbv2_PutItemRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct IotTopicRule_Dynamodbv2Dynamic {
    put_item: std::option::Option<terrars::DynamicBlock<IotTopicRule_Dynamodbv2_PutItem>>,
}

#[derive(Serialize)]
pub struct IotTopicRule_Dynamodbv2 {
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    put_item: std::option::Option<std::vec::Vec<IotTopicRule_Dynamodbv2_PutItem>>,
    dynamic: IotTopicRule_Dynamodbv2Dynamic,
}

impl IotTopicRule_Dynamodbv2 {
    #[doc = "Set the field `put_item`.\n"]
    pub fn set_put_item(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Dynamodbv2_PutItem>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.put_item = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.put_item = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IotTopicRule_Dynamodbv2 {
    type O = terrars::BlockAssignable<IotTopicRule_Dynamodbv2>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_Dynamodbv2 {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_Dynamodbv2 {
    pub fn build(self) -> IotTopicRule_Dynamodbv2 {
        IotTopicRule_Dynamodbv2 {
            role_arn: self.role_arn,
            put_item: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_Dynamodbv2Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_Dynamodbv2Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_Dynamodbv2Ref {
        IotTopicRule_Dynamodbv2Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_Dynamodbv2Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `put_item` after provisioning.\n"]
    pub fn put_item(&self) -> terrars::ListRef<IotTopicRule_Dynamodbv2_PutItemRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.put_item", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRule_Elasticsearch {
    endpoint: terrars::PrimField<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    index: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl IotTopicRule_Elasticsearch { }

impl terrars::ToListMappable for IotTopicRule_Elasticsearch {
    type O = terrars::BlockAssignable<IotTopicRule_Elasticsearch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_Elasticsearch {
    #[doc = ""]
    pub endpoint: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub index: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_Elasticsearch {
    pub fn build(self) -> IotTopicRule_Elasticsearch {
        IotTopicRule_Elasticsearch {
            endpoint: self.endpoint,
            id: self.id,
            index: self.index,
            role_arn: self.role_arn,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_ElasticsearchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_ElasticsearchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_ElasticsearchRef {
        IotTopicRule_ElasticsearchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_ElasticsearchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `index` after provisioning.\n"]
    pub fn index(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRule_Firehose {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_mode: std::option::Option<terrars::PrimField<bool>>,
    delivery_stream_name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    separator: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IotTopicRule_Firehose {
    #[doc = "Set the field `batch_mode`.\n"]
    pub fn set_batch_mode(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.batch_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `separator`.\n"]
    pub fn set_separator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.separator = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRule_Firehose {
    type O = terrars::BlockAssignable<IotTopicRule_Firehose>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_Firehose {
    #[doc = ""]
    pub delivery_stream_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_Firehose {
    pub fn build(self) -> IotTopicRule_Firehose {
        IotTopicRule_Firehose {
            batch_mode: core::default::Default::default(),
            delivery_stream_name: self.delivery_stream_name,
            role_arn: self.role_arn,
            separator: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_FirehoseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_FirehoseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_FirehoseRef {
        IotTopicRule_FirehoseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_FirehoseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `batch_mode` after provisioning.\n"]
    pub fn batch_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `delivery_stream_name` after provisioning.\n"]
    pub fn delivery_stream_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delivery_stream_name", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `separator` after provisioning.\n"]
    pub fn separator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.separator", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRule_Http_HttpHeader {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl IotTopicRule_Http_HttpHeader { }

impl terrars::ToListMappable for IotTopicRule_Http_HttpHeader {
    type O = terrars::BlockAssignable<IotTopicRule_Http_HttpHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_Http_HttpHeader {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_Http_HttpHeader {
    pub fn build(self) -> IotTopicRule_Http_HttpHeader {
        IotTopicRule_Http_HttpHeader {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_Http_HttpHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_Http_HttpHeaderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_Http_HttpHeaderRef {
        IotTopicRule_Http_HttpHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_Http_HttpHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct IotTopicRule_HttpDynamic {
    http_header: std::option::Option<terrars::DynamicBlock<IotTopicRule_Http_HttpHeader>>,
}

#[derive(Serialize)]
pub struct IotTopicRule_Http {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confirmation_url: std::option::Option<terrars::PrimField<std::string::String>>,
    url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_header: std::option::Option<std::vec::Vec<IotTopicRule_Http_HttpHeader>>,
    dynamic: IotTopicRule_HttpDynamic,
}

impl IotTopicRule_Http {
    #[doc = "Set the field `confirmation_url`.\n"]
    pub fn set_confirmation_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.confirmation_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_header`.\n"]
    pub fn set_http_header(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Http_HttpHeader>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_header = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IotTopicRule_Http {
    type O = terrars::BlockAssignable<IotTopicRule_Http>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_Http {
    #[doc = ""]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_Http {
    pub fn build(self) -> IotTopicRule_Http {
        IotTopicRule_Http {
            confirmation_url: core::default::Default::default(),
            url: self.url,
            http_header: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_HttpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_HttpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_HttpRef {
        IotTopicRule_HttpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_HttpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `confirmation_url` after provisioning.\n"]
    pub fn confirmation_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.confirmation_url", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `http_header` after provisioning.\n"]
    pub fn http_header(&self) -> terrars::ListRef<IotTopicRule_Http_HttpHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_header", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRule_IotAnalytics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_mode: std::option::Option<terrars::PrimField<bool>>,
    channel_name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl IotTopicRule_IotAnalytics {
    #[doc = "Set the field `batch_mode`.\n"]
    pub fn set_batch_mode(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.batch_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRule_IotAnalytics {
    type O = terrars::BlockAssignable<IotTopicRule_IotAnalytics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_IotAnalytics {
    #[doc = ""]
    pub channel_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_IotAnalytics {
    pub fn build(self) -> IotTopicRule_IotAnalytics {
        IotTopicRule_IotAnalytics {
            batch_mode: core::default::Default::default(),
            channel_name: self.channel_name,
            role_arn: self.role_arn,
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_IotAnalyticsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_IotAnalyticsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_IotAnalyticsRef {
        IotTopicRule_IotAnalyticsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_IotAnalyticsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `batch_mode` after provisioning.\n"]
    pub fn batch_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `channel_name` after provisioning.\n"]
    pub fn channel_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel_name", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRule_IotEvents {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_mode: std::option::Option<terrars::PrimField<bool>>,
    input_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_id: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl IotTopicRule_IotEvents {
    #[doc = "Set the field `batch_mode`.\n"]
    pub fn set_batch_mode(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.batch_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message_id`.\n"]
    pub fn set_message_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRule_IotEvents {
    type O = terrars::BlockAssignable<IotTopicRule_IotEvents>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_IotEvents {
    #[doc = ""]
    pub input_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_IotEvents {
    pub fn build(self) -> IotTopicRule_IotEvents {
        IotTopicRule_IotEvents {
            batch_mode: core::default::Default::default(),
            input_name: self.input_name,
            message_id: core::default::Default::default(),
            role_arn: self.role_arn,
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_IotEventsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_IotEventsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_IotEventsRef {
        IotTopicRule_IotEventsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_IotEventsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `batch_mode` after provisioning.\n"]
    pub fn batch_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `input_name` after provisioning.\n"]
    pub fn input_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_name", self.base))
    }

    #[doc = "Get a reference to the value of field `message_id` after provisioning.\n"]
    pub fn message_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_id", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRule_Kafka_Header {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl IotTopicRule_Kafka_Header { }

impl terrars::ToListMappable for IotTopicRule_Kafka_Header {
    type O = terrars::BlockAssignable<IotTopicRule_Kafka_Header>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_Kafka_Header {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_Kafka_Header {
    pub fn build(self) -> IotTopicRule_Kafka_Header {
        IotTopicRule_Kafka_Header {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_Kafka_HeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_Kafka_HeaderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_Kafka_HeaderRef {
        IotTopicRule_Kafka_HeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_Kafka_HeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct IotTopicRule_KafkaDynamic {
    header: std::option::Option<terrars::DynamicBlock<IotTopicRule_Kafka_Header>>,
}

#[derive(Serialize)]
pub struct IotTopicRule_Kafka {
    client_properties: terrars::RecField<terrars::PrimField<std::string::String>>,
    destination_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition: std::option::Option<terrars::PrimField<std::string::String>>,
    topic: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<std::vec::Vec<IotTopicRule_Kafka_Header>>,
    dynamic: IotTopicRule_KafkaDynamic,
}

impl IotTopicRule_Kafka {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `partition`.\n"]
    pub fn set_partition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.partition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Kafka_Header>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IotTopicRule_Kafka {
    type O = terrars::BlockAssignable<IotTopicRule_Kafka>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_Kafka {
    #[doc = ""]
    pub client_properties: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub destination_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub topic: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_Kafka {
    pub fn build(self) -> IotTopicRule_Kafka {
        IotTopicRule_Kafka {
            client_properties: self.client_properties,
            destination_arn: self.destination_arn,
            key: core::default::Default::default(),
            partition: core::default::Default::default(),
            topic: self.topic,
            header: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_KafkaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_KafkaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_KafkaRef {
        IotTopicRule_KafkaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_KafkaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_properties` after provisioning.\n"]
    pub fn client_properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.client_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_arn` after provisioning.\n"]
    pub fn destination_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `partition` after provisioning.\n"]
    pub fn partition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partition", self.base))
    }

    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::ListRef<IotTopicRule_Kafka_HeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRule_Kinesis {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_key: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    stream_name: terrars::PrimField<std::string::String>,
}

impl IotTopicRule_Kinesis {
    #[doc = "Set the field `partition_key`.\n"]
    pub fn set_partition_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.partition_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRule_Kinesis {
    type O = terrars::BlockAssignable<IotTopicRule_Kinesis>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_Kinesis {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub stream_name: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_Kinesis {
    pub fn build(self) -> IotTopicRule_Kinesis {
        IotTopicRule_Kinesis {
            partition_key: core::default::Default::default(),
            role_arn: self.role_arn,
            stream_name: self.stream_name,
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_KinesisRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_KinesisRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_KinesisRef {
        IotTopicRule_KinesisRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_KinesisRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `partition_key` after provisioning.\n"]
    pub fn partition_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partition_key", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `stream_name` after provisioning.\n"]
    pub fn stream_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_name", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRule_Lambda {
    function_arn: terrars::PrimField<std::string::String>,
}

impl IotTopicRule_Lambda { }

impl terrars::ToListMappable for IotTopicRule_Lambda {
    type O = terrars::BlockAssignable<IotTopicRule_Lambda>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_Lambda {
    #[doc = ""]
    pub function_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_Lambda {
    pub fn build(self) -> IotTopicRule_Lambda {
        IotTopicRule_Lambda { function_arn: self.function_arn }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_LambdaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_LambdaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_LambdaRef {
        IotTopicRule_LambdaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_LambdaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRule_Republish {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qos: std::option::Option<terrars::PrimField<f64>>,
    role_arn: terrars::PrimField<std::string::String>,
    topic: terrars::PrimField<std::string::String>,
}

impl IotTopicRule_Republish {
    #[doc = "Set the field `qos`.\n"]
    pub fn set_qos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.qos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRule_Republish {
    type O = terrars::BlockAssignable<IotTopicRule_Republish>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_Republish {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub topic: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_Republish {
    pub fn build(self) -> IotTopicRule_Republish {
        IotTopicRule_Republish {
            qos: core::default::Default::default(),
            role_arn: self.role_arn,
            topic: self.topic,
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_RepublishRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_RepublishRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_RepublishRef {
        IotTopicRule_RepublishRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_RepublishRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `qos` after provisioning.\n"]
    pub fn qos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qos", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRule_S3 {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canned_acl: std::option::Option<terrars::PrimField<std::string::String>>,
    key: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl IotTopicRule_S3 {
    #[doc = "Set the field `canned_acl`.\n"]
    pub fn set_canned_acl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.canned_acl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRule_S3 {
    type O = terrars::BlockAssignable<IotTopicRule_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_S3 {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_S3 {
    pub fn build(self) -> IotTopicRule_S3 {
        IotTopicRule_S3 {
            bucket_name: self.bucket_name,
            canned_acl: core::default::Default::default(),
            key: self.key,
            role_arn: self.role_arn,
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_S3Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_S3Ref {
        IotTopicRule_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_S3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `canned_acl` after provisioning.\n"]
    pub fn canned_acl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.canned_acl", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRule_Sns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_format: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    target_arn: terrars::PrimField<std::string::String>,
}

impl IotTopicRule_Sns {
    #[doc = "Set the field `message_format`.\n"]
    pub fn set_message_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message_format = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRule_Sns {
    type O = terrars::BlockAssignable<IotTopicRule_Sns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_Sns {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_Sns {
    pub fn build(self) -> IotTopicRule_Sns {
        IotTopicRule_Sns {
            message_format: core::default::Default::default(),
            role_arn: self.role_arn,
            target_arn: self.target_arn,
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_SnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_SnsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_SnsRef {
        IotTopicRule_SnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_SnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message_format` after provisioning.\n"]
    pub fn message_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_format", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `target_arn` after provisioning.\n"]
    pub fn target_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRule_Sqs {
    queue_url: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    use_base64: terrars::PrimField<bool>,
}

impl IotTopicRule_Sqs { }

impl terrars::ToListMappable for IotTopicRule_Sqs {
    type O = terrars::BlockAssignable<IotTopicRule_Sqs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_Sqs {
    #[doc = ""]
    pub queue_url: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub use_base64: terrars::PrimField<bool>,
}

impl BuildIotTopicRule_Sqs {
    pub fn build(self) -> IotTopicRule_Sqs {
        IotTopicRule_Sqs {
            queue_url: self.queue_url,
            role_arn: self.role_arn,
            use_base64: self.use_base64,
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_SqsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_SqsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_SqsRef {
        IotTopicRule_SqsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_SqsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `queue_url` after provisioning.\n"]
    pub fn queue_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_url", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `use_base64` after provisioning.\n"]
    pub fn use_base64(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_base64", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRule_StepFunctions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    state_machine_name: terrars::PrimField<std::string::String>,
}

impl IotTopicRule_StepFunctions {
    #[doc = "Set the field `execution_name_prefix`.\n"]
    pub fn set_execution_name_prefix(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.execution_name_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRule_StepFunctions {
    type O = terrars::BlockAssignable<IotTopicRule_StepFunctions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_StepFunctions {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub state_machine_name: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_StepFunctions {
    pub fn build(self) -> IotTopicRule_StepFunctions {
        IotTopicRule_StepFunctions {
            execution_name_prefix: core::default::Default::default(),
            role_arn: self.role_arn,
            state_machine_name: self.state_machine_name,
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_StepFunctionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_StepFunctionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_StepFunctionsRef {
        IotTopicRule_StepFunctionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_StepFunctionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execution_name_prefix` after provisioning.\n"]
    pub fn execution_name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_name_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `state_machine_name` after provisioning.\n"]
    pub fn state_machine_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_machine_name", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRule_Timestream_Dimension {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl IotTopicRule_Timestream_Dimension { }

impl terrars::ToListMappable for IotTopicRule_Timestream_Dimension {
    type O = terrars::BlockAssignable<IotTopicRule_Timestream_Dimension>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_Timestream_Dimension {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_Timestream_Dimension {
    pub fn build(self) -> IotTopicRule_Timestream_Dimension {
        IotTopicRule_Timestream_Dimension {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_Timestream_DimensionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_Timestream_DimensionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_Timestream_DimensionRef {
        IotTopicRule_Timestream_DimensionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_Timestream_DimensionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRule_Timestream_Timestamp {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl IotTopicRule_Timestream_Timestamp { }

impl terrars::ToListMappable for IotTopicRule_Timestream_Timestamp {
    type O = terrars::BlockAssignable<IotTopicRule_Timestream_Timestamp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_Timestream_Timestamp {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_Timestream_Timestamp {
    pub fn build(self) -> IotTopicRule_Timestream_Timestamp {
        IotTopicRule_Timestream_Timestamp {
            unit: self.unit,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_Timestream_TimestampRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_Timestream_TimestampRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_Timestream_TimestampRef {
        IotTopicRule_Timestream_TimestampRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_Timestream_TimestampRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct IotTopicRule_TimestreamDynamic {
    dimension: std::option::Option<terrars::DynamicBlock<IotTopicRule_Timestream_Dimension>>,
    timestamp: std::option::Option<terrars::DynamicBlock<IotTopicRule_Timestream_Timestamp>>,
}

#[derive(Serialize)]
pub struct IotTopicRule_Timestream {
    database_name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<IotTopicRule_Timestream_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamp: std::option::Option<std::vec::Vec<IotTopicRule_Timestream_Timestamp>>,
    dynamic: IotTopicRule_TimestreamDynamic,
}

impl IotTopicRule_Timestream {
    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Timestream_Dimension>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dimension = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timestamp`.\n"]
    pub fn set_timestamp(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Timestream_Timestamp>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timestamp = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timestamp = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IotTopicRule_Timestream {
    type O = terrars::BlockAssignable<IotTopicRule_Timestream>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_Timestream {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRule_Timestream {
    pub fn build(self) -> IotTopicRule_Timestream {
        IotTopicRule_Timestream {
            database_name: self.database_name,
            role_arn: self.role_arn,
            table_name: self.table_name,
            dimension: core::default::Default::default(),
            timestamp: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_TimestreamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_TimestreamRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_TimestreamRef {
        IotTopicRule_TimestreamRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_TimestreamRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `timestamp` after provisioning.\n"]
    pub fn timestamp(&self) -> terrars::ListRef<IotTopicRule_Timestream_TimestampRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timestamp", self.base))
    }
}

#[derive(Serialize, Default)]
struct IotTopicRule_ErrorActionDynamic {
    cloudwatch_alarm: std::option::Option<terrars::DynamicBlock<IotTopicRule_CloudwatchAlarm>>,
    cloudwatch_logs: std::option::Option<terrars::DynamicBlock<IotTopicRule_CloudwatchLogs>>,
    cloudwatch_metric: std::option::Option<terrars::DynamicBlock<IotTopicRule_CloudwatchMetric>>,
    dynamodb: std::option::Option<terrars::DynamicBlock<IotTopicRule_Dynamodb>>,
    dynamodbv2: std::option::Option<terrars::DynamicBlock<IotTopicRule_Dynamodbv2>>,
    elasticsearch: std::option::Option<terrars::DynamicBlock<IotTopicRule_Elasticsearch>>,
    firehose: std::option::Option<terrars::DynamicBlock<IotTopicRule_Firehose>>,
    http: std::option::Option<terrars::DynamicBlock<IotTopicRule_Http>>,
    iot_analytics: std::option::Option<terrars::DynamicBlock<IotTopicRule_IotAnalytics>>,
    iot_events: std::option::Option<terrars::DynamicBlock<IotTopicRule_IotEvents>>,
    kafka: std::option::Option<terrars::DynamicBlock<IotTopicRule_Kafka>>,
    kinesis: std::option::Option<terrars::DynamicBlock<IotTopicRule_Kinesis>>,
    lambda: std::option::Option<terrars::DynamicBlock<IotTopicRule_Lambda>>,
    republish: std::option::Option<terrars::DynamicBlock<IotTopicRule_Republish>>,
    s3: std::option::Option<terrars::DynamicBlock<IotTopicRule_S3>>,
    sns: std::option::Option<terrars::DynamicBlock<IotTopicRule_Sns>>,
    sqs: std::option::Option<terrars::DynamicBlock<IotTopicRule_Sqs>>,
    step_functions: std::option::Option<terrars::DynamicBlock<IotTopicRule_StepFunctions>>,
    timestream: std::option::Option<terrars::DynamicBlock<IotTopicRule_Timestream>>,
}

#[derive(Serialize)]
pub struct IotTopicRule_ErrorAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_alarm: std::option::Option<std::vec::Vec<IotTopicRule_CloudwatchAlarm>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs: std::option::Option<std::vec::Vec<IotTopicRule_CloudwatchLogs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_metric: std::option::Option<std::vec::Vec<IotTopicRule_CloudwatchMetric>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynamodb: std::option::Option<std::vec::Vec<IotTopicRule_Dynamodb>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynamodbv2: std::option::Option<std::vec::Vec<IotTopicRule_Dynamodbv2>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elasticsearch: std::option::Option<std::vec::Vec<IotTopicRule_Elasticsearch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firehose: std::option::Option<std::vec::Vec<IotTopicRule_Firehose>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http: std::option::Option<std::vec::Vec<IotTopicRule_Http>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iot_analytics: std::option::Option<std::vec::Vec<IotTopicRule_IotAnalytics>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iot_events: std::option::Option<std::vec::Vec<IotTopicRule_IotEvents>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kafka: std::option::Option<std::vec::Vec<IotTopicRule_Kafka>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis: std::option::Option<std::vec::Vec<IotTopicRule_Kinesis>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda: std::option::Option<std::vec::Vec<IotTopicRule_Lambda>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    republish: std::option::Option<std::vec::Vec<IotTopicRule_Republish>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<IotTopicRule_S3>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sns: std::option::Option<std::vec::Vec<IotTopicRule_Sns>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sqs: std::option::Option<std::vec::Vec<IotTopicRule_Sqs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step_functions: std::option::Option<std::vec::Vec<IotTopicRule_StepFunctions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestream: std::option::Option<std::vec::Vec<IotTopicRule_Timestream>>,
    dynamic: IotTopicRule_ErrorActionDynamic,
}

impl IotTopicRule_ErrorAction {
    #[doc = "Set the field `cloudwatch_alarm`.\n"]
    pub fn set_cloudwatch_alarm(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_CloudwatchAlarm>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_alarm = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_alarm = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_CloudwatchLogs>>,
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

    #[doc = "Set the field `cloudwatch_metric`.\n"]
    pub fn set_cloudwatch_metric(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_CloudwatchMetric>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_metric = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_metric = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dynamodb`.\n"]
    pub fn set_dynamodb(mut self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Dynamodb>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dynamodb = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dynamodb = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dynamodbv2`.\n"]
    pub fn set_dynamodbv2(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Dynamodbv2>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dynamodbv2 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dynamodbv2 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `elasticsearch`.\n"]
    pub fn set_elasticsearch(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Elasticsearch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.elasticsearch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.elasticsearch = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `firehose`.\n"]
    pub fn set_firehose(mut self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Firehose>>) -> Self {
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

    #[doc = "Set the field `http`.\n"]
    pub fn set_http(mut self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Http>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `iot_analytics`.\n"]
    pub fn set_iot_analytics(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_IotAnalytics>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.iot_analytics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.iot_analytics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `iot_events`.\n"]
    pub fn set_iot_events(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_IotEvents>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.iot_events = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.iot_events = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kafka`.\n"]
    pub fn set_kafka(mut self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Kafka>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kafka = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kafka = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis`.\n"]
    pub fn set_kinesis(mut self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Kinesis>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda`.\n"]
    pub fn set_lambda(mut self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Lambda>>) -> Self {
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

    #[doc = "Set the field `republish`.\n"]
    pub fn set_republish(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Republish>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.republish = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.republish = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(mut self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_S3>>) -> Self {
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

    #[doc = "Set the field `sns`.\n"]
    pub fn set_sns(mut self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Sns>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sns = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sqs`.\n"]
    pub fn set_sqs(mut self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Sqs>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sqs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sqs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `step_functions`.\n"]
    pub fn set_step_functions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_StepFunctions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.step_functions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.step_functions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timestream`.\n"]
    pub fn set_timestream(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRule_Timestream>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timestream = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timestream = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IotTopicRule_ErrorAction {
    type O = terrars::BlockAssignable<IotTopicRule_ErrorAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRule_ErrorAction {}

impl BuildIotTopicRule_ErrorAction {
    pub fn build(self) -> IotTopicRule_ErrorAction {
        IotTopicRule_ErrorAction {
            cloudwatch_alarm: core::default::Default::default(),
            cloudwatch_logs: core::default::Default::default(),
            cloudwatch_metric: core::default::Default::default(),
            dynamodb: core::default::Default::default(),
            dynamodbv2: core::default::Default::default(),
            elasticsearch: core::default::Default::default(),
            firehose: core::default::Default::default(),
            http: core::default::Default::default(),
            iot_analytics: core::default::Default::default(),
            iot_events: core::default::Default::default(),
            kafka: core::default::Default::default(),
            kinesis: core::default::Default::default(),
            lambda: core::default::Default::default(),
            republish: core::default::Default::default(),
            s3: core::default::Default::default(),
            sns: core::default::Default::default(),
            sqs: core::default::Default::default(),
            step_functions: core::default::Default::default(),
            timestream: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IotTopicRule_ErrorActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRule_ErrorActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRule_ErrorActionRef {
        IotTopicRule_ErrorActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRule_ErrorActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_alarm` after provisioning.\n"]
    pub fn cloudwatch_alarm(&self) -> terrars::ListRef<IotTopicRule_CloudwatchAlarmRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_alarm", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(&self) -> terrars::ListRef<IotTopicRule_CloudwatchLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_metric` after provisioning.\n"]
    pub fn cloudwatch_metric(&self) -> terrars::ListRef<IotTopicRule_CloudwatchMetricRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_metric", self.base))
    }

    #[doc = "Get a reference to the value of field `dynamodb` after provisioning.\n"]
    pub fn dynamodb(&self) -> terrars::ListRef<IotTopicRule_DynamodbRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynamodb", self.base))
    }

    #[doc = "Get a reference to the value of field `dynamodbv2` after provisioning.\n"]
    pub fn dynamodbv2(&self) -> terrars::ListRef<IotTopicRule_Dynamodbv2Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynamodbv2", self.base))
    }

    #[doc = "Get a reference to the value of field `elasticsearch` after provisioning.\n"]
    pub fn elasticsearch(&self) -> terrars::ListRef<IotTopicRule_ElasticsearchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elasticsearch", self.base))
    }

    #[doc = "Get a reference to the value of field `firehose` after provisioning.\n"]
    pub fn firehose(&self) -> terrars::ListRef<IotTopicRule_FirehoseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firehose", self.base))
    }

    #[doc = "Get a reference to the value of field `http` after provisioning.\n"]
    pub fn http(&self) -> terrars::ListRef<IotTopicRule_HttpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http", self.base))
    }

    #[doc = "Get a reference to the value of field `iot_analytics` after provisioning.\n"]
    pub fn iot_analytics(&self) -> terrars::ListRef<IotTopicRule_IotAnalyticsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iot_analytics", self.base))
    }

    #[doc = "Get a reference to the value of field `iot_events` after provisioning.\n"]
    pub fn iot_events(&self) -> terrars::ListRef<IotTopicRule_IotEventsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iot_events", self.base))
    }

    #[doc = "Get a reference to the value of field `kafka` after provisioning.\n"]
    pub fn kafka(&self) -> terrars::ListRef<IotTopicRule_KafkaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kafka", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis` after provisioning.\n"]
    pub fn kinesis(&self) -> terrars::ListRef<IotTopicRule_KinesisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda` after provisioning.\n"]
    pub fn lambda(&self) -> terrars::ListRef<IotTopicRule_LambdaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda", self.base))
    }

    #[doc = "Get a reference to the value of field `republish` after provisioning.\n"]
    pub fn republish(&self) -> terrars::ListRef<IotTopicRule_RepublishRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.republish", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<IotTopicRule_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }

    #[doc = "Get a reference to the value of field `sns` after provisioning.\n"]
    pub fn sns(&self) -> terrars::ListRef<IotTopicRule_SnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sns", self.base))
    }

    #[doc = "Get a reference to the value of field `sqs` after provisioning.\n"]
    pub fn sqs(&self) -> terrars::ListRef<IotTopicRule_SqsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sqs", self.base))
    }

    #[doc = "Get a reference to the value of field `step_functions` after provisioning.\n"]
    pub fn step_functions(&self) -> terrars::ListRef<IotTopicRule_StepFunctionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.step_functions", self.base))
    }

    #[doc = "Get a reference to the value of field `timestream` after provisioning.\n"]
    pub fn timestream(&self) -> terrars::ListRef<IotTopicRule_TimestreamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timestream", self.base))
    }
}
