use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct IotTopicRuleData {
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
    sql: terrars::PrimField<std::string::String>,
    sql_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_alarm: std::option::Option<std::vec::Vec<IotTopicRuleCloudwatchAlarmEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs: std::option::Option<std::vec::Vec<IotTopicRuleCloudwatchLogsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_metric: std::option::Option<std::vec::Vec<IotTopicRuleCloudwatchMetricEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynamodb: std::option::Option<std::vec::Vec<IotTopicRuleDynamodbEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynamodbv2: std::option::Option<std::vec::Vec<IotTopicRuleDynamodbv2El>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elasticsearch: std::option::Option<std::vec::Vec<IotTopicRuleElasticsearchEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_action: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firehose: std::option::Option<std::vec::Vec<IotTopicRuleFirehoseEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http: std::option::Option<std::vec::Vec<IotTopicRuleHttpEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iot_analytics: std::option::Option<std::vec::Vec<IotTopicRuleIotAnalyticsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iot_events: std::option::Option<std::vec::Vec<IotTopicRuleIotEventsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kafka: std::option::Option<std::vec::Vec<IotTopicRuleKafkaEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis: std::option::Option<std::vec::Vec<IotTopicRuleKinesisEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda: std::option::Option<std::vec::Vec<IotTopicRuleLambdaEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    republish: std::option::Option<std::vec::Vec<IotTopicRuleRepublishEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<IotTopicRuleS3El>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sns: std::option::Option<std::vec::Vec<IotTopicRuleSnsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sqs: std::option::Option<std::vec::Vec<IotTopicRuleSqsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step_functions: std::option::Option<std::vec::Vec<IotTopicRuleStepFunctionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestream: std::option::Option<std::vec::Vec<IotTopicRuleTimestreamEl>>,
    dynamic: IotTopicRuleDynamic,
}

