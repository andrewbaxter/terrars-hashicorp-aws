#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LexBotData_ {
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
    child_directed: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_version: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    detect_sentiment: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_model_improvements: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_session_ttl_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locale: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nlu_intent_confidence_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    process_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    voice_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    abort_statement: std::option::Option<std::vec::Vec<LexBot_AbortStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clarification_prompt: std::option::Option<std::vec::Vec<LexBot_ClarificationPrompt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    intent: std::option::Option<std::vec::Vec<LexBot_Intent>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LexBot_Timeouts>,
    dynamic: LexBotDynamic,
}

struct LexBot_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LexBotData_>,
}

#[derive(Clone)]
pub struct LexBot(std::rc::Rc<LexBot_>);

impl LexBot {
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

    #[doc = "Set the field `detect_sentiment`.\n"]
    pub fn set_detect_sentiment(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().detect_sentiment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_model_improvements`.\n"]
    pub fn set_enable_model_improvements(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_model_improvements = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `idle_session_ttl_in_seconds`.\n"]
    pub fn set_idle_session_ttl_in_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().idle_session_ttl_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `locale`.\n"]
    pub fn set_locale(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().locale = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nlu_intent_confidence_threshold`.\n"]
    pub fn set_nlu_intent_confidence_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().nlu_intent_confidence_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `process_behavior`.\n"]
    pub fn set_process_behavior(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().process_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `voice_id`.\n"]
    pub fn set_voice_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().voice_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `abort_statement`.\n"]
    pub fn set_abort_statement(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexBot_AbortStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().abort_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.abort_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `clarification_prompt`.\n"]
    pub fn set_clarification_prompt(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexBot_ClarificationPrompt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().clarification_prompt = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.clarification_prompt = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `intent`.\n"]
    pub fn set_intent(self, v: impl std::convert::Into<terrars::BlockAssignable<LexBot_Intent>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().intent = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.intent = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<LexBot_Timeouts>) -> Self {
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

    #[doc = "Get a reference to the value of field `child_directed` after provisioning.\n"]
    pub fn child_directed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.child_directed", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `detect_sentiment` after provisioning.\n"]
    pub fn detect_sentiment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detect_sentiment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_model_improvements` after provisioning.\n"]
    pub fn enable_model_improvements(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_model_improvements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\n"]
    pub fn failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_session_ttl_in_seconds` after provisioning.\n"]
    pub fn idle_session_ttl_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_session_ttl_in_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locale` after provisioning.\n"]
    pub fn locale(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locale", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nlu_intent_confidence_threshold` after provisioning.\n"]
    pub fn nlu_intent_confidence_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.nlu_intent_confidence_threshold", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `process_behavior` after provisioning.\n"]
    pub fn process_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.process_behavior", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `voice_id` after provisioning.\n"]
    pub fn voice_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.voice_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `abort_statement` after provisioning.\n"]
    pub fn abort_statement(&self) -> terrars::ListRef<LexBot_AbortStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.abort_statement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `clarification_prompt` after provisioning.\n"]
    pub fn clarification_prompt(&self) -> terrars::ListRef<LexBot_ClarificationPromptRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clarification_prompt", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LexBot_TimeoutsRef {
        LexBot_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for LexBot {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LexBot { }

impl terrars::ToListMappable for LexBot {
    type O = terrars::ListRef<LexBotRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LexBot_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lex_bot".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLexBot {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub child_directed: terrars::PrimField<bool>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildLexBot {
    pub fn build(self, stack: &mut terrars::Stack) -> LexBot {
        let out = LexBot(std::rc::Rc::new(LexBot_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LexBotData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                child_directed: self.child_directed,
                create_version: core::default::Default::default(),
                description: core::default::Default::default(),
                detect_sentiment: core::default::Default::default(),
                enable_model_improvements: core::default::Default::default(),
                id: core::default::Default::default(),
                idle_session_ttl_in_seconds: core::default::Default::default(),
                locale: core::default::Default::default(),
                name: self.name,
                nlu_intent_confidence_threshold: core::default::Default::default(),
                process_behavior: core::default::Default::default(),
                region: core::default::Default::default(),
                voice_id: core::default::Default::default(),
                abort_statement: core::default::Default::default(),
                clarification_prompt: core::default::Default::default(),
                intent: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LexBotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexBotRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LexBotRef {
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

    #[doc = "Get a reference to the value of field `child_directed` after provisioning.\n"]
    pub fn child_directed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.child_directed", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `detect_sentiment` after provisioning.\n"]
    pub fn detect_sentiment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detect_sentiment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_model_improvements` after provisioning.\n"]
    pub fn enable_model_improvements(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_model_improvements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\n"]
    pub fn failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_session_ttl_in_seconds` after provisioning.\n"]
    pub fn idle_session_ttl_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_session_ttl_in_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locale` after provisioning.\n"]
    pub fn locale(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locale", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nlu_intent_confidence_threshold` after provisioning.\n"]
    pub fn nlu_intent_confidence_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.nlu_intent_confidence_threshold", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `process_behavior` after provisioning.\n"]
    pub fn process_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.process_behavior", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `voice_id` after provisioning.\n"]
    pub fn voice_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.voice_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `abort_statement` after provisioning.\n"]
    pub fn abort_statement(&self) -> terrars::ListRef<LexBot_AbortStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.abort_statement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `clarification_prompt` after provisioning.\n"]
    pub fn clarification_prompt(&self) -> terrars::ListRef<LexBot_ClarificationPromptRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clarification_prompt", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LexBot_TimeoutsRef {
        LexBot_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct LexBotDynamic {
    abort_statement: std::option::Option<terrars::DynamicBlock<LexBot_AbortStatement>>,
    clarification_prompt: std::option::Option<terrars::DynamicBlock<LexBot_ClarificationPrompt>>,
    intent: std::option::Option<terrars::DynamicBlock<LexBot_Intent>>,
}

#[derive(Serialize)]
pub struct LexBot_AbortStatement_Message {
    content: terrars::PrimField<std::string::String>,
    content_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_number: std::option::Option<terrars::PrimField<f64>>,
}

impl LexBot_AbortStatement_Message {
    #[doc = "Set the field `group_number`.\n"]
    pub fn set_group_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.group_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LexBot_AbortStatement_Message {
    type O = terrars::BlockAssignable<LexBot_AbortStatement_Message>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexBot_AbortStatement_Message {
    #[doc = ""]
    pub content: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub content_type: terrars::PrimField<std::string::String>,
}

impl BuildLexBot_AbortStatement_Message {
    pub fn build(self) -> LexBot_AbortStatement_Message {
        LexBot_AbortStatement_Message {
            content: self.content,
            content_type: self.content_type,
            group_number: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LexBot_AbortStatement_MessageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexBot_AbortStatement_MessageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexBot_AbortStatement_MessageRef {
        LexBot_AbortStatement_MessageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexBot_AbortStatement_MessageRef {
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
struct LexBot_AbortStatementDynamic {
    message: std::option::Option<terrars::DynamicBlock<LexBot_AbortStatement_Message>>,
}

#[derive(Serialize)]
pub struct LexBot_AbortStatement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_card: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<std::vec::Vec<LexBot_AbortStatement_Message>>,
    dynamic: LexBot_AbortStatementDynamic,
}

impl LexBot_AbortStatement {
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response_card = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexBot_AbortStatement_Message>>,
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

impl terrars::ToListMappable for LexBot_AbortStatement {
    type O = terrars::BlockAssignable<LexBot_AbortStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexBot_AbortStatement {}

impl BuildLexBot_AbortStatement {
    pub fn build(self) -> LexBot_AbortStatement {
        LexBot_AbortStatement {
            response_card: core::default::Default::default(),
            message: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LexBot_AbortStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexBot_AbortStatementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexBot_AbortStatementRef {
        LexBot_AbortStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexBot_AbortStatementRef {
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
struct LexBot_ClarificationPromptDynamic {
    message: std::option::Option<terrars::DynamicBlock<LexBot_AbortStatement_Message>>,
}

#[derive(Serialize)]
pub struct LexBot_ClarificationPrompt {
    max_attempts: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_card: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<std::vec::Vec<LexBot_AbortStatement_Message>>,
    dynamic: LexBot_ClarificationPromptDynamic,
}

impl LexBot_ClarificationPrompt {
    #[doc = "Set the field `response_card`.\n"]
    pub fn set_response_card(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response_card = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexBot_AbortStatement_Message>>,
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

impl terrars::ToListMappable for LexBot_ClarificationPrompt {
    type O = terrars::BlockAssignable<LexBot_ClarificationPrompt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexBot_ClarificationPrompt {
    #[doc = ""]
    pub max_attempts: terrars::PrimField<f64>,
}

impl BuildLexBot_ClarificationPrompt {
    pub fn build(self) -> LexBot_ClarificationPrompt {
        LexBot_ClarificationPrompt {
            max_attempts: self.max_attempts,
            response_card: core::default::Default::default(),
            message: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LexBot_ClarificationPromptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexBot_ClarificationPromptRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexBot_ClarificationPromptRef {
        LexBot_ClarificationPromptRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexBot_ClarificationPromptRef {
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
pub struct LexBot_Intent {
    intent_name: terrars::PrimField<std::string::String>,
    intent_version: terrars::PrimField<std::string::String>,
}

impl LexBot_Intent { }

impl terrars::ToListMappable for LexBot_Intent {
    type O = terrars::BlockAssignable<LexBot_Intent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexBot_Intent {
    #[doc = ""]
    pub intent_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub intent_version: terrars::PrimField<std::string::String>,
}

impl BuildLexBot_Intent {
    pub fn build(self) -> LexBot_Intent {
        LexBot_Intent {
            intent_name: self.intent_name,
            intent_version: self.intent_version,
        }
    }
}

#[allow(dead_code)]
pub struct LexBot_IntentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexBot_IntentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexBot_IntentRef {
        LexBot_IntentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexBot_IntentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `intent_name` after provisioning.\n"]
    pub fn intent_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.intent_name", self.base))
    }

    #[doc = "Get a reference to the value of field `intent_version` after provisioning.\n"]
    pub fn intent_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.intent_version", self.base))
    }
}

#[derive(Serialize)]
pub struct LexBot_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LexBot_Timeouts {
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

impl terrars::ToListMappable for LexBot_Timeouts {
    type O = terrars::BlockAssignable<LexBot_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexBot_Timeouts {}

impl BuildLexBot_Timeouts {
    pub fn build(self) -> LexBot_Timeouts {
        LexBot_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LexBot_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexBot_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexBot_TimeoutsRef {
        LexBot_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexBot_TimeoutsRef {
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
