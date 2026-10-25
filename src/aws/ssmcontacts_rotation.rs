#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SsmcontactsRotationData_ {
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
    contact_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    time_zone_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurrence: std::option::Option<std::vec::Vec<SsmcontactsRotation_Recurrence>>,
    dynamic: SsmcontactsRotationDynamic,
}

struct SsmcontactsRotation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SsmcontactsRotationData_>,
}

#[derive(Clone)]
pub struct SsmcontactsRotation(std::rc::Rc<SsmcontactsRotation_>);

impl SsmcontactsRotation {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().start_time = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `recurrence`.\n"]
    pub fn set_recurrence(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SsmcontactsRotation_Recurrence>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().recurrence = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.recurrence = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `contact_ids` after provisioning.\n"]
    pub fn contact_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.contact_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_zone_id` after provisioning.\n"]
    pub fn time_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recurrence` after provisioning.\n"]
    pub fn recurrence(&self) -> terrars::ListRef<SsmcontactsRotation_RecurrenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recurrence", self.extract_ref()))
    }
}

impl terrars::Referable for SsmcontactsRotation {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SsmcontactsRotation { }

impl terrars::ToListMappable for SsmcontactsRotation {
    type O = terrars::ListRef<SsmcontactsRotationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SsmcontactsRotation_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ssmcontacts_rotation".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsmcontactsRotation {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub contact_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub time_zone_id: terrars::PrimField<std::string::String>,
}

impl BuildSsmcontactsRotation {
    pub fn build(self, stack: &mut terrars::Stack) -> SsmcontactsRotation {
        let out = SsmcontactsRotation(std::rc::Rc::new(SsmcontactsRotation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SsmcontactsRotationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                contact_ids: self.contact_ids,
                name: self.name,
                region: core::default::Default::default(),
                start_time: core::default::Default::default(),
                tags: core::default::Default::default(),
                time_zone_id: self.time_zone_id,
                recurrence: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsmcontactsRotationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmcontactsRotationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SsmcontactsRotationRef {
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

    #[doc = "Get a reference to the value of field `contact_ids` after provisioning.\n"]
    pub fn contact_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.contact_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_zone_id` after provisioning.\n"]
    pub fn time_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recurrence` after provisioning.\n"]
    pub fn recurrence(&self) -> terrars::ListRef<SsmcontactsRotation_RecurrenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recurrence", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SsmcontactsRotationDynamic {
    recurrence: std::option::Option<terrars::DynamicBlock<SsmcontactsRotation_Recurrence>>,
}

#[derive(Serialize)]
pub struct SsmcontactsRotation_Recurrence_DailySettings {
    hour_of_day: terrars::PrimField<f64>,
    minute_of_hour: terrars::PrimField<f64>,
}

impl SsmcontactsRotation_Recurrence_DailySettings { }

impl terrars::ToListMappable for SsmcontactsRotation_Recurrence_DailySettings {
    type O = terrars::BlockAssignable<SsmcontactsRotation_Recurrence_DailySettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsRotation_Recurrence_DailySettings {
    #[doc = ""]
    pub hour_of_day: terrars::PrimField<f64>,
    #[doc = ""]
    pub minute_of_hour: terrars::PrimField<f64>,
}

impl BuildSsmcontactsRotation_Recurrence_DailySettings {
    pub fn build(self) -> SsmcontactsRotation_Recurrence_DailySettings {
        SsmcontactsRotation_Recurrence_DailySettings {
            hour_of_day: self.hour_of_day,
            minute_of_hour: self.minute_of_hour,
        }
    }
}

#[allow(dead_code)]
pub struct SsmcontactsRotation_Recurrence_DailySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmcontactsRotation_Recurrence_DailySettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmcontactsRotation_Recurrence_DailySettingsRef {
        SsmcontactsRotation_Recurrence_DailySettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsRotation_Recurrence_DailySettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hour_of_day` after provisioning.\n"]
    pub fn hour_of_day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hour_of_day", self.base))
    }

    #[doc = "Get a reference to the value of field `minute_of_hour` after provisioning.\n"]
    pub fn minute_of_hour(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minute_of_hour", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime {
    hour_of_day: terrars::PrimField<f64>,
    minute_of_hour: terrars::PrimField<f64>,
}

impl SsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime { }

impl terrars::ToListMappable for SsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime {
    type O = terrars::BlockAssignable<SsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime {
    #[doc = ""]
    pub hour_of_day: terrars::PrimField<f64>,
    #[doc = ""]
    pub minute_of_hour: terrars::PrimField<f64>,
}

impl BuildSsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime {
    pub fn build(self) -> SsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime {
        SsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime {
            hour_of_day: self.hour_of_day,
            minute_of_hour: self.minute_of_hour,
        }
    }
}

#[allow(dead_code)]
pub struct SsmcontactsRotation_Recurrence_WeeklySettings_HandOffTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmcontactsRotation_Recurrence_WeeklySettings_HandOffTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmcontactsRotation_Recurrence_WeeklySettings_HandOffTimeRef {
        SsmcontactsRotation_Recurrence_WeeklySettings_HandOffTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsRotation_Recurrence_WeeklySettings_HandOffTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hour_of_day` after provisioning.\n"]
    pub fn hour_of_day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hour_of_day", self.base))
    }

    #[doc = "Get a reference to the value of field `minute_of_hour` after provisioning.\n"]
    pub fn minute_of_hour(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minute_of_hour", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmcontactsRotation_Recurrence_MonthlySettingsDynamic {
    hand_off_time: std
    ::option
    ::Option<terrars::DynamicBlock<SsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime>>,
}

#[derive(Serialize)]
pub struct SsmcontactsRotation_Recurrence_MonthlySettings {
    day_of_month: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hand_off_time: std::option::Option<std::vec::Vec<SsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime>>,
    dynamic: SsmcontactsRotation_Recurrence_MonthlySettingsDynamic,
}

impl SsmcontactsRotation_Recurrence_MonthlySettings {
    #[doc = "Set the field `hand_off_time`.\n"]
    pub fn set_hand_off_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hand_off_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hand_off_time = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SsmcontactsRotation_Recurrence_MonthlySettings {
    type O = terrars::BlockAssignable<SsmcontactsRotation_Recurrence_MonthlySettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsRotation_Recurrence_MonthlySettings {
    #[doc = ""]
    pub day_of_month: terrars::PrimField<f64>,
}

impl BuildSsmcontactsRotation_Recurrence_MonthlySettings {
    pub fn build(self) -> SsmcontactsRotation_Recurrence_MonthlySettings {
        SsmcontactsRotation_Recurrence_MonthlySettings {
            day_of_month: self.day_of_month,
            hand_off_time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsmcontactsRotation_Recurrence_MonthlySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmcontactsRotation_Recurrence_MonthlySettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmcontactsRotation_Recurrence_MonthlySettingsRef {
        SsmcontactsRotation_Recurrence_MonthlySettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsRotation_Recurrence_MonthlySettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_month` after provisioning.\n"]
    pub fn day_of_month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_month", self.base))
    }

    #[doc = "Get a reference to the value of field `hand_off_time` after provisioning.\n"]
    pub fn hand_off_time(&self) -> terrars::ListRef<SsmcontactsRotation_Recurrence_WeeklySettings_HandOffTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hand_off_time", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_End {
    hour_of_day: terrars::PrimField<f64>,
    minute_of_hour: terrars::PrimField<f64>,
}

impl SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_End { }

impl terrars::ToListMappable for SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_End {
    type O = terrars::BlockAssignable<SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_End>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_End {
    #[doc = ""]
    pub hour_of_day: terrars::PrimField<f64>,
    #[doc = ""]
    pub minute_of_hour: terrars::PrimField<f64>,
}

impl BuildSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_End {
    pub fn build(self) -> SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_End {
        SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_End {
            hour_of_day: self.hour_of_day,
            minute_of_hour: self.minute_of_hour,
        }
    }
}

#[allow(dead_code)]
pub struct SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_EndRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_EndRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_EndRef {
        SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_EndRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_EndRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hour_of_day` after provisioning.\n"]
    pub fn hour_of_day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hour_of_day", self.base))
    }

    #[doc = "Get a reference to the value of field `minute_of_hour` after provisioning.\n"]
    pub fn minute_of_hour(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minute_of_hour", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_Start {
    hour_of_day: terrars::PrimField<f64>,
    minute_of_hour: terrars::PrimField<f64>,
}

impl SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_Start { }

impl terrars::ToListMappable for SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_Start {
    type O = terrars::BlockAssignable<SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_Start>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_Start {
    #[doc = ""]
    pub hour_of_day: terrars::PrimField<f64>,
    #[doc = ""]
    pub minute_of_hour: terrars::PrimField<f64>,
}

impl BuildSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_Start {
    pub fn build(self) -> SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_Start {
        SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_Start {
            hour_of_day: self.hour_of_day,
            minute_of_hour: self.minute_of_hour,
        }
    }
}

#[allow(dead_code)]
pub struct SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_StartRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_StartRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_StartRef {
        SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_StartRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_StartRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hour_of_day` after provisioning.\n"]
    pub fn hour_of_day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hour_of_day", self.base))
    }

    #[doc = "Get a reference to the value of field `minute_of_hour` after provisioning.\n"]
    pub fn minute_of_hour(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minute_of_hour", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimesDynamic {
    end: std::option::Option<terrars::DynamicBlock<SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_End>>,
    start: std
    ::option
    ::Option<terrars::DynamicBlock<SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_Start>>,
}

#[derive(Serialize)]
pub struct SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<std::vec::Vec<SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_End>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<std::vec::Vec<SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_Start>>,
    dynamic: SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimesDynamic,
}

impl SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_End>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.end = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.end = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_Start>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.start = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.start = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes {
    type O = terrars::BlockAssignable<SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes {}

impl BuildSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes {
    pub fn build(self) -> SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes {
        SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimesRef {
        SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::ListRef<SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_EndRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::ListRef<SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_StartRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmcontactsRotation_Recurrence_ShiftCoveragesDynamic {
    coverage_times: std
    ::option
    ::Option<terrars::DynamicBlock<SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes>>,
}

#[derive(Serialize)]
pub struct SsmcontactsRotation_Recurrence_ShiftCoverages {
    map_block_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coverage_times: std::option::Option<std::vec::Vec<SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes>>,
    dynamic: SsmcontactsRotation_Recurrence_ShiftCoveragesDynamic,
}

impl SsmcontactsRotation_Recurrence_ShiftCoverages {
    #[doc = "Set the field `coverage_times`.\n"]
    pub fn set_coverage_times(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.coverage_times = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.coverage_times = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SsmcontactsRotation_Recurrence_ShiftCoverages {
    type O = terrars::BlockAssignable<SsmcontactsRotation_Recurrence_ShiftCoverages>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsRotation_Recurrence_ShiftCoverages {
    #[doc = ""]
    pub map_block_key: terrars::PrimField<std::string::String>,
}

impl BuildSsmcontactsRotation_Recurrence_ShiftCoverages {
    pub fn build(self) -> SsmcontactsRotation_Recurrence_ShiftCoverages {
        SsmcontactsRotation_Recurrence_ShiftCoverages {
            map_block_key: self.map_block_key,
            coverage_times: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsmcontactsRotation_Recurrence_ShiftCoveragesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmcontactsRotation_Recurrence_ShiftCoveragesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmcontactsRotation_Recurrence_ShiftCoveragesRef {
        SsmcontactsRotation_Recurrence_ShiftCoveragesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsRotation_Recurrence_ShiftCoveragesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `map_block_key` after provisioning.\n"]
    pub fn map_block_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.map_block_key", self.base))
    }

    #[doc = "Get a reference to the value of field `coverage_times` after provisioning.\n"]
    pub fn coverage_times(&self) -> terrars::ListRef<SsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.coverage_times", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmcontactsRotation_Recurrence_WeeklySettingsDynamic {
    hand_off_time: std
    ::option
    ::Option<terrars::DynamicBlock<SsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime>>,
}

#[derive(Serialize)]
pub struct SsmcontactsRotation_Recurrence_WeeklySettings {
    day_of_week: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hand_off_time: std::option::Option<std::vec::Vec<SsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime>>,
    dynamic: SsmcontactsRotation_Recurrence_WeeklySettingsDynamic,
}

impl SsmcontactsRotation_Recurrence_WeeklySettings {
    #[doc = "Set the field `hand_off_time`.\n"]
    pub fn set_hand_off_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hand_off_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hand_off_time = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SsmcontactsRotation_Recurrence_WeeklySettings {
    type O = terrars::BlockAssignable<SsmcontactsRotation_Recurrence_WeeklySettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsRotation_Recurrence_WeeklySettings {
    #[doc = ""]
    pub day_of_week: terrars::PrimField<std::string::String>,
}

impl BuildSsmcontactsRotation_Recurrence_WeeklySettings {
    pub fn build(self) -> SsmcontactsRotation_Recurrence_WeeklySettings {
        SsmcontactsRotation_Recurrence_WeeklySettings {
            day_of_week: self.day_of_week,
            hand_off_time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsmcontactsRotation_Recurrence_WeeklySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmcontactsRotation_Recurrence_WeeklySettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmcontactsRotation_Recurrence_WeeklySettingsRef {
        SsmcontactsRotation_Recurrence_WeeklySettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsRotation_Recurrence_WeeklySettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\n"]
    pub fn day_of_week(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `hand_off_time` after provisioning.\n"]
    pub fn hand_off_time(&self) -> terrars::ListRef<SsmcontactsRotation_Recurrence_WeeklySettings_HandOffTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hand_off_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmcontactsRotation_RecurrenceDynamic {
    daily_settings: std::option::Option<terrars::DynamicBlock<SsmcontactsRotation_Recurrence_DailySettings>>,
    monthly_settings: std::option::Option<terrars::DynamicBlock<SsmcontactsRotation_Recurrence_MonthlySettings>>,
    shift_coverages: std::option::Option<terrars::DynamicBlock<SsmcontactsRotation_Recurrence_ShiftCoverages>>,
    weekly_settings: std::option::Option<terrars::DynamicBlock<SsmcontactsRotation_Recurrence_WeeklySettings>>,
}

#[derive(Serialize)]
pub struct SsmcontactsRotation_Recurrence {
    number_of_on_calls: terrars::PrimField<f64>,
    recurrence_multiplier: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    daily_settings: std::option::Option<std::vec::Vec<SsmcontactsRotation_Recurrence_DailySettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monthly_settings: std::option::Option<std::vec::Vec<SsmcontactsRotation_Recurrence_MonthlySettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shift_coverages: std::option::Option<std::vec::Vec<SsmcontactsRotation_Recurrence_ShiftCoverages>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_settings: std::option::Option<std::vec::Vec<SsmcontactsRotation_Recurrence_WeeklySettings>>,
    dynamic: SsmcontactsRotation_RecurrenceDynamic,
}

impl SsmcontactsRotation_Recurrence {
    #[doc = "Set the field `daily_settings`.\n"]
    pub fn set_daily_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SsmcontactsRotation_Recurrence_DailySettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.daily_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.daily_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monthly_settings`.\n"]
    pub fn set_monthly_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SsmcontactsRotation_Recurrence_MonthlySettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.monthly_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.monthly_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `shift_coverages`.\n"]
    pub fn set_shift_coverages(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SsmcontactsRotation_Recurrence_ShiftCoverages>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.shift_coverages = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.shift_coverages = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `weekly_settings`.\n"]
    pub fn set_weekly_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SsmcontactsRotation_Recurrence_WeeklySettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.weekly_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.weekly_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SsmcontactsRotation_Recurrence {
    type O = terrars::BlockAssignable<SsmcontactsRotation_Recurrence>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsRotation_Recurrence {
    #[doc = ""]
    pub number_of_on_calls: terrars::PrimField<f64>,
    #[doc = ""]
    pub recurrence_multiplier: terrars::PrimField<f64>,
}

impl BuildSsmcontactsRotation_Recurrence {
    pub fn build(self) -> SsmcontactsRotation_Recurrence {
        SsmcontactsRotation_Recurrence {
            number_of_on_calls: self.number_of_on_calls,
            recurrence_multiplier: self.recurrence_multiplier,
            daily_settings: core::default::Default::default(),
            monthly_settings: core::default::Default::default(),
            shift_coverages: core::default::Default::default(),
            weekly_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsmcontactsRotation_RecurrenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmcontactsRotation_RecurrenceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SsmcontactsRotation_RecurrenceRef {
        SsmcontactsRotation_RecurrenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsRotation_RecurrenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `number_of_on_calls` after provisioning.\n"]
    pub fn number_of_on_calls(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_on_calls", self.base))
    }

    #[doc = "Get a reference to the value of field `recurrence_multiplier` after provisioning.\n"]
    pub fn recurrence_multiplier(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurrence_multiplier", self.base))
    }

    #[doc = "Get a reference to the value of field `daily_settings` after provisioning.\n"]
    pub fn daily_settings(&self) -> terrars::ListRef<SsmcontactsRotation_Recurrence_DailySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.daily_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `monthly_settings` after provisioning.\n"]
    pub fn monthly_settings(&self) -> terrars::ListRef<SsmcontactsRotation_Recurrence_MonthlySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monthly_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `shift_coverages` after provisioning.\n"]
    pub fn shift_coverages(&self) -> terrars::ListRef<SsmcontactsRotation_Recurrence_ShiftCoveragesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shift_coverages", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly_settings` after provisioning.\n"]
    pub fn weekly_settings(&self) -> terrars::ListRef<SsmcontactsRotation_Recurrence_WeeklySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_settings", self.base))
    }
}
