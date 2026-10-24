use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BatchJobDefinitionData {
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
    container_properties: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform_capabilities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagate_tags: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_strategy: std::option::Option<std::vec::Vec<BatchJobDefinitionRetryStrategyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<BatchJobDefinitionTimeoutEl>>,
    dynamic: BatchJobDefinitionDynamic,
}

struct BatchJobDefinition_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BatchJobDefinitionData>,
}

#[derive(Clone)]
pub struct BatchJobDefinition(std::rc::Rc<BatchJobDefinition_>);

impl BatchJobDefinition {
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

    #[doc = "Set the field `container_properties`.\n"]
    pub fn set_container_properties(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().container_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `platform_capabilities`.\n"]
    pub fn set_platform_capabilities(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().platform_capabilities = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `propagate_tags`.\n"]
    pub fn set_propagate_tags(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().propagate_tags = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `retry_strategy`.\n"]
    pub fn set_retry_strategy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BatchJobDefinitionRetryStrategyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retry_strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retry_strategy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(self, v: impl std::convert::Into<terrars::BlockAssignable<BatchJobDefinitionTimeoutEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().timeout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.timeout = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_properties` after provisioning.\n"]
    pub fn container_properties(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_capabilities` after provisioning.\n"]
    pub fn platform_capabilities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.platform_capabilities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_strategy` after provisioning.\n"]
    pub fn retry_strategy(&self) -> terrars::ListRef<BatchJobDefinitionRetryStrategyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<BatchJobDefinitionTimeoutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }
}

impl terrars::Referable for BatchJobDefinition {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BatchJobDefinition { }

impl terrars::ToListMappable for BatchJobDefinition {
    type O = terrars::ListRef<BatchJobDefinitionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BatchJobDefinition_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_batch_job_definition".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBatchJobDefinition {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBatchJobDefinition {
    pub fn build(self, stack: &mut terrars::Stack) -> BatchJobDefinition {
        let out = BatchJobDefinition(std::rc::Rc::new(BatchJobDefinition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BatchJobDefinitionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                container_properties: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                parameters: core::default::Default::default(),
                platform_capabilities: core::default::Default::default(),
                propagate_tags: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                retry_strategy: core::default::Default::default(),
                timeout: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BatchJobDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinitionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BatchJobDefinitionRef {
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

    #[doc = "Get a reference to the value of field `container_properties` after provisioning.\n"]
    pub fn container_properties(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_capabilities` after provisioning.\n"]
    pub fn platform_capabilities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.platform_capabilities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_strategy` after provisioning.\n"]
    pub fn retry_strategy(&self) -> terrars::ListRef<BatchJobDefinitionRetryStrategyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<BatchJobDefinitionTimeoutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_exit_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_status_reason: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
    #[doc = "Set the field `on_exit_code`.\n"]
    pub fn set_on_exit_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.on_exit_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_reason`.\n"]
    pub fn set_on_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.on_reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_status_reason`.\n"]
    pub fn set_on_status_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.on_status_reason = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
    type O = terrars::BlockAssignable<BatchJobDefinitionRetryStrategyElEvaluateOnExitEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
    #[doc = ""]
    pub action: terrars::PrimField<std::string::String>,
}

impl BuildBatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
    pub fn build(self) -> BatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
        BatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
            action: self.action,
            on_exit_code: core::default::Default::default(),
            on_reason: core::default::Default::default(),
            on_status_reason: core::default::Default::default(),
        }
    }
}

pub struct BatchJobDefinitionRetryStrategyElEvaluateOnExitElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinitionRetryStrategyElEvaluateOnExitElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BatchJobDefinitionRetryStrategyElEvaluateOnExitElRef {
        BatchJobDefinitionRetryStrategyElEvaluateOnExitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionRetryStrategyElEvaluateOnExitElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `on_exit_code` after provisioning.\n"]
    pub fn on_exit_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_exit_code", self.base))
    }

    #[doc = "Get a reference to the value of field `on_reason` after provisioning.\n"]
    pub fn on_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_reason", self.base))
    }

    #[doc = "Get a reference to the value of field `on_status_reason` after provisioning.\n"]
    pub fn on_status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_status_reason", self.base))
    }
}

#[derive(Serialize, Default)]
struct BatchJobDefinitionRetryStrategyElDynamic {
    evaluate_on_exit: std::option::Option<terrars::DynamicBlock<BatchJobDefinitionRetryStrategyElEvaluateOnExitEl>>,
}

#[derive(Serialize)]
pub struct BatchJobDefinitionRetryStrategyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attempts: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate_on_exit: std::option::Option<std::vec::Vec<BatchJobDefinitionRetryStrategyElEvaluateOnExitEl>>,
    dynamic: BatchJobDefinitionRetryStrategyElDynamic,
}

impl BatchJobDefinitionRetryStrategyEl {
    #[doc = "Set the field `attempts`.\n"]
    pub fn set_attempts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.attempts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `evaluate_on_exit`.\n"]
    pub fn set_evaluate_on_exit(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BatchJobDefinitionRetryStrategyElEvaluateOnExitEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.evaluate_on_exit = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.evaluate_on_exit = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BatchJobDefinitionRetryStrategyEl {
    type O = terrars::BlockAssignable<BatchJobDefinitionRetryStrategyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionRetryStrategyEl {}

impl BuildBatchJobDefinitionRetryStrategyEl {
    pub fn build(self) -> BatchJobDefinitionRetryStrategyEl {
        BatchJobDefinitionRetryStrategyEl {
            attempts: core::default::Default::default(),
            evaluate_on_exit: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct BatchJobDefinitionRetryStrategyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinitionRetryStrategyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BatchJobDefinitionRetryStrategyElRef {
        BatchJobDefinitionRetryStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionRetryStrategyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attempts` after provisioning.\n"]
    pub fn attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attempts", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluate_on_exit` after provisioning.\n"]
    pub fn evaluate_on_exit(&self) -> terrars::ListRef<BatchJobDefinitionRetryStrategyElEvaluateOnExitElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluate_on_exit", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinitionTimeoutEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attempt_duration_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl BatchJobDefinitionTimeoutEl {
    #[doc = "Set the field `attempt_duration_seconds`.\n"]
    pub fn set_attempt_duration_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.attempt_duration_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BatchJobDefinitionTimeoutEl {
    type O = terrars::BlockAssignable<BatchJobDefinitionTimeoutEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionTimeoutEl {}

impl BuildBatchJobDefinitionTimeoutEl {
    pub fn build(self) -> BatchJobDefinitionTimeoutEl {
        BatchJobDefinitionTimeoutEl { attempt_duration_seconds: core::default::Default::default() }
    }
}

pub struct BatchJobDefinitionTimeoutElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinitionTimeoutElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BatchJobDefinitionTimeoutElRef {
        BatchJobDefinitionTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionTimeoutElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attempt_duration_seconds` after provisioning.\n"]
    pub fn attempt_duration_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attempt_duration_seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct BatchJobDefinitionDynamic {
    retry_strategy: std::option::Option<terrars::DynamicBlock<BatchJobDefinitionRetryStrategyEl>>,
    timeout: std::option::Option<terrars::DynamicBlock<BatchJobDefinitionTimeoutEl>>,
}
