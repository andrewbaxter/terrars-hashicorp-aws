use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataBackupReportPlanData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataBackupReportPlan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBackupReportPlanData>,
}

#[derive(Clone)]
pub struct DataBackupReportPlan(std::rc::Rc<DataBackupReportPlan_>);

impl DataBackupReportPlan {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_status` after provisioning.\n"]
    pub fn deployment_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `report_delivery_channel` after provisioning.\n"]
    pub fn report_delivery_channel(&self) -> terrars::ListRef<DataBackupReportPlanReportDeliveryChannelElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.report_delivery_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `report_setting` after provisioning.\n"]
    pub fn report_setting(&self) -> terrars::ListRef<DataBackupReportPlanReportSettingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.report_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataBackupReportPlan {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBackupReportPlan { }

impl terrars::ToListMappable for DataBackupReportPlan {
    type O = terrars::ListRef<DataBackupReportPlanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBackupReportPlan_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_backup_report_plan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBackupReportPlan {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataBackupReportPlan {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBackupReportPlan {
        let out = DataBackupReportPlan(std::rc::Rc::new(DataBackupReportPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBackupReportPlanData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBackupReportPlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupReportPlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBackupReportPlanRef {
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

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_status` after provisioning.\n"]
    pub fn deployment_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `report_delivery_channel` after provisioning.\n"]
    pub fn report_delivery_channel(&self) -> terrars::ListRef<DataBackupReportPlanReportDeliveryChannelElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.report_delivery_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `report_setting` after provisioning.\n"]
    pub fn report_setting(&self) -> terrars::ListRef<DataBackupReportPlanReportSettingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.report_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBackupReportPlanReportDeliveryChannelEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    formats: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_key_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBackupReportPlanReportDeliveryChannelEl {
    #[doc = "Set the field `formats`.\n"]
    pub fn set_formats(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.formats = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_bucket_name`.\n"]
    pub fn set_s3_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_key_prefix`.\n"]
    pub fn set_s3_key_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_key_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupReportPlanReportDeliveryChannelEl {
    type O = terrars::BlockAssignable<DataBackupReportPlanReportDeliveryChannelEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupReportPlanReportDeliveryChannelEl {}

impl BuildDataBackupReportPlanReportDeliveryChannelEl {
    pub fn build(self) -> DataBackupReportPlanReportDeliveryChannelEl {
        DataBackupReportPlanReportDeliveryChannelEl {
            formats: core::default::Default::default(),
            s3_bucket_name: core::default::Default::default(),
            s3_key_prefix: core::default::Default::default(),
        }
    }
}

pub struct DataBackupReportPlanReportDeliveryChannelElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupReportPlanReportDeliveryChannelElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBackupReportPlanReportDeliveryChannelElRef {
        DataBackupReportPlanReportDeliveryChannelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupReportPlanReportDeliveryChannelElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `formats` after provisioning.\n"]
    pub fn formats(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.formats", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_key_prefix` after provisioning.\n"]
    pub fn s3_key_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_key_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBackupReportPlanReportSettingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    framework_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_of_frameworks: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    report_template: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBackupReportPlanReportSettingEl {
    #[doc = "Set the field `framework_arns`.\n"]
    pub fn set_framework_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.framework_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_of_frameworks`.\n"]
    pub fn set_number_of_frameworks(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.number_of_frameworks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `report_template`.\n"]
    pub fn set_report_template(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.report_template = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupReportPlanReportSettingEl {
    type O = terrars::BlockAssignable<DataBackupReportPlanReportSettingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupReportPlanReportSettingEl {}

impl BuildDataBackupReportPlanReportSettingEl {
    pub fn build(self) -> DataBackupReportPlanReportSettingEl {
        DataBackupReportPlanReportSettingEl {
            framework_arns: core::default::Default::default(),
            number_of_frameworks: core::default::Default::default(),
            report_template: core::default::Default::default(),
        }
    }
}

pub struct DataBackupReportPlanReportSettingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupReportPlanReportSettingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBackupReportPlanReportSettingElRef {
        DataBackupReportPlanReportSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupReportPlanReportSettingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `framework_arns` after provisioning.\n"]
    pub fn framework_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.framework_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `number_of_frameworks` after provisioning.\n"]
    pub fn number_of_frameworks(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_frameworks", self.base))
    }

    #[doc = "Get a reference to the value of field `report_template` after provisioning.\n"]
    pub fn report_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.report_template", self.base))
    }
}
