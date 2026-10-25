#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Lexv2modelsSlotTypeData_ {
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
    bot_id: terrars::PrimField<std::string::String>,
    bot_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    locale_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent_slot_type_signature: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    composite_slot_type_setting: std::option::Option<std::vec::Vec<Lexv2modelsSlotType_CompositeSlotTypeSetting>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_source_setting: std::option::Option<std::vec::Vec<Lexv2modelsSlotType_ExternalSourceSetting>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slot_type_values: std::option::Option<std::vec::Vec<Lexv2modelsSlotType_SlotTypeValues>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Lexv2modelsSlotType_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_selection_setting: std::option::Option<std::vec::Vec<Lexv2modelsSlotType_ValueSelectionSetting>>,
    dynamic: Lexv2modelsSlotTypeDynamic,
}

struct Lexv2modelsSlotType_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Lexv2modelsSlotTypeData_>,
}

#[derive(Clone)]
pub struct Lexv2modelsSlotType(std::rc::Rc<Lexv2modelsSlotType_>);

impl Lexv2modelsSlotType {
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

    #[doc = "Set the field `parent_slot_type_signature`.\n"]
    pub fn set_parent_slot_type_signature(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().parent_slot_type_signature = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `composite_slot_type_setting`.\n"]
    pub fn set_composite_slot_type_setting(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsSlotType_CompositeSlotTypeSetting>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().composite_slot_type_setting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.composite_slot_type_setting = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `external_source_setting`.\n"]
    pub fn set_external_source_setting(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsSlotType_ExternalSourceSetting>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().external_source_setting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.external_source_setting = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `slot_type_values`.\n"]
    pub fn set_slot_type_values(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsSlotType_SlotTypeValues>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().slot_type_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.slot_type_values = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Lexv2modelsSlotType_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value_selection_setting`.\n"]
    pub fn set_value_selection_setting(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsSlotType_ValueSelectionSetting>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().value_selection_setting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.value_selection_setting = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `bot_id` after provisioning.\n"]
    pub fn bot_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bot_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bot_version` after provisioning.\n"]
    pub fn bot_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bot_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locale_id` after provisioning.\n"]
    pub fn locale_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locale_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_slot_type_signature` after provisioning.\n"]
    pub fn parent_slot_type_signature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_slot_type_signature", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `slot_type_id` after provisioning.\n"]
    pub fn slot_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slot_type_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `composite_slot_type_setting` after provisioning.\n"]
    pub fn composite_slot_type_setting(&self) -> terrars::ListRef<Lexv2modelsSlotType_CompositeSlotTypeSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.composite_slot_type_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_source_setting` after provisioning.\n"]
    pub fn external_source_setting(&self) -> terrars::ListRef<Lexv2modelsSlotType_ExternalSourceSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_source_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `slot_type_values` after provisioning.\n"]
    pub fn slot_type_values(&self) -> terrars::ListRef<Lexv2modelsSlotType_SlotTypeValuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slot_type_values", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Lexv2modelsSlotType_TimeoutsRef {
        Lexv2modelsSlotType_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value_selection_setting` after provisioning.\n"]
    pub fn value_selection_setting(&self) -> terrars::ListRef<Lexv2modelsSlotType_ValueSelectionSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value_selection_setting", self.extract_ref()))
    }
}

impl terrars::Referable for Lexv2modelsSlotType {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Lexv2modelsSlotType { }

impl terrars::ToListMappable for Lexv2modelsSlotType {
    type O = terrars::ListRef<Lexv2modelsSlotTypeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Lexv2modelsSlotType_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lexv2models_slot_type".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLexv2modelsSlotType {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bot_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub bot_version: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub locale_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsSlotType {
    pub fn build(self, stack: &mut terrars::Stack) -> Lexv2modelsSlotType {
        let out = Lexv2modelsSlotType(std::rc::Rc::new(Lexv2modelsSlotType_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Lexv2modelsSlotTypeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bot_id: self.bot_id,
                bot_version: self.bot_version,
                description: core::default::Default::default(),
                locale_id: self.locale_id,
                name: self.name,
                parent_slot_type_signature: core::default::Default::default(),
                region: core::default::Default::default(),
                composite_slot_type_setting: core::default::Default::default(),
                external_source_setting: core::default::Default::default(),
                slot_type_values: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                value_selection_setting: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Lexv2modelsSlotTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlotTypeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Lexv2modelsSlotTypeRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bot_id` after provisioning.\n"]
    pub fn bot_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bot_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bot_version` after provisioning.\n"]
    pub fn bot_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bot_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locale_id` after provisioning.\n"]
    pub fn locale_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locale_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_slot_type_signature` after provisioning.\n"]
    pub fn parent_slot_type_signature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_slot_type_signature", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `slot_type_id` after provisioning.\n"]
    pub fn slot_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slot_type_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `composite_slot_type_setting` after provisioning.\n"]
    pub fn composite_slot_type_setting(&self) -> terrars::ListRef<Lexv2modelsSlotType_CompositeSlotTypeSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.composite_slot_type_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_source_setting` after provisioning.\n"]
    pub fn external_source_setting(&self) -> terrars::ListRef<Lexv2modelsSlotType_ExternalSourceSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_source_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `slot_type_values` after provisioning.\n"]
    pub fn slot_type_values(&self) -> terrars::ListRef<Lexv2modelsSlotType_SlotTypeValuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slot_type_values", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Lexv2modelsSlotType_TimeoutsRef {
        Lexv2modelsSlotType_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value_selection_setting` after provisioning.\n"]
    pub fn value_selection_setting(&self) -> terrars::ListRef<Lexv2modelsSlotType_ValueSelectionSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value_selection_setting", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlotTypeDynamic {
    composite_slot_type_setting: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsSlotType_CompositeSlotTypeSetting>>,
    external_source_setting: std::option::Option<terrars::DynamicBlock<Lexv2modelsSlotType_ExternalSourceSetting>>,
    slot_type_values: std::option::Option<terrars::DynamicBlock<Lexv2modelsSlotType_SlotTypeValues>>,
    value_selection_setting: std::option::Option<terrars::DynamicBlock<Lexv2modelsSlotType_ValueSelectionSetting>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlotType_CompositeSlotTypeSetting_SubSlots {
    name: terrars::PrimField<std::string::String>,
    slot_type_id: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsSlotType_CompositeSlotTypeSetting_SubSlots { }

impl terrars::ToListMappable for Lexv2modelsSlotType_CompositeSlotTypeSetting_SubSlots {
    type O = terrars::BlockAssignable<Lexv2modelsSlotType_CompositeSlotTypeSetting_SubSlots>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlotType_CompositeSlotTypeSetting_SubSlots {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub slot_type_id: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsSlotType_CompositeSlotTypeSetting_SubSlots {
    pub fn build(self) -> Lexv2modelsSlotType_CompositeSlotTypeSetting_SubSlots {
        Lexv2modelsSlotType_CompositeSlotTypeSetting_SubSlots {
            name: self.name,
            slot_type_id: self.slot_type_id,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlotType_CompositeSlotTypeSetting_SubSlotsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlotType_CompositeSlotTypeSetting_SubSlotsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlotType_CompositeSlotTypeSetting_SubSlotsRef {
        Lexv2modelsSlotType_CompositeSlotTypeSetting_SubSlotsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlotType_CompositeSlotTypeSetting_SubSlotsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `slot_type_id` after provisioning.\n"]
    pub fn slot_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slot_type_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlotType_CompositeSlotTypeSettingDynamic {
    sub_slots: std::option::Option<terrars::DynamicBlock<Lexv2modelsSlotType_CompositeSlotTypeSetting_SubSlots>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlotType_CompositeSlotTypeSetting {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sub_slots: std::option::Option<std::vec::Vec<Lexv2modelsSlotType_CompositeSlotTypeSetting_SubSlots>>,
    dynamic: Lexv2modelsSlotType_CompositeSlotTypeSettingDynamic,
}

impl Lexv2modelsSlotType_CompositeSlotTypeSetting {
    #[doc = "Set the field `sub_slots`.\n"]
    pub fn set_sub_slots(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsSlotType_CompositeSlotTypeSetting_SubSlots>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sub_slots = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sub_slots = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsSlotType_CompositeSlotTypeSetting {
    type O = terrars::BlockAssignable<Lexv2modelsSlotType_CompositeSlotTypeSetting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlotType_CompositeSlotTypeSetting {}

impl BuildLexv2modelsSlotType_CompositeSlotTypeSetting {
    pub fn build(self) -> Lexv2modelsSlotType_CompositeSlotTypeSetting {
        Lexv2modelsSlotType_CompositeSlotTypeSetting {
            sub_slots: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlotType_CompositeSlotTypeSettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlotType_CompositeSlotTypeSettingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlotType_CompositeSlotTypeSettingRef {
        Lexv2modelsSlotType_CompositeSlotTypeSettingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlotType_CompositeSlotTypeSettingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sub_slots` after provisioning.\n"]
    pub fn sub_slots(&self) -> terrars::ListRef<Lexv2modelsSlotType_CompositeSlotTypeSetting_SubSlotsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sub_slots", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting_Source {
    kms_key_arn: terrars::PrimField<std::string::String>,
    s3_bucket_name: terrars::PrimField<std::string::String>,
    s3_object_key: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting_Source { }

impl terrars::ToListMappable for Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting_Source {
    type O = terrars::BlockAssignable<Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting_Source {
    #[doc = ""]
    pub kms_key_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_object_key: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting_Source {
    pub fn build(self) -> Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting_Source {
        Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting_Source {
            kms_key_arn: self.kms_key_arn,
            s3_bucket_name: self.s3_bucket_name,
            s3_object_key: self.s3_object_key,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting_SourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting_SourceRef {
        Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_object_key` after provisioning.\n"]
    pub fn s3_object_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_object_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSettingDynamic {
    source: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting_Source>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting_Source>>,
    dynamic: Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSettingDynamic,
}

impl Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting {
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting_Source>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting {
    type O = terrars::BlockAssignable<Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting {}

impl BuildLexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting {
    pub fn build(self) -> Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting {
        Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting {
            source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSettingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSettingRef {
        Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSettingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSettingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlotType_ExternalSourceSettingDynamic {
    grammar_slot_type_setting: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlotType_ExternalSourceSetting {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grammar_slot_type_setting: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting>>,
    dynamic: Lexv2modelsSlotType_ExternalSourceSettingDynamic,
}

impl Lexv2modelsSlotType_ExternalSourceSetting {
    #[doc = "Set the field `grammar_slot_type_setting`.\n"]
    pub fn set_grammar_slot_type_setting(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSetting>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.grammar_slot_type_setting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.grammar_slot_type_setting = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsSlotType_ExternalSourceSetting {
    type O = terrars::BlockAssignable<Lexv2modelsSlotType_ExternalSourceSetting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlotType_ExternalSourceSetting {}

impl BuildLexv2modelsSlotType_ExternalSourceSetting {
    pub fn build(self) -> Lexv2modelsSlotType_ExternalSourceSetting {
        Lexv2modelsSlotType_ExternalSourceSetting {
            grammar_slot_type_setting: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlotType_ExternalSourceSettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlotType_ExternalSourceSettingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsSlotType_ExternalSourceSettingRef {
        Lexv2modelsSlotType_ExternalSourceSettingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlotType_ExternalSourceSettingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grammar_slot_type_setting` after provisioning.\n"]
    pub fn grammar_slot_type_setting(
        &self,
    ) -> terrars::ListRef<Lexv2modelsSlotType_ExternalSourceSetting_GrammarSlotTypeSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grammar_slot_type_setting", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsSlotType_SlotTypeValues_SampleValue {
    value: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsSlotType_SlotTypeValues_SampleValue { }

impl terrars::ToListMappable for Lexv2modelsSlotType_SlotTypeValues_SampleValue {
    type O = terrars::BlockAssignable<Lexv2modelsSlotType_SlotTypeValues_SampleValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlotType_SlotTypeValues_SampleValue {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsSlotType_SlotTypeValues_SampleValue {
    pub fn build(self) -> Lexv2modelsSlotType_SlotTypeValues_SampleValue {
        Lexv2modelsSlotType_SlotTypeValues_SampleValue { value: self.value }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlotType_SlotTypeValues_SampleValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlotType_SlotTypeValues_SampleValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlotType_SlotTypeValues_SampleValueRef {
        Lexv2modelsSlotType_SlotTypeValues_SampleValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlotType_SlotTypeValues_SampleValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsSlotType_SlotTypeValues_Synonyms {
    value: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsSlotType_SlotTypeValues_Synonyms { }

impl terrars::ToListMappable for Lexv2modelsSlotType_SlotTypeValues_Synonyms {
    type O = terrars::BlockAssignable<Lexv2modelsSlotType_SlotTypeValues_Synonyms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlotType_SlotTypeValues_Synonyms {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsSlotType_SlotTypeValues_Synonyms {
    pub fn build(self) -> Lexv2modelsSlotType_SlotTypeValues_Synonyms {
        Lexv2modelsSlotType_SlotTypeValues_Synonyms { value: self.value }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlotType_SlotTypeValues_SynonymsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlotType_SlotTypeValues_SynonymsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlotType_SlotTypeValues_SynonymsRef {
        Lexv2modelsSlotType_SlotTypeValues_SynonymsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlotType_SlotTypeValues_SynonymsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlotType_SlotTypeValuesDynamic {
    sample_value: std::option::Option<terrars::DynamicBlock<Lexv2modelsSlotType_SlotTypeValues_SampleValue>>,
    synonyms: std::option::Option<terrars::DynamicBlock<Lexv2modelsSlotType_SlotTypeValues_Synonyms>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlotType_SlotTypeValues {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_value: std::option::Option<std::vec::Vec<Lexv2modelsSlotType_SlotTypeValues_SampleValue>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    synonyms: std::option::Option<std::vec::Vec<Lexv2modelsSlotType_SlotTypeValues_Synonyms>>,
    dynamic: Lexv2modelsSlotType_SlotTypeValuesDynamic,
}

impl Lexv2modelsSlotType_SlotTypeValues {
    #[doc = "Set the field `sample_value`.\n"]
    pub fn set_sample_value(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsSlotType_SlotTypeValues_SampleValue>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sample_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sample_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `synonyms`.\n"]
    pub fn set_synonyms(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsSlotType_SlotTypeValues_Synonyms>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.synonyms = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.synonyms = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsSlotType_SlotTypeValues {
    type O = terrars::BlockAssignable<Lexv2modelsSlotType_SlotTypeValues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlotType_SlotTypeValues {}

impl BuildLexv2modelsSlotType_SlotTypeValues {
    pub fn build(self) -> Lexv2modelsSlotType_SlotTypeValues {
        Lexv2modelsSlotType_SlotTypeValues {
            sample_value: core::default::Default::default(),
            synonyms: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlotType_SlotTypeValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlotType_SlotTypeValuesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsSlotType_SlotTypeValuesRef {
        Lexv2modelsSlotType_SlotTypeValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlotType_SlotTypeValuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sample_value` after provisioning.\n"]
    pub fn sample_value(&self) -> terrars::ListRef<Lexv2modelsSlotType_SlotTypeValues_SampleValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sample_value", self.base))
    }

    #[doc = "Get a reference to the value of field `synonyms` after provisioning.\n"]
    pub fn synonyms(&self) -> terrars::ListRef<Lexv2modelsSlotType_SlotTypeValues_SynonymsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.synonyms", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsSlotType_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Lexv2modelsSlotType_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsSlotType_Timeouts {
    type O = terrars::BlockAssignable<Lexv2modelsSlotType_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlotType_Timeouts {}

impl BuildLexv2modelsSlotType_Timeouts {
    pub fn build(self) -> Lexv2modelsSlotType_Timeouts {
        Lexv2modelsSlotType_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlotType_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlotType_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsSlotType_TimeoutsRef {
        Lexv2modelsSlotType_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlotType_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsSlotType_ValueSelectionSetting_AdvancedRecognitionSetting {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_recognition_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Lexv2modelsSlotType_ValueSelectionSetting_AdvancedRecognitionSetting {
    #[doc = "Set the field `audio_recognition_strategy`.\n"]
    pub fn set_audio_recognition_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.audio_recognition_strategy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsSlotType_ValueSelectionSetting_AdvancedRecognitionSetting {
    type O = terrars::BlockAssignable<Lexv2modelsSlotType_ValueSelectionSetting_AdvancedRecognitionSetting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlotType_ValueSelectionSetting_AdvancedRecognitionSetting {}

impl BuildLexv2modelsSlotType_ValueSelectionSetting_AdvancedRecognitionSetting {
    pub fn build(self) -> Lexv2modelsSlotType_ValueSelectionSetting_AdvancedRecognitionSetting {
        Lexv2modelsSlotType_ValueSelectionSetting_AdvancedRecognitionSetting {
            audio_recognition_strategy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlotType_ValueSelectionSetting_AdvancedRecognitionSettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlotType_ValueSelectionSetting_AdvancedRecognitionSettingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlotType_ValueSelectionSetting_AdvancedRecognitionSettingRef {
        Lexv2modelsSlotType_ValueSelectionSetting_AdvancedRecognitionSettingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlotType_ValueSelectionSetting_AdvancedRecognitionSettingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_recognition_strategy` after provisioning.\n"]
    pub fn audio_recognition_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_recognition_strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsSlotType_ValueSelectionSetting_RegexFilter {
    pattern: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsSlotType_ValueSelectionSetting_RegexFilter { }

impl terrars::ToListMappable for Lexv2modelsSlotType_ValueSelectionSetting_RegexFilter {
    type O = terrars::BlockAssignable<Lexv2modelsSlotType_ValueSelectionSetting_RegexFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlotType_ValueSelectionSetting_RegexFilter {
    #[doc = ""]
    pub pattern: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsSlotType_ValueSelectionSetting_RegexFilter {
    pub fn build(self) -> Lexv2modelsSlotType_ValueSelectionSetting_RegexFilter {
        Lexv2modelsSlotType_ValueSelectionSetting_RegexFilter { pattern: self.pattern }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlotType_ValueSelectionSetting_RegexFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlotType_ValueSelectionSetting_RegexFilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlotType_ValueSelectionSetting_RegexFilterRef {
        Lexv2modelsSlotType_ValueSelectionSetting_RegexFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlotType_ValueSelectionSetting_RegexFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlotType_ValueSelectionSettingDynamic {
    advanced_recognition_setting: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsSlotType_ValueSelectionSetting_AdvancedRecognitionSetting>>,
    regex_filter: std::option::Option<terrars::DynamicBlock<Lexv2modelsSlotType_ValueSelectionSetting_RegexFilter>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlotType_ValueSelectionSetting {
    resolution_strategy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_recognition_setting: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsSlotType_ValueSelectionSetting_AdvancedRecognitionSetting>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_filter: std::option::Option<std::vec::Vec<Lexv2modelsSlotType_ValueSelectionSetting_RegexFilter>>,
    dynamic: Lexv2modelsSlotType_ValueSelectionSettingDynamic,
}

impl Lexv2modelsSlotType_ValueSelectionSetting {
    #[doc = "Set the field `advanced_recognition_setting`.\n"]
    pub fn set_advanced_recognition_setting(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Lexv2modelsSlotType_ValueSelectionSetting_AdvancedRecognitionSetting>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.advanced_recognition_setting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_recognition_setting = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regex_filter`.\n"]
    pub fn set_regex_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsSlotType_ValueSelectionSetting_RegexFilter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regex_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regex_filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsSlotType_ValueSelectionSetting {
    type O = terrars::BlockAssignable<Lexv2modelsSlotType_ValueSelectionSetting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlotType_ValueSelectionSetting {
    #[doc = ""]
    pub resolution_strategy: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsSlotType_ValueSelectionSetting {
    pub fn build(self) -> Lexv2modelsSlotType_ValueSelectionSetting {
        Lexv2modelsSlotType_ValueSelectionSetting {
            resolution_strategy: self.resolution_strategy,
            advanced_recognition_setting: core::default::Default::default(),
            regex_filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlotType_ValueSelectionSettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlotType_ValueSelectionSettingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsSlotType_ValueSelectionSettingRef {
        Lexv2modelsSlotType_ValueSelectionSettingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlotType_ValueSelectionSettingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resolution_strategy` after provisioning.\n"]
    pub fn resolution_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resolution_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_recognition_setting` after provisioning.\n"]
    pub fn advanced_recognition_setting(
        &self,
    ) -> terrars::ListRef<Lexv2modelsSlotType_ValueSelectionSetting_AdvancedRecognitionSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_recognition_setting", self.base))
    }

    #[doc = "Get a reference to the value of field `regex_filter` after provisioning.\n"]
    pub fn regex_filter(&self) -> terrars::ListRef<Lexv2modelsSlotType_ValueSelectionSetting_RegexFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex_filter", self.base))
    }
}
