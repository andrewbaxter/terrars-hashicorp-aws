#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Lexv2modelsSlotData_ {
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
    intent_id: terrars::PrimField<std::string::String>,
    locale_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slot_type_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multiple_values_setting: std::option::Option<std::vec::Vec<Lexv2modelsSlot_MultipleValuesSetting>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    obfuscation_setting: std::option::Option<std::vec::Vec<Lexv2modelsSlot_ObfuscationSetting>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sub_slot_setting: std::option::Option<std::vec::Vec<Lexv2modelsSlot_SubSlotSetting>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Lexv2modelsSlot_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_elicitation_setting: std::option::Option<std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting>>,
    dynamic: Lexv2modelsSlotDynamic,
}

struct Lexv2modelsSlot_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Lexv2modelsSlotData_>,
}

#[derive(Clone)]
pub struct Lexv2modelsSlot(std::rc::Rc<Lexv2modelsSlot_>);

impl Lexv2modelsSlot {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slot_type_id`.\n"]
    pub fn set_slot_type_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().slot_type_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `multiple_values_setting`.\n"]
    pub fn set_multiple_values_setting(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsSlot_MultipleValuesSetting>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().multiple_values_setting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.multiple_values_setting = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `obfuscation_setting`.\n"]
    pub fn set_obfuscation_setting(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsSlot_ObfuscationSetting>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().obfuscation_setting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.obfuscation_setting = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sub_slot_setting`.\n"]
    pub fn set_sub_slot_setting(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsSlot_SubSlotSetting>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sub_slot_setting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sub_slot_setting = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Lexv2modelsSlot_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value_elicitation_setting`.\n"]
    pub fn set_value_elicitation_setting(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().value_elicitation_setting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.value_elicitation_setting = std::option::Option::Some(d);
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

    #[doc = "Get a reference to the value of field `intent_id` after provisioning.\n"]
    pub fn intent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.intent_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locale_id` after provisioning.\n"]
    pub fn locale_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locale_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `slot_id` after provisioning.\n"]
    pub fn slot_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slot_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `slot_type_id` after provisioning.\n"]
    pub fn slot_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slot_type_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multiple_values_setting` after provisioning.\n"]
    pub fn multiple_values_setting(&self) -> terrars::ListRef<Lexv2modelsSlot_MultipleValuesSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multiple_values_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `obfuscation_setting` after provisioning.\n"]
    pub fn obfuscation_setting(&self) -> terrars::ListRef<Lexv2modelsSlot_ObfuscationSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.obfuscation_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sub_slot_setting` after provisioning.\n"]
    pub fn sub_slot_setting(&self) -> terrars::ListRef<Lexv2modelsSlot_SubSlotSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sub_slot_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Lexv2modelsSlot_TimeoutsRef {
        Lexv2modelsSlot_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value_elicitation_setting` after provisioning.\n"]
    pub fn value_elicitation_setting(&self) -> terrars::ListRef<Lexv2modelsSlot_ValueElicitationSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value_elicitation_setting", self.extract_ref()))
    }
}

impl terrars::Referable for Lexv2modelsSlot {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Lexv2modelsSlot { }

impl terrars::ToListMappable for Lexv2modelsSlot {
    type O = terrars::ListRef<Lexv2modelsSlotRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Lexv2modelsSlot_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lexv2models_slot".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLexv2modelsSlot {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bot_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub bot_version: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub intent_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub locale_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsSlot {
    pub fn build(self, stack: &mut terrars::Stack) -> Lexv2modelsSlot {
        let out = Lexv2modelsSlot(std::rc::Rc::new(Lexv2modelsSlot_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Lexv2modelsSlotData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bot_id: self.bot_id,
                bot_version: self.bot_version,
                description: core::default::Default::default(),
                intent_id: self.intent_id,
                locale_id: self.locale_id,
                name: self.name,
                region: core::default::Default::default(),
                slot_type_id: core::default::Default::default(),
                multiple_values_setting: core::default::Default::default(),
                obfuscation_setting: core::default::Default::default(),
                sub_slot_setting: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                value_elicitation_setting: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Lexv2modelsSlotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlotRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Lexv2modelsSlotRef {
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

    #[doc = "Get a reference to the value of field `intent_id` after provisioning.\n"]
    pub fn intent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.intent_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locale_id` after provisioning.\n"]
    pub fn locale_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locale_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `slot_id` after provisioning.\n"]
    pub fn slot_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slot_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `slot_type_id` after provisioning.\n"]
    pub fn slot_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slot_type_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multiple_values_setting` after provisioning.\n"]
    pub fn multiple_values_setting(&self) -> terrars::ListRef<Lexv2modelsSlot_MultipleValuesSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multiple_values_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `obfuscation_setting` after provisioning.\n"]
    pub fn obfuscation_setting(&self) -> terrars::ListRef<Lexv2modelsSlot_ObfuscationSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.obfuscation_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sub_slot_setting` after provisioning.\n"]
    pub fn sub_slot_setting(&self) -> terrars::ListRef<Lexv2modelsSlot_SubSlotSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sub_slot_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Lexv2modelsSlot_TimeoutsRef {
        Lexv2modelsSlot_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value_elicitation_setting` after provisioning.\n"]
    pub fn value_elicitation_setting(&self) -> terrars::ListRef<Lexv2modelsSlot_ValueElicitationSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value_elicitation_setting", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlotDynamic {
    multiple_values_setting: std::option::Option<terrars::DynamicBlock<Lexv2modelsSlot_MultipleValuesSetting>>,
    obfuscation_setting: std::option::Option<terrars::DynamicBlock<Lexv2modelsSlot_ObfuscationSetting>>,
    sub_slot_setting: std::option::Option<terrars::DynamicBlock<Lexv2modelsSlot_SubSlotSetting>>,
    value_elicitation_setting: std::option::Option<terrars::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_MultipleValuesSetting {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_multiple_values: std::option::Option<terrars::PrimField<bool>>,
}

impl Lexv2modelsSlot_MultipleValuesSetting {
    #[doc = "Set the field `allow_multiple_values`.\n"]
    pub fn set_allow_multiple_values(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_multiple_values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsSlot_MultipleValuesSetting {
    type O = terrars::BlockAssignable<Lexv2modelsSlot_MultipleValuesSetting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_MultipleValuesSetting {}

impl BuildLexv2modelsSlot_MultipleValuesSetting {
    pub fn build(self) -> Lexv2modelsSlot_MultipleValuesSetting {
        Lexv2modelsSlot_MultipleValuesSetting { allow_multiple_values: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_MultipleValuesSettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlot_MultipleValuesSettingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsSlot_MultipleValuesSettingRef {
        Lexv2modelsSlot_MultipleValuesSettingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_MultipleValuesSettingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_multiple_values` after provisioning.\n"]
    pub fn allow_multiple_values(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_multiple_values", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ObfuscationSetting {
    obfuscation_setting_type: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsSlot_ObfuscationSetting { }

impl terrars::ToListMappable for Lexv2modelsSlot_ObfuscationSetting {
    type O = terrars::BlockAssignable<Lexv2modelsSlot_ObfuscationSetting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ObfuscationSetting {
    #[doc = ""]
    pub obfuscation_setting_type: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsSlot_ObfuscationSetting {
    pub fn build(self) -> Lexv2modelsSlot_ObfuscationSetting {
        Lexv2modelsSlot_ObfuscationSetting { obfuscation_setting_type: self.obfuscation_setting_type }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ObfuscationSettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlot_ObfuscationSettingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsSlot_ObfuscationSettingRef {
        Lexv2modelsSlot_ObfuscationSettingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ObfuscationSettingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `obfuscation_setting_type` after provisioning.\n"]
    pub fn obfuscation_setting_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.obfuscation_setting_type", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification_DefaultValueList {
    default_value: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification_DefaultValueList { }

impl terrars::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification_DefaultValueList {
    type O =
        terrars::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification_DefaultValueList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification_DefaultValueList {
    #[doc = ""]
    pub default_value: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification_DefaultValueList {
    pub fn build(self) -> Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification_DefaultValueList {
        Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification_DefaultValueList {
            default_value: self.default_value,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification_DefaultValueListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification_DefaultValueListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification_DefaultValueListRef {
        Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification_DefaultValueListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification_DefaultValueListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecificationDynamic {
    default_value_list: std
    ::option
    ::Option<
        terrars::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification_DefaultValueList>,
    >,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_value_list: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification_DefaultValueList>>,
    dynamic: Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecificationDynamic,
}

impl Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification {
    #[doc = "Set the field `default_value_list`.\n"]
    pub fn set_default_value_list(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification_DefaultValueList,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_value_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_value_list = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification {
    type O = terrars::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification {}

impl BuildLexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification {
    pub fn build(self) -> Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification {
        Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification {
            default_value_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecificationRef {
        Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_value_list` after provisioning.\n"]
    pub fn default_value_list(
        &self,
    ) -> terrars::ListRef<Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification_DefaultValueListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_value_list", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_CustomPayload {
    value: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_CustomPayload { }

impl terrars
::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_CustomPayload {
    type O =
        terrars
        ::BlockAssignable<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_CustomPayload,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_CustomPayload {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_CustomPayload {
    pub fn build(
        self,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_CustomPayload {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_CustomPayload {
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_CustomPayloadRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_CustomPayloadRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_CustomPayloadRef {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_CustomPayloadRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_CustomPayloadRef {
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
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard_Button {
    text: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard_Button { }

impl terrars
::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard_Button {
    type O =
        terrars
        ::BlockAssignable<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard_Button,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard_Button {
    #[doc = ""]
    pub text: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard_Button {
    pub fn build(
        self,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard_Button {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard_Button {
            text: self.text,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard_ButtonRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard_ButtonRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard_ButtonRef {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard_ButtonRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard_ButtonRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCardDynamic {
    button: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard_Button,
        >,
    >,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subtitle: std::option::Option<terrars::PrimField<std::string::String>>,
    title: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    button: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard_Button,
        >,
    >,
    dynamic: Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCardDynamic,
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard {
    #[doc = "Set the field `image_url`.\n"]
    pub fn set_image_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subtitle`.\n"]
    pub fn set_subtitle(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subtitle = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `button`.\n"]
    pub fn set_button(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard_Button,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.button = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.button = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard {
    type O =
        terrars
        ::BlockAssignable<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard {
    #[doc = ""]
    pub title: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard {
    pub fn build(
        self,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard {
            image_url: core::default::Default::default(),
            subtitle: core::default::Default::default(),
            title: self.title,
            button: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCardRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCardRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCardRef {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCardRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCardRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image_url` after provisioning.\n"]
    pub fn image_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_url", self.base))
    }

    #[doc = "Get a reference to the value of field `subtitle` after provisioning.\n"]
    pub fn subtitle(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subtitle", self.base))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }

    #[doc = "Get a reference to the value of field `button` after provisioning.\n"]
    pub fn button(
        &self,
    ) -> terrars
    ::ListRef<
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard_ButtonRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.button", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_PlainTextMessage {
    value: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_PlainTextMessage { }

impl terrars
::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_PlainTextMessage {
    type O =
        terrars
        ::BlockAssignable<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_PlainTextMessage,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_PlainTextMessage {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_PlainTextMessage {
    pub fn build(
        self,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_PlainTextMessage {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_PlainTextMessage {
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_PlainTextMessageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_PlainTextMessageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_PlainTextMessageRef {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_PlainTextMessageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_PlainTextMessageRef {
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
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_SsmlMessage {
    value: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_SsmlMessage { }

impl terrars
::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_SsmlMessage {
    type O =
        terrars
        ::BlockAssignable<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_SsmlMessage,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_SsmlMessage {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_SsmlMessage {
    pub fn build(
        self,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_SsmlMessage {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_SsmlMessage {
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_SsmlMessageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_SsmlMessageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_SsmlMessageRef {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_SsmlMessageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_SsmlMessageRef {
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
struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_MessageDynamic {
    custom_payload: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_CustomPayload>,
    >,
    image_response_card: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard,
        >,
    >,
    plain_text_message: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_PlainTextMessage,
        >,
    >,
    ssml_message: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_SsmlMessage>,
    >,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_payload: std
    ::option
    ::Option<
        std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_CustomPayload>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_response_card: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plain_text_message: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_PlainTextMessage>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssml_message: std
    ::option
    ::Option<
        std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_SsmlMessage>,
    >,
    dynamic: Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_MessageDynamic,
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message {
    #[doc = "Set the field `custom_payload`.\n"]
    pub fn set_custom_payload(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_CustomPayload,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_payload = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_payload = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `image_response_card`.\n"]
    pub fn set_image_response_card(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.image_response_card = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.image_response_card = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `plain_text_message`.\n"]
    pub fn set_plain_text_message(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_PlainTextMessage,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.plain_text_message = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.plain_text_message = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ssml_message`.\n"]
    pub fn set_ssml_message(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_SsmlMessage,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ssml_message = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ssml_message = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message {
    type O =
        terrars::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message {}

impl BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message {
    pub fn build(self) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message {
            custom_payload: core::default::Default::default(),
            image_response_card: core::default::Default::default(),
            plain_text_message: core::default::Default::default(),
            ssml_message: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_MessageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_MessageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_MessageRef {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_MessageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_MessageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_payload` after provisioning.\n"]
    pub fn custom_payload(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_CustomPayloadRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_payload", self.base))
    }

    #[doc = "Get a reference to the value of field `image_response_card` after provisioning.\n"]
    pub fn image_response_card(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCardRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_response_card", self.base))
    }

    #[doc = "Get a reference to the value of field `plain_text_message` after provisioning.\n"]
    pub fn plain_text_message(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_PlainTextMessageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.plain_text_message", self.base))
    }

    #[doc = "Get a reference to the value of field `ssml_message` after provisioning.\n"]
    pub fn ssml_message(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_SsmlMessageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssml_message", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_VariationDynamic {
    custom_payload: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_CustomPayload>,
    >,
    image_response_card: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard,
        >,
    >,
    plain_text_message: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_PlainTextMessage,
        >,
    >,
    ssml_message: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_SsmlMessage>,
    >,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Variation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_payload: std
    ::option
    ::Option<
        std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_CustomPayload>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_response_card: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plain_text_message: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_PlainTextMessage>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssml_message: std
    ::option
    ::Option<
        std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_SsmlMessage>,
    >,
    dynamic: Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_VariationDynamic,
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Variation {
    #[doc = "Set the field `custom_payload`.\n"]
    pub fn set_custom_payload(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_CustomPayload,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_payload = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_payload = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `image_response_card`.\n"]
    pub fn set_image_response_card(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCard,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.image_response_card = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.image_response_card = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `plain_text_message`.\n"]
    pub fn set_plain_text_message(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_PlainTextMessage,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.plain_text_message = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.plain_text_message = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ssml_message`.\n"]
    pub fn set_ssml_message(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_SsmlMessage,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ssml_message = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ssml_message = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Variation {
    type O =
        terrars::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Variation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Variation {}

impl BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Variation {
    pub fn build(self) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Variation {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Variation {
            custom_payload: core::default::Default::default(),
            image_response_card: core::default::Default::default(),
            plain_text_message: core::default::Default::default(),
            ssml_message: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_VariationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_VariationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_VariationRef {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_VariationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_VariationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_payload` after provisioning.\n"]
    pub fn custom_payload(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_CustomPayloadRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_payload", self.base))
    }

    #[doc = "Get a reference to the value of field `image_response_card` after provisioning.\n"]
    pub fn image_response_card(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_ImageResponseCardRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_response_card", self.base))
    }

    #[doc = "Get a reference to the value of field `plain_text_message` after provisioning.\n"]
    pub fn plain_text_message(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_PlainTextMessageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.plain_text_message", self.base))
    }

    #[doc = "Get a reference to the value of field `ssml_message` after provisioning.\n"]
    pub fn ssml_message(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message_SsmlMessageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssml_message", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroupDynamic {
    message: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message>>,
    variation: std
    ::option
    ::Option<
        terrars::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Variation>,
    >,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    variation: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Variation>>,
    dynamic: Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroupDynamic,
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Message,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.message = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.message = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `variation`.\n"]
    pub fn set_variation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_Variation,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.variation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.variation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup {
    type O = terrars::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup {}

impl BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup {
    pub fn build(self) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup {
            message: core::default::Default::default(),
            variation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroupRef {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(
        &self,
    ) -> terrars::ListRef<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_MessageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `variation` after provisioning.\n"]
    pub fn variation(
        &self,
    ) -> terrars::ListRef<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup_VariationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.variation", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes {
    allow_audio_input: terrars::PrimField<bool>,
    allow_dtmf_input: terrars::PrimField<bool>,
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes { }

impl terrars
::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes {
    type O =
        terrars
        ::BlockAssignable<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes {
    #[doc = ""]
    pub allow_audio_input: terrars::PrimField<bool>,
    #[doc = ""]
    pub allow_dtmf_input: terrars::PrimField<bool>,
}

impl BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes {
    pub fn build(
        self,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes {
            allow_audio_input: self.allow_audio_input,
            allow_dtmf_input: self.allow_dtmf_input,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypesRef {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_audio_input` after provisioning.\n"]
    pub fn allow_audio_input(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_audio_input", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_dtmf_input` after provisioning.\n"]
    pub fn allow_dtmf_input(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_dtmf_input", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification {
    end_timeout_ms: terrars::PrimField<f64>,
    max_length_ms: terrars::PrimField<f64>,
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification {

}

impl terrars
::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification {
    type O =
        terrars
        ::BlockAssignable<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification {
    #[doc = ""]
    pub end_timeout_ms: terrars::PrimField<f64>,
    #[doc = ""]
    pub max_length_ms: terrars::PrimField<f64>,
}

impl BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification {
    pub fn build(
        self,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification {
            end_timeout_ms: self.end_timeout_ms,
            max_length_ms: self.max_length_ms,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecificationRef {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_timeout_ms` after provisioning.\n"]
    pub fn end_timeout_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_timeout_ms", self.base))
    }

    #[doc = "Get a reference to the value of field `max_length_ms` after provisioning.\n"]
    pub fn max_length_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_length_ms", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification {
    deletion_character: terrars::PrimField<std::string::String>,
    end_character: terrars::PrimField<std::string::String>,
    end_timeout_ms: terrars::PrimField<f64>,
    max_length: terrars::PrimField<f64>,
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification {

}

impl terrars
::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification {
    type O =
        terrars
        ::BlockAssignable<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification {
    #[doc = ""]
    pub deletion_character: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub end_character: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub end_timeout_ms: terrars::PrimField<f64>,
    #[doc = ""]
    pub max_length: terrars::PrimField<f64>,
}

impl BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification {
    pub fn build(
        self,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification {
            deletion_character: self.deletion_character,
            end_character: self.end_character,
            end_timeout_ms: self.end_timeout_ms,
            max_length: self.max_length,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecificationRef {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deletion_character` after provisioning.\n"]
    pub fn deletion_character(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_character", self.base))
    }

    #[doc = "Get a reference to the value of field `end_character` after provisioning.\n"]
    pub fn end_character(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_character", self.base))
    }

    #[doc = "Get a reference to the value of field `end_timeout_ms` after provisioning.\n"]
    pub fn end_timeout_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_timeout_ms", self.base))
    }

    #[doc = "Get a reference to the value of field `max_length` after provisioning.\n"]
    pub fn max_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_length", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecificationDynamic {
    audio_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification,
        >,
    >,
    dtmf_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification,
        >,
    >,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification {
    start_timeout_ms: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dtmf_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification,
        >,
    >,
    dynamic: Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecificationDynamic,
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification {
    #[doc = "Set the field `audio_specification`.\n"]
    pub fn set_audio_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audio_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dtmf_specification`.\n"]
    pub fn set_dtmf_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dtmf_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dtmf_specification = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification {
    type O =
        terrars
        ::BlockAssignable<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification {
    #[doc = ""]
    pub start_timeout_ms: terrars::PrimField<f64>,
}

impl BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification {
    pub fn build(
        self,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification {
            start_timeout_ms: self.start_timeout_ms,
            audio_specification: core::default::Default::default(),
            dtmf_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecificationRef {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `start_timeout_ms` after provisioning.\n"]
    pub fn start_timeout_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_timeout_ms", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_specification` after provisioning.\n"]
    pub fn audio_specification(
        &self,
    ) -> terrars
    ::ListRef<
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecificationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `dtmf_specification` after provisioning.\n"]
    pub fn dtmf_specification(
        &self,
    ) -> terrars
    ::ListRef<
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecificationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dtmf_specification", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification {
    start_timeout_ms: terrars::PrimField<f64>,
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification {

}

impl terrars
::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification {
    type O =
        terrars
        ::BlockAssignable<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification {
    #[doc = ""]
    pub start_timeout_ms: terrars::PrimField<f64>,
}

impl BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification {
    pub fn build(
        self,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification {
            start_timeout_ms: self.start_timeout_ms,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecificationRef {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `start_timeout_ms` after provisioning.\n"]
    pub fn start_timeout_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_timeout_ms", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecificationDynamic {
    allowed_input_types: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes,
        >,
    >,
    audio_and_dtmf_input_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification,
        >,
    >,
    text_input_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification,
        >,
    >,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_interrupt: std::option::Option<terrars::PrimField<bool>>,
    map_block_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_input_types: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_and_dtmf_input_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_input_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification,
        >,
    >,
    dynamic: Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecificationDynamic,
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification {
    #[doc = "Set the field `allow_interrupt`.\n"]
    pub fn set_allow_interrupt(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_interrupt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_input_types`.\n"]
    pub fn set_allowed_input_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allowed_input_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_input_types = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `audio_and_dtmf_input_specification`.\n"]
    pub fn set_audio_and_dtmf_input_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audio_and_dtmf_input_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_and_dtmf_input_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text_input_specification`.\n"]
    pub fn set_text_input_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text_input_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text_input_specification = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification {
    type O =
        terrars
        ::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification {
    #[doc = ""]
    pub map_block_key: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification {
    pub fn build(self) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification {
            allow_interrupt: core::default::Default::default(),
            map_block_key: self.map_block_key,
            allowed_input_types: core::default::Default::default(),
            audio_and_dtmf_input_specification: core::default::Default::default(),
            text_input_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecificationRef {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"]
    pub fn allow_interrupt(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_interrupt", self.base))
    }

    #[doc = "Get a reference to the value of field `map_block_key` after provisioning.\n"]
    pub fn map_block_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.map_block_key", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_input_types` after provisioning.\n"]
    pub fn allowed_input_types(
        &self,
    ) -> terrars
    ::ListRef<
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_input_types", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_and_dtmf_input_specification` after provisioning.\n"]
    pub fn audio_and_dtmf_input_specification(
        &self,
    ) -> terrars
    ::ListRef<
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecificationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_and_dtmf_input_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `text_input_specification` after provisioning.\n"]
    pub fn text_input_specification(
        &self,
    ) -> terrars
    ::ListRef<
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecificationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_input_specification", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecificationDynamic {
    message_group: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup>>,
    prompt_attempts_specification: std
    ::option
    ::Option<
        terrars::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification>,
    >,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_interrupt: std::option::Option<terrars::PrimField<bool>>,
    max_retries: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_selection_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_group: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt_attempts_specification: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification>>,
    dynamic: Lexv2modelsSlot_ValueElicitationSetting_PromptSpecificationDynamic,
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification {
    #[doc = "Set the field `allow_interrupt`.\n"]
    pub fn set_allow_interrupt(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_interrupt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message_selection_strategy`.\n"]
    pub fn set_message_selection_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.message_selection_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message_group`.\n"]
    pub fn set_message_group(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.message_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.message_group = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `prompt_attempts_specification`.\n"]
    pub fn set_prompt_attempts_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_PromptAttemptsSpecification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.prompt_attempts_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.prompt_attempts_specification = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification {
    type O = terrars::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification {
    #[doc = ""]
    pub max_retries: terrars::PrimField<f64>,
}

impl BuildLexv2modelsSlot_ValueElicitationSetting_PromptSpecification {
    pub fn build(self) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification {
            allow_interrupt: core::default::Default::default(),
            max_retries: self.max_retries,
            message_selection_strategy: core::default::Default::default(),
            message_group: core::default::Default::default(),
            prompt_attempts_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_PromptSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlot_ValueElicitationSetting_PromptSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_PromptSpecificationRef {
        Lexv2modelsSlot_ValueElicitationSetting_PromptSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_PromptSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"]
    pub fn allow_interrupt(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_interrupt", self.base))
    }

    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }

    #[doc = "Get a reference to the value of field `message_selection_strategy` after provisioning.\n"]
    pub fn message_selection_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_selection_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `message_group` after provisioning.\n"]
    pub fn message_group(
        &self,
    ) -> terrars::ListRef<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_group", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_SampleUtterance {
    utterance: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsSlot_ValueElicitationSetting_SampleUtterance { }

impl terrars::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_SampleUtterance {
    type O = terrars::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_SampleUtterance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_SampleUtterance {
    #[doc = ""]
    pub utterance: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsSlot_ValueElicitationSetting_SampleUtterance {
    pub fn build(self) -> Lexv2modelsSlot_ValueElicitationSetting_SampleUtterance {
        Lexv2modelsSlot_ValueElicitationSetting_SampleUtterance { utterance: self.utterance }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_SampleUtteranceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlot_ValueElicitationSetting_SampleUtteranceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_SampleUtteranceRef {
        Lexv2modelsSlot_ValueElicitationSetting_SampleUtteranceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_SampleUtteranceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `utterance` after provisioning.\n"]
    pub fn utterance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.utterance", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_ContinueResponseDynamic {
    message_group: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_ContinueResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_interrupt: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_group: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup>>,
    dynamic: Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_ContinueResponseDynamic,
}

impl Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_ContinueResponse {
    #[doc = "Set the field `allow_interrupt`.\n"]
    pub fn set_allow_interrupt(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_interrupt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message_group`.\n"]
    pub fn set_message_group(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.message_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.message_group = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_ContinueResponse {
    type O =
        terrars
        ::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_ContinueResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_ContinueResponse {}

impl BuildLexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_ContinueResponse {
    pub fn build(self) -> Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_ContinueResponse {
        Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_ContinueResponse {
            allow_interrupt: core::default::Default::default(),
            message_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_ContinueResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_ContinueResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_ContinueResponseRef {
        Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_ContinueResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_ContinueResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"]
    pub fn allow_interrupt(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_interrupt", self.base))
    }

    #[doc = "Get a reference to the value of field `message_group` after provisioning.\n"]
    pub fn message_group(
        &self,
    ) -> terrars::ListRef<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_group", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_StillWaitingResponseDynamic {
    message_group: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_StillWaitingResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_interrupt: std::option::Option<terrars::PrimField<bool>>,
    frequency_in_seconds: terrars::PrimField<f64>,
    timeout_in_seconds: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_group: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup>>,
    dynamic: Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_StillWaitingResponseDynamic,
}

impl Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_StillWaitingResponse {
    #[doc = "Set the field `allow_interrupt`.\n"]
    pub fn set_allow_interrupt(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_interrupt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message_group`.\n"]
    pub fn set_message_group(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.message_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.message_group = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_StillWaitingResponse {
    type O =
        terrars
        ::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_StillWaitingResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_StillWaitingResponse {
    #[doc = ""]
    pub frequency_in_seconds: terrars::PrimField<f64>,
    #[doc = ""]
    pub timeout_in_seconds: terrars::PrimField<f64>,
}

impl BuildLexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_StillWaitingResponse {
    pub fn build(self) -> Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_StillWaitingResponse {
        Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_StillWaitingResponse {
            allow_interrupt: core::default::Default::default(),
            frequency_in_seconds: self.frequency_in_seconds,
            timeout_in_seconds: self.timeout_in_seconds,
            message_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_StillWaitingResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_StillWaitingResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_StillWaitingResponseRef {
        Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_StillWaitingResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_StillWaitingResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"]
    pub fn allow_interrupt(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_interrupt", self.base))
    }

    #[doc = "Get a reference to the value of field `frequency_in_seconds` after provisioning.\n"]
    pub fn frequency_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_in_seconds` after provisioning.\n"]
    pub fn timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `message_group` after provisioning.\n"]
    pub fn message_group(
        &self,
    ) -> terrars::ListRef<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_group", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_WaitingResponseDynamic {
    message_group: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_WaitingResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_interrupt: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_group: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup>>,
    dynamic: Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_WaitingResponseDynamic,
}

impl Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_WaitingResponse {
    #[doc = "Set the field `allow_interrupt`.\n"]
    pub fn set_allow_interrupt(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_interrupt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message_group`.\n"]
    pub fn set_message_group(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroup>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.message_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.message_group = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_WaitingResponse {
    type O =
        terrars
        ::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_WaitingResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_WaitingResponse {}

impl BuildLexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_WaitingResponse {
    pub fn build(self) -> Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_WaitingResponse {
        Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_WaitingResponse {
            allow_interrupt: core::default::Default::default(),
            message_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_WaitingResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_WaitingResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_WaitingResponseRef {
        Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_WaitingResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_WaitingResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"]
    pub fn allow_interrupt(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_interrupt", self.base))
    }

    #[doc = "Get a reference to the value of field `message_group` after provisioning.\n"]
    pub fn message_group(
        &self,
    ) -> terrars::ListRef<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification_MessageGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_group", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecificationDynamic {
    continue_response: std
    ::option
    ::Option<
        terrars::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_ContinueResponse>,
    >,
    still_waiting_response: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_StillWaitingResponse>,
    >,
    waiting_response: std
    ::option
    ::Option<
        terrars::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_WaitingResponse>,
    >,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    continue_response: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_ContinueResponse>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    still_waiting_response: std
    ::option
    ::Option<
        std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_StillWaitingResponse>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    waiting_response: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_WaitingResponse>>,
    dynamic: Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecificationDynamic,
}

impl Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification {
    #[doc = "Set the field `active`.\n"]
    pub fn set_active(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.active = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `continue_response`.\n"]
    pub fn set_continue_response(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_ContinueResponse,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.continue_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.continue_response = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `still_waiting_response`.\n"]
    pub fn set_still_waiting_response(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_StillWaitingResponse,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.still_waiting_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.still_waiting_response = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `waiting_response`.\n"]
    pub fn set_waiting_response(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_WaitingResponse,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.waiting_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.waiting_response = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification {
    type O = terrars::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification {}

impl BuildLexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification {
    pub fn build(self) -> Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification {
        Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification {
            active: core::default::Default::default(),
            continue_response: core::default::Default::default(),
            still_waiting_response: core::default::Default::default(),
            waiting_response: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecificationRef {
        Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.base))
    }

    #[doc = "Get a reference to the value of field `continue_response` after provisioning.\n"]
    pub fn continue_response(
        &self,
    ) -> terrars::ListRef<Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_ContinueResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.continue_response", self.base))
    }

    #[doc = "Get a reference to the value of field `still_waiting_response` after provisioning.\n"]
    pub fn still_waiting_response(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_StillWaitingResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.still_waiting_response", self.base))
    }

    #[doc = "Get a reference to the value of field `waiting_response` after provisioning.\n"]
    pub fn waiting_response(
        &self,
    ) -> terrars::ListRef<Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification_WaitingResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.waiting_response", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlot_SubSlotSetting_SlotSpecification_ValueElicitationSettingDynamic {
    default_value_specification: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification>>,
    prompt_specification: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification>>,
    sample_utterance: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_SampleUtterance>>,
    wait_and_continue_specification: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_SubSlotSetting_SlotSpecification_ValueElicitationSetting {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_value_specification: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt_specification: std::option::Option<std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_utterance: std::option::Option<std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_SampleUtterance>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_and_continue_specification: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification>>,
    dynamic: Lexv2modelsSlot_SubSlotSetting_SlotSpecification_ValueElicitationSettingDynamic,
}

impl Lexv2modelsSlot_SubSlotSetting_SlotSpecification_ValueElicitationSetting {
    #[doc = "Set the field `default_value_specification`.\n"]
    pub fn set_default_value_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_value_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_value_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `prompt_specification`.\n"]
    pub fn set_prompt_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.prompt_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.prompt_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sample_utterance`.\n"]
    pub fn set_sample_utterance(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_SampleUtterance>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sample_utterance = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sample_utterance = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `wait_and_continue_specification`.\n"]
    pub fn set_wait_and_continue_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.wait_and_continue_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.wait_and_continue_specification = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsSlot_SubSlotSetting_SlotSpecification_ValueElicitationSetting {
    type O = terrars::BlockAssignable<Lexv2modelsSlot_SubSlotSetting_SlotSpecification_ValueElicitationSetting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_SubSlotSetting_SlotSpecification_ValueElicitationSetting {}

impl BuildLexv2modelsSlot_SubSlotSetting_SlotSpecification_ValueElicitationSetting {
    pub fn build(self) -> Lexv2modelsSlot_SubSlotSetting_SlotSpecification_ValueElicitationSetting {
        Lexv2modelsSlot_SubSlotSetting_SlotSpecification_ValueElicitationSetting {
            default_value_specification: core::default::Default::default(),
            prompt_specification: core::default::Default::default(),
            sample_utterance: core::default::Default::default(),
            wait_and_continue_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_SubSlotSetting_SlotSpecification_ValueElicitationSettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlot_SubSlotSetting_SlotSpecification_ValueElicitationSettingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_SubSlotSetting_SlotSpecification_ValueElicitationSettingRef {
        Lexv2modelsSlot_SubSlotSetting_SlotSpecification_ValueElicitationSettingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_SubSlotSetting_SlotSpecification_ValueElicitationSettingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_value_specification` after provisioning.\n"]
    pub fn default_value_specification(
        &self,
    ) -> terrars::ListRef<Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_value_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `prompt_specification` after provisioning.\n"]
    pub fn prompt_specification(
        &self,
    ) -> terrars::ListRef<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prompt_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `sample_utterance` after provisioning.\n"]
    pub fn sample_utterance(&self) -> terrars::ListRef<Lexv2modelsSlot_ValueElicitationSetting_SampleUtteranceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sample_utterance", self.base))
    }

    #[doc = "Get a reference to the value of field `wait_and_continue_specification` after provisioning.\n"]
    pub fn wait_and_continue_specification(
        &self,
    ) -> terrars::ListRef<Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.wait_and_continue_specification", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlot_SubSlotSetting_SlotSpecificationDynamic {
    value_elicitation_setting: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsSlot_SubSlotSetting_SlotSpecification_ValueElicitationSetting>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_SubSlotSetting_SlotSpecification {
    map_block_key: terrars::PrimField<std::string::String>,
    slot_type_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_elicitation_setting: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsSlot_SubSlotSetting_SlotSpecification_ValueElicitationSetting>>,
    dynamic: Lexv2modelsSlot_SubSlotSetting_SlotSpecificationDynamic,
}

impl Lexv2modelsSlot_SubSlotSetting_SlotSpecification {
    #[doc = "Set the field `value_elicitation_setting`.\n"]
    pub fn set_value_elicitation_setting(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Lexv2modelsSlot_SubSlotSetting_SlotSpecification_ValueElicitationSetting>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.value_elicitation_setting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.value_elicitation_setting = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsSlot_SubSlotSetting_SlotSpecification {
    type O = terrars::BlockAssignable<Lexv2modelsSlot_SubSlotSetting_SlotSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_SubSlotSetting_SlotSpecification {
    #[doc = ""]
    pub map_block_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub slot_type_id: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsSlot_SubSlotSetting_SlotSpecification {
    pub fn build(self) -> Lexv2modelsSlot_SubSlotSetting_SlotSpecification {
        Lexv2modelsSlot_SubSlotSetting_SlotSpecification {
            map_block_key: self.map_block_key,
            slot_type_id: self.slot_type_id,
            value_elicitation_setting: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_SubSlotSetting_SlotSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlot_SubSlotSetting_SlotSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_SubSlotSetting_SlotSpecificationRef {
        Lexv2modelsSlot_SubSlotSetting_SlotSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_SubSlotSetting_SlotSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `map_block_key` after provisioning.\n"]
    pub fn map_block_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.map_block_key", self.base))
    }

    #[doc = "Get a reference to the value of field `slot_type_id` after provisioning.\n"]
    pub fn slot_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slot_type_id", self.base))
    }

    #[doc = "Get a reference to the value of field `value_elicitation_setting` after provisioning.\n"]
    pub fn value_elicitation_setting(
        &self,
    ) -> terrars::ListRef<Lexv2modelsSlot_SubSlotSetting_SlotSpecification_ValueElicitationSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value_elicitation_setting", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlot_SubSlotSettingDynamic {
    slot_specification: std::option::Option<terrars::DynamicBlock<Lexv2modelsSlot_SubSlotSetting_SlotSpecification>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_SubSlotSetting {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slot_specification: std::option::Option<std::vec::Vec<Lexv2modelsSlot_SubSlotSetting_SlotSpecification>>,
    dynamic: Lexv2modelsSlot_SubSlotSettingDynamic,
}

impl Lexv2modelsSlot_SubSlotSetting {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slot_specification`.\n"]
    pub fn set_slot_specification(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsSlot_SubSlotSetting_SlotSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.slot_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.slot_specification = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsSlot_SubSlotSetting {
    type O = terrars::BlockAssignable<Lexv2modelsSlot_SubSlotSetting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_SubSlotSetting {}

impl BuildLexv2modelsSlot_SubSlotSetting {
    pub fn build(self) -> Lexv2modelsSlot_SubSlotSetting {
        Lexv2modelsSlot_SubSlotSetting {
            expression: core::default::Default::default(),
            slot_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_SubSlotSettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlot_SubSlotSettingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsSlot_SubSlotSettingRef {
        Lexv2modelsSlot_SubSlotSettingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_SubSlotSettingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Lexv2modelsSlot_Timeouts {
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

impl terrars::ToListMappable for Lexv2modelsSlot_Timeouts {
    type O = terrars::BlockAssignable<Lexv2modelsSlot_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_Timeouts {}

impl BuildLexv2modelsSlot_Timeouts {
    pub fn build(self) -> Lexv2modelsSlot_Timeouts {
        Lexv2modelsSlot_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlot_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsSlot_TimeoutsRef {
        Lexv2modelsSlot_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_TimeoutsRef {
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
pub struct Lexv2modelsSlot_ValueElicitationSetting_SlotResolutionSetting {
    slot_resolution_strategy: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsSlot_ValueElicitationSetting_SlotResolutionSetting { }

impl terrars::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting_SlotResolutionSetting {
    type O = terrars::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_SlotResolutionSetting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting_SlotResolutionSetting {
    #[doc = ""]
    pub slot_resolution_strategy: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsSlot_ValueElicitationSetting_SlotResolutionSetting {
    pub fn build(self) -> Lexv2modelsSlot_ValueElicitationSetting_SlotResolutionSetting {
        Lexv2modelsSlot_ValueElicitationSetting_SlotResolutionSetting {
            slot_resolution_strategy: self.slot_resolution_strategy,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSetting_SlotResolutionSettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlot_ValueElicitationSetting_SlotResolutionSettingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsSlot_ValueElicitationSetting_SlotResolutionSettingRef {
        Lexv2modelsSlot_ValueElicitationSetting_SlotResolutionSettingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSetting_SlotResolutionSettingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `slot_resolution_strategy` after provisioning.\n"]
    pub fn slot_resolution_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slot_resolution_strategy", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsSlot_ValueElicitationSettingDynamic {
    default_value_specification: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification>>,
    prompt_specification: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification>>,
    sample_utterance: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_SampleUtterance>>,
    slot_resolution_setting: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_SlotResolutionSetting>>,
    wait_and_continue_specification: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsSlot_ValueElicitationSetting {
    slot_constraint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_value_specification: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt_specification: std::option::Option<std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_utterance: std::option::Option<std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_SampleUtterance>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slot_resolution_setting: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_SlotResolutionSetting>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_and_continue_specification: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification>>,
    dynamic: Lexv2modelsSlot_ValueElicitationSettingDynamic,
}

impl Lexv2modelsSlot_ValueElicitationSetting {
    #[doc = "Set the field `default_value_specification`.\n"]
    pub fn set_default_value_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecification>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_value_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_value_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `prompt_specification`.\n"]
    pub fn set_prompt_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.prompt_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.prompt_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sample_utterance`.\n"]
    pub fn set_sample_utterance(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_SampleUtterance>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sample_utterance = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sample_utterance = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `slot_resolution_setting`.\n"]
    pub fn set_slot_resolution_setting(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_SlotResolutionSetting>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.slot_resolution_setting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.slot_resolution_setting = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `wait_and_continue_specification`.\n"]
    pub fn set_wait_and_continue_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecification>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.wait_and_continue_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.wait_and_continue_specification = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsSlot_ValueElicitationSetting {
    type O = terrars::BlockAssignable<Lexv2modelsSlot_ValueElicitationSetting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsSlot_ValueElicitationSetting {
    #[doc = ""]
    pub slot_constraint: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsSlot_ValueElicitationSetting {
    pub fn build(self) -> Lexv2modelsSlot_ValueElicitationSetting {
        Lexv2modelsSlot_ValueElicitationSetting {
            slot_constraint: self.slot_constraint,
            default_value_specification: core::default::Default::default(),
            prompt_specification: core::default::Default::default(),
            sample_utterance: core::default::Default::default(),
            slot_resolution_setting: core::default::Default::default(),
            wait_and_continue_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsSlot_ValueElicitationSettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsSlot_ValueElicitationSettingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsSlot_ValueElicitationSettingRef {
        Lexv2modelsSlot_ValueElicitationSettingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsSlot_ValueElicitationSettingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `slot_constraint` after provisioning.\n"]
    pub fn slot_constraint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slot_constraint", self.base))
    }

    #[doc = "Get a reference to the value of field `default_value_specification` after provisioning.\n"]
    pub fn default_value_specification(
        &self,
    ) -> terrars::ListRef<Lexv2modelsSlot_ValueElicitationSetting_DefaultValueSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_value_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `prompt_specification` after provisioning.\n"]
    pub fn prompt_specification(
        &self,
    ) -> terrars::ListRef<Lexv2modelsSlot_ValueElicitationSetting_PromptSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prompt_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `sample_utterance` after provisioning.\n"]
    pub fn sample_utterance(&self) -> terrars::ListRef<Lexv2modelsSlot_ValueElicitationSetting_SampleUtteranceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sample_utterance", self.base))
    }

    #[doc = "Get a reference to the value of field `slot_resolution_setting` after provisioning.\n"]
    pub fn slot_resolution_setting(
        &self,
    ) -> terrars::ListRef<Lexv2modelsSlot_ValueElicitationSetting_SlotResolutionSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slot_resolution_setting", self.base))
    }

    #[doc = "Get a reference to the value of field `wait_and_continue_specification` after provisioning.\n"]
    pub fn wait_and_continue_specification(
        &self,
    ) -> terrars::ListRef<Lexv2modelsSlot_ValueElicitationSetting_WaitAndContinueSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.wait_and_continue_specification", self.base))
    }
}
