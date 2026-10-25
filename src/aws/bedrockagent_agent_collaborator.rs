#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentAgentCollaboratorData_ {
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
    agent_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_version: std::option::Option<terrars::PrimField<std::string::String>>,
    collaboration_instruction: terrars::PrimField<std::string::String>,
    collaborator_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prepare_agent: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relay_conversation_history: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_descriptor: std::option::Option<std::vec::Vec<BedrockagentAgentCollaborator_AgentDescriptor>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockagentAgentCollaborator_Timeouts>,
    dynamic: BedrockagentAgentCollaboratorDynamic,
}

struct BedrockagentAgentCollaborator_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockagentAgentCollaboratorData_>,
}

#[derive(Clone)]
pub struct BedrockagentAgentCollaborator(std::rc::Rc<BedrockagentAgentCollaborator_>);

impl BedrockagentAgentCollaborator {
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

    #[doc = "Set the field `agent_version`.\n"]
    pub fn set_agent_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().agent_version = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `relay_conversation_history`.\n"]
    pub fn set_relay_conversation_history(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().relay_conversation_history = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `agent_descriptor`.\n"]
    pub fn set_agent_descriptor(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentAgentCollaborator_AgentDescriptor>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().agent_descriptor = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.agent_descriptor = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockagentAgentCollaborator_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `collaboration_instruction` after provisioning.\n"]
    pub fn collaboration_instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collaboration_instruction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `collaborator_id` after provisioning.\n"]
    pub fn collaborator_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collaborator_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `collaborator_name` after provisioning.\n"]
    pub fn collaborator_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collaborator_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `relay_conversation_history` after provisioning.\n"]
    pub fn relay_conversation_history(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.relay_conversation_history", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_descriptor` after provisioning.\n"]
    pub fn agent_descriptor(&self) -> terrars::ListRef<BedrockagentAgentCollaborator_AgentDescriptorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_descriptor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentAgentCollaborator_TimeoutsRef {
        BedrockagentAgentCollaborator_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for BedrockagentAgentCollaborator {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockagentAgentCollaborator { }

impl terrars::ToListMappable for BedrockagentAgentCollaborator {
    type O = terrars::ListRef<BedrockagentAgentCollaboratorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockagentAgentCollaborator_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrockagent_agent_collaborator".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentAgentCollaborator {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub agent_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub collaboration_instruction: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub collaborator_name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentAgentCollaborator {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockagentAgentCollaborator {
        let out = BedrockagentAgentCollaborator(std::rc::Rc::new(BedrockagentAgentCollaborator_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockagentAgentCollaboratorData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                agent_id: self.agent_id,
                agent_version: core::default::Default::default(),
                collaboration_instruction: self.collaboration_instruction,
                collaborator_name: self.collaborator_name,
                prepare_agent: core::default::Default::default(),
                region: core::default::Default::default(),
                relay_conversation_history: core::default::Default::default(),
                agent_descriptor: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentAgentCollaboratorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentAgentCollaboratorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockagentAgentCollaboratorRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `collaboration_instruction` after provisioning.\n"]
    pub fn collaboration_instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collaboration_instruction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `collaborator_id` after provisioning.\n"]
    pub fn collaborator_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collaborator_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `collaborator_name` after provisioning.\n"]
    pub fn collaborator_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collaborator_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `relay_conversation_history` after provisioning.\n"]
    pub fn relay_conversation_history(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.relay_conversation_history", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_descriptor` after provisioning.\n"]
    pub fn agent_descriptor(&self) -> terrars::ListRef<BedrockagentAgentCollaborator_AgentDescriptorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_descriptor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentAgentCollaborator_TimeoutsRef {
        BedrockagentAgentCollaborator_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct BedrockagentAgentCollaboratorDynamic {
    agent_descriptor: std::option::Option<terrars::DynamicBlock<BedrockagentAgentCollaborator_AgentDescriptor>>,
}

#[derive(Serialize)]
pub struct BedrockagentAgentCollaborator_AgentDescriptor {
    alias_arn: terrars::PrimField<std::string::String>,
}

impl BedrockagentAgentCollaborator_AgentDescriptor { }

impl terrars::ToListMappable for BedrockagentAgentCollaborator_AgentDescriptor {
    type O = terrars::BlockAssignable<BedrockagentAgentCollaborator_AgentDescriptor>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgentCollaborator_AgentDescriptor {
    #[doc = ""]
    pub alias_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentAgentCollaborator_AgentDescriptor {
    pub fn build(self) -> BedrockagentAgentCollaborator_AgentDescriptor {
        BedrockagentAgentCollaborator_AgentDescriptor { alias_arn: self.alias_arn }
    }
}

#[allow(dead_code)]
pub struct BedrockagentAgentCollaborator_AgentDescriptorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentAgentCollaborator_AgentDescriptorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentAgentCollaborator_AgentDescriptorRef {
        BedrockagentAgentCollaborator_AgentDescriptorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgentCollaborator_AgentDescriptorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alias_arn` after provisioning.\n"]
    pub fn alias_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentAgentCollaborator_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentAgentCollaborator_Timeouts {
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

impl terrars::ToListMappable for BedrockagentAgentCollaborator_Timeouts {
    type O = terrars::BlockAssignable<BedrockagentAgentCollaborator_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentAgentCollaborator_Timeouts {}

impl BuildBedrockagentAgentCollaborator_Timeouts {
    pub fn build(self) -> BedrockagentAgentCollaborator_Timeouts {
        BedrockagentAgentCollaborator_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentAgentCollaborator_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentAgentCollaborator_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentAgentCollaborator_TimeoutsRef {
        BedrockagentAgentCollaborator_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentAgentCollaborator_TimeoutsRef {
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
