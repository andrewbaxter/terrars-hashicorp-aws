#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Lexv2modelsIntentData_ {
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
    parent_intent_signature: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    closing_setting: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confirmation_setting: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dialog_code_hook: std::option::Option<std::vec::Vec<Lexv2modelsIntent_DialogCodeHook>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fulfillment_code_hook: std::option::Option<std::vec::Vec<Lexv2modelsIntent_FulfillmentCodeHook>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_response_setting: std::option::Option<std::vec::Vec<Lexv2modelsIntent_InitialResponseSetting>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_context: std::option::Option<std::vec::Vec<Lexv2modelsIntent_InputContext>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kendra_configuration: std::option::Option<std::vec::Vec<Lexv2modelsIntent_KendraConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_context: std::option::Option<std::vec::Vec<Lexv2modelsIntent_OutputContext>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qna_intent_configuration: std::option::Option<std::vec::Vec<Lexv2modelsIntent_QnaIntentConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_utterance: std::option::Option<std::vec::Vec<Lexv2modelsIntent_SampleUtterance>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slot_priority: std::option::Option<std::vec::Vec<Lexv2modelsIntent_SlotPriority>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Lexv2modelsIntent_Timeouts>,
    dynamic: Lexv2modelsIntentDynamic,
}

struct Lexv2modelsIntent_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Lexv2modelsIntentData_>,
}

#[derive(Clone)]
pub struct Lexv2modelsIntent(std::rc::Rc<Lexv2modelsIntent_>);

impl Lexv2modelsIntent {
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

