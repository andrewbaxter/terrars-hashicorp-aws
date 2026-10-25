#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentFlowData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    execution_role_arn: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    definition: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockagentFlow_Timeouts>,
    dynamic: BedrockagentFlowDynamic,
}

struct BedrockagentFlow_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockagentFlowData_>,
}

#[derive(Clone)]
pub struct BedrockagentFlow(std::rc::Rc<BedrockagentFlow_>);

impl BedrockagentFlow {
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

    #[doc = "Set the field `definition`.\n"]
    pub fn set_definition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentFlow_Definition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.definition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockagentFlow_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> terrars::ListRef<BedrockagentFlow_DefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentFlow_TimeoutsRef {
        BedrockagentFlow_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BedrockagentFlow {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockagentFlow { }

impl terrars::ToListMappable for BedrockagentFlow {
    type O = terrars::ListRef<BedrockagentFlowRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockagentFlow_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrockagent_flow".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentFlow {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub execution_role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockagentFlow {
        let out = BedrockagentFlow(std::rc::Rc::new(BedrockagentFlow_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockagentFlowData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                customer_encryption_key_arn: core::default::Default::default(),
                description: core::default::Default::default(),
                execution_role_arn: self.execution_role_arn,
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                definition: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentFlowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlowRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockagentFlowRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> terrars::ListRef<BedrockagentFlow_DefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentFlow_TimeoutsRef {
        BedrockagentFlow_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlowDynamic {
    definition: std::option::Option<terrars::DynamicBlock<BedrockagentFlow_Definition>>,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Connection_Configuration_Conditional {
    condition: terrars::PrimField<std::string::String>,
}

impl BedrockagentFlow_Definition_Connection_Configuration_Conditional { }

impl terrars::ToListMappable for BedrockagentFlow_Definition_Connection_Configuration_Conditional {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Connection_Configuration_Conditional>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Connection_Configuration_Conditional {
    #[doc = ""]
    pub condition: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Connection_Configuration_Conditional {
    pub fn build(self) -> BedrockagentFlow_Definition_Connection_Configuration_Conditional {
        BedrockagentFlow_Definition_Connection_Configuration_Conditional { condition: self.condition }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Connection_Configuration_ConditionalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Connection_Configuration_ConditionalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Connection_Configuration_ConditionalRef {
        BedrockagentFlow_Definition_Connection_Configuration_ConditionalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Connection_Configuration_ConditionalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.condition", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Connection_Configuration_Data {
    source_output: terrars::PrimField<std::string::String>,
    target_input: terrars::PrimField<std::string::String>,
}

impl BedrockagentFlow_Definition_Connection_Configuration_Data { }

impl terrars::ToListMappable for BedrockagentFlow_Definition_Connection_Configuration_Data {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Connection_Configuration_Data>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Connection_Configuration_Data {
    #[doc = ""]
    pub source_output: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_input: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Connection_Configuration_Data {
    pub fn build(self) -> BedrockagentFlow_Definition_Connection_Configuration_Data {
        BedrockagentFlow_Definition_Connection_Configuration_Data {
            source_output: self.source_output,
            target_input: self.target_input,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Connection_Configuration_DataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Connection_Configuration_DataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Connection_Configuration_DataRef {
        BedrockagentFlow_Definition_Connection_Configuration_DataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Connection_Configuration_DataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_output` after provisioning.\n"]
    pub fn source_output(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_output", self.base))
    }

    #[doc = "Get a reference to the value of field `target_input` after provisioning.\n"]
    pub fn target_input(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_input", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlow_Definition_Connection_ConfigurationDynamic {
    conditional: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Connection_Configuration_Conditional>>,
    data: std::option::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Connection_Configuration_Data>>,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Connection_Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditional: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Connection_Configuration_Conditional>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Connection_Configuration_Data>>,
    dynamic: BedrockagentFlow_Definition_Connection_ConfigurationDynamic,
}

impl BedrockagentFlow_Definition_Connection_Configuration {
    #[doc = "Set the field `conditional`.\n"]
    pub fn set_conditional(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentFlow_Definition_Connection_Configuration_Conditional>,
                    >,
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

    #[doc = "Set the field `data`.\n"]
    pub fn set_data(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentFlow_Definition_Connection_Configuration_Data>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentFlow_Definition_Connection_Configuration {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Connection_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Connection_Configuration {}

impl BuildBedrockagentFlow_Definition_Connection_Configuration {
    pub fn build(self) -> BedrockagentFlow_Definition_Connection_Configuration {
        BedrockagentFlow_Definition_Connection_Configuration {
            conditional: core::default::Default::default(),
            data: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Connection_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Connection_ConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Connection_ConfigurationRef {
        BedrockagentFlow_Definition_Connection_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Connection_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conditional` after provisioning.\n"]
    pub fn conditional(&self) -> terrars::ListRef<BedrockagentFlow_Definition_Connection_Configuration_ConditionalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditional", self.base))
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> terrars::ListRef<BedrockagentFlow_Definition_Connection_Configuration_DataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlow_Definition_ConnectionDynamic {
    configuration: std::option::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Connection_Configuration>>,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Connection {
    name: terrars::PrimField<std::string::String>,
    source: terrars::PrimField<std::string::String>,
    target: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Connection_Configuration>>,
    dynamic: BedrockagentFlow_Definition_ConnectionDynamic,
}

impl BedrockagentFlow_Definition_Connection {
    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentFlow_Definition_Connection_Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentFlow_Definition_Connection {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Connection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Connection {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Connection {
    pub fn build(self) -> BedrockagentFlow_Definition_Connection {
        BedrockagentFlow_Definition_Connection {
            name: self.name,
            source: self.source,
            target: self.target,
            type_: self.type_,
            configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_ConnectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_ConnectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentFlow_Definition_ConnectionRef {
        BedrockagentFlow_Definition_ConnectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_ConnectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<BedrockagentFlow_Definition_Connection_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Agent {
    agent_alias_arn: terrars::PrimField<std::string::String>,
}

impl BedrockagentFlow_Definition_Node_Configuration_Agent { }

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Agent {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Agent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Agent {
    #[doc = ""]
    pub agent_alias_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Agent {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_Agent {
        BedrockagentFlow_Definition_Node_Configuration_Agent { agent_alias_arn: self.agent_alias_arn }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_AgentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_AgentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_AgentRef {
        BedrockagentFlow_Definition_Node_Configuration_AgentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_AgentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_alias_arn` after provisioning.\n"]
    pub fn agent_alias_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_alias_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Collector {}

impl BedrockagentFlow_Definition_Node_Configuration_Collector { }

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Collector {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Collector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Collector {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Collector {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_Collector {
        BedrockagentFlow_Definition_Node_Configuration_Collector {}
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_CollectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_CollectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_CollectorRef {
        BedrockagentFlow_Definition_Node_Configuration_CollectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_CollectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Condition_Condition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl BedrockagentFlow_Definition_Node_Configuration_Condition_Condition {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Condition_Condition {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Condition_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Condition_Condition {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Condition_Condition {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_Condition_Condition {
        BedrockagentFlow_Definition_Node_Configuration_Condition_Condition {
            expression: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Condition_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_Condition_ConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Condition_ConditionRef {
        BedrockagentFlow_Definition_Node_Configuration_Condition_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Condition_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlow_Definition_Node_Configuration_ConditionDynamic {
    condition: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_Condition_Condition>>,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Condition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_Condition_Condition>>,
    dynamic: BedrockagentFlow_Definition_Node_Configuration_ConditionDynamic,
}

impl BedrockagentFlow_Definition_Node_Configuration_Condition {
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Condition_Condition>,
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
}

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Condition {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Condition {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Condition {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_Condition {
        BedrockagentFlow_Definition_Node_Configuration_Condition {
            condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_ConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_ConditionRef {
        BedrockagentFlow_Definition_Node_Configuration_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_Condition_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_InlineCode {
    code: terrars::PrimField<std::string::String>,
    language: terrars::PrimField<std::string::String>,
}

impl BedrockagentFlow_Definition_Node_Configuration_InlineCode { }

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_InlineCode {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_InlineCode>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_InlineCode {
    #[doc = ""]
    pub code: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub language: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Node_Configuration_InlineCode {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_InlineCode {
        BedrockagentFlow_Definition_Node_Configuration_InlineCode {
            code: self.code,
            language: self.language,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_InlineCodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_InlineCodeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_InlineCodeRef {
        BedrockagentFlow_Definition_Node_Configuration_InlineCodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_InlineCodeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `language` after provisioning.\n"]
    pub fn language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Input {}

impl BedrockagentFlow_Definition_Node_Configuration_Input { }

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Input {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Input>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Input {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Input {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_Input {
        BedrockagentFlow_Definition_Node_Configuration_Input {}
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_InputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_InputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_InputRef {
        BedrockagentFlow_Definition_Node_Configuration_InputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_InputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Iterator {}

impl BedrockagentFlow_Definition_Node_Configuration_Iterator { }

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Iterator {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Iterator>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Iterator {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Iterator {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_Iterator {
        BedrockagentFlow_Definition_Node_Configuration_Iterator {}
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_IteratorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_IteratorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_IteratorRef {
        BedrockagentFlow_Definition_Node_Configuration_IteratorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_IteratorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_GuardrailConfiguration {
    guardrail_identifier: terrars::PrimField<std::string::String>,
    guardrail_version: terrars::PrimField<std::string::String>,
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_GuardrailConfiguration { }

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Prompt_GuardrailConfiguration {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Prompt_GuardrailConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_GuardrailConfiguration {
    #[doc = ""]
    pub guardrail_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub guardrail_version: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_GuardrailConfiguration {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_GuardrailConfiguration {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_GuardrailConfiguration {
            guardrail_identifier: self.guardrail_identifier,
            guardrail_version: self.guardrail_version,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_GuardrailConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_Prompt_GuardrailConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_GuardrailConfigurationRef {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_GuardrailConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_GuardrailConfigurationRef {
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
pub struct BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration_Text {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_tokens: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stop_sequences: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temperature: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_p: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration_Text {
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

impl terrars
::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration_Text {
    type O =
        terrars
        ::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration_Text>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration_Text {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration_Text {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration_Text {
        BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration_Text {
            max_tokens: core::default::Default::default(),
            stop_sequences: core::default::Default::default(),
            temperature: core::default::Default::default(),
            top_p: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration_TextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration_TextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration_TextRef {
        BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration_TextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration_TextRef {
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
struct BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfigurationDynamic {
    text: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration_Text>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std
    ::option
    ::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration_Text>>,
    dynamic: BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfigurationDynamic,
}

impl BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration_Text,
                        >,
                    >,
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

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration {
    type O =
        terrars
        ::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration {
        BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration {
            text: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfigurationRef {
        BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration_TextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlow_Definition_Node_Configuration_KnowledgeBaseDynamic {
    guardrail_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_Prompt_GuardrailConfiguration>>,
    inference_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase {
    knowledge_base_id: terrars::PrimField<std::string::String>,
    model_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_of_results: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guardrail_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_Prompt_GuardrailConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration>>,
    dynamic: BedrockagentFlow_Definition_Node_Configuration_KnowledgeBaseDynamic,
}

impl BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase {
    #[doc = "Set the field `number_of_results`.\n"]
    pub fn set_number_of_results(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.number_of_results = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guardrail_configuration`.\n"]
    pub fn set_guardrail_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_GuardrailConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.guardrail_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.guardrail_configuration = std::option::Option::Some(d);
            },
        }
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
                        ::BlockAssignable<
                            BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration,
                        >,
                    >,
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
}

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_KnowledgeBase {
    #[doc = ""]
    pub knowledge_base_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub model_id: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Node_Configuration_KnowledgeBase {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase {
        BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase {
            knowledge_base_id: self.knowledge_base_id,
            model_id: self.model_id,
            number_of_results: core::default::Default::default(),
            guardrail_configuration: core::default::Default::default(),
            inference_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_KnowledgeBaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_KnowledgeBaseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_KnowledgeBaseRef {
        BedrockagentFlow_Definition_Node_Configuration_KnowledgeBaseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_KnowledgeBaseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `knowledge_base_id` after provisioning.\n"]
    pub fn knowledge_base_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.knowledge_base_id", self.base))
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
    }

    #[doc = "Get a reference to the value of field `number_of_results` after provisioning.\n"]
    pub fn number_of_results(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_results", self.base))
    }

    #[doc = "Get a reference to the value of field `guardrail_configuration` after provisioning.\n"]
    pub fn guardrail_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_Prompt_GuardrailConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guardrail_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `inference_configuration` after provisioning.\n"]
    pub fn inference_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inference_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_LambdaFunction {
    lambda_arn: terrars::PrimField<std::string::String>,
}

impl BedrockagentFlow_Definition_Node_Configuration_LambdaFunction { }

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_LambdaFunction {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_LambdaFunction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_LambdaFunction {
    #[doc = ""]
    pub lambda_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Node_Configuration_LambdaFunction {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_LambdaFunction {
        BedrockagentFlow_Definition_Node_Configuration_LambdaFunction { lambda_arn: self.lambda_arn }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_LambdaFunctionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_LambdaFunctionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_LambdaFunctionRef {
        BedrockagentFlow_Definition_Node_Configuration_LambdaFunctionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_LambdaFunctionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Lex {
    bot_alias_arn: terrars::PrimField<std::string::String>,
    locale_id: terrars::PrimField<std::string::String>,
}

impl BedrockagentFlow_Definition_Node_Configuration_Lex { }

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Lex {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Lex>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Lex {
    #[doc = ""]
    pub bot_alias_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub locale_id: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Lex {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_Lex {
        BedrockagentFlow_Definition_Node_Configuration_Lex {
            bot_alias_arn: self.bot_alias_arn,
            locale_id: self.locale_id,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_LexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_LexRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_LexRef {
        BedrockagentFlow_Definition_Node_Configuration_LexRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_LexRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bot_alias_arn` after provisioning.\n"]
    pub fn bot_alias_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bot_alias_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `locale_id` after provisioning.\n"]
    pub fn locale_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locale_id", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Output {}

impl BedrockagentFlow_Definition_Node_Configuration_Output { }

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Output {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Output>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Output {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Output {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_Output {
        BedrockagentFlow_Definition_Node_Configuration_Output {}
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_OutputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_OutputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_OutputRef {
        BedrockagentFlow_Definition_Node_Configuration_OutputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_OutputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_InputVariable {
    name: terrars::PrimField<std::string::String>,
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_InputVariable {

}

impl terrars
::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_InputVariable {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_InputVariable,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_InputVariable {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_InputVariable {
    pub fn build(
        self,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_InputVariable {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_InputVariable {
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_InputVariableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_InputVariableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_InputVariableRef {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_InputVariableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_InputVariableRef {
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
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePoint {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePoint {

}

impl terrars
::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePoint {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePoint,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePoint {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePoint {
    pub fn build(
        self,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePoint {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePoint {
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePointRef {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePointRef {
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
struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message_ContentDynamic {
    cache_point: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePoint,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message_Content {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_point: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePoint,
        >,
    >,
    dynamic: BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message_ContentDynamic,
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message_Content {
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
                        terrars
                        ::BlockAssignable<
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePoint,
                        >,
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

impl terrars
::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message_Content {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message_Content,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message_Content {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message_Content {
    pub fn build(
        self,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message_Content {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message_Content {
            text: core::default::Default::default(),
            cache_point: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message_ContentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message_ContentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message_ContentRef {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message_ContentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message_ContentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_point` after provisioning.\n"]
    pub fn cache_point(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePointRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_point", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_MessageDynamic {
    content: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message_Content,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message {
    role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message_Content,
        >,
    >,
    dynamic: BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_MessageDynamic,
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message {
    #[doc = "Set the field `content`.\n"]
    pub fn set_content(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message_Content,
                        >,
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

impl terrars
::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message {
    #[doc = ""]
    pub role: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message {
    pub fn build(
        self,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message {
            role: self.role,
            content: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_MessageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_MessageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_MessageRef {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_MessageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_MessageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message_ContentRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_SystemDynamic {
    cache_point: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePoint,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_System {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_point: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePoint,
        >,
    >,
    dynamic: BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_SystemDynamic,
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_System {
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
                        terrars
                        ::BlockAssignable<
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePoint,
                        >,
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

impl terrars
::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_System {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_System,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_System {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_System {
    pub fn build(
        self,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_System {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_System {
            text: core::default::Default::default(),
            cache_point: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_SystemRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_SystemRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_SystemRef {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_SystemRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_SystemRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_point` after provisioning.\n"]
    pub fn cache_point(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePointRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_point", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema {
    #[doc = "Set the field `json`.\n"]
    pub fn set_json(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.json = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema {
    pub fn build(
        self,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema {
            json: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchemaRef {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchemaRef {
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
struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpecDynamic {
    input_schema: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_schema: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema,
        >,
    >,
    dynamic: BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpecDynamic,
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec {
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
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchema,
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
::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec {
    pub fn build(
        self,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec {
            description: core::default::Default::default(),
            name: self.name,
            input_schema: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpecRef {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpecRef {
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
    ::ListRef<
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec_InputSchemaRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_schema", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolDynamic {
    cache_point: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePoint,
        >,
    >,
    tool_spec: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_point: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePoint,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_spec: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec,
        >,
    >,
    dynamic: BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolDynamic,
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool {
    #[doc = "Set the field `cache_point`.\n"]
    pub fn set_cache_point(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePoint,
                        >,
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
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpec,
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

impl terrars
::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool {
    pub fn build(
        self,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool {
            cache_point: core::default::Default::default(),
            tool_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolRef {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_point` after provisioning.\n"]
    pub fn cache_point(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePointRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_point", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_spec` after provisioning.\n"]
    pub fn tool_spec(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool_ToolSpecRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any {}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any {

}

impl terrars
::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any {
    pub fn build(
        self,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any {}
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AnyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AnyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AnyRef {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AnyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AnyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto {}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto {

}

impl terrars
::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto {
    pub fn build(
        self,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto {}
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AutoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AutoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AutoRef {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AutoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AutoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool {
    name: terrars::PrimField<std::string::String>,
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool {

}

impl terrars
::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool {
    pub fn build(
        self,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool {
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_ToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_ToolRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_ToolRef {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_ToolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_ToolRef {
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
struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoiceDynamic {
    any: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any,
        >,
    >,
    auto: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto,
        >,
    >,
    tool: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    any: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool,
        >,
    >,
    dynamic: BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoiceDynamic,
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice {
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
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Any,
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
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Auto,
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
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_Tool,
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

impl terrars
::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice {
    pub fn build(
        self,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice {
            any: core::default::Default::default(),
            auto: core::default::Default::default(),
            tool: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoiceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoiceRef {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoiceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoiceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `any` after provisioning.\n"]
    pub fn any(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AnyRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.any", self.base))
    }

    #[doc = "Get a reference to the value of field `auto` after provisioning.\n"]
    pub fn auto(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_AutoRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto", self.base))
    }

    #[doc = "Get a reference to the value of field `tool` after provisioning.\n"]
    pub fn tool(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice_ToolRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfigurationDynamic {
    tool: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool,
        >,
    >,
    tool_choice: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_choice: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice,
        >,
    >,
    dynamic: BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfigurationDynamic,
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration {
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
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_Tool,
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
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoice,
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

impl terrars
::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration {
            tool: core::default::Default::default(),
            tool_choice: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfigurationRef {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tool` after provisioning.\n"]
    pub fn tool(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_choice` after provisioning.\n"]
    pub fn tool_choice(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration_ToolChoiceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_choice", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_ChatDynamic {
    input_variable: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_InputVariable,
        >,
    >,
    message: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message,
        >,
    >,
    system: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_System,
        >,
    >,
    tool_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_variable: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_InputVariable,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_System,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration,
        >,
    >,
    dynamic: BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_ChatDynamic,
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat {
    #[doc = "Set the field `input_variable`.\n"]
    pub fn set_input_variable(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_InputVariable,
                        >,
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
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_Message,
                        >,
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

    #[doc = "Set the field `system`.\n"]
    pub fn set_system(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_System,
                        >,
                    >,
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
                        ::BlockAssignable<
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfiguration,
                        >,
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

impl terrars
::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat {
    pub fn build(
        self,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat {
            input_variable: core::default::Default::default(),
            message: core::default::Default::default(),
            system: core::default::Default::default(),
            tool_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_ChatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_ChatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_ChatRef {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_ChatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_ChatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_variable` after provisioning.\n"]
    pub fn input_variable(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_InputVariableRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_variable", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_MessageRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `system` after provisioning.\n"]
    pub fn system(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_SystemRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.system", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_configuration` after provisioning.\n"]
    pub fn tool_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_ToolConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_TextDynamic {
    cache_point: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePoint,
        >,
    >,
    input_variable: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_InputVariable,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text {
    text: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_point: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePoint,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_variable: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_InputVariable,
        >,
    >,
    dynamic: BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_TextDynamic,
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text {
    #[doc = "Set the field `cache_point`.\n"]
    pub fn set_cache_point(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePoint,
                        >,
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
                        terrars
                        ::BlockAssignable<
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_InputVariable,
                        >,
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

impl terrars
::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text {
    #[doc = ""]
    pub text: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text {
    pub fn build(
        self,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text {
            text: self.text,
            cache_point: core::default::Default::default(),
            input_variable: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_TextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_TextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_TextRef {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_TextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_TextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_point` after provisioning.\n"]
    pub fn cache_point(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text_CachePointRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_point", self.base))
    }

    #[doc = "Get a reference to the value of field `input_variable` after provisioning.\n"]
    pub fn input_variable(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat_InputVariableRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_variable", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfigurationDynamic {
    chat: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat,
        >,
    >,
    text: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chat: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text,
        >,
    >,
    dynamic: BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfigurationDynamic,
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration {
    #[doc = "Set the field `chat`.\n"]
    pub fn set_chat(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Chat,
                        >,
                    >,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_Text,
                        >,
                    >,
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

impl terrars
::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration {
            chat: core::default::Default::default(),
            text: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfigurationRef {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `chat` after provisioning.\n"]
    pub fn chat(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_ChatRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chat", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration_TextRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_InlineDynamic {
    inference_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration>,
    >,
    template_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_model_request_fields: std::option::Option<terrars::PrimField<std::string::String>>,
    model_id: terrars::PrimField<std::string::String>,
    template_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration>,
    >,
    dynamic: BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_InlineDynamic,
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline {
    #[doc = "Set the field `additional_model_request_fields`.\n"]
    pub fn set_additional_model_request_fields(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.additional_model_request_fields = std::option::Option::Some(v.into());
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
                        ::BlockAssignable<
                            BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfiguration,
                        >,
                    >,
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

    #[doc = "Set the field `template_configuration`.\n"]
    pub fn set_template_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfiguration,
                        >,
                    >,
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

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline {
    type O =
        terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline {
    #[doc = ""]
    pub model_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub template_type: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline {
            additional_model_request_fields: core::default::Default::default(),
            model_id: self.model_id,
            template_type: self.template_type,
            inference_configuration: core::default::Default::default(),
            template_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_InlineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_InlineRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_InlineRef {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_InlineRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_InlineRef {
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

    #[doc = "Get a reference to the value of field `template_type` after provisioning.\n"]
    pub fn template_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template_type", self.base))
    }

    #[doc = "Get a reference to the value of field `inference_configuration` after provisioning.\n"]
    pub fn inference_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase_InferenceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inference_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `template_configuration` after provisioning.\n"]
    pub fn template_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline_TemplateConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Resource {
    prompt_arn: terrars::PrimField<std::string::String>,
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Resource { }

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Resource {
    type O =
        terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Resource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Resource {
    #[doc = ""]
    pub prompt_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Resource {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Resource {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Resource {
            prompt_arn: self.prompt_arn,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_ResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_ResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_ResourceRef {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_ResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_ResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prompt_arn` after provisioning.\n"]
    pub fn prompt_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prompt_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfigurationDynamic {
    inline: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline>>,
    resource: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Resource>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inline: std
    ::option
    ::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std
    ::option
    ::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Resource>>,
    dynamic: BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfigurationDynamic,
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration {
    #[doc = "Set the field `inline`.\n"]
    pub fn set_inline(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Inline,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inline = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inline = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_Resource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration {
            inline: core::default::Default::default(),
            resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfigurationRef {
        BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `inline` after provisioning.\n"]
    pub fn inline(
        &self,
    ) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_InlineRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inline", self.base))
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(
        &self,
    ) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration_ResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlow_Definition_Node_Configuration_PromptDynamic {
    guardrail_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_Prompt_GuardrailConfiguration>>,
    source_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration>>,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Prompt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guardrail_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_Prompt_GuardrailConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration>>,
    dynamic: BedrockagentFlow_Definition_Node_Configuration_PromptDynamic,
}

impl BedrockagentFlow_Definition_Node_Configuration_Prompt {
    #[doc = "Set the field `guardrail_configuration`.\n"]
    pub fn set_guardrail_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentFlow_Definition_Node_Configuration_Prompt_GuardrailConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.guardrail_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.guardrail_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_configuration`.\n"]
    pub fn set_source_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Prompt {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Prompt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Prompt {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Prompt {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_Prompt {
        BedrockagentFlow_Definition_Node_Configuration_Prompt {
            guardrail_configuration: core::default::Default::default(),
            source_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_PromptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_PromptRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_PromptRef {
        BedrockagentFlow_Definition_Node_Configuration_PromptRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_PromptRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `guardrail_configuration` after provisioning.\n"]
    pub fn guardrail_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_Prompt_GuardrailConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guardrail_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `source_configuration` after provisioning.\n"]
    pub fn source_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_Prompt_SourceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration_S3 {
    bucket_name: terrars::PrimField<std::string::String>,
}

impl BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration_S3 { }

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration_S3 {
    type O =
        terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration_S3 {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration_S3 {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration_S3 {
        BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration_S3 {
            bucket_name: self.bucket_name,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration_S3Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration_S3Ref {
        BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration_S3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfigurationDynamic {
    s3: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration_S3>>,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std
    ::option
    ::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration_S3>>,
    dynamic: BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfigurationDynamic,
}

impl BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration_S3,
                        >,
                    >,
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

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration {
        BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration {
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfigurationRef {
        BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(
        &self,
    ) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlow_Definition_Node_Configuration_RetrievalDynamic {
    service_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration>>,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Retrieval {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration>>,
    dynamic: BedrockagentFlow_Definition_Node_Configuration_RetrievalDynamic,
}

impl BedrockagentFlow_Definition_Node_Configuration_Retrieval {
    #[doc = "Set the field `service_configuration`.\n"]
    pub fn set_service_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Retrieval {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Retrieval>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Retrieval {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Retrieval {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_Retrieval {
        BedrockagentFlow_Definition_Node_Configuration_Retrieval {
            service_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_RetrievalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_RetrievalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_RetrievalRef {
        BedrockagentFlow_Definition_Node_Configuration_RetrievalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_RetrievalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `service_configuration` after provisioning.\n"]
    pub fn service_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlow_Definition_Node_Configuration_StorageDynamic {
    service_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration>>,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration_Storage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration>>,
    dynamic: BedrockagentFlow_Definition_Node_Configuration_StorageDynamic,
}

impl BedrockagentFlow_Definition_Node_Configuration_Storage {
    #[doc = "Set the field `service_configuration`.\n"]
    pub fn set_service_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration_Storage {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Storage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration_Storage {}

impl BuildBedrockagentFlow_Definition_Node_Configuration_Storage {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration_Storage {
        BedrockagentFlow_Definition_Node_Configuration_Storage {
            service_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_Configuration_StorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_Configuration_StorageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_Configuration_StorageRef {
        BedrockagentFlow_Definition_Node_Configuration_StorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_Configuration_StorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `service_configuration` after provisioning.\n"]
    pub fn service_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_Storage_ServiceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlow_Definition_Node_ConfigurationDynamic {
    agent: std::option::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_Agent>>,
    collector: std::option::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_Collector>>,
    condition: std::option::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_Condition>>,
    inline_code: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_InlineCode>>,
    input: std::option::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_Input>>,
    iterator: std::option::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_Iterator>>,
    knowledge_base: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase>>,
    lambda_function: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_LambdaFunction>>,
    lex: std::option::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_Lex>>,
    output: std::option::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_Output>>,
    prompt: std::option::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_Prompt>>,
    retrieval: std::option::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_Retrieval>>,
    storage: std::option::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration_Storage>>,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_Agent>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collector: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_Collector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_Condition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inline_code: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_InlineCode>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_Input>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iterator: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_Iterator>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    knowledge_base: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_LambdaFunction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lex: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_Lex>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_Output>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_Prompt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retrieval: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_Retrieval>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration_Storage>>,
    dynamic: BedrockagentFlow_Definition_Node_ConfigurationDynamic,
}

impl BedrockagentFlow_Definition_Node_Configuration {
    #[doc = "Set the field `agent`.\n"]
    pub fn set_agent(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Agent>>,
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

    #[doc = "Set the field `collector`.\n"]
    pub fn set_collector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Collector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.collector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.collector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Condition>>,
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

    #[doc = "Set the field `inline_code`.\n"]
    pub fn set_inline_code(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_InlineCode>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inline_code = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inline_code = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input`.\n"]
    pub fn set_input(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Input>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `iterator`.\n"]
    pub fn set_iterator(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Iterator>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.iterator = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.iterator = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `knowledge_base`.\n"]
    pub fn set_knowledge_base(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_KnowledgeBase>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.knowledge_base = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.knowledge_base = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_function`.\n"]
    pub fn set_lambda_function(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_LambdaFunction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda_function = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_function = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lex`.\n"]
    pub fn set_lex(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Lex>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lex = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lex = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output`.\n"]
    pub fn set_output(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Output>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `prompt`.\n"]
    pub fn set_prompt(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Prompt>>,
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

    #[doc = "Set the field `retrieval`.\n"]
    pub fn set_retrieval(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Retrieval>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retrieval = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retrieval = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage`.\n"]
    pub fn set_storage(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration_Storage>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Configuration {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Configuration {}

impl BuildBedrockagentFlow_Definition_Node_Configuration {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Configuration {
        BedrockagentFlow_Definition_Node_Configuration {
            agent: core::default::Default::default(),
            collector: core::default::Default::default(),
            condition: core::default::Default::default(),
            inline_code: core::default::Default::default(),
            input: core::default::Default::default(),
            iterator: core::default::Default::default(),
            knowledge_base: core::default::Default::default(),
            lambda_function: core::default::Default::default(),
            lex: core::default::Default::default(),
            output: core::default::Default::default(),
            prompt: core::default::Default::default(),
            retrieval: core::default::Default::default(),
            storage: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_ConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentFlow_Definition_Node_ConfigurationRef {
        BedrockagentFlow_Definition_Node_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent` after provisioning.\n"]
    pub fn agent(&self) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_AgentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent", self.base))
    }

    #[doc = "Get a reference to the value of field `collector` after provisioning.\n"]
    pub fn collector(&self) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_CollectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.collector", self.base))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc = "Get a reference to the value of field `inline_code` after provisioning.\n"]
    pub fn inline_code(&self) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_InlineCodeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inline_code", self.base))
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_InputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input", self.base))
    }

    #[doc = "Get a reference to the value of field `iterator` after provisioning.\n"]
    pub fn iterator(&self) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_IteratorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iterator", self.base))
    }

    #[doc = "Get a reference to the value of field `knowledge_base` after provisioning.\n"]
    pub fn knowledge_base(&self) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_KnowledgeBaseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.knowledge_base", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_function` after provisioning.\n"]
    pub fn lambda_function(&self) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_LambdaFunctionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_function", self.base))
    }

    #[doc = "Get a reference to the value of field `lex` after provisioning.\n"]
    pub fn lex(&self) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_LexRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lex", self.base))
    }

    #[doc = "Get a reference to the value of field `output` after provisioning.\n"]
    pub fn output(&self) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_OutputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output", self.base))
    }

    #[doc = "Get a reference to the value of field `prompt` after provisioning.\n"]
    pub fn prompt(&self) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_PromptRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prompt", self.base))
    }

    #[doc = "Get a reference to the value of field `retrieval` after provisioning.\n"]
    pub fn retrieval(&self) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_RetrievalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retrieval", self.base))
    }

    #[doc = "Get a reference to the value of field `storage` after provisioning.\n"]
    pub fn storage(&self) -> terrars::ListRef<BedrockagentFlow_Definition_Node_Configuration_StorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Input {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    category: std::option::Option<terrars::PrimField<std::string::String>>,
    expression: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl BedrockagentFlow_Definition_Node_Input {
    #[doc = "Set the field `category`.\n"]
    pub fn set_category(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.category = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Input {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Input>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Input {
    #[doc = ""]
    pub expression: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Node_Input {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Input {
        BedrockagentFlow_Definition_Node_Input {
            category: core::default::Default::default(),
            expression: self.expression,
            name: self.name,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_InputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_InputRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentFlow_Definition_Node_InputRef {
        BedrockagentFlow_Definition_Node_InputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_InputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
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

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node_Output {
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl BedrockagentFlow_Definition_Node_Output { }

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node_Output {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Output>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node_Output {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Node_Output {
    pub fn build(self) -> BedrockagentFlow_Definition_Node_Output {
        BedrockagentFlow_Definition_Node_Output {
            name: self.name,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_Node_OutputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_Node_OutputRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentFlow_Definition_Node_OutputRef {
        BedrockagentFlow_Definition_Node_OutputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_Node_OutputRef {
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
}

#[derive(Serialize, Default)]
struct BedrockagentFlow_Definition_NodeDynamic {
    configuration: std::option::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Configuration>>,
    input: std::option::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Input>>,
    output: std::option::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node_Output>>,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition_Node {
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Input>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Node_Output>>,
    dynamic: BedrockagentFlow_Definition_NodeDynamic,
}

impl BedrockagentFlow_Definition_Node {
    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input`.\n"]
    pub fn set_input(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Input>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output`.\n"]
    pub fn set_output(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentFlow_Definition_Node_Output>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentFlow_Definition_Node {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition_Node>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition_Node {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentFlow_Definition_Node {
    pub fn build(self) -> BedrockagentFlow_Definition_Node {
        BedrockagentFlow_Definition_Node {
            name: self.name,
            type_: self.type_,
            configuration: core::default::Default::default(),
            input: core::default::Default::default(),
            output: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_Definition_NodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_Definition_NodeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentFlow_Definition_NodeRef {
        BedrockagentFlow_Definition_NodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_Definition_NodeRef {
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

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<BedrockagentFlow_Definition_Node_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> terrars::ListRef<BedrockagentFlow_Definition_Node_InputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input", self.base))
    }

    #[doc = "Get a reference to the value of field `output` after provisioning.\n"]
    pub fn output(&self) -> terrars::ListRef<BedrockagentFlow_Definition_Node_OutputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentFlow_DefinitionDynamic {
    connection: std::option::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Connection>>,
    node: std::option::Option<terrars::DynamicBlock<BedrockagentFlow_Definition_Node>>,
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Definition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Connection>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node: std::option::Option<std::vec::Vec<BedrockagentFlow_Definition_Node>>,
    dynamic: BedrockagentFlow_DefinitionDynamic,
}

impl BedrockagentFlow_Definition {
    #[doc = "Set the field `connection`.\n"]
    pub fn set_connection(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentFlow_Definition_Connection>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.connection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.connection = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node`.\n"]
    pub fn set_node(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentFlow_Definition_Node>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentFlow_Definition {
    type O = terrars::BlockAssignable<BedrockagentFlow_Definition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Definition {}

impl BuildBedrockagentFlow_Definition {
    pub fn build(self) -> BedrockagentFlow_Definition {
        BedrockagentFlow_Definition {
            connection: core::default::Default::default(),
            node: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_DefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_DefinitionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentFlow_DefinitionRef {
        BedrockagentFlow_DefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_DefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection` after provisioning.\n"]
    pub fn connection(&self) -> terrars::ListRef<BedrockagentFlow_Definition_ConnectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection", self.base))
    }

    #[doc = "Get a reference to the value of field `node` after provisioning.\n"]
    pub fn node(&self) -> terrars::ListRef<BedrockagentFlow_Definition_NodeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentFlow_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentFlow_Timeouts {
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

impl terrars::ToListMappable for BedrockagentFlow_Timeouts {
    type O = terrars::BlockAssignable<BedrockagentFlow_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentFlow_Timeouts {}

impl BuildBedrockagentFlow_Timeouts {
    pub fn build(self) -> BedrockagentFlow_Timeouts {
        BedrockagentFlow_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentFlow_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentFlow_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentFlow_TimeoutsRef {
        BedrockagentFlow_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentFlow_TimeoutsRef {
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
