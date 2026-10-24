use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudwatchEventTargetData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    rule: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_target: std::option::Option<std::vec::Vec<CloudwatchEventTargetBatchTargetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dead_letter_config: std::option::Option<std::vec::Vec<CloudwatchEventTargetDeadLetterConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecs_target: std::option::Option<std::vec::Vec<CloudwatchEventTargetEcsTargetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_target: std::option::Option<std::vec::Vec<CloudwatchEventTargetHttpTargetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_transformer: std::option::Option<std::vec::Vec<CloudwatchEventTargetInputTransformerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_target: std::option::Option<std::vec::Vec<CloudwatchEventTargetKinesisTargetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redshift_target: std::option::Option<std::vec::Vec<CloudwatchEventTargetRedshiftTargetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<CloudwatchEventTargetRetryPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_command_targets: std::option::Option<std::vec::Vec<CloudwatchEventTargetRunCommandTargetsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sqs_target: std::option::Option<std::vec::Vec<CloudwatchEventTargetSqsTargetEl>>,
    dynamic: CloudwatchEventTargetDynamic,
}

struct CloudwatchEventTarget_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudwatchEventTargetData>,
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

    #[doc = "Set the field `batch_target`.\n"]
    pub fn set_batch_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTargetBatchTargetEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTargetDeadLetterConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTargetEcsTargetEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTargetHttpTargetEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTargetInputTransformerEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTargetKinesisTargetEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTargetRedshiftTargetEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTargetRetryPolicyEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTargetRunCommandTargetsEl>>,
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

    #[doc = "Set the field `sqs_target`.\n"]
    pub fn set_sqs_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTargetSqsTargetEl>>,
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

    #[doc = "Get a reference to the value of field `batch_target` after provisioning.\n"]
    pub fn batch_target(&self) -> terrars::ListRef<CloudwatchEventTargetBatchTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.batch_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> terrars::ListRef<CloudwatchEventTargetDeadLetterConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ecs_target` after provisioning.\n"]
    pub fn ecs_target(&self) -> terrars::ListRef<CloudwatchEventTargetEcsTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ecs_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_target` after provisioning.\n"]
    pub fn http_target(&self) -> terrars::ListRef<CloudwatchEventTargetHttpTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_transformer` after provisioning.\n"]
    pub fn input_transformer(&self) -> terrars::ListRef<CloudwatchEventTargetInputTransformerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_transformer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kinesis_target` after provisioning.\n"]
    pub fn kinesis_target(&self) -> terrars::ListRef<CloudwatchEventTargetKinesisTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redshift_target` after provisioning.\n"]
    pub fn redshift_target(&self) -> terrars::ListRef<CloudwatchEventTargetRedshiftTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<CloudwatchEventTargetRetryPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `run_command_targets` after provisioning.\n"]
    pub fn run_command_targets(&self) -> terrars::ListRef<CloudwatchEventTargetRunCommandTargetsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.run_command_targets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sqs_target` after provisioning.\n"]
    pub fn sqs_target(&self) -> terrars::ListRef<CloudwatchEventTargetSqsTargetElRef> {
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
            data: std::cell::RefCell::new(CloudwatchEventTargetData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                arn: self.arn,
                event_bus_name: core::default::Default::default(),
                id: core::default::Default::default(),
                input: core::default::Default::default(),
                input_path: core::default::Default::default(),
                role_arn: core::default::Default::default(),
                rule: self.rule,
                target_id: core::default::Default::default(),
                batch_target: core::default::Default::default(),
                dead_letter_config: core::default::Default::default(),
                ecs_target: core::default::Default::default(),
                http_target: core::default::Default::default(),
                input_transformer: core::default::Default::default(),
                kinesis_target: core::default::Default::default(),
                redshift_target: core::default::Default::default(),
                retry_policy: core::default::Default::default(),
                run_command_targets: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `batch_target` after provisioning.\n"]
    pub fn batch_target(&self) -> terrars::ListRef<CloudwatchEventTargetBatchTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.batch_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> terrars::ListRef<CloudwatchEventTargetDeadLetterConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ecs_target` after provisioning.\n"]
    pub fn ecs_target(&self) -> terrars::ListRef<CloudwatchEventTargetEcsTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ecs_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_target` after provisioning.\n"]
    pub fn http_target(&self) -> terrars::ListRef<CloudwatchEventTargetHttpTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_transformer` after provisioning.\n"]
    pub fn input_transformer(&self) -> terrars::ListRef<CloudwatchEventTargetInputTransformerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_transformer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kinesis_target` after provisioning.\n"]
    pub fn kinesis_target(&self) -> terrars::ListRef<CloudwatchEventTargetKinesisTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redshift_target` after provisioning.\n"]
    pub fn redshift_target(&self) -> terrars::ListRef<CloudwatchEventTargetRedshiftTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<CloudwatchEventTargetRetryPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `run_command_targets` after provisioning.\n"]
    pub fn run_command_targets(&self) -> terrars::ListRef<CloudwatchEventTargetRunCommandTargetsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.run_command_targets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sqs_target` after provisioning.\n"]
    pub fn sqs_target(&self) -> terrars::ListRef<CloudwatchEventTargetSqsTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sqs_target", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetBatchTargetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    array_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_attempts: std::option::Option<terrars::PrimField<f64>>,
    job_definition: terrars::PrimField<std::string::String>,
    job_name: terrars::PrimField<std::string::String>,
}

impl CloudwatchEventTargetBatchTargetEl {
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

impl terrars::ToListMappable for CloudwatchEventTargetBatchTargetEl {
    type O = terrars::BlockAssignable<CloudwatchEventTargetBatchTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetBatchTargetEl {
    #[doc = ""]
    pub job_definition: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub job_name: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventTargetBatchTargetEl {
    pub fn build(self) -> CloudwatchEventTargetBatchTargetEl {
        CloudwatchEventTargetBatchTargetEl {
            array_size: core::default::Default::default(),
            job_attempts: core::default::Default::default(),
            job_definition: self.job_definition,
            job_name: self.job_name,
        }
    }
}

pub struct CloudwatchEventTargetBatchTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTargetBatchTargetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventTargetBatchTargetElRef {
        CloudwatchEventTargetBatchTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetBatchTargetElRef {
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
pub struct CloudwatchEventTargetDeadLetterConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventTargetDeadLetterConfigEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventTargetDeadLetterConfigEl {
    type O = terrars::BlockAssignable<CloudwatchEventTargetDeadLetterConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetDeadLetterConfigEl {}

impl BuildCloudwatchEventTargetDeadLetterConfigEl {
    pub fn build(self) -> CloudwatchEventTargetDeadLetterConfigEl {
        CloudwatchEventTargetDeadLetterConfigEl { arn: core::default::Default::default() }
    }
}

pub struct CloudwatchEventTargetDeadLetterConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTargetDeadLetterConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventTargetDeadLetterConfigElRef {
        CloudwatchEventTargetDeadLetterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetDeadLetterConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base: std::option::Option<terrars::PrimField<f64>>,
    capacity_provider: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl CloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl {
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

impl terrars::ToListMappable for CloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl {
    type O = terrars::BlockAssignable<CloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl {
    #[doc = ""]
    pub capacity_provider: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl {
    pub fn build(self) -> CloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl {
        CloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl {
            base: core::default::Default::default(),
            capacity_provider: self.capacity_provider,
            weight: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventTargetEcsTargetElCapacityProviderStrategyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTargetEcsTargetElCapacityProviderStrategyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventTargetEcsTargetElCapacityProviderStrategyElRef {
        CloudwatchEventTargetEcsTargetElCapacityProviderStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetEcsTargetElCapacityProviderStrategyElRef {
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
pub struct CloudwatchEventTargetEcsTargetElNetworkConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    assign_public_ip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventTargetEcsTargetElNetworkConfigurationEl {
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

impl terrars::ToListMappable for CloudwatchEventTargetEcsTargetElNetworkConfigurationEl {
    type O = terrars::BlockAssignable<CloudwatchEventTargetEcsTargetElNetworkConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetEcsTargetElNetworkConfigurationEl {
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudwatchEventTargetEcsTargetElNetworkConfigurationEl {
    pub fn build(self) -> CloudwatchEventTargetEcsTargetElNetworkConfigurationEl {
        CloudwatchEventTargetEcsTargetElNetworkConfigurationEl {
            assign_public_ip: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            subnets: self.subnets,
        }
    }
}

pub struct CloudwatchEventTargetEcsTargetElNetworkConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTargetEcsTargetElNetworkConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventTargetEcsTargetElNetworkConfigurationElRef {
        CloudwatchEventTargetEcsTargetElNetworkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetEcsTargetElNetworkConfigurationElRef {
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
pub struct CloudwatchEventTargetEcsTargetElPlacementConstraintEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl CloudwatchEventTargetEcsTargetElPlacementConstraintEl {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventTargetEcsTargetElPlacementConstraintEl {
    type O = terrars::BlockAssignable<CloudwatchEventTargetEcsTargetElPlacementConstraintEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetEcsTargetElPlacementConstraintEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventTargetEcsTargetElPlacementConstraintEl {
    pub fn build(self) -> CloudwatchEventTargetEcsTargetElPlacementConstraintEl {
        CloudwatchEventTargetEcsTargetElPlacementConstraintEl {
            expression: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct CloudwatchEventTargetEcsTargetElPlacementConstraintElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTargetEcsTargetElPlacementConstraintElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventTargetEcsTargetElPlacementConstraintElRef {
        CloudwatchEventTargetEcsTargetElPlacementConstraintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetEcsTargetElPlacementConstraintElRef {
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
struct CloudwatchEventTargetEcsTargetElDynamic {
    capacity_provider_strategy: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl>>,
    network_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventTargetEcsTargetElNetworkConfigurationEl>>,
    placement_constraint: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventTargetEcsTargetElPlacementConstraintEl>>,
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetEcsTargetEl {
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
    ::Option<std::vec::Vec<CloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std::option::Option<std::vec::Vec<CloudwatchEventTargetEcsTargetElNetworkConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_constraint: std::option::Option<std::vec::Vec<CloudwatchEventTargetEcsTargetElPlacementConstraintEl>>,
    dynamic: CloudwatchEventTargetEcsTargetElDynamic,
}

impl CloudwatchEventTargetEcsTargetEl {
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
                    ::Into<terrars::BlockAssignable<CloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTargetEcsTargetElNetworkConfigurationEl>>,
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

    #[doc = "Set the field `placement_constraint`.\n"]
    pub fn set_placement_constraint(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventTargetEcsTargetElPlacementConstraintEl>>,
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

impl terrars::ToListMappable for CloudwatchEventTargetEcsTargetEl {
    type O = terrars::BlockAssignable<CloudwatchEventTargetEcsTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetEcsTargetEl {
    #[doc = ""]
    pub task_definition_arn: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventTargetEcsTargetEl {
    pub fn build(self) -> CloudwatchEventTargetEcsTargetEl {
        CloudwatchEventTargetEcsTargetEl {
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
            placement_constraint: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventTargetEcsTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTargetEcsTargetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventTargetEcsTargetElRef {
        CloudwatchEventTargetEcsTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetEcsTargetElRef {
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
    pub fn network_configuration(&self) -> terrars::ListRef<CloudwatchEventTargetEcsTargetElNetworkConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetHttpTargetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_parameter_values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl CloudwatchEventTargetHttpTargetEl {
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

impl terrars::ToListMappable for CloudwatchEventTargetHttpTargetEl {
    type O = terrars::BlockAssignable<CloudwatchEventTargetHttpTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetHttpTargetEl {}

impl BuildCloudwatchEventTargetHttpTargetEl {
    pub fn build(self) -> CloudwatchEventTargetHttpTargetEl {
        CloudwatchEventTargetHttpTargetEl {
            header_parameters: core::default::Default::default(),
            path_parameter_values: core::default::Default::default(),
            query_string_parameters: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventTargetHttpTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTargetHttpTargetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventTargetHttpTargetElRef {
        CloudwatchEventTargetHttpTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetHttpTargetElRef {
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
pub struct CloudwatchEventTargetInputTransformerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_paths: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    input_template: terrars::PrimField<std::string::String>,
}

impl CloudwatchEventTargetInputTransformerEl {
    #[doc = "Set the field `input_paths`.\n"]
    pub fn set_input_paths(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.input_paths = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventTargetInputTransformerEl {
    type O = terrars::BlockAssignable<CloudwatchEventTargetInputTransformerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetInputTransformerEl {
    #[doc = ""]
    pub input_template: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventTargetInputTransformerEl {
    pub fn build(self) -> CloudwatchEventTargetInputTransformerEl {
        CloudwatchEventTargetInputTransformerEl {
            input_paths: core::default::Default::default(),
            input_template: self.input_template,
        }
    }
}

pub struct CloudwatchEventTargetInputTransformerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTargetInputTransformerElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventTargetInputTransformerElRef {
        CloudwatchEventTargetInputTransformerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetInputTransformerElRef {
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
pub struct CloudwatchEventTargetKinesisTargetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_key_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventTargetKinesisTargetEl {
    #[doc = "Set the field `partition_key_path`.\n"]
    pub fn set_partition_key_path(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.partition_key_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventTargetKinesisTargetEl {
    type O = terrars::BlockAssignable<CloudwatchEventTargetKinesisTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetKinesisTargetEl {}

impl BuildCloudwatchEventTargetKinesisTargetEl {
    pub fn build(self) -> CloudwatchEventTargetKinesisTargetEl {
        CloudwatchEventTargetKinesisTargetEl { partition_key_path: core::default::Default::default() }
    }
}

pub struct CloudwatchEventTargetKinesisTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTargetKinesisTargetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventTargetKinesisTargetElRef {
        CloudwatchEventTargetKinesisTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetKinesisTargetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `partition_key_path` after provisioning.\n"]
    pub fn partition_key_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partition_key_path", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetRedshiftTargetEl {
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

impl CloudwatchEventTargetRedshiftTargetEl {
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

impl terrars::ToListMappable for CloudwatchEventTargetRedshiftTargetEl {
    type O = terrars::BlockAssignable<CloudwatchEventTargetRedshiftTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetRedshiftTargetEl {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventTargetRedshiftTargetEl {
    pub fn build(self) -> CloudwatchEventTargetRedshiftTargetEl {
        CloudwatchEventTargetRedshiftTargetEl {
            database: self.database,
            db_user: core::default::Default::default(),
            secrets_manager_arn: core::default::Default::default(),
            sql: core::default::Default::default(),
            statement_name: core::default::Default::default(),
            with_event: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventTargetRedshiftTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTargetRedshiftTargetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventTargetRedshiftTargetElRef {
        CloudwatchEventTargetRedshiftTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetRedshiftTargetElRef {
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
pub struct CloudwatchEventTargetRetryPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_event_age_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_retry_attempts: std::option::Option<terrars::PrimField<f64>>,
}

impl CloudwatchEventTargetRetryPolicyEl {
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

impl terrars::ToListMappable for CloudwatchEventTargetRetryPolicyEl {
    type O = terrars::BlockAssignable<CloudwatchEventTargetRetryPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetRetryPolicyEl {}

impl BuildCloudwatchEventTargetRetryPolicyEl {
    pub fn build(self) -> CloudwatchEventTargetRetryPolicyEl {
        CloudwatchEventTargetRetryPolicyEl {
            maximum_event_age_in_seconds: core::default::Default::default(),
            maximum_retry_attempts: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventTargetRetryPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTargetRetryPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventTargetRetryPolicyElRef {
        CloudwatchEventTargetRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetRetryPolicyElRef {
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
pub struct CloudwatchEventTargetRunCommandTargetsEl {
    key: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventTargetRunCommandTargetsEl { }

impl terrars::ToListMappable for CloudwatchEventTargetRunCommandTargetsEl {
    type O = terrars::BlockAssignable<CloudwatchEventTargetRunCommandTargetsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetRunCommandTargetsEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudwatchEventTargetRunCommandTargetsEl {
    pub fn build(self) -> CloudwatchEventTargetRunCommandTargetsEl {
        CloudwatchEventTargetRunCommandTargetsEl {
            key: self.key,
            values: self.values,
        }
    }
}

pub struct CloudwatchEventTargetRunCommandTargetsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTargetRunCommandTargetsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventTargetRunCommandTargetsElRef {
        CloudwatchEventTargetRunCommandTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetRunCommandTargetsElRef {
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
pub struct CloudwatchEventTargetSqsTargetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventTargetSqsTargetEl {
    #[doc = "Set the field `message_group_id`.\n"]
    pub fn set_message_group_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message_group_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventTargetSqsTargetEl {
    type O = terrars::BlockAssignable<CloudwatchEventTargetSqsTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetSqsTargetEl {}

impl BuildCloudwatchEventTargetSqsTargetEl {
    pub fn build(self) -> CloudwatchEventTargetSqsTargetEl {
        CloudwatchEventTargetSqsTargetEl { message_group_id: core::default::Default::default() }
    }
}

pub struct CloudwatchEventTargetSqsTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventTargetSqsTargetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventTargetSqsTargetElRef {
        CloudwatchEventTargetSqsTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetSqsTargetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message_group_id` after provisioning.\n"]
    pub fn message_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_group_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchEventTargetDynamic {
    batch_target: std::option::Option<terrars::DynamicBlock<CloudwatchEventTargetBatchTargetEl>>,
    dead_letter_config: std::option::Option<terrars::DynamicBlock<CloudwatchEventTargetDeadLetterConfigEl>>,
    ecs_target: std::option::Option<terrars::DynamicBlock<CloudwatchEventTargetEcsTargetEl>>,
    http_target: std::option::Option<terrars::DynamicBlock<CloudwatchEventTargetHttpTargetEl>>,
    input_transformer: std::option::Option<terrars::DynamicBlock<CloudwatchEventTargetInputTransformerEl>>,
    kinesis_target: std::option::Option<terrars::DynamicBlock<CloudwatchEventTargetKinesisTargetEl>>,
    redshift_target: std::option::Option<terrars::DynamicBlock<CloudwatchEventTargetRedshiftTargetEl>>,
    retry_policy: std::option::Option<terrars::DynamicBlock<CloudwatchEventTargetRetryPolicyEl>>,
    run_command_targets: std::option::Option<terrars::DynamicBlock<CloudwatchEventTargetRunCommandTargetsEl>>,
    sqs_target: std::option::Option<terrars::DynamicBlock<CloudwatchEventTargetSqsTargetEl>>,
}
