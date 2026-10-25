#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSsmcontactsPlanData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    contact_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataSsmcontactsPlan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSsmcontactsPlanData_>,
}

#[derive(Clone)]
pub struct DataSsmcontactsPlan(std::rc::Rc<DataSsmcontactsPlan_>);

impl DataSsmcontactsPlan {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `contact_id` after provisioning.\n"]
    pub fn contact_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> terrars::ListRef<DataSsmcontactsPlan_StageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stage", self.extract_ref()))
    }
}

impl terrars::Referable for DataSsmcontactsPlan {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSsmcontactsPlan { }

impl terrars::ToListMappable for DataSsmcontactsPlan {
    type O = terrars::ListRef<DataSsmcontactsPlanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSsmcontactsPlan_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ssmcontacts_plan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSsmcontactsPlan {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub contact_id: terrars::PrimField<std::string::String>,
}

impl BuildDataSsmcontactsPlan {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSsmcontactsPlan {
        let out = DataSsmcontactsPlan(std::rc::Rc::new(DataSsmcontactsPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSsmcontactsPlanData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                contact_id: self.contact_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSsmcontactsPlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmcontactsPlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSsmcontactsPlanRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `contact_id` after provisioning.\n"]
    pub fn contact_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> terrars::ListRef<DataSsmcontactsPlan_StageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stage", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSsmcontactsPlan_Stage_Target_ChannelTargetInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contact_channel_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_interval_in_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl DataSsmcontactsPlan_Stage_Target_ChannelTargetInfo {
    #[doc = "Set the field `contact_channel_id`.\n"]
    pub fn set_contact_channel_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.contact_channel_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_interval_in_minutes`.\n"]
    pub fn set_retry_interval_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retry_interval_in_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmcontactsPlan_Stage_Target_ChannelTargetInfo {
    type O = terrars::BlockAssignable<DataSsmcontactsPlan_Stage_Target_ChannelTargetInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsPlan_Stage_Target_ChannelTargetInfo {}

impl BuildDataSsmcontactsPlan_Stage_Target_ChannelTargetInfo {
    pub fn build(self) -> DataSsmcontactsPlan_Stage_Target_ChannelTargetInfo {
        DataSsmcontactsPlan_Stage_Target_ChannelTargetInfo {
            contact_channel_id: core::default::Default::default(),
            retry_interval_in_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsmcontactsPlan_Stage_Target_ChannelTargetInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmcontactsPlan_Stage_Target_ChannelTargetInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSsmcontactsPlan_Stage_Target_ChannelTargetInfoRef {
        DataSsmcontactsPlan_Stage_Target_ChannelTargetInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsPlan_Stage_Target_ChannelTargetInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `contact_channel_id` after provisioning.\n"]
    pub fn contact_channel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_channel_id", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_interval_in_minutes` after provisioning.\n"]
    pub fn retry_interval_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_interval_in_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmcontactsPlan_Stage_Target_ContactTargetInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contact_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_essential: std::option::Option<terrars::PrimField<bool>>,
}

impl DataSsmcontactsPlan_Stage_Target_ContactTargetInfo {
    #[doc = "Set the field `contact_id`.\n"]
    pub fn set_contact_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contact_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_essential`.\n"]
    pub fn set_is_essential(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_essential = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmcontactsPlan_Stage_Target_ContactTargetInfo {
    type O = terrars::BlockAssignable<DataSsmcontactsPlan_Stage_Target_ContactTargetInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsPlan_Stage_Target_ContactTargetInfo {}

impl BuildDataSsmcontactsPlan_Stage_Target_ContactTargetInfo {
    pub fn build(self) -> DataSsmcontactsPlan_Stage_Target_ContactTargetInfo {
        DataSsmcontactsPlan_Stage_Target_ContactTargetInfo {
            contact_id: core::default::Default::default(),
            is_essential: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsmcontactsPlan_Stage_Target_ContactTargetInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmcontactsPlan_Stage_Target_ContactTargetInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSsmcontactsPlan_Stage_Target_ContactTargetInfoRef {
        DataSsmcontactsPlan_Stage_Target_ContactTargetInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsPlan_Stage_Target_ContactTargetInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `contact_id` after provisioning.\n"]
    pub fn contact_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_id", self.base))
    }

    #[doc = "Get a reference to the value of field `is_essential` after provisioning.\n"]
    pub fn is_essential(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_essential", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmcontactsPlan_Stage_Target {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel_target_info: std::option::Option<terrars::ListField<DataSsmcontactsPlan_Stage_Target_ChannelTargetInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contact_target_info: std::option::Option<terrars::ListField<DataSsmcontactsPlan_Stage_Target_ContactTargetInfo>>,
}

impl DataSsmcontactsPlan_Stage_Target {
    #[doc = "Set the field `channel_target_info`.\n"]
    pub fn set_channel_target_info(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSsmcontactsPlan_Stage_Target_ChannelTargetInfo>>,
    ) -> Self {
        self.channel_target_info = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `contact_target_info`.\n"]
    pub fn set_contact_target_info(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSsmcontactsPlan_Stage_Target_ContactTargetInfo>>,
    ) -> Self {
        self.contact_target_info = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmcontactsPlan_Stage_Target {
    type O = terrars::BlockAssignable<DataSsmcontactsPlan_Stage_Target>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsPlan_Stage_Target {}

impl BuildDataSsmcontactsPlan_Stage_Target {
    pub fn build(self) -> DataSsmcontactsPlan_Stage_Target {
        DataSsmcontactsPlan_Stage_Target {
            channel_target_info: core::default::Default::default(),
            contact_target_info: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsmcontactsPlan_Stage_TargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmcontactsPlan_Stage_TargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSsmcontactsPlan_Stage_TargetRef {
        DataSsmcontactsPlan_Stage_TargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsPlan_Stage_TargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `channel_target_info` after provisioning.\n"]
    pub fn channel_target_info(&self) -> terrars::ListRef<DataSsmcontactsPlan_Stage_Target_ChannelTargetInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.channel_target_info", self.base))
    }

    #[doc = "Get a reference to the value of field `contact_target_info` after provisioning.\n"]
    pub fn contact_target_info(&self) -> terrars::ListRef<DataSsmcontactsPlan_Stage_Target_ContactTargetInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.contact_target_info", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmcontactsPlan_Stage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration_in_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::ListField<DataSsmcontactsPlan_Stage_Target>>,
}

impl DataSsmcontactsPlan_Stage {
    #[doc = "Set the field `duration_in_minutes`.\n"]
    pub fn set_duration_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.duration_in_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSsmcontactsPlan_Stage_Target>>,
    ) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmcontactsPlan_Stage {
    type O = terrars::BlockAssignable<DataSsmcontactsPlan_Stage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsPlan_Stage {}

impl BuildDataSsmcontactsPlan_Stage {
    pub fn build(self) -> DataSsmcontactsPlan_Stage {
        DataSsmcontactsPlan_Stage {
            duration_in_minutes: core::default::Default::default(),
            target: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsmcontactsPlan_StageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmcontactsPlan_StageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSsmcontactsPlan_StageRef {
        DataSsmcontactsPlan_StageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsPlan_StageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `duration_in_minutes` after provisioning.\n"]
    pub fn duration_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::ListRef<DataSsmcontactsPlan_Stage_TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}
