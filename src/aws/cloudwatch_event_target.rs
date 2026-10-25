#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudwatchEventTargetData_ {
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
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_bus_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    rule: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    appsync_target: std::option::Option<std::vec::Vec<CloudwatchEventTarget_AppsyncTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_target: std::option::Option<std::vec::Vec<CloudwatchEventTarget_BatchTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dead_letter_config: std::option::Option<std::vec::Vec<CloudwatchEventTarget_DeadLetterConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecs_target: std::option::Option<std::vec::Vec<CloudwatchEventTarget_EcsTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_target: std::option::Option<std::vec::Vec<CloudwatchEventTarget_HttpTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_transformer: std::option::Option<std::vec::Vec<CloudwatchEventTarget_InputTransformer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_target: std::option::Option<std::vec::Vec<CloudwatchEventTarget_KinesisTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redshift_target: std::option::Option<std::vec::Vec<CloudwatchEventTarget_RedshiftTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<CloudwatchEventTarget_RetryPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_command_targets: std::option::Option<std::vec::Vec<CloudwatchEventTarget_RunCommandTargets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_pipeline_target: std::option::Option<std::vec::Vec<CloudwatchEventTarget_SagemakerPipelineTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sqs_target: std::option::Option<std::vec::Vec<CloudwatchEventTarget_SqsTarget>>,
    dynamic: CloudwatchEventTargetDynamic,
}

struct CloudwatchEventTarget_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudwatchEventTargetData_>,
}

#[derive(Clone)]
pub struct CloudwatchEventTarget(std::rc::Rc<CloudwatchEventTarget_>);

impl CloudwatchEventTarget {
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

