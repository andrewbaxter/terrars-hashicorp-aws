#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SchedulerScheduleData_ {
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
    action_after_completion: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    schedule_expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_expression_timezone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flexible_time_window: std::option::Option<std::vec::Vec<SchedulerSchedule_FlexibleTimeWindow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<std::vec::Vec<SchedulerSchedule_Target>>,
    dynamic: SchedulerScheduleDynamic,
}

struct SchedulerSchedule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SchedulerScheduleData_>,
}

#[derive(Clone)]
pub struct SchedulerSchedule(std::rc::Rc<SchedulerSchedule_>);

impl SchedulerSchedule {
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

    #[doc = "Set the field `action_after_completion`.\n"]
    pub fn set_action_after_completion(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().action_after_completion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `end_date`.\n"]
    pub fn set_end_date(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().end_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_name`.\n"]
    pub fn set_group_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_expression_timezone`.\n"]
    pub fn set_schedule_expression_timezone(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().schedule_expression_timezone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_date`.\n"]
    pub fn set_start_date(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().start_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flexible_time_window`.\n"]
    pub fn set_flexible_time_window(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SchedulerSchedule_FlexibleTimeWindow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().flexible_time_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.flexible_time_window = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(self, v: impl std::convert::Into<terrars::BlockAssignable<SchedulerSchedule_Target>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `action_after_completion` after provisioning.\n"]
    pub fn action_after_completion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_after_completion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_expression_timezone` after provisioning.\n"]
    pub fn schedule_expression_timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression_timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `flexible_time_window` after provisioning.\n"]
    pub fn flexible_time_window(&self) -> terrars::ListRef<SchedulerSchedule_FlexibleTimeWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.flexible_time_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::ListRef<SchedulerSchedule_TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }
}

impl terrars::Referable for SchedulerSchedule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SchedulerSchedule { }

impl terrars::ToListMappable for SchedulerSchedule {
    type O = terrars::ListRef<SchedulerScheduleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SchedulerSchedule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_scheduler_schedule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSchedulerSchedule {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub schedule_expression: terrars::PrimField<std::string::String>,
}

impl BuildSchedulerSchedule {
    pub fn build(self, stack: &mut terrars::Stack) -> SchedulerSchedule {
        let out = SchedulerSchedule(std::rc::Rc::new(SchedulerSchedule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SchedulerScheduleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                action_after_completion: core::default::Default::default(),
                description: core::default::Default::default(),
                end_date: core::default::Default::default(),
                group_name: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                region: core::default::Default::default(),
                schedule_expression: self.schedule_expression,
                schedule_expression_timezone: core::default::Default::default(),
                start_date: core::default::Default::default(),
                state: core::default::Default::default(),
                flexible_time_window: core::default::Default::default(),
                target: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SchedulerScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SchedulerScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SchedulerScheduleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_after_completion` after provisioning.\n"]
    pub fn action_after_completion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_after_completion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_expression_timezone` after provisioning.\n"]
    pub fn schedule_expression_timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression_timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `flexible_time_window` after provisioning.\n"]
    pub fn flexible_time_window(&self) -> terrars::ListRef<SchedulerSchedule_FlexibleTimeWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.flexible_time_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::ListRef<SchedulerSchedule_TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SchedulerScheduleDynamic {
    flexible_time_window: std::option::Option<terrars::DynamicBlock<SchedulerSchedule_FlexibleTimeWindow>>,
    target: std::option::Option<terrars::DynamicBlock<SchedulerSchedule_Target>>,
}

#[derive(Serialize)]
pub struct SchedulerSchedule_FlexibleTimeWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_window_in_minutes: std::option::Option<terrars::PrimField<f64>>,
    mode: terrars::PrimField<std::string::String>,
}

impl SchedulerSchedule_FlexibleTimeWindow {
    #[doc = "Set the field `maximum_window_in_minutes`.\n"]
    pub fn set_maximum_window_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_window_in_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SchedulerSchedule_FlexibleTimeWindow {
    type O = terrars::BlockAssignable<SchedulerSchedule_FlexibleTimeWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerSchedule_FlexibleTimeWindow {
    #[doc = ""]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildSchedulerSchedule_FlexibleTimeWindow {
    pub fn build(self) -> SchedulerSchedule_FlexibleTimeWindow {
        SchedulerSchedule_FlexibleTimeWindow {
            maximum_window_in_minutes: core::default::Default::default(),
            mode: self.mode,
        }
    }
}

#[allow(dead_code)]
pub struct SchedulerSchedule_FlexibleTimeWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SchedulerSchedule_FlexibleTimeWindowRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SchedulerSchedule_FlexibleTimeWindowRef {
        SchedulerSchedule_FlexibleTimeWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerSchedule_FlexibleTimeWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_window_in_minutes` after provisioning.\n"]
    pub fn maximum_window_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_window_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct SchedulerSchedule_Target_DeadLetterConfig {
    arn: terrars::PrimField<std::string::String>,
}

impl SchedulerSchedule_Target_DeadLetterConfig { }

impl terrars::ToListMappable for SchedulerSchedule_Target_DeadLetterConfig {
    type O = terrars::BlockAssignable<SchedulerSchedule_Target_DeadLetterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerSchedule_Target_DeadLetterConfig {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildSchedulerSchedule_Target_DeadLetterConfig {
    pub fn build(self) -> SchedulerSchedule_Target_DeadLetterConfig {
        SchedulerSchedule_Target_DeadLetterConfig { arn: self.arn }
    }
}

#[allow(dead_code)]
pub struct SchedulerSchedule_Target_DeadLetterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SchedulerSchedule_Target_DeadLetterConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SchedulerSchedule_Target_DeadLetterConfigRef {
        SchedulerSchedule_Target_DeadLetterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerSchedule_Target_DeadLetterConfigRef {
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
pub struct SchedulerSchedule_Target_EcsParameters_CapacityProviderStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base: std::option::Option<terrars::PrimField<f64>>,
    capacity_provider: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl SchedulerSchedule_Target_EcsParameters_CapacityProviderStrategy {
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

impl terrars::ToListMappable for SchedulerSchedule_Target_EcsParameters_CapacityProviderStrategy {
    type O = terrars::BlockAssignable<SchedulerSchedule_Target_EcsParameters_CapacityProviderStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerSchedule_Target_EcsParameters_CapacityProviderStrategy {
    #[doc = ""]
    pub capacity_provider: terrars::PrimField<std::string::String>,
}

impl BuildSchedulerSchedule_Target_EcsParameters_CapacityProviderStrategy {
    pub fn build(self) -> SchedulerSchedule_Target_EcsParameters_CapacityProviderStrategy {
        SchedulerSchedule_Target_EcsParameters_CapacityProviderStrategy {
            base: core::default::Default::default(),
            capacity_provider: self.capacity_provider,
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SchedulerSchedule_Target_EcsParameters_CapacityProviderStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SchedulerSchedule_Target_EcsParameters_CapacityProviderStrategyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SchedulerSchedule_Target_EcsParameters_CapacityProviderStrategyRef {
        SchedulerSchedule_Target_EcsParameters_CapacityProviderStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerSchedule_Target_EcsParameters_CapacityProviderStrategyRef {
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
pub struct SchedulerSchedule_Target_EcsParameters_NetworkConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    assign_public_ip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl SchedulerSchedule_Target_EcsParameters_NetworkConfiguration {
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

impl terrars::ToListMappable for SchedulerSchedule_Target_EcsParameters_NetworkConfiguration {
    type O = terrars::BlockAssignable<SchedulerSchedule_Target_EcsParameters_NetworkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerSchedule_Target_EcsParameters_NetworkConfiguration {
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildSchedulerSchedule_Target_EcsParameters_NetworkConfiguration {
    pub fn build(self) -> SchedulerSchedule_Target_EcsParameters_NetworkConfiguration {
        SchedulerSchedule_Target_EcsParameters_NetworkConfiguration {
            assign_public_ip: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            subnets: self.subnets,
        }
    }
}

#[allow(dead_code)]
pub struct SchedulerSchedule_Target_EcsParameters_NetworkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SchedulerSchedule_Target_EcsParameters_NetworkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SchedulerSchedule_Target_EcsParameters_NetworkConfigurationRef {
        SchedulerSchedule_Target_EcsParameters_NetworkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerSchedule_Target_EcsParameters_NetworkConfigurationRef {
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
pub struct SchedulerSchedule_Target_EcsParameters_PlacementConstraints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl SchedulerSchedule_Target_EcsParameters_PlacementConstraints {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SchedulerSchedule_Target_EcsParameters_PlacementConstraints {
    type O = terrars::BlockAssignable<SchedulerSchedule_Target_EcsParameters_PlacementConstraints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerSchedule_Target_EcsParameters_PlacementConstraints {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildSchedulerSchedule_Target_EcsParameters_PlacementConstraints {
    pub fn build(self) -> SchedulerSchedule_Target_EcsParameters_PlacementConstraints {
        SchedulerSchedule_Target_EcsParameters_PlacementConstraints {
            expression: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct SchedulerSchedule_Target_EcsParameters_PlacementConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SchedulerSchedule_Target_EcsParameters_PlacementConstraintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SchedulerSchedule_Target_EcsParameters_PlacementConstraintsRef {
        SchedulerSchedule_Target_EcsParameters_PlacementConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerSchedule_Target_EcsParameters_PlacementConstraintsRef {
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

#[derive(Serialize)]
pub struct SchedulerSchedule_Target_EcsParameters_PlacementStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl SchedulerSchedule_Target_EcsParameters_PlacementStrategy {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SchedulerSchedule_Target_EcsParameters_PlacementStrategy {
    type O = terrars::BlockAssignable<SchedulerSchedule_Target_EcsParameters_PlacementStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerSchedule_Target_EcsParameters_PlacementStrategy {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildSchedulerSchedule_Target_EcsParameters_PlacementStrategy {
    pub fn build(self) -> SchedulerSchedule_Target_EcsParameters_PlacementStrategy {
        SchedulerSchedule_Target_EcsParameters_PlacementStrategy {
            field: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct SchedulerSchedule_Target_EcsParameters_PlacementStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SchedulerSchedule_Target_EcsParameters_PlacementStrategyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SchedulerSchedule_Target_EcsParameters_PlacementStrategyRef {
        SchedulerSchedule_Target_EcsParameters_PlacementStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerSchedule_Target_EcsParameters_PlacementStrategyRef {
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

#[derive(Serialize, Default)]
struct SchedulerSchedule_Target_EcsParametersDynamic {
    capacity_provider_strategy: std
    ::option
    ::Option<terrars::DynamicBlock<SchedulerSchedule_Target_EcsParameters_CapacityProviderStrategy>>,
    network_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<SchedulerSchedule_Target_EcsParameters_NetworkConfiguration>>,
    placement_constraints: std
    ::option
    ::Option<terrars::DynamicBlock<SchedulerSchedule_Target_EcsParameters_PlacementConstraints>>,
    placement_strategy: std
    ::option
    ::Option<terrars::DynamicBlock<SchedulerSchedule_Target_EcsParameters_PlacementStrategy>>,
}

#[derive(Serialize)]
pub struct SchedulerSchedule_Target_EcsParameters {
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
    reference_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_count: std::option::Option<terrars::PrimField<f64>>,
    task_definition_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_provider_strategy: std
    ::option
    ::Option<std::vec::Vec<SchedulerSchedule_Target_EcsParameters_CapacityProviderStrategy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std
    ::option
    ::Option<std::vec::Vec<SchedulerSchedule_Target_EcsParameters_NetworkConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_constraints: std
    ::option
    ::Option<std::vec::Vec<SchedulerSchedule_Target_EcsParameters_PlacementConstraints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_strategy: std::option::Option<std::vec::Vec<SchedulerSchedule_Target_EcsParameters_PlacementStrategy>>,
    dynamic: SchedulerSchedule_Target_EcsParametersDynamic,
}

impl SchedulerSchedule_Target_EcsParameters {
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

    #[doc = "Set the field `reference_id`.\n"]
    pub fn set_reference_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reference_id = std::option::Option::Some(v.into());
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
                    ::Into<terrars::BlockAssignable<SchedulerSchedule_Target_EcsParameters_CapacityProviderStrategy>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SchedulerSchedule_Target_EcsParameters_NetworkConfiguration>>,
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

    #[doc = "Set the field `placement_constraints`.\n"]
    pub fn set_placement_constraints(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SchedulerSchedule_Target_EcsParameters_PlacementConstraints>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.placement_constraints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.placement_constraints = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placement_strategy`.\n"]
    pub fn set_placement_strategy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SchedulerSchedule_Target_EcsParameters_PlacementStrategy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.placement_strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.placement_strategy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SchedulerSchedule_Target_EcsParameters {
    type O = terrars::BlockAssignable<SchedulerSchedule_Target_EcsParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerSchedule_Target_EcsParameters {
    #[doc = ""]
    pub task_definition_arn: terrars::PrimField<std::string::String>,
}

impl BuildSchedulerSchedule_Target_EcsParameters {
    pub fn build(self) -> SchedulerSchedule_Target_EcsParameters {
        SchedulerSchedule_Target_EcsParameters {
            enable_ecs_managed_tags: core::default::Default::default(),
            enable_execute_command: core::default::Default::default(),
            group: core::default::Default::default(),
            launch_type: core::default::Default::default(),
            platform_version: core::default::Default::default(),
            propagate_tags: core::default::Default::default(),
            reference_id: core::default::Default::default(),
            tags: core::default::Default::default(),
            task_count: core::default::Default::default(),
            task_definition_arn: self.task_definition_arn,
            capacity_provider_strategy: core::default::Default::default(),
            network_configuration: core::default::Default::default(),
            placement_constraints: core::default::Default::default(),
            placement_strategy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SchedulerSchedule_Target_EcsParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SchedulerSchedule_Target_EcsParametersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SchedulerSchedule_Target_EcsParametersRef {
        SchedulerSchedule_Target_EcsParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerSchedule_Target_EcsParametersRef {
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

    #[doc = "Get a reference to the value of field `reference_id` after provisioning.\n"]
    pub fn reference_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reference_id", self.base))
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
    pub fn network_configuration(
        &self,
    ) -> terrars::ListRef<SchedulerSchedule_Target_EcsParameters_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct SchedulerSchedule_Target_EventbridgeParameters {
    detail_type: terrars::PrimField<std::string::String>,
    source: terrars::PrimField<std::string::String>,
}

impl SchedulerSchedule_Target_EventbridgeParameters { }

impl terrars::ToListMappable for SchedulerSchedule_Target_EventbridgeParameters {
    type O = terrars::BlockAssignable<SchedulerSchedule_Target_EventbridgeParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerSchedule_Target_EventbridgeParameters {
    #[doc = ""]
    pub detail_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source: terrars::PrimField<std::string::String>,
}

impl BuildSchedulerSchedule_Target_EventbridgeParameters {
    pub fn build(self) -> SchedulerSchedule_Target_EventbridgeParameters {
        SchedulerSchedule_Target_EventbridgeParameters {
            detail_type: self.detail_type,
            source: self.source,
        }
    }
}

#[allow(dead_code)]
pub struct SchedulerSchedule_Target_EventbridgeParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SchedulerSchedule_Target_EventbridgeParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SchedulerSchedule_Target_EventbridgeParametersRef {
        SchedulerSchedule_Target_EventbridgeParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerSchedule_Target_EventbridgeParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `detail_type` after provisioning.\n"]
    pub fn detail_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detail_type", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct SchedulerSchedule_Target_KinesisParameters {
    partition_key: terrars::PrimField<std::string::String>,
}

impl SchedulerSchedule_Target_KinesisParameters { }

impl terrars::ToListMappable for SchedulerSchedule_Target_KinesisParameters {
    type O = terrars::BlockAssignable<SchedulerSchedule_Target_KinesisParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerSchedule_Target_KinesisParameters {
    #[doc = ""]
    pub partition_key: terrars::PrimField<std::string::String>,
}

impl BuildSchedulerSchedule_Target_KinesisParameters {
    pub fn build(self) -> SchedulerSchedule_Target_KinesisParameters {
        SchedulerSchedule_Target_KinesisParameters { partition_key: self.partition_key }
    }
}

#[allow(dead_code)]
pub struct SchedulerSchedule_Target_KinesisParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SchedulerSchedule_Target_KinesisParametersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SchedulerSchedule_Target_KinesisParametersRef {
        SchedulerSchedule_Target_KinesisParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerSchedule_Target_KinesisParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `partition_key` after provisioning.\n"]
    pub fn partition_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partition_key", self.base))
    }
}

#[derive(Serialize)]
pub struct SchedulerSchedule_Target_RetryPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_event_age_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_retry_attempts: std::option::Option<terrars::PrimField<f64>>,
}

impl SchedulerSchedule_Target_RetryPolicy {
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

impl terrars::ToListMappable for SchedulerSchedule_Target_RetryPolicy {
    type O = terrars::BlockAssignable<SchedulerSchedule_Target_RetryPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerSchedule_Target_RetryPolicy {}

impl BuildSchedulerSchedule_Target_RetryPolicy {
    pub fn build(self) -> SchedulerSchedule_Target_RetryPolicy {
        SchedulerSchedule_Target_RetryPolicy {
            maximum_event_age_in_seconds: core::default::Default::default(),
            maximum_retry_attempts: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SchedulerSchedule_Target_RetryPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SchedulerSchedule_Target_RetryPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SchedulerSchedule_Target_RetryPolicyRef {
        SchedulerSchedule_Target_RetryPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerSchedule_Target_RetryPolicyRef {
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
pub struct SchedulerSchedule_Target_SagemakerPipelineParameters_PipelineParameter {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SchedulerSchedule_Target_SagemakerPipelineParameters_PipelineParameter { }

impl terrars::ToListMappable for SchedulerSchedule_Target_SagemakerPipelineParameters_PipelineParameter {
    type O = terrars::BlockAssignable<SchedulerSchedule_Target_SagemakerPipelineParameters_PipelineParameter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerSchedule_Target_SagemakerPipelineParameters_PipelineParameter {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSchedulerSchedule_Target_SagemakerPipelineParameters_PipelineParameter {
    pub fn build(self) -> SchedulerSchedule_Target_SagemakerPipelineParameters_PipelineParameter {
        SchedulerSchedule_Target_SagemakerPipelineParameters_PipelineParameter {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SchedulerSchedule_Target_SagemakerPipelineParameters_PipelineParameterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SchedulerSchedule_Target_SagemakerPipelineParameters_PipelineParameterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SchedulerSchedule_Target_SagemakerPipelineParameters_PipelineParameterRef {
        SchedulerSchedule_Target_SagemakerPipelineParameters_PipelineParameterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerSchedule_Target_SagemakerPipelineParameters_PipelineParameterRef {
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
struct SchedulerSchedule_Target_SagemakerPipelineParametersDynamic {
    pipeline_parameter: std
    ::option
    ::Option<terrars::DynamicBlock<SchedulerSchedule_Target_SagemakerPipelineParameters_PipelineParameter>>,
}

#[derive(Serialize)]
pub struct SchedulerSchedule_Target_SagemakerPipelineParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_parameter: std
    ::option
    ::Option<std::vec::Vec<SchedulerSchedule_Target_SagemakerPipelineParameters_PipelineParameter>>,
    dynamic: SchedulerSchedule_Target_SagemakerPipelineParametersDynamic,
}

impl SchedulerSchedule_Target_SagemakerPipelineParameters {
    #[doc = "Set the field `pipeline_parameter`.\n"]
    pub fn set_pipeline_parameter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SchedulerSchedule_Target_SagemakerPipelineParameters_PipelineParameter>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pipeline_parameter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pipeline_parameter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SchedulerSchedule_Target_SagemakerPipelineParameters {
    type O = terrars::BlockAssignable<SchedulerSchedule_Target_SagemakerPipelineParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerSchedule_Target_SagemakerPipelineParameters {}

impl BuildSchedulerSchedule_Target_SagemakerPipelineParameters {
    pub fn build(self) -> SchedulerSchedule_Target_SagemakerPipelineParameters {
        SchedulerSchedule_Target_SagemakerPipelineParameters {
            pipeline_parameter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SchedulerSchedule_Target_SagemakerPipelineParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SchedulerSchedule_Target_SagemakerPipelineParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SchedulerSchedule_Target_SagemakerPipelineParametersRef {
        SchedulerSchedule_Target_SagemakerPipelineParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerSchedule_Target_SagemakerPipelineParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct SchedulerSchedule_Target_SqsParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SchedulerSchedule_Target_SqsParameters {
    #[doc = "Set the field `message_group_id`.\n"]
    pub fn set_message_group_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message_group_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SchedulerSchedule_Target_SqsParameters {
    type O = terrars::BlockAssignable<SchedulerSchedule_Target_SqsParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerSchedule_Target_SqsParameters {}

impl BuildSchedulerSchedule_Target_SqsParameters {
    pub fn build(self) -> SchedulerSchedule_Target_SqsParameters {
        SchedulerSchedule_Target_SqsParameters { message_group_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct SchedulerSchedule_Target_SqsParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SchedulerSchedule_Target_SqsParametersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SchedulerSchedule_Target_SqsParametersRef {
        SchedulerSchedule_Target_SqsParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerSchedule_Target_SqsParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message_group_id` after provisioning.\n"]
    pub fn message_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_group_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct SchedulerSchedule_TargetDynamic {
    dead_letter_config: std::option::Option<terrars::DynamicBlock<SchedulerSchedule_Target_DeadLetterConfig>>,
    ecs_parameters: std::option::Option<terrars::DynamicBlock<SchedulerSchedule_Target_EcsParameters>>,
    eventbridge_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<SchedulerSchedule_Target_EventbridgeParameters>>,
    kinesis_parameters: std::option::Option<terrars::DynamicBlock<SchedulerSchedule_Target_KinesisParameters>>,
    retry_policy: std::option::Option<terrars::DynamicBlock<SchedulerSchedule_Target_RetryPolicy>>,
    sagemaker_pipeline_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<SchedulerSchedule_Target_SagemakerPipelineParameters>>,
    sqs_parameters: std::option::Option<terrars::DynamicBlock<SchedulerSchedule_Target_SqsParameters>>,
}

#[derive(Serialize)]
pub struct SchedulerSchedule_Target {
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dead_letter_config: std::option::Option<std::vec::Vec<SchedulerSchedule_Target_DeadLetterConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecs_parameters: std::option::Option<std::vec::Vec<SchedulerSchedule_Target_EcsParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eventbridge_parameters: std::option::Option<std::vec::Vec<SchedulerSchedule_Target_EventbridgeParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_parameters: std::option::Option<std::vec::Vec<SchedulerSchedule_Target_KinesisParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<SchedulerSchedule_Target_RetryPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_pipeline_parameters: std
    ::option
    ::Option<std::vec::Vec<SchedulerSchedule_Target_SagemakerPipelineParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sqs_parameters: std::option::Option<std::vec::Vec<SchedulerSchedule_Target_SqsParameters>>,
    dynamic: SchedulerSchedule_TargetDynamic,
}

impl SchedulerSchedule_Target {
    #[doc = "Set the field `input`.\n"]
    pub fn set_input(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dead_letter_config`.\n"]
    pub fn set_dead_letter_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SchedulerSchedule_Target_DeadLetterConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dead_letter_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dead_letter_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ecs_parameters`.\n"]
    pub fn set_ecs_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SchedulerSchedule_Target_EcsParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ecs_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ecs_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `eventbridge_parameters`.\n"]
    pub fn set_eventbridge_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SchedulerSchedule_Target_EventbridgeParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.eventbridge_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.eventbridge_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_parameters`.\n"]
    pub fn set_kinesis_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SchedulerSchedule_Target_KinesisParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SchedulerSchedule_Target_RetryPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retry_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sagemaker_pipeline_parameters`.\n"]
    pub fn set_sagemaker_pipeline_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SchedulerSchedule_Target_SagemakerPipelineParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sagemaker_pipeline_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sagemaker_pipeline_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sqs_parameters`.\n"]
    pub fn set_sqs_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SchedulerSchedule_Target_SqsParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sqs_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sqs_parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SchedulerSchedule_Target {
    type O = terrars::BlockAssignable<SchedulerSchedule_Target>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerSchedule_Target {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildSchedulerSchedule_Target {
    pub fn build(self) -> SchedulerSchedule_Target {
        SchedulerSchedule_Target {
            arn: self.arn,
            input: core::default::Default::default(),
            role_arn: self.role_arn,
            dead_letter_config: core::default::Default::default(),
            ecs_parameters: core::default::Default::default(),
            eventbridge_parameters: core::default::Default::default(),
            kinesis_parameters: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            sagemaker_pipeline_parameters: core::default::Default::default(),
            sqs_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SchedulerSchedule_TargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SchedulerSchedule_TargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SchedulerSchedule_TargetRef {
        SchedulerSchedule_TargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerSchedule_TargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> terrars::ListRef<SchedulerSchedule_Target_DeadLetterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ecs_parameters` after provisioning.\n"]
    pub fn ecs_parameters(&self) -> terrars::ListRef<SchedulerSchedule_Target_EcsParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ecs_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `eventbridge_parameters` after provisioning.\n"]
    pub fn eventbridge_parameters(&self) -> terrars::ListRef<SchedulerSchedule_Target_EventbridgeParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eventbridge_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_parameters` after provisioning.\n"]
    pub fn kinesis_parameters(&self) -> terrars::ListRef<SchedulerSchedule_Target_KinesisParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<SchedulerSchedule_Target_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_pipeline_parameters` after provisioning.\n"]
    pub fn sagemaker_pipeline_parameters(
        &self,
    ) -> terrars::ListRef<SchedulerSchedule_Target_SagemakerPipelineParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sagemaker_pipeline_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `sqs_parameters` after provisioning.\n"]
    pub fn sqs_parameters(&self) -> terrars::ListRef<SchedulerSchedule_Target_SqsParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sqs_parameters", self.base))
    }
}
