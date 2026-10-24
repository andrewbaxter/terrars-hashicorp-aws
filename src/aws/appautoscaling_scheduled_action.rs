use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppautoscalingScheduledActionData {
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
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    resource_id: terrars::PrimField<std::string::String>,
    scalable_dimension: terrars::PrimField<std::string::String>,
    schedule: terrars::PrimField<std::string::String>,
    service_namespace: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timezone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scalable_target_action: std::option::Option<std::vec::Vec<AppautoscalingScheduledActionScalableTargetActionEl>>,
    dynamic: AppautoscalingScheduledActionDynamic,
}

struct AppautoscalingScheduledAction_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppautoscalingScheduledActionData>,
}

#[derive(Clone)]
pub struct AppautoscalingScheduledAction(std::rc::Rc<AppautoscalingScheduledAction_>);

impl AppautoscalingScheduledAction {
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

    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timezone`.\n"]
    pub fn set_timezone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().timezone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scalable_target_action`.\n"]
    pub fn set_scalable_target_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppautoscalingScheduledActionScalableTargetActionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scalable_target_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scalable_target_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scalable_dimension` after provisioning.\n"]
    pub fn scalable_dimension(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scalable_dimension", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_namespace` after provisioning.\n"]
    pub fn service_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scalable_target_action` after provisioning.\n"]
    pub fn scalable_target_action(&self) -> terrars::ListRef<AppautoscalingScheduledActionScalableTargetActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scalable_target_action", self.extract_ref()))
    }
}

impl terrars::Referable for AppautoscalingScheduledAction {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppautoscalingScheduledAction { }

impl terrars::ToListMappable for AppautoscalingScheduledAction {
    type O = terrars::ListRef<AppautoscalingScheduledActionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppautoscalingScheduledAction_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_appautoscaling_scheduled_action".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppautoscalingScheduledAction {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub resource_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub scalable_dimension: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub schedule: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_namespace: terrars::PrimField<std::string::String>,
}

impl BuildAppautoscalingScheduledAction {
    pub fn build(self, stack: &mut terrars::Stack) -> AppautoscalingScheduledAction {
        let out = AppautoscalingScheduledAction(std::rc::Rc::new(AppautoscalingScheduledAction_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppautoscalingScheduledActionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                end_time: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                resource_id: self.resource_id,
                scalable_dimension: self.scalable_dimension,
                schedule: self.schedule,
                service_namespace: self.service_namespace,
                start_time: core::default::Default::default(),
                timezone: core::default::Default::default(),
                scalable_target_action: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppautoscalingScheduledActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppautoscalingScheduledActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppautoscalingScheduledActionRef {
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

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scalable_dimension` after provisioning.\n"]
    pub fn scalable_dimension(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scalable_dimension", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_namespace` after provisioning.\n"]
    pub fn service_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scalable_target_action` after provisioning.\n"]
    pub fn scalable_target_action(&self) -> terrars::ListRef<AppautoscalingScheduledActionScalableTargetActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scalable_target_action", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppautoscalingScheduledActionScalableTargetActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_capacity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_capacity: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppautoscalingScheduledActionScalableTargetActionEl {
    #[doc = "Set the field `max_capacity`.\n"]
    pub fn set_max_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_capacity`.\n"]
    pub fn set_min_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_capacity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppautoscalingScheduledActionScalableTargetActionEl {
    type O = terrars::BlockAssignable<AppautoscalingScheduledActionScalableTargetActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingScheduledActionScalableTargetActionEl {}

impl BuildAppautoscalingScheduledActionScalableTargetActionEl {
    pub fn build(self) -> AppautoscalingScheduledActionScalableTargetActionEl {
        AppautoscalingScheduledActionScalableTargetActionEl {
            max_capacity: core::default::Default::default(),
            min_capacity: core::default::Default::default(),
        }
    }
}

pub struct AppautoscalingScheduledActionScalableTargetActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppautoscalingScheduledActionScalableTargetActionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppautoscalingScheduledActionScalableTargetActionElRef {
        AppautoscalingScheduledActionScalableTargetActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingScheduledActionScalableTargetActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_capacity` after provisioning.\n"]
    pub fn max_capacity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `min_capacity` after provisioning.\n"]
    pub fn min_capacity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_capacity", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingScheduledActionDynamic {
    scalable_target_action: std
    ::option
    ::Option<terrars::DynamicBlock<AppautoscalingScheduledActionScalableTargetActionEl>>,
}
