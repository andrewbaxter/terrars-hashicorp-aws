#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataS3BucketNotificationData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataS3BucketNotification_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataS3BucketNotificationData_>,
}

#[derive(Clone)]
pub struct DataS3BucketNotification(std::rc::Rc<DataS3BucketNotification_>);

impl DataS3BucketNotification {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `eventbridge` after provisioning.\n"]
    pub fn eventbridge(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eventbridge", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_function` after provisioning.\n"]
    pub fn lambda_function(&self) -> terrars::ListRef<DataS3BucketNotification_LambdaFunctionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_function", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queue` after provisioning.\n"]
    pub fn queue(&self) -> terrars::ListRef<DataS3BucketNotification_QueueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.queue", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> terrars::ListRef<DataS3BucketNotification_TopicRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topic", self.extract_ref()))
    }
}

impl terrars::Referable for DataS3BucketNotification {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataS3BucketNotification { }

impl terrars::ToListMappable for DataS3BucketNotification {
    type O = terrars::ListRef<DataS3BucketNotificationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataS3BucketNotification_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_s3_bucket_notification".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataS3BucketNotification {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildDataS3BucketNotification {
    pub fn build(self, stack: &mut terrars::Stack) -> DataS3BucketNotification {
        let out = DataS3BucketNotification(std::rc::Rc::new(DataS3BucketNotification_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataS3BucketNotificationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                bucket: self.bucket,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataS3BucketNotificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3BucketNotificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataS3BucketNotificationRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `eventbridge` after provisioning.\n"]
    pub fn eventbridge(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eventbridge", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_function` after provisioning.\n"]
    pub fn lambda_function(&self) -> terrars::ListRef<DataS3BucketNotification_LambdaFunctionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_function", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queue` after provisioning.\n"]
    pub fn queue(&self) -> terrars::ListRef<DataS3BucketNotification_QueueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.queue", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> terrars::ListRef<DataS3BucketNotification_TopicRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topic", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataS3BucketNotification_LambdaFunction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataS3BucketNotification_LambdaFunction {
    #[doc = "Set the field `events`.\n"]
    pub fn set_events(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.events = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter_prefix`.\n"]
    pub fn set_filter_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter_suffix`.\n"]
    pub fn set_filter_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter_suffix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lambda_function_arn`.\n"]
    pub fn set_lambda_function_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.lambda_function_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataS3BucketNotification_LambdaFunction {
    type O = terrars::BlockAssignable<DataS3BucketNotification_LambdaFunction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3BucketNotification_LambdaFunction {}

impl BuildDataS3BucketNotification_LambdaFunction {
    pub fn build(self) -> DataS3BucketNotification_LambdaFunction {
        DataS3BucketNotification_LambdaFunction {
            events: core::default::Default::default(),
            filter_prefix: core::default::Default::default(),
            filter_suffix: core::default::Default::default(),
            id: core::default::Default::default(),
            lambda_function_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataS3BucketNotification_LambdaFunctionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3BucketNotification_LambdaFunctionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataS3BucketNotification_LambdaFunctionRef {
        DataS3BucketNotification_LambdaFunctionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3BucketNotification_LambdaFunctionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.events", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_prefix` after provisioning.\n"]
    pub fn filter_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_suffix` after provisioning.\n"]
    pub fn filter_suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_suffix", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_function_arn` after provisioning.\n"]
    pub fn lambda_function_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda_function_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataS3BucketNotification_Queue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataS3BucketNotification_Queue {
    #[doc = "Set the field `events`.\n"]
    pub fn set_events(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.events = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter_prefix`.\n"]
    pub fn set_filter_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter_suffix`.\n"]
    pub fn set_filter_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter_suffix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `queue_arn`.\n"]
    pub fn set_queue_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.queue_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataS3BucketNotification_Queue {
    type O = terrars::BlockAssignable<DataS3BucketNotification_Queue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3BucketNotification_Queue {}

impl BuildDataS3BucketNotification_Queue {
    pub fn build(self) -> DataS3BucketNotification_Queue {
        DataS3BucketNotification_Queue {
            events: core::default::Default::default(),
            filter_prefix: core::default::Default::default(),
            filter_suffix: core::default::Default::default(),
            id: core::default::Default::default(),
            queue_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataS3BucketNotification_QueueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3BucketNotification_QueueRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataS3BucketNotification_QueueRef {
        DataS3BucketNotification_QueueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3BucketNotification_QueueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.events", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_prefix` after provisioning.\n"]
    pub fn filter_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_suffix` after provisioning.\n"]
    pub fn filter_suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_suffix", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_arn` after provisioning.\n"]
    pub fn queue_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataS3BucketNotification_Topic {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataS3BucketNotification_Topic {
    #[doc = "Set the field `events`.\n"]
    pub fn set_events(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.events = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter_prefix`.\n"]
    pub fn set_filter_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter_suffix`.\n"]
    pub fn set_filter_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter_suffix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `topic_arn`.\n"]
    pub fn set_topic_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topic_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataS3BucketNotification_Topic {
    type O = terrars::BlockAssignable<DataS3BucketNotification_Topic>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3BucketNotification_Topic {}

impl BuildDataS3BucketNotification_Topic {
    pub fn build(self) -> DataS3BucketNotification_Topic {
        DataS3BucketNotification_Topic {
            events: core::default::Default::default(),
            filter_prefix: core::default::Default::default(),
            filter_suffix: core::default::Default::default(),
            id: core::default::Default::default(),
            topic_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataS3BucketNotification_TopicRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3BucketNotification_TopicRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataS3BucketNotification_TopicRef {
        DataS3BucketNotification_TopicRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3BucketNotification_TopicRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.events", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_prefix` after provisioning.\n"]
    pub fn filter_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_suffix` after provisioning.\n"]
    pub fn filter_suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_suffix", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}
