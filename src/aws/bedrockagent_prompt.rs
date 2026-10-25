#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentPromptData_ {
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
    customer_encryption_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_variant: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    variant: std::option::Option<std::vec::Vec<BedrockagentPrompt_Variant>>,
    dynamic: BedrockagentPromptDynamic,
}

struct BedrockagentPrompt_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockagentPromptData_>,
}

#[derive(Clone)]
pub struct BedrockagentPrompt(std::rc::Rc<BedrockagentPrompt_>);

impl BedrockagentPrompt {
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

    #[doc = "Set the field `customer_encryption_key_arn`.\n"]
    pub fn set_customer_encryption_key_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().customer_encryption_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_variant`.\n"]
    pub fn set_default_variant(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().default_variant = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `variant`.\n"]
    pub fn set_variant(self, v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentPrompt_Variant>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().variant = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.variant = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_encryption_key_arn` after provisioning.\n"]
    pub fn customer_encryption_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_encryption_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_variant` after provisioning.\n"]
    pub fn default_variant(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_variant", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `variant` after provisioning.\n"]
    pub fn variant(&self) -> terrars::ListRef<BedrockagentPrompt_VariantRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.variant", self.extract_ref()))
    }
}

impl terrars::Referable for BedrockagentPrompt {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockagentPrompt { }

impl terrars::ToListMappable for BedrockagentPrompt {
    type O = terrars::ListRef<BedrockagentPromptRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockagentPrompt_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrockagent_prompt".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentPrompt {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentPrompt {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockagentPrompt {
        let out = BedrockagentPrompt(std::rc::Rc::new(BedrockagentPrompt_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockagentPromptData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                customer_encryption_key_arn: core::default::Default::default(),
                default_variant: core::default::Default::default(),
                description: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                variant: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentPromptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPromptRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockagentPromptRef {
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

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_encryption_key_arn` after provisioning.\n"]
    pub fn customer_encryption_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_encryption_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_variant` after provisioning.\n"]
    pub fn default_variant(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_variant", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `variant` after provisioning.\n"]
    pub fn variant(&self) -> terrars::ListRef<BedrockagentPrompt_VariantRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.variant", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPromptDynamic {
    variant: std::option::Option<terrars::DynamicBlock<BedrockagentPrompt_Variant>>,
}

#[derive(Serialize)]
pub struct BedrockagentPrompt_Variant_GenAiResource_Agent {
    agent_identifier: terrars::PrimField<std::string::String>,
}

impl BedrockagentPrompt_Variant_GenAiResource_Agent { }

impl terrars::ToListMappable for BedrockagentPrompt_Variant_GenAiResource_Agent {
    type O = terrars::BlockAssignable<BedrockagentPrompt_Variant_GenAiResource_Agent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant_GenAiResource_Agent {
    #[doc = ""]
    pub agent_identifier: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentPrompt_Variant_GenAiResource_Agent {
    pub fn build(self) -> BedrockagentPrompt_Variant_GenAiResource_Agent {
        BedrockagentPrompt_Variant_GenAiResource_Agent { agent_identifier: self.agent_identifier }
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_Variant_GenAiResource_AgentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPrompt_Variant_GenAiResource_AgentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentPrompt_Variant_GenAiResource_AgentRef {
        BedrockagentPrompt_Variant_GenAiResource_AgentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_Variant_GenAiResource_AgentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_identifier` after provisioning.\n"]
    pub fn agent_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_identifier", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPrompt_Variant_GenAiResourceDynamic {
    agent: std::option::Option<terrars::DynamicBlock<BedrockagentPrompt_Variant_GenAiResource_Agent>>,
}

#[derive(Serialize)]
pub struct BedrockagentPrompt_Variant_GenAiResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent: std::option::Option<std::vec::Vec<BedrockagentPrompt_Variant_GenAiResource_Agent>>,
    dynamic: BedrockagentPrompt_Variant_GenAiResourceDynamic,
}