    #[doc = "Set the field `event_bus_name`.\n"]
    pub fn set_event_bus_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().event_bus_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `force_destroy`.\n"]
    pub fn set_force_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input`.\n"]
    pub fn set_input(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().input = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_path`.\n"]
    pub fn set_input_path(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().input_path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_id`.\n"]
    pub fn set_target_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().target_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `appsync_target`.\n"]
    pub fn set_appsync_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTarget_AppsyncTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().appsync_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.appsync_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `batch_target`.\n"]
    pub fn set_batch_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTarget_BatchTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().batch_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.batch_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dead_letter_config`.\n"]
    pub fn set_dead_letter_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTarget_DeadLetterConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dead_letter_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dead_letter_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ecs_target`.\n"]
    pub fn set_ecs_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTarget_EcsTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ecs_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ecs_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_target`.\n"]
    pub fn set_http_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTarget_HttpTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().http_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.http_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_transformer`.\n"]
    pub fn set_input_transformer(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTarget_InputTransformer>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input_transformer = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input_transformer = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_target`.\n"]
    pub fn set_kinesis_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTarget_KinesisTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kinesis_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kinesis_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redshift_target`.\n"]
    pub fn set_redshift_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTarget_RedshiftTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().redshift_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.redshift_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTarget_RetryPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retry_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retry_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `run_command_targets`.\n"]
    pub fn set_run_command_targets(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTarget_RunCommandTargets>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().run_command_targets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.run_command_targets = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sagemaker_pipeline_target`.\n"]
    pub fn set_sagemaker_pipeline_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTarget_SagemakerPipelineTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sagemaker_pipeline_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sagemaker_pipeline_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sqs_target`.\n"]
    pub fn set_sqs_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTarget_SqsTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sqs_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sqs_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_bus_name` after provisioning.\n"]
    pub fn event_bus_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_bus_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_path` after provisioning.\n"]
    pub fn input_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_path", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_id` after provisioning.\n"]
    pub fn target_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `appsync_target` after provisioning.\n"]
    pub fn appsync_target(&self) -> terrars::ListRef<CloudwatchEventTarget_AppsyncTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.appsync_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `batch_target` after provisioning.\n"]
    pub fn batch_target(&self) -> terrars::ListRef<CloudwatchEventTarget_BatchTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.batch_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> terrars::ListRef<CloudwatchEventTarget_DeadLetterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ecs_target` after provisioning.\n"]
    pub fn ecs_target(&self) -> terrars::ListRef<CloudwatchEventTarget_EcsTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ecs_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_target` after provisioning.\n"]
    pub fn http_target(&self) -> terrars::ListRef<CloudwatchEventTarget_HttpTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_transformer` after provisioning.\n"]
    pub fn input_transformer(&self) -> terrars::ListRef<CloudwatchEventTarget_InputTransformerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_transformer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kinesis_target` after provisioning.\n"]
    pub fn kinesis_target(&self) -> terrars::ListRef<CloudwatchEventTarget_KinesisTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redshift_target` after provisioning.\n"]
    pub fn redshift_target(&self) -> terrars::ListRef<CloudwatchEventTarget_RedshiftTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<CloudwatchEventTarget_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `run_command_targets` after provisioning.\n"]
    pub fn run_command_targets(&self) -> terrars::ListRef<CloudwatchEventTarget_RunCommandTargetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.run_command_targets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sagemaker_pipeline_target` after provisioning.\n"]
    pub fn sagemaker_pipeline_target(&self) -> terrars::ListRef<CloudwatchEventTarget_SagemakerPipelineTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sagemaker_pipeline_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sqs_target` after provisioning.\n"]
    pub fn sqs_target(&self) -> terrars::ListRef<CloudwatchEventTarget_SqsTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sqs_target", self.extract_ref()))
    }
}

impl terrars::Referable for CloudwatchEventTarget {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudwatchEventTarget { }

impl terrars::ToListMappable for CloudwatchEventTarget {
    type O = terrars::ListRef<CloudwatchEventTargetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudwatchEventTarget_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudwatch_event_target".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudwatchEventTarget {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventTarget {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudwatchEventTarget {
        let out = CloudwatchEventTarget(std::rc::Rc::new(CloudwatchEventTarget_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudwatchEventTargetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                arn: self.arn,
                event_bus_name: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                input: core::default::Default::default(),
                input_path: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: core::default::Default::default(),
                rule: self.rule,
                target_id: core::default::Default::default(),
                appsync_target: core::default::Default::default(),
                batch_target: core::default::Default::default(),
                dead_letter_config: core::default::Default::default(),
                ecs_target: core::default::Default::default(),
                http_target: core::default::Default::default(),
                input_transformer: core::default::Default::default(),
                kinesis_target: core::default::Default::default(),
                redshift_target: core::default::Default::default(),
                retry_policy: core::default::Default::default(),
                run_command_targets: core::default::Default::default(),
                sagemaker_pipeline_target: core::default::Default::default(),
                sqs_target: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudwatchEventTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudwatchEventTargetRef {
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

    #[doc = "Get a reference to the value of field `event_bus_name` after provisioning.\n"]
    pub fn event_bus_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_bus_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_path` after provisioning.\n"]
    pub fn input_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_path", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_id` after provisioning.\n"]
    pub fn target_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `appsync_target` after provisioning.\n"]
    pub fn appsync_target(&self) -> terrars::ListRef<CloudwatchEventTarget_AppsyncTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.appsync_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `batch_target` after provisioning.\n"]
    pub fn batch_target(&self) -> terrars::ListRef<CloudwatchEventTarget_BatchTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.batch_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> terrars::ListRef<CloudwatchEventTarget_DeadLetterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ecs_target` after provisioning.\n"]
    pub fn ecs_target(&self) -> terrars::ListRef<CloudwatchEventTarget_EcsTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ecs_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_target` after provisioning.\n"]
    pub fn http_target(&self) -> terrars::ListRef<CloudwatchEventTarget_HttpTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_transformer` after provisioning.\n"]
    pub fn input_transformer(&self) -> terrars::ListRef<CloudwatchEventTarget_InputTransformerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_transformer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kinesis_target` after provisioning.\n"]
    pub fn kinesis_target(&self) -> terrars::ListRef<CloudwatchEventTarget_KinesisTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redshift_target` after provisioning.\n"]
    pub fn redshift_target(&self) -> terrars::ListRef<CloudwatchEventTarget_RedshiftTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<CloudwatchEventTarget_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `run_command_targets` after provisioning.\n"]
    pub fn run_command_targets(&self) -> terrars::ListRef<CloudwatchEventTarget_RunCommandTargetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.run_command_targets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sagemaker_pipeline_target` after provisioning.\n"]
    pub fn sagemaker_pipeline_target(&self) -> terrars::ListRef<CloudwatchEventTarget_SagemakerPipelineTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sagemaker_pipeline_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sqs_target` after provisioning.\n"]
    pub fn sqs_target(&self) -> terrars::ListRef<CloudwatchEventTarget_SqsTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sqs_target", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchEventTargetDynamic {
    appsync_target: std::option::Option<terrars::DynamicBlock<CloudwatchEventTarget_AppsyncTarget>>,
    batch_target: std::option::Option<terrars::DynamicBlock<CloudwatchEventTarget_BatchTarget>>,
    dead_letter_config: std::option::Option<terrars::DynamicBlock<CloudwatchEventTarget_DeadLetterConfig>>,
    ecs_target: std::option::Option<terrars::DynamicBlock<CloudwatchEventTarget_EcsTarget>>,
    http_target: std::option::Option<terrars::DynamicBlock<CloudwatchEventTarget_HttpTarget>>,
    input_transformer: std::option::Option<terrars::DynamicBlock<CloudwatchEventTarget_InputTransformer>>,
    kinesis_target: std::option::Option<terrars::DynamicBlock<CloudwatchEventTarget_KinesisTarget>>,
    redshift_target: std::option::Option<terrars::DynamicBlock<CloudwatchEventTarget_RedshiftTarget>>,
    retry_policy: std::option::Option<terrars::DynamicBlock<CloudwatchEventTarget_RetryPolicy>>,
    run_command_targets: std::option::Option<terrars::DynamicBlock<CloudwatchEventTarget_RunCommandTargets>>,
    sagemaker_pipeline_target: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventTarget_SagemakerPipelineTarget>>,
    sqs_target: std::option::Option<terrars::DynamicBlock<CloudwatchEventTarget_SqsTarget>>,
}

#[derive(Serialize)]
pub struct CloudwatchEventTarget_AppsyncTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    graphql_operation: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventTarget_AppsyncTarget {
    #[doc = "Set the field `graphql_operation`.\n"]
    pub fn set_graphql_operation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.graphql_operation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventTarget_AppsyncTarget {
    type O = terrars::BlockAssignable<CloudwatchEventTarget_AppsyncTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTarget_AppsyncTarget {}

impl BuildCloudwatchEventTarget_AppsyncTarget {
    pub fn build(self) -> CloudwatchEventTarget_AppsyncTarget {
        CloudwatchEventTarget_AppsyncTarget { graphql_operation: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventTarget_AppsyncTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTarget_AppsyncTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventTarget_AppsyncTargetRef {
        CloudwatchEventTarget_AppsyncTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTarget_AppsyncTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `graphql_operation` after provisioning.\n"]
    pub fn graphql_operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.graphql_operation", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTarget_BatchTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    array_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_attempts: std::option::Option<terrars::PrimField<f64>>,
    job_definition: terrars::PrimField<std::string::String>,
    job_name: terrars::PrimField<std::string::String>,
}

impl CloudwatchEventTarget_BatchTarget {
    #[doc = "Set the field `array_size`.\n"]
    pub fn set_array_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.array_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `job_attempts`.\n"]
    pub fn set_job_attempts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.job_attempts = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventTarget_BatchTarget {
    type O = terrars::BlockAssignable<CloudwatchEventTarget_BatchTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTarget_BatchTarget {
    #[doc = ""]
    pub job_definition: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub job_name: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventTarget_BatchTarget {
    pub fn build(self) -> CloudwatchEventTarget_BatchTarget {
        CloudwatchEventTarget_BatchTarget {
            array_size: core::default::Default::default(),
            job_attempts: core::default::Default::default(),
            job_definition: self.job_definition,
            job_name: self.job_name,
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventTarget_BatchTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTarget_BatchTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventTarget_BatchTargetRef {
        CloudwatchEventTarget_BatchTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTarget_BatchTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `array_size` after provisioning.\n"]
    pub fn array_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.array_size", self.base))
    }

    #[doc = "Get a reference to the value of field `job_attempts` after provisioning.\n"]
    pub fn job_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_attempts", self.base))
    }

    #[doc = "Get a reference to the value of field `job_definition` after provisioning.\n"]
    pub fn job_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_definition", self.base))
    }

    #[doc = "Get a reference to the value of field `job_name` after provisioning.\n"]
    pub fn job_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_name", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTarget_DeadLetterConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventTarget_DeadLetterConfig {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventTarget_DeadLetterConfig {
    type O = terrars::BlockAssignable<CloudwatchEventTarget_DeadLetterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTarget_DeadLetterConfig {}

impl BuildCloudwatchEventTarget_DeadLetterConfig {
    pub fn build(self) -> CloudwatchEventTarget_DeadLetterConfig {
        CloudwatchEventTarget_DeadLetterConfig { arn: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventTarget_DeadLetterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTarget_DeadLetterConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventTarget_DeadLetterConfigRef {
        CloudwatchEventTarget_DeadLetterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTarget_DeadLetterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTarget_EcsTarget_CapacityProviderStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base: std::option::Option<terrars::PrimField<f64>>,
    capacity_provider: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl CloudwatchEventTarget_EcsTarget_CapacityProviderStrategy {
    #[doc = "Set the field `base`.\n"]
    pub fn set_base(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.base = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventTarget_EcsTarget_CapacityProviderStrategy {
    type O = terrars::BlockAssignable<CloudwatchEventTarget_EcsTarget_CapacityProviderStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTarget_EcsTarget_CapacityProviderStrategy {
    #[doc = ""]
    pub capacity_provider: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventTarget_EcsTarget_CapacityProviderStrategy {
    pub fn build(self) -> CloudwatchEventTarget_EcsTarget_CapacityProviderStrategy {
        CloudwatchEventTarget_EcsTarget_CapacityProviderStrategy {
            base: core::default::Default::default(),
            capacity_provider: self.capacity_provider,
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventTarget_EcsTarget_CapacityProviderStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTarget_EcsTarget_CapacityProviderStrategyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventTarget_EcsTarget_CapacityProviderStrategyRef {
        CloudwatchEventTarget_EcsTarget_CapacityProviderStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTarget_EcsTarget_CapacityProviderStrategyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base` after provisioning.\n"]
    pub fn base(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_provider` after provisioning.\n"]
    pub fn capacity_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_provider", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTarget_EcsTarget_NetworkConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    assign_public_ip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventTarget_EcsTarget_NetworkConfiguration {
    #[doc = "Set the field `assign_public_ip`.\n"]
    pub fn set_assign_public_ip(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.assign_public_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_groups = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventTarget_EcsTarget_NetworkConfiguration {
    type O = terrars::BlockAssignable<CloudwatchEventTarget_EcsTarget_NetworkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTarget_EcsTarget_NetworkConfiguration {
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudwatchEventTarget_EcsTarget_NetworkConfiguration {
    pub fn build(self) -> CloudwatchEventTarget_EcsTarget_NetworkConfiguration {
        CloudwatchEventTarget_EcsTarget_NetworkConfiguration {
            assign_public_ip: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            subnets: self.subnets,
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventTarget_EcsTarget_NetworkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTarget_EcsTarget_NetworkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventTarget_EcsTarget_NetworkConfigurationRef {
        CloudwatchEventTarget_EcsTarget_NetworkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTarget_EcsTarget_NetworkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `assign_public_ip` after provisioning.\n"]
    pub fn assign_public_ip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.assign_public_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTarget_EcsTarget_OrderedPlacementStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl CloudwatchEventTarget_EcsTarget_OrderedPlacementStrategy {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventTarget_EcsTarget_OrderedPlacementStrategy {
    type O = terrars::BlockAssignable<CloudwatchEventTarget_EcsTarget_OrderedPlacementStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTarget_EcsTarget_OrderedPlacementStrategy {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventTarget_EcsTarget_OrderedPlacementStrategy {
    pub fn build(self) -> CloudwatchEventTarget_EcsTarget_OrderedPlacementStrategy {
        CloudwatchEventTarget_EcsTarget_OrderedPlacementStrategy {
            field: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventTarget_EcsTarget_OrderedPlacementStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTarget_EcsTarget_OrderedPlacementStrategyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventTarget_EcsTarget_OrderedPlacementStrategyRef {
        CloudwatchEventTarget_EcsTarget_OrderedPlacementStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTarget_EcsTarget_OrderedPlacementStrategyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTarget_EcsTarget_PlacementConstraint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl CloudwatchEventTarget_EcsTarget_PlacementConstraint {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventTarget_EcsTarget_PlacementConstraint {
    type O = terrars::BlockAssignable<CloudwatchEventTarget_EcsTarget_PlacementConstraint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTarget_EcsTarget_PlacementConstraint {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventTarget_EcsTarget_PlacementConstraint {
    pub fn build(self) -> CloudwatchEventTarget_EcsTarget_PlacementConstraint {
        CloudwatchEventTarget_EcsTarget_PlacementConstraint {
            expression: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventTarget_EcsTarget_PlacementConstraintRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTarget_EcsTarget_PlacementConstraintRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventTarget_EcsTarget_PlacementConstraintRef {
        CloudwatchEventTarget_EcsTarget_PlacementConstraintRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTarget_EcsTarget_PlacementConstraintRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchEventTarget_EcsTargetDynamic {
    capacity_provider_strategy: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventTarget_EcsTarget_CapacityProviderStrategy>>,
    network_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventTarget_EcsTarget_NetworkConfiguration>>,
    ordered_placement_strategy: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventTarget_EcsTarget_OrderedPlacementStrategy>>,
    placement_constraint: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventTarget_EcsTarget_PlacementConstraint>>,
}

#[derive(Serialize)]
pub struct CloudwatchEventTarget_EcsTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_ecs_managed_tags: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_execute_command: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagate_tags: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_count: std::option::Option<terrars::PrimField<f64>>,
    task_definition_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_provider_strategy: std
    ::option
    ::Option<std::vec::Vec<CloudwatchEventTarget_EcsTarget_CapacityProviderStrategy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std::option::Option<std::vec::Vec<CloudwatchEventTarget_EcsTarget_NetworkConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ordered_placement_strategy: std
    ::option
    ::Option<std::vec::Vec<CloudwatchEventTarget_EcsTarget_OrderedPlacementStrategy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_constraint: std::option::Option<std::vec::Vec<CloudwatchEventTarget_EcsTarget_PlacementConstraint>>,
    dynamic: CloudwatchEventTarget_EcsTargetDynamic,
}

impl CloudwatchEventTarget_EcsTarget {
    #[doc = "Set the field `enable_ecs_managed_tags`.\n"]
    pub fn set_enable_ecs_managed_tags(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_ecs_managed_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_execute_command`.\n"]
    pub fn set_enable_execute_command(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_execute_command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_type`.\n"]
    pub fn set_launch_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.launch_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `platform_version`.\n"]
    pub fn set_platform_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.platform_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `propagate_tags`.\n"]
    pub fn set_propagate_tags(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.propagate_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `task_count`.\n"]
    pub fn set_task_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.task_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_provider_strategy`.\n"]
    pub fn set_capacity_provider_strategy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchEventTarget_EcsTarget_CapacityProviderStrategy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.capacity_provider_strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.capacity_provider_strategy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTarget_EcsTarget_NetworkConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ordered_placement_strategy`.\n"]
    pub fn set_ordered_placement_strategy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchEventTarget_EcsTarget_OrderedPlacementStrategy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ordered_placement_strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ordered_placement_strategy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placement_constraint`.\n"]
    pub fn set_placement_constraint(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTarget_EcsTarget_PlacementConstraint>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.placement_constraint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.placement_constraint = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventTarget_EcsTarget {
    type O = terrars::BlockAssignable<CloudwatchEventTarget_EcsTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTarget_EcsTarget {
    #[doc = ""]
    pub task_definition_arn: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventTarget_EcsTarget {
    pub fn build(self) -> CloudwatchEventTarget_EcsTarget {
        CloudwatchEventTarget_EcsTarget {
            enable_ecs_managed_tags: core::default::Default::default(),
            enable_execute_command: core::default::Default::default(),
            group: core::default::Default::default(),
            launch_type: core::default::Default::default(),
            platform_version: core::default::Default::default(),
            propagate_tags: core::default::Default::default(),
            tags: core::default::Default::default(),
            task_count: core::default::Default::default(),
            task_definition_arn: self.task_definition_arn,
            capacity_provider_strategy: core::default::Default::default(),
            network_configuration: core::default::Default::default(),
            ordered_placement_strategy: core::default::Default::default(),
            placement_constraint: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventTarget_EcsTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTarget_EcsTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventTarget_EcsTargetRef {
        CloudwatchEventTarget_EcsTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTarget_EcsTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_ecs_managed_tags` after provisioning.\n"]
    pub fn enable_ecs_managed_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_ecs_managed_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_execute_command` after provisioning.\n"]
    pub fn enable_execute_command(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_execute_command", self.base))
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_type` after provisioning.\n"]
    pub fn launch_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_type", self.base))
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.base))
    }

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `task_count` after provisioning.\n"]
    pub fn task_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_count", self.base))
    }

    #[doc = "Get a reference to the value of field `task_definition_arn` after provisioning.\n"]
    pub fn task_definition_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_definition_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<CloudwatchEventTarget_EcsTarget_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `ordered_placement_strategy` after provisioning.\n"]
    pub fn ordered_placement_strategy(
        &self,
    ) -> terrars::ListRef<CloudwatchEventTarget_EcsTarget_OrderedPlacementStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ordered_placement_strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTarget_HttpTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_parameter_values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl CloudwatchEventTarget_HttpTarget {
    #[doc = "Set the field `header_parameters`.\n"]
    pub fn set_header_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.header_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path_parameter_values`.\n"]
    pub fn set_path_parameter_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.path_parameter_values = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_string_parameters`.\n"]
    pub fn set_query_string_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.query_string_parameters = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventTarget_HttpTarget {
    type O = terrars::BlockAssignable<CloudwatchEventTarget_HttpTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTarget_HttpTarget {}

impl BuildCloudwatchEventTarget_HttpTarget {
    pub fn build(self) -> CloudwatchEventTarget_HttpTarget {
        CloudwatchEventTarget_HttpTarget {
            header_parameters: core::default::Default::default(),
            path_parameter_values: core::default::Default::default(),
            query_string_parameters: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventTarget_HttpTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTarget_HttpTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventTarget_HttpTargetRef {
        CloudwatchEventTarget_HttpTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTarget_HttpTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_parameters` after provisioning.\n"]
    pub fn header_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.header_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `path_parameter_values` after provisioning.\n"]
    pub fn path_parameter_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path_parameter_values", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string_parameters` after provisioning.\n"]
    pub fn query_string_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.query_string_parameters", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTarget_InputTransformer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_paths: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    input_template: terrars::PrimField<std::string::String>,
}

impl CloudwatchEventTarget_InputTransformer {
    #[doc = "Set the field `input_paths`.\n"]
    pub fn set_input_paths(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.input_paths = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventTarget_InputTransformer {
    type O = terrars::BlockAssignable<CloudwatchEventTarget_InputTransformer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTarget_InputTransformer {
    #[doc = ""]
    pub input_template: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventTarget_InputTransformer {
    pub fn build(self) -> CloudwatchEventTarget_InputTransformer {
        CloudwatchEventTarget_InputTransformer {
            input_paths: core::default::Default::default(),
            input_template: self.input_template,
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventTarget_InputTransformerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTarget_InputTransformerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventTarget_InputTransformerRef {
        CloudwatchEventTarget_InputTransformerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTarget_InputTransformerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_paths` after provisioning.\n"]
    pub fn input_paths(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.input_paths", self.base))
    }

    #[doc = "Get a reference to the value of field `input_template` after provisioning.\n"]
    pub fn input_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_template", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTarget_KinesisTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_key_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventTarget_KinesisTarget {
    #[doc = "Set the field `partition_key_path`.\n"]
    pub fn set_partition_key_path(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.partition_key_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventTarget_KinesisTarget {
    type O = terrars::BlockAssignable<CloudwatchEventTarget_KinesisTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTarget_KinesisTarget {}

impl BuildCloudwatchEventTarget_KinesisTarget {
    pub fn build(self) -> CloudwatchEventTarget_KinesisTarget {
        CloudwatchEventTarget_KinesisTarget { partition_key_path: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventTarget_KinesisTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTarget_KinesisTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventTarget_KinesisTargetRef {
        CloudwatchEventTarget_KinesisTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTarget_KinesisTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `partition_key_path` after provisioning.\n"]
    pub fn partition_key_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partition_key_path", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTarget_RedshiftTarget {
    database: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_user: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_manager_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statement_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    with_event: std::option::Option<terrars::PrimField<bool>>,
}

impl CloudwatchEventTarget_RedshiftTarget {
    #[doc = "Set the field `db_user`.\n"]
    pub fn set_db_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.db_user = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secrets_manager_arn`.\n"]
    pub fn set_secrets_manager_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secrets_manager_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sql`.\n"]
    pub fn set_sql(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sql = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `statement_name`.\n"]
    pub fn set_statement_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.statement_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `with_event`.\n"]
    pub fn set_with_event(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.with_event = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventTarget_RedshiftTarget {
    type O = terrars::BlockAssignable<CloudwatchEventTarget_RedshiftTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTarget_RedshiftTarget {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventTarget_RedshiftTarget {
    pub fn build(self) -> CloudwatchEventTarget_RedshiftTarget {
        CloudwatchEventTarget_RedshiftTarget {
            database: self.database,
            db_user: core::default::Default::default(),
            secrets_manager_arn: core::default::Default::default(),
            sql: core::default::Default::default(),
            statement_name: core::default::Default::default(),
            with_event: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventTarget_RedshiftTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTarget_RedshiftTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventTarget_RedshiftTargetRef {
        CloudwatchEventTarget_RedshiftTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTarget_RedshiftTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `db_user` after provisioning.\n"]
    pub fn db_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_user", self.base))
    }

    #[doc = "Get a reference to the value of field `secrets_manager_arn` after provisioning.\n"]
    pub fn secrets_manager_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secrets_manager_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sql` after provisioning.\n"]
    pub fn sql(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql", self.base))
    }

    #[doc = "Get a reference to the value of field `statement_name` after provisioning.\n"]
    pub fn statement_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statement_name", self.base))
    }

    #[doc = "Get a reference to the value of field `with_event` after provisioning.\n"]
    pub fn with_event(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.with_event", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTarget_RetryPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_event_age_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_retry_attempts: std::option::Option<terrars::PrimField<f64>>,
}

impl CloudwatchEventTarget_RetryPolicy {
    #[doc = "Set the field `maximum_event_age_in_seconds`.\n"]
    pub fn set_maximum_event_age_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_event_age_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_retry_attempts`.\n"]
    pub fn set_maximum_retry_attempts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_retry_attempts = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventTarget_RetryPolicy {
    type O = terrars::BlockAssignable<CloudwatchEventTarget_RetryPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTarget_RetryPolicy {}

impl BuildCloudwatchEventTarget_RetryPolicy {
    pub fn build(self) -> CloudwatchEventTarget_RetryPolicy {
        CloudwatchEventTarget_RetryPolicy {
            maximum_event_age_in_seconds: core::default::Default::default(),
            maximum_retry_attempts: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventTarget_RetryPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTarget_RetryPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventTarget_RetryPolicyRef {
        CloudwatchEventTarget_RetryPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTarget_RetryPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_event_age_in_seconds` after provisioning.\n"]
    pub fn maximum_event_age_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_event_age_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_retry_attempts` after provisioning.\n"]
    pub fn maximum_retry_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_retry_attempts", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTarget_RunCommandTargets {
    key: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventTarget_RunCommandTargets { }

impl terrars::ToListMappable for CloudwatchEventTarget_RunCommandTargets {
    type O = terrars::BlockAssignable<CloudwatchEventTarget_RunCommandTargets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTarget_RunCommandTargets {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudwatchEventTarget_RunCommandTargets {
    pub fn build(self) -> CloudwatchEventTarget_RunCommandTargets {
        CloudwatchEventTarget_RunCommandTargets {
            key: self.key,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventTarget_RunCommandTargetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTarget_RunCommandTargetsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventTarget_RunCommandTargetsRef {
        CloudwatchEventTarget_RunCommandTargetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTarget_RunCommandTargetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTarget_SagemakerPipelineTarget_PipelineParameterList {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl CloudwatchEventTarget_SagemakerPipelineTarget_PipelineParameterList { }

impl terrars::ToListMappable for CloudwatchEventTarget_SagemakerPipelineTarget_PipelineParameterList {
    type O = terrars::BlockAssignable<CloudwatchEventTarget_SagemakerPipelineTarget_PipelineParameterList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTarget_SagemakerPipelineTarget_PipelineParameterList {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventTarget_SagemakerPipelineTarget_PipelineParameterList {
    pub fn build(self) -> CloudwatchEventTarget_SagemakerPipelineTarget_PipelineParameterList {
        CloudwatchEventTarget_SagemakerPipelineTarget_PipelineParameterList {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventTarget_SagemakerPipelineTarget_PipelineParameterListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTarget_SagemakerPipelineTarget_PipelineParameterListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventTarget_SagemakerPipelineTarget_PipelineParameterListRef {
        CloudwatchEventTarget_SagemakerPipelineTarget_PipelineParameterListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTarget_SagemakerPipelineTarget_PipelineParameterListRef {
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

#[derive(Serialize, Default)]
struct CloudwatchEventTarget_SagemakerPipelineTargetDynamic {
    pipeline_parameter_list: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventTarget_SagemakerPipelineTarget_PipelineParameterList>>,
}

#[derive(Serialize)]
pub struct CloudwatchEventTarget_SagemakerPipelineTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_parameter_list: std
    ::option
    ::Option<std::vec::Vec<CloudwatchEventTarget_SagemakerPipelineTarget_PipelineParameterList>>,
    dynamic: CloudwatchEventTarget_SagemakerPipelineTargetDynamic,
}

impl CloudwatchEventTarget_SagemakerPipelineTarget {
    #[doc = "Set the field `pipeline_parameter_list`.\n"]
    pub fn set_pipeline_parameter_list(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CloudwatchEventTarget_SagemakerPipelineTarget_PipelineParameterList>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pipeline_parameter_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pipeline_parameter_list = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventTarget_SagemakerPipelineTarget {
    type O = terrars::BlockAssignable<CloudwatchEventTarget_SagemakerPipelineTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTarget_SagemakerPipelineTarget {}

impl BuildCloudwatchEventTarget_SagemakerPipelineTarget {
    pub fn build(self) -> CloudwatchEventTarget_SagemakerPipelineTarget {
        CloudwatchEventTarget_SagemakerPipelineTarget {
            pipeline_parameter_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventTarget_SagemakerPipelineTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTarget_SagemakerPipelineTargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventTarget_SagemakerPipelineTargetRef {
        CloudwatchEventTarget_SagemakerPipelineTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTarget_SagemakerPipelineTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTarget_SqsTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventTarget_SqsTarget {
    #[doc = "Set the field `message_group_id`.\n"]
    pub fn set_message_group_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message_group_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventTarget_SqsTarget {
    type O = terrars::BlockAssignable<CloudwatchEventTarget_SqsTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTarget_SqsTarget {}

impl BuildCloudwatchEventTarget_SqsTarget {
    pub fn build(self) -> CloudwatchEventTarget_SqsTarget {
        CloudwatchEventTarget_SqsTarget { message_group_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventTarget_SqsTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTarget_SqsTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventTarget_SqsTargetRef {
        CloudwatchEventTarget_SqsTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTarget_SqsTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message_group_id` after provisioning.\n"]
    pub fn message_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_group_id", self.base))
    }
}
