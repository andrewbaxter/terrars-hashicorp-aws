#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentcoreMemoryStrategyData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_execution_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    memory_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace_templates: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespaces: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<BedrockagentcoreMemoryStrategy_Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_record_schema: std::option::Option<std::vec::Vec<BedrockagentcoreMemoryStrategy_MemoryRecordSchema>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reflection_configuration: std::option::Option<std::vec::Vec<BedrockagentcoreMemoryStrategy_ReflectionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockagentcoreMemoryStrategy_Timeouts>,
    dynamic: BedrockagentcoreMemoryStrategyDynamic,
}

struct BedrockagentcoreMemoryStrategy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockagentcoreMemoryStrategyData_>,
}

#[derive(Clone)]
pub struct BedrockagentcoreMemoryStrategy(std::rc::Rc<BedrockagentcoreMemoryStrategy_>);

impl BedrockagentcoreMemoryStrategy {
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

    #[doc = "Set the field `memory_execution_role_arn`.\n"]
    pub fn set_memory_execution_role_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().memory_execution_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespace_templates`.\n"]
    pub fn set_namespace_templates(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().namespace_templates = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespaces`.\n"]
    pub fn set_namespaces(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().namespaces = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreMemoryStrategy_Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `memory_record_schema`.\n"]
    pub fn set_memory_record_schema(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreMemoryStrategy_MemoryRecordSchema>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().memory_record_schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.memory_record_schema = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `reflection_configuration`.\n"]
    pub fn set_reflection_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreMemoryStrategy_ReflectionConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().reflection_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.reflection_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockagentcoreMemoryStrategy_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_execution_role_arn` after provisioning.\n"]
    pub fn memory_execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_id` after provisioning.\n"]
    pub fn memory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_strategy_id` after provisioning.\n"]
    pub fn memory_strategy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_strategy_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespace_templates` after provisioning.\n"]
    pub fn namespace_templates(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.namespace_templates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespaces` after provisioning.\n"]
    pub fn namespaces(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.namespaces", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<BedrockagentcoreMemoryStrategy_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_record_schema` after provisioning.\n"]
    pub fn memory_record_schema(&self) -> terrars::ListRef<BedrockagentcoreMemoryStrategy_MemoryRecordSchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_record_schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reflection_configuration` after provisioning.\n"]
    pub fn reflection_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreMemoryStrategy_ReflectionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reflection_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreMemoryStrategy_TimeoutsRef {
        BedrockagentcoreMemoryStrategy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for BedrockagentcoreMemoryStrategy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockagentcoreMemoryStrategy { }

impl terrars::ToListMappable for BedrockagentcoreMemoryStrategy {
    type O = terrars::ListRef<BedrockagentcoreMemoryStrategyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockagentcoreMemoryStrategy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrockagentcore_memory_strategy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub memory_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreMemoryStrategy {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockagentcoreMemoryStrategy {
        let out = BedrockagentcoreMemoryStrategy(std::rc::Rc::new(BedrockagentcoreMemoryStrategy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockagentcoreMemoryStrategyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                memory_execution_role_arn: core::default::Default::default(),
                memory_id: self.memory_id,
                name: self.name,
                namespace_templates: core::default::Default::default(),
                namespaces: core::default::Default::default(),
                region: core::default::Default::default(),
                type_: self.type_,
                configuration: core::default::Default::default(),
                memory_record_schema: core::default::Default::default(),
                reflection_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentcoreMemoryStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreMemoryStrategyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockagentcoreMemoryStrategyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_execution_role_arn` after provisioning.\n"]
    pub fn memory_execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_id` after provisioning.\n"]
    pub fn memory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_strategy_id` after provisioning.\n"]
    pub fn memory_strategy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_strategy_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespace_templates` after provisioning.\n"]
    pub fn namespace_templates(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.namespace_templates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespaces` after provisioning.\n"]
    pub fn namespaces(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.namespaces", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<BedrockagentcoreMemoryStrategy_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_record_schema` after provisioning.\n"]
    pub fn memory_record_schema(&self) -> terrars::ListRef<BedrockagentcoreMemoryStrategy_MemoryRecordSchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_record_schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reflection_configuration` after provisioning.\n"]
    pub fn reflection_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreMemoryStrategy_ReflectionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reflection_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreMemoryStrategy_TimeoutsRef {
        BedrockagentcoreMemoryStrategy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreMemoryStrategyDynamic {
    configuration: std::option::Option<terrars::DynamicBlock<BedrockagentcoreMemoryStrategy_Configuration>>,
    memory_record_schema: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreMemoryStrategy_MemoryRecordSchema>>,
    reflection_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreMemoryStrategy_ReflectionConfiguration>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_Consolidation {
    append_to_prompt: terrars::PrimField<std::string::String>,
    model_id: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreMemoryStrategy_Configuration_Consolidation { }

impl terrars::ToListMappable for BedrockagentcoreMemoryStrategy_Configuration_Consolidation {
    type O = terrars::BlockAssignable<BedrockagentcoreMemoryStrategy_Configuration_Consolidation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_Configuration_Consolidation {
    #[doc = ""]
    pub append_to_prompt: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub model_id: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreMemoryStrategy_Configuration_Consolidation {
    pub fn build(self) -> BedrockagentcoreMemoryStrategy_Configuration_Consolidation {
        BedrockagentcoreMemoryStrategy_Configuration_Consolidation {
            append_to_prompt: self.append_to_prompt,
            model_id: self.model_id,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_ConsolidationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreMemoryStrategy_Configuration_ConsolidationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_Configuration_ConsolidationRef {
        BedrockagentcoreMemoryStrategy_Configuration_ConsolidationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_Configuration_ConsolidationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `append_to_prompt` after provisioning.\n"]
    pub fn append_to_prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.append_to_prompt", self.base))
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_Extraction {
    append_to_prompt: terrars::PrimField<std::string::String>,
    model_id: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreMemoryStrategy_Configuration_Extraction { }

impl terrars::ToListMappable for BedrockagentcoreMemoryStrategy_Configuration_Extraction {
    type O = terrars::BlockAssignable<BedrockagentcoreMemoryStrategy_Configuration_Extraction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_Configuration_Extraction {
    #[doc = ""]
    pub append_to_prompt: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub model_id: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreMemoryStrategy_Configuration_Extraction {
    pub fn build(self) -> BedrockagentcoreMemoryStrategy_Configuration_Extraction {
        BedrockagentcoreMemoryStrategy_Configuration_Extraction {
            append_to_prompt: self.append_to_prompt,
            model_id: self.model_id,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_ExtractionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreMemoryStrategy_Configuration_ExtractionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_Configuration_ExtractionRef {
        BedrockagentcoreMemoryStrategy_Configuration_ExtractionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_Configuration_ExtractionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `append_to_prompt` after provisioning.\n"]
    pub fn append_to_prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.append_to_prompt", self.base))
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_Reflection {
    append_to_prompt: terrars::PrimField<std::string::String>,
    model_id: terrars::PrimField<std::string::String>,
    namespace_templates: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreMemoryStrategy_Configuration_Reflection { }

impl terrars::ToListMappable for BedrockagentcoreMemoryStrategy_Configuration_Reflection {
    type O = terrars::BlockAssignable<BedrockagentcoreMemoryStrategy_Configuration_Reflection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_Configuration_Reflection {
    #[doc = ""]
    pub append_to_prompt: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub model_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub namespace_templates: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildBedrockagentcoreMemoryStrategy_Configuration_Reflection {
    pub fn build(self) -> BedrockagentcoreMemoryStrategy_Configuration_Reflection {
        BedrockagentcoreMemoryStrategy_Configuration_Reflection {
            append_to_prompt: self.append_to_prompt,
            model_id: self.model_id,
            namespace_templates: self.namespace_templates,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_ReflectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreMemoryStrategy_Configuration_ReflectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_Configuration_ReflectionRef {
        BedrockagentcoreMemoryStrategy_Configuration_ReflectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_Configuration_ReflectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `append_to_prompt` after provisioning.\n"]
    pub fn append_to_prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.append_to_prompt", self.base))
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace_templates` after provisioning.\n"]
    pub fn namespace_templates(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.namespace_templates", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_MessageBasedTrigger {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_count: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_MessageBasedTrigger {
    #[doc = "Set the field `message_count`.\n"]
    pub fn set_message_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.message_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_MessageBasedTrigger {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_MessageBasedTrigger,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_MessageBasedTrigger {}

impl BuildBedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_MessageBasedTrigger {
    pub fn build(
        self,
    ) -> BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_MessageBasedTrigger {
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_MessageBasedTrigger {
            message_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_MessageBasedTriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_MessageBasedTriggerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_MessageBasedTriggerRef {
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_MessageBasedTriggerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_MessageBasedTriggerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message_count` after provisioning.\n"]
    pub fn message_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_count", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TimeBasedTrigger {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_session_timeout: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TimeBasedTrigger {
    #[doc = "Set the field `idle_session_timeout`.\n"]
    pub fn set_idle_session_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.idle_session_timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TimeBasedTrigger {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TimeBasedTrigger,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TimeBasedTrigger {}

impl BuildBedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TimeBasedTrigger {
    pub fn build(
        self,
    ) -> BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TimeBasedTrigger {
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TimeBasedTrigger {
            idle_session_timeout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TimeBasedTriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TimeBasedTriggerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TimeBasedTriggerRef {
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TimeBasedTriggerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TimeBasedTriggerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_session_timeout` after provisioning.\n"]
    pub fn idle_session_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_session_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TokenBasedTrigger {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_count: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TokenBasedTrigger {
    #[doc = "Set the field `token_count`.\n"]
    pub fn set_token_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.token_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TokenBasedTrigger {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TokenBasedTrigger,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TokenBasedTrigger {}

impl BuildBedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TokenBasedTrigger {
    pub fn build(
        self,
    ) -> BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TokenBasedTrigger {
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TokenBasedTrigger {
            token_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TokenBasedTriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TokenBasedTriggerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TokenBasedTriggerRef {
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TokenBasedTriggerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TokenBasedTriggerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token_count` after provisioning.\n"]
    pub fn token_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_count", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_based_trigger: std
    ::option
    ::Option<
        terrars
        ::ListField<
            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_MessageBasedTrigger,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_based_trigger: std
    ::option
    ::Option<
        terrars
        ::ListField<
            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TimeBasedTrigger,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_based_trigger: std
    ::option
    ::Option<
        terrars
        ::ListField<
            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TokenBasedTrigger,
        >,
    >,
}

impl BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual {
    #[doc = "Set the field `message_based_trigger`.\n"]
    pub fn set_message_based_trigger(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_MessageBasedTrigger,
                        >,
                    >,
    ) -> Self {
        self.message_based_trigger = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_based_trigger`.\n"]
    pub fn set_time_based_trigger(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TimeBasedTrigger,
                        >,
                    >,
    ) -> Self {
        self.time_based_trigger = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_based_trigger`.\n"]
    pub fn set_token_based_trigger(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TokenBasedTrigger,
                        >,
                    >,
    ) -> Self {
        self.token_based_trigger = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual {}

impl BuildBedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual {
    pub fn build(
        self,
    ) -> BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual {
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual {
            message_based_trigger: core::default::Default::default(),
            time_based_trigger: core::default::Default::default(),
            token_based_trigger: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActualRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActualRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActualRef {
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActualRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActualRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message_based_trigger` after provisioning.\n"]
    pub fn message_based_trigger(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_MessageBasedTriggerRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_based_trigger", self.base))
    }

    #[doc = "Get a reference to the value of field `time_based_trigger` after provisioning.\n"]
    pub fn time_based_trigger(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TimeBasedTriggerRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_based_trigger", self.base))
    }

    #[doc = "Get a reference to the value of field `token_based_trigger` after provisioning.\n"]
    pub fn token_based_trigger(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActual_TokenBasedTriggerRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.token_based_trigger", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_InvocationConfiguration {
    payload_delivery_bucket_name: terrars::PrimField<std::string::String>,
    topic_arn: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_InvocationConfiguration { }

impl terrars
::ToListMappable for BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_InvocationConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_InvocationConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_InvocationConfiguration {
    #[doc = ""]
    pub payload_delivery_bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub topic_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_InvocationConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_InvocationConfiguration {
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_InvocationConfiguration {
            payload_delivery_bucket_name: self.payload_delivery_bucket_name,
            topic_arn: self.topic_arn,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_InvocationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_InvocationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_InvocationConfigurationRef {
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_InvocationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_InvocationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `payload_delivery_bucket_name` after provisioning.\n"]
    pub fn payload_delivery_bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload_delivery_bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_MessageBasedTrigger {
    message_count: terrars::PrimField<f64>,
}

impl BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_MessageBasedTrigger { }

impl terrars
::ToListMappable for BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_MessageBasedTrigger {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_MessageBasedTrigger,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_MessageBasedTrigger {
    #[doc = ""]
    pub message_count: terrars::PrimField<f64>,
}

impl BuildBedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_MessageBasedTrigger {
    pub fn build(
        self,
    ) -> BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_MessageBasedTrigger {
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_MessageBasedTrigger {
            message_count: self.message_count,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_MessageBasedTriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_MessageBasedTriggerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_MessageBasedTriggerRef {
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_MessageBasedTriggerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_MessageBasedTriggerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message_count` after provisioning.\n"]
    pub fn message_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_count", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TimeBasedTrigger {
    idle_session_timeout: terrars::PrimField<f64>,
}

impl BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TimeBasedTrigger { }

impl terrars
::ToListMappable for BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TimeBasedTrigger {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TimeBasedTrigger,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TimeBasedTrigger {
    #[doc = ""]
    pub idle_session_timeout: terrars::PrimField<f64>,
}

impl BuildBedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TimeBasedTrigger {
    pub fn build(
        self,
    ) -> BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TimeBasedTrigger {
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TimeBasedTrigger {
            idle_session_timeout: self.idle_session_timeout,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TimeBasedTriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TimeBasedTriggerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TimeBasedTriggerRef {
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TimeBasedTriggerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TimeBasedTriggerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_session_timeout` after provisioning.\n"]
    pub fn idle_session_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_session_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TokenBasedTrigger {
    token_count: terrars::PrimField<f64>,
}

impl BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TokenBasedTrigger { }

impl terrars
::ToListMappable for BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TokenBasedTrigger {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TokenBasedTrigger,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TokenBasedTrigger {
    #[doc = ""]
    pub token_count: terrars::PrimField<f64>,
}

impl BuildBedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TokenBasedTrigger {
    pub fn build(
        self,
    ) -> BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TokenBasedTrigger {
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TokenBasedTrigger {
            token_count: self.token_count,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TokenBasedTriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TokenBasedTriggerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TokenBasedTriggerRef {
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TokenBasedTriggerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TokenBasedTriggerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token_count` after provisioning.\n"]
    pub fn token_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_count", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsDynamic {
    message_based_trigger: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_MessageBasedTrigger,
        >,
    >,
    time_based_trigger: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TimeBasedTrigger,
        >,
    >,
    token_based_trigger: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TokenBasedTrigger,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_based_trigger: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_MessageBasedTrigger,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_based_trigger: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TimeBasedTrigger,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_based_trigger: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TokenBasedTrigger,
        >,
    >,
    dynamic: BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsDynamic,
}

impl BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions {
    #[doc = "Set the field `message_based_trigger`.\n"]
    pub fn set_message_based_trigger(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_MessageBasedTrigger,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.message_based_trigger = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.message_based_trigger = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `time_based_trigger`.\n"]
    pub fn set_time_based_trigger(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TimeBasedTrigger,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.time_based_trigger = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.time_based_trigger = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `token_based_trigger`.\n"]
    pub fn set_token_based_trigger(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TokenBasedTrigger,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.token_based_trigger = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.token_based_trigger = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions {
    type O =
        terrars
        ::BlockAssignable<BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions {}

impl BuildBedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions {
    pub fn build(self) -> BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions {
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions {
            message_based_trigger: core::default::Default::default(),
            time_based_trigger: core::default::Default::default(),
            token_based_trigger: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsRef {
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message_based_trigger` after provisioning.\n"]
    pub fn message_based_trigger(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_MessageBasedTriggerRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_based_trigger", self.base))
    }

    #[doc = "Get a reference to the value of field `time_based_trigger` after provisioning.\n"]
    pub fn time_based_trigger(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TimeBasedTriggerRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_based_trigger", self.base))
    }

    #[doc = "Get a reference to the value of field `token_based_trigger` after provisioning.\n"]
    pub fn token_based_trigger(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions_TokenBasedTriggerRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.token_based_trigger", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfigurationDynamic {
    invocation_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_InvocationConfiguration>,
    >,
    trigger_conditions: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    historical_context_window_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invocation_configuration: std
    ::option
    ::Option<
        std::vec::Vec<BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_InvocationConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_conditions: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions>>,
    dynamic: BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfigurationDynamic,
}

impl BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration {
    #[doc = "Set the field `historical_context_window_size`.\n"]
    pub fn set_historical_context_window_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.historical_context_window_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `invocation_configuration`.\n"]
    pub fn set_invocation_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_InvocationConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.invocation_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.invocation_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trigger_conditions`.\n"]
    pub fn set_trigger_conditions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditions,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trigger_conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trigger_conditions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration {}

impl BuildBedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration {
    pub fn build(self) -> BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration {
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration {
            historical_context_window_size: core::default::Default::default(),
            invocation_configuration: core::default::Default::default(),
            trigger_conditions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfigurationRef {
        BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `historical_context_window_size` after provisioning.\n"]
    pub fn historical_context_window_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.historical_context_window_size", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger_conditions_actual` after provisioning.\n"]
    pub fn trigger_conditions_actual(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsActualRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_conditions_actual", self.base))
    }

    #[doc = "Get a reference to the value of field `invocation_configuration` after provisioning.\n"]
    pub fn invocation_configuration(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_InvocationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.invocation_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger_conditions` after provisioning.\n"]
    pub fn trigger_conditions(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration_TriggerConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_conditions", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreMemoryStrategy_ConfigurationDynamic {
    consolidation: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreMemoryStrategy_Configuration_Consolidation>>,
    extraction: std::option::Option<terrars::DynamicBlock<BedrockagentcoreMemoryStrategy_Configuration_Extraction>>,
    reflection: std::option::Option<terrars::DynamicBlock<BedrockagentcoreMemoryStrategy_Configuration_Reflection>>,
    self_managed_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_Configuration {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consolidation: std::option::Option<std::vec::Vec<BedrockagentcoreMemoryStrategy_Configuration_Consolidation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extraction: std::option::Option<std::vec::Vec<BedrockagentcoreMemoryStrategy_Configuration_Extraction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reflection: std::option::Option<std::vec::Vec<BedrockagentcoreMemoryStrategy_Configuration_Reflection>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_managed_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration>>,
    dynamic: BedrockagentcoreMemoryStrategy_ConfigurationDynamic,
}

impl BedrockagentcoreMemoryStrategy_Configuration {
    #[doc = "Set the field `consolidation`.\n"]
    pub fn set_consolidation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreMemoryStrategy_Configuration_Consolidation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.consolidation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.consolidation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `extraction`.\n"]
    pub fn set_extraction(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreMemoryStrategy_Configuration_Extraction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.extraction = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.extraction = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `reflection`.\n"]
    pub fn set_reflection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreMemoryStrategy_Configuration_Reflection>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.reflection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.reflection = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `self_managed_configuration`.\n"]
    pub fn set_self_managed_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.self_managed_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.self_managed_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreMemoryStrategy_Configuration {
    type O = terrars::BlockAssignable<BedrockagentcoreMemoryStrategy_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_Configuration {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreMemoryStrategy_Configuration {
    pub fn build(self) -> BedrockagentcoreMemoryStrategy_Configuration {
        BedrockagentcoreMemoryStrategy_Configuration {
            type_: self.type_,
            consolidation: core::default::Default::default(),
            extraction: core::default::Default::default(),
            reflection: core::default::Default::default(),
            self_managed_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreMemoryStrategy_ConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_ConfigurationRef {
        BedrockagentcoreMemoryStrategy_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `consolidation` after provisioning.\n"]
    pub fn consolidation(&self) -> terrars::ListRef<BedrockagentcoreMemoryStrategy_Configuration_ConsolidationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consolidation", self.base))
    }

    #[doc = "Get a reference to the value of field `extraction` after provisioning.\n"]
    pub fn extraction(&self) -> terrars::ListRef<BedrockagentcoreMemoryStrategy_Configuration_ExtractionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extraction", self.base))
    }

    #[doc = "Get a reference to the value of field `reflection` after provisioning.\n"]
    pub fn reflection(&self) -> terrars::ListRef<BedrockagentcoreMemoryStrategy_Configuration_ReflectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reflection", self.base))
    }

    #[doc = "Get a reference to the value of field `self_managed_configuration` after provisioning.\n"]
    pub fn self_managed_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreMemoryStrategy_Configuration_SelfManagedConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_managed_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_NumberValidation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_value: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_NumberValidation {
    #[doc = "Set the field `max_value`.\n"]
    pub fn set_max_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_value`.\n"]
    pub fn set_min_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_NumberValidation {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_NumberValidation,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_NumberValidation {}

impl BuildBedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_NumberValidation {
    pub fn build(
        self,
    ) -> BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_NumberValidation {
        BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_NumberValidation {
            max_value: core::default::Default::default(),
            min_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_NumberValidationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_NumberValidationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_NumberValidationRef {
        BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_NumberValidationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_NumberValidationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_value` after provisioning.\n"]
    pub fn max_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_value", self.base))
    }

    #[doc = "Get a reference to the value of field `min_value` after provisioning.\n"]
    pub fn min_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_value", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringListValidation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringListValidation {
    #[doc = "Set the field `allowed_values`.\n"]
    pub fn set_allowed_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_values = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\n"]
    pub fn set_max_items(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringListValidation {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringListValidation,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringListValidation {}

impl BuildBedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringListValidation {
    pub fn build(
        self,
    ) -> BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringListValidation {
        BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringListValidation {
            allowed_values: core::default::Default::default(),
            max_items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringListValidationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringListValidationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringListValidationRef {
        BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringListValidationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringListValidationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_values` after provisioning.\n"]
    pub fn allowed_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_values", self.base))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\n"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringValidation {
    allowed_values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringValidation {

}

impl terrars
::ToListMappable for BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringValidation {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringValidation,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringValidation {
    #[doc = ""]
    pub allowed_values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildBedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringValidation {
    pub fn build(
        self,
    ) -> BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringValidation {
        BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringValidation {
            allowed_values: self.allowed_values,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringValidationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringValidationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringValidationRef {
        BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringValidationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringValidationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_values` after provisioning.\n"]
    pub fn allowed_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_values", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_ValidationDynamic {
    number_validation: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_NumberValidation,
        >,
    >,
    string_list_validation: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringListValidation,
        >,
    >,
    string_validation: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringValidation,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_validation: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_NumberValidation,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_list_validation: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringListValidation,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_validation: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringValidation,
        >,
    >,
    dynamic: BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_ValidationDynamic,
}

impl BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation {
    #[doc = "Set the field `number_validation`.\n"]
    pub fn set_number_validation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_NumberValidation,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.number_validation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.number_validation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `string_list_validation`.\n"]
    pub fn set_string_list_validation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringListValidation,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.string_list_validation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.string_list_validation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `string_validation`.\n"]
    pub fn set_string_validation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringValidation,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.string_validation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.string_validation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation {}

impl BuildBedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation {
    pub fn build(
        self,
    ) -> BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation {
        BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation {
            number_validation: core::default::Default::default(),
            string_list_validation: core::default::Default::default(),
            string_validation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_ValidationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_ValidationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_ValidationRef {
        BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_ValidationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_ValidationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `number_validation` after provisioning.\n"]
    pub fn number_validation(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_NumberValidationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.number_validation", self.base))
    }

    #[doc = "Get a reference to the value of field `string_list_validation` after provisioning.\n"]
    pub fn string_list_validation(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringListValidationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_list_validation", self.base))
    }

    #[doc = "Get a reference to the value of field `string_validation` after provisioning.\n"]
    pub fn string_validation(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation_StringValidationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_validation", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfigDynamic {
    validation: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig {
    definition: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    llm_extraction_instruction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation,
        >,
    >,
    dynamic: BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfigDynamic,
}

impl BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig {
    #[doc = "Set the field `llm_extraction_instruction`.\n"]
    pub fn set_llm_extraction_instruction(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.llm_extraction_instruction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `validation`.\n"]
    pub fn set_validation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_Validation,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.validation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.validation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig {
    #[doc = ""]
    pub definition: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig {
    pub fn build(
        self,
    ) -> BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig {
        BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig {
            definition: self.definition,
            llm_extraction_instruction: core::default::Default::default(),
            validation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfigRef {
        BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.definition", self.base))
    }

    #[doc = "Get a reference to the value of field `llm_extraction_instruction` after provisioning.\n"]
    pub fn llm_extraction_instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.llm_extraction_instruction", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig_ValidationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfigDynamic {
    llm_extraction_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    llm_extraction_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig>,
    >,
    dynamic: BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfigDynamic,
}

impl BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig {
    #[doc = "Set the field `llm_extraction_config`.\n"]
    pub fn set_llm_extraction_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.llm_extraction_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.llm_extraction_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig {
    type O =
        terrars::BlockAssignable<BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig {}

impl BuildBedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig {
    pub fn build(self) -> BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig {
        BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig {
            llm_extraction_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfigRef {
        BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `llm_extraction_config` after provisioning.\n"]
    pub fn llm_extraction_config(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig_LlmExtractionConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.llm_extraction_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchemaDynamic {
    extraction_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extraction_type: std::option::Option<terrars::PrimField<std::string::String>>,
    key: terrars::PrimField<std::string::String>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extraction_config: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig>>,
    dynamic: BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchemaDynamic,
}

impl BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema {
    #[doc = "Set the field `extraction_type`.\n"]
    pub fn set_extraction_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.extraction_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `extraction_config`.\n"]
    pub fn set_extraction_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.extraction_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.extraction_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema {
    type O = terrars::BlockAssignable<BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema {
    pub fn build(self) -> BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema {
        BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema {
            extraction_type: core::default::Default::default(),
            key: self.key,
            type_: core::default::Default::default(),
            extraction_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchemaRef {
        BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `extraction_type` after provisioning.\n"]
    pub fn extraction_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.extraction_type", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `extraction_config` after provisioning.\n"]
    pub fn extraction_config(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema_ExtractionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extraction_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreMemoryStrategy_MemoryRecordSchemaDynamic {
    metadata_schema: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_MemoryRecordSchema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_schema: std::option::Option<std::vec::Vec<BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema>>,
    dynamic: BedrockagentcoreMemoryStrategy_MemoryRecordSchemaDynamic,
}

impl BedrockagentcoreMemoryStrategy_MemoryRecordSchema {
    #[doc = "Set the field `metadata_schema`.\n"]
    pub fn set_metadata_schema(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchema>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metadata_schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata_schema = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreMemoryStrategy_MemoryRecordSchema {
    type O = terrars::BlockAssignable<BedrockagentcoreMemoryStrategy_MemoryRecordSchema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_MemoryRecordSchema {}

impl BuildBedrockagentcoreMemoryStrategy_MemoryRecordSchema {
    pub fn build(self) -> BedrockagentcoreMemoryStrategy_MemoryRecordSchema {
        BedrockagentcoreMemoryStrategy_MemoryRecordSchema {
            metadata_schema: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_MemoryRecordSchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreMemoryStrategy_MemoryRecordSchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_MemoryRecordSchemaRef {
        BedrockagentcoreMemoryStrategy_MemoryRecordSchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_MemoryRecordSchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metadata_schema` after provisioning.\n"]
    pub fn metadata_schema(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreMemoryStrategy_MemoryRecordSchema_MetadataSchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_schema", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_ReflectionConfiguration {
    namespace_templates: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreMemoryStrategy_ReflectionConfiguration { }

impl terrars::ToListMappable for BedrockagentcoreMemoryStrategy_ReflectionConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreMemoryStrategy_ReflectionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_ReflectionConfiguration {
    #[doc = ""]
    pub namespace_templates: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildBedrockagentcoreMemoryStrategy_ReflectionConfiguration {
    pub fn build(self) -> BedrockagentcoreMemoryStrategy_ReflectionConfiguration {
        BedrockagentcoreMemoryStrategy_ReflectionConfiguration { namespace_templates: self.namespace_templates }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_ReflectionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreMemoryStrategy_ReflectionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemoryStrategy_ReflectionConfigurationRef {
        BedrockagentcoreMemoryStrategy_ReflectionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_ReflectionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespace_templates` after provisioning.\n"]
    pub fn namespace_templates(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.namespace_templates", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemoryStrategy_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreMemoryStrategy_Timeouts {
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

impl terrars::ToListMappable for BedrockagentcoreMemoryStrategy_Timeouts {
    type O = terrars::BlockAssignable<BedrockagentcoreMemoryStrategy_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemoryStrategy_Timeouts {}

impl BuildBedrockagentcoreMemoryStrategy_Timeouts {
    pub fn build(self) -> BedrockagentcoreMemoryStrategy_Timeouts {
        BedrockagentcoreMemoryStrategy_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemoryStrategy_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreMemoryStrategy_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreMemoryStrategy_TimeoutsRef {
        BedrockagentcoreMemoryStrategy_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemoryStrategy_TimeoutsRef {
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
