use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LexIntentData {
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
    create_version: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent_intent_signature: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_utterances: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conclusion_statement: std::option::Option<std::vec::Vec<LexIntentConclusionStatementEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confirmation_prompt: std::option::Option<std::vec::Vec<LexIntentConfirmationPromptEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dialog_code_hook: std::option::Option<std::vec::Vec<LexIntentDialogCodeHookEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    follow_up_prompt: std::option::Option<std::vec::Vec<LexIntentFollowUpPromptEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fulfillment_activity: std::option::Option<std::vec::Vec<LexIntentFulfillmentActivityEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rejection_statement: std::option::Option<std::vec::Vec<LexIntentRejectionStatementEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slot: std::option::Option<std::vec::Vec<LexIntentSlotEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LexIntentTimeoutsEl>,
    dynamic: LexIntentDynamic,
}

struct LexIntent_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LexIntentData>,
}

#[derive(Clone)]
pub struct LexIntent(std::rc::Rc<LexIntent_>);

impl LexIntent {
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

    #[doc = "Set the field `create_version`.\n"]
    pub fn set_create_version(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().create_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `sample_utterances`.\n"]
    pub fn set_sample_utterances(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().sample_utterances = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conclusion_statement`.\n"]
    pub fn set_conclusion_statement(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntentConclusionStatementEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().conclusion_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.conclusion_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `confirmation_prompt`.\n"]
    pub fn set_confirmation_prompt(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntentConfirmationPromptEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().confirmation_prompt = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.confirmation_prompt = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dialog_code_hook`.\n"]
    pub fn set_dialog_code_hook(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntentDialogCodeHookEl>>,
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

    #[doc = "Set the field `follow_up_prompt`.\n"]
    pub fn set_follow_up_prompt(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntentFollowUpPromptEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().follow_up_prompt = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.follow_up_prompt = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fulfillment_activity`.\n"]
    pub fn set_fulfillment_activity(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntentFulfillmentActivityEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fulfillment_activity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fulfillment_activity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rejection_statement`.\n"]
    pub fn set_rejection_statement(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntentRejectionStatementEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rejection_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rejection_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `slot`.\n"]
    pub fn set_slot(self, v: impl std::convert::Into<terrars::BlockAssignable<LexIntentSlotEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().slot = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.slot = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<LexIntentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `checksum` after provisioning.\n"]
    pub fn checksum(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.checksum", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_version` after provisioning.\n"]
    pub fn create_version(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_intent_signature` after provisioning.\n"]
    pub fn parent_intent_signature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_intent_signature", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sample_utterances` after provisioning.\n"]
    pub fn sample_utterances(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.sample_utterances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `conclusion_statement` after provisioning.\n"]
    pub fn conclusion_statement(&self) -> terrars::ListRef<LexIntentConclusionStatementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conclusion_statement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confirmation_prompt` after provisioning.\n"]
    pub fn confirmation_prompt(&self) -> terrars::ListRef<LexIntentConfirmationPromptElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confirmation_prompt", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dialog_code_hook` after provisioning.\n"]
    pub fn dialog_code_hook(&self) -> terrars::ListRef<LexIntentDialogCodeHookElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dialog_code_hook", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `follow_up_prompt` after provisioning.\n"]
    pub fn follow_up_prompt(&self) -> terrars::ListRef<LexIntentFollowUpPromptElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.follow_up_prompt", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fulfillment_activity` after provisioning.\n"]
    pub fn fulfillment_activity(&self) -> terrars::ListRef<LexIntentFulfillmentActivityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fulfillment_activity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rejection_statement` after provisioning.\n"]
    pub fn rejection_statement(&self) -> terrars::ListRef<LexIntentRejectionStatementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rejection_statement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LexIntentTimeoutsElRef {
        LexIntentTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for LexIntent {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LexIntent { }

impl terrars::ToListMappable for LexIntent {
    type O = terrars::ListRef<LexIntentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LexIntent_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lex_intent".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLexIntent {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildLexIntent {
    pub fn build(self, stack: &mut terrars::Stack) -> LexIntent {
        let out = LexIntent(std::rc::Rc::new(LexIntent_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LexIntentData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                create_version: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                parent_intent_signature: core::default::Default::default(),
                sample_utterances: core::default::Default::default(),
                conclusion_statement: core::default::Default::default(),
                confirmation_prompt: core::default::Default::default(),
                dialog_code_hook: core::default::Default::default(),
                follow_up_prompt: core::default::Default::default(),
                fulfillment_activity: core::default::Default::default(),
                rejection_statement: core::default::Default::default(),
                slot: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LexIntentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LexIntentRef {
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

    #[doc = "Get a reference to the value of field `checksum` after provisioning.\n"]
    pub fn checksum(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.checksum", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_version` after provisioning.\n"]
    pub fn create_version(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_intent_signature` after provisioning.\n"]
    pub fn parent_intent_signature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_intent_signature", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sample_utterances` after provisioning.\n"]
    pub fn sample_utterances(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.sample_utterances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `conclusion_statement` after provisioning.\n"]
    pub fn conclusion_statement(&self) -> terrars::ListRef<LexIntentConclusionStatementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conclusion_statement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confirmation_prompt` after provisioning.\n"]
    pub fn confirmation_prompt(&self) -> terrars::ListRef<LexIntentConfirmationPromptElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confirmation_prompt", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dialog_code_hook` after provisioning.\n"]
    pub fn dialog_code_hook(&self) -> terrars::ListRef<LexIntentDialogCodeHookElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dialog_code_hook", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `follow_up_prompt` after provisioning.\n"]
    pub fn follow_up_prompt(&self) -> terrars::ListRef<LexIntentFollowUpPromptElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.follow_up_prompt", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fulfillment_activity` after provisioning.\n"]
    pub fn fulfillment_activity(&self) -> terrars::ListRef<LexIntentFulfillmentActivityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fulfillment_activity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rejection_statement` after provisioning.\n"]
    pub fn rejection_statement(&self) -> terrars::ListRef<LexIntentRejectionStatementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rejection_statement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LexIntentTimeoutsElRef {
        LexIntentTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LexIntentConclusionStatementElMessageEl {
    content: terrars::PrimField<std::string::String>,
    content_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_number: std::option::Option<terrars::PrimField<f64>>,
}

impl LexIntentConclusionStatementElMessageEl {
    #[doc = "Set the field `group_number`.\n"]
    pub fn set_group_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.group_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LexIntentConclusionStatementElMessageEl {
    type O = terrars::BlockAssignable<LexIntentConclusionStatementElMessageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntentConclusionStatementElMessageEl {
    #[doc = ""]
    pub content: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub content_type: terrars::PrimField<std::string::String>,
}

impl BuildLexIntentConclusionStatementElMessageEl {
    pub fn build(self) -> LexIntentConclusionStatementElMessageEl {
        LexIntentConclusionStatementElMessageEl {
            content: self.content,
            content_type: self.content_type,
            group_number: core::default::Default::default(),
        }
    }
}

pub struct LexIntentConclusionStatementElMessageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntentConclusionStatementElMessageElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntentConclusionStatementElMessageElRef {
        LexIntentConclusionStatementElMessageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntentConclusionStatementElMessageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `group_number` after provisioning.\n"]
    pub fn group_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_number", self.base))
    }
}

#[derive(Serialize, Default)]
struct LexIntentConclusionStatementElDynamic {
    message: std::option::Option<terrars::DynamicBlock<LexIntentConclusionStatementElMessageEl>>,
}

#[derive(Serialize)]
pub struct LexIntentConclusionStatementEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_card: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<std::vec::Vec<LexIntentConclusionStatementElMessageEl>>,
    dynamic: LexIntentConclusionStatementElDynamic,
}

impl LexIntentConclusionStatementEl {
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response_card = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntentConclusionStatementElMessageEl>>,
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
}

impl terrars::ToListMappable for LexIntentConclusionStatementEl {
    type O = terrars::BlockAssignable<LexIntentConclusionStatementEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntentConclusionStatementEl {}

impl BuildLexIntentConclusionStatementEl {
    pub fn build(self) -> LexIntentConclusionStatementEl {
        LexIntentConclusionStatementEl {
            response_card: core::default::Default::default(),
            message: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LexIntentConclusionStatementElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntentConclusionStatementElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntentConclusionStatementElRef {
        LexIntentConclusionStatementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntentConclusionStatementElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `response_card` after provisioning.\n"]
    pub fn response_card(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_card", self.base))
    }
}

#[derive(Serialize)]
pub struct LexIntentConfirmationPromptElMessageEl {
    content: terrars::PrimField<std::string::String>,
    content_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_number: std::option::Option<terrars::PrimField<f64>>,
}

impl LexIntentConfirmationPromptElMessageEl {
    #[doc = "Set the field `group_number`.\n"]
    pub fn set_group_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.group_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LexIntentConfirmationPromptElMessageEl {
    type O = terrars::BlockAssignable<LexIntentConfirmationPromptElMessageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntentConfirmationPromptElMessageEl {
    #[doc = ""]
    pub content: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub content_type: terrars::PrimField<std::string::String>,
}

impl BuildLexIntentConfirmationPromptElMessageEl {
    pub fn build(self) -> LexIntentConfirmationPromptElMessageEl {
        LexIntentConfirmationPromptElMessageEl {
            content: self.content,
            content_type: self.content_type,
            group_number: core::default::Default::default(),
        }
    }
}

pub struct LexIntentConfirmationPromptElMessageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntentConfirmationPromptElMessageElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntentConfirmationPromptElMessageElRef {
        LexIntentConfirmationPromptElMessageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntentConfirmationPromptElMessageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `group_number` after provisioning.\n"]
    pub fn group_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_number", self.base))
    }
}

#[derive(Serialize, Default)]
struct LexIntentConfirmationPromptElDynamic {
    message: std::option::Option<terrars::DynamicBlock<LexIntentConfirmationPromptElMessageEl>>,
}

#[derive(Serialize)]
pub struct LexIntentConfirmationPromptEl {
    max_attempts: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_card: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<std::vec::Vec<LexIntentConfirmationPromptElMessageEl>>,
    dynamic: LexIntentConfirmationPromptElDynamic,
}

impl LexIntentConfirmationPromptEl {
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response_card = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntentConfirmationPromptElMessageEl>>,
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
}

impl terrars::ToListMappable for LexIntentConfirmationPromptEl {
    type O = terrars::BlockAssignable<LexIntentConfirmationPromptEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntentConfirmationPromptEl {
    #[doc = ""]
    pub max_attempts: terrars::PrimField<f64>,
}

impl BuildLexIntentConfirmationPromptEl {
    pub fn build(self) -> LexIntentConfirmationPromptEl {
        LexIntentConfirmationPromptEl {
            max_attempts: self.max_attempts,
            response_card: core::default::Default::default(),
            message: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LexIntentConfirmationPromptElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntentConfirmationPromptElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntentConfirmationPromptElRef {
        LexIntentConfirmationPromptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntentConfirmationPromptElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_attempts` after provisioning.\n"]
    pub fn max_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_attempts", self.base))
    }

    #[doc = "Get a reference to the value of field `response_card` after provisioning.\n"]
    pub fn response_card(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_card", self.base))
    }
}

#[derive(Serialize)]
pub struct LexIntentDialogCodeHookEl {
    message_version: terrars::PrimField<std::string::String>,
    uri: terrars::PrimField<std::string::String>,
}

impl LexIntentDialogCodeHookEl { }

impl terrars::ToListMappable for LexIntentDialogCodeHookEl {
    type O = terrars::BlockAssignable<LexIntentDialogCodeHookEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntentDialogCodeHookEl {
    #[doc = ""]
    pub message_version: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildLexIntentDialogCodeHookEl {
    pub fn build(self) -> LexIntentDialogCodeHookEl {
        LexIntentDialogCodeHookEl {
            message_version: self.message_version,
            uri: self.uri,
        }
    }
}

pub struct LexIntentDialogCodeHookElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntentDialogCodeHookElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntentDialogCodeHookElRef {
        LexIntentDialogCodeHookElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntentDialogCodeHookElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message_version` after provisioning.\n"]
    pub fn message_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_version", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct LexIntentFollowUpPromptElPromptElMessageEl {
    content: terrars::PrimField<std::string::String>,
    content_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_number: std::option::Option<terrars::PrimField<f64>>,
}

impl LexIntentFollowUpPromptElPromptElMessageEl {
    #[doc = "Set the field `group_number`.\n"]
    pub fn set_group_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.group_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LexIntentFollowUpPromptElPromptElMessageEl {
    type O = terrars::BlockAssignable<LexIntentFollowUpPromptElPromptElMessageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntentFollowUpPromptElPromptElMessageEl {
    #[doc = ""]
    pub content: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub content_type: terrars::PrimField<std::string::String>,
}

impl BuildLexIntentFollowUpPromptElPromptElMessageEl {
    pub fn build(self) -> LexIntentFollowUpPromptElPromptElMessageEl {
        LexIntentFollowUpPromptElPromptElMessageEl {
            content: self.content,
            content_type: self.content_type,
            group_number: core::default::Default::default(),
        }
    }
}

pub struct LexIntentFollowUpPromptElPromptElMessageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntentFollowUpPromptElPromptElMessageElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntentFollowUpPromptElPromptElMessageElRef {
        LexIntentFollowUpPromptElPromptElMessageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntentFollowUpPromptElPromptElMessageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `group_number` after provisioning.\n"]
    pub fn group_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_number", self.base))
    }
}

#[derive(Serialize, Default)]
struct LexIntentFollowUpPromptElPromptElDynamic {
    message: std::option::Option<terrars::DynamicBlock<LexIntentFollowUpPromptElPromptElMessageEl>>,
}

#[derive(Serialize)]
pub struct LexIntentFollowUpPromptElPromptEl {
    max_attempts: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_card: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<std::vec::Vec<LexIntentFollowUpPromptElPromptElMessageEl>>,
    dynamic: LexIntentFollowUpPromptElPromptElDynamic,
}

impl LexIntentFollowUpPromptElPromptEl {
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response_card = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntentFollowUpPromptElPromptElMessageEl>>,
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
}

impl terrars::ToListMappable for LexIntentFollowUpPromptElPromptEl {
    type O = terrars::BlockAssignable<LexIntentFollowUpPromptElPromptEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntentFollowUpPromptElPromptEl {
    #[doc = ""]
    pub max_attempts: terrars::PrimField<f64>,
}

impl BuildLexIntentFollowUpPromptElPromptEl {
    pub fn build(self) -> LexIntentFollowUpPromptElPromptEl {
        LexIntentFollowUpPromptElPromptEl {
            max_attempts: self.max_attempts,
            response_card: core::default::Default::default(),
            message: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LexIntentFollowUpPromptElPromptElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntentFollowUpPromptElPromptElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntentFollowUpPromptElPromptElRef {
        LexIntentFollowUpPromptElPromptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntentFollowUpPromptElPromptElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_attempts` after provisioning.\n"]
    pub fn max_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_attempts", self.base))
    }

    #[doc = "Get a reference to the value of field `response_card` after provisioning.\n"]
    pub fn response_card(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_card", self.base))
    }
}

#[derive(Serialize)]
pub struct LexIntentFollowUpPromptElRejectionStatementElMessageEl {
    content: terrars::PrimField<std::string::String>,
    content_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_number: std::option::Option<terrars::PrimField<f64>>,
}

impl LexIntentFollowUpPromptElRejectionStatementElMessageEl {
    #[doc = "Set the field `group_number`.\n"]
    pub fn set_group_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.group_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LexIntentFollowUpPromptElRejectionStatementElMessageEl {
    type O = terrars::BlockAssignable<LexIntentFollowUpPromptElRejectionStatementElMessageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntentFollowUpPromptElRejectionStatementElMessageEl {
    #[doc = ""]
    pub content: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub content_type: terrars::PrimField<std::string::String>,
}

impl BuildLexIntentFollowUpPromptElRejectionStatementElMessageEl {
    pub fn build(self) -> LexIntentFollowUpPromptElRejectionStatementElMessageEl {
        LexIntentFollowUpPromptElRejectionStatementElMessageEl {
            content: self.content,
            content_type: self.content_type,
            group_number: core::default::Default::default(),
        }
    }
}

pub struct LexIntentFollowUpPromptElRejectionStatementElMessageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntentFollowUpPromptElRejectionStatementElMessageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LexIntentFollowUpPromptElRejectionStatementElMessageElRef {
        LexIntentFollowUpPromptElRejectionStatementElMessageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntentFollowUpPromptElRejectionStatementElMessageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `group_number` after provisioning.\n"]
    pub fn group_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_number", self.base))
    }
}

#[derive(Serialize, Default)]
struct LexIntentFollowUpPromptElRejectionStatementElDynamic {
    message: std::option::Option<terrars::DynamicBlock<LexIntentFollowUpPromptElRejectionStatementElMessageEl>>,
}

#[derive(Serialize)]
pub struct LexIntentFollowUpPromptElRejectionStatementEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_card: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<std::vec::Vec<LexIntentFollowUpPromptElRejectionStatementElMessageEl>>,
    dynamic: LexIntentFollowUpPromptElRejectionStatementElDynamic,
}

impl LexIntentFollowUpPromptElRejectionStatementEl {
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response_card = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntentFollowUpPromptElRejectionStatementElMessageEl>>,
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
}

impl terrars::ToListMappable for LexIntentFollowUpPromptElRejectionStatementEl {
    type O = terrars::BlockAssignable<LexIntentFollowUpPromptElRejectionStatementEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntentFollowUpPromptElRejectionStatementEl {}

impl BuildLexIntentFollowUpPromptElRejectionStatementEl {
    pub fn build(self) -> LexIntentFollowUpPromptElRejectionStatementEl {
        LexIntentFollowUpPromptElRejectionStatementEl {
            response_card: core::default::Default::default(),
            message: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LexIntentFollowUpPromptElRejectionStatementElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntentFollowUpPromptElRejectionStatementElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LexIntentFollowUpPromptElRejectionStatementElRef {
        LexIntentFollowUpPromptElRejectionStatementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntentFollowUpPromptElRejectionStatementElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `response_card` after provisioning.\n"]
    pub fn response_card(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_card", self.base))
    }
}

#[derive(Serialize, Default)]
struct LexIntentFollowUpPromptElDynamic {
    prompt: std::option::Option<terrars::DynamicBlock<LexIntentFollowUpPromptElPromptEl>>,
    rejection_statement: std::option::Option<terrars::DynamicBlock<LexIntentFollowUpPromptElRejectionStatementEl>>,
}

#[derive(Serialize)]
pub struct LexIntentFollowUpPromptEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt: std::option::Option<std::vec::Vec<LexIntentFollowUpPromptElPromptEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rejection_statement: std::option::Option<std::vec::Vec<LexIntentFollowUpPromptElRejectionStatementEl>>,
    dynamic: LexIntentFollowUpPromptElDynamic,
}

impl LexIntentFollowUpPromptEl {
    #[doc = "Set the field `prompt`.\n"]
    pub fn set_prompt(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntentFollowUpPromptElPromptEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.prompt = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.prompt = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rejection_statement`.\n"]
    pub fn set_rejection_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntentFollowUpPromptElRejectionStatementEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rejection_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rejection_statement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LexIntentFollowUpPromptEl {
    type O = terrars::BlockAssignable<LexIntentFollowUpPromptEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntentFollowUpPromptEl {}

impl BuildLexIntentFollowUpPromptEl {
    pub fn build(self) -> LexIntentFollowUpPromptEl {
        LexIntentFollowUpPromptEl {
            prompt: core::default::Default::default(),
            rejection_statement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LexIntentFollowUpPromptElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntentFollowUpPromptElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntentFollowUpPromptElRef {
        LexIntentFollowUpPromptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntentFollowUpPromptElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prompt` after provisioning.\n"]
    pub fn prompt(&self) -> terrars::ListRef<LexIntentFollowUpPromptElPromptElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prompt", self.base))
    }

    #[doc = "Get a reference to the value of field `rejection_statement` after provisioning.\n"]
    pub fn rejection_statement(&self) -> terrars::ListRef<LexIntentFollowUpPromptElRejectionStatementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rejection_statement", self.base))
    }
}

#[derive(Serialize)]
pub struct LexIntentFulfillmentActivityElCodeHookEl {
    message_version: terrars::PrimField<std::string::String>,
    uri: terrars::PrimField<std::string::String>,
}

impl LexIntentFulfillmentActivityElCodeHookEl { }

impl terrars::ToListMappable for LexIntentFulfillmentActivityElCodeHookEl {
    type O = terrars::BlockAssignable<LexIntentFulfillmentActivityElCodeHookEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntentFulfillmentActivityElCodeHookEl {
    #[doc = ""]
    pub message_version: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildLexIntentFulfillmentActivityElCodeHookEl {
    pub fn build(self) -> LexIntentFulfillmentActivityElCodeHookEl {
        LexIntentFulfillmentActivityElCodeHookEl {
            message_version: self.message_version,
            uri: self.uri,
        }
    }
}

pub struct LexIntentFulfillmentActivityElCodeHookElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntentFulfillmentActivityElCodeHookElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntentFulfillmentActivityElCodeHookElRef {
        LexIntentFulfillmentActivityElCodeHookElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntentFulfillmentActivityElCodeHookElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message_version` after provisioning.\n"]
    pub fn message_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_version", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct LexIntentFulfillmentActivityElDynamic {
    code_hook: std::option::Option<terrars::DynamicBlock<LexIntentFulfillmentActivityElCodeHookEl>>,
}

#[derive(Serialize)]
pub struct LexIntentFulfillmentActivityEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_hook: std::option::Option<std::vec::Vec<LexIntentFulfillmentActivityElCodeHookEl>>,
    dynamic: LexIntentFulfillmentActivityElDynamic,
}

impl LexIntentFulfillmentActivityEl {
    #[doc = "Set the field `code_hook`.\n"]
    pub fn set_code_hook(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntentFulfillmentActivityElCodeHookEl>>,
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
}

impl terrars::ToListMappable for LexIntentFulfillmentActivityEl {
    type O = terrars::BlockAssignable<LexIntentFulfillmentActivityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntentFulfillmentActivityEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildLexIntentFulfillmentActivityEl {
    pub fn build(self) -> LexIntentFulfillmentActivityEl {
        LexIntentFulfillmentActivityEl {
            type_: self.type_,
            code_hook: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LexIntentFulfillmentActivityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntentFulfillmentActivityElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntentFulfillmentActivityElRef {
        LexIntentFulfillmentActivityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntentFulfillmentActivityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `code_hook` after provisioning.\n"]
    pub fn code_hook(&self) -> terrars::ListRef<LexIntentFulfillmentActivityElCodeHookElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_hook", self.base))
    }
}

#[derive(Serialize)]
pub struct LexIntentRejectionStatementElMessageEl {
    content: terrars::PrimField<std::string::String>,
    content_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_number: std::option::Option<terrars::PrimField<f64>>,
}

impl LexIntentRejectionStatementElMessageEl {
    #[doc = "Set the field `group_number`.\n"]
    pub fn set_group_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.group_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LexIntentRejectionStatementElMessageEl {
    type O = terrars::BlockAssignable<LexIntentRejectionStatementElMessageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntentRejectionStatementElMessageEl {
    #[doc = ""]
    pub content: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub content_type: terrars::PrimField<std::string::String>,
}

impl BuildLexIntentRejectionStatementElMessageEl {
    pub fn build(self) -> LexIntentRejectionStatementElMessageEl {
        LexIntentRejectionStatementElMessageEl {
            content: self.content,
            content_type: self.content_type,
            group_number: core::default::Default::default(),
        }
    }
}

pub struct LexIntentRejectionStatementElMessageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntentRejectionStatementElMessageElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntentRejectionStatementElMessageElRef {
        LexIntentRejectionStatementElMessageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntentRejectionStatementElMessageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `group_number` after provisioning.\n"]
    pub fn group_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_number", self.base))
    }
}

#[derive(Serialize, Default)]
struct LexIntentRejectionStatementElDynamic {
    message: std::option::Option<terrars::DynamicBlock<LexIntentRejectionStatementElMessageEl>>,
}

#[derive(Serialize)]
pub struct LexIntentRejectionStatementEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_card: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<std::vec::Vec<LexIntentRejectionStatementElMessageEl>>,
    dynamic: LexIntentRejectionStatementElDynamic,
}

impl LexIntentRejectionStatementEl {
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response_card = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntentRejectionStatementElMessageEl>>,
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
}

impl terrars::ToListMappable for LexIntentRejectionStatementEl {
    type O = terrars::BlockAssignable<LexIntentRejectionStatementEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntentRejectionStatementEl {}

impl BuildLexIntentRejectionStatementEl {
    pub fn build(self) -> LexIntentRejectionStatementEl {
        LexIntentRejectionStatementEl {
            response_card: core::default::Default::default(),
            message: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LexIntentRejectionStatementElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntentRejectionStatementElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntentRejectionStatementElRef {
        LexIntentRejectionStatementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntentRejectionStatementElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `response_card` after provisioning.\n"]
    pub fn response_card(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_card", self.base))
    }
}

#[derive(Serialize)]
pub struct LexIntentSlotElValueElicitationPromptElMessageEl {
    content: terrars::PrimField<std::string::String>,
    content_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_number: std::option::Option<terrars::PrimField<f64>>,
}

impl LexIntentSlotElValueElicitationPromptElMessageEl {
    #[doc = "Set the field `group_number`.\n"]
    pub fn set_group_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.group_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LexIntentSlotElValueElicitationPromptElMessageEl {
    type O = terrars::BlockAssignable<LexIntentSlotElValueElicitationPromptElMessageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntentSlotElValueElicitationPromptElMessageEl {
    #[doc = ""]
    pub content: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub content_type: terrars::PrimField<std::string::String>,
}

impl BuildLexIntentSlotElValueElicitationPromptElMessageEl {
    pub fn build(self) -> LexIntentSlotElValueElicitationPromptElMessageEl {
        LexIntentSlotElValueElicitationPromptElMessageEl {
            content: self.content,
            content_type: self.content_type,
            group_number: core::default::Default::default(),
        }
    }
}

pub struct LexIntentSlotElValueElicitationPromptElMessageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntentSlotElValueElicitationPromptElMessageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LexIntentSlotElValueElicitationPromptElMessageElRef {
        LexIntentSlotElValueElicitationPromptElMessageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntentSlotElValueElicitationPromptElMessageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `group_number` after provisioning.\n"]
    pub fn group_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_number", self.base))
    }
}

#[derive(Serialize, Default)]
struct LexIntentSlotElValueElicitationPromptElDynamic {
    message: std::option::Option<terrars::DynamicBlock<LexIntentSlotElValueElicitationPromptElMessageEl>>,
}

#[derive(Serialize)]
pub struct LexIntentSlotElValueElicitationPromptEl {
    max_attempts: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_card: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<std::vec::Vec<LexIntentSlotElValueElicitationPromptElMessageEl>>,
    dynamic: LexIntentSlotElValueElicitationPromptElDynamic,
}

impl LexIntentSlotElValueElicitationPromptEl {
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response_card = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntentSlotElValueElicitationPromptElMessageEl>>,
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
}

impl terrars::ToListMappable for LexIntentSlotElValueElicitationPromptEl {
    type O = terrars::BlockAssignable<LexIntentSlotElValueElicitationPromptEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntentSlotElValueElicitationPromptEl {
    #[doc = ""]
    pub max_attempts: terrars::PrimField<f64>,
}

impl BuildLexIntentSlotElValueElicitationPromptEl {
    pub fn build(self) -> LexIntentSlotElValueElicitationPromptEl {
        LexIntentSlotElValueElicitationPromptEl {
            max_attempts: self.max_attempts,
            response_card: core::default::Default::default(),
            message: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LexIntentSlotElValueElicitationPromptElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntentSlotElValueElicitationPromptElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntentSlotElValueElicitationPromptElRef {
        LexIntentSlotElValueElicitationPromptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntentSlotElValueElicitationPromptElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_attempts` after provisioning.\n"]
    pub fn max_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_attempts", self.base))
    }

    #[doc = "Get a reference to the value of field `response_card` after provisioning.\n"]
    pub fn response_card(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_card", self.base))
    }
}

#[derive(Serialize, Default)]
struct LexIntentSlotElDynamic {
    value_elicitation_prompt: std::option::Option<terrars::DynamicBlock<LexIntentSlotElValueElicitationPromptEl>>,
}

#[derive(Serialize)]
pub struct LexIntentSlotEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_card: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_utterances: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    slot_constraint: terrars::PrimField<std::string::String>,
    slot_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slot_type_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_elicitation_prompt: std::option::Option<std::vec::Vec<LexIntentSlotElValueElicitationPromptEl>>,
    dynamic: LexIntentSlotElDynamic,
}

impl LexIntentSlotEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response_card = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sample_utterances`.\n"]
    pub fn set_sample_utterances(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.sample_utterances = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slot_type_version`.\n"]
    pub fn set_slot_type_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.slot_type_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value_elicitation_prompt`.\n"]
    pub fn set_value_elicitation_prompt(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntentSlotElValueElicitationPromptEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.value_elicitation_prompt = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.value_elicitation_prompt = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LexIntentSlotEl {
    type O = terrars::BlockAssignable<LexIntentSlotEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntentSlotEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub slot_constraint: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub slot_type: terrars::PrimField<std::string::String>,
}

impl BuildLexIntentSlotEl {
    pub fn build(self) -> LexIntentSlotEl {
        LexIntentSlotEl {
            description: core::default::Default::default(),
            name: self.name,
            priority: core::default::Default::default(),
            response_card: core::default::Default::default(),
            sample_utterances: core::default::Default::default(),
            slot_constraint: self.slot_constraint,
            slot_type: self.slot_type,
            slot_type_version: core::default::Default::default(),
            value_elicitation_prompt: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LexIntentSlotElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntentSlotElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntentSlotElRef {
        LexIntentSlotElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntentSlotElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `response_card` after provisioning.\n"]
    pub fn response_card(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_card", self.base))
    }

    #[doc = "Get a reference to the value of field `sample_utterances` after provisioning.\n"]
    pub fn sample_utterances(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sample_utterances", self.base))
    }

    #[doc = "Get a reference to the value of field `slot_constraint` after provisioning.\n"]
    pub fn slot_constraint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slot_constraint", self.base))
    }

    #[doc = "Get a reference to the value of field `slot_type` after provisioning.\n"]
    pub fn slot_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slot_type", self.base))
    }

    #[doc = "Get a reference to the value of field `slot_type_version` after provisioning.\n"]
    pub fn slot_type_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slot_type_version", self.base))
    }

    #[doc = "Get a reference to the value of field `value_elicitation_prompt` after provisioning.\n"]
    pub fn value_elicitation_prompt(&self) -> terrars::ListRef<LexIntentSlotElValueElicitationPromptElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value_elicitation_prompt", self.base))
    }
}

#[derive(Serialize)]
pub struct LexIntentTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LexIntentTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LexIntentTimeoutsEl {
    type O = terrars::BlockAssignable<LexIntentTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntentTimeoutsEl {}

impl BuildLexIntentTimeoutsEl {
    pub fn build(self) -> LexIntentTimeoutsEl {
        LexIntentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct LexIntentTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntentTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntentTimeoutsElRef {
        LexIntentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntentTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct LexIntentDynamic {
    conclusion_statement: std::option::Option<terrars::DynamicBlock<LexIntentConclusionStatementEl>>,
    confirmation_prompt: std::option::Option<terrars::DynamicBlock<LexIntentConfirmationPromptEl>>,
    dialog_code_hook: std::option::Option<terrars::DynamicBlock<LexIntentDialogCodeHookEl>>,
    follow_up_prompt: std::option::Option<terrars::DynamicBlock<LexIntentFollowUpPromptEl>>,
    fulfillment_activity: std::option::Option<terrars::DynamicBlock<LexIntentFulfillmentActivityEl>>,
    rejection_statement: std::option::Option<terrars::DynamicBlock<LexIntentRejectionStatementEl>>,
    slot: std::option::Option<terrars::DynamicBlock<LexIntentSlotEl>>,
}
