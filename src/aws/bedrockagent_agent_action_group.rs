#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentAgentActionGroupData_ {
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
    action_group_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_group_state: std::option::Option<terrars::PrimField<std::string::String>>,
    agent_id: terrars::PrimField<std::string::String>,
    agent_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent_action_group_signature: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prepare_agent: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_resource_in_use_check: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_group_executor: std::option::Option<std::vec::Vec<BedrockagentAgentActionGroup_ActionGroupExecutor>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_schema: std::option::Option<std::vec::Vec<BedrockagentAgentActionGroup_ApiSchema>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function_schema: std::option::Option<std::vec::Vec<BedrockagentAgentActionGroup_FunctionSchema>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockagentAgentActionGroup_Timeouts>,
    dynamic: BedrockagentAgentActionGroupDynamic,
}

struct BedrockagentAgentActionGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockagentAgentActionGroupData_>,
}

#[derive(Clone)]
pub struct BedrockagentAgentActionGroup(std::rc::Rc<BedrockagentAgentActionGroup_>);

impl BedrockagentAgentActionGroup {
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

    #[doc = "Set the field `action_group_state`.\n"]
    pub fn set_action_group_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().action_group_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parent_action_group_signature`.\n"]
    pub fn set_parent_action_group_signature(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().parent_action_group_signature = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prepare_agent`.\n"]
    pub fn set_prepare_agent(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().prepare_agent = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `action_group_executor`.\n"]
    pub fn set_action_group_executor(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentAgentActionGroup_ActionGroupExecutor>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action_group_executor = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action_group_executor = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `api_schema`.\n"]
    pub fn set_api_schema(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentAgentActionGroup_ApiSchema>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().api_schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.api_schema = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `function_schema`.\n"]
    pub fn set_function_schema(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentAgentActionGroup_FunctionSchema>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().function_schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.function_schema = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockagentAgentActionGroup_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `action_group_id` after provisioning.\n"]
    pub fn action_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action_group_name` after provisioning.\n"]
    pub fn action_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action_group_state` after provisioning.\n"]
    pub fn action_group_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_group_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_action_group_signature` after provisioning.\n"]
    pub fn parent_action_group_signature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_action_group_signature", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `prepare_agent` after provisioning.\n"]
    pub fn prepare_agent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prepare_agent", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `action_group_executor` after provisioning.\n"]
    pub fn action_group_executor(&self) -> terrars::ListRef<BedrockagentAgentActionGroup_ActionGroupExecutorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_group_executor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_schema` after provisioning.\n"]
    pub fn api_schema(&self) -> terrars::ListRef<BedrockagentAgentActionGroup_ApiSchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_schema` after provisioning.\n"]
    pub fn function_schema(&self) -> terrars::ListRef<BedrockagentAgentActionGroup_FunctionSchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.function_schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentAgentActionGroup_TimeoutsRef {
        BedrockagentAgentActionGroup_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for BedrockagentAgentActionGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockagentAgentActionGroup { }

impl terrars::ToListMappable for BedrockagentAgentActionGroup {
    type O = terrars::ListRef<BedrockagentAgentActionGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockagentAgentActionGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrockagent_agent_action_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentAgentActionGroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub action_group_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub agent_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub agent_version: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentAgentActionGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockagentAgentActionGroup {
        let out = BedrockagentAgentActionGroup(std::rc::Rc::new(BedrockagentAgentActionGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockagentAgentActionGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                action_group_name: self.action_group_name,
                action_group_state: core::default::Default::default(),
                agent_id: self.agent_id,
                agent_version: self.agent_version,
                description: core::default::Default::default(),
                parent_action_group_signature: core::default::Default::default(),
                prepare_agent: core::default::Default::default(),
                region: core::default::Default::default(),
                skip_resource_in_use_check: core::default::Default::default(),
                action_group_executor: core::default::Default::default(),
                api_schema: core::default::Default::default(),
                function_schema: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentAgentActionGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentAgentActionGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockagentAgentActionGroupRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_group_id` after provisioning.\n"]
    pub fn action_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action_group_name` after provisioning.\n"]
    pub fn action_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action_group_state` after provisioning.\n"]
    pub fn action_group_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_group_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_action_group_signature` after provisioning.\n"]
    pub fn parent_action_group_signature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_action_group_signature", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `prepare_agent` after provisioning.\n"]
    pub fn prepare_agent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prepare_agent", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `action_group_executor` after provisioning.\n"]
    pub fn action_group_executor(&self) -> terrars::ListRef<BedrockagentAgentActionGroup_ActionGroupExecutorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_group_executor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_schema` after provisioning.\n"]
    pub fn api_schema(&self) -> terrars::ListRef<BedrockagentAgentActionGroup_ApiSchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_schema` after provisioning.\n"]
    pub fn function_schema(&self) -> terrars::ListRef<BedrockagentAgentActionGroup_FunctionSchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.function_schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentAgentActionGroup_TimeoutsRef {
        BedrockagentAgentActionGroup_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct BedrockagentAgentActionGroupDynamic {
    action_group_executor: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentAgentActionGroup_ActionGroupExecutor>>,
    api_schema: std::option::Option<terrars::DynamicBlock<BedrockagentAgentActionGroup_ApiSchema>>,
    function_schema: std::option::Option<terrars::DynamicBlock<BedrockagentAgentActionGroup_FunctionSchema>>,
}

#[derive(Serialize)]
pub struct BedrockagentAgentActionGroup_ActionGroupExecutor {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentAgentActionGroup_ActionGroupExecutor {
    #[doc = "Set the field `custom_control`.\n"]
    pub fn set_custom_control(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.custom_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lambda`.\n"]
    pub fn set_lambda(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lambda = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentAgentActionGroup_ActionGroupExecutor {
    type O = terrars::BlockAssignable<BedrockagentAgentActionGroup_ActionGroupExecutor>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgentActionGroup_ActionGroupExecutor {}

impl BuildBedrockagentAgentActionGroup_ActionGroupExecutor {
    pub fn build(self) -> BedrockagentAgentActionGroup_ActionGroupExecutor {
        BedrockagentAgentActionGroup_ActionGroupExecutor {
            custom_control: core::default::Default::default(),
            lambda: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentAgentActionGroup_ActionGroupExecutorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentAgentActionGroup_ActionGroupExecutorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentAgentActionGroup_ActionGroupExecutorRef {
        BedrockagentAgentActionGroup_ActionGroupExecutorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgentActionGroup_ActionGroupExecutorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_control` after provisioning.\n"]
    pub fn custom_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_control", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda` after provisioning.\n"]
    pub fn lambda(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentAgentActionGroup_ApiSchema_S3 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_object_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentAgentActionGroup_ApiSchema_S3 {
    #[doc = "Set the field `s3_bucket_name`.\n"]
    pub fn set_s3_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_object_key`.\n"]
    pub fn set_s3_object_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_object_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentAgentActionGroup_ApiSchema_S3 {
    type O = terrars::BlockAssignable<BedrockagentAgentActionGroup_ApiSchema_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgentActionGroup_ApiSchema_S3 {}

impl BuildBedrockagentAgentActionGroup_ApiSchema_S3 {
    pub fn build(self) -> BedrockagentAgentActionGroup_ApiSchema_S3 {
        BedrockagentAgentActionGroup_ApiSchema_S3 {
            s3_bucket_name: core::default::Default::default(),
            s3_object_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentAgentActionGroup_ApiSchema_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentAgentActionGroup_ApiSchema_S3Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentAgentActionGroup_ApiSchema_S3Ref {
        BedrockagentAgentActionGroup_ApiSchema_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgentActionGroup_ApiSchema_S3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_object_key` after provisioning.\n"]
    pub fn s3_object_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_object_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentAgentActionGroup_ApiSchemaDynamic {
    s3: std::option::Option<terrars::DynamicBlock<BedrockagentAgentActionGroup_ApiSchema_S3>>,
}

#[derive(Serialize)]
pub struct BedrockagentAgentActionGroup_ApiSchema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payload: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<BedrockagentAgentActionGroup_ApiSchema_S3>>,
    dynamic: BedrockagentAgentActionGroup_ApiSchemaDynamic,
}

impl BedrockagentAgentActionGroup_ApiSchema {
    #[doc = "Set the field `payload`.\n"]
    pub fn set_payload(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.payload = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentAgentActionGroup_ApiSchema_S3>>,
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

impl terrars::ToListMappable for BedrockagentAgentActionGroup_ApiSchema {
    type O = terrars::BlockAssignable<BedrockagentAgentActionGroup_ApiSchema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgentActionGroup_ApiSchema {}

impl BuildBedrockagentAgentActionGroup_ApiSchema {
    pub fn build(self) -> BedrockagentAgentActionGroup_ApiSchema {
        BedrockagentAgentActionGroup_ApiSchema {
            payload: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentAgentActionGroup_ApiSchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentAgentActionGroup_ApiSchemaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentAgentActionGroup_ApiSchemaRef {
        BedrockagentAgentActionGroup_ApiSchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgentActionGroup_ApiSchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `payload` after provisioning.\n"]
    pub fn payload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<BedrockagentAgentActionGroup_ApiSchema_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions_Parameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    map_block_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions_Parameters {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `required`.\n"]
    pub fn set_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.required = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions_Parameters {
    type O =
        terrars::BlockAssignable<BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions_Parameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions_Parameters {
    #[doc = ""]
    pub map_block_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions_Parameters {
    pub fn build(self) -> BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions_Parameters {
        BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions_Parameters {
            description: core::default::Default::default(),
            map_block_key: self.map_block_key,
            required: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions_ParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions_ParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions_ParametersRef {
        BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions_ParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions_ParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `map_block_key` after provisioning.\n"]
    pub fn map_block_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.map_block_key", self.base))
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_FunctionsDynamic {
    parameters: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions_Parameters>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std
    ::option
    ::Option<std::vec::Vec<BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions_Parameters>>,
    dynamic: BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_FunctionsDynamic,
}

impl BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions_Parameters,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions {
    type O = terrars::BlockAssignable<BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions {
    pub fn build(self) -> BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions {
        BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions {
            description: core::default::Default::default(),
            name: self.name,
            parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_FunctionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_FunctionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_FunctionsRef {
        BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_FunctionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_FunctionsRef {
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
}

#[derive(Serialize, Default)]
struct BedrockagentAgentActionGroup_FunctionSchema_MemberFunctionsDynamic {
    functions: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions>>,
}

#[derive(Serialize)]
pub struct BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    functions: std::option::Option<std::vec::Vec<BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions>>,
    dynamic: BedrockagentAgentActionGroup_FunctionSchema_MemberFunctionsDynamic,
}

impl BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions {
    #[doc = "Set the field `functions`.\n"]
    pub fn set_functions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_Functions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.functions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.functions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions {
    type O = terrars::BlockAssignable<BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgentActionGroup_FunctionSchema_MemberFunctions {}

impl BuildBedrockagentAgentActionGroup_FunctionSchema_MemberFunctions {
    pub fn build(self) -> BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions {
        BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions {
            functions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentAgentActionGroup_FunctionSchema_MemberFunctionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentAgentActionGroup_FunctionSchema_MemberFunctionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentAgentActionGroup_FunctionSchema_MemberFunctionsRef {
        BedrockagentAgentActionGroup_FunctionSchema_MemberFunctionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgentActionGroup_FunctionSchema_MemberFunctionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `functions` after provisioning.\n"]
    pub fn functions(
        &self,
    ) -> terrars::ListRef<BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions_FunctionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.functions", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentAgentActionGroup_FunctionSchemaDynamic {
    member_functions: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions>>,
}

#[derive(Serialize)]
pub struct BedrockagentAgentActionGroup_FunctionSchema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    member_functions: std::option::Option<std::vec::Vec<BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions>>,
    dynamic: BedrockagentAgentActionGroup_FunctionSchemaDynamic,
}

impl BedrockagentAgentActionGroup_FunctionSchema {
    #[doc = "Set the field `member_functions`.\n"]
    pub fn set_member_functions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentAgentActionGroup_FunctionSchema_MemberFunctions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.member_functions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.member_functions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentAgentActionGroup_FunctionSchema {
    type O = terrars::BlockAssignable<BedrockagentAgentActionGroup_FunctionSchema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgentActionGroup_FunctionSchema {}

impl BuildBedrockagentAgentActionGroup_FunctionSchema {
    pub fn build(self) -> BedrockagentAgentActionGroup_FunctionSchema {
        BedrockagentAgentActionGroup_FunctionSchema {
            member_functions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentAgentActionGroup_FunctionSchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentAgentActionGroup_FunctionSchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentAgentActionGroup_FunctionSchemaRef {
        BedrockagentAgentActionGroup_FunctionSchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgentActionGroup_FunctionSchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `member_functions` after provisioning.\n"]
    pub fn member_functions(&self) -> terrars::ListRef<BedrockagentAgentActionGroup_FunctionSchema_MemberFunctionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.member_functions", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentAgentActionGroup_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentAgentActionGroup_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentAgentActionGroup_Timeouts {
    type O = terrars::BlockAssignable<BedrockagentAgentActionGroup_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgentActionGroup_Timeouts {}

impl BuildBedrockagentAgentActionGroup_Timeouts {
    pub fn build(self) -> BedrockagentAgentActionGroup_Timeouts {
        BedrockagentAgentActionGroup_Timeouts {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentAgentActionGroup_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentAgentActionGroup_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentAgentActionGroup_TimeoutsRef {
        BedrockagentAgentActionGroup_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgentActionGroup_TimeoutsRef {
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
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
