#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataConnectHoursOfOperationData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours_of_operation_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataConnectHoursOfOperation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataConnectHoursOfOperationData_>,
}

#[derive(Clone)]
pub struct DataConnectHoursOfOperation(std::rc::Rc<DataConnectHoursOfOperation_>);

impl DataConnectHoursOfOperation {
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

    #[doc = "Set the field `hours_of_operation_id`.\n"]
    pub fn set_hours_of_operation_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().hours_of_operation_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::SetRef<DataConnectHoursOfOperation_ConfigRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hours_of_operation_id` after provisioning.\n"]
    pub fn hours_of_operation_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours_of_operation_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.extract_ref()))
    }
}

impl terrars::Referable for DataConnectHoursOfOperation {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataConnectHoursOfOperation { }

impl terrars::ToListMappable for DataConnectHoursOfOperation {
    type O = terrars::ListRef<DataConnectHoursOfOperationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataConnectHoursOfOperation_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_connect_hours_of_operation".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataConnectHoursOfOperation {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub instance_id: terrars::PrimField<std::string::String>,
}

impl BuildDataConnectHoursOfOperation {
    pub fn build(self, stack: &mut terrars::Stack) -> DataConnectHoursOfOperation {
        let out = DataConnectHoursOfOperation(std::rc::Rc::new(DataConnectHoursOfOperation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataConnectHoursOfOperationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                hours_of_operation_id: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataConnectHoursOfOperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectHoursOfOperationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataConnectHoursOfOperationRef {
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

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::SetRef<DataConnectHoursOfOperation_ConfigRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hours_of_operation_id` after provisioning.\n"]
    pub fn hours_of_operation_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours_of_operation_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataConnectHoursOfOperation_Config_EndTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl DataConnectHoursOfOperation_Config_EndTime {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectHoursOfOperation_Config_EndTime {
    type O = terrars::BlockAssignable<DataConnectHoursOfOperation_Config_EndTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectHoursOfOperation_Config_EndTime {}

impl BuildDataConnectHoursOfOperation_Config_EndTime {
    pub fn build(self) -> DataConnectHoursOfOperation_Config_EndTime {
        DataConnectHoursOfOperation_Config_EndTime {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataConnectHoursOfOperation_Config_EndTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectHoursOfOperation_Config_EndTimeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataConnectHoursOfOperation_Config_EndTimeRef {
        DataConnectHoursOfOperation_Config_EndTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectHoursOfOperation_Config_EndTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectHoursOfOperation_Config_StartTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl DataConnectHoursOfOperation_Config_StartTime {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectHoursOfOperation_Config_StartTime {
    type O = terrars::BlockAssignable<DataConnectHoursOfOperation_Config_StartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectHoursOfOperation_Config_StartTime {}

impl BuildDataConnectHoursOfOperation_Config_StartTime {
    pub fn build(self) -> DataConnectHoursOfOperation_Config_StartTime {
        DataConnectHoursOfOperation_Config_StartTime {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataConnectHoursOfOperation_Config_StartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectHoursOfOperation_Config_StartTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectHoursOfOperation_Config_StartTimeRef {
        DataConnectHoursOfOperation_Config_StartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectHoursOfOperation_Config_StartTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectHoursOfOperation_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::ListField<DataConnectHoursOfOperation_Config_EndTime>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::ListField<DataConnectHoursOfOperation_Config_StartTime>>,
}

impl DataConnectHoursOfOperation_Config {
    #[doc = "Set the field `day`.\n"]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataConnectHoursOfOperation_Config_EndTime>>,
    ) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataConnectHoursOfOperation_Config_StartTime>>,
    ) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectHoursOfOperation_Config {
    type O = terrars::BlockAssignable<DataConnectHoursOfOperation_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectHoursOfOperation_Config {}

impl BuildDataConnectHoursOfOperation_Config {
    pub fn build(self) -> DataConnectHoursOfOperation_Config {
        DataConnectHoursOfOperation_Config {
            day: core::default::Default::default(),
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataConnectHoursOfOperation_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectHoursOfOperation_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataConnectHoursOfOperation_ConfigRef {
        DataConnectHoursOfOperation_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectHoursOfOperation_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day` after provisioning.\n"]
    pub fn day(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::ListRef<DataConnectHoursOfOperation_Config_EndTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::ListRef<DataConnectHoursOfOperation_Config_StartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
