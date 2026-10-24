use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3BucketNotificationData {
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
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eventbridge: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function: std::option::Option<std::vec::Vec<S3BucketNotificationLambdaFunctionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue: std::option::Option<std::vec::Vec<S3BucketNotificationQueueEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic: std::option::Option<std::vec::Vec<S3BucketNotificationTopicEl>>,
    dynamic: S3BucketNotificationDynamic,
}

struct S3BucketNotification_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3BucketNotificationData>,
}

#[derive(Clone)]
pub struct S3BucketNotification(std::rc::Rc<S3BucketNotification_>);

impl S3BucketNotification {
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

    #[doc = "Set the field `eventbridge`.\n"]
    pub fn set_eventbridge(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().eventbridge = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lambda_function`.\n"]
    pub fn set_lambda_function(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketNotificationLambdaFunctionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lambda_function = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lambda_function = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `queue`.\n"]
    pub fn set_queue(self, v: impl std::convert::Into<terrars::BlockAssignable<S3BucketNotificationQueueEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().queue = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.queue = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `topic`.\n"]
    pub fn set_topic(self, v: impl std::convert::Into<terrars::BlockAssignable<S3BucketNotificationTopicEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().topic = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.topic = std::option::Option::Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_function` after provisioning.\n"]
    pub fn lambda_function(&self) -> terrars::ListRef<S3BucketNotificationLambdaFunctionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_function", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queue` after provisioning.\n"]
    pub fn queue(&self) -> terrars::ListRef<S3BucketNotificationQueueElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.queue", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> terrars::ListRef<S3BucketNotificationTopicElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topic", self.extract_ref()))
    }
}

impl terrars::Referable for S3BucketNotification {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3BucketNotification { }

impl terrars::ToListMappable for S3BucketNotification {
    type O = terrars::ListRef<S3BucketNotificationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3BucketNotification_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3_bucket_notification".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketNotification {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketNotification {
    pub fn build(self, stack: &mut terrars::Stack) -> S3BucketNotification {
        let out = S3BucketNotification(std::rc::Rc::new(S3BucketNotification_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3BucketNotificationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bucket: self.bucket,
                eventbridge: core::default::Default::default(),
                id: core::default::Default::default(),
                lambda_function: core::default::Default::default(),
                queue: core::default::Default::default(),
                topic: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketNotificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketNotificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3BucketNotificationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `eventbridge` after provisioning.\n"]
    pub fn eventbridge(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eventbridge", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_function` after provisioning.\n"]
    pub fn lambda_function(&self) -> terrars::ListRef<S3BucketNotificationLambdaFunctionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_function", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queue` after provisioning.\n"]
    pub fn queue(&self) -> terrars::ListRef<S3BucketNotificationQueueElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.queue", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> terrars::ListRef<S3BucketNotificationTopicElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topic", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct S3BucketNotificationLambdaFunctionEl {
    events: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl S3BucketNotificationLambdaFunctionEl {
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

impl terrars::ToListMappable for S3BucketNotificationLambdaFunctionEl {
    type O = terrars::BlockAssignable<S3BucketNotificationLambdaFunctionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketNotificationLambdaFunctionEl {
    #[doc = ""]
    pub events: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildS3BucketNotificationLambdaFunctionEl {
    pub fn build(self) -> S3BucketNotificationLambdaFunctionEl {
        S3BucketNotificationLambdaFunctionEl {
            events: self.events,
            filter_prefix: core::default::Default::default(),
            filter_suffix: core::default::Default::default(),
            id: core::default::Default::default(),
            lambda_function_arn: core::default::Default::default(),
        }
    }
}

pub struct S3BucketNotificationLambdaFunctionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketNotificationLambdaFunctionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketNotificationLambdaFunctionElRef {
        S3BucketNotificationLambdaFunctionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketNotificationLambdaFunctionElRef {
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
pub struct S3BucketNotificationQueueEl {
    events: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    queue_arn: terrars::PrimField<std::string::String>,
}

impl S3BucketNotificationQueueEl {
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
}

impl terrars::ToListMappable for S3BucketNotificationQueueEl {
    type O = terrars::BlockAssignable<S3BucketNotificationQueueEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketNotificationQueueEl {
    #[doc = ""]
    pub events: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub queue_arn: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketNotificationQueueEl {
    pub fn build(self) -> S3BucketNotificationQueueEl {
        S3BucketNotificationQueueEl {
            events: self.events,
            filter_prefix: core::default::Default::default(),
            filter_suffix: core::default::Default::default(),
            id: core::default::Default::default(),
            queue_arn: self.queue_arn,
        }
    }
}

pub struct S3BucketNotificationQueueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketNotificationQueueElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketNotificationQueueElRef {
        S3BucketNotificationQueueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketNotificationQueueElRef {
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
pub struct S3BucketNotificationTopicEl {
    events: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    topic_arn: terrars::PrimField<std::string::String>,
}

impl S3BucketNotificationTopicEl {
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
}

impl terrars::ToListMappable for S3BucketNotificationTopicEl {
    type O = terrars::BlockAssignable<S3BucketNotificationTopicEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketNotificationTopicEl {
    #[doc = ""]
    pub events: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub topic_arn: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketNotificationTopicEl {
    pub fn build(self) -> S3BucketNotificationTopicEl {
        S3BucketNotificationTopicEl {
            events: self.events,
            filter_prefix: core::default::Default::default(),
            filter_suffix: core::default::Default::default(),
            id: core::default::Default::default(),
            topic_arn: self.topic_arn,
        }
    }
}

pub struct S3BucketNotificationTopicElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketNotificationTopicElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketNotificationTopicElRef {
        S3BucketNotificationTopicElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketNotificationTopicElRef {
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

#[derive(Serialize, Default)]
struct S3BucketNotificationDynamic {
    lambda_function: std::option::Option<terrars::DynamicBlock<S3BucketNotificationLambdaFunctionEl>>,
    queue: std::option::Option<terrars::DynamicBlock<S3BucketNotificationQueueEl>>,
    topic: std::option::Option<terrars::DynamicBlock<S3BucketNotificationTopicEl>>,
}
