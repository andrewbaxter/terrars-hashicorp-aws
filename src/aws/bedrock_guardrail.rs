#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockGuardrailData_ {
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
    blocked_input_messaging: terrars::PrimField<std::string::String>,
    blocked_outputs_messaging: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_policy_config: std::option::Option<std::vec::Vec<BedrockGuardrail_ContentPolicyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contextual_grounding_policy_config: std
    ::option
    ::Option<std::vec::Vec<BedrockGuardrail_ContextualGroundingPolicyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_region_config: std::option::Option<std::vec::Vec<BedrockGuardrail_CrossRegionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitive_information_policy_config: std
    ::option
    ::Option<std::vec::Vec<BedrockGuardrail_SensitiveInformationPolicyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockGuardrail_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic_policy_config: std::option::Option<std::vec::Vec<BedrockGuardrail_TopicPolicyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    word_policy_config: std::option::Option<std::vec::Vec<BedrockGuardrail_WordPolicyConfig>>,
    dynamic: BedrockGuardrailDynamic,
}

struct BedrockGuardrail_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockGuardrailData_>,
}

#[derive(Clone)]
pub struct BedrockGuardrail(std::rc::Rc<BedrockGuardrail_>);

impl BedrockGuardrail {
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

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `content_policy_config`.\n"]
    pub fn set_content_policy_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockGuardrail_ContentPolicyConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().content_policy_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.content_policy_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `contextual_grounding_policy_config`.\n"]
    pub fn set_contextual_grounding_policy_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockGuardrail_ContextualGroundingPolicyConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().contextual_grounding_policy_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.contextual_grounding_policy_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cross_region_config`.\n"]
    pub fn set_cross_region_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockGuardrail_CrossRegionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cross_region_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cross_region_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sensitive_information_policy_config`.\n"]
    pub fn set_sensitive_information_policy_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockGuardrail_SensitiveInformationPolicyConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sensitive_information_policy_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sensitive_information_policy_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockGuardrail_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `topic_policy_config`.\n"]
    pub fn set_topic_policy_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockGuardrail_TopicPolicyConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().topic_policy_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.topic_policy_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `word_policy_config`.\n"]
    pub fn set_word_policy_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockGuardrail_WordPolicyConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().word_policy_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.word_policy_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `blocked_input_messaging` after provisioning.\n"]
    pub fn blocked_input_messaging(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.blocked_input_messaging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blocked_outputs_messaging` after provisioning.\n"]
    pub fn blocked_outputs_messaging(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.blocked_outputs_messaging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardrail_arn` after provisioning.\n"]
    pub fn guardrail_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guardrail_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardrail_id` after provisioning.\n"]
    pub fn guardrail_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guardrail_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `content_policy_config` after provisioning.\n"]
    pub fn content_policy_config(&self) -> terrars::ListRef<BedrockGuardrail_ContentPolicyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_policy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `contextual_grounding_policy_config` after provisioning.\n"]
    pub fn contextual_grounding_policy_config(
        &self,
    ) -> terrars::ListRef<BedrockGuardrail_ContextualGroundingPolicyConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.contextual_grounding_policy_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cross_region_config` after provisioning.\n"]
    pub fn cross_region_config(&self) -> terrars::ListRef<BedrockGuardrail_CrossRegionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cross_region_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sensitive_information_policy_config` after provisioning.\n"]
    pub fn sensitive_information_policy_config(
        &self,
    ) -> terrars::ListRef<BedrockGuardrail_SensitiveInformationPolicyConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.sensitive_information_policy_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockGuardrail_TimeoutsRef {
        BedrockGuardrail_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `topic_policy_config` after provisioning.\n"]
    pub fn topic_policy_config(&self) -> terrars::ListRef<BedrockGuardrail_TopicPolicyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topic_policy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `word_policy_config` after provisioning.\n"]
    pub fn word_policy_config(&self) -> terrars::ListRef<BedrockGuardrail_WordPolicyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.word_policy_config", self.extract_ref()))
    }
}

impl terrars::Referable for BedrockGuardrail {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockGuardrail { }

impl terrars::ToListMappable for BedrockGuardrail {
    type O = terrars::ListRef<BedrockGuardrailRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockGuardrail_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrock_guardrail".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockGuardrail {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub blocked_input_messaging: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub blocked_outputs_messaging: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockGuardrail {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockGuardrail {
        let out = BedrockGuardrail(std::rc::Rc::new(BedrockGuardrail_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockGuardrailData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                blocked_input_messaging: self.blocked_input_messaging,
                blocked_outputs_messaging: self.blocked_outputs_messaging,
                description: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                content_policy_config: core::default::Default::default(),
                contextual_grounding_policy_config: core::default::Default::default(),
                cross_region_config: core::default::Default::default(),
                sensitive_information_policy_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                topic_policy_config: core::default::Default::default(),
                word_policy_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockGuardrailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockGuardrailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockGuardrailRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `blocked_input_messaging` after provisioning.\n"]
    pub fn blocked_input_messaging(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.blocked_input_messaging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blocked_outputs_messaging` after provisioning.\n"]
    pub fn blocked_outputs_messaging(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.blocked_outputs_messaging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardrail_arn` after provisioning.\n"]
    pub fn guardrail_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guardrail_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardrail_id` after provisioning.\n"]
    pub fn guardrail_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guardrail_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `content_policy_config` after provisioning.\n"]
    pub fn content_policy_config(&self) -> terrars::ListRef<BedrockGuardrail_ContentPolicyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_policy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `contextual_grounding_policy_config` after provisioning.\n"]
    pub fn contextual_grounding_policy_config(
        &self,
    ) -> terrars::ListRef<BedrockGuardrail_ContextualGroundingPolicyConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.contextual_grounding_policy_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cross_region_config` after provisioning.\n"]
    pub fn cross_region_config(&self) -> terrars::ListRef<BedrockGuardrail_CrossRegionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cross_region_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sensitive_information_policy_config` after provisioning.\n"]
    pub fn sensitive_information_policy_config(
        &self,
    ) -> terrars::ListRef<BedrockGuardrail_SensitiveInformationPolicyConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.sensitive_information_policy_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockGuardrail_TimeoutsRef {
        BedrockGuardrail_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `topic_policy_config` after provisioning.\n"]
    pub fn topic_policy_config(&self) -> terrars::ListRef<BedrockGuardrail_TopicPolicyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topic_policy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `word_policy_config` after provisioning.\n"]
    pub fn word_policy_config(&self) -> terrars::ListRef<BedrockGuardrail_WordPolicyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.word_policy_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BedrockGuardrailDynamic {
    content_policy_config: std::option::Option<terrars::DynamicBlock<BedrockGuardrail_ContentPolicyConfig>>,
    contextual_grounding_policy_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockGuardrail_ContextualGroundingPolicyConfig>>,
    cross_region_config: std::option::Option<terrars::DynamicBlock<BedrockGuardrail_CrossRegionConfig>>,
    sensitive_information_policy_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockGuardrail_SensitiveInformationPolicyConfig>>,
    topic_policy_config: std::option::Option<terrars::DynamicBlock<BedrockGuardrail_TopicPolicyConfig>>,
    word_policy_config: std::option::Option<terrars::DynamicBlock<BedrockGuardrail_WordPolicyConfig>>,
}

#[derive(Serialize)]
pub struct BedrockGuardrail_TopicPolicyConfig_TierConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tier_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockGuardrail_TopicPolicyConfig_TierConfig {
    #[doc = "Set the field `tier_name`.\n"]
    pub fn set_tier_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tier_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockGuardrail_TopicPolicyConfig_TierConfig {
    type O = terrars::BlockAssignable<BedrockGuardrail_TopicPolicyConfig_TierConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrail_TopicPolicyConfig_TierConfig {}

impl BuildBedrockGuardrail_TopicPolicyConfig_TierConfig {
    pub fn build(self) -> BedrockGuardrail_TopicPolicyConfig_TierConfig {
        BedrockGuardrail_TopicPolicyConfig_TierConfig { tier_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BedrockGuardrail_TopicPolicyConfig_TierConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockGuardrail_TopicPolicyConfig_TierConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockGuardrail_TopicPolicyConfig_TierConfigRef {
        BedrockGuardrail_TopicPolicyConfig_TierConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrail_TopicPolicyConfig_TierConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tier_name` after provisioning.\n"]
    pub fn tier_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier_name", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockGuardrail_ContentPolicyConfig_FiltersConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_modalities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    input_strength: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_modalities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    output_strength: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl BedrockGuardrail_ContentPolicyConfig_FiltersConfig {
    #[doc = "Set the field `input_action`.\n"]
    pub fn set_input_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_enabled`.\n"]
    pub fn set_input_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.input_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_modalities`.\n"]
    pub fn set_input_modalities(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.input_modalities = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_action`.\n"]
    pub fn set_output_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_enabled`.\n"]
    pub fn set_output_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.output_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_modalities`.\n"]
    pub fn set_output_modalities(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.output_modalities = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockGuardrail_ContentPolicyConfig_FiltersConfig {
    type O = terrars::BlockAssignable<BedrockGuardrail_ContentPolicyConfig_FiltersConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrail_ContentPolicyConfig_FiltersConfig {
    #[doc = ""]
    pub input_strength: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub output_strength: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockGuardrail_ContentPolicyConfig_FiltersConfig {
    pub fn build(self) -> BedrockGuardrail_ContentPolicyConfig_FiltersConfig {
        BedrockGuardrail_ContentPolicyConfig_FiltersConfig {
            input_action: core::default::Default::default(),
            input_enabled: core::default::Default::default(),
            input_modalities: core::default::Default::default(),
            input_strength: self.input_strength,
            output_action: core::default::Default::default(),
            output_enabled: core::default::Default::default(),
            output_modalities: core::default::Default::default(),
            output_strength: self.output_strength,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockGuardrail_ContentPolicyConfig_FiltersConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockGuardrail_ContentPolicyConfig_FiltersConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockGuardrail_ContentPolicyConfig_FiltersConfigRef {
        BedrockGuardrail_ContentPolicyConfig_FiltersConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrail_ContentPolicyConfig_FiltersConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_action` after provisioning.\n"]
    pub fn input_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_action", self.base))
    }

    #[doc = "Get a reference to the value of field `input_enabled` after provisioning.\n"]
    pub fn input_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `input_modalities` after provisioning.\n"]
    pub fn input_modalities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.input_modalities", self.base))
    }

    #[doc = "Get a reference to the value of field `input_strength` after provisioning.\n"]
    pub fn input_strength(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_strength", self.base))
    }

    #[doc = "Get a reference to the value of field `output_action` after provisioning.\n"]
    pub fn output_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_action", self.base))
    }

    #[doc = "Get a reference to the value of field `output_enabled` after provisioning.\n"]
    pub fn output_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `output_modalities` after provisioning.\n"]
    pub fn output_modalities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.output_modalities", self.base))
    }

    #[doc = "Get a reference to the value of field `output_strength` after provisioning.\n"]
    pub fn output_strength(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_strength", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockGuardrail_ContentPolicyConfigDynamic {
    filters_config: std::option::Option<terrars::DynamicBlock<BedrockGuardrail_ContentPolicyConfig_FiltersConfig>>,
}

#[derive(Serialize)]
pub struct BedrockGuardrail_ContentPolicyConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tier_config: std::option::Option<terrars::ListField<BedrockGuardrail_TopicPolicyConfig_TierConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filters_config: std::option::Option<std::vec::Vec<BedrockGuardrail_ContentPolicyConfig_FiltersConfig>>,
    dynamic: BedrockGuardrail_ContentPolicyConfigDynamic,
}

impl BedrockGuardrail_ContentPolicyConfig {
    #[doc = "Set the field `tier_config`.\n"]
    pub fn set_tier_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<BedrockGuardrail_TopicPolicyConfig_TierConfig>>,
    ) -> Self {
        self.tier_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filters_config`.\n"]
    pub fn set_filters_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockGuardrail_ContentPolicyConfig_FiltersConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filters_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filters_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockGuardrail_ContentPolicyConfig {
    type O = terrars::BlockAssignable<BedrockGuardrail_ContentPolicyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrail_ContentPolicyConfig {}

impl BuildBedrockGuardrail_ContentPolicyConfig {
    pub fn build(self) -> BedrockGuardrail_ContentPolicyConfig {
        BedrockGuardrail_ContentPolicyConfig {
            tier_config: core::default::Default::default(),
            filters_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockGuardrail_ContentPolicyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockGuardrail_ContentPolicyConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockGuardrail_ContentPolicyConfigRef {
        BedrockGuardrail_ContentPolicyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrail_ContentPolicyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tier_config` after provisioning.\n"]
    pub fn tier_config(&self) -> terrars::ListRef<BedrockGuardrail_TopicPolicyConfig_TierConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tier_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockGuardrail_ContextualGroundingPolicyConfig_FiltersConfig {
    threshold: terrars::PrimField<f64>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl BedrockGuardrail_ContextualGroundingPolicyConfig_FiltersConfig { }

impl terrars::ToListMappable for BedrockGuardrail_ContextualGroundingPolicyConfig_FiltersConfig {
    type O = terrars::BlockAssignable<BedrockGuardrail_ContextualGroundingPolicyConfig_FiltersConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrail_ContextualGroundingPolicyConfig_FiltersConfig {
    #[doc = ""]
    pub threshold: terrars::PrimField<f64>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockGuardrail_ContextualGroundingPolicyConfig_FiltersConfig {
    pub fn build(self) -> BedrockGuardrail_ContextualGroundingPolicyConfig_FiltersConfig {
        BedrockGuardrail_ContextualGroundingPolicyConfig_FiltersConfig {
            threshold: self.threshold,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockGuardrail_ContextualGroundingPolicyConfig_FiltersConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockGuardrail_ContextualGroundingPolicyConfig_FiltersConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockGuardrail_ContextualGroundingPolicyConfig_FiltersConfigRef {
        BedrockGuardrail_ContextualGroundingPolicyConfig_FiltersConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrail_ContextualGroundingPolicyConfig_FiltersConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `threshold` after provisioning.\n"]
    pub fn threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockGuardrail_ContextualGroundingPolicyConfigDynamic {
    filters_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockGuardrail_ContextualGroundingPolicyConfig_FiltersConfig>>,
}

#[derive(Serialize)]
pub struct BedrockGuardrail_ContextualGroundingPolicyConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filters_config: std::option::Option<std::vec::Vec<BedrockGuardrail_ContextualGroundingPolicyConfig_FiltersConfig>>,
    dynamic: BedrockGuardrail_ContextualGroundingPolicyConfigDynamic,
}

impl BedrockGuardrail_ContextualGroundingPolicyConfig {
    #[doc = "Set the field `filters_config`.\n"]
    pub fn set_filters_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockGuardrail_ContextualGroundingPolicyConfig_FiltersConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filters_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filters_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockGuardrail_ContextualGroundingPolicyConfig {
    type O = terrars::BlockAssignable<BedrockGuardrail_ContextualGroundingPolicyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrail_ContextualGroundingPolicyConfig {}

impl BuildBedrockGuardrail_ContextualGroundingPolicyConfig {
    pub fn build(self) -> BedrockGuardrail_ContextualGroundingPolicyConfig {
        BedrockGuardrail_ContextualGroundingPolicyConfig {
            filters_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockGuardrail_ContextualGroundingPolicyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockGuardrail_ContextualGroundingPolicyConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockGuardrail_ContextualGroundingPolicyConfigRef {
        BedrockGuardrail_ContextualGroundingPolicyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrail_ContextualGroundingPolicyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filters_config` after provisioning.\n"]
    pub fn filters_config(&self) -> terrars::ListRef<BedrockGuardrail_ContextualGroundingPolicyConfig_FiltersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockGuardrail_CrossRegionConfig {
    guardrail_profile_identifier: terrars::PrimField<std::string::String>,
}

impl BedrockGuardrail_CrossRegionConfig { }

impl terrars::ToListMappable for BedrockGuardrail_CrossRegionConfig {
    type O = terrars::BlockAssignable<BedrockGuardrail_CrossRegionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrail_CrossRegionConfig {
    #[doc = ""]
    pub guardrail_profile_identifier: terrars::PrimField<std::string::String>,
}

impl BuildBedrockGuardrail_CrossRegionConfig {
    pub fn build(self) -> BedrockGuardrail_CrossRegionConfig {
        BedrockGuardrail_CrossRegionConfig { guardrail_profile_identifier: self.guardrail_profile_identifier }
    }
}

#[allow(dead_code)]
pub struct BedrockGuardrail_CrossRegionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockGuardrail_CrossRegionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockGuardrail_CrossRegionConfigRef {
        BedrockGuardrail_CrossRegionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrail_CrossRegionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `guardrail_profile_identifier` after provisioning.\n"]
    pub fn guardrail_profile_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guardrail_profile_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockGuardrail_SensitiveInformationPolicyConfig_PiiEntitiesConfig {
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl BedrockGuardrail_SensitiveInformationPolicyConfig_PiiEntitiesConfig {
    #[doc = "Set the field `input_action`.\n"]
    pub fn set_input_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_enabled`.\n"]
    pub fn set_input_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.input_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_action`.\n"]
    pub fn set_output_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_enabled`.\n"]
    pub fn set_output_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.output_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockGuardrail_SensitiveInformationPolicyConfig_PiiEntitiesConfig {
    type O = terrars::BlockAssignable<BedrockGuardrail_SensitiveInformationPolicyConfig_PiiEntitiesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrail_SensitiveInformationPolicyConfig_PiiEntitiesConfig {
    #[doc = ""]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockGuardrail_SensitiveInformationPolicyConfig_PiiEntitiesConfig {
    pub fn build(self) -> BedrockGuardrail_SensitiveInformationPolicyConfig_PiiEntitiesConfig {
        BedrockGuardrail_SensitiveInformationPolicyConfig_PiiEntitiesConfig {
            action: self.action,
            input_action: core::default::Default::default(),
            input_enabled: core::default::Default::default(),
            output_action: core::default::Default::default(),
            output_enabled: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockGuardrail_SensitiveInformationPolicyConfig_PiiEntitiesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockGuardrail_SensitiveInformationPolicyConfig_PiiEntitiesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockGuardrail_SensitiveInformationPolicyConfig_PiiEntitiesConfigRef {
        BedrockGuardrail_SensitiveInformationPolicyConfig_PiiEntitiesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrail_SensitiveInformationPolicyConfig_PiiEntitiesConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `input_action` after provisioning.\n"]
    pub fn input_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_action", self.base))
    }

    #[doc = "Get a reference to the value of field `input_enabled` after provisioning.\n"]
    pub fn input_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `output_action` after provisioning.\n"]
    pub fn output_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_action", self.base))
    }

    #[doc = "Get a reference to the value of field `output_enabled` after provisioning.\n"]
    pub fn output_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockGuardrail_SensitiveInformationPolicyConfig_RegexesConfig {
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_enabled: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_enabled: std::option::Option<terrars::PrimField<bool>>,
    pattern: terrars::PrimField<std::string::String>,
}

impl BedrockGuardrail_SensitiveInformationPolicyConfig_RegexesConfig {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_action`.\n"]
    pub fn set_input_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_enabled`.\n"]
    pub fn set_input_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.input_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_action`.\n"]
    pub fn set_output_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_enabled`.\n"]
    pub fn set_output_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.output_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockGuardrail_SensitiveInformationPolicyConfig_RegexesConfig {
    type O = terrars::BlockAssignable<BedrockGuardrail_SensitiveInformationPolicyConfig_RegexesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrail_SensitiveInformationPolicyConfig_RegexesConfig {
    #[doc = ""]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub pattern: terrars::PrimField<std::string::String>,
}

impl BuildBedrockGuardrail_SensitiveInformationPolicyConfig_RegexesConfig {
    pub fn build(self) -> BedrockGuardrail_SensitiveInformationPolicyConfig_RegexesConfig {
        BedrockGuardrail_SensitiveInformationPolicyConfig_RegexesConfig {
            action: self.action,
            description: core::default::Default::default(),
            input_action: core::default::Default::default(),
            input_enabled: core::default::Default::default(),
            name: self.name,
            output_action: core::default::Default::default(),
            output_enabled: core::default::Default::default(),
            pattern: self.pattern,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockGuardrail_SensitiveInformationPolicyConfig_RegexesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockGuardrail_SensitiveInformationPolicyConfig_RegexesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockGuardrail_SensitiveInformationPolicyConfig_RegexesConfigRef {
        BedrockGuardrail_SensitiveInformationPolicyConfig_RegexesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrail_SensitiveInformationPolicyConfig_RegexesConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `input_action` after provisioning.\n"]
    pub fn input_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_action", self.base))
    }

    #[doc = "Get a reference to the value of field `input_enabled` after provisioning.\n"]
    pub fn input_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `output_action` after provisioning.\n"]
    pub fn output_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_action", self.base))
    }

    #[doc = "Get a reference to the value of field `output_enabled` after provisioning.\n"]
    pub fn output_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockGuardrail_SensitiveInformationPolicyConfigDynamic {
    pii_entities_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockGuardrail_SensitiveInformationPolicyConfig_PiiEntitiesConfig>>,
    regexes_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockGuardrail_SensitiveInformationPolicyConfig_RegexesConfig>>,
}

#[derive(Serialize)]
pub struct BedrockGuardrail_SensitiveInformationPolicyConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pii_entities_config: std
    ::option
    ::Option<std::vec::Vec<BedrockGuardrail_SensitiveInformationPolicyConfig_PiiEntitiesConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regexes_config: std::option::Option<std::vec::Vec<BedrockGuardrail_SensitiveInformationPolicyConfig_RegexesConfig>>,
    dynamic: BedrockGuardrail_SensitiveInformationPolicyConfigDynamic,
}

impl BedrockGuardrail_SensitiveInformationPolicyConfig {
    #[doc = "Set the field `pii_entities_config`.\n"]
    pub fn set_pii_entities_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockGuardrail_SensitiveInformationPolicyConfig_PiiEntitiesConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pii_entities_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pii_entities_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regexes_config`.\n"]
    pub fn set_regexes_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockGuardrail_SensitiveInformationPolicyConfig_RegexesConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regexes_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regexes_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockGuardrail_SensitiveInformationPolicyConfig {
    type O = terrars::BlockAssignable<BedrockGuardrail_SensitiveInformationPolicyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrail_SensitiveInformationPolicyConfig {}

impl BuildBedrockGuardrail_SensitiveInformationPolicyConfig {
    pub fn build(self) -> BedrockGuardrail_SensitiveInformationPolicyConfig {
        BedrockGuardrail_SensitiveInformationPolicyConfig {
            pii_entities_config: core::default::Default::default(),
            regexes_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockGuardrail_SensitiveInformationPolicyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockGuardrail_SensitiveInformationPolicyConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockGuardrail_SensitiveInformationPolicyConfigRef {
        BedrockGuardrail_SensitiveInformationPolicyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrail_SensitiveInformationPolicyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pii_entities_config` after provisioning.\n"]
    pub fn pii_entities_config(
        &self,
    ) -> terrars::ListRef<BedrockGuardrail_SensitiveInformationPolicyConfig_PiiEntitiesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pii_entities_config", self.base))
    }

    #[doc = "Get a reference to the value of field `regexes_config` after provisioning.\n"]
    pub fn regexes_config(&self) -> terrars::ListRef<BedrockGuardrail_SensitiveInformationPolicyConfig_RegexesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regexes_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockGuardrail_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockGuardrail_Timeouts {
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

impl terrars::ToListMappable for BedrockGuardrail_Timeouts {
    type O = terrars::BlockAssignable<BedrockGuardrail_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrail_Timeouts {}

impl BuildBedrockGuardrail_Timeouts {
    pub fn build(self) -> BedrockGuardrail_Timeouts {
        BedrockGuardrail_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockGuardrail_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockGuardrail_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockGuardrail_TimeoutsRef {
        BedrockGuardrail_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrail_TimeoutsRef {
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
pub struct BedrockGuardrail_TopicPolicyConfig_TopicsConfig {
    definition: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    examples: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl BedrockGuardrail_TopicPolicyConfig_TopicsConfig {
    #[doc = "Set the field `examples`.\n"]
    pub fn set_examples(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.examples = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockGuardrail_TopicPolicyConfig_TopicsConfig {
    type O = terrars::BlockAssignable<BedrockGuardrail_TopicPolicyConfig_TopicsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrail_TopicPolicyConfig_TopicsConfig {
    #[doc = ""]
    pub definition: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockGuardrail_TopicPolicyConfig_TopicsConfig {
    pub fn build(self) -> BedrockGuardrail_TopicPolicyConfig_TopicsConfig {
        BedrockGuardrail_TopicPolicyConfig_TopicsConfig {
            definition: self.definition,
            examples: core::default::Default::default(),
            name: self.name,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockGuardrail_TopicPolicyConfig_TopicsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockGuardrail_TopicPolicyConfig_TopicsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockGuardrail_TopicPolicyConfig_TopicsConfigRef {
        BedrockGuardrail_TopicPolicyConfig_TopicsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrail_TopicPolicyConfig_TopicsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.definition", self.base))
    }

    #[doc = "Get a reference to the value of field `examples` after provisioning.\n"]
    pub fn examples(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.examples", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockGuardrail_TopicPolicyConfigDynamic {
    topics_config: std::option::Option<terrars::DynamicBlock<BedrockGuardrail_TopicPolicyConfig_TopicsConfig>>,
}

#[derive(Serialize)]
pub struct BedrockGuardrail_TopicPolicyConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tier_config: std::option::Option<terrars::ListField<BedrockGuardrail_TopicPolicyConfig_TierConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topics_config: std::option::Option<std::vec::Vec<BedrockGuardrail_TopicPolicyConfig_TopicsConfig>>,
    dynamic: BedrockGuardrail_TopicPolicyConfigDynamic,
}

impl BedrockGuardrail_TopicPolicyConfig {
    #[doc = "Set the field `tier_config`.\n"]
    pub fn set_tier_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<BedrockGuardrail_TopicPolicyConfig_TierConfig>>,
    ) -> Self {
        self.tier_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `topics_config`.\n"]
    pub fn set_topics_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockGuardrail_TopicPolicyConfig_TopicsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.topics_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.topics_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockGuardrail_TopicPolicyConfig {
    type O = terrars::BlockAssignable<BedrockGuardrail_TopicPolicyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrail_TopicPolicyConfig {}

impl BuildBedrockGuardrail_TopicPolicyConfig {
    pub fn build(self) -> BedrockGuardrail_TopicPolicyConfig {
        BedrockGuardrail_TopicPolicyConfig {
            tier_config: core::default::Default::default(),
            topics_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockGuardrail_TopicPolicyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockGuardrail_TopicPolicyConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockGuardrail_TopicPolicyConfigRef {
        BedrockGuardrail_TopicPolicyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrail_TopicPolicyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tier_config` after provisioning.\n"]
    pub fn tier_config(&self) -> terrars::ListRef<BedrockGuardrail_TopicPolicyConfig_TierConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tier_config", self.base))
    }

    #[doc = "Get a reference to the value of field `topics_config` after provisioning.\n"]
    pub fn topics_config(&self) -> terrars::ListRef<BedrockGuardrail_TopicPolicyConfig_TopicsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topics_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockGuardrail_WordPolicyConfig_ManagedWordListsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl BedrockGuardrail_WordPolicyConfig_ManagedWordListsConfig {
    #[doc = "Set the field `input_action`.\n"]
    pub fn set_input_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_enabled`.\n"]
    pub fn set_input_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.input_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_action`.\n"]
    pub fn set_output_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_enabled`.\n"]
    pub fn set_output_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.output_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockGuardrail_WordPolicyConfig_ManagedWordListsConfig {
    type O = terrars::BlockAssignable<BedrockGuardrail_WordPolicyConfig_ManagedWordListsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrail_WordPolicyConfig_ManagedWordListsConfig {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockGuardrail_WordPolicyConfig_ManagedWordListsConfig {
    pub fn build(self) -> BedrockGuardrail_WordPolicyConfig_ManagedWordListsConfig {
        BedrockGuardrail_WordPolicyConfig_ManagedWordListsConfig {
            input_action: core::default::Default::default(),
            input_enabled: core::default::Default::default(),
            output_action: core::default::Default::default(),
            output_enabled: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockGuardrail_WordPolicyConfig_ManagedWordListsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockGuardrail_WordPolicyConfig_ManagedWordListsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockGuardrail_WordPolicyConfig_ManagedWordListsConfigRef {
        BedrockGuardrail_WordPolicyConfig_ManagedWordListsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrail_WordPolicyConfig_ManagedWordListsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_action` after provisioning.\n"]
    pub fn input_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_action", self.base))
    }

    #[doc = "Get a reference to the value of field `input_enabled` after provisioning.\n"]
    pub fn input_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `output_action` after provisioning.\n"]
    pub fn output_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_action", self.base))
    }

    #[doc = "Get a reference to the value of field `output_enabled` after provisioning.\n"]
    pub fn output_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockGuardrail_WordPolicyConfig_WordsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_enabled: std::option::Option<terrars::PrimField<bool>>,
    text: terrars::PrimField<std::string::String>,
}

impl BedrockGuardrail_WordPolicyConfig_WordsConfig {
    #[doc = "Set the field `input_action`.\n"]
    pub fn set_input_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_enabled`.\n"]
    pub fn set_input_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.input_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_action`.\n"]
    pub fn set_output_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_enabled`.\n"]
    pub fn set_output_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.output_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockGuardrail_WordPolicyConfig_WordsConfig {
    type O = terrars::BlockAssignable<BedrockGuardrail_WordPolicyConfig_WordsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrail_WordPolicyConfig_WordsConfig {
    #[doc = ""]
    pub text: terrars::PrimField<std::string::String>,
}

impl BuildBedrockGuardrail_WordPolicyConfig_WordsConfig {
    pub fn build(self) -> BedrockGuardrail_WordPolicyConfig_WordsConfig {
        BedrockGuardrail_WordPolicyConfig_WordsConfig {
            input_action: core::default::Default::default(),
            input_enabled: core::default::Default::default(),
            output_action: core::default::Default::default(),
            output_enabled: core::default::Default::default(),
            text: self.text,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockGuardrail_WordPolicyConfig_WordsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockGuardrail_WordPolicyConfig_WordsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockGuardrail_WordPolicyConfig_WordsConfigRef {
        BedrockGuardrail_WordPolicyConfig_WordsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrail_WordPolicyConfig_WordsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_action` after provisioning.\n"]
    pub fn input_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_action", self.base))
    }

    #[doc = "Get a reference to the value of field `input_enabled` after provisioning.\n"]
    pub fn input_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `output_action` after provisioning.\n"]
    pub fn output_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_action", self.base))
    }

    #[doc = "Get a reference to the value of field `output_enabled` after provisioning.\n"]
    pub fn output_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockGuardrail_WordPolicyConfigDynamic {
    managed_word_lists_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockGuardrail_WordPolicyConfig_ManagedWordListsConfig>>,
    words_config: std::option::Option<terrars::DynamicBlock<BedrockGuardrail_WordPolicyConfig_WordsConfig>>,
}

#[derive(Serialize)]
pub struct BedrockGuardrail_WordPolicyConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_word_lists_config: std
    ::option
    ::Option<std::vec::Vec<BedrockGuardrail_WordPolicyConfig_ManagedWordListsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    words_config: std::option::Option<std::vec::Vec<BedrockGuardrail_WordPolicyConfig_WordsConfig>>,
    dynamic: BedrockGuardrail_WordPolicyConfigDynamic,
}

impl BedrockGuardrail_WordPolicyConfig {
    #[doc = "Set the field `managed_word_lists_config`.\n"]
    pub fn set_managed_word_lists_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockGuardrail_WordPolicyConfig_ManagedWordListsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_word_lists_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_word_lists_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `words_config`.\n"]
    pub fn set_words_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockGuardrail_WordPolicyConfig_WordsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.words_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.words_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockGuardrail_WordPolicyConfig {
    type O = terrars::BlockAssignable<BedrockGuardrail_WordPolicyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockGuardrail_WordPolicyConfig {}

impl BuildBedrockGuardrail_WordPolicyConfig {
    pub fn build(self) -> BedrockGuardrail_WordPolicyConfig {
        BedrockGuardrail_WordPolicyConfig {
            managed_word_lists_config: core::default::Default::default(),
            words_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockGuardrail_WordPolicyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockGuardrail_WordPolicyConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockGuardrail_WordPolicyConfigRef {
        BedrockGuardrail_WordPolicyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockGuardrail_WordPolicyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `managed_word_lists_config` after provisioning.\n"]
    pub fn managed_word_lists_config(
        &self,
    ) -> terrars::ListRef<BedrockGuardrail_WordPolicyConfig_ManagedWordListsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_word_lists_config", self.base))
    }

    #[doc = "Get a reference to the value of field `words_config` after provisioning.\n"]
    pub fn words_config(&self) -> terrars::ListRef<BedrockGuardrail_WordPolicyConfig_WordsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.words_config", self.base))
    }
}
