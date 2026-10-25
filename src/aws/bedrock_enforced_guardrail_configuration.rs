#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockEnforcedGuardrailConfigurationData_ {
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
    guardrail_arn: terrars::PrimField<std::string::String>,
    guardrail_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_enforcement: std::option::Option<std::vec::Vec<BedrockEnforcedGuardrailConfiguration_ModelEnforcement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selective_content_guarding: std
    ::option
    ::Option<std::vec::Vec<BedrockEnforcedGuardrailConfiguration_SelectiveContentGuarding>>,
    dynamic: BedrockEnforcedGuardrailConfigurationDynamic,
}

struct BedrockEnforcedGuardrailConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockEnforcedGuardrailConfigurationData_>,
}

#[derive(Clone)]
pub struct BedrockEnforcedGuardrailConfiguration(std::rc::Rc<BedrockEnforcedGuardrailConfiguration_>);

impl BedrockEnforcedGuardrailConfiguration {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_enforcement`.\n"]
    pub fn set_model_enforcement(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockEnforcedGuardrailConfiguration_ModelEnforcement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().model_enforcement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.model_enforcement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `selective_content_guarding`.\n"]
    pub fn set_selective_content_guarding(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockEnforcedGuardrailConfiguration_SelectiveContentGuarding>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().selective_content_guarding = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.selective_content_guarding = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `config_id` after provisioning.\n"]
    pub fn config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardrail_arn` after provisioning.\n"]
    pub fn guardrail_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guardrail_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardrail_id` after provisioning.\n"]
    pub fn guardrail_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guardrail_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardrail_version` after provisioning.\n"]
    pub fn guardrail_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guardrail_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_enforcement` after provisioning.\n"]
    pub fn model_enforcement(&self) -> terrars::ListRef<BedrockEnforcedGuardrailConfiguration_ModelEnforcementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_enforcement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `selective_content_guarding` after provisioning.\n"]
    pub fn selective_content_guarding(
        &self,
    ) -> terrars::ListRef<BedrockEnforcedGuardrailConfiguration_SelectiveContentGuardingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selective_content_guarding", self.extract_ref()))
    }
}

impl terrars::Referable for BedrockEnforcedGuardrailConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockEnforcedGuardrailConfiguration { }

impl terrars::ToListMappable for BedrockEnforcedGuardrailConfiguration {
    type O = terrars::ListRef<BedrockEnforcedGuardrailConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockEnforcedGuardrailConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrock_enforced_guardrail_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockEnforcedGuardrailConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub guardrail_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub guardrail_version: terrars::PrimField<std::string::String>,
}

impl BuildBedrockEnforcedGuardrailConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockEnforcedGuardrailConfiguration {
        let out = BedrockEnforcedGuardrailConfiguration(std::rc::Rc::new(BedrockEnforcedGuardrailConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockEnforcedGuardrailConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                guardrail_arn: self.guardrail_arn,
                guardrail_version: self.guardrail_version,
                region: core::default::Default::default(),
                model_enforcement: core::default::Default::default(),
                selective_content_guarding: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockEnforcedGuardrailConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEnforcedGuardrailConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockEnforcedGuardrailConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `config_id` after provisioning.\n"]
    pub fn config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardrail_arn` after provisioning.\n"]
    pub fn guardrail_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guardrail_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardrail_id` after provisioning.\n"]
    pub fn guardrail_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guardrail_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardrail_version` after provisioning.\n"]
    pub fn guardrail_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guardrail_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_enforcement` after provisioning.\n"]
    pub fn model_enforcement(&self) -> terrars::ListRef<BedrockEnforcedGuardrailConfiguration_ModelEnforcementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_enforcement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `selective_content_guarding` after provisioning.\n"]
    pub fn selective_content_guarding(
        &self,
    ) -> terrars::ListRef<BedrockEnforcedGuardrailConfiguration_SelectiveContentGuardingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selective_content_guarding", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BedrockEnforcedGuardrailConfigurationDynamic {
    model_enforcement: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockEnforcedGuardrailConfiguration_ModelEnforcement>>,
    selective_content_guarding: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockEnforcedGuardrailConfiguration_SelectiveContentGuarding>>,
}

#[derive(Serialize)]
pub struct BedrockEnforcedGuardrailConfiguration_ModelEnforcement {
    excluded_models: terrars::ListField<terrars::PrimField<std::string::String>>,
    included_models: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BedrockEnforcedGuardrailConfiguration_ModelEnforcement { }

impl terrars::ToListMappable for BedrockEnforcedGuardrailConfiguration_ModelEnforcement {
    type O = terrars::BlockAssignable<BedrockEnforcedGuardrailConfiguration_ModelEnforcement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEnforcedGuardrailConfiguration_ModelEnforcement {
    #[doc = ""]
    pub excluded_models: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub included_models: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildBedrockEnforcedGuardrailConfiguration_ModelEnforcement {
    pub fn build(self) -> BedrockEnforcedGuardrailConfiguration_ModelEnforcement {
        BedrockEnforcedGuardrailConfiguration_ModelEnforcement {
            excluded_models: self.excluded_models,
            included_models: self.included_models,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEnforcedGuardrailConfiguration_ModelEnforcementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEnforcedGuardrailConfiguration_ModelEnforcementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEnforcedGuardrailConfiguration_ModelEnforcementRef {
        BedrockEnforcedGuardrailConfiguration_ModelEnforcementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEnforcedGuardrailConfiguration_ModelEnforcementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excluded_models` after provisioning.\n"]
    pub fn excluded_models(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_models", self.base))
    }

    #[doc = "Get a reference to the value of field `included_models` after provisioning.\n"]
    pub fn included_models(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_models", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockEnforcedGuardrailConfiguration_SelectiveContentGuarding {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    messages: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockEnforcedGuardrailConfiguration_SelectiveContentGuarding {
    #[doc = "Set the field `messages`.\n"]
    pub fn set_messages(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.messages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `system`.\n"]
    pub fn set_system(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.system = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockEnforcedGuardrailConfiguration_SelectiveContentGuarding {
    type O = terrars::BlockAssignable<BedrockEnforcedGuardrailConfiguration_SelectiveContentGuarding>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEnforcedGuardrailConfiguration_SelectiveContentGuarding {}

impl BuildBedrockEnforcedGuardrailConfiguration_SelectiveContentGuarding {
    pub fn build(self) -> BedrockEnforcedGuardrailConfiguration_SelectiveContentGuarding {
        BedrockEnforcedGuardrailConfiguration_SelectiveContentGuarding {
            messages: core::default::Default::default(),
            system: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEnforcedGuardrailConfiguration_SelectiveContentGuardingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEnforcedGuardrailConfiguration_SelectiveContentGuardingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEnforcedGuardrailConfiguration_SelectiveContentGuardingRef {
        BedrockEnforcedGuardrailConfiguration_SelectiveContentGuardingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEnforcedGuardrailConfiguration_SelectiveContentGuardingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.messages", self.base))
    }

    #[doc = "Get a reference to the value of field `system` after provisioning.\n"]
    pub fn system(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.system", self.base))
    }
}
