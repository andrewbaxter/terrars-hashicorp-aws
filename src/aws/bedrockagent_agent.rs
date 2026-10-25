#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentAgentData_ {
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
    agent_collaboration: std::option::Option<terrars::PrimField<std::string::String>>,
    agent_name: terrars::PrimField<std::string::String>,
    agent_resource_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_encryption_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    foundation_model: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guardrail_configuration: std::option::Option<terrars::ListField<BedrockagentAgent_GuardrailConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_session_ttl_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instruction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_configuration: std::option::Option<terrars::ListField<BedrockagentAgent_MemoryConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prepare_agent: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt_override_configuration: std::option::Option<terrars::ListField<BedrockagentAgent_PromptOverrideConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_resource_in_use_check: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockagentAgent_Timeouts>,
}

struct BedrockagentAgent_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockagentAgentData_>,
}

#[derive(Clone)]
pub struct BedrockagentAgent(std::rc::Rc<BedrockagentAgent_>);

impl BedrockagentAgent {
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

    #[doc = "Set the field `agent_collaboration`.\n"]
    pub fn set_agent_collaboration(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().agent_collaboration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer_encryption_key_arn`.\n"]
    pub fn set_customer_encryption_key_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().customer_encryption_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guardrail_configuration`.\n"]
    pub fn set_guardrail_configuration(
        self,
        v: impl std::convert::Into<terrars::ListField<BedrockagentAgent_GuardrailConfiguration>>,
    ) -> Self {
        self.0.data.borrow_mut().guardrail_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `idle_session_ttl_in_seconds`.\n"]
    pub fn set_idle_session_ttl_in_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().idle_session_ttl_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instruction`.\n"]
    pub fn set_instruction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().instruction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_configuration`.\n"]
    pub fn set_memory_configuration(
        self,
        v: impl std::convert::Into<terrars::ListField<BedrockagentAgent_MemoryConfiguration>>,
    ) -> Self {
        self.0.data.borrow_mut().memory_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prepare_agent`.\n"]
    pub fn set_prepare_agent(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().prepare_agent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prompt_override_configuration`.\n"]
    pub fn set_prompt_override_configuration(
        self,
        v: impl std::convert::Into<terrars::ListField<BedrockagentAgent_PromptOverrideConfiguration>>,
    ) -> Self {
        self.0.data.borrow_mut().prompt_override_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `skip_resource_in_use_check`.\n"]
    pub fn set_skip_resource_in_use_check(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_resource_in_use_check = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockagentAgent_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `agent_arn` after provisioning.\n"]
    pub fn agent_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_collaboration` after provisioning.\n"]
    pub fn agent_collaboration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_collaboration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_name` after provisioning.\n"]
    pub fn agent_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_resource_role_arn` after provisioning.\n"]
    pub fn agent_resource_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_resource_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_encryption_key_arn` after provisioning.\n"]
    pub fn customer_encryption_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_encryption_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `foundation_model` after provisioning.\n"]
    pub fn foundation_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.foundation_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardrail_configuration` after provisioning.\n"]
    pub fn guardrail_configuration(&self) -> terrars::ListRef<BedrockagentAgent_GuardrailConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guardrail_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_session_ttl_in_seconds` after provisioning.\n"]
    pub fn idle_session_ttl_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_session_ttl_in_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instruction` after provisioning.\n"]
    pub fn instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instruction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_configuration` after provisioning.\n"]
    pub fn memory_configuration(&self) -> terrars::ListRef<BedrockagentAgent_MemoryConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prepare_agent` after provisioning.\n"]
    pub fn prepare_agent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prepare_agent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prepared_at` after provisioning.\n"]
    pub fn prepared_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prepared_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prompt_override_configuration` after provisioning.\n"]
    pub fn prompt_override_configuration(&self) -> terrars::ListRef<BedrockagentAgent_PromptOverrideConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prompt_override_configuration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_resource_in_use_check` after provisioning.\n"]
    pub fn skip_resource_in_use_check(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_resource_in_use_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentAgent_TimeoutsRef {
        BedrockagentAgent_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BedrockagentAgent {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockagentAgent { }

impl terrars::ToListMappable for BedrockagentAgent {
    type O = terrars::ListRef<BedrockagentAgentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockagentAgent_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrockagent_agent".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentAgent {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub agent_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub agent_resource_role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub foundation_model: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentAgent {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockagentAgent {
        let out = BedrockagentAgent(std::rc::Rc::new(BedrockagentAgent_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockagentAgentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                agent_collaboration: core::default::Default::default(),
                agent_name: self.agent_name,
                agent_resource_role_arn: self.agent_resource_role_arn,
                customer_encryption_key_arn: core::default::Default::default(),
                description: core::default::Default::default(),
                foundation_model: self.foundation_model,
                guardrail_configuration: core::default::Default::default(),
                idle_session_ttl_in_seconds: core::default::Default::default(),
                instruction: core::default::Default::default(),
                memory_configuration: core::default::Default::default(),
                prepare_agent: core::default::Default::default(),
                prompt_override_configuration: core::default::Default::default(),
                region: core::default::Default::default(),
                skip_resource_in_use_check: core::default::Default::default(),
                tags: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentAgentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentAgentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockagentAgentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_arn` after provisioning.\n"]
    pub fn agent_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_collaboration` after provisioning.\n"]
    pub fn agent_collaboration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_collaboration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_name` after provisioning.\n"]
    pub fn agent_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_resource_role_arn` after provisioning.\n"]
    pub fn agent_resource_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_resource_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_encryption_key_arn` after provisioning.\n"]
    pub fn customer_encryption_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_encryption_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `foundation_model` after provisioning.\n"]
    pub fn foundation_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.foundation_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardrail_configuration` after provisioning.\n"]
    pub fn guardrail_configuration(&self) -> terrars::ListRef<BedrockagentAgent_GuardrailConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guardrail_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_session_ttl_in_seconds` after provisioning.\n"]
    pub fn idle_session_ttl_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_session_ttl_in_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instruction` after provisioning.\n"]
    pub fn instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instruction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_configuration` after provisioning.\n"]
    pub fn memory_configuration(&self) -> terrars::ListRef<BedrockagentAgent_MemoryConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prepare_agent` after provisioning.\n"]
    pub fn prepare_agent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prepare_agent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prepared_at` after provisioning.\n"]
    pub fn prepared_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prepared_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prompt_override_configuration` after provisioning.\n"]
    pub fn prompt_override_configuration(&self) -> terrars::ListRef<BedrockagentAgent_PromptOverrideConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prompt_override_configuration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_resource_in_use_check` after provisioning.\n"]
    pub fn skip_resource_in_use_check(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_resource_in_use_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentAgent_TimeoutsRef {
        BedrockagentAgent_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BedrockagentAgent_GuardrailConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guardrail_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guardrail_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentAgent_GuardrailConfiguration {
    #[doc = "Set the field `guardrail_identifier`.\n"]
    pub fn set_guardrail_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.guardrail_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guardrail_version`.\n"]
    pub fn set_guardrail_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.guardrail_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentAgent_GuardrailConfiguration {
    type O = terrars::BlockAssignable<BedrockagentAgent_GuardrailConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgent_GuardrailConfiguration {}

impl BuildBedrockagentAgent_GuardrailConfiguration {
    pub fn build(self) -> BedrockagentAgent_GuardrailConfiguration {
        BedrockagentAgent_GuardrailConfiguration {
            guardrail_identifier: core::default::Default::default(),
            guardrail_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentAgent_GuardrailConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentAgent_GuardrailConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentAgent_GuardrailConfigurationRef {
        BedrockagentAgent_GuardrailConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgent_GuardrailConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `guardrail_identifier` after provisioning.\n"]
    pub fn guardrail_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guardrail_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `guardrail_version` after provisioning.\n"]
    pub fn guardrail_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guardrail_version", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentAgent_MemoryConfiguration_SessionSummaryConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_recent_sessions: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentAgent_MemoryConfiguration_SessionSummaryConfiguration {
    #[doc = "Set the field `max_recent_sessions`.\n"]
    pub fn set_max_recent_sessions(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_recent_sessions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentAgent_MemoryConfiguration_SessionSummaryConfiguration {
    type O = terrars::BlockAssignable<BedrockagentAgent_MemoryConfiguration_SessionSummaryConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgent_MemoryConfiguration_SessionSummaryConfiguration {}

impl BuildBedrockagentAgent_MemoryConfiguration_SessionSummaryConfiguration {
    pub fn build(self) -> BedrockagentAgent_MemoryConfiguration_SessionSummaryConfiguration {
        BedrockagentAgent_MemoryConfiguration_SessionSummaryConfiguration {
            max_recent_sessions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentAgent_MemoryConfiguration_SessionSummaryConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentAgent_MemoryConfiguration_SessionSummaryConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentAgent_MemoryConfiguration_SessionSummaryConfigurationRef {
        BedrockagentAgent_MemoryConfiguration_SessionSummaryConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgent_MemoryConfiguration_SessionSummaryConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_recent_sessions` after provisioning.\n"]
    pub fn max_recent_sessions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_recent_sessions", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentAgent_MemoryConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled_memory_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_summary_configuration: std
    ::option
    ::Option<terrars::ListField<BedrockagentAgent_MemoryConfiguration_SessionSummaryConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_days: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentAgent_MemoryConfiguration {
    #[doc = "Set the field `enabled_memory_types`.\n"]
    pub fn set_enabled_memory_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.enabled_memory_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `session_summary_configuration`.\n"]
    pub fn set_session_summary_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<BedrockagentAgent_MemoryConfiguration_SessionSummaryConfiguration>>,
    ) -> Self {
        self.session_summary_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_days`.\n"]
    pub fn set_storage_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_days = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentAgent_MemoryConfiguration {
    type O = terrars::BlockAssignable<BedrockagentAgent_MemoryConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgent_MemoryConfiguration {}

impl BuildBedrockagentAgent_MemoryConfiguration {
    pub fn build(self) -> BedrockagentAgent_MemoryConfiguration {
        BedrockagentAgent_MemoryConfiguration {
            enabled_memory_types: core::default::Default::default(),
            session_summary_configuration: core::default::Default::default(),
            storage_days: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentAgent_MemoryConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentAgent_MemoryConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentAgent_MemoryConfigurationRef {
        BedrockagentAgent_MemoryConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgent_MemoryConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled_memory_types` after provisioning.\n"]
    pub fn enabled_memory_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enabled_memory_types", self.base))
    }

    #[doc = "Get a reference to the value of field `session_summary_configuration` after provisioning.\n"]
    pub fn session_summary_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentAgent_MemoryConfiguration_SessionSummaryConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.session_summary_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_days` after provisioning.\n"]
    pub fn storage_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_days", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations_InferenceConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stop_sequences: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temperature: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_k: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_p: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations_InferenceConfiguration {
    #[doc = "Set the field `max_length`.\n"]
    pub fn set_max_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stop_sequences`.\n"]
    pub fn set_stop_sequences(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.stop_sequences = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `temperature`.\n"]
    pub fn set_temperature(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.temperature = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `top_k`.\n"]
    pub fn set_top_k(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.top_k = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `top_p`.\n"]
    pub fn set_top_p(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.top_p = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations_InferenceConfiguration {
    type O =
        terrars
        ::BlockAssignable<BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations_InferenceConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations_InferenceConfiguration {}

impl BuildBedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations_InferenceConfiguration {
    pub fn build(self) -> BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations_InferenceConfiguration {
        BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations_InferenceConfiguration {
            max_length: core::default::Default::default(),
            stop_sequences: core::default::Default::default(),
            temperature: core::default::Default::default(),
            top_k: core::default::Default::default(),
            top_p: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations_InferenceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations_InferenceConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations_InferenceConfigurationRef {
        BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations_InferenceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations_InferenceConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_length` after provisioning.\n"]
    pub fn max_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_length", self.base))
    }

    #[doc = "Get a reference to the value of field `stop_sequences` after provisioning.\n"]
    pub fn stop_sequences(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stop_sequences", self.base))
    }

    #[doc = "Get a reference to the value of field `temperature` after provisioning.\n"]
    pub fn temperature(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }

    #[doc = "Get a reference to the value of field `top_k` after provisioning.\n"]
    pub fn top_k(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.top_k", self.base))
    }

    #[doc = "Get a reference to the value of field `top_p` after provisioning.\n"]
    pub fn top_p(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.top_p", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_prompt_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_configuration: std
    ::option
    ::Option<
        terrars::ListField<BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations_InferenceConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parser_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt_creation_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations {
    #[doc = "Set the field `base_prompt_template`.\n"]
    pub fn set_base_prompt_template(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.base_prompt_template = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inference_configuration`.\n"]
    pub fn set_inference_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations_InferenceConfiguration,
                        >,
                    >,
    ) -> Self {
        self.inference_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parser_mode`.\n"]
    pub fn set_parser_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parser_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prompt_creation_mode`.\n"]
    pub fn set_prompt_creation_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.prompt_creation_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prompt_state`.\n"]
    pub fn set_prompt_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prompt_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prompt_type`.\n"]
    pub fn set_prompt_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prompt_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations {
    type O = terrars::BlockAssignable<BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations {}

impl BuildBedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations {
    pub fn build(self) -> BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations {
        BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations {
            base_prompt_template: core::default::Default::default(),
            inference_configuration: core::default::Default::default(),
            parser_mode: core::default::Default::default(),
            prompt_creation_mode: core::default::Default::default(),
            prompt_state: core::default::Default::default(),
            prompt_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurationsRef {
        BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base_prompt_template` after provisioning.\n"]
    pub fn base_prompt_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_prompt_template", self.base))
    }

    #[doc = "Get a reference to the value of field `inference_configuration` after provisioning.\n"]
    pub fn inference_configuration(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations_InferenceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inference_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `parser_mode` after provisioning.\n"]
    pub fn parser_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parser_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `prompt_creation_mode` after provisioning.\n"]
    pub fn prompt_creation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prompt_creation_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `prompt_state` after provisioning.\n"]
    pub fn prompt_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prompt_state", self.base))
    }

    #[doc = "Get a reference to the value of field `prompt_type` after provisioning.\n"]
    pub fn prompt_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prompt_type", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentAgent_PromptOverrideConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_lambda: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt_configurations: std
    ::option
    ::Option<terrars::SetField<BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations>>,
}

impl BedrockagentAgent_PromptOverrideConfiguration {
    #[doc = "Set the field `override_lambda`.\n"]
    pub fn set_override_lambda(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.override_lambda = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prompt_configurations`.\n"]
    pub fn set_prompt_configurations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::SetField<BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurations>>,
    ) -> Self {
        self.prompt_configurations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentAgent_PromptOverrideConfiguration {
    type O = terrars::BlockAssignable<BedrockagentAgent_PromptOverrideConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgent_PromptOverrideConfiguration {}

impl BuildBedrockagentAgent_PromptOverrideConfiguration {
    pub fn build(self) -> BedrockagentAgent_PromptOverrideConfiguration {
        BedrockagentAgent_PromptOverrideConfiguration {
            override_lambda: core::default::Default::default(),
            prompt_configurations: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentAgent_PromptOverrideConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentAgent_PromptOverrideConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentAgent_PromptOverrideConfigurationRef {
        BedrockagentAgent_PromptOverrideConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgent_PromptOverrideConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `override_lambda` after provisioning.\n"]
    pub fn override_lambda(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override_lambda", self.base))
    }

    #[doc = "Get a reference to the value of field `prompt_configurations` after provisioning.\n"]
    pub fn prompt_configurations(
        &self,
    ) -> terrars::SetRef<BedrockagentAgent_PromptOverrideConfiguration_PromptConfigurationsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.prompt_configurations", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentAgent_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentAgent_Timeouts {
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

impl terrars::ToListMappable for BedrockagentAgent_Timeouts {
    type O = terrars::BlockAssignable<BedrockagentAgent_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgent_Timeouts {}

impl BuildBedrockagentAgent_Timeouts {
    pub fn build(self) -> BedrockagentAgent_Timeouts {
        BedrockagentAgent_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentAgent_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentAgent_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentAgent_TimeoutsRef {
        BedrockagentAgent_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgent_TimeoutsRef {
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
