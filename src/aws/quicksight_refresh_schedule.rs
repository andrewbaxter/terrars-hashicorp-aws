#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct QuicksightRefreshScheduleData_ {
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
    aws_account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    data_set_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    schedule_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<std::vec::Vec<QuicksightRefreshSchedule_Schedule>>,
    dynamic: QuicksightRefreshScheduleDynamic,
}

struct QuicksightRefreshSchedule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<QuicksightRefreshScheduleData_>,
}

#[derive(Clone)]
pub struct QuicksightRefreshSchedule(std::rc::Rc<QuicksightRefreshSchedule_>);

impl QuicksightRefreshSchedule {
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

    #[doc = "Set the field `aws_account_id`.\n"]
    pub fn set_aws_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().aws_account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightRefreshSchedule_Schedule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_set_id` after provisioning.\n"]
    pub fn data_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_set_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_id` after provisioning.\n"]
    pub fn schedule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<QuicksightRefreshSchedule_ScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }
}

impl terrars::Referable for QuicksightRefreshSchedule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for QuicksightRefreshSchedule { }

impl terrars::ToListMappable for QuicksightRefreshSchedule {
    type O = terrars::ListRef<QuicksightRefreshScheduleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for QuicksightRefreshSchedule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_quicksight_refresh_schedule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildQuicksightRefreshSchedule {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub data_set_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub schedule_id: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightRefreshSchedule {
    pub fn build(self, stack: &mut terrars::Stack) -> QuicksightRefreshSchedule {
        let out = QuicksightRefreshSchedule(std::rc::Rc::new(QuicksightRefreshSchedule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(QuicksightRefreshScheduleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                aws_account_id: core::default::Default::default(),
                data_set_id: self.data_set_id,
                region: core::default::Default::default(),
                schedule_id: self.schedule_id,
                schedule: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct QuicksightRefreshScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightRefreshScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl QuicksightRefreshScheduleRef {
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

    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_set_id` after provisioning.\n"]
    pub fn data_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_set_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_id` after provisioning.\n"]
    pub fn schedule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<QuicksightRefreshSchedule_ScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct QuicksightRefreshScheduleDynamic {
    schedule: std::option::Option<terrars::DynamicBlock<QuicksightRefreshSchedule_Schedule>>,
}

#[derive(Serialize)]
pub struct QuicksightRefreshSchedule_Schedule_ScheduleFrequency_RefreshOnDay {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day_of_month: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day_of_week: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl QuicksightRefreshSchedule_Schedule_ScheduleFrequency_RefreshOnDay {
    #[doc = "Set the field `day_of_month`.\n"]
    pub fn set_day_of_month(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.day_of_month = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `day_of_week`.\n"]
    pub fn set_day_of_week(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.day_of_week = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for QuicksightRefreshSchedule_Schedule_ScheduleFrequency_RefreshOnDay {
    type O = terrars::BlockAssignable<QuicksightRefreshSchedule_Schedule_ScheduleFrequency_RefreshOnDay>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightRefreshSchedule_Schedule_ScheduleFrequency_RefreshOnDay {}

impl BuildQuicksightRefreshSchedule_Schedule_ScheduleFrequency_RefreshOnDay {
    pub fn build(self) -> QuicksightRefreshSchedule_Schedule_ScheduleFrequency_RefreshOnDay {
        QuicksightRefreshSchedule_Schedule_ScheduleFrequency_RefreshOnDay {
            day_of_month: core::default::Default::default(),
            day_of_week: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightRefreshSchedule_Schedule_ScheduleFrequency_RefreshOnDayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightRefreshSchedule_Schedule_ScheduleFrequency_RefreshOnDayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightRefreshSchedule_Schedule_ScheduleFrequency_RefreshOnDayRef {
        QuicksightRefreshSchedule_Schedule_ScheduleFrequency_RefreshOnDayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightRefreshSchedule_Schedule_ScheduleFrequency_RefreshOnDayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_month` after provisioning.\n"]
    pub fn day_of_month(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_month", self.base))
    }

    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\n"]
    pub fn day_of_week(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }
}

#[derive(Serialize, Default)]
struct QuicksightRefreshSchedule_Schedule_ScheduleFrequencyDynamic {
    refresh_on_day: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightRefreshSchedule_Schedule_ScheduleFrequency_RefreshOnDay>>,
}

#[derive(Serialize)]
pub struct QuicksightRefreshSchedule_Schedule_ScheduleFrequency {
    interval: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_of_the_day: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timezone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_on_day: std::option::Option<std::vec::Vec<QuicksightRefreshSchedule_Schedule_ScheduleFrequency_RefreshOnDay>>,
    dynamic: QuicksightRefreshSchedule_Schedule_ScheduleFrequencyDynamic,
}

impl QuicksightRefreshSchedule_Schedule_ScheduleFrequency {
    #[doc = "Set the field `time_of_the_day`.\n"]
    pub fn set_time_of_the_day(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_of_the_day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timezone`.\n"]
    pub fn set_timezone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timezone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `refresh_on_day`.\n"]
    pub fn set_refresh_on_day(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<QuicksightRefreshSchedule_Schedule_ScheduleFrequency_RefreshOnDay>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.refresh_on_day = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.refresh_on_day = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightRefreshSchedule_Schedule_ScheduleFrequency {
    type O = terrars::BlockAssignable<QuicksightRefreshSchedule_Schedule_ScheduleFrequency>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightRefreshSchedule_Schedule_ScheduleFrequency {
    #[doc = ""]
    pub interval: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightRefreshSchedule_Schedule_ScheduleFrequency {
    pub fn build(self) -> QuicksightRefreshSchedule_Schedule_ScheduleFrequency {
        QuicksightRefreshSchedule_Schedule_ScheduleFrequency {
            interval: self.interval,
            time_of_the_day: core::default::Default::default(),
            timezone: core::default::Default::default(),
            refresh_on_day: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightRefreshSchedule_Schedule_ScheduleFrequencyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightRefreshSchedule_Schedule_ScheduleFrequencyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightRefreshSchedule_Schedule_ScheduleFrequencyRef {
        QuicksightRefreshSchedule_Schedule_ScheduleFrequencyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightRefreshSchedule_Schedule_ScheduleFrequencyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `time_of_the_day` after provisioning.\n"]
    pub fn time_of_the_day(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_of_the_day", self.base))
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.base))
    }

    #[doc = "Get a reference to the value of field `refresh_on_day` after provisioning.\n"]
    pub fn refresh_on_day(
        &self,
    ) -> terrars::ListRef<QuicksightRefreshSchedule_Schedule_ScheduleFrequency_RefreshOnDayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.refresh_on_day", self.base))
    }
}

#[derive(Serialize, Default)]
struct QuicksightRefreshSchedule_ScheduleDynamic {
    schedule_frequency: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightRefreshSchedule_Schedule_ScheduleFrequency>>,
}

#[derive(Serialize)]
pub struct QuicksightRefreshSchedule_Schedule {
    refresh_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_after_date_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_frequency: std::option::Option<std::vec::Vec<QuicksightRefreshSchedule_Schedule_ScheduleFrequency>>,
    dynamic: QuicksightRefreshSchedule_ScheduleDynamic,
}

impl QuicksightRefreshSchedule_Schedule {
    #[doc = "Set the field `start_after_date_time`.\n"]
    pub fn set_start_after_date_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.start_after_date_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_frequency`.\n"]
    pub fn set_schedule_frequency(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightRefreshSchedule_Schedule_ScheduleFrequency>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schedule_frequency = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schedule_frequency = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightRefreshSchedule_Schedule {
    type O = terrars::BlockAssignable<QuicksightRefreshSchedule_Schedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightRefreshSchedule_Schedule {
    #[doc = ""]
    pub refresh_type: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightRefreshSchedule_Schedule {
    pub fn build(self) -> QuicksightRefreshSchedule_Schedule {
        QuicksightRefreshSchedule_Schedule {
            refresh_type: self.refresh_type,
            start_after_date_time: core::default::Default::default(),
            schedule_frequency: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightRefreshSchedule_ScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightRefreshSchedule_ScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightRefreshSchedule_ScheduleRef {
        QuicksightRefreshSchedule_ScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightRefreshSchedule_ScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `refresh_type` after provisioning.\n"]
    pub fn refresh_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_type", self.base))
    }

    #[doc = "Get a reference to the value of field `start_after_date_time` after provisioning.\n"]
    pub fn start_after_date_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_after_date_time", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_frequency` after provisioning.\n"]
    pub fn schedule_frequency(&self) -> terrars::ListRef<QuicksightRefreshSchedule_Schedule_ScheduleFrequencyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule_frequency", self.base))
    }
}