impl BedrockagentPrompt_Variant_GenAiResource {
    #[doc = "Set the field `agent`.\n"]
    pub fn set_agent(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentPrompt_Variant_GenAiResource_Agent>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.agent = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.agent = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentPrompt_Variant_GenAiResource {
    type O = terrars::BlockAssignable<BedrockagentPrompt_Variant_GenAiResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant_GenAiResource {}

impl BuildBedrockagentPrompt_Variant_GenAiResource {
    pub fn build(self) -> BedrockagentPrompt_Variant_GenAiResource {
        BedrockagentPrompt_Variant_GenAiResource {
            agent: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_Variant_GenAiResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPrompt_Variant_GenAiResourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentPrompt_Variant_GenAiResourceRef {
        BedrockagentPrompt_Variant_GenAiResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_Variant_GenAiResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent` after provisioning.\n"]
    pub fn agent(&self) -> terrars::ListRef<BedrockagentPrompt_Variant_GenAiResource_AgentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentPrompt_Variant_InferenceConfiguration_Text {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_tokens: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stop_sequences: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temperature: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_p: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentPrompt_Variant_InferenceConfiguration_Text {
    #[doc = "Set the field `max_tokens`.\n"]
    pub fn set_max_tokens(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_tokens = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `top_p`.\n"]
    pub fn set_top_p(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.top_p = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentPrompt_Variant_InferenceConfiguration_Text {
    type O = terrars::BlockAssignable<BedrockagentPrompt_Variant_InferenceConfiguration_Text>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant_InferenceConfiguration_Text {}

impl BuildBedrockagentPrompt_Variant_InferenceConfiguration_Text {
    pub fn build(self) -> BedrockagentPrompt_Variant_InferenceConfiguration_Text {
        BedrockagentPrompt_Variant_InferenceConfiguration_Text {
            max_tokens: core::default::Default::default(),
            stop_sequences: core::default::Default::default(),
            temperature: core::default::Default::default(),
            top_p: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_Variant_InferenceConfiguration_TextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPrompt_Variant_InferenceConfiguration_TextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentPrompt_Variant_InferenceConfiguration_TextRef {
        BedrockagentPrompt_Variant_InferenceConfiguration_TextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_Variant_InferenceConfiguration_TextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_tokens` after provisioning.\n"]
    pub fn max_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `stop_sequences` after provisioning.\n"]
    pub fn stop_sequences(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stop_sequences", self.base))
    }

    #[doc = "Get a reference to the value of field `temperature` after provisioning.\n"]
    pub fn temperature(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }

    #[doc = "Get a reference to the value of field `top_p` after provisioning.\n"]
    pub fn top_p(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.top_p", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPrompt_Variant_InferenceConfigurationDynamic {
    text: std::option::Option<terrars::DynamicBlock<BedrockagentPrompt_Variant_InferenceConfiguration_Text>>,
}

#[derive(Serialize)]
pub struct BedrockagentPrompt_Variant_InferenceConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<std::vec::Vec<BedrockagentPrompt_Variant_InferenceConfiguration_Text>>,
    dynamic: BedrockagentPrompt_Variant_InferenceConfigurationDynamic,
}

impl BedrockagentPrompt_Variant_InferenceConfiguration {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentPrompt_Variant_InferenceConfiguration_Text>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentPrompt_Variant_InferenceConfiguration {
    type O = terrars::BlockAssignable<BedrockagentPrompt_Variant_InferenceConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant_InferenceConfiguration {}

impl BuildBedrockagentPrompt_Variant_InferenceConfiguration {
    pub fn build(self) -> BedrockagentPrompt_Variant_InferenceConfiguration {
        BedrockagentPrompt_Variant_InferenceConfiguration {
            text: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_Variant_InferenceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPrompt_Variant_InferenceConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentPrompt_Variant_InferenceConfigurationRef {
        BedrockagentPrompt_Variant_InferenceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_Variant_InferenceConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::ListRef<BedrockagentPrompt_Variant_InferenceConfiguration_TextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentPrompt_Variant_Metadata {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl BedrockagentPrompt_Variant_Metadata { }

impl terrars::ToListMappable for BedrockagentPrompt_Variant_Metadata {
    type O = terrars::BlockAssignable<BedrockagentPrompt_Variant_Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant_Metadata {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentPrompt_Variant_Metadata {
    pub fn build(self) -> BedrockagentPrompt_Variant_Metadata {
        BedrockagentPrompt_Variant_Metadata {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_Variant_MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPrompt_Variant_MetadataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentPrompt_Variant_MetadataRef {
        BedrockagentPrompt_Variant_MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_Variant_MetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_InputVariable {
    name: terrars::PrimField<std::string::String>,
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_InputVariable { }

impl terrars::ToListMappable for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_InputVariable {
    type O = terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_InputVariable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_InputVariable {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_InputVariable {
    pub fn build(self) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_InputVariable {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_InputVariable { name: self.name }
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_InputVariableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_InputVariableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_InputVariableRef {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_InputVariableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_InputVariableRef {
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
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePoint {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePoint { }

impl terrars::ToListMappable for BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePoint {
    type O = terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePoint {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePoint {
    pub fn build(self) -> BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePoint {
        BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePoint { type_: self.type_ }
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePointRef {
        BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message_ContentDynamic {
    cache_point: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePoint>>,
}

#[derive(Serialize)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message_Content {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_point: std::option::Option<std::vec::Vec<BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePoint>>,
    dynamic: BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message_ContentDynamic,
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message_Content {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_point`.\n"]
    pub fn set_cache_point(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePoint>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cache_point = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_point = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message_Content {
    type O = terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message_Content>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message_Content {}

impl BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message_Content {
    pub fn build(self) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message_Content {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message_Content {
            text: core::default::Default::default(),
            cache_point: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message_ContentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message_ContentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message_ContentRef {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message_ContentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message_ContentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_point` after provisioning.\n"]
    pub fn cache_point(&self) -> terrars::ListRef<BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_point", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_MessageDynamic {
    content: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message_Content>>,
}

#[derive(Serialize)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message {
    role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std::option::Option<std::vec::Vec<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message_Content>>,
    dynamic: BedrockagentPrompt_Variant_TemplateConfiguration_Chat_MessageDynamic,
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message {
    #[doc = "Set the field `content`.\n"]
    pub fn set_content(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message_Content>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.content = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.content = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message {
    type O = terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message {
    #[doc = ""]
    pub role: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message {
    pub fn build(self) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message {
            role: self.role,
            content: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_MessageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_MessageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_MessageRef {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_MessageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_MessageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> terrars::ListRef<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message_ContentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_SystemDynamic {
    cache_point: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePoint>>,
}

#[derive(Serialize)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_System {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_point: std::option::Option<std::vec::Vec<BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePoint>>,
    dynamic: BedrockagentPrompt_Variant_TemplateConfiguration_Chat_SystemDynamic,
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_System {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_point`.\n"]
    pub fn set_cache_point(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePoint>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cache_point = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_point = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_System {
    type O = terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_System>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_System {}

impl BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_System {
    pub fn build(self) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_System {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_System {
            text: core::default::Default::default(),
            cache_point: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_SystemRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_SystemRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_SystemRef {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_SystemRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_SystemRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_point` after provisioning.\n"]
    pub fn cache_point(&self) -> terrars::ListRef<BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_point", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema {
    #[doc = "Set the field `json`.\n"]
    pub fn set_json(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.json = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema {}

impl BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema {
    pub fn build(
        self,
    ) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema {
            json: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchemaRef {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpecDynamic {
    input_schema: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_schema: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema>,
    >,
    dynamic: BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpecDynamic,
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_schema`.\n"]
    pub fn set_input_schema(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_schema = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec {
    type O =
        terrars
        ::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec {
    pub fn build(self) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec {
            description: core::default::Default::default(),
            name: self.name,
            input_schema: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpecRef {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpecRef {
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

    #[doc = "Get a reference to the value of field `input_schema` after provisioning.\n"]
    pub fn input_schema(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_schema", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolDynamic {
    cache_point: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePoint>>,
    tool_spec: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_point: std::option::Option<std::vec::Vec<BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePoint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_spec: std
    ::option
    ::Option<std::vec::Vec<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec>>,
    dynamic: BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolDynamic,
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool {
    #[doc = "Set the field `cache_point`.\n"]
    pub fn set_cache_point(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePoint>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cache_point = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_point = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tool_spec`.\n"]
    pub fn set_tool_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tool_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool {
    type O = terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool {}

impl BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool {
    pub fn build(self) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool {
            cache_point: core::default::Default::default(),
            tool_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolRef {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_point` after provisioning.\n"]
    pub fn cache_point(&self) -> terrars::ListRef<BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_point", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_spec` after provisioning.\n"]
    pub fn tool_spec(
        &self,
    ) -> terrars::ListRef<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any {}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any { }

impl terrars
::ToListMappable for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any {
    type O =
        terrars
        ::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any {}

impl BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any {
    pub fn build(self) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any {}
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AnyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AnyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AnyRef {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AnyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AnyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto {}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto { }

impl terrars
::ToListMappable for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto {
    type O =
        terrars
        ::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto {}

impl BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto {
    pub fn build(self) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto {}
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AutoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AutoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AutoRef {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AutoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AutoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool {
    name: terrars::PrimField<std::string::String>,
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool { }

impl terrars
::ToListMappable for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool {
    type O =
        terrars
        ::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool {
    pub fn build(self) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool { name: self.name }
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_ToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_ToolRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_ToolRef {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_ToolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_ToolRef {
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
struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoiceDynamic {
    any: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any>,
    >,
    auto: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto>,
    >,
    tool: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    any: std
    ::option
    ::Option<std::vec::Vec<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto: std
    ::option
    ::Option<std::vec::Vec<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool: std
    ::option
    ::Option<std::vec::Vec<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool>>,
    dynamic: BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoiceDynamic,
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice {
    #[doc = "Set the field `any`.\n"]
    pub fn set_any(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.any = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.any = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `auto`.\n"]
    pub fn set_auto(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auto = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auto = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tool`.\n"]
    pub fn set_tool(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tool = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tool = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice {
    type O =
        terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice {}

impl BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice {
    pub fn build(self) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice {
            any: core::default::Default::default(),
            auto: core::default::Default::default(),
            tool: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoiceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoiceRef {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoiceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoiceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `any` after provisioning.\n"]
    pub fn any(
        &self,
    ) -> terrars::ListRef<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AnyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.any", self.base))
    }

    #[doc = "Get a reference to the value of field `auto` after provisioning.\n"]
    pub fn auto(
        &self,
    ) -> terrars::ListRef<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AutoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto", self.base))
    }

    #[doc = "Get a reference to the value of field `tool` after provisioning.\n"]
    pub fn tool(
        &self,
    ) -> terrars::ListRef<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_ToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfigurationDynamic {
    tool: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool>>,
    tool_choice: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool: std
    ::option
    ::Option<std::vec::Vec<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_choice: std
    ::option
    ::Option<std::vec::Vec<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice>>,
    dynamic: BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfigurationDynamic,
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration {
    #[doc = "Set the field `tool`.\n"]
    pub fn set_tool(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_Tool,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tool = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tool = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tool_choice`.\n"]
    pub fn set_tool_choice(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tool_choice = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_choice = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration {
    type O = terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration {}

impl BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration {
    pub fn build(self) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration {
            tool: core::default::Default::default(),
            tool_choice: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfigurationRef {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tool` after provisioning.\n"]
    pub fn tool(
        &self,
    ) -> terrars::ListRef<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_choice` after provisioning.\n"]
    pub fn tool_choice(
        &self,
    ) -> terrars::ListRef<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration_ToolChoiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_choice", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPrompt_Variant_TemplateConfiguration_ChatDynamic {
    input_variable: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_InputVariable>>,
    message: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message>>,
    system: std::option::Option<terrars::DynamicBlock<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_System>>,
    tool_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration>>,
}

#[derive(Serialize)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Chat {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_variable: std
    ::option
    ::Option<std::vec::Vec<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_InputVariable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<std::vec::Vec<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system: std::option::Option<std::vec::Vec<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_System>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration>>,
    dynamic: BedrockagentPrompt_Variant_TemplateConfiguration_ChatDynamic,
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Chat {
    #[doc = "Set the field `input_variable`.\n"]
    pub fn set_input_variable(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_InputVariable>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_variable = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_variable = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_Message>>,
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

    #[doc = "Set the field `system`.\n"]
    pub fn set_system(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_System>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.system = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.system = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tool_configuration`.\n"]
    pub fn set_tool_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tool_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentPrompt_Variant_TemplateConfiguration_Chat {
    type O = terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Chat>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat {}

impl BuildBedrockagentPrompt_Variant_TemplateConfiguration_Chat {
    pub fn build(self) -> BedrockagentPrompt_Variant_TemplateConfiguration_Chat {
        BedrockagentPrompt_Variant_TemplateConfiguration_Chat {
            input_variable: core::default::Default::default(),
            message: core::default::Default::default(),
            system: core::default::Default::default(),
            tool_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_ChatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPrompt_Variant_TemplateConfiguration_ChatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentPrompt_Variant_TemplateConfiguration_ChatRef {
        BedrockagentPrompt_Variant_TemplateConfiguration_ChatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_ChatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_variable` after provisioning.\n"]
    pub fn input_variable(
        &self,
    ) -> terrars::ListRef<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_InputVariableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_variable", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::ListRef<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_MessageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `system` after provisioning.\n"]
    pub fn system(&self) -> terrars::ListRef<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_SystemRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.system", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_configuration` after provisioning.\n"]
    pub fn tool_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_ToolConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPrompt_Variant_TemplateConfiguration_TextDynamic {
    cache_point: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePoint>>,
    input_variable: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_InputVariable>>,
}

#[derive(Serialize)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_Text {
    text: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_point: std::option::Option<std::vec::Vec<BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePoint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_variable: std
    ::option
    ::Option<std::vec::Vec<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_InputVariable>>,
    dynamic: BedrockagentPrompt_Variant_TemplateConfiguration_TextDynamic,
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_Text {
    #[doc = "Set the field `cache_point`.\n"]
    pub fn set_cache_point(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePoint>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cache_point = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_point = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_variable`.\n"]
    pub fn set_input_variable(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_InputVariable>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_variable = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_variable = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentPrompt_Variant_TemplateConfiguration_Text {
    type O = terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Text>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant_TemplateConfiguration_Text {
    #[doc = ""]
    pub text: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentPrompt_Variant_TemplateConfiguration_Text {
    pub fn build(self) -> BedrockagentPrompt_Variant_TemplateConfiguration_Text {
        BedrockagentPrompt_Variant_TemplateConfiguration_Text {
            text: self.text,
            cache_point: core::default::Default::default(),
            input_variable: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration_TextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPrompt_Variant_TemplateConfiguration_TextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentPrompt_Variant_TemplateConfiguration_TextRef {
        BedrockagentPrompt_Variant_TemplateConfiguration_TextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_Variant_TemplateConfiguration_TextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_point` after provisioning.\n"]
    pub fn cache_point(&self) -> terrars::ListRef<BedrockagentPrompt_Variant_TemplateConfiguration_Text_CachePointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_point", self.base))
    }

    #[doc = "Get a reference to the value of field `input_variable` after provisioning.\n"]
    pub fn input_variable(
        &self,
    ) -> terrars::ListRef<BedrockagentPrompt_Variant_TemplateConfiguration_Chat_InputVariableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_variable", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPrompt_Variant_TemplateConfigurationDynamic {
    chat: std::option::Option<terrars::DynamicBlock<BedrockagentPrompt_Variant_TemplateConfiguration_Chat>>,
    text: std::option::Option<terrars::DynamicBlock<BedrockagentPrompt_Variant_TemplateConfiguration_Text>>,
}

#[derive(Serialize)]
pub struct BedrockagentPrompt_Variant_TemplateConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chat: std::option::Option<std::vec::Vec<BedrockagentPrompt_Variant_TemplateConfiguration_Chat>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<std::vec::Vec<BedrockagentPrompt_Variant_TemplateConfiguration_Text>>,
    dynamic: BedrockagentPrompt_Variant_TemplateConfigurationDynamic,
}

impl BedrockagentPrompt_Variant_TemplateConfiguration {
    #[doc = "Set the field `chat`.\n"]
    pub fn set_chat(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Chat>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.chat = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.chat = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text`.\n"]
    pub fn set_text(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration_Text>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentPrompt_Variant_TemplateConfiguration {
    type O = terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant_TemplateConfiguration {}

impl BuildBedrockagentPrompt_Variant_TemplateConfiguration {
    pub fn build(self) -> BedrockagentPrompt_Variant_TemplateConfiguration {
        BedrockagentPrompt_Variant_TemplateConfiguration {
            chat: core::default::Default::default(),
            text: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_Variant_TemplateConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPrompt_Variant_TemplateConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentPrompt_Variant_TemplateConfigurationRef {
        BedrockagentPrompt_Variant_TemplateConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_Variant_TemplateConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `chat` after provisioning.\n"]
    pub fn chat(&self) -> terrars::ListRef<BedrockagentPrompt_Variant_TemplateConfiguration_ChatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chat", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::ListRef<BedrockagentPrompt_Variant_TemplateConfiguration_TextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentPrompt_VariantDynamic {
    gen_ai_resource: std::option::Option<terrars::DynamicBlock<BedrockagentPrompt_Variant_GenAiResource>>,
    inference_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentPrompt_Variant_InferenceConfiguration>>,
    metadata: std::option::Option<terrars::DynamicBlock<BedrockagentPrompt_Variant_Metadata>>,
    template_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentPrompt_Variant_TemplateConfiguration>>,
}

#[derive(Serialize)]
pub struct BedrockagentPrompt_Variant {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_model_request_fields: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    template_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gen_ai_resource: std::option::Option<std::vec::Vec<BedrockagentPrompt_Variant_GenAiResource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_configuration: std::option::Option<std::vec::Vec<BedrockagentPrompt_Variant_InferenceConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<std::vec::Vec<BedrockagentPrompt_Variant_Metadata>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template_configuration: std::option::Option<std::vec::Vec<BedrockagentPrompt_Variant_TemplateConfiguration>>,
    dynamic: BedrockagentPrompt_VariantDynamic,
}

impl BedrockagentPrompt_Variant {
    #[doc = "Set the field `additional_model_request_fields`.\n"]
    pub fn set_additional_model_request_fields(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.additional_model_request_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_id`.\n"]
    pub fn set_model_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gen_ai_resource`.\n"]
    pub fn set_gen_ai_resource(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentPrompt_Variant_GenAiResource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gen_ai_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gen_ai_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inference_configuration`.\n"]
    pub fn set_inference_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentPrompt_Variant_InferenceConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inference_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inference_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentPrompt_Variant_Metadata>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `template_configuration`.\n"]
    pub fn set_template_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentPrompt_Variant_TemplateConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.template_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.template_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentPrompt_Variant {
    type O = terrars::BlockAssignable<BedrockagentPrompt_Variant>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentPrompt_Variant {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub template_type: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentPrompt_Variant {
    pub fn build(self) -> BedrockagentPrompt_Variant {
        BedrockagentPrompt_Variant {
            additional_model_request_fields: core::default::Default::default(),
            model_id: core::default::Default::default(),
            name: self.name,
            template_type: self.template_type,
            gen_ai_resource: core::default::Default::default(),
            inference_configuration: core::default::Default::default(),
            metadata: core::default::Default::default(),
            template_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentPrompt_VariantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentPrompt_VariantRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentPrompt_VariantRef {
        BedrockagentPrompt_VariantRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentPrompt_VariantRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_model_request_fields` after provisioning.\n"]
    pub fn additional_model_request_fields(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_model_request_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `template_type` after provisioning.\n"]
    pub fn template_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template_type", self.base))
    }

    #[doc = "Get a reference to the value of field `gen_ai_resource` after provisioning.\n"]
    pub fn gen_ai_resource(&self) -> terrars::ListRef<BedrockagentPrompt_Variant_GenAiResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gen_ai_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `inference_configuration` after provisioning.\n"]
    pub fn inference_configuration(&self) -> terrars::ListRef<BedrockagentPrompt_Variant_InferenceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inference_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::ListRef<BedrockagentPrompt_Variant_MetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `template_configuration` after provisioning.\n"]
    pub fn template_configuration(&self) -> terrars::ListRef<BedrockagentPrompt_Variant_TemplateConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template_configuration", self.base))
    }
}