struct IotTopicRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IotTopicRuleData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleCloudwatchAlarmEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleCloudwatchLogsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleCloudwatchMetricEl>>,
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
    pub fn set_dynamodb(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleDynamodbEl>>) -> Self {
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
    pub fn set_dynamodbv2(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleDynamodbv2El>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleElasticsearchEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionEl>>,
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
    pub fn set_firehose(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleFirehoseEl>>) -> Self {
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
    pub fn set_http(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleHttpEl>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleIotAnalyticsEl>>,
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
    pub fn set_iot_events(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleIotEventsEl>>) -> Self {
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
    pub fn set_kafka(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleKafkaEl>>) -> Self {
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
    pub fn set_kinesis(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleKinesisEl>>) -> Self {
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
    pub fn set_lambda(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleLambdaEl>>) -> Self {
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
    pub fn set_republish(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleRepublishEl>>) -> Self {
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
    pub fn set_s3(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleS3El>>) -> Self {
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
    pub fn set_sns(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleSnsEl>>) -> Self {
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
    pub fn set_sqs(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleSqsEl>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleStepFunctionsEl>>,
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
    pub fn set_timestream(self, v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleTimestreamEl>>) -> Self {
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
    pub fn error_action(&self) -> terrars::ListRef<IotTopicRuleErrorActionElRef> {
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
            data: std::cell::RefCell::new(IotTopicRuleData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                enabled: self.enabled,
                id: core::default::Default::default(),
                name: self.name,
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
    pub fn error_action(&self) -> terrars::ListRef<IotTopicRuleErrorActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_action", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct IotTopicRuleCloudwatchAlarmEl {
    alarm_name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    state_reason: terrars::PrimField<std::string::String>,
    state_value: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleCloudwatchAlarmEl { }

impl terrars::ToListMappable for IotTopicRuleCloudwatchAlarmEl {
    type O = terrars::BlockAssignable<IotTopicRuleCloudwatchAlarmEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleCloudwatchAlarmEl {
    #[doc = ""]
    pub alarm_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub state_reason: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub state_value: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleCloudwatchAlarmEl {
    pub fn build(self) -> IotTopicRuleCloudwatchAlarmEl {
        IotTopicRuleCloudwatchAlarmEl {
            alarm_name: self.alarm_name,
            role_arn: self.role_arn,
            state_reason: self.state_reason,
            state_value: self.state_value,
        }
    }
}

pub struct IotTopicRuleCloudwatchAlarmElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleCloudwatchAlarmElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleCloudwatchAlarmElRef {
        IotTopicRuleCloudwatchAlarmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleCloudwatchAlarmElRef {
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
pub struct IotTopicRuleCloudwatchLogsEl {
    log_group_name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleCloudwatchLogsEl { }

impl terrars::ToListMappable for IotTopicRuleCloudwatchLogsEl {
    type O = terrars::BlockAssignable<IotTopicRuleCloudwatchLogsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleCloudwatchLogsEl {
    #[doc = ""]
    pub log_group_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleCloudwatchLogsEl {
    pub fn build(self) -> IotTopicRuleCloudwatchLogsEl {
        IotTopicRuleCloudwatchLogsEl {
            log_group_name: self.log_group_name,
            role_arn: self.role_arn,
        }
    }
}

pub struct IotTopicRuleCloudwatchLogsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleCloudwatchLogsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleCloudwatchLogsElRef {
        IotTopicRuleCloudwatchLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleCloudwatchLogsElRef {
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
}

#[derive(Serialize)]
pub struct IotTopicRuleCloudwatchMetricEl {
    metric_name: terrars::PrimField<std::string::String>,
    metric_namespace: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_timestamp: std::option::Option<terrars::PrimField<std::string::String>>,
    metric_unit: terrars::PrimField<std::string::String>,
    metric_value: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleCloudwatchMetricEl {
    #[doc = "Set the field `metric_timestamp`.\n"]
    pub fn set_metric_timestamp(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metric_timestamp = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRuleCloudwatchMetricEl {
    type O = terrars::BlockAssignable<IotTopicRuleCloudwatchMetricEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleCloudwatchMetricEl {
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

impl BuildIotTopicRuleCloudwatchMetricEl {
    pub fn build(self) -> IotTopicRuleCloudwatchMetricEl {
        IotTopicRuleCloudwatchMetricEl {
            metric_name: self.metric_name,
            metric_namespace: self.metric_namespace,
            metric_timestamp: core::default::Default::default(),
            metric_unit: self.metric_unit,
            metric_value: self.metric_value,
            role_arn: self.role_arn,
        }
    }
}

pub struct IotTopicRuleCloudwatchMetricElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleCloudwatchMetricElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleCloudwatchMetricElRef {
        IotTopicRuleCloudwatchMetricElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleCloudwatchMetricElRef {
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
pub struct IotTopicRuleDynamodbEl {
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

impl IotTopicRuleDynamodbEl {
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

impl terrars::ToListMappable for IotTopicRuleDynamodbEl {
    type O = terrars::BlockAssignable<IotTopicRuleDynamodbEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleDynamodbEl {
    #[doc = ""]
    pub hash_key_field: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub hash_key_value: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleDynamodbEl {
    pub fn build(self) -> IotTopicRuleDynamodbEl {
        IotTopicRuleDynamodbEl {
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

pub struct IotTopicRuleDynamodbElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleDynamodbElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleDynamodbElRef {
        IotTopicRuleDynamodbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleDynamodbElRef {
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
pub struct IotTopicRuleDynamodbv2ElPutItemEl {
    table_name: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleDynamodbv2ElPutItemEl { }

impl terrars::ToListMappable for IotTopicRuleDynamodbv2ElPutItemEl {
    type O = terrars::BlockAssignable<IotTopicRuleDynamodbv2ElPutItemEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleDynamodbv2ElPutItemEl {
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleDynamodbv2ElPutItemEl {
    pub fn build(self) -> IotTopicRuleDynamodbv2ElPutItemEl {
        IotTopicRuleDynamodbv2ElPutItemEl { table_name: self.table_name }
    }
}

pub struct IotTopicRuleDynamodbv2ElPutItemElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleDynamodbv2ElPutItemElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleDynamodbv2ElPutItemElRef {
        IotTopicRuleDynamodbv2ElPutItemElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleDynamodbv2ElPutItemElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct IotTopicRuleDynamodbv2ElDynamic {
    put_item: std::option::Option<terrars::DynamicBlock<IotTopicRuleDynamodbv2ElPutItemEl>>,
}

#[derive(Serialize)]
pub struct IotTopicRuleDynamodbv2El {
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    put_item: std::option::Option<std::vec::Vec<IotTopicRuleDynamodbv2ElPutItemEl>>,
    dynamic: IotTopicRuleDynamodbv2ElDynamic,
}

impl IotTopicRuleDynamodbv2El {
    #[doc = "Set the field `put_item`.\n"]
    pub fn set_put_item(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleDynamodbv2ElPutItemEl>>,
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

impl terrars::ToListMappable for IotTopicRuleDynamodbv2El {
    type O = terrars::BlockAssignable<IotTopicRuleDynamodbv2El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleDynamodbv2El {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleDynamodbv2El {
    pub fn build(self) -> IotTopicRuleDynamodbv2El {
        IotTopicRuleDynamodbv2El {
            role_arn: self.role_arn,
            put_item: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IotTopicRuleDynamodbv2ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleDynamodbv2ElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleDynamodbv2ElRef {
        IotTopicRuleDynamodbv2ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleDynamodbv2ElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `put_item` after provisioning.\n"]
    pub fn put_item(&self) -> terrars::ListRef<IotTopicRuleDynamodbv2ElPutItemElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.put_item", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRuleElasticsearchEl {
    endpoint: terrars::PrimField<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    index: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleElasticsearchEl { }

impl terrars::ToListMappable for IotTopicRuleElasticsearchEl {
    type O = terrars::BlockAssignable<IotTopicRuleElasticsearchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleElasticsearchEl {
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

impl BuildIotTopicRuleElasticsearchEl {
    pub fn build(self) -> IotTopicRuleElasticsearchEl {
        IotTopicRuleElasticsearchEl {
            endpoint: self.endpoint,
            id: self.id,
            index: self.index,
            role_arn: self.role_arn,
            type_: self.type_,
        }
    }
}

pub struct IotTopicRuleElasticsearchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleElasticsearchElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleElasticsearchElRef {
        IotTopicRuleElasticsearchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleElasticsearchElRef {
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
pub struct IotTopicRuleErrorActionElCloudwatchAlarmEl {
    alarm_name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    state_reason: terrars::PrimField<std::string::String>,
    state_value: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleErrorActionElCloudwatchAlarmEl { }

impl terrars::ToListMappable for IotTopicRuleErrorActionElCloudwatchAlarmEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElCloudwatchAlarmEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElCloudwatchAlarmEl {
    #[doc = ""]
    pub alarm_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub state_reason: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub state_value: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleErrorActionElCloudwatchAlarmEl {
    pub fn build(self) -> IotTopicRuleErrorActionElCloudwatchAlarmEl {
        IotTopicRuleErrorActionElCloudwatchAlarmEl {
            alarm_name: self.alarm_name,
            role_arn: self.role_arn,
            state_reason: self.state_reason,
            state_value: self.state_value,
        }
    }
}

pub struct IotTopicRuleErrorActionElCloudwatchAlarmElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElCloudwatchAlarmElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleErrorActionElCloudwatchAlarmElRef {
        IotTopicRuleErrorActionElCloudwatchAlarmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElCloudwatchAlarmElRef {
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
pub struct IotTopicRuleErrorActionElCloudwatchLogsEl {
    log_group_name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleErrorActionElCloudwatchLogsEl { }

impl terrars::ToListMappable for IotTopicRuleErrorActionElCloudwatchLogsEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElCloudwatchLogsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElCloudwatchLogsEl {
    #[doc = ""]
    pub log_group_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleErrorActionElCloudwatchLogsEl {
    pub fn build(self) -> IotTopicRuleErrorActionElCloudwatchLogsEl {
        IotTopicRuleErrorActionElCloudwatchLogsEl {
            log_group_name: self.log_group_name,
            role_arn: self.role_arn,
        }
    }
}

pub struct IotTopicRuleErrorActionElCloudwatchLogsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElCloudwatchLogsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleErrorActionElCloudwatchLogsElRef {
        IotTopicRuleErrorActionElCloudwatchLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElCloudwatchLogsElRef {
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
}

#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElCloudwatchMetricEl {
    metric_name: terrars::PrimField<std::string::String>,
    metric_namespace: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_timestamp: std::option::Option<terrars::PrimField<std::string::String>>,
    metric_unit: terrars::PrimField<std::string::String>,
    metric_value: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleErrorActionElCloudwatchMetricEl {
    #[doc = "Set the field `metric_timestamp`.\n"]
    pub fn set_metric_timestamp(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metric_timestamp = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRuleErrorActionElCloudwatchMetricEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElCloudwatchMetricEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElCloudwatchMetricEl {
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

impl BuildIotTopicRuleErrorActionElCloudwatchMetricEl {
    pub fn build(self) -> IotTopicRuleErrorActionElCloudwatchMetricEl {
        IotTopicRuleErrorActionElCloudwatchMetricEl {
            metric_name: self.metric_name,
            metric_namespace: self.metric_namespace,
            metric_timestamp: core::default::Default::default(),
            metric_unit: self.metric_unit,
            metric_value: self.metric_value,
            role_arn: self.role_arn,
        }
    }
}

pub struct IotTopicRuleErrorActionElCloudwatchMetricElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElCloudwatchMetricElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IotTopicRuleErrorActionElCloudwatchMetricElRef {
        IotTopicRuleErrorActionElCloudwatchMetricElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElCloudwatchMetricElRef {
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
pub struct IotTopicRuleErrorActionElDynamodbEl {
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

impl IotTopicRuleErrorActionElDynamodbEl {
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

impl terrars::ToListMappable for IotTopicRuleErrorActionElDynamodbEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElDynamodbEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElDynamodbEl {
    #[doc = ""]
    pub hash_key_field: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub hash_key_value: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleErrorActionElDynamodbEl {
    pub fn build(self) -> IotTopicRuleErrorActionElDynamodbEl {
        IotTopicRuleErrorActionElDynamodbEl {
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

pub struct IotTopicRuleErrorActionElDynamodbElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElDynamodbElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleErrorActionElDynamodbElRef {
        IotTopicRuleErrorActionElDynamodbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElDynamodbElRef {
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
pub struct IotTopicRuleErrorActionElDynamodbv2ElPutItemEl {
    table_name: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleErrorActionElDynamodbv2ElPutItemEl { }

impl terrars::ToListMappable for IotTopicRuleErrorActionElDynamodbv2ElPutItemEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElDynamodbv2ElPutItemEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElDynamodbv2ElPutItemEl {
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleErrorActionElDynamodbv2ElPutItemEl {
    pub fn build(self) -> IotTopicRuleErrorActionElDynamodbv2ElPutItemEl {
        IotTopicRuleErrorActionElDynamodbv2ElPutItemEl { table_name: self.table_name }
    }
}

pub struct IotTopicRuleErrorActionElDynamodbv2ElPutItemElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElDynamodbv2ElPutItemElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IotTopicRuleErrorActionElDynamodbv2ElPutItemElRef {
        IotTopicRuleErrorActionElDynamodbv2ElPutItemElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElDynamodbv2ElPutItemElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct IotTopicRuleErrorActionElDynamodbv2ElDynamic {
    put_item: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElDynamodbv2ElPutItemEl>>,
}

#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElDynamodbv2El {
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    put_item: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElDynamodbv2ElPutItemEl>>,
    dynamic: IotTopicRuleErrorActionElDynamodbv2ElDynamic,
}

impl IotTopicRuleErrorActionElDynamodbv2El {
    #[doc = "Set the field `put_item`.\n"]
    pub fn set_put_item(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElDynamodbv2ElPutItemEl>>,
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

impl terrars::ToListMappable for IotTopicRuleErrorActionElDynamodbv2El {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElDynamodbv2El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElDynamodbv2El {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleErrorActionElDynamodbv2El {
    pub fn build(self) -> IotTopicRuleErrorActionElDynamodbv2El {
        IotTopicRuleErrorActionElDynamodbv2El {
            role_arn: self.role_arn,
            put_item: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IotTopicRuleErrorActionElDynamodbv2ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElDynamodbv2ElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleErrorActionElDynamodbv2ElRef {
        IotTopicRuleErrorActionElDynamodbv2ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElDynamodbv2ElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `put_item` after provisioning.\n"]
    pub fn put_item(&self) -> terrars::ListRef<IotTopicRuleErrorActionElDynamodbv2ElPutItemElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.put_item", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElElasticsearchEl {
    endpoint: terrars::PrimField<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    index: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleErrorActionElElasticsearchEl { }

impl terrars::ToListMappable for IotTopicRuleErrorActionElElasticsearchEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElElasticsearchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElElasticsearchEl {
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

impl BuildIotTopicRuleErrorActionElElasticsearchEl {
    pub fn build(self) -> IotTopicRuleErrorActionElElasticsearchEl {
        IotTopicRuleErrorActionElElasticsearchEl {
            endpoint: self.endpoint,
            id: self.id,
            index: self.index,
            role_arn: self.role_arn,
            type_: self.type_,
        }
    }
}

pub struct IotTopicRuleErrorActionElElasticsearchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElElasticsearchElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleErrorActionElElasticsearchElRef {
        IotTopicRuleErrorActionElElasticsearchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElElasticsearchElRef {
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
pub struct IotTopicRuleErrorActionElFirehoseEl {
    delivery_stream_name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    separator: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IotTopicRuleErrorActionElFirehoseEl {
    #[doc = "Set the field `separator`.\n"]
    pub fn set_separator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.separator = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRuleErrorActionElFirehoseEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElFirehoseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElFirehoseEl {
    #[doc = ""]
    pub delivery_stream_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleErrorActionElFirehoseEl {
    pub fn build(self) -> IotTopicRuleErrorActionElFirehoseEl {
        IotTopicRuleErrorActionElFirehoseEl {
            delivery_stream_name: self.delivery_stream_name,
            role_arn: self.role_arn,
            separator: core::default::Default::default(),
        }
    }
}

pub struct IotTopicRuleErrorActionElFirehoseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElFirehoseElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleErrorActionElFirehoseElRef {
        IotTopicRuleErrorActionElFirehoseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElFirehoseElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
pub struct IotTopicRuleErrorActionElHttpElHttpHeaderEl {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleErrorActionElHttpElHttpHeaderEl { }

impl terrars::ToListMappable for IotTopicRuleErrorActionElHttpElHttpHeaderEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElHttpElHttpHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElHttpElHttpHeaderEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleErrorActionElHttpElHttpHeaderEl {
    pub fn build(self) -> IotTopicRuleErrorActionElHttpElHttpHeaderEl {
        IotTopicRuleErrorActionElHttpElHttpHeaderEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct IotTopicRuleErrorActionElHttpElHttpHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElHttpElHttpHeaderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IotTopicRuleErrorActionElHttpElHttpHeaderElRef {
        IotTopicRuleErrorActionElHttpElHttpHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElHttpElHttpHeaderElRef {
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
struct IotTopicRuleErrorActionElHttpElDynamic {
    http_header: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElHttpElHttpHeaderEl>>,
}

#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElHttpEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confirmation_url: std::option::Option<terrars::PrimField<std::string::String>>,
    url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_header: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElHttpElHttpHeaderEl>>,
    dynamic: IotTopicRuleErrorActionElHttpElDynamic,
}

impl IotTopicRuleErrorActionElHttpEl {
    #[doc = "Set the field `confirmation_url`.\n"]
    pub fn set_confirmation_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.confirmation_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_header`.\n"]
    pub fn set_http_header(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElHttpElHttpHeaderEl>>,
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

impl terrars::ToListMappable for IotTopicRuleErrorActionElHttpEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElHttpEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElHttpEl {
    #[doc = ""]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleErrorActionElHttpEl {
    pub fn build(self) -> IotTopicRuleErrorActionElHttpEl {
        IotTopicRuleErrorActionElHttpEl {
            confirmation_url: core::default::Default::default(),
            url: self.url,
            http_header: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IotTopicRuleErrorActionElHttpElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElHttpElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleErrorActionElHttpElRef {
        IotTopicRuleErrorActionElHttpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElHttpElRef {
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
    pub fn http_header(&self) -> terrars::ListRef<IotTopicRuleErrorActionElHttpElHttpHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_header", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElIotAnalyticsEl {
    channel_name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleErrorActionElIotAnalyticsEl { }

impl terrars::ToListMappable for IotTopicRuleErrorActionElIotAnalyticsEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElIotAnalyticsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElIotAnalyticsEl {
    #[doc = ""]
    pub channel_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleErrorActionElIotAnalyticsEl {
    pub fn build(self) -> IotTopicRuleErrorActionElIotAnalyticsEl {
        IotTopicRuleErrorActionElIotAnalyticsEl {
            channel_name: self.channel_name,
            role_arn: self.role_arn,
        }
    }
}

pub struct IotTopicRuleErrorActionElIotAnalyticsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElIotAnalyticsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleErrorActionElIotAnalyticsElRef {
        IotTopicRuleErrorActionElIotAnalyticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElIotAnalyticsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
pub struct IotTopicRuleErrorActionElIotEventsEl {
    input_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_id: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleErrorActionElIotEventsEl {
    #[doc = "Set the field `message_id`.\n"]
    pub fn set_message_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRuleErrorActionElIotEventsEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElIotEventsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElIotEventsEl {
    #[doc = ""]
    pub input_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleErrorActionElIotEventsEl {
    pub fn build(self) -> IotTopicRuleErrorActionElIotEventsEl {
        IotTopicRuleErrorActionElIotEventsEl {
            input_name: self.input_name,
            message_id: core::default::Default::default(),
            role_arn: self.role_arn,
        }
    }
}

pub struct IotTopicRuleErrorActionElIotEventsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElIotEventsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleErrorActionElIotEventsElRef {
        IotTopicRuleErrorActionElIotEventsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElIotEventsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
pub struct IotTopicRuleErrorActionElKafkaEl {
    client_properties: terrars::RecField<terrars::PrimField<std::string::String>>,
    destination_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition: std::option::Option<terrars::PrimField<std::string::String>>,
    topic: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleErrorActionElKafkaEl {
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
}

impl terrars::ToListMappable for IotTopicRuleErrorActionElKafkaEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElKafkaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElKafkaEl {
    #[doc = ""]
    pub client_properties: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub destination_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub topic: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleErrorActionElKafkaEl {
    pub fn build(self) -> IotTopicRuleErrorActionElKafkaEl {
        IotTopicRuleErrorActionElKafkaEl {
            client_properties: self.client_properties,
            destination_arn: self.destination_arn,
            key: core::default::Default::default(),
            partition: core::default::Default::default(),
            topic: self.topic,
        }
    }
}

pub struct IotTopicRuleErrorActionElKafkaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElKafkaElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleErrorActionElKafkaElRef {
        IotTopicRuleErrorActionElKafkaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElKafkaElRef {
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
}

#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElKinesisEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_key: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    stream_name: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleErrorActionElKinesisEl {
    #[doc = "Set the field `partition_key`.\n"]
    pub fn set_partition_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.partition_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRuleErrorActionElKinesisEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElKinesisEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElKinesisEl {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub stream_name: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleErrorActionElKinesisEl {
    pub fn build(self) -> IotTopicRuleErrorActionElKinesisEl {
        IotTopicRuleErrorActionElKinesisEl {
            partition_key: core::default::Default::default(),
            role_arn: self.role_arn,
            stream_name: self.stream_name,
        }
    }
}

pub struct IotTopicRuleErrorActionElKinesisElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElKinesisElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleErrorActionElKinesisElRef {
        IotTopicRuleErrorActionElKinesisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElKinesisElRef {
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
pub struct IotTopicRuleErrorActionElLambdaEl {
    function_arn: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleErrorActionElLambdaEl { }

impl terrars::ToListMappable for IotTopicRuleErrorActionElLambdaEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElLambdaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElLambdaEl {
    #[doc = ""]
    pub function_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleErrorActionElLambdaEl {
    pub fn build(self) -> IotTopicRuleErrorActionElLambdaEl {
        IotTopicRuleErrorActionElLambdaEl { function_arn: self.function_arn }
    }
}

pub struct IotTopicRuleErrorActionElLambdaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElLambdaElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleErrorActionElLambdaElRef {
        IotTopicRuleErrorActionElLambdaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElLambdaElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElRepublishEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qos: std::option::Option<terrars::PrimField<f64>>,
    role_arn: terrars::PrimField<std::string::String>,
    topic: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleErrorActionElRepublishEl {
    #[doc = "Set the field `qos`.\n"]
    pub fn set_qos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.qos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRuleErrorActionElRepublishEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElRepublishEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElRepublishEl {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub topic: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleErrorActionElRepublishEl {
    pub fn build(self) -> IotTopicRuleErrorActionElRepublishEl {
        IotTopicRuleErrorActionElRepublishEl {
            qos: core::default::Default::default(),
            role_arn: self.role_arn,
            topic: self.topic,
        }
    }
}

pub struct IotTopicRuleErrorActionElRepublishElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElRepublishElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleErrorActionElRepublishElRef {
        IotTopicRuleErrorActionElRepublishElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElRepublishElRef {
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
pub struct IotTopicRuleErrorActionElS3El {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canned_acl: std::option::Option<terrars::PrimField<std::string::String>>,
    key: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleErrorActionElS3El {
    #[doc = "Set the field `canned_acl`.\n"]
    pub fn set_canned_acl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.canned_acl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRuleErrorActionElS3El {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElS3El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElS3El {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleErrorActionElS3El {
    pub fn build(self) -> IotTopicRuleErrorActionElS3El {
        IotTopicRuleErrorActionElS3El {
            bucket_name: self.bucket_name,
            canned_acl: core::default::Default::default(),
            key: self.key,
            role_arn: self.role_arn,
        }
    }
}

pub struct IotTopicRuleErrorActionElS3ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElS3ElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleErrorActionElS3ElRef {
        IotTopicRuleErrorActionElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElS3ElRef {
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
pub struct IotTopicRuleErrorActionElSnsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_format: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    target_arn: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleErrorActionElSnsEl {
    #[doc = "Set the field `message_format`.\n"]
    pub fn set_message_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message_format = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRuleErrorActionElSnsEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElSnsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElSnsEl {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleErrorActionElSnsEl {
    pub fn build(self) -> IotTopicRuleErrorActionElSnsEl {
        IotTopicRuleErrorActionElSnsEl {
            message_format: core::default::Default::default(),
            role_arn: self.role_arn,
            target_arn: self.target_arn,
        }
    }
}

pub struct IotTopicRuleErrorActionElSnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElSnsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleErrorActionElSnsElRef {
        IotTopicRuleErrorActionElSnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElSnsElRef {
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
pub struct IotTopicRuleErrorActionElSqsEl {
    queue_url: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    use_base64: terrars::PrimField<bool>,
}

impl IotTopicRuleErrorActionElSqsEl { }

impl terrars::ToListMappable for IotTopicRuleErrorActionElSqsEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElSqsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElSqsEl {
    #[doc = ""]
    pub queue_url: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub use_base64: terrars::PrimField<bool>,
}

impl BuildIotTopicRuleErrorActionElSqsEl {
    pub fn build(self) -> IotTopicRuleErrorActionElSqsEl {
        IotTopicRuleErrorActionElSqsEl {
            queue_url: self.queue_url,
            role_arn: self.role_arn,
            use_base64: self.use_base64,
        }
    }
}

pub struct IotTopicRuleErrorActionElSqsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElSqsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleErrorActionElSqsElRef {
        IotTopicRuleErrorActionElSqsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElSqsElRef {
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
pub struct IotTopicRuleErrorActionElStepFunctionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    state_machine_name: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleErrorActionElStepFunctionsEl {
    #[doc = "Set the field `execution_name_prefix`.\n"]
    pub fn set_execution_name_prefix(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.execution_name_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRuleErrorActionElStepFunctionsEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElStepFunctionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElStepFunctionsEl {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub state_machine_name: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleErrorActionElStepFunctionsEl {
    pub fn build(self) -> IotTopicRuleErrorActionElStepFunctionsEl {
        IotTopicRuleErrorActionElStepFunctionsEl {
            execution_name_prefix: core::default::Default::default(),
            role_arn: self.role_arn,
            state_machine_name: self.state_machine_name,
        }
    }
}

pub struct IotTopicRuleErrorActionElStepFunctionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElStepFunctionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleErrorActionElStepFunctionsElRef {
        IotTopicRuleErrorActionElStepFunctionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElStepFunctionsElRef {
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
pub struct IotTopicRuleErrorActionElTimestreamElDimensionEl {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleErrorActionElTimestreamElDimensionEl { }

impl terrars::ToListMappable for IotTopicRuleErrorActionElTimestreamElDimensionEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElTimestreamElDimensionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElTimestreamElDimensionEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleErrorActionElTimestreamElDimensionEl {
    pub fn build(self) -> IotTopicRuleErrorActionElTimestreamElDimensionEl {
        IotTopicRuleErrorActionElTimestreamElDimensionEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct IotTopicRuleErrorActionElTimestreamElDimensionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElTimestreamElDimensionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IotTopicRuleErrorActionElTimestreamElDimensionElRef {
        IotTopicRuleErrorActionElTimestreamElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElTimestreamElDimensionElRef {
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
pub struct IotTopicRuleErrorActionElTimestreamElTimestampEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleErrorActionElTimestreamElTimestampEl { }

impl terrars::ToListMappable for IotTopicRuleErrorActionElTimestreamElTimestampEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElTimestreamElTimestampEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElTimestreamElTimestampEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleErrorActionElTimestreamElTimestampEl {
    pub fn build(self) -> IotTopicRuleErrorActionElTimestreamElTimestampEl {
        IotTopicRuleErrorActionElTimestreamElTimestampEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct IotTopicRuleErrorActionElTimestreamElTimestampElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElTimestreamElTimestampElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IotTopicRuleErrorActionElTimestreamElTimestampElRef {
        IotTopicRuleErrorActionElTimestreamElTimestampElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElTimestreamElTimestampElRef {
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
struct IotTopicRuleErrorActionElTimestreamElDynamic {
    dimension: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElTimestreamElDimensionEl>>,
    timestamp: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElTimestreamElTimestampEl>>,
}

#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElTimestreamEl {
    database_name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElTimestreamElDimensionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamp: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElTimestreamElTimestampEl>>,
    dynamic: IotTopicRuleErrorActionElTimestreamElDynamic,
}

impl IotTopicRuleErrorActionElTimestreamEl {
    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElTimestreamElDimensionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElTimestreamElTimestampEl>>,
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

impl terrars::ToListMappable for IotTopicRuleErrorActionElTimestreamEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionElTimestreamEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionElTimestreamEl {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleErrorActionElTimestreamEl {
    pub fn build(self) -> IotTopicRuleErrorActionElTimestreamEl {
        IotTopicRuleErrorActionElTimestreamEl {
            database_name: self.database_name,
            role_arn: self.role_arn,
            table_name: self.table_name,
            dimension: core::default::Default::default(),
            timestamp: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IotTopicRuleErrorActionElTimestreamElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElTimestreamElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleErrorActionElTimestreamElRef {
        IotTopicRuleErrorActionElTimestreamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElTimestreamElRef {
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
    pub fn timestamp(&self) -> terrars::ListRef<IotTopicRuleErrorActionElTimestreamElTimestampElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timestamp", self.base))
    }
}

#[derive(Serialize, Default)]
struct IotTopicRuleErrorActionElDynamic {
    cloudwatch_alarm: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElCloudwatchAlarmEl>>,
    cloudwatch_logs: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElCloudwatchLogsEl>>,
    cloudwatch_metric: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElCloudwatchMetricEl>>,
    dynamodb: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElDynamodbEl>>,
    dynamodbv2: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElDynamodbv2El>>,
    elasticsearch: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElElasticsearchEl>>,
    firehose: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElFirehoseEl>>,
    http: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElHttpEl>>,
    iot_analytics: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElIotAnalyticsEl>>,
    iot_events: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElIotEventsEl>>,
    kafka: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElKafkaEl>>,
    kinesis: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElKinesisEl>>,
    lambda: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElLambdaEl>>,
    republish: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElRepublishEl>>,
    s3: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElS3El>>,
    sns: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElSnsEl>>,
    sqs: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElSqsEl>>,
    step_functions: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElStepFunctionsEl>>,
    timestream: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionElTimestreamEl>>,
}

#[derive(Serialize)]
pub struct IotTopicRuleErrorActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_alarm: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElCloudwatchAlarmEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElCloudwatchLogsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_metric: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElCloudwatchMetricEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynamodb: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElDynamodbEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynamodbv2: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElDynamodbv2El>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elasticsearch: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElElasticsearchEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firehose: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElFirehoseEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElHttpEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iot_analytics: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElIotAnalyticsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iot_events: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElIotEventsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kafka: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElKafkaEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElKinesisEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElLambdaEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    republish: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElRepublishEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElS3El>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sns: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElSnsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sqs: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElSqsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step_functions: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElStepFunctionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestream: std::option::Option<std::vec::Vec<IotTopicRuleErrorActionElTimestreamEl>>,
    dynamic: IotTopicRuleErrorActionElDynamic,
}

impl IotTopicRuleErrorActionEl {
    #[doc = "Set the field `cloudwatch_alarm`.\n"]
    pub fn set_cloudwatch_alarm(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElCloudwatchAlarmEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElCloudwatchLogsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElCloudwatchMetricEl>>,
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
    pub fn set_dynamodb(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElDynamodbEl>>,
    ) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElDynamodbv2El>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElElasticsearchEl>>,
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
    pub fn set_firehose(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElFirehoseEl>>,
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

    #[doc = "Set the field `http`.\n"]
    pub fn set_http(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElHttpEl>>,
    ) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElIotAnalyticsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElIotEventsEl>>,
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
    pub fn set_kafka(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElKafkaEl>>,
    ) -> Self {
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
    pub fn set_kinesis(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElKinesisEl>>,
    ) -> Self {
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
    pub fn set_lambda(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElLambdaEl>>,
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

    #[doc = "Set the field `republish`.\n"]
    pub fn set_republish(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElRepublishEl>>,
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
    pub fn set_s3(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElS3El>>,
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

    #[doc = "Set the field `sns`.\n"]
    pub fn set_sns(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElSnsEl>>,
    ) -> Self {
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
    pub fn set_sqs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElSqsEl>>,
    ) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElStepFunctionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleErrorActionElTimestreamEl>>,
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

impl terrars::ToListMappable for IotTopicRuleErrorActionEl {
    type O = terrars::BlockAssignable<IotTopicRuleErrorActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleErrorActionEl {}

impl BuildIotTopicRuleErrorActionEl {
    pub fn build(self) -> IotTopicRuleErrorActionEl {
        IotTopicRuleErrorActionEl {
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

pub struct IotTopicRuleErrorActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleErrorActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleErrorActionElRef {
        IotTopicRuleErrorActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleErrorActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_alarm` after provisioning.\n"]
    pub fn cloudwatch_alarm(&self) -> terrars::ListRef<IotTopicRuleErrorActionElCloudwatchAlarmElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_alarm", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(&self) -> terrars::ListRef<IotTopicRuleErrorActionElCloudwatchLogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_metric` after provisioning.\n"]
    pub fn cloudwatch_metric(&self) -> terrars::ListRef<IotTopicRuleErrorActionElCloudwatchMetricElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_metric", self.base))
    }

    #[doc = "Get a reference to the value of field `dynamodb` after provisioning.\n"]
    pub fn dynamodb(&self) -> terrars::ListRef<IotTopicRuleErrorActionElDynamodbElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynamodb", self.base))
    }

    #[doc = "Get a reference to the value of field `dynamodbv2` after provisioning.\n"]
    pub fn dynamodbv2(&self) -> terrars::ListRef<IotTopicRuleErrorActionElDynamodbv2ElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynamodbv2", self.base))
    }

    #[doc = "Get a reference to the value of field `elasticsearch` after provisioning.\n"]
    pub fn elasticsearch(&self) -> terrars::ListRef<IotTopicRuleErrorActionElElasticsearchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elasticsearch", self.base))
    }

    #[doc = "Get a reference to the value of field `firehose` after provisioning.\n"]
    pub fn firehose(&self) -> terrars::ListRef<IotTopicRuleErrorActionElFirehoseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firehose", self.base))
    }

    #[doc = "Get a reference to the value of field `http` after provisioning.\n"]
    pub fn http(&self) -> terrars::ListRef<IotTopicRuleErrorActionElHttpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http", self.base))
    }

    #[doc = "Get a reference to the value of field `iot_analytics` after provisioning.\n"]
    pub fn iot_analytics(&self) -> terrars::ListRef<IotTopicRuleErrorActionElIotAnalyticsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iot_analytics", self.base))
    }

    #[doc = "Get a reference to the value of field `iot_events` after provisioning.\n"]
    pub fn iot_events(&self) -> terrars::ListRef<IotTopicRuleErrorActionElIotEventsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iot_events", self.base))
    }

    #[doc = "Get a reference to the value of field `kafka` after provisioning.\n"]
    pub fn kafka(&self) -> terrars::ListRef<IotTopicRuleErrorActionElKafkaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kafka", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis` after provisioning.\n"]
    pub fn kinesis(&self) -> terrars::ListRef<IotTopicRuleErrorActionElKinesisElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda` after provisioning.\n"]
    pub fn lambda(&self) -> terrars::ListRef<IotTopicRuleErrorActionElLambdaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda", self.base))
    }

    #[doc = "Get a reference to the value of field `republish` after provisioning.\n"]
    pub fn republish(&self) -> terrars::ListRef<IotTopicRuleErrorActionElRepublishElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.republish", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<IotTopicRuleErrorActionElS3ElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }

    #[doc = "Get a reference to the value of field `sns` after provisioning.\n"]
    pub fn sns(&self) -> terrars::ListRef<IotTopicRuleErrorActionElSnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sns", self.base))
    }

    #[doc = "Get a reference to the value of field `sqs` after provisioning.\n"]
    pub fn sqs(&self) -> terrars::ListRef<IotTopicRuleErrorActionElSqsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sqs", self.base))
    }

    #[doc = "Get a reference to the value of field `step_functions` after provisioning.\n"]
    pub fn step_functions(&self) -> terrars::ListRef<IotTopicRuleErrorActionElStepFunctionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.step_functions", self.base))
    }

    #[doc = "Get a reference to the value of field `timestream` after provisioning.\n"]
    pub fn timestream(&self) -> terrars::ListRef<IotTopicRuleErrorActionElTimestreamElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timestream", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRuleFirehoseEl {
    delivery_stream_name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    separator: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IotTopicRuleFirehoseEl {
    #[doc = "Set the field `separator`.\n"]
    pub fn set_separator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.separator = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRuleFirehoseEl {
    type O = terrars::BlockAssignable<IotTopicRuleFirehoseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleFirehoseEl {
    #[doc = ""]
    pub delivery_stream_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleFirehoseEl {
    pub fn build(self) -> IotTopicRuleFirehoseEl {
        IotTopicRuleFirehoseEl {
            delivery_stream_name: self.delivery_stream_name,
            role_arn: self.role_arn,
            separator: core::default::Default::default(),
        }
    }
}

pub struct IotTopicRuleFirehoseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleFirehoseElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleFirehoseElRef {
        IotTopicRuleFirehoseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleFirehoseElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
pub struct IotTopicRuleHttpElHttpHeaderEl {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleHttpElHttpHeaderEl { }

impl terrars::ToListMappable for IotTopicRuleHttpElHttpHeaderEl {
    type O = terrars::BlockAssignable<IotTopicRuleHttpElHttpHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleHttpElHttpHeaderEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleHttpElHttpHeaderEl {
    pub fn build(self) -> IotTopicRuleHttpElHttpHeaderEl {
        IotTopicRuleHttpElHttpHeaderEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct IotTopicRuleHttpElHttpHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleHttpElHttpHeaderElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleHttpElHttpHeaderElRef {
        IotTopicRuleHttpElHttpHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleHttpElHttpHeaderElRef {
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
struct IotTopicRuleHttpElDynamic {
    http_header: std::option::Option<terrars::DynamicBlock<IotTopicRuleHttpElHttpHeaderEl>>,
}

#[derive(Serialize)]
pub struct IotTopicRuleHttpEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confirmation_url: std::option::Option<terrars::PrimField<std::string::String>>,
    url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_header: std::option::Option<std::vec::Vec<IotTopicRuleHttpElHttpHeaderEl>>,
    dynamic: IotTopicRuleHttpElDynamic,
}

impl IotTopicRuleHttpEl {
    #[doc = "Set the field `confirmation_url`.\n"]
    pub fn set_confirmation_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.confirmation_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_header`.\n"]
    pub fn set_http_header(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleHttpElHttpHeaderEl>>,
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

impl terrars::ToListMappable for IotTopicRuleHttpEl {
    type O = terrars::BlockAssignable<IotTopicRuleHttpEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleHttpEl {
    #[doc = ""]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleHttpEl {
    pub fn build(self) -> IotTopicRuleHttpEl {
        IotTopicRuleHttpEl {
            confirmation_url: core::default::Default::default(),
            url: self.url,
            http_header: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IotTopicRuleHttpElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleHttpElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleHttpElRef {
        IotTopicRuleHttpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleHttpElRef {
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
    pub fn http_header(&self) -> terrars::ListRef<IotTopicRuleHttpElHttpHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_header", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRuleIotAnalyticsEl {
    channel_name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleIotAnalyticsEl { }

impl terrars::ToListMappable for IotTopicRuleIotAnalyticsEl {
    type O = terrars::BlockAssignable<IotTopicRuleIotAnalyticsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleIotAnalyticsEl {
    #[doc = ""]
    pub channel_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleIotAnalyticsEl {
    pub fn build(self) -> IotTopicRuleIotAnalyticsEl {
        IotTopicRuleIotAnalyticsEl {
            channel_name: self.channel_name,
            role_arn: self.role_arn,
        }
    }
}

pub struct IotTopicRuleIotAnalyticsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleIotAnalyticsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleIotAnalyticsElRef {
        IotTopicRuleIotAnalyticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleIotAnalyticsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
pub struct IotTopicRuleIotEventsEl {
    input_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_id: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleIotEventsEl {
    #[doc = "Set the field `message_id`.\n"]
    pub fn set_message_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRuleIotEventsEl {
    type O = terrars::BlockAssignable<IotTopicRuleIotEventsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleIotEventsEl {
    #[doc = ""]
    pub input_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleIotEventsEl {
    pub fn build(self) -> IotTopicRuleIotEventsEl {
        IotTopicRuleIotEventsEl {
            input_name: self.input_name,
            message_id: core::default::Default::default(),
            role_arn: self.role_arn,
        }
    }
}

pub struct IotTopicRuleIotEventsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleIotEventsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleIotEventsElRef {
        IotTopicRuleIotEventsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleIotEventsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
pub struct IotTopicRuleKafkaEl {
    client_properties: terrars::RecField<terrars::PrimField<std::string::String>>,
    destination_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition: std::option::Option<terrars::PrimField<std::string::String>>,
    topic: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleKafkaEl {
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
}

impl terrars::ToListMappable for IotTopicRuleKafkaEl {
    type O = terrars::BlockAssignable<IotTopicRuleKafkaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleKafkaEl {
    #[doc = ""]
    pub client_properties: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub destination_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub topic: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleKafkaEl {
    pub fn build(self) -> IotTopicRuleKafkaEl {
        IotTopicRuleKafkaEl {
            client_properties: self.client_properties,
            destination_arn: self.destination_arn,
            key: core::default::Default::default(),
            partition: core::default::Default::default(),
            topic: self.topic,
        }
    }
}

pub struct IotTopicRuleKafkaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleKafkaElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleKafkaElRef {
        IotTopicRuleKafkaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleKafkaElRef {
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
}

#[derive(Serialize)]
pub struct IotTopicRuleKinesisEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_key: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    stream_name: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleKinesisEl {
    #[doc = "Set the field `partition_key`.\n"]
    pub fn set_partition_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.partition_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRuleKinesisEl {
    type O = terrars::BlockAssignable<IotTopicRuleKinesisEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleKinesisEl {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub stream_name: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleKinesisEl {
    pub fn build(self) -> IotTopicRuleKinesisEl {
        IotTopicRuleKinesisEl {
            partition_key: core::default::Default::default(),
            role_arn: self.role_arn,
            stream_name: self.stream_name,
        }
    }
}

pub struct IotTopicRuleKinesisElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleKinesisElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleKinesisElRef {
        IotTopicRuleKinesisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleKinesisElRef {
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
pub struct IotTopicRuleLambdaEl {
    function_arn: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleLambdaEl { }

impl terrars::ToListMappable for IotTopicRuleLambdaEl {
    type O = terrars::BlockAssignable<IotTopicRuleLambdaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleLambdaEl {
    #[doc = ""]
    pub function_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleLambdaEl {
    pub fn build(self) -> IotTopicRuleLambdaEl {
        IotTopicRuleLambdaEl { function_arn: self.function_arn }
    }
}

pub struct IotTopicRuleLambdaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleLambdaElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleLambdaElRef {
        IotTopicRuleLambdaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleLambdaElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct IotTopicRuleRepublishEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qos: std::option::Option<terrars::PrimField<f64>>,
    role_arn: terrars::PrimField<std::string::String>,
    topic: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleRepublishEl {
    #[doc = "Set the field `qos`.\n"]
    pub fn set_qos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.qos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRuleRepublishEl {
    type O = terrars::BlockAssignable<IotTopicRuleRepublishEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleRepublishEl {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub topic: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleRepublishEl {
    pub fn build(self) -> IotTopicRuleRepublishEl {
        IotTopicRuleRepublishEl {
            qos: core::default::Default::default(),
            role_arn: self.role_arn,
            topic: self.topic,
        }
    }
}

pub struct IotTopicRuleRepublishElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleRepublishElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleRepublishElRef {
        IotTopicRuleRepublishElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleRepublishElRef {
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
pub struct IotTopicRuleS3El {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canned_acl: std::option::Option<terrars::PrimField<std::string::String>>,
    key: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleS3El {
    #[doc = "Set the field `canned_acl`.\n"]
    pub fn set_canned_acl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.canned_acl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRuleS3El {
    type O = terrars::BlockAssignable<IotTopicRuleS3El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleS3El {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleS3El {
    pub fn build(self) -> IotTopicRuleS3El {
        IotTopicRuleS3El {
            bucket_name: self.bucket_name,
            canned_acl: core::default::Default::default(),
            key: self.key,
            role_arn: self.role_arn,
        }
    }
}

pub struct IotTopicRuleS3ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleS3ElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleS3ElRef {
        IotTopicRuleS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleS3ElRef {
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
pub struct IotTopicRuleSnsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_format: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    target_arn: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleSnsEl {
    #[doc = "Set the field `message_format`.\n"]
    pub fn set_message_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message_format = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRuleSnsEl {
    type O = terrars::BlockAssignable<IotTopicRuleSnsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleSnsEl {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_arn: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleSnsEl {
    pub fn build(self) -> IotTopicRuleSnsEl {
        IotTopicRuleSnsEl {
            message_format: core::default::Default::default(),
            role_arn: self.role_arn,
            target_arn: self.target_arn,
        }
    }
}

pub struct IotTopicRuleSnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleSnsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleSnsElRef {
        IotTopicRuleSnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleSnsElRef {
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
pub struct IotTopicRuleSqsEl {
    queue_url: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    use_base64: terrars::PrimField<bool>,
}

impl IotTopicRuleSqsEl { }

impl terrars::ToListMappable for IotTopicRuleSqsEl {
    type O = terrars::BlockAssignable<IotTopicRuleSqsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleSqsEl {
    #[doc = ""]
    pub queue_url: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub use_base64: terrars::PrimField<bool>,
}

impl BuildIotTopicRuleSqsEl {
    pub fn build(self) -> IotTopicRuleSqsEl {
        IotTopicRuleSqsEl {
            queue_url: self.queue_url,
            role_arn: self.role_arn,
            use_base64: self.use_base64,
        }
    }
}

pub struct IotTopicRuleSqsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleSqsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleSqsElRef {
        IotTopicRuleSqsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleSqsElRef {
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
pub struct IotTopicRuleStepFunctionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    state_machine_name: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleStepFunctionsEl {
    #[doc = "Set the field `execution_name_prefix`.\n"]
    pub fn set_execution_name_prefix(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.execution_name_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotTopicRuleStepFunctionsEl {
    type O = terrars::BlockAssignable<IotTopicRuleStepFunctionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleStepFunctionsEl {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub state_machine_name: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleStepFunctionsEl {
    pub fn build(self) -> IotTopicRuleStepFunctionsEl {
        IotTopicRuleStepFunctionsEl {
            execution_name_prefix: core::default::Default::default(),
            role_arn: self.role_arn,
            state_machine_name: self.state_machine_name,
        }
    }
}

pub struct IotTopicRuleStepFunctionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleStepFunctionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleStepFunctionsElRef {
        IotTopicRuleStepFunctionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleStepFunctionsElRef {
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
pub struct IotTopicRuleTimestreamElDimensionEl {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleTimestreamElDimensionEl { }

impl terrars::ToListMappable for IotTopicRuleTimestreamElDimensionEl {
    type O = terrars::BlockAssignable<IotTopicRuleTimestreamElDimensionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleTimestreamElDimensionEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleTimestreamElDimensionEl {
    pub fn build(self) -> IotTopicRuleTimestreamElDimensionEl {
        IotTopicRuleTimestreamElDimensionEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct IotTopicRuleTimestreamElDimensionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleTimestreamElDimensionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleTimestreamElDimensionElRef {
        IotTopicRuleTimestreamElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleTimestreamElDimensionElRef {
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
pub struct IotTopicRuleTimestreamElTimestampEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl IotTopicRuleTimestreamElTimestampEl { }

impl terrars::ToListMappable for IotTopicRuleTimestreamElTimestampEl {
    type O = terrars::BlockAssignable<IotTopicRuleTimestreamElTimestampEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleTimestreamElTimestampEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleTimestreamElTimestampEl {
    pub fn build(self) -> IotTopicRuleTimestreamElTimestampEl {
        IotTopicRuleTimestreamElTimestampEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct IotTopicRuleTimestreamElTimestampElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleTimestreamElTimestampElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleTimestreamElTimestampElRef {
        IotTopicRuleTimestreamElTimestampElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleTimestreamElTimestampElRef {
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
struct IotTopicRuleTimestreamElDynamic {
    dimension: std::option::Option<terrars::DynamicBlock<IotTopicRuleTimestreamElDimensionEl>>,
    timestamp: std::option::Option<terrars::DynamicBlock<IotTopicRuleTimestreamElTimestampEl>>,
}

#[derive(Serialize)]
pub struct IotTopicRuleTimestreamEl {
    database_name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<IotTopicRuleTimestreamElDimensionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamp: std::option::Option<std::vec::Vec<IotTopicRuleTimestreamElTimestampEl>>,
    dynamic: IotTopicRuleTimestreamElDynamic,
}

impl IotTopicRuleTimestreamEl {
    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleTimestreamElDimensionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IotTopicRuleTimestreamElTimestampEl>>,
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

impl terrars::ToListMappable for IotTopicRuleTimestreamEl {
    type O = terrars::BlockAssignable<IotTopicRuleTimestreamEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotTopicRuleTimestreamEl {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildIotTopicRuleTimestreamEl {
    pub fn build(self) -> IotTopicRuleTimestreamEl {
        IotTopicRuleTimestreamEl {
            database_name: self.database_name,
            role_arn: self.role_arn,
            table_name: self.table_name,
            dimension: core::default::Default::default(),
            timestamp: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IotTopicRuleTimestreamElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotTopicRuleTimestreamElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotTopicRuleTimestreamElRef {
        IotTopicRuleTimestreamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotTopicRuleTimestreamElRef {
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
    pub fn timestamp(&self) -> terrars::ListRef<IotTopicRuleTimestreamElTimestampElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timestamp", self.base))
    }
}

#[derive(Serialize, Default)]
struct IotTopicRuleDynamic {
    cloudwatch_alarm: std::option::Option<terrars::DynamicBlock<IotTopicRuleCloudwatchAlarmEl>>,
    cloudwatch_logs: std::option::Option<terrars::DynamicBlock<IotTopicRuleCloudwatchLogsEl>>,
    cloudwatch_metric: std::option::Option<terrars::DynamicBlock<IotTopicRuleCloudwatchMetricEl>>,
    dynamodb: std::option::Option<terrars::DynamicBlock<IotTopicRuleDynamodbEl>>,
    dynamodbv2: std::option::Option<terrars::DynamicBlock<IotTopicRuleDynamodbv2El>>,
    elasticsearch: std::option::Option<terrars::DynamicBlock<IotTopicRuleElasticsearchEl>>,
    error_action: std::option::Option<terrars::DynamicBlock<IotTopicRuleErrorActionEl>>,
    firehose: std::option::Option<terrars::DynamicBlock<IotTopicRuleFirehoseEl>>,
    http: std::option::Option<terrars::DynamicBlock<IotTopicRuleHttpEl>>,
    iot_analytics: std::option::Option<terrars::DynamicBlock<IotTopicRuleIotAnalyticsEl>>,
    iot_events: std::option::Option<terrars::DynamicBlock<IotTopicRuleIotEventsEl>>,
    kafka: std::option::Option<terrars::DynamicBlock<IotTopicRuleKafkaEl>>,
    kinesis: std::option::Option<terrars::DynamicBlock<IotTopicRuleKinesisEl>>,
    lambda: std::option::Option<terrars::DynamicBlock<IotTopicRuleLambdaEl>>,
    republish: std::option::Option<terrars::DynamicBlock<IotTopicRuleRepublishEl>>,
    s3: std::option::Option<terrars::DynamicBlock<IotTopicRuleS3El>>,
    sns: std::option::Option<terrars::DynamicBlock<IotTopicRuleSnsEl>>,
    sqs: std::option::Option<terrars::DynamicBlock<IotTopicRuleSqsEl>>,
    step_functions: std::option::Option<terrars::DynamicBlock<IotTopicRuleStepFunctionsEl>>,
    timestream: std::option::Option<terrars::DynamicBlock<IotTopicRuleTimestreamEl>>,
}
