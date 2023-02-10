use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BatchJobDefinitionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_properties: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_capabilities: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    propagate_tags: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_strategy: Option<Vec<BatchJobDefinitionRetryStrategyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Vec<BatchJobDefinitionTimeoutEl>>,
    dynamic: BatchJobDefinitionDynamic,
}

struct BatchJobDefinition_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BatchJobDefinitionData>,
}

#[derive(Clone)]
pub struct BatchJobDefinition(Rc<BatchJobDefinition_>);

impl BatchJobDefinition {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `container_properties`.\n"]
    pub fn set_container_properties(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().container_properties = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `parameters`.\n"]
    pub fn set_parameters(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().parameters = Some(v.into());
        self
    }

    #[doc= "Set the field `platform_capabilities`.\n"]
    pub fn set_platform_capabilities(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().platform_capabilities = Some(v.into());
        self
    }

    #[doc= "Set the field `propagate_tags`.\n"]
    pub fn set_propagate_tags(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().propagate_tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `retry_strategy`.\n"]
    pub fn set_retry_strategy(self, v: impl Into<BlockAssignable<BatchJobDefinitionRetryStrategyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retry_strategy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retry_strategy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeout`.\n"]
    pub fn set_timeout(self, v: impl Into<BlockAssignable<BatchJobDefinitionTimeoutEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().timeout = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.timeout = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `container_properties` after provisioning.\n"]
    pub fn container_properties(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_properties", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `platform_capabilities` after provisioning.\n"]
    pub fn platform_capabilities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.platform_capabilities", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retry_strategy` after provisioning.\n"]
    pub fn retry_strategy(&self) -> ListRef<BatchJobDefinitionRetryStrategyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_strategy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<BatchJobDefinitionTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }
}

impl Referable for BatchJobDefinition {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BatchJobDefinition { }

impl ToListMappable for BatchJobDefinition {
    type O = ListRef<BatchJobDefinitionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BatchJobDefinition_ {
    fn extract_resource_type(&self) -> String {
        "aws_batch_job_definition".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBatchJobDefinition {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub type_: PrimField<String>,
}

impl BuildBatchJobDefinition {
    pub fn build(self, stack: &mut Stack) -> BatchJobDefinition {
        let out = BatchJobDefinition(Rc::new(BatchJobDefinition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BatchJobDefinitionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BatchJobDefinitionRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BatchJobDefinitionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `container_properties` after provisioning.\n"]
    pub fn container_properties(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_properties", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `platform_capabilities` after provisioning.\n"]
    pub fn platform_capabilities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.platform_capabilities", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retry_strategy` after provisioning.\n"]
    pub fn retry_strategy(&self) -> ListRef<BatchJobDefinitionRetryStrategyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_strategy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<BatchJobDefinitionTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
    action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_exit_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_status_reason: Option<PrimField<String>>,
}

impl BatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
    #[doc= "Set the field `on_exit_code`.\n"]
    pub fn set_on_exit_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_exit_code = Some(v.into());
        self
    }

    #[doc= "Set the field `on_reason`.\n"]
    pub fn set_on_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_reason = Some(v.into());
        self
    }

    #[doc= "Set the field `on_status_reason`.\n"]
    pub fn set_on_status_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_status_reason = Some(v.into());
        self
    }
}

impl ToListMappable for BatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
    type O = BlockAssignable<BatchJobDefinitionRetryStrategyElEvaluateOnExitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
    #[doc= ""]
    pub action: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionRetryStrategyElEvaluateOnExitElRef {
    fn new(shared: StackShared, base: String) -> BatchJobDefinitionRetryStrategyElEvaluateOnExitElRef {
        BatchJobDefinitionRetryStrategyElEvaluateOnExitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionRetryStrategyElEvaluateOnExitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc= "Get a reference to the value of field `on_exit_code` after provisioning.\n"]
    pub fn on_exit_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_exit_code", self.base))
    }

    #[doc= "Get a reference to the value of field `on_reason` after provisioning.\n"]
    pub fn on_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_reason", self.base))
    }

    #[doc= "Get a reference to the value of field `on_status_reason` after provisioning.\n"]
    pub fn on_status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_status_reason", self.base))
    }
}

#[derive(Serialize, Default)]
struct BatchJobDefinitionRetryStrategyElDynamic {
    evaluate_on_exit: Option<DynamicBlock<BatchJobDefinitionRetryStrategyElEvaluateOnExitEl>>,
}

#[derive(Serialize)]
pub struct BatchJobDefinitionRetryStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attempts: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluate_on_exit: Option<Vec<BatchJobDefinitionRetryStrategyElEvaluateOnExitEl>>,
    dynamic: BatchJobDefinitionRetryStrategyElDynamic,
}

impl BatchJobDefinitionRetryStrategyEl {
    #[doc= "Set the field `attempts`.\n"]
    pub fn set_attempts(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.attempts = Some(v.into());
        self
    }

    #[doc= "Set the field `evaluate_on_exit`.\n"]
    pub fn set_evaluate_on_exit(
        mut self,
        v: impl Into<BlockAssignable<BatchJobDefinitionRetryStrategyElEvaluateOnExitEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.evaluate_on_exit = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.evaluate_on_exit = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BatchJobDefinitionRetryStrategyEl {
    type O = BlockAssignable<BatchJobDefinitionRetryStrategyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct BatchJobDefinitionRetryStrategyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionRetryStrategyElRef {
    fn new(shared: StackShared, base: String) -> BatchJobDefinitionRetryStrategyElRef {
        BatchJobDefinitionRetryStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionRetryStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `attempts` after provisioning.\n"]
    pub fn attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.attempts", self.base))
    }

    #[doc= "Get a reference to the value of field `evaluate_on_exit` after provisioning.\n"]
    pub fn evaluate_on_exit(&self) -> ListRef<BatchJobDefinitionRetryStrategyElEvaluateOnExitElRef> {
        ListRef::new(self.shared().clone(), format!("{}.evaluate_on_exit", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinitionTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attempt_duration_seconds: Option<PrimField<f64>>,
}

impl BatchJobDefinitionTimeoutEl {
    #[doc= "Set the field `attempt_duration_seconds`.\n"]
    pub fn set_attempt_duration_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.attempt_duration_seconds = Some(v.into());
        self
    }
}

impl ToListMappable for BatchJobDefinitionTimeoutEl {
    type O = BlockAssignable<BatchJobDefinitionTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobDefinitionTimeoutElRef {
    fn new(shared: StackShared, base: String) -> BatchJobDefinitionTimeoutElRef {
        BatchJobDefinitionTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinitionTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `attempt_duration_seconds` after provisioning.\n"]
    pub fn attempt_duration_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.attempt_duration_seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct BatchJobDefinitionDynamic {
    retry_strategy: Option<DynamicBlock<BatchJobDefinitionRetryStrategyEl>>,
    timeout: Option<DynamicBlock<BatchJobDefinitionTimeoutEl>>,
}
