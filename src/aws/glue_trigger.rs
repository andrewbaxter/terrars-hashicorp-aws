use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlueTriggerData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_on_creation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workflow_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<std::vec::Vec<GlueTriggerActionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_batching_condition: std::option::Option<std::vec::Vec<GlueTriggerEventBatchingConditionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predicate: std::option::Option<std::vec::Vec<GlueTriggerPredicateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GlueTriggerTimeoutsEl>,
    dynamic: GlueTriggerDynamic,
}

struct GlueTrigger_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GlueTriggerData>,
}

#[derive(Clone)]
pub struct GlueTrigger(std::rc::Rc<GlueTrigger_>);

impl GlueTrigger {
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

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_on_creation`.\n"]
    pub fn set_start_on_creation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().start_on_creation = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `workflow_name`.\n"]
    pub fn set_workflow_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().workflow_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(self, v: impl std::convert::Into<terrars::BlockAssignable<GlueTriggerActionsEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.actions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `event_batching_condition`.\n"]
    pub fn set_event_batching_condition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueTriggerEventBatchingConditionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_batching_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_batching_condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `predicate`.\n"]
    pub fn set_predicate(self, v: impl std::convert::Into<terrars::BlockAssignable<GlueTriggerPredicateEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().predicate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.predicate = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<GlueTriggerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_on_creation` after provisioning.\n"]
    pub fn start_on_creation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_on_creation", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workflow_name` after provisioning.\n"]
    pub fn workflow_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workflow_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> terrars::ListRef<GlueTriggerActionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_batching_condition` after provisioning.\n"]
    pub fn event_batching_condition(&self) -> terrars::ListRef<GlueTriggerEventBatchingConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_batching_condition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `predicate` after provisioning.\n"]
    pub fn predicate(&self) -> terrars::ListRef<GlueTriggerPredicateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predicate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GlueTriggerTimeoutsElRef {
        GlueTriggerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for GlueTrigger {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GlueTrigger { }

impl terrars::ToListMappable for GlueTrigger {
    type O = terrars::ListRef<GlueTriggerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GlueTrigger_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_glue_trigger".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlueTrigger {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildGlueTrigger {
    pub fn build(self, stack: &mut terrars::Stack) -> GlueTrigger {
        let out = GlueTrigger(std::rc::Rc::new(GlueTrigger_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GlueTriggerData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                schedule: core::default::Default::default(),
                start_on_creation: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                workflow_name: core::default::Default::default(),
                actions: core::default::Default::default(),
                event_batching_condition: core::default::Default::default(),
                predicate: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlueTriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueTriggerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GlueTriggerRef {
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

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_on_creation` after provisioning.\n"]
    pub fn start_on_creation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_on_creation", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workflow_name` after provisioning.\n"]
    pub fn workflow_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workflow_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> terrars::ListRef<GlueTriggerActionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_batching_condition` after provisioning.\n"]
    pub fn event_batching_condition(&self) -> terrars::ListRef<GlueTriggerEventBatchingConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_batching_condition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `predicate` after provisioning.\n"]
    pub fn predicate(&self) -> terrars::ListRef<GlueTriggerPredicateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predicate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GlueTriggerTimeoutsElRef {
        GlueTriggerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GlueTriggerActionsElNotificationPropertyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notify_delay_after: std::option::Option<terrars::PrimField<f64>>,
}

impl GlueTriggerActionsElNotificationPropertyEl {
    #[doc = "Set the field `notify_delay_after`.\n"]
    pub fn set_notify_delay_after(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.notify_delay_after = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueTriggerActionsElNotificationPropertyEl {
    type O = terrars::BlockAssignable<GlueTriggerActionsElNotificationPropertyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueTriggerActionsElNotificationPropertyEl {}

impl BuildGlueTriggerActionsElNotificationPropertyEl {
    pub fn build(self) -> GlueTriggerActionsElNotificationPropertyEl {
        GlueTriggerActionsElNotificationPropertyEl { notify_delay_after: core::default::Default::default() }
    }
}

pub struct GlueTriggerActionsElNotificationPropertyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueTriggerActionsElNotificationPropertyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueTriggerActionsElNotificationPropertyElRef {
        GlueTriggerActionsElNotificationPropertyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueTriggerActionsElNotificationPropertyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `notify_delay_after` after provisioning.\n"]
    pub fn notify_delay_after(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notify_delay_after", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueTriggerActionsElDynamic {
    notification_property: std::option::Option<terrars::DynamicBlock<GlueTriggerActionsElNotificationPropertyEl>>,
}

#[derive(Serialize)]
pub struct GlueTriggerActionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arguments: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crawler_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_configuration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_property: std::option::Option<std::vec::Vec<GlueTriggerActionsElNotificationPropertyEl>>,
    dynamic: GlueTriggerActionsElDynamic,
}

impl GlueTriggerActionsEl {
    #[doc = "Set the field `arguments`.\n"]
    pub fn set_arguments(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.arguments = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `crawler_name`.\n"]
    pub fn set_crawler_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.crawler_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `job_name`.\n"]
    pub fn set_job_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.job_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_configuration`.\n"]
    pub fn set_security_configuration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.security_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `notification_property`.\n"]
    pub fn set_notification_property(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueTriggerActionsElNotificationPropertyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.notification_property = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.notification_property = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueTriggerActionsEl {
    type O = terrars::BlockAssignable<GlueTriggerActionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueTriggerActionsEl {}

impl BuildGlueTriggerActionsEl {
    pub fn build(self) -> GlueTriggerActionsEl {
        GlueTriggerActionsEl {
            arguments: core::default::Default::default(),
            crawler_name: core::default::Default::default(),
            job_name: core::default::Default::default(),
            security_configuration: core::default::Default::default(),
            timeout: core::default::Default::default(),
            notification_property: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GlueTriggerActionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueTriggerActionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueTriggerActionsElRef {
        GlueTriggerActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueTriggerActionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arguments` after provisioning.\n"]
    pub fn arguments(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.arguments", self.base))
    }

    #[doc = "Get a reference to the value of field `crawler_name` after provisioning.\n"]
    pub fn crawler_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crawler_name", self.base))
    }

    #[doc = "Get a reference to the value of field `job_name` after provisioning.\n"]
    pub fn job_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_name", self.base))
    }

    #[doc = "Get a reference to the value of field `security_configuration` after provisioning.\n"]
    pub fn security_configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `notification_property` after provisioning.\n"]
    pub fn notification_property(&self) -> terrars::ListRef<GlueTriggerActionsElNotificationPropertyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_property", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueTriggerEventBatchingConditionEl {
    batch_size: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_window: std::option::Option<terrars::PrimField<f64>>,
}

impl GlueTriggerEventBatchingConditionEl {
    #[doc = "Set the field `batch_window`.\n"]
    pub fn set_batch_window(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.batch_window = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueTriggerEventBatchingConditionEl {
    type O = terrars::BlockAssignable<GlueTriggerEventBatchingConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueTriggerEventBatchingConditionEl {
    #[doc = ""]
    pub batch_size: terrars::PrimField<f64>,
}

impl BuildGlueTriggerEventBatchingConditionEl {
    pub fn build(self) -> GlueTriggerEventBatchingConditionEl {
        GlueTriggerEventBatchingConditionEl {
            batch_size: self.batch_size,
            batch_window: core::default::Default::default(),
        }
    }
}

pub struct GlueTriggerEventBatchingConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueTriggerEventBatchingConditionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueTriggerEventBatchingConditionElRef {
        GlueTriggerEventBatchingConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueTriggerEventBatchingConditionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.base))
    }

    #[doc = "Get a reference to the value of field `batch_window` after provisioning.\n"]
    pub fn batch_window(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_window", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueTriggerPredicateElConditionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crawl_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crawler_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logical_operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueTriggerPredicateElConditionsEl {
    #[doc = "Set the field `crawl_state`.\n"]
    pub fn set_crawl_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.crawl_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `crawler_name`.\n"]
    pub fn set_crawler_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.crawler_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `job_name`.\n"]
    pub fn set_job_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.job_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logical_operator`.\n"]
    pub fn set_logical_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logical_operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueTriggerPredicateElConditionsEl {
    type O = terrars::BlockAssignable<GlueTriggerPredicateElConditionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueTriggerPredicateElConditionsEl {}

impl BuildGlueTriggerPredicateElConditionsEl {
    pub fn build(self) -> GlueTriggerPredicateElConditionsEl {
        GlueTriggerPredicateElConditionsEl {
            crawl_state: core::default::Default::default(),
            crawler_name: core::default::Default::default(),
            job_name: core::default::Default::default(),
            logical_operator: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct GlueTriggerPredicateElConditionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueTriggerPredicateElConditionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueTriggerPredicateElConditionsElRef {
        GlueTriggerPredicateElConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueTriggerPredicateElConditionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `crawl_state` after provisioning.\n"]
    pub fn crawl_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crawl_state", self.base))
    }

    #[doc = "Get a reference to the value of field `crawler_name` after provisioning.\n"]
    pub fn crawler_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crawler_name", self.base))
    }

    #[doc = "Get a reference to the value of field `job_name` after provisioning.\n"]
    pub fn job_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_name", self.base))
    }

    #[doc = "Get a reference to the value of field `logical_operator` after provisioning.\n"]
    pub fn logical_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logical_operator", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueTriggerPredicateElDynamic {
    conditions: std::option::Option<terrars::DynamicBlock<GlueTriggerPredicateElConditionsEl>>,
}

#[derive(Serialize)]
pub struct GlueTriggerPredicateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logical: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std::option::Option<std::vec::Vec<GlueTriggerPredicateElConditionsEl>>,
    dynamic: GlueTriggerPredicateElDynamic,
}

impl GlueTriggerPredicateEl {
    #[doc = "Set the field `logical`.\n"]
    pub fn set_logical(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logical = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueTriggerPredicateElConditionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueTriggerPredicateEl {
    type O = terrars::BlockAssignable<GlueTriggerPredicateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueTriggerPredicateEl {}

impl BuildGlueTriggerPredicateEl {
    pub fn build(self) -> GlueTriggerPredicateEl {
        GlueTriggerPredicateEl {
            logical: core::default::Default::default(),
            conditions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GlueTriggerPredicateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueTriggerPredicateElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueTriggerPredicateElRef {
        GlueTriggerPredicateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueTriggerPredicateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `logical` after provisioning.\n"]
    pub fn logical(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logical", self.base))
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> terrars::ListRef<GlueTriggerPredicateElConditionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueTriggerTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueTriggerTimeoutsEl {
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
}

impl terrars::ToListMappable for GlueTriggerTimeoutsEl {
    type O = terrars::BlockAssignable<GlueTriggerTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueTriggerTimeoutsEl {}

impl BuildGlueTriggerTimeoutsEl {
    pub fn build(self) -> GlueTriggerTimeoutsEl {
        GlueTriggerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct GlueTriggerTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueTriggerTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueTriggerTimeoutsElRef {
        GlueTriggerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueTriggerTimeoutsElRef {
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
}

#[derive(Serialize, Default)]
struct GlueTriggerDynamic {
    actions: std::option::Option<terrars::DynamicBlock<GlueTriggerActionsEl>>,
    event_batching_condition: std::option::Option<terrars::DynamicBlock<GlueTriggerEventBatchingConditionEl>>,
    predicate: std::option::Option<terrars::DynamicBlock<GlueTriggerPredicateEl>>,
}