    #[doc = "Set the field `parent_intent_signature`.\n"]
    pub fn set_parent_intent_signature(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().parent_intent_signature = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `closing_setting`.\n"]
    pub fn set_closing_setting(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().closing_setting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.closing_setting = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `confirmation_setting`.\n"]
    pub fn set_confirmation_setting(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().confirmation_setting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.confirmation_setting = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dialog_code_hook`.\n"]
    pub fn set_dialog_code_hook(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_DialogCodeHook>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dialog_code_hook = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dialog_code_hook = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fulfillment_code_hook`.\n"]
    pub fn set_fulfillment_code_hook(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_FulfillmentCodeHook>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fulfillment_code_hook = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fulfillment_code_hook = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `initial_response_setting`.\n"]
    pub fn set_initial_response_setting(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_InitialResponseSetting>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().initial_response_setting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.initial_response_setting = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_context`.\n"]
    pub fn set_input_context(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_InputContext>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input_context = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input_context = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kendra_configuration`.\n"]
    pub fn set_kendra_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_KendraConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kendra_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kendra_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_context`.\n"]
    pub fn set_output_context(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_OutputContext>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().output_context = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.output_context = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `qna_intent_configuration`.\n"]
    pub fn set_qna_intent_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_QnaIntentConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().qna_intent_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.qna_intent_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sample_utterance`.\n"]
    pub fn set_sample_utterance(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_SampleUtterance>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sample_utterance = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sample_utterance = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `slot_priority`.\n"]
    pub fn set_slot_priority(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_SlotPriority>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().slot_priority = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.slot_priority = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Lexv2modelsIntent_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `creation_date_time` after provisioning.\n"]
    pub fn creation_date_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_date_time", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `last_updated_date_time` after provisioning.\n"]
    pub fn last_updated_date_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locale_id` after provisioning.\n"]
    pub fn locale_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locale_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_intent_signature` after provisioning.\n"]
    pub fn parent_intent_signature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_intent_signature", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `closing_setting` after provisioning.\n"]
    pub fn closing_setting(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.closing_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confirmation_setting` after provisioning.\n"]
    pub fn confirmation_setting(&self) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confirmation_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dialog_code_hook` after provisioning.\n"]
    pub fn dialog_code_hook(&self) -> terrars::ListRef<Lexv2modelsIntent_DialogCodeHookRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dialog_code_hook", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fulfillment_code_hook` after provisioning.\n"]
    pub fn fulfillment_code_hook(&self) -> terrars::ListRef<Lexv2modelsIntent_FulfillmentCodeHookRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fulfillment_code_hook", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initial_response_setting` after provisioning.\n"]
    pub fn initial_response_setting(&self) -> terrars::ListRef<Lexv2modelsIntent_InitialResponseSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initial_response_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_context` after provisioning.\n"]
    pub fn input_context(&self) -> terrars::ListRef<Lexv2modelsIntent_InputContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kendra_configuration` after provisioning.\n"]
    pub fn kendra_configuration(&self) -> terrars::ListRef<Lexv2modelsIntent_KendraConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kendra_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_context` after provisioning.\n"]
    pub fn output_context(&self) -> terrars::ListRef<Lexv2modelsIntent_OutputContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qna_intent_configuration` after provisioning.\n"]
    pub fn qna_intent_configuration(&self) -> terrars::ListRef<Lexv2modelsIntent_QnaIntentConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.qna_intent_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sample_utterance` after provisioning.\n"]
    pub fn sample_utterance(&self) -> terrars::ListRef<Lexv2modelsIntent_SampleUtteranceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sample_utterance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `slot_priority` after provisioning.\n"]
    pub fn slot_priority(&self) -> terrars::ListRef<Lexv2modelsIntent_SlotPriorityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slot_priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Lexv2modelsIntent_TimeoutsRef {
        Lexv2modelsIntent_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for Lexv2modelsIntent {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Lexv2modelsIntent { }

impl terrars::ToListMappable for Lexv2modelsIntent {
    type O = terrars::ListRef<Lexv2modelsIntentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Lexv2modelsIntent_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lexv2models_intent".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLexv2modelsIntent {
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

impl BuildLexv2modelsIntent {
    pub fn build(self, stack: &mut terrars::Stack) -> Lexv2modelsIntent {
        let out = Lexv2modelsIntent(std::rc::Rc::new(Lexv2modelsIntent_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Lexv2modelsIntentData_ {
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
                parent_intent_signature: core::default::Default::default(),
                region: core::default::Default::default(),
                closing_setting: core::default::Default::default(),
                confirmation_setting: core::default::Default::default(),
                dialog_code_hook: core::default::Default::default(),
                fulfillment_code_hook: core::default::Default::default(),
                initial_response_setting: core::default::Default::default(),
                input_context: core::default::Default::default(),
                kendra_configuration: core::default::Default::default(),
                output_context: core::default::Default::default(),
                qna_intent_configuration: core::default::Default::default(),
                sample_utterance: core::default::Default::default(),
                slot_priority: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Lexv2modelsIntentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Lexv2modelsIntentRef {
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

    #[doc = "Get a reference to the value of field `creation_date_time` after provisioning.\n"]
    pub fn creation_date_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_date_time", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `last_updated_date_time` after provisioning.\n"]
    pub fn last_updated_date_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locale_id` after provisioning.\n"]
    pub fn locale_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locale_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_intent_signature` after provisioning.\n"]
    pub fn parent_intent_signature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_intent_signature", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `closing_setting` after provisioning.\n"]
    pub fn closing_setting(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.closing_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confirmation_setting` after provisioning.\n"]
    pub fn confirmation_setting(&self) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confirmation_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dialog_code_hook` after provisioning.\n"]
    pub fn dialog_code_hook(&self) -> terrars::ListRef<Lexv2modelsIntent_DialogCodeHookRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dialog_code_hook", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fulfillment_code_hook` after provisioning.\n"]
    pub fn fulfillment_code_hook(&self) -> terrars::ListRef<Lexv2modelsIntent_FulfillmentCodeHookRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fulfillment_code_hook", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initial_response_setting` after provisioning.\n"]
    pub fn initial_response_setting(&self) -> terrars::ListRef<Lexv2modelsIntent_InitialResponseSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initial_response_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_context` after provisioning.\n"]
    pub fn input_context(&self) -> terrars::ListRef<Lexv2modelsIntent_InputContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kendra_configuration` after provisioning.\n"]
    pub fn kendra_configuration(&self) -> terrars::ListRef<Lexv2modelsIntent_KendraConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kendra_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_context` after provisioning.\n"]
    pub fn output_context(&self) -> terrars::ListRef<Lexv2modelsIntent_OutputContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qna_intent_configuration` after provisioning.\n"]
    pub fn qna_intent_configuration(&self) -> terrars::ListRef<Lexv2modelsIntent_QnaIntentConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.qna_intent_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sample_utterance` after provisioning.\n"]
    pub fn sample_utterance(&self) -> terrars::ListRef<Lexv2modelsIntent_SampleUtteranceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sample_utterance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `slot_priority` after provisioning.\n"]
    pub fn slot_priority(&self) -> terrars::ListRef<Lexv2modelsIntent_SlotPriorityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slot_priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Lexv2modelsIntent_TimeoutsRef {
        Lexv2modelsIntent_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntentDynamic {
    closing_setting: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting>>,
    confirmation_setting: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting>>,
    dialog_code_hook: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_DialogCodeHook>>,
    fulfillment_code_hook: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_FulfillmentCodeHook>>,
    initial_response_setting: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_InitialResponseSetting>>,
    input_context: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_InputContext>>,
    kendra_configuration: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_KendraConfiguration>>,
    output_context: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_OutputContext>>,
    qna_intent_configuration: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_QnaIntentConfiguration>>,
    sample_utterance: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_SampleUtterance>>,
    slot_priority: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_SlotPriority>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_CustomPayload {
    value: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_CustomPayload { }

impl terrars::ToListMappable for Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_CustomPayload {
    type O =
        terrars
        ::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_CustomPayload>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_CustomPayload {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_CustomPayload {
    pub fn build(self) -> Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_CustomPayload {
        Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_CustomPayload { value: self.value }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_CustomPayloadRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_CustomPayloadRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_CustomPayloadRef {
        Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_CustomPayloadRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_CustomPayloadRef {
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
pub struct Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard_Button {
    text: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard_Button { }

impl terrars
::ToListMappable for Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard_Button {
    type O =
        terrars
        ::BlockAssignable<
            Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard_Button,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard_Button {
    #[doc = ""]
    pub text: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard_Button {
    pub fn build(
        self,
    ) -> Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard_Button {
        Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard_Button {
            text: self.text,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard_ButtonRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard_ButtonRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard_ButtonRef {
        Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard_ButtonRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard_ButtonRef {
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
struct Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCardDynamic {
    button: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard_Button>,
    >,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subtitle: std::option::Option<terrars::PrimField<std::string::String>>,
    title: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    button: std
    ::option
    ::Option<
        std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard_Button>,
    >,
    dynamic: Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCardDynamic,
}

impl Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard {
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
                            Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard_Button,
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
::ToListMappable for Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard {
    type O =
        terrars
        ::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard {
    #[doc = ""]
    pub title: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard {
    pub fn build(self) -> Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard {
        Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard {
            image_url: core::default::Default::default(),
            subtitle: core::default::Default::default(),
            title: self.title,
            button: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCardRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCardRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCardRef {
        Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCardRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCardRef {
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
    ::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard_ButtonRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.button", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_PlainTextMessage {
    value: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_PlainTextMessage { }

impl terrars
::ToListMappable for Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_PlainTextMessage {
    type O =
        terrars
        ::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_PlainTextMessage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_PlainTextMessage {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_PlainTextMessage {
    pub fn build(self) -> Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_PlainTextMessage {
        Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_PlainTextMessage { value: self.value }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_PlainTextMessageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_PlainTextMessageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_PlainTextMessageRef {
        Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_PlainTextMessageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_PlainTextMessageRef {
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
pub struct Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_SsmlMessage {
    value: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_SsmlMessage { }

impl terrars::ToListMappable for Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_SsmlMessage {
    type O =
        terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_SsmlMessage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_SsmlMessage {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_SsmlMessage {
    pub fn build(self) -> Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_SsmlMessage {
        Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_SsmlMessage { value: self.value }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_SsmlMessageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_SsmlMessageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_SsmlMessageRef {
        Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_SsmlMessageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_SsmlMessageRef {
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
struct Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_MessageDynamic {
    custom_payload: std
    ::option
    ::Option<
        terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_CustomPayload>,
    >,
    image_response_card: std
    ::option
    ::Option<
        terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard>,
    >,
    plain_text_message: std
    ::option
    ::Option<
        terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_PlainTextMessage>,
    >,
    ssml_message: std
    ::option
    ::Option<
        terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_SsmlMessage>,
    >,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_payload: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_CustomPayload>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_response_card: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plain_text_message: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_PlainTextMessage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssml_message: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_SsmlMessage>>,
    dynamic: Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_MessageDynamic,
}

impl Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message {
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
                            Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_CustomPayload,
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
                            Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard,
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
                            Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_PlainTextMessage,
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
                            Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_SsmlMessage,
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

impl terrars::ToListMappable for Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message {}

impl BuildLexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message {
    pub fn build(self) -> Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message {
        Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message {
            custom_payload: core::default::Default::default(),
            image_response_card: core::default::Default::default(),
            plain_text_message: core::default::Default::default(),
            ssml_message: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_MessageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_MessageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_MessageRef {
        Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_MessageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_MessageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_payload` after provisioning.\n"]
    pub fn custom_payload(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_CustomPayloadRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_payload", self.base))
    }

    #[doc = "Get a reference to the value of field `image_response_card` after provisioning.\n"]
    pub fn image_response_card(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCardRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_response_card", self.base))
    }

    #[doc = "Get a reference to the value of field `plain_text_message` after provisioning.\n"]
    pub fn plain_text_message(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_PlainTextMessageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.plain_text_message", self.base))
    }

    #[doc = "Get a reference to the value of field `ssml_message` after provisioning.\n"]
    pub fn ssml_message(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_SsmlMessageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssml_message", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_VariationDynamic {
    custom_payload: std
    ::option
    ::Option<
        terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_CustomPayload>,
    >,
    image_response_card: std
    ::option
    ::Option<
        terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard>,
    >,
    plain_text_message: std
    ::option
    ::Option<
        terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_PlainTextMessage>,
    >,
    ssml_message: std
    ::option
    ::Option<
        terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_SsmlMessage>,
    >,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Variation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_payload: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_CustomPayload>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_response_card: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plain_text_message: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_PlainTextMessage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssml_message: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_SsmlMessage>>,
    dynamic: Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_VariationDynamic,
}

impl Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Variation {
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
                            Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_CustomPayload,
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
                            Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCard,
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
                            Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_PlainTextMessage,
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
                            Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_SsmlMessage,
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

impl terrars::ToListMappable for Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Variation {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Variation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Variation {}

impl BuildLexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Variation {
    pub fn build(self) -> Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Variation {
        Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Variation {
            custom_payload: core::default::Default::default(),
            image_response_card: core::default::Default::default(),
            plain_text_message: core::default::Default::default(),
            ssml_message: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_VariationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_VariationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_VariationRef {
        Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_VariationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_VariationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_payload` after provisioning.\n"]
    pub fn custom_payload(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_CustomPayloadRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_payload", self.base))
    }

    #[doc = "Get a reference to the value of field `image_response_card` after provisioning.\n"]
    pub fn image_response_card(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_ImageResponseCardRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_response_card", self.base))
    }

    #[doc = "Get a reference to the value of field `plain_text_message` after provisioning.\n"]
    pub fn plain_text_message(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_PlainTextMessageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.plain_text_message", self.base))
    }

    #[doc = "Get a reference to the value of field `ssml_message` after provisioning.\n"]
    pub fn ssml_message(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message_SsmlMessageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssml_message", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroupDynamic {
    message: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message>>,
    variation: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Variation>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    variation: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Variation>>,
    dynamic: Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroupDynamic,
}

impl Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Message>,
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
                        ::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_Variation>,
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

impl terrars::ToListMappable for Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup {}

impl BuildLexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup {
    pub fn build(self) -> Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup {
        Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup {
            message: core::default::Default::default(),
            variation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroupRef {
        Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_MessageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `variation` after provisioning.\n"]
    pub fn variation(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup_VariationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.variation", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ClosingSetting_ClosingResponseDynamic {
    message_group: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ClosingSetting_ClosingResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_interrupt: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_group: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
    dynamic: Lexv2modelsIntent_ClosingSetting_ClosingResponseDynamic,
}

impl Lexv2modelsIntent_ClosingSetting_ClosingResponse {
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
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
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

impl terrars::ToListMappable for Lexv2modelsIntent_ClosingSetting_ClosingResponse {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ClosingSetting_ClosingResponse {}

impl BuildLexv2modelsIntent_ClosingSetting_ClosingResponse {
    pub fn build(self) -> Lexv2modelsIntent_ClosingSetting_ClosingResponse {
        Lexv2modelsIntent_ClosingSetting_ClosingResponse {
            allow_interrupt: core::default::Default::default(),
            message_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ClosingSetting_ClosingResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ClosingSetting_ClosingResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ClosingSetting_ClosingResponseRef {
        Lexv2modelsIntent_ClosingSetting_ClosingResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ClosingSetting_ClosingResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"]
    pub fn allow_interrupt(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_interrupt", self.base))
    }

    #[doc = "Get a reference to the value of field `message_group` after provisioning.\n"]
    pub fn message_group(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_group", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch_Condition {
    expression_string: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch_Condition { }

impl terrars::ToListMappable for Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch_Condition {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch_Condition {
    #[doc = ""]
    pub expression_string: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch_Condition {
    pub fn build(self) -> Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch_Condition {
        Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch_Condition {
            expression_string: self.expression_string,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch_ConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch_ConditionRef {
        Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression_string` after provisioning.\n"]
    pub fn expression_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression_string", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slot_to_elicit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suppress_next_message: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction {
    #[doc = "Set the field `slot_to_elicit`.\n"]
    pub fn set_slot_to_elicit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.slot_to_elicit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `suppress_next_message`.\n"]
    pub fn set_suppress_next_message(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.suppress_next_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ClosingSetting_NextStep_DialogAction {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsIntent_ClosingSetting_NextStep_DialogAction {
    pub fn build(self) -> Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction {
        Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction {
            slot_to_elicit: core::default::Default::default(),
            suppress_next_message: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ClosingSetting_NextStep_DialogActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ClosingSetting_NextStep_DialogActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ClosingSetting_NextStep_DialogActionRef {
        Lexv2modelsIntent_ClosingSetting_NextStep_DialogActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ClosingSetting_NextStep_DialogActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `slot_to_elicit` after provisioning.\n"]
    pub fn slot_to_elicit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slot_to_elicit", self.base))
    }

    #[doc = "Get a reference to the value of field `suppress_next_message` after provisioning.\n"]
    pub fn suppress_next_message(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suppress_next_message", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot_Value {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interpreted_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot_Value {
    #[doc = "Set the field `interpreted_value`.\n"]
    pub fn set_interpreted_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interpreted_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot_Value {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot_Value>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot_Value {}

impl BuildLexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot_Value {
    pub fn build(self) -> Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot_Value {
        Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot_Value {
            interpreted_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot_ValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot_ValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot_ValueRef {
        Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot_ValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot_ValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `interpreted_value` after provisioning.\n"]
    pub fn interpreted_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interpreted_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ClosingSetting_NextStep_Intent_SlotDynamic {
    value: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot_Value>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot {
    map_block_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shape: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot_Value>>,
    dynamic: Lexv2modelsIntent_ClosingSetting_NextStep_Intent_SlotDynamic,
}

impl Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot {
    #[doc = "Set the field `shape`.\n"]
    pub fn set_shape(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.shape = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot_Value>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot {
    #[doc = ""]
    pub map_block_key: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot {
    pub fn build(self) -> Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot {
        Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot {
            map_block_key: self.map_block_key,
            shape: core::default::Default::default(),
            value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ClosingSetting_NextStep_Intent_SlotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ClosingSetting_NextStep_Intent_SlotRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ClosingSetting_NextStep_Intent_SlotRef {
        Lexv2modelsIntent_ClosingSetting_NextStep_Intent_SlotRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ClosingSetting_NextStep_Intent_SlotRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `map_block_key` after provisioning.\n"]
    pub fn map_block_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.map_block_key", self.base))
    }

    #[doc = "Get a reference to the value of field `shape` after provisioning.\n"]
    pub fn shape(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot_ValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ClosingSetting_NextStep_IntentDynamic {
    slot: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ClosingSetting_NextStep_Intent {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slot: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot>>,
    dynamic: Lexv2modelsIntent_ClosingSetting_NextStep_IntentDynamic,
}

impl Lexv2modelsIntent_ClosingSetting_NextStep_Intent {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slot`.\n"]
    pub fn set_slot(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep_Intent_Slot>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.slot = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.slot = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_ClosingSetting_NextStep_Intent {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep_Intent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ClosingSetting_NextStep_Intent {}

impl BuildLexv2modelsIntent_ClosingSetting_NextStep_Intent {
    pub fn build(self) -> Lexv2modelsIntent_ClosingSetting_NextStep_Intent {
        Lexv2modelsIntent_ClosingSetting_NextStep_Intent {
            name: core::default::Default::default(),
            slot: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ClosingSetting_NextStep_IntentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ClosingSetting_NextStep_IntentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ClosingSetting_NextStep_IntentRef {
        Lexv2modelsIntent_ClosingSetting_NextStep_IntentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ClosingSetting_NextStep_IntentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ClosingSetting_NextStepDynamic {
    dialog_action: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction>>,
    intent: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_NextStep_Intent>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ClosingSetting_NextStep {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_attributes: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dialog_action: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    intent: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_NextStep_Intent>>,
    dynamic: Lexv2modelsIntent_ClosingSetting_NextStepDynamic,
}

impl Lexv2modelsIntent_ClosingSetting_NextStep {
    #[doc = "Set the field `session_attributes`.\n"]
    pub fn set_session_attributes(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.session_attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dialog_action`.\n"]
    pub fn set_dialog_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dialog_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dialog_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `intent`.\n"]
    pub fn set_intent(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep_Intent>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.intent = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.intent = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_ClosingSetting_NextStep {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ClosingSetting_NextStep {}

impl BuildLexv2modelsIntent_ClosingSetting_NextStep {
    pub fn build(self) -> Lexv2modelsIntent_ClosingSetting_NextStep {
        Lexv2modelsIntent_ClosingSetting_NextStep {
            session_attributes: core::default::Default::default(),
            dialog_action: core::default::Default::default(),
            intent: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ClosingSetting_NextStepRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ClosingSetting_NextStepRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsIntent_ClosingSetting_NextStepRef {
        Lexv2modelsIntent_ClosingSetting_NextStepRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ClosingSetting_NextStepRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `session_attributes` after provisioning.\n"]
    pub fn session_attributes(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.session_attributes", self.base))
    }

    #[doc = "Get a reference to the value of field `dialog_action` after provisioning.\n"]
    pub fn dialog_action(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_NextStep_DialogActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dialog_action", self.base))
    }

    #[doc = "Get a reference to the value of field `intent` after provisioning.\n"]
    pub fn intent(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_NextStep_IntentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.intent", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_ResponseDynamic {
    message_group: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_Response {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_interrupt: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_group: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
    dynamic: Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_ResponseDynamic,
}

impl Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_Response {
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
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
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

impl terrars::ToListMappable for Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_Response {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_Response>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_Response {}

impl BuildLexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_Response {
    pub fn build(self) -> Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_Response {
        Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_Response {
            allow_interrupt: core::default::Default::default(),
            message_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_ResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_ResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_ResponseRef {
        Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_ResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_ResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"]
    pub fn allow_interrupt(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_interrupt", self.base))
    }

    #[doc = "Get a reference to the value of field `message_group` after provisioning.\n"]
    pub fn message_group(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_group", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranchDynamic {
    condition: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch_Condition>>,
    next_step: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_NextStep>>,
    response: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_Response>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch_Condition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_step: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_NextStep>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_Response>>,
    dynamic: Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranchDynamic,
}

impl Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch {
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch_Condition>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `next_step`.\n"]
    pub fn set_next_step(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.next_step = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.next_step = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `response`.\n"]
    pub fn set_response(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_Response>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.response = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch {
    pub fn build(self) -> Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch {
        Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch {
            name: self.name,
            condition: core::default::Default::default(),
            next_step: core::default::Default::default(),
            response: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranchRef {
        Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc = "Get a reference to the value of field `next_step` after provisioning.\n"]
    pub fn next_step(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_NextStepRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.next_step", self.base))
    }

    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_ResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranchDynamic {
    next_step: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_NextStep>>,
    response: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_Response>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_step: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_NextStep>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_Response>>,
    dynamic: Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranchDynamic,
}

impl Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch {
    #[doc = "Set the field `next_step`.\n"]
    pub fn set_next_step(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.next_step = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.next_step = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `response`.\n"]
    pub fn set_response(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_Response>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.response = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch {}

impl BuildLexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch {
    pub fn build(self) -> Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch {
        Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch {
            next_step: core::default::Default::default(),
            response: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranchRef {
        Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `next_step` after provisioning.\n"]
    pub fn next_step(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_NextStepRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.next_step", self.base))
    }

    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch_ResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ClosingSetting_ConditionalDynamic {
    conditional_branch: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch>>,
    default_branch: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ClosingSetting_Conditional {
    active: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditional_branch: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_branch: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch>>,
    dynamic: Lexv2modelsIntent_ClosingSetting_ConditionalDynamic,
}

impl Lexv2modelsIntent_ClosingSetting_Conditional {
    #[doc = "Set the field `conditional_branch`.\n"]
    pub fn set_conditional_branch(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditional_branch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional_branch = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_branch`.\n"]
    pub fn set_default_branch(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_branch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_branch = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_ClosingSetting_Conditional {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ClosingSetting_Conditional {
    #[doc = ""]
    pub active: terrars::PrimField<bool>,
}

impl BuildLexv2modelsIntent_ClosingSetting_Conditional {
    pub fn build(self) -> Lexv2modelsIntent_ClosingSetting_Conditional {
        Lexv2modelsIntent_ClosingSetting_Conditional {
            active: self.active,
            conditional_branch: core::default::Default::default(),
            default_branch: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ClosingSetting_ConditionalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ClosingSetting_ConditionalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ClosingSetting_ConditionalRef {
        Lexv2modelsIntent_ClosingSetting_ConditionalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ClosingSetting_ConditionalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.base))
    }

    #[doc = "Get a reference to the value of field `conditional_branch` after provisioning.\n"]
    pub fn conditional_branch(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditional_branch", self.base))
    }

    #[doc = "Get a reference to the value of field `default_branch` after provisioning.\n"]
    pub fn default_branch(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_branch", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ClosingSettingDynamic {
    closing_response: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse>>,
    conditional: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_Conditional>>,
    next_step: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_NextStep>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ClosingSetting {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    closing_response: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditional: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_Conditional>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_step: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_NextStep>>,
    dynamic: Lexv2modelsIntent_ClosingSettingDynamic,
}

impl Lexv2modelsIntent_ClosingSetting {
    #[doc = "Set the field `active`.\n"]
    pub fn set_active(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.active = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `closing_response`.\n"]
    pub fn set_closing_response(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.closing_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.closing_response = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `conditional`.\n"]
    pub fn set_conditional(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditional = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `next_step`.\n"]
    pub fn set_next_step(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.next_step = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.next_step = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_ClosingSetting {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ClosingSetting {}

impl BuildLexv2modelsIntent_ClosingSetting {
    pub fn build(self) -> Lexv2modelsIntent_ClosingSetting {
        Lexv2modelsIntent_ClosingSetting {
            active: core::default::Default::default(),
            closing_response: core::default::Default::default(),
            conditional: core::default::Default::default(),
            next_step: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ClosingSettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ClosingSettingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsIntent_ClosingSettingRef {
        Lexv2modelsIntent_ClosingSettingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ClosingSettingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.base))
    }

    #[doc = "Get a reference to the value of field `closing_response` after provisioning.\n"]
    pub fn closing_response(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.closing_response", self.base))
    }

    #[doc = "Get a reference to the value of field `conditional` after provisioning.\n"]
    pub fn conditional(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ConditionalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditional", self.base))
    }

    #[doc = "Get a reference to the value of field `next_step` after provisioning.\n"]
    pub fn next_step(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_NextStepRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.next_step", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ConfirmationSetting_FailureConditionalDynamic {
    conditional_branch: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch>>,
    default_branch: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_FailureConditional {
    active: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditional_branch: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_branch: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch>>,
    dynamic: Lexv2modelsIntent_ConfirmationSetting_FailureConditionalDynamic,
}

impl Lexv2modelsIntent_ConfirmationSetting_FailureConditional {
    #[doc = "Set the field `conditional_branch`.\n"]
    pub fn set_conditional_branch(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditional_branch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional_branch = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_branch`.\n"]
    pub fn set_default_branch(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_branch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_branch = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_FailureConditional {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_FailureConditional>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_FailureConditional {
    #[doc = ""]
    pub active: terrars::PrimField<bool>,
}

impl BuildLexv2modelsIntent_ConfirmationSetting_FailureConditional {
    pub fn build(self) -> Lexv2modelsIntent_ConfirmationSetting_FailureConditional {
        Lexv2modelsIntent_ConfirmationSetting_FailureConditional {
            active: self.active,
            conditional_branch: core::default::Default::default(),
            default_branch: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_FailureConditionalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ConfirmationSetting_FailureConditionalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_FailureConditionalRef {
        Lexv2modelsIntent_ConfirmationSetting_FailureConditionalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_FailureConditionalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.base))
    }

    #[doc = "Get a reference to the value of field `conditional_branch` after provisioning.\n"]
    pub fn conditional_branch(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditional_branch", self.base))
    }

    #[doc = "Get a reference to the value of field `default_branch` after provisioning.\n"]
    pub fn default_branch(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_branch", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ConfirmationSetting_FailureNextStepDynamic {
    dialog_action: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction>>,
    intent: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_NextStep_Intent>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_FailureNextStep {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_attributes: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dialog_action: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    intent: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_NextStep_Intent>>,
    dynamic: Lexv2modelsIntent_ConfirmationSetting_FailureNextStepDynamic,
}

impl Lexv2modelsIntent_ConfirmationSetting_FailureNextStep {
    #[doc = "Set the field `session_attributes`.\n"]
    pub fn set_session_attributes(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.session_attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dialog_action`.\n"]
    pub fn set_dialog_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dialog_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dialog_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `intent`.\n"]
    pub fn set_intent(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep_Intent>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.intent = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.intent = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_FailureNextStep {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_FailureNextStep>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_FailureNextStep {}

impl BuildLexv2modelsIntent_ConfirmationSetting_FailureNextStep {
    pub fn build(self) -> Lexv2modelsIntent_ConfirmationSetting_FailureNextStep {
        Lexv2modelsIntent_ConfirmationSetting_FailureNextStep {
            session_attributes: core::default::Default::default(),
            dialog_action: core::default::Default::default(),
            intent: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_FailureNextStepRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ConfirmationSetting_FailureNextStepRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_FailureNextStepRef {
        Lexv2modelsIntent_ConfirmationSetting_FailureNextStepRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_FailureNextStepRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `session_attributes` after provisioning.\n"]
    pub fn session_attributes(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.session_attributes", self.base))
    }

    #[doc = "Get a reference to the value of field `dialog_action` after provisioning.\n"]
    pub fn dialog_action(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_NextStep_DialogActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dialog_action", self.base))
    }

    #[doc = "Get a reference to the value of field `intent` after provisioning.\n"]
    pub fn intent(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_NextStep_IntentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.intent", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ConfirmationSetting_FailureResponseDynamic {
    message_group: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_FailureResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_interrupt: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_group: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
    dynamic: Lexv2modelsIntent_ConfirmationSetting_FailureResponseDynamic,
}

impl Lexv2modelsIntent_ConfirmationSetting_FailureResponse {
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
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
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

impl terrars::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_FailureResponse {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_FailureResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_FailureResponse {}

impl BuildLexv2modelsIntent_ConfirmationSetting_FailureResponse {
    pub fn build(self) -> Lexv2modelsIntent_ConfirmationSetting_FailureResponse {
        Lexv2modelsIntent_ConfirmationSetting_FailureResponse {
            allow_interrupt: core::default::Default::default(),
            message_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_FailureResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ConfirmationSetting_FailureResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_FailureResponseRef {
        Lexv2modelsIntent_ConfirmationSetting_FailureResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_FailureResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"]
    pub fn allow_interrupt(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_interrupt", self.base))
    }

    #[doc = "Get a reference to the value of field `message_group` after provisioning.\n"]
    pub fn message_group(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_group", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditionalDynamic {
    conditional_branch: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch>>,
    default_branch: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditional {
    active: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditional_branch: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_branch: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch>>,
    dynamic: Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditionalDynamic,
}

impl Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditional {
    #[doc = "Set the field `conditional_branch`.\n"]
    pub fn set_conditional_branch(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditional_branch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional_branch = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_branch`.\n"]
    pub fn set_default_branch(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_branch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_branch = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditional {
    type O =
        terrars
        ::BlockAssignable<
            Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditional,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditional {
    #[doc = ""]
    pub active: terrars::PrimField<bool>,
}

impl BuildLexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditional {
    pub fn build(
        self,
    ) -> Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditional {
        Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditional {
            active: self.active,
            conditional_branch: core::default::Default::default(),
            default_branch: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditionalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditionalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditionalRef {
        Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditionalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditionalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.base))
    }

    #[doc = "Get a reference to the value of field `conditional_branch` after provisioning.\n"]
    pub fn conditional_branch(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditional_branch", self.base))
    }

    #[doc = "Get a reference to the value of field `default_branch` after provisioning.\n"]
    pub fn default_branch(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_branch", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStepDynamic {
    dialog_action: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction>>,
    intent: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_NextStep_Intent>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStep {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_attributes: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dialog_action: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    intent: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_NextStep_Intent>>,
    dynamic: Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStepDynamic,
}

impl Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStep {
    #[doc = "Set the field `session_attributes`.\n"]
    pub fn set_session_attributes(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.session_attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dialog_action`.\n"]
    pub fn set_dialog_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dialog_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dialog_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `intent`.\n"]
    pub fn set_intent(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep_Intent>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.intent = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.intent = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStep {
    type O =
        terrars
        ::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStep>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStep {}

impl BuildLexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStep {
    pub fn build(self) -> Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStep {
        Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStep {
            session_attributes: core::default::Default::default(),
            dialog_action: core::default::Default::default(),
            intent: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStepRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStepRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStepRef {
        Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStepRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStepRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `session_attributes` after provisioning.\n"]
    pub fn session_attributes(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.session_attributes", self.base))
    }

    #[doc = "Get a reference to the value of field `dialog_action` after provisioning.\n"]
    pub fn dialog_action(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_NextStep_DialogActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dialog_action", self.base))
    }

    #[doc = "Get a reference to the value of field `intent` after provisioning.\n"]
    pub fn intent(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_NextStep_IntentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.intent", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponseDynamic {
    message_group: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_interrupt: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_group: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
    dynamic: Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponseDynamic,
}

impl Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponse {
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
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
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
::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponse {
    type O =
        terrars
        ::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponse {}

impl BuildLexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponse {
    pub fn build(self) -> Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponse {
        Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponse {
            allow_interrupt: core::default::Default::default(),
            message_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponseRef {
        Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"]
    pub fn allow_interrupt(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_interrupt", self.base))
    }

    #[doc = "Get a reference to the value of field `message_group` after provisioning.\n"]
    pub fn message_group(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_group", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditionalDynamic {
    conditional_branch: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch>>,
    default_branch: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditional {
    active: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditional_branch: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_branch: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch>>,
    dynamic: Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditionalDynamic,
}

impl Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditional {
    #[doc = "Set the field `conditional_branch`.\n"]
    pub fn set_conditional_branch(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditional_branch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional_branch = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_branch`.\n"]
    pub fn set_default_branch(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_branch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_branch = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditional {
    type O =
        terrars
        ::BlockAssignable<
            Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditional,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditional {
    #[doc = ""]
    pub active: terrars::PrimField<bool>,
}

impl BuildLexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditional {
    pub fn build(
        self,
    ) -> Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditional {
        Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditional {
            active: self.active,
            conditional_branch: core::default::Default::default(),
            default_branch: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditionalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditionalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditionalRef {
        Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditionalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditionalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.base))
    }

    #[doc = "Get a reference to the value of field `conditional_branch` after provisioning.\n"]
    pub fn conditional_branch(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditional_branch", self.base))
    }

    #[doc = "Get a reference to the value of field `default_branch` after provisioning.\n"]
    pub fn default_branch(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_branch", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStepDynamic {
    dialog_action: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction>>,
    intent: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_NextStep_Intent>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStep {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_attributes: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dialog_action: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    intent: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_NextStep_Intent>>,
    dynamic: Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStepDynamic,
}

impl Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStep {
    #[doc = "Set the field `session_attributes`.\n"]
    pub fn set_session_attributes(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.session_attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dialog_action`.\n"]
    pub fn set_dialog_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dialog_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dialog_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `intent`.\n"]
    pub fn set_intent(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep_Intent>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.intent = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.intent = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStep {
    type O =
        terrars
        ::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStep>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStep {}

impl BuildLexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStep {
    pub fn build(self) -> Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStep {
        Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStep {
            session_attributes: core::default::Default::default(),
            dialog_action: core::default::Default::default(),
            intent: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStepRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStepRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStepRef {
        Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStepRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStepRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `session_attributes` after provisioning.\n"]
    pub fn session_attributes(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.session_attributes", self.base))
    }

    #[doc = "Get a reference to the value of field `dialog_action` after provisioning.\n"]
    pub fn dialog_action(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_NextStep_DialogActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dialog_action", self.base))
    }

    #[doc = "Get a reference to the value of field `intent` after provisioning.\n"]
    pub fn intent(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_NextStep_IntentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.intent", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponseDynamic {
    message_group: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_interrupt: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_group: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
    dynamic: Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponseDynamic,
}

impl Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponse {
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
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
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
::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponse {
    type O =
        terrars
        ::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponse {}

impl BuildLexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponse {
    pub fn build(self) -> Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponse {
        Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponse {
            allow_interrupt: core::default::Default::default(),
            message_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponseRef {
        Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"]
    pub fn allow_interrupt(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_interrupt", self.base))
    }

    #[doc = "Get a reference to the value of field `message_group` after provisioning.\n"]
    pub fn message_group(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_group", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecificationDynamic {
    failure_conditional: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_FailureConditional>>,
    failure_next_step: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_FailureNextStep>>,
    failure_response: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_FailureResponse>>,
    success_conditional: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditional>,
    >,
    success_next_step: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStep>,
    >,
    success_response: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponse>,
    >,
    timeout_conditional: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditional>,
    >,
    timeout_next_step: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStep>,
    >,
    timeout_response: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponse>,
    >,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_conditional: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_FailureConditional>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_next_step: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_FailureNextStep>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_response: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_FailureResponse>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_conditional: std
    ::option
    ::Option<
        std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditional>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_next_step: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStep>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_response: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponse>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_conditional: std
    ::option
    ::Option<
        std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditional>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_next_step: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStep>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_response: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponse>>,
    dynamic: Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecificationDynamic,
}

impl Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification {
    #[doc = "Set the field `failure_conditional`.\n"]
    pub fn set_failure_conditional(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_FailureConditional>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.failure_conditional = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.failure_conditional = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `failure_next_step`.\n"]
    pub fn set_failure_next_step(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_FailureNextStep>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.failure_next_step = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.failure_next_step = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `failure_response`.\n"]
    pub fn set_failure_response(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_FailureResponse>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.failure_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.failure_response = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `success_conditional`.\n"]
    pub fn set_success_conditional(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditional,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.success_conditional = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.success_conditional = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `success_next_step`.\n"]
    pub fn set_success_next_step(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStep,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.success_next_step = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.success_next_step = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `success_response`.\n"]
    pub fn set_success_response(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponse,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.success_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.success_response = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeout_conditional`.\n"]
    pub fn set_timeout_conditional(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditional,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timeout_conditional = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout_conditional = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeout_next_step`.\n"]
    pub fn set_timeout_next_step(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStep,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timeout_next_step = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout_next_step = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeout_response`.\n"]
    pub fn set_timeout_response(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponse,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timeout_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout_response = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification {}

impl BuildLexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification {
    pub fn build(self) -> Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification {
        Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification {
            failure_conditional: core::default::Default::default(),
            failure_next_step: core::default::Default::default(),
            failure_response: core::default::Default::default(),
            success_conditional: core::default::Default::default(),
            success_next_step: core::default::Default::default(),
            success_response: core::default::Default::default(),
            timeout_conditional: core::default::Default::default(),
            timeout_next_step: core::default::Default::default(),
            timeout_response: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecificationRef {
        Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_conditional` after provisioning.\n"]
    pub fn failure_conditional(&self) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSetting_FailureConditionalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failure_conditional", self.base))
    }

    #[doc = "Get a reference to the value of field `failure_next_step` after provisioning.\n"]
    pub fn failure_next_step(&self) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSetting_FailureNextStepRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failure_next_step", self.base))
    }

    #[doc = "Get a reference to the value of field `failure_response` after provisioning.\n"]
    pub fn failure_response(&self) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSetting_FailureResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failure_response", self.base))
    }

    #[doc = "Get a reference to the value of field `success_conditional` after provisioning.\n"]
    pub fn success_conditional(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditionalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.success_conditional", self.base))
    }

    #[doc = "Get a reference to the value of field `success_next_step` after provisioning.\n"]
    pub fn success_next_step(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStepRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.success_next_step", self.base))
    }

    #[doc = "Get a reference to the value of field `success_response` after provisioning.\n"]
    pub fn success_response(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.success_response", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_conditional` after provisioning.\n"]
    pub fn timeout_conditional(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditionalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout_conditional", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_next_step` after provisioning.\n"]
    pub fn timeout_next_step(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStepRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout_next_step", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_response` after provisioning.\n"]
    pub fn timeout_response(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout_response", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ConfirmationSetting_CodeHookDynamic {
    post_code_hook_specification: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_CodeHook {
    active: terrars::PrimField<bool>,
    enable_code_hook_invocation: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invocation_label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_code_hook_specification: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification>>,
    dynamic: Lexv2modelsIntent_ConfirmationSetting_CodeHookDynamic,
}

impl Lexv2modelsIntent_ConfirmationSetting_CodeHook {
    #[doc = "Set the field `invocation_label`.\n"]
    pub fn set_invocation_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.invocation_label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `post_code_hook_specification`.\n"]
    pub fn set_post_code_hook_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.post_code_hook_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.post_code_hook_specification = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_CodeHook {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_CodeHook>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_CodeHook {
    #[doc = ""]
    pub active: terrars::PrimField<bool>,
    #[doc = ""]
    pub enable_code_hook_invocation: terrars::PrimField<bool>,
}

impl BuildLexv2modelsIntent_ConfirmationSetting_CodeHook {
    pub fn build(self) -> Lexv2modelsIntent_ConfirmationSetting_CodeHook {
        Lexv2modelsIntent_ConfirmationSetting_CodeHook {
            active: self.active,
            enable_code_hook_invocation: self.enable_code_hook_invocation,
            invocation_label: core::default::Default::default(),
            post_code_hook_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_CodeHookRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ConfirmationSetting_CodeHookRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_CodeHookRef {
        Lexv2modelsIntent_ConfirmationSetting_CodeHookRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_CodeHookRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_code_hook_invocation` after provisioning.\n"]
    pub fn enable_code_hook_invocation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_code_hook_invocation", self.base))
    }

    #[doc = "Get a reference to the value of field `invocation_label` after provisioning.\n"]
    pub fn invocation_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invocation_label", self.base))
    }

    #[doc = "Get a reference to the value of field `post_code_hook_specification` after provisioning.\n"]
    pub fn post_code_hook_specification(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.post_code_hook_specification", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ConfirmationSetting_ConfirmationConditionalDynamic {
    conditional_branch: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch>>,
    default_branch: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_ConfirmationConditional {
    active: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditional_branch: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_branch: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch>>,
    dynamic: Lexv2modelsIntent_ConfirmationSetting_ConfirmationConditionalDynamic,
}

impl Lexv2modelsIntent_ConfirmationSetting_ConfirmationConditional {
    #[doc = "Set the field `conditional_branch`.\n"]
    pub fn set_conditional_branch(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditional_branch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional_branch = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_branch`.\n"]
    pub fn set_default_branch(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_branch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_branch = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_ConfirmationConditional {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_ConfirmationConditional>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_ConfirmationConditional {
    #[doc = ""]
    pub active: terrars::PrimField<bool>,
}

impl BuildLexv2modelsIntent_ConfirmationSetting_ConfirmationConditional {
    pub fn build(self) -> Lexv2modelsIntent_ConfirmationSetting_ConfirmationConditional {
        Lexv2modelsIntent_ConfirmationSetting_ConfirmationConditional {
            active: self.active,
            conditional_branch: core::default::Default::default(),
            default_branch: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_ConfirmationConditionalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ConfirmationSetting_ConfirmationConditionalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_ConfirmationConditionalRef {
        Lexv2modelsIntent_ConfirmationSetting_ConfirmationConditionalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_ConfirmationConditionalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.base))
    }

    #[doc = "Get a reference to the value of field `conditional_branch` after provisioning.\n"]
    pub fn conditional_branch(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditional_branch", self.base))
    }

    #[doc = "Get a reference to the value of field `default_branch` after provisioning.\n"]
    pub fn default_branch(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_branch", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ConfirmationSetting_ConfirmationNextStepDynamic {
    dialog_action: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction>>,
    intent: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_NextStep_Intent>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_ConfirmationNextStep {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_attributes: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dialog_action: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    intent: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_NextStep_Intent>>,
    dynamic: Lexv2modelsIntent_ConfirmationSetting_ConfirmationNextStepDynamic,
}

impl Lexv2modelsIntent_ConfirmationSetting_ConfirmationNextStep {
    #[doc = "Set the field `session_attributes`.\n"]
    pub fn set_session_attributes(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.session_attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dialog_action`.\n"]
    pub fn set_dialog_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dialog_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dialog_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `intent`.\n"]
    pub fn set_intent(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep_Intent>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.intent = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.intent = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_ConfirmationNextStep {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_ConfirmationNextStep>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_ConfirmationNextStep {}

impl BuildLexv2modelsIntent_ConfirmationSetting_ConfirmationNextStep {
    pub fn build(self) -> Lexv2modelsIntent_ConfirmationSetting_ConfirmationNextStep {
        Lexv2modelsIntent_ConfirmationSetting_ConfirmationNextStep {
            session_attributes: core::default::Default::default(),
            dialog_action: core::default::Default::default(),
            intent: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_ConfirmationNextStepRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ConfirmationSetting_ConfirmationNextStepRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_ConfirmationNextStepRef {
        Lexv2modelsIntent_ConfirmationSetting_ConfirmationNextStepRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_ConfirmationNextStepRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `session_attributes` after provisioning.\n"]
    pub fn session_attributes(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.session_attributes", self.base))
    }

    #[doc = "Get a reference to the value of field `dialog_action` after provisioning.\n"]
    pub fn dialog_action(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_NextStep_DialogActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dialog_action", self.base))
    }

    #[doc = "Get a reference to the value of field `intent` after provisioning.\n"]
    pub fn intent(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_NextStep_IntentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.intent", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ConfirmationSetting_ConfirmationResponseDynamic {
    message_group: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_ConfirmationResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_interrupt: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_group: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
    dynamic: Lexv2modelsIntent_ConfirmationSetting_ConfirmationResponseDynamic,
}

impl Lexv2modelsIntent_ConfirmationSetting_ConfirmationResponse {
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
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
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

impl terrars::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_ConfirmationResponse {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_ConfirmationResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_ConfirmationResponse {}

impl BuildLexv2modelsIntent_ConfirmationSetting_ConfirmationResponse {
    pub fn build(self) -> Lexv2modelsIntent_ConfirmationSetting_ConfirmationResponse {
        Lexv2modelsIntent_ConfirmationSetting_ConfirmationResponse {
            allow_interrupt: core::default::Default::default(),
            message_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_ConfirmationResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ConfirmationSetting_ConfirmationResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_ConfirmationResponseRef {
        Lexv2modelsIntent_ConfirmationSetting_ConfirmationResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_ConfirmationResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"]
    pub fn allow_interrupt(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_interrupt", self.base))
    }

    #[doc = "Get a reference to the value of field `message_group` after provisioning.\n"]
    pub fn message_group(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_group", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ConfirmationSetting_DeclinationConditionalDynamic {
    conditional_branch: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch>>,
    default_branch: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_DeclinationConditional {
    active: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditional_branch: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_branch: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch>>,
    dynamic: Lexv2modelsIntent_ConfirmationSetting_DeclinationConditionalDynamic,
}

impl Lexv2modelsIntent_ConfirmationSetting_DeclinationConditional {
    #[doc = "Set the field `conditional_branch`.\n"]
    pub fn set_conditional_branch(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditional_branch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional_branch = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_branch`.\n"]
    pub fn set_default_branch(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_branch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_branch = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_DeclinationConditional {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_DeclinationConditional>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_DeclinationConditional {
    #[doc = ""]
    pub active: terrars::PrimField<bool>,
}

impl BuildLexv2modelsIntent_ConfirmationSetting_DeclinationConditional {
    pub fn build(self) -> Lexv2modelsIntent_ConfirmationSetting_DeclinationConditional {
        Lexv2modelsIntent_ConfirmationSetting_DeclinationConditional {
            active: self.active,
            conditional_branch: core::default::Default::default(),
            default_branch: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_DeclinationConditionalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ConfirmationSetting_DeclinationConditionalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_DeclinationConditionalRef {
        Lexv2modelsIntent_ConfirmationSetting_DeclinationConditionalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_DeclinationConditionalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.base))
    }

    #[doc = "Get a reference to the value of field `conditional_branch` after provisioning.\n"]
    pub fn conditional_branch(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_Conditional_ConditionalBranchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditional_branch", self.base))
    }

    #[doc = "Get a reference to the value of field `default_branch` after provisioning.\n"]
    pub fn default_branch(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_Conditional_DefaultBranchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_branch", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ConfirmationSetting_DeclinationNextStepDynamic {
    dialog_action: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction>>,
    intent: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_NextStep_Intent>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_DeclinationNextStep {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_attributes: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dialog_action: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    intent: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_NextStep_Intent>>,
    dynamic: Lexv2modelsIntent_ConfirmationSetting_DeclinationNextStepDynamic,
}

impl Lexv2modelsIntent_ConfirmationSetting_DeclinationNextStep {
    #[doc = "Set the field `session_attributes`.\n"]
    pub fn set_session_attributes(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.session_attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dialog_action`.\n"]
    pub fn set_dialog_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep_DialogAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dialog_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dialog_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `intent`.\n"]
    pub fn set_intent(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep_Intent>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.intent = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.intent = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_DeclinationNextStep {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_DeclinationNextStep>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_DeclinationNextStep {}

impl BuildLexv2modelsIntent_ConfirmationSetting_DeclinationNextStep {
    pub fn build(self) -> Lexv2modelsIntent_ConfirmationSetting_DeclinationNextStep {
        Lexv2modelsIntent_ConfirmationSetting_DeclinationNextStep {
            session_attributes: core::default::Default::default(),
            dialog_action: core::default::Default::default(),
            intent: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_DeclinationNextStepRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ConfirmationSetting_DeclinationNextStepRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_DeclinationNextStepRef {
        Lexv2modelsIntent_ConfirmationSetting_DeclinationNextStepRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_DeclinationNextStepRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `session_attributes` after provisioning.\n"]
    pub fn session_attributes(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.session_attributes", self.base))
    }

    #[doc = "Get a reference to the value of field `dialog_action` after provisioning.\n"]
    pub fn dialog_action(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_NextStep_DialogActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dialog_action", self.base))
    }

    #[doc = "Get a reference to the value of field `intent` after provisioning.\n"]
    pub fn intent(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_NextStep_IntentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.intent", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ConfirmationSetting_DeclinationResponseDynamic {
    message_group: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_DeclinationResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_interrupt: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_group: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
    dynamic: Lexv2modelsIntent_ConfirmationSetting_DeclinationResponseDynamic,
}

impl Lexv2modelsIntent_ConfirmationSetting_DeclinationResponse {
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
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
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

impl terrars::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_DeclinationResponse {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_DeclinationResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_DeclinationResponse {}

impl BuildLexv2modelsIntent_ConfirmationSetting_DeclinationResponse {
    pub fn build(self) -> Lexv2modelsIntent_ConfirmationSetting_DeclinationResponse {
        Lexv2modelsIntent_ConfirmationSetting_DeclinationResponse {
            allow_interrupt: core::default::Default::default(),
            message_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_DeclinationResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ConfirmationSetting_DeclinationResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_DeclinationResponseRef {
        Lexv2modelsIntent_ConfirmationSetting_DeclinationResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_DeclinationResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"]
    pub fn allow_interrupt(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_interrupt", self.base))
    }

    #[doc = "Get a reference to the value of field `message_group` after provisioning.\n"]
    pub fn message_group(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_group", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_ElicitationCodeHook {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_code_hook_invocation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invocation_label: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Lexv2modelsIntent_ConfirmationSetting_ElicitationCodeHook {
    #[doc = "Set the field `enable_code_hook_invocation`.\n"]
    pub fn set_enable_code_hook_invocation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_code_hook_invocation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `invocation_label`.\n"]
    pub fn set_invocation_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.invocation_label = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_ElicitationCodeHook {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_ElicitationCodeHook>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_ElicitationCodeHook {}

impl BuildLexv2modelsIntent_ConfirmationSetting_ElicitationCodeHook {
    pub fn build(self) -> Lexv2modelsIntent_ConfirmationSetting_ElicitationCodeHook {
        Lexv2modelsIntent_ConfirmationSetting_ElicitationCodeHook {
            enable_code_hook_invocation: core::default::Default::default(),
            invocation_label: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_ElicitationCodeHookRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ConfirmationSetting_ElicitationCodeHookRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_ElicitationCodeHookRef {
        Lexv2modelsIntent_ConfirmationSetting_ElicitationCodeHookRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_ElicitationCodeHookRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_code_hook_invocation` after provisioning.\n"]
    pub fn enable_code_hook_invocation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_code_hook_invocation", self.base))
    }

    #[doc = "Get a reference to the value of field `invocation_label` after provisioning.\n"]
    pub fn invocation_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invocation_label", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes {
    allow_audio_input: terrars::PrimField<bool>,
    allow_dtmf_input: terrars::PrimField<bool>,
}

impl Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes { }

impl terrars
::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes {
    type O =
        terrars
        ::BlockAssignable<
            Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes {
    #[doc = ""]
    pub allow_audio_input: terrars::PrimField<bool>,
    #[doc = ""]
    pub allow_dtmf_input: terrars::PrimField<bool>,
}

impl BuildLexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes {
    pub fn build(
        self,
    ) -> Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes {
        Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes {
            allow_audio_input: self.allow_audio_input,
            allow_dtmf_input: self.allow_dtmf_input,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypesRef {
        Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypesRef {
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
pub struct Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification {
    end_timeout_ms: terrars::PrimField<f64>,
    max_length_ms: terrars::PrimField<f64>,
}

impl Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification {

}

impl terrars
::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification {
    type O =
        terrars
        ::BlockAssignable<
            Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification {
    #[doc = ""]
    pub end_timeout_ms: terrars::PrimField<f64>,
    #[doc = ""]
    pub max_length_ms: terrars::PrimField<f64>,
}

impl BuildLexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification {
    pub fn build(
        self,
    ) -> Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification {
        Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification {
            end_timeout_ms: self.end_timeout_ms,
            max_length_ms: self.max_length_ms,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecificationRef {
        Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecificationRef {
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
pub struct Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification {
    deletion_character: terrars::PrimField<std::string::String>,
    end_character: terrars::PrimField<std::string::String>,
    end_timeout_ms: terrars::PrimField<f64>,
    max_length: terrars::PrimField<f64>,
}

impl Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification {

}

impl terrars
::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification {
    type O =
        terrars
        ::BlockAssignable<
            Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification {
    #[doc = ""]
    pub deletion_character: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub end_character: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub end_timeout_ms: terrars::PrimField<f64>,
    #[doc = ""]
    pub max_length: terrars::PrimField<f64>,
}

impl BuildLexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification {
    pub fn build(
        self,
    ) -> Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification {
        Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification {
            deletion_character: self.deletion_character,
            end_character: self.end_character,
            end_timeout_ms: self.end_timeout_ms,
            max_length: self.max_length,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecificationRef {
        Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecificationRef {
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
struct Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecificationDynamic {
    audio_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification,
        >,
    >,
    dtmf_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification,
        >,
    >,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification {
    start_timeout_ms: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dtmf_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification,
        >,
    >,
    dynamic: Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecificationDynamic,
}

impl Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification {
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
                            Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecification,
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
                            Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecification,
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
::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification {
    type O =
        terrars
        ::BlockAssignable<
            Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification {
    #[doc = ""]
    pub start_timeout_ms: terrars::PrimField<f64>,
}

impl BuildLexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification {
    pub fn build(
        self,
    ) -> Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification {
        Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification {
            start_timeout_ms: self.start_timeout_ms,
            audio_specification: core::default::Default::default(),
            dtmf_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecificationRef {
        Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecificationRef {
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
        Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_AudioSpecificationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `dtmf_specification` after provisioning.\n"]
    pub fn dtmf_specification(
        &self,
    ) -> terrars
    ::ListRef<
        Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification_DtmfSpecificationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dtmf_specification", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification {
    start_timeout_ms: terrars::PrimField<f64>,
}

impl Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification { }

impl terrars
::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification {
    type O =
        terrars
        ::BlockAssignable<
            Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification {
    #[doc = ""]
    pub start_timeout_ms: terrars::PrimField<f64>,
}

impl BuildLexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification {
    pub fn build(
        self,
    ) -> Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification {
        Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification {
            start_timeout_ms: self.start_timeout_ms,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecificationRef {
        Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecificationRef {
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
struct Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecificationDynamic {
    allowed_input_types: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes,
        >,
    >,
    audio_and_dtmf_input_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification,
        >,
    >,
    text_input_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification,
        >,
    >,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_interrupt: std::option::Option<terrars::PrimField<bool>>,
    map_block_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_input_types: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_and_dtmf_input_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_input_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification,
        >,
    >,
    dynamic: Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecificationDynamic,
}

impl Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification {
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
                            Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypes,
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
                            Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecification,
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
                            Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecification,
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
::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification {
    type O =
        terrars
        ::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification {
    #[doc = ""]
    pub map_block_key: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification {
    pub fn build(self) -> Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification {
        Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification {
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
pub struct Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecificationRef {
        Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecificationRef {
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
        Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AllowedInputTypesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_input_types", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_and_dtmf_input_specification` after provisioning.\n"]
    pub fn audio_and_dtmf_input_specification(
        &self,
    ) -> terrars
    ::ListRef<
        Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_AudioAndDtmfInputSpecificationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_and_dtmf_input_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `text_input_specification` after provisioning.\n"]
    pub fn text_input_specification(
        &self,
    ) -> terrars
    ::ListRef<
        Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification_TextInputSpecificationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_input_specification", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ConfirmationSetting_PromptSpecificationDynamic {
    message_group: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
    prompt_attempts_specification: std
    ::option
    ::Option<
        terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification>,
    >,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting_PromptSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_interrupt: std::option::Option<terrars::PrimField<bool>>,
    max_retries: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_selection_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_group: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt_attempts_specification: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification>>,
    dynamic: Lexv2modelsIntent_ConfirmationSetting_PromptSpecificationDynamic,
}

impl Lexv2modelsIntent_ConfirmationSetting_PromptSpecification {
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
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
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
                            Lexv2modelsIntent_ConfirmationSetting_PromptSpecification_PromptAttemptsSpecification,
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

impl terrars::ToListMappable for Lexv2modelsIntent_ConfirmationSetting_PromptSpecification {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_PromptSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting_PromptSpecification {
    #[doc = ""]
    pub max_retries: terrars::PrimField<f64>,
}

impl BuildLexv2modelsIntent_ConfirmationSetting_PromptSpecification {
    pub fn build(self) -> Lexv2modelsIntent_ConfirmationSetting_PromptSpecification {
        Lexv2modelsIntent_ConfirmationSetting_PromptSpecification {
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
pub struct Lexv2modelsIntent_ConfirmationSetting_PromptSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ConfirmationSetting_PromptSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_ConfirmationSetting_PromptSpecificationRef {
        Lexv2modelsIntent_ConfirmationSetting_PromptSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSetting_PromptSpecificationRef {
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
    pub fn message_group(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_group", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_ConfirmationSettingDynamic {
    code_hook: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_CodeHook>>,
    confirmation_conditional: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_ConfirmationConditional>>,
    confirmation_next_step: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_ConfirmationNextStep>>,
    confirmation_response: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_ConfirmationResponse>>,
    declination_conditional: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_DeclinationConditional>>,
    declination_next_step: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_DeclinationNextStep>>,
    declination_response: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_DeclinationResponse>>,
    elicitation_code_hook: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_ElicitationCodeHook>>,
    failure_conditional: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_FailureConditional>>,
    failure_next_step: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_FailureNextStep>>,
    failure_response: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_FailureResponse>>,
    prompt_specification: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_PromptSpecification>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_ConfirmationSetting {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_hook: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_CodeHook>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confirmation_conditional: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_ConfirmationConditional>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confirmation_next_step: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_ConfirmationNextStep>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confirmation_response: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_ConfirmationResponse>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    declination_conditional: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_DeclinationConditional>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    declination_next_step: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_DeclinationNextStep>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    declination_response: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_DeclinationResponse>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elicitation_code_hook: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_ElicitationCodeHook>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_conditional: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_FailureConditional>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_next_step: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_FailureNextStep>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_response: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_FailureResponse>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt_specification: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_PromptSpecification>>,
    dynamic: Lexv2modelsIntent_ConfirmationSettingDynamic,
}

impl Lexv2modelsIntent_ConfirmationSetting {
    #[doc = "Set the field `active`.\n"]
    pub fn set_active(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.active = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `code_hook`.\n"]
    pub fn set_code_hook(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_CodeHook>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_hook = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_hook = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `confirmation_conditional`.\n"]
    pub fn set_confirmation_conditional(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_ConfirmationConditional>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.confirmation_conditional = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.confirmation_conditional = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `confirmation_next_step`.\n"]
    pub fn set_confirmation_next_step(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_ConfirmationNextStep>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.confirmation_next_step = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.confirmation_next_step = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `confirmation_response`.\n"]
    pub fn set_confirmation_response(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_ConfirmationResponse>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.confirmation_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.confirmation_response = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `declination_conditional`.\n"]
    pub fn set_declination_conditional(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_DeclinationConditional>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.declination_conditional = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.declination_conditional = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `declination_next_step`.\n"]
    pub fn set_declination_next_step(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_DeclinationNextStep>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.declination_next_step = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.declination_next_step = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `declination_response`.\n"]
    pub fn set_declination_response(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_DeclinationResponse>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.declination_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.declination_response = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `elicitation_code_hook`.\n"]
    pub fn set_elicitation_code_hook(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_ElicitationCodeHook>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.elicitation_code_hook = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.elicitation_code_hook = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `failure_conditional`.\n"]
    pub fn set_failure_conditional(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_FailureConditional>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.failure_conditional = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.failure_conditional = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `failure_next_step`.\n"]
    pub fn set_failure_next_step(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_FailureNextStep>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.failure_next_step = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.failure_next_step = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `failure_response`.\n"]
    pub fn set_failure_response(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_FailureResponse>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.failure_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.failure_response = std::option::Option::Some(d);
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
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_PromptSpecification>>,
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
}

impl terrars::ToListMappable for Lexv2modelsIntent_ConfirmationSetting {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_ConfirmationSetting {}

impl BuildLexv2modelsIntent_ConfirmationSetting {
    pub fn build(self) -> Lexv2modelsIntent_ConfirmationSetting {
        Lexv2modelsIntent_ConfirmationSetting {
            active: core::default::Default::default(),
            code_hook: core::default::Default::default(),
            confirmation_conditional: core::default::Default::default(),
            confirmation_next_step: core::default::Default::default(),
            confirmation_response: core::default::Default::default(),
            declination_conditional: core::default::Default::default(),
            declination_next_step: core::default::Default::default(),
            declination_response: core::default::Default::default(),
            elicitation_code_hook: core::default::Default::default(),
            failure_conditional: core::default::Default::default(),
            failure_next_step: core::default::Default::default(),
            failure_response: core::default::Default::default(),
            prompt_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_ConfirmationSettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_ConfirmationSettingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsIntent_ConfirmationSettingRef {
        Lexv2modelsIntent_ConfirmationSettingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_ConfirmationSettingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.base))
    }

    #[doc = "Get a reference to the value of field `code_hook` after provisioning.\n"]
    pub fn code_hook(&self) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSetting_CodeHookRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_hook", self.base))
    }

    #[doc = "Get a reference to the value of field `confirmation_conditional` after provisioning.\n"]
    pub fn confirmation_conditional(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSetting_ConfirmationConditionalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confirmation_conditional", self.base))
    }

    #[doc = "Get a reference to the value of field `confirmation_next_step` after provisioning.\n"]
    pub fn confirmation_next_step(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSetting_ConfirmationNextStepRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confirmation_next_step", self.base))
    }

    #[doc = "Get a reference to the value of field `confirmation_response` after provisioning.\n"]
    pub fn confirmation_response(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSetting_ConfirmationResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confirmation_response", self.base))
    }

    #[doc = "Get a reference to the value of field `declination_conditional` after provisioning.\n"]
    pub fn declination_conditional(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSetting_DeclinationConditionalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.declination_conditional", self.base))
    }

    #[doc = "Get a reference to the value of field `declination_next_step` after provisioning.\n"]
    pub fn declination_next_step(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSetting_DeclinationNextStepRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.declination_next_step", self.base))
    }

    #[doc = "Get a reference to the value of field `declination_response` after provisioning.\n"]
    pub fn declination_response(&self) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSetting_DeclinationResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.declination_response", self.base))
    }

    #[doc = "Get a reference to the value of field `elicitation_code_hook` after provisioning.\n"]
    pub fn elicitation_code_hook(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSetting_ElicitationCodeHookRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elicitation_code_hook", self.base))
    }

    #[doc = "Get a reference to the value of field `failure_conditional` after provisioning.\n"]
    pub fn failure_conditional(&self) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSetting_FailureConditionalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failure_conditional", self.base))
    }

    #[doc = "Get a reference to the value of field `failure_next_step` after provisioning.\n"]
    pub fn failure_next_step(&self) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSetting_FailureNextStepRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failure_next_step", self.base))
    }

    #[doc = "Get a reference to the value of field `failure_response` after provisioning.\n"]
    pub fn failure_response(&self) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSetting_FailureResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failure_response", self.base))
    }

    #[doc = "Get a reference to the value of field `prompt_specification` after provisioning.\n"]
    pub fn prompt_specification(&self) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSetting_PromptSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prompt_specification", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_DialogCodeHook {
    enabled: terrars::PrimField<bool>,
}

impl Lexv2modelsIntent_DialogCodeHook { }

impl terrars::ToListMappable for Lexv2modelsIntent_DialogCodeHook {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_DialogCodeHook>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_DialogCodeHook {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildLexv2modelsIntent_DialogCodeHook {
    pub fn build(self) -> Lexv2modelsIntent_DialogCodeHook {
        Lexv2modelsIntent_DialogCodeHook { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_DialogCodeHookRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_DialogCodeHookRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsIntent_DialogCodeHookRef {
        Lexv2modelsIntent_DialogCodeHookRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_DialogCodeHookRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_StartResponseDynamic {
    message_group: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_StartResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_interrupt: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delay_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_group: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
    dynamic: Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_StartResponseDynamic,
}

impl Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_StartResponse {
    #[doc = "Set the field `allow_interrupt`.\n"]
    pub fn set_allow_interrupt(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_interrupt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delay_in_seconds`.\n"]
    pub fn set_delay_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.delay_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message_group`.\n"]
    pub fn set_message_group(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
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

impl terrars::ToListMappable for Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_StartResponse {
    type O =
        terrars
        ::BlockAssignable<Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_StartResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_StartResponse {}

impl BuildLexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_StartResponse {
    pub fn build(self) -> Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_StartResponse {
        Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_StartResponse {
            allow_interrupt: core::default::Default::default(),
            delay_in_seconds: core::default::Default::default(),
            message_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_StartResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_StartResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_StartResponseRef {
        Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_StartResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_StartResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"]
    pub fn allow_interrupt(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_interrupt", self.base))
    }

    #[doc = "Get a reference to the value of field `delay_in_seconds` after provisioning.\n"]
    pub fn delay_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delay_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `message_group` after provisioning.\n"]
    pub fn message_group(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_group", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_UpdateResponseDynamic {
    message_group: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_UpdateResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_interrupt: std::option::Option<terrars::PrimField<bool>>,
    frequency_in_seconds: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_group: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
    dynamic: Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_UpdateResponseDynamic,
}

impl Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_UpdateResponse {
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
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
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

impl terrars::ToListMappable for Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_UpdateResponse {
    type O =
        terrars
        ::BlockAssignable<Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_UpdateResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_UpdateResponse {
    #[doc = ""]
    pub frequency_in_seconds: terrars::PrimField<f64>,
}

impl BuildLexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_UpdateResponse {
    pub fn build(self) -> Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_UpdateResponse {
        Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_UpdateResponse {
            allow_interrupt: core::default::Default::default(),
            frequency_in_seconds: self.frequency_in_seconds,
            message_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_UpdateResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_UpdateResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_UpdateResponseRef {
        Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_UpdateResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_UpdateResponseRef {
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

    #[doc = "Get a reference to the value of field `message_group` after provisioning.\n"]
    pub fn message_group(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_group", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecificationDynamic {
    start_response: std
    ::option
    ::Option<
        terrars::DynamicBlock<Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_StartResponse>,
    >,
    update_response: std
    ::option
    ::Option<
        terrars::DynamicBlock<Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_UpdateResponse>,
    >,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification {
    active: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_response: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_StartResponse>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_response: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_UpdateResponse>>,
    dynamic: Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecificationDynamic,
}

impl Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification {
    #[doc = "Set the field `timeout_in_seconds`.\n"]
    pub fn set_timeout_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_response`.\n"]
    pub fn set_start_response(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_StartResponse,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.start_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.start_response = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `update_response`.\n"]
    pub fn set_update_response(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_UpdateResponse,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.update_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.update_response = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification {
    #[doc = ""]
    pub active: terrars::PrimField<bool>,
}

impl BuildLexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification {
    pub fn build(self) -> Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification {
        Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification {
            active: self.active,
            timeout_in_seconds: core::default::Default::default(),
            start_response: core::default::Default::default(),
            update_response: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecificationRef {
        Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_in_seconds` after provisioning.\n"]
    pub fn timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `start_response` after provisioning.\n"]
    pub fn start_response(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_StartResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_response", self.base))
    }

    #[doc = "Get a reference to the value of field `update_response` after provisioning.\n"]
    pub fn update_response(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification_UpdateResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_response", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_FulfillmentCodeHook_PostFulfillmentStatusSpecificationDynamic {
    failure_conditional: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_FailureConditional>>,
    failure_next_step: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_FailureNextStep>>,
    failure_response: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_FailureResponse>>,
    success_conditional: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditional>,
    >,
    success_next_step: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStep>,
    >,
    success_response: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponse>,
    >,
    timeout_conditional: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditional>,
    >,
    timeout_next_step: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStep>,
    >,
    timeout_response: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponse>,
    >,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_FulfillmentCodeHook_PostFulfillmentStatusSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_conditional: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_FailureConditional>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_next_step: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_FailureNextStep>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_response: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_FailureResponse>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_conditional: std
    ::option
    ::Option<
        std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditional>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_next_step: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStep>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_response: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponse>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_conditional: std
    ::option
    ::Option<
        std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditional>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_next_step: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStep>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_response: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponse>>,
    dynamic: Lexv2modelsIntent_FulfillmentCodeHook_PostFulfillmentStatusSpecificationDynamic,
}

impl Lexv2modelsIntent_FulfillmentCodeHook_PostFulfillmentStatusSpecification {
    #[doc = "Set the field `failure_conditional`.\n"]
    pub fn set_failure_conditional(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_FailureConditional>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.failure_conditional = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.failure_conditional = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `failure_next_step`.\n"]
    pub fn set_failure_next_step(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_FailureNextStep>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.failure_next_step = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.failure_next_step = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `failure_response`.\n"]
    pub fn set_failure_response(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_FailureResponse>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.failure_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.failure_response = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `success_conditional`.\n"]
    pub fn set_success_conditional(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditional,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.success_conditional = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.success_conditional = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `success_next_step`.\n"]
    pub fn set_success_next_step(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStep,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.success_next_step = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.success_next_step = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `success_response`.\n"]
    pub fn set_success_response(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponse,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.success_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.success_response = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeout_conditional`.\n"]
    pub fn set_timeout_conditional(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditional,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timeout_conditional = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout_conditional = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeout_next_step`.\n"]
    pub fn set_timeout_next_step(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStep,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timeout_next_step = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout_next_step = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeout_response`.\n"]
    pub fn set_timeout_response(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponse,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timeout_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout_response = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_FulfillmentCodeHook_PostFulfillmentStatusSpecification {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_FulfillmentCodeHook_PostFulfillmentStatusSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_FulfillmentCodeHook_PostFulfillmentStatusSpecification {}

impl BuildLexv2modelsIntent_FulfillmentCodeHook_PostFulfillmentStatusSpecification {
    pub fn build(self) -> Lexv2modelsIntent_FulfillmentCodeHook_PostFulfillmentStatusSpecification {
        Lexv2modelsIntent_FulfillmentCodeHook_PostFulfillmentStatusSpecification {
            failure_conditional: core::default::Default::default(),
            failure_next_step: core::default::Default::default(),
            failure_response: core::default::Default::default(),
            success_conditional: core::default::Default::default(),
            success_next_step: core::default::Default::default(),
            success_response: core::default::Default::default(),
            timeout_conditional: core::default::Default::default(),
            timeout_next_step: core::default::Default::default(),
            timeout_response: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_FulfillmentCodeHook_PostFulfillmentStatusSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_FulfillmentCodeHook_PostFulfillmentStatusSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_FulfillmentCodeHook_PostFulfillmentStatusSpecificationRef {
        Lexv2modelsIntent_FulfillmentCodeHook_PostFulfillmentStatusSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_FulfillmentCodeHook_PostFulfillmentStatusSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_conditional` after provisioning.\n"]
    pub fn failure_conditional(&self) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSetting_FailureConditionalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failure_conditional", self.base))
    }

    #[doc = "Get a reference to the value of field `failure_next_step` after provisioning.\n"]
    pub fn failure_next_step(&self) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSetting_FailureNextStepRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failure_next_step", self.base))
    }

    #[doc = "Get a reference to the value of field `failure_response` after provisioning.\n"]
    pub fn failure_response(&self) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSetting_FailureResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failure_response", self.base))
    }

    #[doc = "Get a reference to the value of field `success_conditional` after provisioning.\n"]
    pub fn success_conditional(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessConditionalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.success_conditional", self.base))
    }

    #[doc = "Get a reference to the value of field `success_next_step` after provisioning.\n"]
    pub fn success_next_step(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessNextStepRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.success_next_step", self.base))
    }

    #[doc = "Get a reference to the value of field `success_response` after provisioning.\n"]
    pub fn success_response(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_SuccessResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.success_response", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_conditional` after provisioning.\n"]
    pub fn timeout_conditional(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutConditionalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout_conditional", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_next_step` after provisioning.\n"]
    pub fn timeout_next_step(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutNextStepRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout_next_step", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_response` after provisioning.\n"]
    pub fn timeout_response(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsIntent_ConfirmationSetting_CodeHook_PostCodeHookSpecification_TimeoutResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout_response", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_FulfillmentCodeHookDynamic {
    fulfillment_updates_specification: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification>>,
    post_fulfillment_status_specification: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_FulfillmentCodeHook_PostFulfillmentStatusSpecification>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_FulfillmentCodeHook {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active: std::option::Option<terrars::PrimField<bool>>,
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fulfillment_updates_specification: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_fulfillment_status_specification: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_FulfillmentCodeHook_PostFulfillmentStatusSpecification>>,
    dynamic: Lexv2modelsIntent_FulfillmentCodeHookDynamic,
}

impl Lexv2modelsIntent_FulfillmentCodeHook {
    #[doc = "Set the field `active`.\n"]
    pub fn set_active(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.active = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fulfillment_updates_specification`.\n"]
    pub fn set_fulfillment_updates_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecification>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fulfillment_updates_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fulfillment_updates_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `post_fulfillment_status_specification`.\n"]
    pub fn set_post_fulfillment_status_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Lexv2modelsIntent_FulfillmentCodeHook_PostFulfillmentStatusSpecification>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.post_fulfillment_status_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.post_fulfillment_status_specification = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_FulfillmentCodeHook {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_FulfillmentCodeHook>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_FulfillmentCodeHook {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildLexv2modelsIntent_FulfillmentCodeHook {
    pub fn build(self) -> Lexv2modelsIntent_FulfillmentCodeHook {
        Lexv2modelsIntent_FulfillmentCodeHook {
            active: core::default::Default::default(),
            enabled: self.enabled,
            fulfillment_updates_specification: core::default::Default::default(),
            post_fulfillment_status_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_FulfillmentCodeHookRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_FulfillmentCodeHookRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsIntent_FulfillmentCodeHookRef {
        Lexv2modelsIntent_FulfillmentCodeHookRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_FulfillmentCodeHookRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `fulfillment_updates_specification` after provisioning.\n"]
    pub fn fulfillment_updates_specification(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_FulfillmentCodeHook_FulfillmentUpdatesSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fulfillment_updates_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `post_fulfillment_status_specification` after provisioning.\n"]
    pub fn post_fulfillment_status_specification(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_FulfillmentCodeHook_PostFulfillmentStatusSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.post_fulfillment_status_specification", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_InitialResponseSetting_InitialResponseDynamic {
    message_group: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_InitialResponseSetting_InitialResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_interrupt: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_group: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
    dynamic: Lexv2modelsIntent_InitialResponseSetting_InitialResponseDynamic,
}

impl Lexv2modelsIntent_InitialResponseSetting_InitialResponse {
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
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroup>>,
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

impl terrars::ToListMappable for Lexv2modelsIntent_InitialResponseSetting_InitialResponse {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_InitialResponseSetting_InitialResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_InitialResponseSetting_InitialResponse {}

impl BuildLexv2modelsIntent_InitialResponseSetting_InitialResponse {
    pub fn build(self) -> Lexv2modelsIntent_InitialResponseSetting_InitialResponse {
        Lexv2modelsIntent_InitialResponseSetting_InitialResponse {
            allow_interrupt: core::default::Default::default(),
            message_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_InitialResponseSetting_InitialResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_InitialResponseSetting_InitialResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_InitialResponseSetting_InitialResponseRef {
        Lexv2modelsIntent_InitialResponseSetting_InitialResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_InitialResponseSetting_InitialResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"]
    pub fn allow_interrupt(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_interrupt", self.base))
    }

    #[doc = "Get a reference to the value of field `message_group` after provisioning.\n"]
    pub fn message_group(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ClosingResponse_MessageGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_group", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_InitialResponseSettingDynamic {
    code_hook: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ConfirmationSetting_CodeHook>>,
    conditional: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_Conditional>>,
    initial_response: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_InitialResponseSetting_InitialResponse>>,
    next_step: std::option::Option<terrars::DynamicBlock<Lexv2modelsIntent_ClosingSetting_NextStep>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_InitialResponseSetting {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_hook: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ConfirmationSetting_CodeHook>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditional: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_Conditional>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_response: std::option::Option<std::vec::Vec<Lexv2modelsIntent_InitialResponseSetting_InitialResponse>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_step: std::option::Option<std::vec::Vec<Lexv2modelsIntent_ClosingSetting_NextStep>>,
    dynamic: Lexv2modelsIntent_InitialResponseSettingDynamic,
}

impl Lexv2modelsIntent_InitialResponseSetting {
    #[doc = "Set the field `code_hook`.\n"]
    pub fn set_code_hook(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ConfirmationSetting_CodeHook>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_hook = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_hook = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `conditional`.\n"]
    pub fn set_conditional(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_Conditional>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditional = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `initial_response`.\n"]
    pub fn set_initial_response(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Lexv2modelsIntent_InitialResponseSetting_InitialResponse>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.initial_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.initial_response = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `next_step`.\n"]
    pub fn set_next_step(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Lexv2modelsIntent_ClosingSetting_NextStep>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.next_step = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.next_step = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_InitialResponseSetting {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_InitialResponseSetting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_InitialResponseSetting {}

impl BuildLexv2modelsIntent_InitialResponseSetting {
    pub fn build(self) -> Lexv2modelsIntent_InitialResponseSetting {
        Lexv2modelsIntent_InitialResponseSetting {
            code_hook: core::default::Default::default(),
            conditional: core::default::Default::default(),
            initial_response: core::default::Default::default(),
            next_step: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_InitialResponseSettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_InitialResponseSettingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsIntent_InitialResponseSettingRef {
        Lexv2modelsIntent_InitialResponseSettingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_InitialResponseSettingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code_hook` after provisioning.\n"]
    pub fn code_hook(&self) -> terrars::ListRef<Lexv2modelsIntent_ConfirmationSetting_CodeHookRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_hook", self.base))
    }

    #[doc = "Get a reference to the value of field `conditional` after provisioning.\n"]
    pub fn conditional(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_ConditionalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditional", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_response` after provisioning.\n"]
    pub fn initial_response(&self) -> terrars::ListRef<Lexv2modelsIntent_InitialResponseSetting_InitialResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initial_response", self.base))
    }

    #[doc = "Get a reference to the value of field `next_step` after provisioning.\n"]
    pub fn next_step(&self) -> terrars::ListRef<Lexv2modelsIntent_ClosingSetting_NextStepRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.next_step", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_InputContext {
    name: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsIntent_InputContext { }

impl terrars::ToListMappable for Lexv2modelsIntent_InputContext {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_InputContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_InputContext {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsIntent_InputContext {
    pub fn build(self) -> Lexv2modelsIntent_InputContext {
        Lexv2modelsIntent_InputContext { name: self.name }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_InputContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_InputContextRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsIntent_InputContextRef {
        Lexv2modelsIntent_InputContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_InputContextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_KendraConfiguration {
    kendra_index: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_filter_string: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_filter_string_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl Lexv2modelsIntent_KendraConfiguration {
    #[doc = "Set the field `query_filter_string`.\n"]
    pub fn set_query_filter_string(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.query_filter_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_filter_string_enabled`.\n"]
    pub fn set_query_filter_string_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.query_filter_string_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_KendraConfiguration {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_KendraConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_KendraConfiguration {
    #[doc = ""]
    pub kendra_index: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsIntent_KendraConfiguration {
    pub fn build(self) -> Lexv2modelsIntent_KendraConfiguration {
        Lexv2modelsIntent_KendraConfiguration {
            kendra_index: self.kendra_index,
            query_filter_string: core::default::Default::default(),
            query_filter_string_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_KendraConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_KendraConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsIntent_KendraConfigurationRef {
        Lexv2modelsIntent_KendraConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_KendraConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kendra_index` after provisioning.\n"]
    pub fn kendra_index(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kendra_index", self.base))
    }

    #[doc = "Get a reference to the value of field `query_filter_string` after provisioning.\n"]
    pub fn query_filter_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_filter_string", self.base))
    }

    #[doc = "Get a reference to the value of field `query_filter_string_enabled` after provisioning.\n"]
    pub fn query_filter_string_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_filter_string_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_OutputContext {
    name: terrars::PrimField<std::string::String>,
    time_to_live_in_seconds: terrars::PrimField<f64>,
    turns_to_live: terrars::PrimField<f64>,
}

impl Lexv2modelsIntent_OutputContext { }

impl terrars::ToListMappable for Lexv2modelsIntent_OutputContext {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_OutputContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_OutputContext {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub time_to_live_in_seconds: terrars::PrimField<f64>,
    #[doc = ""]
    pub turns_to_live: terrars::PrimField<f64>,
}

impl BuildLexv2modelsIntent_OutputContext {
    pub fn build(self) -> Lexv2modelsIntent_OutputContext {
        Lexv2modelsIntent_OutputContext {
            name: self.name,
            time_to_live_in_seconds: self.time_to_live_in_seconds,
            turns_to_live: self.turns_to_live,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_OutputContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_OutputContextRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsIntent_OutputContextRef {
        Lexv2modelsIntent_OutputContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_OutputContextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `time_to_live_in_seconds` after provisioning.\n"]
    pub fn time_to_live_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_to_live_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `turns_to_live` after provisioning.\n"]
    pub fn turns_to_live(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.turns_to_live", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration_Guardrail {
    identifier: terrars::PrimField<std::string::String>,
    version: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration_Guardrail { }

impl terrars::ToListMappable for Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration_Guardrail {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration_Guardrail>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration_Guardrail {
    #[doc = ""]
    pub identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration_Guardrail {
    pub fn build(self) -> Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration_Guardrail {
        Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration_Guardrail {
            identifier: self.identifier,
            version: self.version,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration_GuardrailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration_GuardrailRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration_GuardrailRef {
        Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration_GuardrailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration_GuardrailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfigurationDynamic {
    guardrail: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration_Guardrail>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_prompt: std::option::Option<terrars::PrimField<std::string::String>>,
    model_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trace_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guardrail: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration_Guardrail>>,
    dynamic: Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfigurationDynamic,
}

impl Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration {
    #[doc = "Set the field `custom_prompt`.\n"]
    pub fn set_custom_prompt(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.custom_prompt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trace_status`.\n"]
    pub fn set_trace_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.trace_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guardrail`.\n"]
    pub fn set_guardrail(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration_Guardrail,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.guardrail = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.guardrail = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration {
    #[doc = ""]
    pub model_arn: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration {
    pub fn build(self) -> Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration {
        Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration {
            custom_prompt: core::default::Default::default(),
            model_arn: self.model_arn,
            trace_status: core::default::Default::default(),
            guardrail: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfigurationRef {
        Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_prompt` after provisioning.\n"]
    pub fn custom_prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_prompt", self.base))
    }

    #[doc = "Get a reference to the value of field `model_arn` after provisioning.\n"]
    pub fn model_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `trace_status` after provisioning.\n"]
    pub fn trace_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trace_status", self.base))
    }

    #[doc = "Get a reference to the value of field `guardrail` after provisioning.\n"]
    pub fn guardrail(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration_GuardrailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guardrail", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration_ExactResponseFields {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    answer_field: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration_ExactResponseFields {
    #[doc = "Set the field `answer_field`.\n"]
    pub fn set_answer_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.answer_field = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration_ExactResponseFields {
    type O =
        terrars
        ::BlockAssignable<
            Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration_ExactResponseFields,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration_ExactResponseFields {}

impl BuildLexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration_ExactResponseFields {
    pub fn build(
        self,
    ) -> Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration_ExactResponseFields {
        Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration_ExactResponseFields {
            answer_field: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration_ExactResponseFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration_ExactResponseFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration_ExactResponseFieldsRef {
        Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration_ExactResponseFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration_ExactResponseFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `answer_field` after provisioning.\n"]
    pub fn answer_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.answer_field", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfigurationDynamic {
    exact_response_fields: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration_ExactResponseFields,
        >,
    >,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration {
    bedrock_knowledge_base_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact_response: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact_response_fields: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration_ExactResponseFields,
        >,
    >,
    dynamic: Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfigurationDynamic,
}

impl Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration {
    #[doc = "Set the field `exact_response`.\n"]
    pub fn set_exact_response(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exact_response = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exact_response_fields`.\n"]
    pub fn set_exact_response_fields(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration_ExactResponseFields,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exact_response_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exact_response_fields = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration {
    #[doc = ""]
    pub bedrock_knowledge_base_arn: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration {
    pub fn build(
        self,
    ) -> Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration {
        Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration {
            bedrock_knowledge_base_arn: self.bedrock_knowledge_base_arn,
            exact_response: core::default::Default::default(),
            exact_response_fields: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfigurationRef {
        Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bedrock_knowledge_base_arn` after provisioning.\n"]
    pub fn bedrock_knowledge_base_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bedrock_knowledge_base_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `exact_response` after provisioning.\n"]
    pub fn exact_response(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact_response", self.base))
    }

    #[doc = "Get a reference to the value of field `exact_response_fields` after provisioning.\n"]
    pub fn exact_response_fields(
        &self,
    ) -> terrars
    ::ListRef<
        Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration_ExactResponseFieldsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exact_response_fields", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_KendraConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact_response: std::option::Option<terrars::PrimField<bool>>,
    kendra_index: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_filter_string: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_filter_string_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_KendraConfiguration {
    #[doc = "Set the field `exact_response`.\n"]
    pub fn set_exact_response(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exact_response = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_filter_string`.\n"]
    pub fn set_query_filter_string(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.query_filter_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_filter_string_enabled`.\n"]
    pub fn set_query_filter_string_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.query_filter_string_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_KendraConfiguration {
    type O =
        terrars
        ::BlockAssignable<Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_KendraConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_KendraConfiguration {
    #[doc = ""]
    pub kendra_index: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_KendraConfiguration {
    pub fn build(self) -> Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_KendraConfiguration {
        Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_KendraConfiguration {
            exact_response: core::default::Default::default(),
            kendra_index: self.kendra_index,
            query_filter_string: core::default::Default::default(),
            query_filter_string_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_KendraConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_KendraConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_KendraConfigurationRef {
        Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_KendraConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_KendraConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact_response` after provisioning.\n"]
    pub fn exact_response(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact_response", self.base))
    }

    #[doc = "Get a reference to the value of field `kendra_index` after provisioning.\n"]
    pub fn kendra_index(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kendra_index", self.base))
    }

    #[doc = "Get a reference to the value of field `query_filter_string` after provisioning.\n"]
    pub fn query_filter_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_filter_string", self.base))
    }

    #[doc = "Get a reference to the value of field `query_filter_string_enabled` after provisioning.\n"]
    pub fn query_filter_string_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_filter_string_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration_ExactResponseFields {
    answer_field: terrars::PrimField<std::string::String>,
    question_field: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration_ExactResponseFields { }

impl terrars
::ToListMappable for Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration_ExactResponseFields {
    type O =
        terrars
        ::BlockAssignable<
            Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration_ExactResponseFields,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration_ExactResponseFields {
    #[doc = ""]
    pub answer_field: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub question_field: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration_ExactResponseFields {
    pub fn build(
        self,
    ) -> Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration_ExactResponseFields {
        Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration_ExactResponseFields {
            answer_field: self.answer_field,
            question_field: self.question_field,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration_ExactResponseFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration_ExactResponseFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration_ExactResponseFieldsRef {
        Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration_ExactResponseFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration_ExactResponseFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `answer_field` after provisioning.\n"]
    pub fn answer_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.answer_field", self.base))
    }

    #[doc = "Get a reference to the value of field `question_field` after provisioning.\n"]
    pub fn question_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.question_field", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfigurationDynamic {
    exact_response_fields: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration_ExactResponseFields,
        >,
    >,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration {
    domain_endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact_response: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_fields: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    index_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact_response_fields: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration_ExactResponseFields,
        >,
    >,
    dynamic: Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfigurationDynamic,
}

impl Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration {
    #[doc = "Set the field `exact_response`.\n"]
    pub fn set_exact_response(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exact_response = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_fields`.\n"]
    pub fn set_include_fields(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exact_response_fields`.\n"]
    pub fn set_exact_response_fields(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration_ExactResponseFields,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exact_response_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exact_response_fields = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration {
    type O =
        terrars
        ::BlockAssignable<Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration {
    #[doc = ""]
    pub domain_endpoint: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub index_name: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration {
    pub fn build(self) -> Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration {
        Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration {
            domain_endpoint: self.domain_endpoint,
            exact_response: core::default::Default::default(),
            include_fields: core::default::Default::default(),
            index_name: self.index_name,
            exact_response_fields: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfigurationRef {
        Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_endpoint` after provisioning.\n"]
    pub fn domain_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `exact_response` after provisioning.\n"]
    pub fn exact_response(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact_response", self.base))
    }

    #[doc = "Get a reference to the value of field `include_fields` after provisioning.\n"]
    pub fn include_fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `index_name` after provisioning.\n"]
    pub fn index_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_name", self.base))
    }

    #[doc = "Get a reference to the value of field `exact_response_fields` after provisioning.\n"]
    pub fn exact_response_fields(
        &self,
    ) -> terrars
    ::ListRef<
        Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration_ExactResponseFieldsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exact_response_fields", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfigurationDynamic {
    bedrock_knowledge_store_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration,
        >,
    >,
    kendra_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_KendraConfiguration>,
    >,
    opensearch_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bedrock_knowledge_store_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kendra_configuration: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_KendraConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opensearch_configuration: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration>>,
    dynamic: Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfigurationDynamic,
}

impl Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration {
    #[doc = "Set the field `bedrock_knowledge_store_configuration`.\n"]
    pub fn set_bedrock_knowledge_store_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bedrock_knowledge_store_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bedrock_knowledge_store_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kendra_configuration`.\n"]
    pub fn set_kendra_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_KendraConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kendra_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kendra_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `opensearch_configuration`.\n"]
    pub fn set_opensearch_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.opensearch_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.opensearch_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration {}

impl BuildLexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration {
    pub fn build(self) -> Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration {
        Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration {
            bedrock_knowledge_store_configuration: core::default::Default::default(),
            kendra_configuration: core::default::Default::default(),
            opensearch_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfigurationRef {
        Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bedrock_knowledge_store_configuration` after provisioning.\n"]
    pub fn bedrock_knowledge_store_configuration(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_BedrockKnowledgeStoreConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bedrock_knowledge_store_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `kendra_configuration` after provisioning.\n"]
    pub fn kendra_configuration(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_KendraConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kendra_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `opensearch_configuration` after provisioning.\n"]
    pub fn opensearch_configuration(
        &self,
    ) -> terrars
    ::ListRef<Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration_OpensearchConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.opensearch_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct Lexv2modelsIntent_QnaIntentConfigurationDynamic {
    bedrock_model_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration>>,
    data_source_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration>>,
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_QnaIntentConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bedrock_model_configuration: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_configuration: std
    ::option
    ::Option<std::vec::Vec<Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration>>,
    dynamic: Lexv2modelsIntent_QnaIntentConfigurationDynamic,
}

impl Lexv2modelsIntent_QnaIntentConfiguration {
    #[doc = "Set the field `bedrock_model_configuration`.\n"]
    pub fn set_bedrock_model_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bedrock_model_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bedrock_model_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_source_configuration`.\n"]
    pub fn set_data_source_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_source_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_source_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Lexv2modelsIntent_QnaIntentConfiguration {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_QnaIntentConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_QnaIntentConfiguration {}

impl BuildLexv2modelsIntent_QnaIntentConfiguration {
    pub fn build(self) -> Lexv2modelsIntent_QnaIntentConfiguration {
        Lexv2modelsIntent_QnaIntentConfiguration {
            bedrock_model_configuration: core::default::Default::default(),
            data_source_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_QnaIntentConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_QnaIntentConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsIntent_QnaIntentConfigurationRef {
        Lexv2modelsIntent_QnaIntentConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_QnaIntentConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bedrock_model_configuration` after provisioning.\n"]
    pub fn bedrock_model_configuration(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_QnaIntentConfiguration_BedrockModelConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bedrock_model_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `data_source_configuration` after provisioning.\n"]
    pub fn data_source_configuration(
        &self,
    ) -> terrars::ListRef<Lexv2modelsIntent_QnaIntentConfiguration_DataSourceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_source_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_SampleUtterance {
    utterance: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsIntent_SampleUtterance { }

impl terrars::ToListMappable for Lexv2modelsIntent_SampleUtterance {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_SampleUtterance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_SampleUtterance {
    #[doc = ""]
    pub utterance: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsIntent_SampleUtterance {
    pub fn build(self) -> Lexv2modelsIntent_SampleUtterance {
        Lexv2modelsIntent_SampleUtterance { utterance: self.utterance }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_SampleUtteranceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_SampleUtteranceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsIntent_SampleUtteranceRef {
        Lexv2modelsIntent_SampleUtteranceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_SampleUtteranceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `utterance` after provisioning.\n"]
    pub fn utterance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.utterance", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_SlotPriority {
    priority: terrars::PrimField<f64>,
    slot_id: terrars::PrimField<std::string::String>,
}

impl Lexv2modelsIntent_SlotPriority { }

impl terrars::ToListMappable for Lexv2modelsIntent_SlotPriority {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_SlotPriority>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_SlotPriority {
    #[doc = ""]
    pub priority: terrars::PrimField<f64>,
    #[doc = ""]
    pub slot_id: terrars::PrimField<std::string::String>,
}

impl BuildLexv2modelsIntent_SlotPriority {
    pub fn build(self) -> Lexv2modelsIntent_SlotPriority {
        Lexv2modelsIntent_SlotPriority {
            priority: self.priority,
            slot_id: self.slot_id,
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_SlotPriorityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_SlotPriorityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsIntent_SlotPriorityRef {
        Lexv2modelsIntent_SlotPriorityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_SlotPriorityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `slot_id` after provisioning.\n"]
    pub fn slot_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slot_id", self.base))
    }
}

#[derive(Serialize)]
pub struct Lexv2modelsIntent_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Lexv2modelsIntent_Timeouts {
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

impl terrars::ToListMappable for Lexv2modelsIntent_Timeouts {
    type O = terrars::BlockAssignable<Lexv2modelsIntent_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexv2modelsIntent_Timeouts {}

impl BuildLexv2modelsIntent_Timeouts {
    pub fn build(self) -> Lexv2modelsIntent_Timeouts {
        Lexv2modelsIntent_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Lexv2modelsIntent_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Lexv2modelsIntent_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Lexv2modelsIntent_TimeoutsRef {
        Lexv2modelsIntent_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Lexv2modelsIntent_TimeoutsRef {
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
