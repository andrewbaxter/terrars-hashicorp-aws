#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LexIntentData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_utterances: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conclusion_statement: std::option::Option<std::vec::Vec<LexIntent_ConclusionStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confirmation_prompt: std::option::Option<std::vec::Vec<LexIntent_ConfirmationPrompt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dialog_code_hook: std::option::Option<std::vec::Vec<LexIntent_DialogCodeHook>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    follow_up_prompt: std::option::Option<std::vec::Vec<LexIntent_FollowUpPrompt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fulfillment_activity: std::option::Option<std::vec::Vec<LexIntent_FulfillmentActivity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rejection_statement: std::option::Option<std::vec::Vec<LexIntent_RejectionStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slot: std::option::Option<std::vec::Vec<LexIntent_Slot>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LexIntent_Timeouts>,
    dynamic: LexIntentDynamic,
}

struct LexIntent_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LexIntentData_>,
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntent_ConclusionStatement>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntent_ConfirmationPrompt>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntent_DialogCodeHook>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntent_FollowUpPrompt>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntent_FulfillmentActivity>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntent_RejectionStatement>>,
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
    pub fn set_slot(self, v: impl std::convert::Into<terrars::BlockAssignable<LexIntent_Slot>>) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<LexIntent_Timeouts>) -> Self {
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
    pub fn conclusion_statement(&self) -> terrars::ListRef<LexIntent_ConclusionStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conclusion_statement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confirmation_prompt` after provisioning.\n"]
    pub fn confirmation_prompt(&self) -> terrars::ListRef<LexIntent_ConfirmationPromptRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confirmation_prompt", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dialog_code_hook` after provisioning.\n"]
    pub fn dialog_code_hook(&self) -> terrars::ListRef<LexIntent_DialogCodeHookRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dialog_code_hook", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `follow_up_prompt` after provisioning.\n"]
    pub fn follow_up_prompt(&self) -> terrars::ListRef<LexIntent_FollowUpPromptRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.follow_up_prompt", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fulfillment_activity` after provisioning.\n"]
    pub fn fulfillment_activity(&self) -> terrars::ListRef<LexIntent_FulfillmentActivityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fulfillment_activity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rejection_statement` after provisioning.\n"]
    pub fn rejection_statement(&self) -> terrars::ListRef<LexIntent_RejectionStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rejection_statement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LexIntent_TimeoutsRef {
        LexIntent_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(LexIntentData_ {
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
                region: core::default::Default::default(),
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
    pub fn conclusion_statement(&self) -> terrars::ListRef<LexIntent_ConclusionStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conclusion_statement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confirmation_prompt` after provisioning.\n"]
    pub fn confirmation_prompt(&self) -> terrars::ListRef<LexIntent_ConfirmationPromptRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confirmation_prompt", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dialog_code_hook` after provisioning.\n"]
    pub fn dialog_code_hook(&self) -> terrars::ListRef<LexIntent_DialogCodeHookRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dialog_code_hook", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `follow_up_prompt` after provisioning.\n"]
    pub fn follow_up_prompt(&self) -> terrars::ListRef<LexIntent_FollowUpPromptRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.follow_up_prompt", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fulfillment_activity` after provisioning.\n"]
    pub fn fulfillment_activity(&self) -> terrars::ListRef<LexIntent_FulfillmentActivityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fulfillment_activity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rejection_statement` after provisioning.\n"]
    pub fn rejection_statement(&self) -> terrars::ListRef<LexIntent_RejectionStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rejection_statement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LexIntent_TimeoutsRef {
        LexIntent_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct LexIntentDynamic {
    conclusion_statement: std::option::Option<terrars::DynamicBlock<LexIntent_ConclusionStatement>>,
    confirmation_prompt: std::option::Option<terrars::DynamicBlock<LexIntent_ConfirmationPrompt>>,
    dialog_code_hook: std::option::Option<terrars::DynamicBlock<LexIntent_DialogCodeHook>>,
    follow_up_prompt: std::option::Option<terrars::DynamicBlock<LexIntent_FollowUpPrompt>>,
    fulfillment_activity: std::option::Option<terrars::DynamicBlock<LexIntent_FulfillmentActivity>>,
    rejection_statement: std::option::Option<terrars::DynamicBlock<LexIntent_RejectionStatement>>,
    slot: std::option::Option<terrars::DynamicBlock<LexIntent_Slot>>,
}

#[derive(Serialize)]
pub struct LexIntent_ConfirmationPrompt_Message {
    content: terrars::PrimField<std::string::String>,
    content_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_number: std::option::Option<terrars::PrimField<f64>>,
}

impl LexIntent_ConfirmationPrompt_Message {
    #[doc = "Set the field `group_number`.\n"]
    pub fn set_group_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.group_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LexIntent_ConfirmationPrompt_Message {
    type O = terrars::BlockAssignable<LexIntent_ConfirmationPrompt_Message>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntent_ConfirmationPrompt_Message {
    #[doc = ""]
    pub content: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub content_type: terrars::PrimField<std::string::String>,
}

impl BuildLexIntent_ConfirmationPrompt_Message {
    pub fn build(self) -> LexIntent_ConfirmationPrompt_Message {
        LexIntent_ConfirmationPrompt_Message {
            content: self.content,
            content_type: self.content_type,
            group_number: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LexIntent_ConfirmationPrompt_MessageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntent_ConfirmationPrompt_MessageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntent_ConfirmationPrompt_MessageRef {
        LexIntent_ConfirmationPrompt_MessageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntent_ConfirmationPrompt_MessageRef {
    #[allow(dead_code)]
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
struct LexIntent_ConclusionStatementDynamic {
    message: std::option::Option<terrars::DynamicBlock<LexIntent_ConfirmationPrompt_Message>>,
}

#[derive(Serialize)]
pub struct LexIntent_ConclusionStatement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_card: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<std::vec::Vec<LexIntent_ConfirmationPrompt_Message>>,
    dynamic: LexIntent_ConclusionStatementDynamic,
}

impl LexIntent_ConclusionStatement {
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response_card = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntent_ConfirmationPrompt_Message>>,
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

impl terrars::ToListMappable for LexIntent_ConclusionStatement {
    type O = terrars::BlockAssignable<LexIntent_ConclusionStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntent_ConclusionStatement {}

impl BuildLexIntent_ConclusionStatement {
    pub fn build(self) -> LexIntent_ConclusionStatement {
        LexIntent_ConclusionStatement {
            response_card: core::default::Default::default(),
            message: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LexIntent_ConclusionStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntent_ConclusionStatementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntent_ConclusionStatementRef {
        LexIntent_ConclusionStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntent_ConclusionStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `response_card` after provisioning.\n"]
    pub fn response_card(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_card", self.base))
    }
}

#[derive(Serialize, Default)]
struct LexIntent_ConfirmationPromptDynamic {
    message: std::option::Option<terrars::DynamicBlock<LexIntent_ConfirmationPrompt_Message>>,
}

#[derive(Serialize)]
pub struct LexIntent_ConfirmationPrompt {
    max_attempts: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_card: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<std::vec::Vec<LexIntent_ConfirmationPrompt_Message>>,
    dynamic: LexIntent_ConfirmationPromptDynamic,
}

impl LexIntent_ConfirmationPrompt {
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response_card = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntent_ConfirmationPrompt_Message>>,
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

impl terrars::ToListMappable for LexIntent_ConfirmationPrompt {
    type O = terrars::BlockAssignable<LexIntent_ConfirmationPrompt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntent_ConfirmationPrompt {
    #[doc = ""]
    pub max_attempts: terrars::PrimField<f64>,
}

impl BuildLexIntent_ConfirmationPrompt {
    pub fn build(self) -> LexIntent_ConfirmationPrompt {
        LexIntent_ConfirmationPrompt {
            max_attempts: self.max_attempts,
            response_card: core::default::Default::default(),
            message: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LexIntent_ConfirmationPromptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntent_ConfirmationPromptRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntent_ConfirmationPromptRef {
        LexIntent_ConfirmationPromptRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntent_ConfirmationPromptRef {
    #[allow(dead_code)]
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
pub struct LexIntent_DialogCodeHook {
    message_version: terrars::PrimField<std::string::String>,
    uri: terrars::PrimField<std::string::String>,
}

impl LexIntent_DialogCodeHook { }

impl terrars::ToListMappable for LexIntent_DialogCodeHook {
    type O = terrars::BlockAssignable<LexIntent_DialogCodeHook>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntent_DialogCodeHook {
    #[doc = ""]
    pub message_version: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildLexIntent_DialogCodeHook {
    pub fn build(self) -> LexIntent_DialogCodeHook {
        LexIntent_DialogCodeHook {
            message_version: self.message_version,
            uri: self.uri,
        }
    }
}

#[allow(dead_code)]
pub struct LexIntent_DialogCodeHookRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntent_DialogCodeHookRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntent_DialogCodeHookRef {
        LexIntent_DialogCodeHookRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntent_DialogCodeHookRef {
    #[allow(dead_code)]
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
struct LexIntent_FollowUpPrompt_PromptDynamic {
    message: std::option::Option<terrars::DynamicBlock<LexIntent_ConfirmationPrompt_Message>>,
}

#[derive(Serialize)]
pub struct LexIntent_FollowUpPrompt_Prompt {
    max_attempts: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_card: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<std::vec::Vec<LexIntent_ConfirmationPrompt_Message>>,
    dynamic: LexIntent_FollowUpPrompt_PromptDynamic,
}

impl LexIntent_FollowUpPrompt_Prompt {
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response_card = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntent_ConfirmationPrompt_Message>>,
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

impl terrars::ToListMappable for LexIntent_FollowUpPrompt_Prompt {
    type O = terrars::BlockAssignable<LexIntent_FollowUpPrompt_Prompt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntent_FollowUpPrompt_Prompt {
    #[doc = ""]
    pub max_attempts: terrars::PrimField<f64>,
}

impl BuildLexIntent_FollowUpPrompt_Prompt {
    pub fn build(self) -> LexIntent_FollowUpPrompt_Prompt {
        LexIntent_FollowUpPrompt_Prompt {
            max_attempts: self.max_attempts,
            response_card: core::default::Default::default(),
            message: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LexIntent_FollowUpPrompt_PromptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntent_FollowUpPrompt_PromptRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntent_FollowUpPrompt_PromptRef {
        LexIntent_FollowUpPrompt_PromptRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntent_FollowUpPrompt_PromptRef {
    #[allow(dead_code)]
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
struct LexIntent_RejectionStatementDynamic {
    message: std::option::Option<terrars::DynamicBlock<LexIntent_ConfirmationPrompt_Message>>,
}

#[derive(Serialize)]
pub struct LexIntent_RejectionStatement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_card: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<std::vec::Vec<LexIntent_ConfirmationPrompt_Message>>,
    dynamic: LexIntent_RejectionStatementDynamic,
}

impl LexIntent_RejectionStatement {
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response_card = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntent_ConfirmationPrompt_Message>>,
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

impl terrars::ToListMappable for LexIntent_RejectionStatement {
    type O = terrars::BlockAssignable<LexIntent_RejectionStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntent_RejectionStatement {}

impl BuildLexIntent_RejectionStatement {
    pub fn build(self) -> LexIntent_RejectionStatement {
        LexIntent_RejectionStatement {
            response_card: core::default::Default::default(),
            message: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LexIntent_RejectionStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntent_RejectionStatementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntent_RejectionStatementRef {
        LexIntent_RejectionStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntent_RejectionStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `response_card` after provisioning.\n"]
    pub fn response_card(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_card", self.base))
    }
}

#[derive(Serialize, Default)]
struct LexIntent_FollowUpPromptDynamic {
    prompt: std::option::Option<terrars::DynamicBlock<LexIntent_FollowUpPrompt_Prompt>>,
    rejection_statement: std::option::Option<terrars::DynamicBlock<LexIntent_RejectionStatement>>,
}

#[derive(Serialize)]
pub struct LexIntent_FollowUpPrompt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt: std::option::Option<std::vec::Vec<LexIntent_FollowUpPrompt_Prompt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rejection_statement: std::option::Option<std::vec::Vec<LexIntent_RejectionStatement>>,
    dynamic: LexIntent_FollowUpPromptDynamic,
}

impl LexIntent_FollowUpPrompt {
    #[doc = "Set the field `prompt`.\n"]
    pub fn set_prompt(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntent_FollowUpPrompt_Prompt>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntent_RejectionStatement>>,
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

impl terrars::ToListMappable for LexIntent_FollowUpPrompt {
    type O = terrars::BlockAssignable<LexIntent_FollowUpPrompt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntent_FollowUpPrompt {}

impl BuildLexIntent_FollowUpPrompt {
    pub fn build(self) -> LexIntent_FollowUpPrompt {
        LexIntent_FollowUpPrompt {
            prompt: core::default::Default::default(),
            rejection_statement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LexIntent_FollowUpPromptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntent_FollowUpPromptRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntent_FollowUpPromptRef {
        LexIntent_FollowUpPromptRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntent_FollowUpPromptRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prompt` after provisioning.\n"]
    pub fn prompt(&self) -> terrars::ListRef<LexIntent_FollowUpPrompt_PromptRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prompt", self.base))
    }

    #[doc = "Get a reference to the value of field `rejection_statement` after provisioning.\n"]
    pub fn rejection_statement(&self) -> terrars::ListRef<LexIntent_RejectionStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rejection_statement", self.base))
    }
}

