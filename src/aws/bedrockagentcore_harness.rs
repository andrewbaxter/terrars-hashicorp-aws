#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentcoreHarnessData_ {
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
    allowed_tools: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    execution_role_arn: terrars::PrimField<std::string::String>,
    harness_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_iterations: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_tokens: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    truncation: std::option::Option<terrars::ListField<BedrockagentcoreHarness_Truncation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizer_configuration: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_AuthorizerConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_Environment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_artifact: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_EnvironmentArtifact>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_Memory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_Model>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skill: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_Skill>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system_prompt: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_SystemPrompt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockagentcoreHarness_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_Tool>>,
    dynamic: BedrockagentcoreHarnessDynamic,
}

struct BedrockagentcoreHarness_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockagentcoreHarnessData_>,
}

#[derive(Clone)]
pub struct BedrockagentcoreHarness(std::rc::Rc<BedrockagentcoreHarness_>);

impl BedrockagentcoreHarness {
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

    #[doc = "Set the field `allowed_tools`.\n"]
    pub fn set_allowed_tools(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().allowed_tools = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_variables`.\n"]
    pub fn set_environment_variables(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().environment_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_iterations`.\n"]
    pub fn set_max_iterations(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_iterations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_tokens`.\n"]
    pub fn set_max_tokens(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_tokens = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `timeout_seconds`.\n"]
    pub fn set_timeout_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `truncation`.\n"]
    pub fn set_truncation(
        self,
        v: impl std::convert::Into<terrars::ListField<BedrockagentcoreHarness_Truncation>>,
    ) -> Self {
        self.0.data.borrow_mut().truncation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorizer_configuration`.\n"]
    pub fn set_authorizer_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_AuthorizerConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().authorizer_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.authorizer_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Environment>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().environment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.environment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `environment_artifact`.\n"]
    pub fn set_environment_artifact(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_EnvironmentArtifact>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().environment_artifact = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.environment_artifact = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Memory>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().memory = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.memory = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `model`.\n"]
    pub fn set_model(self, v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Model>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().model = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.model = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `skill`.\n"]
    pub fn set_skill(self, v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Skill>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().skill = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.skill = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `system_prompt`.\n"]
    pub fn set_system_prompt(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_SystemPrompt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().system_prompt = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.system_prompt = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockagentcoreHarness_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tool`.\n"]
    pub fn set_tool(self, v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Tool>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tool = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tool = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `allowed_tools` after provisioning.\n"]
    pub fn allowed_tools(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_tools", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_actual` after provisioning.\n"]
    pub fn environment_actual(&self) -> terrars::ListRef<BedrockagentcoreHarness_EnvironmentActualRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment_actual", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `harness_id` after provisioning.\n"]
    pub fn harness_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.harness_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `harness_name` after provisioning.\n"]
    pub fn harness_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.harness_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_iterations` after provisioning.\n"]
    pub fn max_iterations(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_iterations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_tokens` after provisioning.\n"]
    pub fn max_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_tokens", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_actual` after provisioning.\n"]
    pub fn memory_actual(&self) -> terrars::ListRef<BedrockagentcoreHarness_MemoryActualRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_actual", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\n"]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `truncation` after provisioning.\n"]
    pub fn truncation(&self) -> terrars::ListRef<BedrockagentcoreHarness_TruncationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.truncation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_configuration` after provisioning.\n"]
    pub fn authorizer_configuration(&self) -> terrars::ListRef<BedrockagentcoreHarness_AuthorizerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizer_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::ListRef<BedrockagentcoreHarness_EnvironmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_artifact` after provisioning.\n"]
    pub fn environment_artifact(&self) -> terrars::ListRef<BedrockagentcoreHarness_EnvironmentArtifactRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment_artifact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::ListRef<BedrockagentcoreHarness_MemoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> terrars::ListRef<BedrockagentcoreHarness_ModelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skill` after provisioning.\n"]
    pub fn skill(&self) -> terrars::ListRef<BedrockagentcoreHarness_SkillRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.skill", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `system_prompt` after provisioning.\n"]
    pub fn system_prompt(&self) -> terrars::ListRef<BedrockagentcoreHarness_SystemPromptRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.system_prompt", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreHarness_TimeoutsRef {
        BedrockagentcoreHarness_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tool` after provisioning.\n"]
    pub fn tool(&self) -> terrars::ListRef<BedrockagentcoreHarness_ToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool", self.extract_ref()))
    }
}

impl terrars::Referable for BedrockagentcoreHarness {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockagentcoreHarness { }

impl terrars::ToListMappable for BedrockagentcoreHarness {
    type O = terrars::ListRef<BedrockagentcoreHarnessRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockagentcoreHarness_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrockagentcore_harness".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentcoreHarness {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub execution_role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub harness_name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockagentcoreHarness {
        let out = BedrockagentcoreHarness(std::rc::Rc::new(BedrockagentcoreHarness_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockagentcoreHarnessData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allowed_tools: core::default::Default::default(),
                environment_variables: core::default::Default::default(),
                execution_role_arn: self.execution_role_arn,
                harness_name: self.harness_name,
                max_iterations: core::default::Default::default(),
                max_tokens: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                timeout_seconds: core::default::Default::default(),
                truncation: core::default::Default::default(),
                authorizer_configuration: core::default::Default::default(),
                environment: core::default::Default::default(),
                environment_artifact: core::default::Default::default(),
                memory: core::default::Default::default(),
                model: core::default::Default::default(),
                skill: core::default::Default::default(),
                system_prompt: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                tool: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentcoreHarnessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarnessRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockagentcoreHarnessRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_tools` after provisioning.\n"]
    pub fn allowed_tools(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_tools", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_actual` after provisioning.\n"]
    pub fn environment_actual(&self) -> terrars::ListRef<BedrockagentcoreHarness_EnvironmentActualRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment_actual", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `harness_id` after provisioning.\n"]
    pub fn harness_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.harness_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `harness_name` after provisioning.\n"]
    pub fn harness_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.harness_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_iterations` after provisioning.\n"]
    pub fn max_iterations(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_iterations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_tokens` after provisioning.\n"]
    pub fn max_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_tokens", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_actual` after provisioning.\n"]
    pub fn memory_actual(&self) -> terrars::ListRef<BedrockagentcoreHarness_MemoryActualRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_actual", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\n"]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `truncation` after provisioning.\n"]
    pub fn truncation(&self) -> terrars::ListRef<BedrockagentcoreHarness_TruncationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.truncation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_configuration` after provisioning.\n"]
    pub fn authorizer_configuration(&self) -> terrars::ListRef<BedrockagentcoreHarness_AuthorizerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizer_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::ListRef<BedrockagentcoreHarness_EnvironmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_artifact` after provisioning.\n"]
    pub fn environment_artifact(&self) -> terrars::ListRef<BedrockagentcoreHarness_EnvironmentArtifactRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment_artifact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::ListRef<BedrockagentcoreHarness_MemoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> terrars::ListRef<BedrockagentcoreHarness_ModelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skill` after provisioning.\n"]
    pub fn skill(&self) -> terrars::ListRef<BedrockagentcoreHarness_SkillRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.skill", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `system_prompt` after provisioning.\n"]
    pub fn system_prompt(&self) -> terrars::ListRef<BedrockagentcoreHarness_SystemPromptRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.system_prompt", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreHarness_TimeoutsRef {
        BedrockagentcoreHarness_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tool` after provisioning.\n"]
    pub fn tool(&self) -> terrars::ListRef<BedrockagentcoreHarness_ToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreHarnessDynamic {
    authorizer_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreHarness_AuthorizerConfiguration>>,
    environment: std::option::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Environment>>,
    environment_artifact: std::option::Option<terrars::DynamicBlock<BedrockagentcoreHarness_EnvironmentArtifact>>,
    memory: std::option::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Memory>>,
    model: std::option::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Model>>,
    skill: std::option::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Skill>>,
    system_prompt: std::option::Option<terrars::DynamicBlock<BedrockagentcoreHarness_SystemPrompt>>,
    tool: std::option::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Tool>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_point_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mount_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPoint {
    #[doc = "Set the field `access_point_arn`.\n"]
    pub fn set_access_point_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_point_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mount_path`.\n"]
    pub fn set_mount_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mount_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPoint {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPoint,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPoint {}

impl BuildBedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPoint {
    pub fn build(
        self,
    ) -> BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPoint {
        BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPoint {
            access_point_arn: core::default::Default::default(),
            mount_path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPointRef {
        BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_point_arn` after provisioning.\n"]
    pub fn access_point_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_point_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\n"]
    pub fn mount_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_point_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mount_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPoint {
    #[doc = "Set the field `access_point_arn`.\n"]
    pub fn set_access_point_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_point_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mount_path`.\n"]
    pub fn set_mount_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mount_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPoint {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPoint,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPoint {}

impl BuildBedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPoint {
    pub fn build(
        self,
    ) -> BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPoint {
        BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPoint {
            access_point_arn: core::default::Default::default(),
            mount_path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPointRef {
        BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_point_arn` after provisioning.\n"]
    pub fn access_point_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_point_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\n"]
    pub fn mount_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mount_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorage {
    #[doc = "Set the field `mount_path`.\n"]
    pub fn set_mount_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mount_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorage {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorage,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorage {}

impl BuildBedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorage {
    pub fn build(
        self,
    ) -> BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorage {
        BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorage {
            mount_path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorageRef {
        BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\n"]
    pub fn mount_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    efs_access_point: std
    ::option
    ::Option<
        terrars
        ::ListField<
            BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPoint,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_files_access_point: std
    ::option
    ::Option<
        terrars
        ::ListField<
            BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPoint,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_storage: std
    ::option
    ::Option<
        terrars
        ::ListField<
            BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorage,
        >,
    >,
}

impl BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration {
    #[doc = "Set the field `efs_access_point`.\n"]
    pub fn set_efs_access_point(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPoint,
                        >,
                    >,
    ) -> Self {
        self.efs_access_point = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_files_access_point`.\n"]
    pub fn set_s3_files_access_point(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPoint,
                        >,
                    >,
    ) -> Self {
        self.s3_files_access_point = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `session_storage`.\n"]
    pub fn set_session_storage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorage,
                        >,
                    >,
    ) -> Self {
        self.session_storage = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration {}

impl BuildBedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration {
        BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration {
            efs_access_point: core::default::Default::default(),
            s3_files_access_point: core::default::Default::default(),
            session_storage: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfigurationRef {
        BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `efs_access_point` after provisioning.\n"]
    pub fn efs_access_point(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPointRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.efs_access_point", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_files_access_point` after provisioning.\n"]
    pub fn s3_files_access_point(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPointRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_files_access_point", self.base))
    }

    #[doc = "Get a reference to the value of field `session_storage` after provisioning.\n"]
    pub fn session_storage(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorageRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.session_storage", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_LifecycleConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_runtime_session_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_lifetime: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_LifecycleConfiguration {
    #[doc = "Set the field `idle_runtime_session_timeout`.\n"]
    pub fn set_idle_runtime_session_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.idle_runtime_session_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_lifetime`.\n"]
    pub fn set_max_lifetime(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_lifetime = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_LifecycleConfiguration {
    type O =
        terrars
        ::BlockAssignable<BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_LifecycleConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_LifecycleConfiguration {}

impl BuildBedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_LifecycleConfiguration {
    pub fn build(self) -> BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_LifecycleConfiguration {
        BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_LifecycleConfiguration {
            idle_runtime_session_timeout: core::default::Default::default(),
            max_lifetime: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_LifecycleConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_LifecycleConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_LifecycleConfigurationRef {
        BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_LifecycleConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_LifecycleConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_runtime_session_timeout` after provisioning.\n"]
    pub fn idle_runtime_session_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_runtime_session_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `max_lifetime` after provisioning.\n"]
    pub fn max_lifetime(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_lifetime", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_service_s3_endpoint: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnets: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfig {
    #[doc = "Set the field `require_service_s3_endpoint`.\n"]
    pub fn set_require_service_s3_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_service_s3_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnets`.\n"]
    pub fn set_subnets(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfig {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfig {}

impl BuildBedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfig {
    pub fn build(
        self,
    ) -> BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfig {
        BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfig {
            require_service_s3_endpoint: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            subnets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfigRef {
        BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `require_service_s3_endpoint` after provisioning.\n"]
    pub fn require_service_s3_endpoint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_service_s3_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_mode_config: std
    ::option
    ::Option<
        terrars
        ::ListField<
            BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfig,
        >,
    >,
}

impl BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration {
    #[doc = "Set the field `network_mode`.\n"]
    pub fn set_network_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_mode_config`.\n"]
    pub fn set_network_mode_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfig,
                        >,
                    >,
    ) -> Self {
        self.network_mode_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration {
    type O =
        terrars
        ::BlockAssignable<BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration {}

impl BuildBedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration {
    pub fn build(self) -> BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration {
        BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration {
            network_mode: core::default::Default::default(),
            network_mode_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfigurationRef {
        BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_mode` after provisioning.\n"]
    pub fn network_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `network_mode_config` after provisioning.\n"]
    pub fn network_mode_config(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_mode_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_runtime_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_runtime_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_runtime_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filesystem_configuration: std
    ::option
    ::Option<
        terrars
        ::ListField<BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_configuration: std
    ::option
    ::Option<
        terrars::ListField<BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_LifecycleConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std
    ::option
    ::Option<
        terrars::ListField<BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration>,
    >,
}

impl BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment {
    #[doc = "Set the field `agent_runtime_arn`.\n"]
    pub fn set_agent_runtime_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.agent_runtime_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `agent_runtime_id`.\n"]
    pub fn set_agent_runtime_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.agent_runtime_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `agent_runtime_name`.\n"]
    pub fn set_agent_runtime_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.agent_runtime_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filesystem_configuration`.\n"]
    pub fn set_filesystem_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfiguration,
                        >,
                    >,
    ) -> Self {
        self.filesystem_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_configuration`.\n"]
    pub fn set_lifecycle_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_LifecycleConfiguration,
                        >,
                    >,
    ) -> Self {
        self.lifecycle_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfiguration,
                        >,
                    >,
    ) -> Self {
        self.network_configuration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment {}

impl BuildBedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment {
    pub fn build(self) -> BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment {
        BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment {
            agent_runtime_arn: core::default::Default::default(),
            agent_runtime_id: core::default::Default::default(),
            agent_runtime_name: core::default::Default::default(),
            filesystem_configuration: core::default::Default::default(),
            lifecycle_configuration: core::default::Default::default(),
            network_configuration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironmentRef {
        BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_runtime_arn` after provisioning.\n"]
    pub fn agent_runtime_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `agent_runtime_id` after provisioning.\n"]
    pub fn agent_runtime_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_id", self.base))
    }

    #[doc = "Get a reference to the value of field `agent_runtime_name` after provisioning.\n"]
    pub fn agent_runtime_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_name", self.base))
    }

    #[doc = "Get a reference to the value of field `filesystem_configuration` after provisioning.\n"]
    pub fn filesystem_configuration(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_FilesystemConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filesystem_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_configuration` after provisioning.\n"]
    pub fn lifecycle_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_LifecycleConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_EnvironmentActual {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agentcore_runtime_environment: std
    ::option
    ::Option<terrars::ListField<BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment>>,
}

impl BedrockagentcoreHarness_EnvironmentActual {
    #[doc = "Set the field `agentcore_runtime_environment`.\n"]
    pub fn set_agentcore_runtime_environment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironment>>,
    ) -> Self {
        self.agentcore_runtime_environment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_EnvironmentActual {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_EnvironmentActual>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_EnvironmentActual {}

impl BuildBedrockagentcoreHarness_EnvironmentActual {
    pub fn build(self) -> BedrockagentcoreHarness_EnvironmentActual {
        BedrockagentcoreHarness_EnvironmentActual {
            agentcore_runtime_environment: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_EnvironmentActualRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_EnvironmentActualRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreHarness_EnvironmentActualRef {
        BedrockagentcoreHarness_EnvironmentActualRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_EnvironmentActualRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agentcore_runtime_environment` after provisioning.\n"]
    pub fn agentcore_runtime_environment(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreHarness_EnvironmentActual_AgentcoreRuntimeEnvironmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agentcore_runtime_environment", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration_RetrievalConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    map_block_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relevance_score: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strategy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_k: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration_RetrievalConfig {
    #[doc = "Set the field `map_block_key`.\n"]
    pub fn set_map_block_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.map_block_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `relevance_score`.\n"]
    pub fn set_relevance_score(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.relevance_score = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `strategy_id`.\n"]
    pub fn set_strategy_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.strategy_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `top_k`.\n"]
    pub fn set_top_k(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.top_k = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration_RetrievalConfig {
    type O =
        terrars::BlockAssignable<BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration_RetrievalConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration_RetrievalConfig {}

impl BuildBedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration_RetrievalConfig {
    pub fn build(self) -> BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration_RetrievalConfig {
        BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration_RetrievalConfig {
            map_block_key: core::default::Default::default(),
            relevance_score: core::default::Default::default(),
            strategy_id: core::default::Default::default(),
            top_k: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration_RetrievalConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration_RetrievalConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration_RetrievalConfigRef {
        BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration_RetrievalConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration_RetrievalConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `map_block_key` after provisioning.\n"]
    pub fn map_block_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.map_block_key", self.base))
    }

    #[doc = "Get a reference to the value of field `relevance_score` after provisioning.\n"]
    pub fn relevance_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.relevance_score", self.base))
    }

    #[doc = "Get a reference to the value of field `strategy_id` after provisioning.\n"]
    pub fn strategy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strategy_id", self.base))
    }

    #[doc = "Get a reference to the value of field `top_k` after provisioning.\n"]
    pub fn top_k(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.top_k", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actor_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    messages_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retrieval_config: std
    ::option
    ::Option<terrars::ListField<BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration_RetrievalConfig>>,
}

impl BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration {
    #[doc = "Set the field `actor_id`.\n"]
    pub fn set_actor_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.actor_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `messages_count`.\n"]
    pub fn set_messages_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.messages_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retrieval_config`.\n"]
    pub fn set_retrieval_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration_RetrievalConfig>,
                    >,
    ) -> Self {
        self.retrieval_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration {}

impl BuildBedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration {
    pub fn build(self) -> BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration {
        BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration {
            actor_id: core::default::Default::default(),
            arn: core::default::Default::default(),
            messages_count: core::default::Default::default(),
            retrieval_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfigurationRef {
        BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `actor_id` after provisioning.\n"]
    pub fn actor_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.actor_id", self.base))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `messages_count` after provisioning.\n"]
    pub fn messages_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.messages_count", self.base))
    }

    #[doc = "Get a reference to the value of field `retrieval_config` after provisioning.\n"]
    pub fn retrieval_config(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration_RetrievalConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retrieval_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Memory_Disabled {}

impl BedrockagentcoreHarness_Memory_Disabled { }

impl terrars::ToListMappable for BedrockagentcoreHarness_Memory_Disabled {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Memory_Disabled>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Memory_Disabled {}

impl BuildBedrockagentcoreHarness_Memory_Disabled {
    pub fn build(self) -> BedrockagentcoreHarness_Memory_Disabled {
        BedrockagentcoreHarness_Memory_Disabled {}
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Memory_DisabledRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Memory_DisabledRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreHarness_Memory_DisabledRef {
        BedrockagentcoreHarness_Memory_DisabledRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Memory_DisabledRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_MemoryActual_ManagedMemoryConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_expiry_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strategies: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl BedrockagentcoreHarness_MemoryActual_ManagedMemoryConfiguration {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_key_arn`.\n"]
    pub fn set_encryption_key_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.encryption_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event_expiry_duration`.\n"]
    pub fn set_event_expiry_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.event_expiry_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `strategies`.\n"]
    pub fn set_strategies(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.strategies = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_MemoryActual_ManagedMemoryConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_MemoryActual_ManagedMemoryConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_MemoryActual_ManagedMemoryConfiguration {}

impl BuildBedrockagentcoreHarness_MemoryActual_ManagedMemoryConfiguration {
    pub fn build(self) -> BedrockagentcoreHarness_MemoryActual_ManagedMemoryConfiguration {
        BedrockagentcoreHarness_MemoryActual_ManagedMemoryConfiguration {
            arn: core::default::Default::default(),
            encryption_key_arn: core::default::Default::default(),
            event_expiry_duration: core::default::Default::default(),
            strategies: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_MemoryActual_ManagedMemoryConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_MemoryActual_ManagedMemoryConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_MemoryActual_ManagedMemoryConfigurationRef {
        BedrockagentcoreHarness_MemoryActual_ManagedMemoryConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_MemoryActual_ManagedMemoryConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_key_arn` after provisioning.\n"]
    pub fn encryption_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `event_expiry_duration` after provisioning.\n"]
    pub fn event_expiry_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_expiry_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `strategies` after provisioning.\n"]
    pub fn strategies(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.strategies", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_MemoryActual {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agentcore_memory_configuration: std
    ::option
    ::Option<terrars::ListField<BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::ListField<BedrockagentcoreHarness_Memory_Disabled>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_memory_configuration: std
    ::option
    ::Option<terrars::ListField<BedrockagentcoreHarness_MemoryActual_ManagedMemoryConfiguration>>,
}

impl BedrockagentcoreHarness_MemoryActual {
    #[doc = "Set the field `agentcore_memory_configuration`.\n"]
    pub fn set_agentcore_memory_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfiguration>>,
    ) -> Self {
        self.agentcore_memory_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(
        mut self,
        v: impl std::convert::Into<terrars::ListField<BedrockagentcoreHarness_Memory_Disabled>>,
    ) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `managed_memory_configuration`.\n"]
    pub fn set_managed_memory_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<BedrockagentcoreHarness_MemoryActual_ManagedMemoryConfiguration>>,
    ) -> Self {
        self.managed_memory_configuration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_MemoryActual {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_MemoryActual>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_MemoryActual {}

impl BuildBedrockagentcoreHarness_MemoryActual {
    pub fn build(self) -> BedrockagentcoreHarness_MemoryActual {
        BedrockagentcoreHarness_MemoryActual {
            agentcore_memory_configuration: core::default::Default::default(),
            disabled: core::default::Default::default(),
            managed_memory_configuration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_MemoryActualRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_MemoryActualRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreHarness_MemoryActualRef {
        BedrockagentcoreHarness_MemoryActualRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_MemoryActualRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agentcore_memory_configuration` after provisioning.\n"]
    pub fn agentcore_memory_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreHarness_MemoryActual_AgentcoreMemoryConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agentcore_memory_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::ListRef<BedrockagentcoreHarness_Memory_DisabledRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_memory_configuration` after provisioning.\n"]
    pub fn managed_memory_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreHarness_MemoryActual_ManagedMemoryConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_memory_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Truncation_Config_SlidingWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    messages_count: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentcoreHarness_Truncation_Config_SlidingWindow {
    #[doc = "Set the field `messages_count`.\n"]
    pub fn set_messages_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.messages_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Truncation_Config_SlidingWindow {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Truncation_Config_SlidingWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Truncation_Config_SlidingWindow {}

impl BuildBedrockagentcoreHarness_Truncation_Config_SlidingWindow {
    pub fn build(self) -> BedrockagentcoreHarness_Truncation_Config_SlidingWindow {
        BedrockagentcoreHarness_Truncation_Config_SlidingWindow { messages_count: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Truncation_Config_SlidingWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Truncation_Config_SlidingWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Truncation_Config_SlidingWindowRef {
        BedrockagentcoreHarness_Truncation_Config_SlidingWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Truncation_Config_SlidingWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `messages_count` after provisioning.\n"]
    pub fn messages_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.messages_count", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Truncation_Config_Summarization {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_recent_messages: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    summarization_system_prompt: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    summary_ratio: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentcoreHarness_Truncation_Config_Summarization {
    #[doc = "Set the field `preserve_recent_messages`.\n"]
    pub fn set_preserve_recent_messages(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.preserve_recent_messages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `summarization_system_prompt`.\n"]
    pub fn set_summarization_system_prompt(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.summarization_system_prompt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `summary_ratio`.\n"]
    pub fn set_summary_ratio(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.summary_ratio = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Truncation_Config_Summarization {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Truncation_Config_Summarization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Truncation_Config_Summarization {}

impl BuildBedrockagentcoreHarness_Truncation_Config_Summarization {
    pub fn build(self) -> BedrockagentcoreHarness_Truncation_Config_Summarization {
        BedrockagentcoreHarness_Truncation_Config_Summarization {
            preserve_recent_messages: core::default::Default::default(),
            summarization_system_prompt: core::default::Default::default(),
            summary_ratio: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Truncation_Config_SummarizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Truncation_Config_SummarizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Truncation_Config_SummarizationRef {
        BedrockagentcoreHarness_Truncation_Config_SummarizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Truncation_Config_SummarizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `preserve_recent_messages` after provisioning.\n"]
    pub fn preserve_recent_messages(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_recent_messages", self.base))
    }

    #[doc = "Get a reference to the value of field `summarization_system_prompt` after provisioning.\n"]
    pub fn summarization_system_prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.summarization_system_prompt", self.base))
    }

    #[doc = "Get a reference to the value of field `summary_ratio` after provisioning.\n"]
    pub fn summary_ratio(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.summary_ratio", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Truncation_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sliding_window: std::option::Option<terrars::ListField<BedrockagentcoreHarness_Truncation_Config_SlidingWindow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    summarization: std::option::Option<terrars::ListField<BedrockagentcoreHarness_Truncation_Config_Summarization>>,
}

impl BedrockagentcoreHarness_Truncation_Config {
    #[doc = "Set the field `sliding_window`.\n"]
    pub fn set_sliding_window(
        mut self,
        v: impl std::convert::Into<terrars::ListField<BedrockagentcoreHarness_Truncation_Config_SlidingWindow>>,
    ) -> Self {
        self.sliding_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `summarization`.\n"]
    pub fn set_summarization(
        mut self,
        v: impl std::convert::Into<terrars::ListField<BedrockagentcoreHarness_Truncation_Config_Summarization>>,
    ) -> Self {
        self.summarization = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Truncation_Config {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Truncation_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Truncation_Config {}

impl BuildBedrockagentcoreHarness_Truncation_Config {
    pub fn build(self) -> BedrockagentcoreHarness_Truncation_Config {
        BedrockagentcoreHarness_Truncation_Config {
            sliding_window: core::default::Default::default(),
            summarization: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Truncation_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Truncation_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreHarness_Truncation_ConfigRef {
        BedrockagentcoreHarness_Truncation_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Truncation_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sliding_window` after provisioning.\n"]
    pub fn sliding_window(&self) -> terrars::ListRef<BedrockagentcoreHarness_Truncation_Config_SlidingWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sliding_window", self.base))
    }

    #[doc = "Get a reference to the value of field `summarization` after provisioning.\n"]
    pub fn summarization(&self) -> terrars::ListRef<BedrockagentcoreHarness_Truncation_Config_SummarizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.summarization", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Truncation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<terrars::ListField<BedrockagentcoreHarness_Truncation_Config>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strategy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreHarness_Truncation {
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<BedrockagentcoreHarness_Truncation_Config>>,
    ) -> Self {
        self.config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `strategy`.\n"]
    pub fn set_strategy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.strategy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Truncation {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Truncation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Truncation {}

impl BuildBedrockagentcoreHarness_Truncation {
    pub fn build(self) -> BedrockagentcoreHarness_Truncation {
        BedrockagentcoreHarness_Truncation {
            config: core::default::Default::default(),
            strategy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_TruncationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_TruncationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreHarness_TruncationRef {
        BedrockagentcoreHarness_TruncationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_TruncationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<BedrockagentcoreHarness_Truncation_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }

    #[doc = "Get a reference to the value of field `strategy` after provisioning.\n"]
    pub fn strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
    arn: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {

}

impl terrars
::ToListMappable for BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
    pub fn build(
        self,
    ) -> BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
            arn: self.arn,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef {
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationDynamic {
    hosting_environment: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_identities: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosting_environment: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment,
        >,
    >,
    dynamic: BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationDynamic,
}

impl BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
    #[doc = "Set the field `workload_identities`.\n"]
    pub fn set_workload_identities(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.workload_identities = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hosting_environment`.\n"]
    pub fn set_hosting_environment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hosting_environment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hosting_environment = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {}

impl BuildBedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
            workload_identities: core::default::Default::default(),
            hosting_environment: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef {
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `workload_identities` after provisioning.\n"]
    pub fn workload_identities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_identities", self.base))
    }

    #[doc = "Get a reference to the value of field `hosting_environment` after provisioning.\n"]
    pub fn hosting_environment(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hosting_environment", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_value_string: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_value_string_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
    #[doc = "Set the field `match_value_string`.\n"]
    pub fn set_match_value_string(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.match_value_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_value_string_list`.\n"]
    pub fn set_match_value_string_list(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.match_value_string_list = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {}

impl BuildBedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
    pub fn build(
        self,
    ) -> BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
            match_value_string: core::default::Default::default(),
            match_value_string_list: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_value_string` after provisioning.\n"]
    pub fn match_value_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match_value_string", self.base))
    }

    #[doc = "Get a reference to the value of field `match_value_string_list` after provisioning.\n"]
    pub fn match_value_string_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.match_value_string_list", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueDynamic {
    claim_match_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    claim_match_operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    claim_match_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue,
        >,
    >,
    dynamic: BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueDynamic,
}

impl BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    #[doc = "Set the field `claim_match_value`.\n"]
    pub fn set_claim_match_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.claim_match_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.claim_match_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    #[doc = ""]
    pub claim_match_operator: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    pub fn build(
        self,
    ) -> BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
            claim_match_operator: self.claim_match_operator,
            claim_match_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `claim_match_operator` after provisioning.\n"]
    pub fn claim_match_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.claim_match_operator", self.base))
    }

    #[doc = "Get a reference to the value of field `claim_match_value` after provisioning.\n"]
    pub fn claim_match_value(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.claim_match_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimDynamic {
    authorizing_claim_match_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    inbound_token_claim_name: terrars::PrimField<std::string::String>,
    inbound_token_claim_value_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizing_claim_match_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue,
        >,
    >,
    dynamic: BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimDynamic,
}

impl BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    #[doc = "Set the field `authorizing_claim_match_value`.\n"]
    pub fn set_authorizing_claim_match_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authorizing_claim_match_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authorizing_claim_match_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    type O =
        terrars::BlockAssignable<BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    #[doc = ""]
    pub inbound_token_claim_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub inbound_token_claim_value_type: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    pub fn build(self) -> BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
            inbound_token_claim_name: self.inbound_token_claim_name,
            inbound_token_claim_value_type: self.inbound_token_claim_value_type,
            authorizing_claim_match_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `inbound_token_claim_name` after provisioning.\n"]
    pub fn inbound_token_claim_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inbound_token_claim_name", self.base))
    }

    #[doc = "Get a reference to the value of field `inbound_token_claim_value_type` after provisioning.\n"]
    pub fn inbound_token_claim_value_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inbound_token_claim_value_type", self.base))
    }

    #[doc = "Get a reference to the value of field `authorizing_claim_match_value` after provisioning.\n"]
    pub fn authorizing_claim_match_value(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizing_claim_match_value", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    endpoint_ip_address_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    vpc_identifier: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    #[doc = "Set the field `routing_domain`.\n"]
    pub fn set_routing_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.routing_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    #[doc = ""]
    pub endpoint_ip_address_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub vpc_identifier: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    pub fn build(
        self,
    ) -> BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
            endpoint_ip_address_type: self.endpoint_ip_address_type,
            routing_domain: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
            tags: core::default::Default::default(),
            vpc_identifier: self.vpc_identifier,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_ip_address_type` after provisioning.\n"]
    pub fn endpoint_ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_ip_address_type", self.base))
    }

    #[doc = "Get a reference to the value of field `routing_domain` after provisioning.\n"]
    pub fn routing_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_domain", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_identifier` after provisioning.\n"]
    pub fn vpc_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_configuration_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
    #[doc = "Set the field `resource_configuration_identifier`.\n"]
    pub fn set_resource_configuration_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.resource_configuration_identifier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {}

impl BuildBedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
    pub fn build(
        self,
    ) -> BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
            resource_configuration_identifier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_configuration_identifier` after provisioning.\n"]
    pub fn resource_configuration_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_configuration_identifier", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointDynamic {
    managed_vpc_resource: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource,
        >,
    >,
    self_managed_lattice_resource: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_vpc_resource: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_managed_lattice_resource: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource,
        >,
    >,
    dynamic: BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointDynamic,
}

impl BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
    #[doc = "Set the field `managed_vpc_resource`.\n"]
    pub fn set_managed_vpc_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_vpc_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_vpc_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `self_managed_lattice_resource`.\n"]
    pub fn set_self_managed_lattice_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.self_managed_lattice_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.self_managed_lattice_resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
    type O =
        terrars
        ::BlockAssignable<BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {}

impl BuildBedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
    pub fn build(self) -> BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
            managed_vpc_resource: core::default::Default::default(),
            self_managed_lattice_resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `managed_vpc_resource` after provisioning.\n"]
    pub fn managed_vpc_resource(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_vpc_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `self_managed_lattice_resource` after provisioning.\n"]
    pub fn self_managed_lattice_resource(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_managed_lattice_resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesDynamic {
    private_endpoint: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
    domain: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint>>,
    dynamic: BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesDynamic,
}

impl BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
    #[doc = "Set the field `private_endpoint`.\n"]
    pub fn set_private_endpoint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_endpoint = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
    #[doc = ""]
    pub domain: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
    pub fn build(
        self,
    ) -> BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
            domain: self.domain,
            private_endpoint: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef {
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `private_endpoint` after provisioning.\n"]
    pub fn private_endpoint(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizerDynamic {
    allowed_workload_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration,
        >,
    >,
    custom_claim: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim>>,
    private_endpoint: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint>,
    >,
    private_endpoint_overrides: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_audience: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_clients: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_scopes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    discovery_url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_workload_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_claim: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint_overrides: std
    ::option
    ::Option<
        std::vec::Vec<BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides>,
    >,
    dynamic: BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizerDynamic,
}

impl BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer {
    #[doc = "Set the field `allowed_audience`.\n"]
    pub fn set_allowed_audience(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_audience = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_clients`.\n"]
    pub fn set_allowed_clients(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_clients = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_scopes`.\n"]
    pub fn set_allowed_scopes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_workload_configuration`.\n"]
    pub fn set_allowed_workload_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allowed_workload_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_workload_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_claim`.\n"]
    pub fn set_custom_claim(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_claim = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_claim = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_endpoint`.\n"]
    pub fn set_private_endpoint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_endpoint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_endpoint_overrides`.\n"]
    pub fn set_private_endpoint_overrides(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_endpoint_overrides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_endpoint_overrides = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer {
    #[doc = ""]
    pub discovery_url: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer {
    pub fn build(self) -> BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer {
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer {
            allowed_audience: core::default::Default::default(),
            allowed_clients: core::default::Default::default(),
            allowed_scopes: core::default::Default::default(),
            discovery_url: self.discovery_url,
            allowed_workload_configuration: core::default::Default::default(),
            custom_claim: core::default::Default::default(),
            private_endpoint: core::default::Default::default(),
            private_endpoint_overrides: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizerRef {
        BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_audience` after provisioning.\n"]
    pub fn allowed_audience(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_audience", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_clients` after provisioning.\n"]
    pub fn allowed_clients(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_clients", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_scopes` after provisioning.\n"]
    pub fn allowed_scopes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_scopes", self.base))
    }

    #[doc = "Get a reference to the value of field `discovery_url` after provisioning.\n"]
    pub fn discovery_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discovery_url", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_workload_configuration` after provisioning.\n"]
    pub fn allowed_workload_configuration(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_workload_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `private_endpoint` after provisioning.\n"]
    pub fn private_endpoint(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `private_endpoint_overrides` after provisioning.\n"]
    pub fn private_endpoint_overrides(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoint_overrides", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreHarness_AuthorizerConfigurationDynamic {
    custom_jwt_authorizer: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_AuthorizerConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_jwt_authorizer: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer>>,
    dynamic: BedrockagentcoreHarness_AuthorizerConfigurationDynamic,
}

impl BedrockagentcoreHarness_AuthorizerConfiguration {
    #[doc = "Set the field `custom_jwt_authorizer`.\n"]
    pub fn set_custom_jwt_authorizer(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizer>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_jwt_authorizer = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_jwt_authorizer = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_AuthorizerConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_AuthorizerConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_AuthorizerConfiguration {}

impl BuildBedrockagentcoreHarness_AuthorizerConfiguration {
    pub fn build(self) -> BedrockagentcoreHarness_AuthorizerConfiguration {
        BedrockagentcoreHarness_AuthorizerConfiguration {
            custom_jwt_authorizer: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_AuthorizerConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_AuthorizerConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_AuthorizerConfigurationRef {
        BedrockagentcoreHarness_AuthorizerConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_AuthorizerConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_jwt_authorizer` after provisioning.\n"]
    pub fn custom_jwt_authorizer(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreHarness_AuthorizerConfiguration_CustomJwtAuthorizerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_jwt_authorizer", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPoint {
    access_point_arn: terrars::PrimField<std::string::String>,
    mount_path: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPoint { }

impl terrars
::ToListMappable for BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPoint {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPoint,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPoint {
    #[doc = ""]
    pub access_point_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub mount_path: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPoint {
    pub fn build(
        self,
    ) -> BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPoint {
        BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPoint {
            access_point_arn: self.access_point_arn,
            mount_path: self.mount_path,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPointRef {
        BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_point_arn` after provisioning.\n"]
    pub fn access_point_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_point_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\n"]
    pub fn mount_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPoint {
    access_point_arn: terrars::PrimField<std::string::String>,
    mount_path: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPoint { }

impl terrars
::ToListMappable for BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPoint {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPoint,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPoint {
    #[doc = ""]
    pub access_point_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub mount_path: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPoint {
    pub fn build(
        self,
    ) -> BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPoint {
        BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPoint {
            access_point_arn: self.access_point_arn,
            mount_path: self.mount_path,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPointRef {
        BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_point_arn` after provisioning.\n"]
    pub fn access_point_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_point_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\n"]
    pub fn mount_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorage {
    mount_path: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorage { }

impl terrars
::ToListMappable for BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorage {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorage,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorage {
    #[doc = ""]
    pub mount_path: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorage {
    pub fn build(
        self,
    ) -> BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorage {
        BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorage {
            mount_path: self.mount_path,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorageRef {
        BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\n"]
    pub fn mount_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfigurationDynamic {
    efs_access_point: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPoint,
        >,
    >,
    s3_files_access_point: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPoint,
        >,
    >,
    session_storage: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorage,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    efs_access_point: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPoint>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_files_access_point: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPoint,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_storage: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorage>,
    >,
    dynamic: BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfigurationDynamic,
}

impl BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration {
    #[doc = "Set the field `efs_access_point`.\n"]
    pub fn set_efs_access_point(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPoint,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.efs_access_point = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.efs_access_point = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_files_access_point`.\n"]
    pub fn set_s3_files_access_point(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPoint,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_files_access_point = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_files_access_point = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `session_storage`.\n"]
    pub fn set_session_storage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorage,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.session_storage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.session_storage = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration {
    type O =
        terrars
        ::BlockAssignable<BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration {}

impl BuildBedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration {
    pub fn build(self) -> BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration {
        BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration {
            efs_access_point: core::default::Default::default(),
            s3_files_access_point: core::default::Default::default(),
            session_storage: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfigurationRef {
        BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `efs_access_point` after provisioning.\n"]
    pub fn efs_access_point(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_EfsAccessPointRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.efs_access_point", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_files_access_point` after provisioning.\n"]
    pub fn s3_files_access_point(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_S3FilesAccessPointRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_files_access_point", self.base))
    }

    #[doc = "Get a reference to the value of field `session_storage` after provisioning.\n"]
    pub fn session_storage(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration_SessionStorageRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.session_storage", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfig {
    security_groups: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfig { }

impl terrars
::ToListMappable for BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfig {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfig {
    #[doc = ""]
    pub security_groups: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildBedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfig {
    pub fn build(
        self,
    ) -> BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfig {
        BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfig {
            security_groups: self.security_groups,
            subnets: self.subnets,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfigRef {
        BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `require_service_s3_endpoint` after provisioning.\n"]
    pub fn require_service_s3_endpoint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_service_s3_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfigurationDynamic {
    network_mode_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration {
    network_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_mode_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfig>,
    >,
    dynamic: BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfigurationDynamic,
}

impl BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration {
    #[doc = "Set the field `network_mode_config`.\n"]
    pub fn set_network_mode_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_mode_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_mode_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration {
    type O =
        terrars
        ::BlockAssignable<BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration {
    #[doc = ""]
    pub network_mode: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration {
    pub fn build(self) -> BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration {
        BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration {
            network_mode: self.network_mode,
            network_mode_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfigurationRef {
        BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_mode` after provisioning.\n"]
    pub fn network_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `network_mode_config` after provisioning.\n"]
    pub fn network_mode_config(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration_NetworkModeConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_mode_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironmentDynamic {
    filesystem_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration>,
    >,
    network_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_configuration: std
    ::option
    ::Option<
        terrars::ListField<BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_LifecycleConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filesystem_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration>>,
    dynamic: BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironmentDynamic,
}

impl BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment {
    #[doc = "Set the field `lifecycle_configuration`.\n"]
    pub fn set_lifecycle_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_LifecycleConfiguration,
                        >,
                    >,
    ) -> Self {
        self.lifecycle_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filesystem_configuration`.\n"]
    pub fn set_filesystem_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filesystem_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filesystem_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment {}

impl BuildBedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment {
    pub fn build(self) -> BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment {
        BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment {
            lifecycle_configuration: core::default::Default::default(),
            filesystem_configuration: core::default::Default::default(),
            network_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironmentRef {
        BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_runtime_arn` after provisioning.\n"]
    pub fn agent_runtime_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `agent_runtime_id` after provisioning.\n"]
    pub fn agent_runtime_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_id", self.base))
    }

    #[doc = "Get a reference to the value of field `agent_runtime_name` after provisioning.\n"]
    pub fn agent_runtime_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_name", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_configuration` after provisioning.\n"]
    pub fn lifecycle_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_LifecycleConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `filesystem_configuration` after provisioning.\n"]
    pub fn filesystem_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_FilesystemConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filesystem_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreHarness_EnvironmentDynamic {
    agentcore_runtime_environment: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Environment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agentcore_runtime_environment: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment>>,
    dynamic: BedrockagentcoreHarness_EnvironmentDynamic,
}

impl BedrockagentcoreHarness_Environment {
    #[doc = "Set the field `agentcore_runtime_environment`.\n"]
    pub fn set_agentcore_runtime_environment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironment>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.agentcore_runtime_environment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.agentcore_runtime_environment = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Environment {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Environment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Environment {}

impl BuildBedrockagentcoreHarness_Environment {
    pub fn build(self) -> BedrockagentcoreHarness_Environment {
        BedrockagentcoreHarness_Environment {
            agentcore_runtime_environment: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_EnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_EnvironmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreHarness_EnvironmentRef {
        BedrockagentcoreHarness_EnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_EnvironmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agentcore_runtime_environment` after provisioning.\n"]
    pub fn agentcore_runtime_environment(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreHarness_Environment_AgentcoreRuntimeEnvironmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agentcore_runtime_environment", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_EnvironmentArtifact_ContainerConfiguration {
    container_uri: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreHarness_EnvironmentArtifact_ContainerConfiguration { }

impl terrars::ToListMappable for BedrockagentcoreHarness_EnvironmentArtifact_ContainerConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_EnvironmentArtifact_ContainerConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_EnvironmentArtifact_ContainerConfiguration {
    #[doc = ""]
    pub container_uri: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_EnvironmentArtifact_ContainerConfiguration {
    pub fn build(self) -> BedrockagentcoreHarness_EnvironmentArtifact_ContainerConfiguration {
        BedrockagentcoreHarness_EnvironmentArtifact_ContainerConfiguration { container_uri: self.container_uri }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_EnvironmentArtifact_ContainerConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_EnvironmentArtifact_ContainerConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_EnvironmentArtifact_ContainerConfigurationRef {
        BedrockagentcoreHarness_EnvironmentArtifact_ContainerConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_EnvironmentArtifact_ContainerConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_uri` after provisioning.\n"]
    pub fn container_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreHarness_EnvironmentArtifactDynamic {
    container_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreHarness_EnvironmentArtifact_ContainerConfiguration>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_EnvironmentArtifact {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreHarness_EnvironmentArtifact_ContainerConfiguration>>,
    dynamic: BedrockagentcoreHarness_EnvironmentArtifactDynamic,
}

impl BedrockagentcoreHarness_EnvironmentArtifact {
    #[doc = "Set the field `container_configuration`.\n"]
    pub fn set_container_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreHarness_EnvironmentArtifact_ContainerConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_EnvironmentArtifact {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_EnvironmentArtifact>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_EnvironmentArtifact {}

impl BuildBedrockagentcoreHarness_EnvironmentArtifact {
    pub fn build(self) -> BedrockagentcoreHarness_EnvironmentArtifact {
        BedrockagentcoreHarness_EnvironmentArtifact {
            container_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_EnvironmentArtifactRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_EnvironmentArtifactRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_EnvironmentArtifactRef {
        BedrockagentcoreHarness_EnvironmentArtifactRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_EnvironmentArtifactRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_configuration` after provisioning.\n"]
    pub fn container_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreHarness_EnvironmentArtifact_ContainerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration_RetrievalConfig {
    map_block_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relevance_score: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strategy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_k: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration_RetrievalConfig {
    #[doc = "Set the field `relevance_score`.\n"]
    pub fn set_relevance_score(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.relevance_score = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `strategy_id`.\n"]
    pub fn set_strategy_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.strategy_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `top_k`.\n"]
    pub fn set_top_k(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.top_k = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration_RetrievalConfig {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration_RetrievalConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration_RetrievalConfig {
    #[doc = ""]
    pub map_block_key: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration_RetrievalConfig {
    pub fn build(self) -> BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration_RetrievalConfig {
        BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration_RetrievalConfig {
            map_block_key: self.map_block_key,
            relevance_score: core::default::Default::default(),
            strategy_id: core::default::Default::default(),
            top_k: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration_RetrievalConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration_RetrievalConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration_RetrievalConfigRef {
        BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration_RetrievalConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration_RetrievalConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `map_block_key` after provisioning.\n"]
    pub fn map_block_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.map_block_key", self.base))
    }

    #[doc = "Get a reference to the value of field `relevance_score` after provisioning.\n"]
    pub fn relevance_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.relevance_score", self.base))
    }

    #[doc = "Get a reference to the value of field `strategy_id` after provisioning.\n"]
    pub fn strategy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strategy_id", self.base))
    }

    #[doc = "Get a reference to the value of field `top_k` after provisioning.\n"]
    pub fn top_k(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.top_k", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreHarness_Memory_AgentcoreMemoryConfigurationDynamic {
    retrieval_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration_RetrievalConfig>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actor_id: std::option::Option<terrars::PrimField<std::string::String>>,
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    messages_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retrieval_config: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration_RetrievalConfig>>,
    dynamic: BedrockagentcoreHarness_Memory_AgentcoreMemoryConfigurationDynamic,
}

impl BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration {
    #[doc = "Set the field `actor_id`.\n"]
    pub fn set_actor_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.actor_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `messages_count`.\n"]
    pub fn set_messages_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.messages_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retrieval_config`.\n"]
    pub fn set_retrieval_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration_RetrievalConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retrieval_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retrieval_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration {
    pub fn build(self) -> BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration {
        BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration {
            actor_id: core::default::Default::default(),
            arn: self.arn,
            messages_count: core::default::Default::default(),
            retrieval_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Memory_AgentcoreMemoryConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Memory_AgentcoreMemoryConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Memory_AgentcoreMemoryConfigurationRef {
        BedrockagentcoreHarness_Memory_AgentcoreMemoryConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Memory_AgentcoreMemoryConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `actor_id` after provisioning.\n"]
    pub fn actor_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.actor_id", self.base))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `messages_count` after provisioning.\n"]
    pub fn messages_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.messages_count", self.base))
    }

    #[doc = "Get a reference to the value of field `retrieval_config` after provisioning.\n"]
    pub fn retrieval_config(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration_RetrievalConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retrieval_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Memory_ManagedMemoryConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_expiry_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strategies: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl BedrockagentcoreHarness_Memory_ManagedMemoryConfiguration {
    #[doc = "Set the field `encryption_key_arn`.\n"]
    pub fn set_encryption_key_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.encryption_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event_expiry_duration`.\n"]
    pub fn set_event_expiry_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.event_expiry_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `strategies`.\n"]
    pub fn set_strategies(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.strategies = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Memory_ManagedMemoryConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Memory_ManagedMemoryConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Memory_ManagedMemoryConfiguration {}

impl BuildBedrockagentcoreHarness_Memory_ManagedMemoryConfiguration {
    pub fn build(self) -> BedrockagentcoreHarness_Memory_ManagedMemoryConfiguration {
        BedrockagentcoreHarness_Memory_ManagedMemoryConfiguration {
            encryption_key_arn: core::default::Default::default(),
            event_expiry_duration: core::default::Default::default(),
            strategies: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Memory_ManagedMemoryConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Memory_ManagedMemoryConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Memory_ManagedMemoryConfigurationRef {
        BedrockagentcoreHarness_Memory_ManagedMemoryConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Memory_ManagedMemoryConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_key_arn` after provisioning.\n"]
    pub fn encryption_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `event_expiry_duration` after provisioning.\n"]
    pub fn event_expiry_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_expiry_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `strategies` after provisioning.\n"]
    pub fn strategies(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.strategies", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreHarness_MemoryDynamic {
    agentcore_memory_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration>>,
    disabled: std::option::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Memory_Disabled>>,
    managed_memory_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Memory_ManagedMemoryConfiguration>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Memory {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agentcore_memory_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_Memory_Disabled>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_memory_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreHarness_Memory_ManagedMemoryConfiguration>>,
    dynamic: BedrockagentcoreHarness_MemoryDynamic,
}

impl BedrockagentcoreHarness_Memory {
    #[doc = "Set the field `agentcore_memory_configuration`.\n"]
    pub fn set_agentcore_memory_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Memory_AgentcoreMemoryConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.agentcore_memory_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.agentcore_memory_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Memory_Disabled>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disabled = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disabled = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_memory_configuration`.\n"]
    pub fn set_managed_memory_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Memory_ManagedMemoryConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_memory_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_memory_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Memory {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Memory>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Memory {}

impl BuildBedrockagentcoreHarness_Memory {
    pub fn build(self) -> BedrockagentcoreHarness_Memory {
        BedrockagentcoreHarness_Memory {
            agentcore_memory_configuration: core::default::Default::default(),
            disabled: core::default::Default::default(),
            managed_memory_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_MemoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_MemoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreHarness_MemoryRef {
        BedrockagentcoreHarness_MemoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_MemoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agentcore_memory_configuration` after provisioning.\n"]
    pub fn agentcore_memory_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreHarness_Memory_AgentcoreMemoryConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agentcore_memory_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::ListRef<BedrockagentcoreHarness_Memory_DisabledRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_memory_configuration` after provisioning.\n"]
    pub fn managed_memory_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreHarness_Memory_ManagedMemoryConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_memory_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Model_BedrockModelConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_params: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_tokens: std::option::Option<terrars::PrimField<f64>>,
    model_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temperature: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_p: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentcoreHarness_Model_BedrockModelConfig {
    #[doc = "Set the field `additional_params`.\n"]
    pub fn set_additional_params(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.additional_params = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_format`.\n"]
    pub fn set_api_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_tokens`.\n"]
    pub fn set_max_tokens(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_tokens = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for BedrockagentcoreHarness_Model_BedrockModelConfig {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Model_BedrockModelConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Model_BedrockModelConfig {
    #[doc = ""]
    pub model_id: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_Model_BedrockModelConfig {
    pub fn build(self) -> BedrockagentcoreHarness_Model_BedrockModelConfig {
        BedrockagentcoreHarness_Model_BedrockModelConfig {
            additional_params: core::default::Default::default(),
            api_format: core::default::Default::default(),
            max_tokens: core::default::Default::default(),
            model_id: self.model_id,
            temperature: core::default::Default::default(),
            top_p: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Model_BedrockModelConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Model_BedrockModelConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Model_BedrockModelConfigRef {
        BedrockagentcoreHarness_Model_BedrockModelConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Model_BedrockModelConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_params` after provisioning.\n"]
    pub fn additional_params(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_params", self.base))
    }

    #[doc = "Get a reference to the value of field `api_format` after provisioning.\n"]
    pub fn api_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_format", self.base))
    }

    #[doc = "Get a reference to the value of field `max_tokens` after provisioning.\n"]
    pub fn max_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
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

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Model_GeminiModelConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_params: std::option::Option<terrars::PrimField<std::string::String>>,
    api_key_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_tokens: std::option::Option<terrars::PrimField<f64>>,
    model_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temperature: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_k: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_p: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentcoreHarness_Model_GeminiModelConfig {
    #[doc = "Set the field `additional_params`.\n"]
    pub fn set_additional_params(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.additional_params = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_tokens`.\n"]
    pub fn set_max_tokens(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_tokens = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for BedrockagentcoreHarness_Model_GeminiModelConfig {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Model_GeminiModelConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Model_GeminiModelConfig {
    #[doc = ""]
    pub api_key_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub model_id: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_Model_GeminiModelConfig {
    pub fn build(self) -> BedrockagentcoreHarness_Model_GeminiModelConfig {
        BedrockagentcoreHarness_Model_GeminiModelConfig {
            additional_params: core::default::Default::default(),
            api_key_arn: self.api_key_arn,
            max_tokens: core::default::Default::default(),
            model_id: self.model_id,
            temperature: core::default::Default::default(),
            top_k: core::default::Default::default(),
            top_p: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Model_GeminiModelConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Model_GeminiModelConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Model_GeminiModelConfigRef {
        BedrockagentcoreHarness_Model_GeminiModelConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Model_GeminiModelConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_params` after provisioning.\n"]
    pub fn additional_params(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_params", self.base))
    }

    #[doc = "Get a reference to the value of field `api_key_arn` after provisioning.\n"]
    pub fn api_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `max_tokens` after provisioning.\n"]
    pub fn max_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
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
pub struct BedrockagentcoreHarness_Model_LitellmModelConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_params: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_base: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_tokens: std::option::Option<terrars::PrimField<f64>>,
    model_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temperature: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_p: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentcoreHarness_Model_LitellmModelConfig {
    #[doc = "Set the field `additional_params`.\n"]
    pub fn set_additional_params(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.additional_params = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_base`.\n"]
    pub fn set_api_base(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_base = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_key_arn`.\n"]
    pub fn set_api_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_tokens`.\n"]
    pub fn set_max_tokens(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_tokens = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for BedrockagentcoreHarness_Model_LitellmModelConfig {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Model_LitellmModelConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Model_LitellmModelConfig {
    #[doc = ""]
    pub model_id: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_Model_LitellmModelConfig {
    pub fn build(self) -> BedrockagentcoreHarness_Model_LitellmModelConfig {
        BedrockagentcoreHarness_Model_LitellmModelConfig {
            additional_params: core::default::Default::default(),
            api_base: core::default::Default::default(),
            api_key_arn: core::default::Default::default(),
            max_tokens: core::default::Default::default(),
            model_id: self.model_id,
            temperature: core::default::Default::default(),
            top_p: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Model_LitellmModelConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Model_LitellmModelConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Model_LitellmModelConfigRef {
        BedrockagentcoreHarness_Model_LitellmModelConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Model_LitellmModelConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_params` after provisioning.\n"]
    pub fn additional_params(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_params", self.base))
    }

    #[doc = "Get a reference to the value of field `api_base` after provisioning.\n"]
    pub fn api_base(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_base", self.base))
    }

    #[doc = "Get a reference to the value of field `api_key_arn` after provisioning.\n"]
    pub fn api_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `max_tokens` after provisioning.\n"]
    pub fn max_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
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

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Model_OpenaiModelConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_params: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_format: std::option::Option<terrars::PrimField<std::string::String>>,
    api_key_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_tokens: std::option::Option<terrars::PrimField<f64>>,
    model_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temperature: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_p: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentcoreHarness_Model_OpenaiModelConfig {
    #[doc = "Set the field `additional_params`.\n"]
    pub fn set_additional_params(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.additional_params = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_format`.\n"]
    pub fn set_api_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_tokens`.\n"]
    pub fn set_max_tokens(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_tokens = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for BedrockagentcoreHarness_Model_OpenaiModelConfig {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Model_OpenaiModelConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Model_OpenaiModelConfig {
    #[doc = ""]
    pub api_key_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub model_id: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_Model_OpenaiModelConfig {
    pub fn build(self) -> BedrockagentcoreHarness_Model_OpenaiModelConfig {
        BedrockagentcoreHarness_Model_OpenaiModelConfig {
            additional_params: core::default::Default::default(),
            api_format: core::default::Default::default(),
            api_key_arn: self.api_key_arn,
            max_tokens: core::default::Default::default(),
            model_id: self.model_id,
            temperature: core::default::Default::default(),
            top_p: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Model_OpenaiModelConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Model_OpenaiModelConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Model_OpenaiModelConfigRef {
        BedrockagentcoreHarness_Model_OpenaiModelConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Model_OpenaiModelConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_params` after provisioning.\n"]
    pub fn additional_params(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_params", self.base))
    }

    #[doc = "Get a reference to the value of field `api_format` after provisioning.\n"]
    pub fn api_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_format", self.base))
    }

    #[doc = "Get a reference to the value of field `api_key_arn` after provisioning.\n"]
    pub fn api_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `max_tokens` after provisioning.\n"]
    pub fn max_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
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
struct BedrockagentcoreHarness_ModelDynamic {
    bedrock_model_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Model_BedrockModelConfig>>,
    gemini_model_config: std::option::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Model_GeminiModelConfig>>,
    litellm_model_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Model_LitellmModelConfig>>,
    openai_model_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Model_OpenaiModelConfig>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Model {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bedrock_model_config: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_Model_BedrockModelConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gemini_model_config: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_Model_GeminiModelConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    litellm_model_config: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_Model_LitellmModelConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    openai_model_config: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_Model_OpenaiModelConfig>>,
    dynamic: BedrockagentcoreHarness_ModelDynamic,
}

impl BedrockagentcoreHarness_Model {
    #[doc = "Set the field `bedrock_model_config`.\n"]
    pub fn set_bedrock_model_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Model_BedrockModelConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bedrock_model_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bedrock_model_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gemini_model_config`.\n"]
    pub fn set_gemini_model_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Model_GeminiModelConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gemini_model_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gemini_model_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `litellm_model_config`.\n"]
    pub fn set_litellm_model_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Model_LitellmModelConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.litellm_model_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.litellm_model_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `openai_model_config`.\n"]
    pub fn set_openai_model_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Model_OpenaiModelConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.openai_model_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.openai_model_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Model {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Model>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Model {}

impl BuildBedrockagentcoreHarness_Model {
    pub fn build(self) -> BedrockagentcoreHarness_Model {
        BedrockagentcoreHarness_Model {
            bedrock_model_config: core::default::Default::default(),
            gemini_model_config: core::default::Default::default(),
            litellm_model_config: core::default::Default::default(),
            openai_model_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_ModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_ModelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreHarness_ModelRef {
        BedrockagentcoreHarness_ModelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_ModelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bedrock_model_config` after provisioning.\n"]
    pub fn bedrock_model_config(&self) -> terrars::ListRef<BedrockagentcoreHarness_Model_BedrockModelConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bedrock_model_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gemini_model_config` after provisioning.\n"]
    pub fn gemini_model_config(&self) -> terrars::ListRef<BedrockagentcoreHarness_Model_GeminiModelConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gemini_model_config", self.base))
    }

    #[doc = "Get a reference to the value of field `litellm_model_config` after provisioning.\n"]
    pub fn litellm_model_config(&self) -> terrars::ListRef<BedrockagentcoreHarness_Model_LitellmModelConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.litellm_model_config", self.base))
    }

    #[doc = "Get a reference to the value of field `openai_model_config` after provisioning.\n"]
    pub fn openai_model_config(&self) -> terrars::ListRef<BedrockagentcoreHarness_Model_OpenaiModelConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.openai_model_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Skill_AwsSkills {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    paths: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl BedrockagentcoreHarness_Skill_AwsSkills {
    #[doc = "Set the field `paths`.\n"]
    pub fn set_paths(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.paths = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Skill_AwsSkills {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Skill_AwsSkills>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Skill_AwsSkills {}

impl BuildBedrockagentcoreHarness_Skill_AwsSkills {
    pub fn build(self) -> BedrockagentcoreHarness_Skill_AwsSkills {
        BedrockagentcoreHarness_Skill_AwsSkills { paths: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Skill_AwsSkillsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Skill_AwsSkillsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreHarness_Skill_AwsSkillsRef {
        BedrockagentcoreHarness_Skill_AwsSkillsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Skill_AwsSkillsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `paths` after provisioning.\n"]
    pub fn paths(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Skill_Git_Auth {
    credential_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreHarness_Skill_Git_Auth {
    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Skill_Git_Auth {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Skill_Git_Auth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Skill_Git_Auth {
    #[doc = ""]
    pub credential_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_Skill_Git_Auth {
    pub fn build(self) -> BedrockagentcoreHarness_Skill_Git_Auth {
        BedrockagentcoreHarness_Skill_Git_Auth {
            credential_arn: self.credential_arn,
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Skill_Git_AuthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Skill_Git_AuthRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreHarness_Skill_Git_AuthRef {
        BedrockagentcoreHarness_Skill_Git_AuthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Skill_Git_AuthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `credential_arn` after provisioning.\n"]
    pub fn credential_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credential_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreHarness_Skill_GitDynamic {
    auth: std::option::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Skill_Git_Auth>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Skill_Git {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_Skill_Git_Auth>>,
    dynamic: BedrockagentcoreHarness_Skill_GitDynamic,
}

impl BedrockagentcoreHarness_Skill_Git {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth`.\n"]
    pub fn set_auth(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Skill_Git_Auth>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auth = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Skill_Git {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Skill_Git>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Skill_Git {
    #[doc = ""]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_Skill_Git {
    pub fn build(self) -> BedrockagentcoreHarness_Skill_Git {
        BedrockagentcoreHarness_Skill_Git {
            path: core::default::Default::default(),
            url: self.url,
            auth: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Skill_GitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Skill_GitRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreHarness_Skill_GitRef {
        BedrockagentcoreHarness_Skill_GitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Skill_GitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `auth` after provisioning.\n"]
    pub fn auth(&self) -> terrars::ListRef<BedrockagentcoreHarness_Skill_Git_AuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Skill_S3 {
    uri: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreHarness_Skill_S3 { }

impl terrars::ToListMappable for BedrockagentcoreHarness_Skill_S3 {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Skill_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Skill_S3 {
    #[doc = ""]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_Skill_S3 {
    pub fn build(self) -> BedrockagentcoreHarness_Skill_S3 {
        BedrockagentcoreHarness_Skill_S3 { uri: self.uri }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Skill_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Skill_S3Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreHarness_Skill_S3Ref {
        BedrockagentcoreHarness_Skill_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Skill_S3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreHarness_SkillDynamic {
    aws_skills: std::option::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Skill_AwsSkills>>,
    git: std::option::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Skill_Git>>,
    s3: std::option::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Skill_S3>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Skill {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_skills: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_Skill_AwsSkills>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_Skill_Git>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_Skill_S3>>,
    dynamic: BedrockagentcoreHarness_SkillDynamic,
}

impl BedrockagentcoreHarness_Skill {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aws_skills`.\n"]
    pub fn set_aws_skills(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Skill_AwsSkills>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_skills = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_skills = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `git`.\n"]
    pub fn set_git(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Skill_Git>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.git = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.git = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Skill_S3>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Skill {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Skill>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Skill {}

impl BuildBedrockagentcoreHarness_Skill {
    pub fn build(self) -> BedrockagentcoreHarness_Skill {
        BedrockagentcoreHarness_Skill {
            path: core::default::Default::default(),
            aws_skills: core::default::Default::default(),
            git: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_SkillRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_SkillRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreHarness_SkillRef {
        BedrockagentcoreHarness_SkillRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_SkillRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_skills` after provisioning.\n"]
    pub fn aws_skills(&self) -> terrars::ListRef<BedrockagentcoreHarness_Skill_AwsSkillsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_skills", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<BedrockagentcoreHarness_Skill_GitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<BedrockagentcoreHarness_Skill_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_SystemPrompt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreHarness_SystemPrompt {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_SystemPrompt {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_SystemPrompt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_SystemPrompt {}

impl BuildBedrockagentcoreHarness_SystemPrompt {
    pub fn build(self) -> BedrockagentcoreHarness_SystemPrompt {
        BedrockagentcoreHarness_SystemPrompt { text: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_SystemPromptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_SystemPromptRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreHarness_SystemPromptRef {
        BedrockagentcoreHarness_SystemPromptRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_SystemPromptRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreHarness_Timeouts {
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

impl terrars::ToListMappable for BedrockagentcoreHarness_Timeouts {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Timeouts {}

impl BuildBedrockagentcoreHarness_Timeouts {
    pub fn build(self) -> BedrockagentcoreHarness_Timeouts {
        BedrockagentcoreHarness_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreHarness_TimeoutsRef {
        BedrockagentcoreHarness_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_TimeoutsRef {
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
pub struct BedrockagentcoreHarness_Tool_Config_AgentcoreBrowser {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    browser_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreHarness_Tool_Config_AgentcoreBrowser {
    #[doc = "Set the field `browser_arn`.\n"]
    pub fn set_browser_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.browser_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Tool_Config_AgentcoreBrowser {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Tool_Config_AgentcoreBrowser>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Tool_Config_AgentcoreBrowser {}

impl BuildBedrockagentcoreHarness_Tool_Config_AgentcoreBrowser {
    pub fn build(self) -> BedrockagentcoreHarness_Tool_Config_AgentcoreBrowser {
        BedrockagentcoreHarness_Tool_Config_AgentcoreBrowser { browser_arn: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Tool_Config_AgentcoreBrowserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Tool_Config_AgentcoreBrowserRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Tool_Config_AgentcoreBrowserRef {
        BedrockagentcoreHarness_Tool_Config_AgentcoreBrowserRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Tool_Config_AgentcoreBrowserRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `browser_arn` after provisioning.\n"]
    pub fn browser_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.browser_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Tool_Config_AgentcoreCodeInterpreter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_interpreter_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreHarness_Tool_Config_AgentcoreCodeInterpreter {
    #[doc = "Set the field `code_interpreter_arn`.\n"]
    pub fn set_code_interpreter_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.code_interpreter_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Tool_Config_AgentcoreCodeInterpreter {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Tool_Config_AgentcoreCodeInterpreter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Tool_Config_AgentcoreCodeInterpreter {}

impl BuildBedrockagentcoreHarness_Tool_Config_AgentcoreCodeInterpreter {
    pub fn build(self) -> BedrockagentcoreHarness_Tool_Config_AgentcoreCodeInterpreter {
        BedrockagentcoreHarness_Tool_Config_AgentcoreCodeInterpreter {
            code_interpreter_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Tool_Config_AgentcoreCodeInterpreterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Tool_Config_AgentcoreCodeInterpreterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Tool_Config_AgentcoreCodeInterpreterRef {
        BedrockagentcoreHarness_Tool_Config_AgentcoreCodeInterpreterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Tool_Config_AgentcoreCodeInterpreterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code_interpreter_arn` after provisioning.\n"]
    pub fn code_interpreter_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_interpreter_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth_Oauth {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_return_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grant_type: std::option::Option<terrars::PrimField<std::string::String>>,
    provider_arn: terrars::PrimField<std::string::String>,
    scopes: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth_Oauth {
    #[doc = "Set the field `custom_parameters`.\n"]
    pub fn set_custom_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.custom_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_return_url`.\n"]
    pub fn set_default_return_url(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_return_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grant_type`.\n"]
    pub fn set_grant_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.grant_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth_Oauth {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth_Oauth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth_Oauth {
    #[doc = ""]
    pub provider_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub scopes: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildBedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth_Oauth {
    pub fn build(self) -> BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth_Oauth {
        BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth_Oauth {
            custom_parameters: core::default::Default::default(),
            default_return_url: core::default::Default::default(),
            grant_type: core::default::Default::default(),
            provider_arn: self.provider_arn,
            scopes: self.scopes,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth_OauthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth_OauthRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth_OauthRef {
        BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth_OauthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth_OauthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_parameters` after provisioning.\n"]
    pub fn custom_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.custom_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `default_return_url` after provisioning.\n"]
    pub fn default_return_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_return_url", self.base))
    }

    #[doc = "Get a reference to the value of field `grant_type` after provisioning.\n"]
    pub fn grant_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grant_type", self.base))
    }

    #[doc = "Get a reference to the value of field `provider_arn` after provisioning.\n"]
    pub fn provider_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `scopes` after provisioning.\n"]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuthDynamic {
    oauth: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth_Oauth>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_iam: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    none: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth_Oauth>>,
    dynamic: BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuthDynamic,
}

impl BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth {
    #[doc = "Set the field `aws_iam`.\n"]
    pub fn set_aws_iam(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.aws_iam = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `none`.\n"]
    pub fn set_none(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.none = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth`.\n"]
    pub fn set_oauth(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth_Oauth>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth {}

impl BuildBedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth {
    pub fn build(self) -> BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth {
        BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth {
            aws_iam: core::default::Default::default(),
            none: core::default::Default::default(),
            oauth: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuthRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuthRef {
        BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_iam` after provisioning.\n"]
    pub fn aws_iam(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_iam", self.base))
    }

    #[doc = "Get a reference to the value of field `none` after provisioning.\n"]
    pub fn none(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.none", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth` after provisioning.\n"]
    pub fn oauth(&self) -> terrars::ListRef<BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth_OauthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreHarness_Tool_Config_AgentcoreGatewayDynamic {
    outbound_auth: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Tool_Config_AgentcoreGateway {
    gateway_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outbound_auth: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth>>,
    dynamic: BedrockagentcoreHarness_Tool_Config_AgentcoreGatewayDynamic,
}

impl BedrockagentcoreHarness_Tool_Config_AgentcoreGateway {
    #[doc = "Set the field `outbound_auth`.\n"]
    pub fn set_outbound_auth(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuth>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.outbound_auth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.outbound_auth = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Tool_Config_AgentcoreGateway {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Tool_Config_AgentcoreGateway>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Tool_Config_AgentcoreGateway {
    #[doc = ""]
    pub gateway_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_Tool_Config_AgentcoreGateway {
    pub fn build(self) -> BedrockagentcoreHarness_Tool_Config_AgentcoreGateway {
        BedrockagentcoreHarness_Tool_Config_AgentcoreGateway {
            gateway_arn: self.gateway_arn,
            outbound_auth: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Tool_Config_AgentcoreGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Tool_Config_AgentcoreGatewayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Tool_Config_AgentcoreGatewayRef {
        BedrockagentcoreHarness_Tool_Config_AgentcoreGatewayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Tool_Config_AgentcoreGatewayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gateway_arn` after provisioning.\n"]
    pub fn gateway_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `outbound_auth` after provisioning.\n"]
    pub fn outbound_auth(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreHarness_Tool_Config_AgentcoreGateway_OutboundAuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outbound_auth", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Tool_Config_InlineFunction {
    description: terrars::PrimField<std::string::String>,
    input_schema: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreHarness_Tool_Config_InlineFunction { }

impl terrars::ToListMappable for BedrockagentcoreHarness_Tool_Config_InlineFunction {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Tool_Config_InlineFunction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Tool_Config_InlineFunction {
    #[doc = ""]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub input_schema: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_Tool_Config_InlineFunction {
    pub fn build(self) -> BedrockagentcoreHarness_Tool_Config_InlineFunction {
        BedrockagentcoreHarness_Tool_Config_InlineFunction {
            description: self.description,
            input_schema: self.input_schema,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Tool_Config_InlineFunctionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Tool_Config_InlineFunctionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Tool_Config_InlineFunctionRef {
        BedrockagentcoreHarness_Tool_Config_InlineFunctionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Tool_Config_InlineFunctionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `input_schema` after provisioning.\n"]
    pub fn input_schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_schema", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Tool_Config_RemoteMcp {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    url: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreHarness_Tool_Config_RemoteMcp {
    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.headers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Tool_Config_RemoteMcp {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Tool_Config_RemoteMcp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Tool_Config_RemoteMcp {
    #[doc = ""]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_Tool_Config_RemoteMcp {
    pub fn build(self) -> BedrockagentcoreHarness_Tool_Config_RemoteMcp {
        BedrockagentcoreHarness_Tool_Config_RemoteMcp {
            headers: core::default::Default::default(),
            url: self.url,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Tool_Config_RemoteMcpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Tool_Config_RemoteMcpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreHarness_Tool_Config_RemoteMcpRef {
        BedrockagentcoreHarness_Tool_Config_RemoteMcpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Tool_Config_RemoteMcpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreHarness_Tool_ConfigDynamic {
    agentcore_browser: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Tool_Config_AgentcoreBrowser>>,
    agentcore_code_interpreter: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Tool_Config_AgentcoreCodeInterpreter>>,
    agentcore_gateway: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Tool_Config_AgentcoreGateway>>,
    inline_function: std::option::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Tool_Config_InlineFunction>>,
    remote_mcp: std::option::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Tool_Config_RemoteMcp>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Tool_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agentcore_browser: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_Tool_Config_AgentcoreBrowser>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agentcore_code_interpreter: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreHarness_Tool_Config_AgentcoreCodeInterpreter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agentcore_gateway: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_Tool_Config_AgentcoreGateway>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inline_function: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_Tool_Config_InlineFunction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_mcp: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_Tool_Config_RemoteMcp>>,
    dynamic: BedrockagentcoreHarness_Tool_ConfigDynamic,
}

impl BedrockagentcoreHarness_Tool_Config {
    #[doc = "Set the field `agentcore_browser`.\n"]
    pub fn set_agentcore_browser(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Tool_Config_AgentcoreBrowser>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.agentcore_browser = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.agentcore_browser = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `agentcore_code_interpreter`.\n"]
    pub fn set_agentcore_code_interpreter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Tool_Config_AgentcoreCodeInterpreter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.agentcore_code_interpreter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.agentcore_code_interpreter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `agentcore_gateway`.\n"]
    pub fn set_agentcore_gateway(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Tool_Config_AgentcoreGateway>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.agentcore_gateway = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.agentcore_gateway = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inline_function`.\n"]
    pub fn set_inline_function(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Tool_Config_InlineFunction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inline_function = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inline_function = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote_mcp`.\n"]
    pub fn set_remote_mcp(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Tool_Config_RemoteMcp>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.remote_mcp = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.remote_mcp = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Tool_Config {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Tool_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Tool_Config {}

impl BuildBedrockagentcoreHarness_Tool_Config {
    pub fn build(self) -> BedrockagentcoreHarness_Tool_Config {
        BedrockagentcoreHarness_Tool_Config {
            agentcore_browser: core::default::Default::default(),
            agentcore_code_interpreter: core::default::Default::default(),
            agentcore_gateway: core::default::Default::default(),
            inline_function: core::default::Default::default(),
            remote_mcp: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_Tool_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_Tool_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreHarness_Tool_ConfigRef {
        BedrockagentcoreHarness_Tool_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_Tool_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agentcore_browser` after provisioning.\n"]
    pub fn agentcore_browser(&self) -> terrars::ListRef<BedrockagentcoreHarness_Tool_Config_AgentcoreBrowserRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agentcore_browser", self.base))
    }

    #[doc = "Get a reference to the value of field `agentcore_code_interpreter` after provisioning.\n"]
    pub fn agentcore_code_interpreter(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreHarness_Tool_Config_AgentcoreCodeInterpreterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agentcore_code_interpreter", self.base))
    }

    #[doc = "Get a reference to the value of field `agentcore_gateway` after provisioning.\n"]
    pub fn agentcore_gateway(&self) -> terrars::ListRef<BedrockagentcoreHarness_Tool_Config_AgentcoreGatewayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agentcore_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `inline_function` after provisioning.\n"]
    pub fn inline_function(&self) -> terrars::ListRef<BedrockagentcoreHarness_Tool_Config_InlineFunctionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inline_function", self.base))
    }

    #[doc = "Get a reference to the value of field `remote_mcp` after provisioning.\n"]
    pub fn remote_mcp(&self) -> terrars::ListRef<BedrockagentcoreHarness_Tool_Config_RemoteMcpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_mcp", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreHarness_ToolDynamic {
    config: std::option::Option<terrars::DynamicBlock<BedrockagentcoreHarness_Tool_Config>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreHarness_Tool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<BedrockagentcoreHarness_Tool_Config>>,
    dynamic: BedrockagentcoreHarness_ToolDynamic,
}

impl BedrockagentcoreHarness_Tool {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreHarness_Tool_Config>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreHarness_Tool {
    type O = terrars::BlockAssignable<BedrockagentcoreHarness_Tool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreHarness_Tool {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreHarness_Tool {
    pub fn build(self) -> BedrockagentcoreHarness_Tool {
        BedrockagentcoreHarness_Tool {
            name: core::default::Default::default(),
            type_: self.type_,
            config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreHarness_ToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreHarness_ToolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreHarness_ToolRef {
        BedrockagentcoreHarness_ToolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreHarness_ToolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<BedrockagentcoreHarness_Tool_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
}
