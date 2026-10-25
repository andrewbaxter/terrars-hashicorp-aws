#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSsmcontactsRotationData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataSsmcontactsRotation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSsmcontactsRotationData_>,
}

#[derive(Clone)]
pub struct DataSsmcontactsRotation(std::rc::Rc<DataSsmcontactsRotation_>);

impl DataSsmcontactsRotation {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Get a reference to the value of field `recurrence` after provisioning.\n"]
    pub fn recurrence(&self) -> terrars::ListRef<DataSsmcontactsRotation_RecurrenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recurrence", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `time_zone_id` after provisioning.\n"]
    pub fn time_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataSsmcontactsRotation {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSsmcontactsRotation { }

impl terrars::ToListMappable for DataSsmcontactsRotation {
    type O = terrars::ListRef<DataSsmcontactsRotationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSsmcontactsRotation_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ssmcontacts_rotation".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSsmcontactsRotation {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildDataSsmcontactsRotation {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSsmcontactsRotation {
        let out = DataSsmcontactsRotation(std::rc::Rc::new(DataSsmcontactsRotation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSsmcontactsRotationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                arn: self.arn,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSsmcontactsRotationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmcontactsRotationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSsmcontactsRotationRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `recurrence` after provisioning.\n"]
    pub fn recurrence(&self) -> terrars::ListRef<DataSsmcontactsRotation_RecurrenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recurrence", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `time_zone_id` after provisioning.\n"]
    pub fn time_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSsmcontactsRotation_Recurrence_DailySettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hour_of_day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minute_of_hour: std::option::Option<terrars::PrimField<f64>>,
}

impl DataSsmcontactsRotation_Recurrence_DailySettings {
    #[doc = "Set the field `hour_of_day`.\n"]
    pub fn set_hour_of_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hour_of_day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minute_of_hour`.\n"]
    pub fn set_minute_of_hour(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minute_of_hour = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmcontactsRotation_Recurrence_DailySettings {
    type O = terrars::BlockAssignable<DataSsmcontactsRotation_Recurrence_DailySettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsRotation_Recurrence_DailySettings {}

impl BuildDataSsmcontactsRotation_Recurrence_DailySettings {
    pub fn build(self) -> DataSsmcontactsRotation_Recurrence_DailySettings {
        DataSsmcontactsRotation_Recurrence_DailySettings {
            hour_of_day: core::default::Default::default(),
            minute_of_hour: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsmcontactsRotation_Recurrence_DailySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmcontactsRotation_Recurrence_DailySettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSsmcontactsRotation_Recurrence_DailySettingsRef {
        DataSsmcontactsRotation_Recurrence_DailySettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsRotation_Recurrence_DailySettingsRef {
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
pub struct DataSsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hour_of_day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minute_of_hour: std::option::Option<terrars::PrimField<f64>>,
}

impl DataSsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime {
    #[doc = "Set the field `hour_of_day`.\n"]
    pub fn set_hour_of_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hour_of_day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minute_of_hour`.\n"]
    pub fn set_minute_of_hour(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minute_of_hour = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime {
    type O = terrars::BlockAssignable<DataSsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime {}

impl BuildDataSsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime {
    pub fn build(self) -> DataSsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime {
        DataSsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime {
            hour_of_day: core::default::Default::default(),
            minute_of_hour: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsmcontactsRotation_Recurrence_WeeklySettings_HandOffTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmcontactsRotation_Recurrence_WeeklySettings_HandOffTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSsmcontactsRotation_Recurrence_WeeklySettings_HandOffTimeRef {
        DataSsmcontactsRotation_Recurrence_WeeklySettings_HandOffTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsRotation_Recurrence_WeeklySettings_HandOffTimeRef {
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
pub struct DataSsmcontactsRotation_Recurrence_MonthlySettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day_of_month: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hand_off_time: std
    ::option
    ::Option<terrars::ListField<DataSsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime>>,
}

impl DataSsmcontactsRotation_Recurrence_MonthlySettings {
    #[doc = "Set the field `day_of_month`.\n"]
    pub fn set_day_of_month(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.day_of_month = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hand_off_time`.\n"]
    pub fn set_hand_off_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime>>,
    ) -> Self {
        self.hand_off_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmcontactsRotation_Recurrence_MonthlySettings {
    type O = terrars::BlockAssignable<DataSsmcontactsRotation_Recurrence_MonthlySettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsRotation_Recurrence_MonthlySettings {}

impl BuildDataSsmcontactsRotation_Recurrence_MonthlySettings {
    pub fn build(self) -> DataSsmcontactsRotation_Recurrence_MonthlySettings {
        DataSsmcontactsRotation_Recurrence_MonthlySettings {
            day_of_month: core::default::Default::default(),
            hand_off_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsmcontactsRotation_Recurrence_MonthlySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmcontactsRotation_Recurrence_MonthlySettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSsmcontactsRotation_Recurrence_MonthlySettingsRef {
        DataSsmcontactsRotation_Recurrence_MonthlySettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsRotation_Recurrence_MonthlySettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_month` after provisioning.\n"]
    pub fn day_of_month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_month", self.base))
    }

    #[doc = "Get a reference to the value of field `hand_off_time` after provisioning.\n"]
    pub fn hand_off_time(&self) -> terrars::ListRef<DataSsmcontactsRotation_Recurrence_WeeklySettings_HandOffTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hand_off_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_End {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hour_of_day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minute_of_hour: std::option::Option<terrars::PrimField<f64>>,
}

impl DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_End {
    #[doc = "Set the field `hour_of_day`.\n"]
    pub fn set_hour_of_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hour_of_day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minute_of_hour`.\n"]
    pub fn set_minute_of_hour(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minute_of_hour = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_End {
    type O = terrars::BlockAssignable<DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_End>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_End {}

impl BuildDataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_End {
    pub fn build(self) -> DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_End {
        DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_End {
            hour_of_day: core::default::Default::default(),
            minute_of_hour: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_EndRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_EndRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_EndRef {
        DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_EndRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_EndRef {
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
pub struct DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_Start {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hour_of_day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minute_of_hour: std::option::Option<terrars::PrimField<f64>>,
}

impl DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_Start {
    #[doc = "Set the field `hour_of_day`.\n"]
    pub fn set_hour_of_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hour_of_day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minute_of_hour`.\n"]
    pub fn set_minute_of_hour(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minute_of_hour = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_Start {
    type O = terrars::BlockAssignable<DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_Start>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_Start {}

impl BuildDataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_Start {
    pub fn build(self) -> DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_Start {
        DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_Start {
            hour_of_day: core::default::Default::default(),
            minute_of_hour: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_StartRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_StartRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_StartRef {
        DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_StartRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_StartRef {
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
pub struct DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::ListField<DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_End>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std
    ::option
    ::Option<terrars::ListField<DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_Start>>,
}

impl DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_End>>,
    ) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_Start>>,
    ) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes {
    type O = terrars::BlockAssignable<DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes {}

impl BuildDataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes {
    pub fn build(self) -> DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes {
        DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimesRef {
        DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::ListRef<DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_EndRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::ListRef<DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes_StartRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmcontactsRotation_Recurrence_ShiftCoverages {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coverage_times: std
    ::option
    ::Option<terrars::ListField<DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    map_block_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSsmcontactsRotation_Recurrence_ShiftCoverages {
    #[doc = "Set the field `coverage_times`.\n"]
    pub fn set_coverage_times(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimes>>,
    ) -> Self {
        self.coverage_times = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `map_block_key`.\n"]
    pub fn set_map_block_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.map_block_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmcontactsRotation_Recurrence_ShiftCoverages {
    type O = terrars::BlockAssignable<DataSsmcontactsRotation_Recurrence_ShiftCoverages>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsRotation_Recurrence_ShiftCoverages {}

impl BuildDataSsmcontactsRotation_Recurrence_ShiftCoverages {
    pub fn build(self) -> DataSsmcontactsRotation_Recurrence_ShiftCoverages {
        DataSsmcontactsRotation_Recurrence_ShiftCoverages {
            coverage_times: core::default::Default::default(),
            map_block_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsmcontactsRotation_Recurrence_ShiftCoveragesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmcontactsRotation_Recurrence_ShiftCoveragesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSsmcontactsRotation_Recurrence_ShiftCoveragesRef {
        DataSsmcontactsRotation_Recurrence_ShiftCoveragesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsRotation_Recurrence_ShiftCoveragesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `coverage_times` after provisioning.\n"]
    pub fn coverage_times(&self) -> terrars::ListRef<DataSsmcontactsRotation_Recurrence_ShiftCoverages_CoverageTimesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.coverage_times", self.base))
    }

    #[doc = "Get a reference to the value of field `map_block_key` after provisioning.\n"]
    pub fn map_block_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.map_block_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmcontactsRotation_Recurrence_WeeklySettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day_of_week: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hand_off_time: std
    ::option
    ::Option<terrars::ListField<DataSsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime>>,
}

impl DataSsmcontactsRotation_Recurrence_WeeklySettings {
    #[doc = "Set the field `day_of_week`.\n"]
    pub fn set_day_of_week(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.day_of_week = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hand_off_time`.\n"]
    pub fn set_hand_off_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSsmcontactsRotation_Recurrence_WeeklySettings_HandOffTime>>,
    ) -> Self {
        self.hand_off_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmcontactsRotation_Recurrence_WeeklySettings {
    type O = terrars::BlockAssignable<DataSsmcontactsRotation_Recurrence_WeeklySettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsRotation_Recurrence_WeeklySettings {}

impl BuildDataSsmcontactsRotation_Recurrence_WeeklySettings {
    pub fn build(self) -> DataSsmcontactsRotation_Recurrence_WeeklySettings {
        DataSsmcontactsRotation_Recurrence_WeeklySettings {
            day_of_week: core::default::Default::default(),
            hand_off_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsmcontactsRotation_Recurrence_WeeklySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmcontactsRotation_Recurrence_WeeklySettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSsmcontactsRotation_Recurrence_WeeklySettingsRef {
        DataSsmcontactsRotation_Recurrence_WeeklySettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsRotation_Recurrence_WeeklySettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\n"]
    pub fn day_of_week(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `hand_off_time` after provisioning.\n"]
    pub fn hand_off_time(&self) -> terrars::ListRef<DataSsmcontactsRotation_Recurrence_WeeklySettings_HandOffTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hand_off_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmcontactsRotation_Recurrence {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    daily_settings: std::option::Option<terrars::ListField<DataSsmcontactsRotation_Recurrence_DailySettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monthly_settings: std::option::Option<terrars::ListField<DataSsmcontactsRotation_Recurrence_MonthlySettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_of_on_calls: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurrence_multiplier: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shift_coverages: std::option::Option<terrars::ListField<DataSsmcontactsRotation_Recurrence_ShiftCoverages>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_settings: std::option::Option<terrars::ListField<DataSsmcontactsRotation_Recurrence_WeeklySettings>>,
}

impl DataSsmcontactsRotation_Recurrence {
    #[doc = "Set the field `daily_settings`.\n"]
    pub fn set_daily_settings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSsmcontactsRotation_Recurrence_DailySettings>>,
    ) -> Self {
        self.daily_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monthly_settings`.\n"]
    pub fn set_monthly_settings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSsmcontactsRotation_Recurrence_MonthlySettings>>,
    ) -> Self {
        self.monthly_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_of_on_calls`.\n"]
    pub fn set_number_of_on_calls(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.number_of_on_calls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `recurrence_multiplier`.\n"]
    pub fn set_recurrence_multiplier(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.recurrence_multiplier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shift_coverages`.\n"]
    pub fn set_shift_coverages(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSsmcontactsRotation_Recurrence_ShiftCoverages>>,
    ) -> Self {
        self.shift_coverages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weekly_settings`.\n"]
    pub fn set_weekly_settings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSsmcontactsRotation_Recurrence_WeeklySettings>>,
    ) -> Self {
        self.weekly_settings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmcontactsRotation_Recurrence {
    type O = terrars::BlockAssignable<DataSsmcontactsRotation_Recurrence>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsRotation_Recurrence {}

impl BuildDataSsmcontactsRotation_Recurrence {
    pub fn build(self) -> DataSsmcontactsRotation_Recurrence {
        DataSsmcontactsRotation_Recurrence {
            daily_settings: core::default::Default::default(),
            monthly_settings: core::default::Default::default(),
            number_of_on_calls: core::default::Default::default(),
            recurrence_multiplier: core::default::Default::default(),
            shift_coverages: core::default::Default::default(),
            weekly_settings: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsmcontactsRotation_RecurrenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmcontactsRotation_RecurrenceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSsmcontactsRotation_RecurrenceRef {
        DataSsmcontactsRotation_RecurrenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsRotation_RecurrenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `daily_settings` after provisioning.\n"]
    pub fn daily_settings(&self) -> terrars::ListRef<DataSsmcontactsRotation_Recurrence_DailySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.daily_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `monthly_settings` after provisioning.\n"]
    pub fn monthly_settings(&self) -> terrars::ListRef<DataSsmcontactsRotation_Recurrence_MonthlySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monthly_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `number_of_on_calls` after provisioning.\n"]
    pub fn number_of_on_calls(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_on_calls", self.base))
    }

    #[doc = "Get a reference to the value of field `recurrence_multiplier` after provisioning.\n"]
    pub fn recurrence_multiplier(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurrence_multiplier", self.base))
    }

    #[doc = "Get a reference to the value of field `shift_coverages` after provisioning.\n"]
    pub fn shift_coverages(&self) -> terrars::ListRef<DataSsmcontactsRotation_Recurrence_ShiftCoveragesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shift_coverages", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly_settings` after provisioning.\n"]
    pub fn weekly_settings(&self) -> terrars::ListRef<DataSsmcontactsRotation_Recurrence_WeeklySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_settings", self.base))
    }
}