#[derive(Serialize)]
pub struct LexIntent_FulfillmentActivity_CodeHook {
    message_version: terrars::PrimField<std::string::String>,
    uri: terrars::PrimField<std::string::String>,
}

impl LexIntent_FulfillmentActivity_CodeHook { }

impl terrars::ToListMappable for LexIntent_FulfillmentActivity_CodeHook {
    type O = terrars::BlockAssignable<LexIntent_FulfillmentActivity_CodeHook>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntent_FulfillmentActivity_CodeHook {
    #[doc = ""]
    pub message_version: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildLexIntent_FulfillmentActivity_CodeHook {
    pub fn build(self) -> LexIntent_FulfillmentActivity_CodeHook {
        LexIntent_FulfillmentActivity_CodeHook {
            message_version: self.message_version,
            uri: self.uri,
        }
    }
}

#[allow(dead_code)]
pub struct LexIntent_FulfillmentActivity_CodeHookRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntent_FulfillmentActivity_CodeHookRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntent_FulfillmentActivity_CodeHookRef {
        LexIntent_FulfillmentActivity_CodeHookRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntent_FulfillmentActivity_CodeHookRef {
    #[allow(dead_code)]
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
struct LexIntent_FulfillmentActivityDynamic {
    code_hook: std::option::Option<terrars::DynamicBlock<LexIntent_FulfillmentActivity_CodeHook>>,
}

#[derive(Serialize)]
pub struct LexIntent_FulfillmentActivity {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_hook: std::option::Option<std::vec::Vec<LexIntent_FulfillmentActivity_CodeHook>>,
    dynamic: LexIntent_FulfillmentActivityDynamic,
}

impl LexIntent_FulfillmentActivity {
    #[doc = "Set the field `code_hook`.\n"]
    pub fn set_code_hook(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntent_FulfillmentActivity_CodeHook>>,
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

impl terrars::ToListMappable for LexIntent_FulfillmentActivity {
    type O = terrars::BlockAssignable<LexIntent_FulfillmentActivity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntent_FulfillmentActivity {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildLexIntent_FulfillmentActivity {
    pub fn build(self) -> LexIntent_FulfillmentActivity {
        LexIntent_FulfillmentActivity {
            type_: self.type_,
            code_hook: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LexIntent_FulfillmentActivityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntent_FulfillmentActivityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntent_FulfillmentActivityRef {
        LexIntent_FulfillmentActivityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntent_FulfillmentActivityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `code_hook` after provisioning.\n"]
    pub fn code_hook(&self) -> terrars::ListRef<LexIntent_FulfillmentActivity_CodeHookRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_hook", self.base))
    }
}

#[derive(Serialize, Default)]
struct LexIntent_Slot_ValueElicitationPromptDynamic {
    message: std::option::Option<terrars::DynamicBlock<LexIntent_ConfirmationPrompt_Message>>,
}

#[derive(Serialize)]
pub struct LexIntent_Slot_ValueElicitationPrompt {
    max_attempts: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_card: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<std::vec::Vec<LexIntent_ConfirmationPrompt_Message>>,
    dynamic: LexIntent_Slot_ValueElicitationPromptDynamic,
}

impl LexIntent_Slot_ValueElicitationPrompt {
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response_card = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntent_ConfirmationPrompt_Message>>,
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

impl terrars::ToListMappable for LexIntent_Slot_ValueElicitationPrompt {
    type O = terrars::BlockAssignable<LexIntent_Slot_ValueElicitationPrompt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntent_Slot_ValueElicitationPrompt {
    #[doc = ""]
    pub max_attempts: terrars::PrimField<f64>,
}

impl BuildLexIntent_Slot_ValueElicitationPrompt {
    pub fn build(self) -> LexIntent_Slot_ValueElicitationPrompt {
        LexIntent_Slot_ValueElicitationPrompt {
            max_attempts: self.max_attempts,
            response_card: core::default::Default::default(),
            message: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LexIntent_Slot_ValueElicitationPromptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntent_Slot_ValueElicitationPromptRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntent_Slot_ValueElicitationPromptRef {
        LexIntent_Slot_ValueElicitationPromptRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntent_Slot_ValueElicitationPromptRef {
    #[allow(dead_code)]
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
struct LexIntent_SlotDynamic {
    value_elicitation_prompt: std::option::Option<terrars::DynamicBlock<LexIntent_Slot_ValueElicitationPrompt>>,
}

#[derive(Serialize)]
pub struct LexIntent_Slot {
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
    value_elicitation_prompt: std::option::Option<std::vec::Vec<LexIntent_Slot_ValueElicitationPrompt>>,
    dynamic: LexIntent_SlotDynamic,
}

impl LexIntent_Slot {
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
        v: impl std::convert::Into<terrars::BlockAssignable<LexIntent_Slot_ValueElicitationPrompt>>,
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

impl terrars::ToListMappable for LexIntent_Slot {
    type O = terrars::BlockAssignable<LexIntent_Slot>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntent_Slot {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub slot_constraint: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub slot_type: terrars::PrimField<std::string::String>,
}

impl BuildLexIntent_Slot {
    pub fn build(self) -> LexIntent_Slot {
        LexIntent_Slot {
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

#[allow(dead_code)]
pub struct LexIntent_SlotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntent_SlotRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntent_SlotRef {
        LexIntent_SlotRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntent_SlotRef {
    #[allow(dead_code)]
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
    pub fn value_elicitation_prompt(&self) -> terrars::ListRef<LexIntent_Slot_ValueElicitationPromptRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value_elicitation_prompt", self.base))
    }
}

#[derive(Serialize)]
pub struct LexIntent_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LexIntent_Timeouts {
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

impl terrars::ToListMappable for LexIntent_Timeouts {
    type O = terrars::BlockAssignable<LexIntent_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexIntent_Timeouts {}

impl BuildLexIntent_Timeouts {
    pub fn build(self) -> LexIntent_Timeouts {
        LexIntent_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LexIntent_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexIntent_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexIntent_TimeoutsRef {
        LexIntent_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexIntent_TimeoutsRef {
    #[allow(dead_code)]
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
