#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SsmcontactsPlanData_ {
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
    contact_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stage: std::option::Option<std::vec::Vec<SsmcontactsPlan_Stage>>,
    dynamic: SsmcontactsPlanDynamic,
}

struct SsmcontactsPlan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SsmcontactsPlanData_>,
}

#[derive(Clone)]
pub struct SsmcontactsPlan(std::rc::Rc<SsmcontactsPlan_>);

impl SsmcontactsPlan {
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

    #[doc = "Set the field `stage`.\n"]
    pub fn set_stage(self, v: impl std::convert::Into<terrars::BlockAssignable<SsmcontactsPlan_Stage>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stage = std::option::Option::Some(d);
            },
        }
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
    pub fn stage(&self) -> terrars::ListRef<SsmcontactsPlan_StageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stage", self.extract_ref()))
    }
}

impl terrars::Referable for SsmcontactsPlan {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SsmcontactsPlan { }

impl terrars::ToListMappable for SsmcontactsPlan {
    type O = terrars::ListRef<SsmcontactsPlanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SsmcontactsPlan_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ssmcontacts_plan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsmcontactsPlan {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub contact_id: terrars::PrimField<std::string::String>,
}

impl BuildSsmcontactsPlan {
    pub fn build(self, stack: &mut terrars::Stack) -> SsmcontactsPlan {
        let out = SsmcontactsPlan(std::rc::Rc::new(SsmcontactsPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SsmcontactsPlanData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                contact_id: self.contact_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                stage: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsmcontactsPlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmcontactsPlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SsmcontactsPlanRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
    pub fn stage(&self) -> terrars::ListRef<SsmcontactsPlan_StageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stage", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SsmcontactsPlanDynamic {
    stage: std::option::Option<terrars::DynamicBlock<SsmcontactsPlan_Stage>>,
}

#[derive(Serialize)]
pub struct SsmcontactsPlan_Stage_Target_ChannelTargetInfo {
    contact_channel_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_interval_in_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl SsmcontactsPlan_Stage_Target_ChannelTargetInfo {
    #[doc = "Set the field `retry_interval_in_minutes`.\n"]
    pub fn set_retry_interval_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retry_interval_in_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SsmcontactsPlan_Stage_Target_ChannelTargetInfo {
    type O = terrars::BlockAssignable<SsmcontactsPlan_Stage_Target_ChannelTargetInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsPlan_Stage_Target_ChannelTargetInfo {
    #[doc = ""]
    pub contact_channel_id: terrars::PrimField<std::string::String>,
}

impl BuildSsmcontactsPlan_Stage_Target_ChannelTargetInfo {
    pub fn build(self) -> SsmcontactsPlan_Stage_Target_ChannelTargetInfo {
        SsmcontactsPlan_Stage_Target_ChannelTargetInfo {
            contact_channel_id: self.contact_channel_id,
            retry_interval_in_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsmcontactsPlan_Stage_Target_ChannelTargetInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmcontactsPlan_Stage_Target_ChannelTargetInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmcontactsPlan_Stage_Target_ChannelTargetInfoRef {
        SsmcontactsPlan_Stage_Target_ChannelTargetInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsPlan_Stage_Target_ChannelTargetInfoRef {
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
pub struct SsmcontactsPlan_Stage_Target_ContactTargetInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contact_id: std::option::Option<terrars::PrimField<std::string::String>>,
    is_essential: terrars::PrimField<bool>,
}

impl SsmcontactsPlan_Stage_Target_ContactTargetInfo {
    #[doc = "Set the field `contact_id`.\n"]
    pub fn set_contact_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contact_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SsmcontactsPlan_Stage_Target_ContactTargetInfo {
    type O = terrars::BlockAssignable<SsmcontactsPlan_Stage_Target_ContactTargetInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsPlan_Stage_Target_ContactTargetInfo {
    #[doc = ""]
    pub is_essential: terrars::PrimField<bool>,
}

impl BuildSsmcontactsPlan_Stage_Target_ContactTargetInfo {
    pub fn build(self) -> SsmcontactsPlan_Stage_Target_ContactTargetInfo {
        SsmcontactsPlan_Stage_Target_ContactTargetInfo {
            contact_id: core::default::Default::default(),
            is_essential: self.is_essential,
        }
    }
}

#[allow(dead_code)]
pub struct SsmcontactsPlan_Stage_Target_ContactTargetInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmcontactsPlan_Stage_Target_ContactTargetInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmcontactsPlan_Stage_Target_ContactTargetInfoRef {
        SsmcontactsPlan_Stage_Target_ContactTargetInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsPlan_Stage_Target_ContactTargetInfoRef {
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

#[derive(Serialize, Default)]
struct SsmcontactsPlan_Stage_TargetDynamic {
    channel_target_info: std::option::Option<terrars::DynamicBlock<SsmcontactsPlan_Stage_Target_ChannelTargetInfo>>,
    contact_target_info: std::option::Option<terrars::DynamicBlock<SsmcontactsPlan_Stage_Target_ContactTargetInfo>>,
}

#[derive(Serialize)]
pub struct SsmcontactsPlan_Stage_Target {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel_target_info: std::option::Option<std::vec::Vec<SsmcontactsPlan_Stage_Target_ChannelTargetInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contact_target_info: std::option::Option<std::vec::Vec<SsmcontactsPlan_Stage_Target_ContactTargetInfo>>,
    dynamic: SsmcontactsPlan_Stage_TargetDynamic,
}

impl SsmcontactsPlan_Stage_Target {
    #[doc = "Set the field `channel_target_info`.\n"]
    pub fn set_channel_target_info(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SsmcontactsPlan_Stage_Target_ChannelTargetInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.channel_target_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.channel_target_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `contact_target_info`.\n"]
    pub fn set_contact_target_info(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SsmcontactsPlan_Stage_Target_ContactTargetInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.contact_target_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.contact_target_info = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SsmcontactsPlan_Stage_Target {
    type O = terrars::BlockAssignable<SsmcontactsPlan_Stage_Target>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsPlan_Stage_Target {}

impl BuildSsmcontactsPlan_Stage_Target {
    pub fn build(self) -> SsmcontactsPlan_Stage_Target {
        SsmcontactsPlan_Stage_Target {
            channel_target_info: core::default::Default::default(),
            contact_target_info: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsmcontactsPlan_Stage_TargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmcontactsPlan_Stage_TargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SsmcontactsPlan_Stage_TargetRef {
        SsmcontactsPlan_Stage_TargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsPlan_Stage_TargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `channel_target_info` after provisioning.\n"]
    pub fn channel_target_info(&self) -> terrars::ListRef<SsmcontactsPlan_Stage_Target_ChannelTargetInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.channel_target_info", self.base))
    }

    #[doc = "Get a reference to the value of field `contact_target_info` after provisioning.\n"]
    pub fn contact_target_info(&self) -> terrars::ListRef<SsmcontactsPlan_Stage_Target_ContactTargetInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.contact_target_info", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmcontactsPlan_StageDynamic {
    target: std::option::Option<terrars::DynamicBlock<SsmcontactsPlan_Stage_Target>>,
}

#[derive(Serialize)]
pub struct SsmcontactsPlan_Stage {
    duration_in_minutes: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<std::vec::Vec<SsmcontactsPlan_Stage_Target>>,
    dynamic: SsmcontactsPlan_StageDynamic,
}

impl SsmcontactsPlan_Stage {
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SsmcontactsPlan_Stage_Target>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SsmcontactsPlan_Stage {
    type O = terrars::BlockAssignable<SsmcontactsPlan_Stage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmcontactsPlan_Stage {
    #[doc = ""]
    pub duration_in_minutes: terrars::PrimField<f64>,
}

impl BuildSsmcontactsPlan_Stage {
    pub fn build(self) -> SsmcontactsPlan_Stage {
        SsmcontactsPlan_Stage {
            duration_in_minutes: self.duration_in_minutes,
            target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsmcontactsPlan_StageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmcontactsPlan_StageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SsmcontactsPlan_StageRef {
        SsmcontactsPlan_StageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmcontactsPlan_StageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `duration_in_minutes` after provisioning.\n"]
    pub fn duration_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::ListRef<SsmcontactsPlan_Stage_TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}
