#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentcoreAgentRuntimeData_ {
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
    agent_runtime_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_configuration: std::option::Option<terrars::ListField<BedrockagentcoreAgentRuntime_LifecycleConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_runtime_artifact: std::option::Option<std::vec::Vec<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizer_configuration: std::option::Option<std::vec::Vec<BedrockagentcoreAgentRuntime_AuthorizerConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filesystem_configuration: std::option::Option<std::vec::Vec<BedrockagentcoreAgentRuntime_FilesystemConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std::option::Option<std::vec::Vec<BedrockagentcoreAgentRuntime_NetworkConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol_configuration: std::option::Option<std::vec::Vec<BedrockagentcoreAgentRuntime_ProtocolConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_header_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreAgentRuntime_RequestHeaderConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockagentcoreAgentRuntime_Timeouts>,
    dynamic: BedrockagentcoreAgentRuntimeDynamic,
}

struct BedrockagentcoreAgentRuntime_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockagentcoreAgentRuntimeData_>,
}

#[derive(Clone)]
pub struct BedrockagentcoreAgentRuntime(std::rc::Rc<BedrockagentcoreAgentRuntime_>);

impl BedrockagentcoreAgentRuntime {
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

    #[doc = "Set the field `environment_variables`.\n"]
    pub fn set_environment_variables(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().environment_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_configuration`.\n"]
    pub fn set_lifecycle_configuration(
        self,
        v: impl std::convert::Into<terrars::ListField<BedrockagentcoreAgentRuntime_LifecycleConfiguration>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `platform_version`.\n"]
    pub fn set_platform_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().platform_version = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `agent_runtime_artifact`.\n"]
    pub fn set_agent_runtime_artifact(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().agent_runtime_artifact = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.agent_runtime_artifact = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `authorizer_configuration`.\n"]
    pub fn set_authorizer_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreAgentRuntime_AuthorizerConfiguration>>,
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

    #[doc = "Set the field `filesystem_configuration`.\n"]
    pub fn set_filesystem_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreAgentRuntime_FilesystemConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filesystem_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filesystem_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreAgentRuntime_NetworkConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `protocol_configuration`.\n"]
    pub fn set_protocol_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreAgentRuntime_ProtocolConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().protocol_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.protocol_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `request_header_configuration`.\n"]
    pub fn set_request_header_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreAgentRuntime_RequestHeaderConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().request_header_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.request_header_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockagentcoreAgentRuntime_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `agent_runtime_arn` after provisioning.\n"]
    pub fn agent_runtime_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_runtime_id` after provisioning.\n"]
    pub fn agent_runtime_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_runtime_name` after provisioning.\n"]
    pub fn agent_runtime_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_runtime_version` after provisioning.\n"]
    pub fn agent_runtime_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_configuration` after provisioning.\n"]
    pub fn lifecycle_configuration(&self) -> terrars::ListRef<BedrockagentcoreAgentRuntime_LifecycleConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload_identity_details` after provisioning.\n"]
    pub fn workload_identity_details(&self) -> terrars::ListRef<BedrockagentcoreAgentRuntime_WorkloadIdentityDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_identity_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_runtime_artifact` after provisioning.\n"]
    pub fn agent_runtime_artifact(&self) -> terrars::ListRef<BedrockagentcoreAgentRuntime_AgentRuntimeArtifactRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_runtime_artifact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_configuration` after provisioning.\n"]
    pub fn authorizer_configuration(&self) -> terrars::ListRef<BedrockagentcoreAgentRuntime_AuthorizerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizer_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filesystem_configuration` after provisioning.\n"]
    pub fn filesystem_configuration(&self) -> terrars::ListRef<BedrockagentcoreAgentRuntime_FilesystemConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filesystem_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<BedrockagentcoreAgentRuntime_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol_configuration` after provisioning.\n"]
    pub fn protocol_configuration(&self) -> terrars::ListRef<BedrockagentcoreAgentRuntime_ProtocolConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protocol_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_header_configuration` after provisioning.\n"]
    pub fn request_header_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreAgentRuntime_RequestHeaderConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_header_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreAgentRuntime_TimeoutsRef {
        BedrockagentcoreAgentRuntime_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for BedrockagentcoreAgentRuntime {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockagentcoreAgentRuntime { }

impl terrars::ToListMappable for BedrockagentcoreAgentRuntime {
    type O = terrars::ListRef<BedrockagentcoreAgentRuntimeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockagentcoreAgentRuntime_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrockagentcore_agent_runtime".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentcoreAgentRuntime {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub agent_runtime_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreAgentRuntime {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockagentcoreAgentRuntime {
        let out = BedrockagentcoreAgentRuntime(std::rc::Rc::new(BedrockagentcoreAgentRuntime_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockagentcoreAgentRuntimeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                agent_runtime_name: self.agent_runtime_name,
                description: core::default::Default::default(),
                environment_variables: core::default::Default::default(),
                lifecycle_configuration: core::default::Default::default(),
                platform_version: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                agent_runtime_artifact: core::default::Default::default(),
                authorizer_configuration: core::default::Default::default(),
                filesystem_configuration: core::default::Default::default(),
                network_configuration: core::default::Default::default(),
                protocol_configuration: core::default::Default::default(),
                request_header_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentcoreAgentRuntimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreAgentRuntimeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockagentcoreAgentRuntimeRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_runtime_arn` after provisioning.\n"]
    pub fn agent_runtime_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_runtime_id` after provisioning.\n"]
    pub fn agent_runtime_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_runtime_name` after provisioning.\n"]
    pub fn agent_runtime_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_runtime_version` after provisioning.\n"]
    pub fn agent_runtime_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_runtime_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_configuration` after provisioning.\n"]
    pub fn lifecycle_configuration(&self) -> terrars::ListRef<BedrockagentcoreAgentRuntime_LifecycleConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload_identity_details` after provisioning.\n"]
    pub fn workload_identity_details(&self) -> terrars::ListRef<BedrockagentcoreAgentRuntime_WorkloadIdentityDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_identity_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_runtime_artifact` after provisioning.\n"]
    pub fn agent_runtime_artifact(&self) -> terrars::ListRef<BedrockagentcoreAgentRuntime_AgentRuntimeArtifactRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_runtime_artifact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_configuration` after provisioning.\n"]
    pub fn authorizer_configuration(&self) -> terrars::ListRef<BedrockagentcoreAgentRuntime_AuthorizerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizer_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filesystem_configuration` after provisioning.\n"]
    pub fn filesystem_configuration(&self) -> terrars::ListRef<BedrockagentcoreAgentRuntime_FilesystemConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filesystem_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<BedrockagentcoreAgentRuntime_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol_configuration` after provisioning.\n"]
    pub fn protocol_configuration(&self) -> terrars::ListRef<BedrockagentcoreAgentRuntime_ProtocolConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protocol_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_header_configuration` after provisioning.\n"]
    pub fn request_header_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreAgentRuntime_RequestHeaderConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_header_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreAgentRuntime_TimeoutsRef {
        BedrockagentcoreAgentRuntime_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreAgentRuntimeDynamic {
    agent_runtime_artifact: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact>>,
    authorizer_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreAgentRuntime_AuthorizerConfiguration>>,
    filesystem_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreAgentRuntime_FilesystemConfiguration>>,
    network_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreAgentRuntime_NetworkConfiguration>>,
    protocol_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreAgentRuntime_ProtocolConfiguration>>,
    request_header_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreAgentRuntime_RequestHeaderConfiguration>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_LifecycleConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_runtime_session_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_lifetime: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentcoreAgentRuntime_LifecycleConfiguration {
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

impl terrars::ToListMappable for BedrockagentcoreAgentRuntime_LifecycleConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreAgentRuntime_LifecycleConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_LifecycleConfiguration {}

impl BuildBedrockagentcoreAgentRuntime_LifecycleConfiguration {
    pub fn build(self) -> BedrockagentcoreAgentRuntime_LifecycleConfiguration {
        BedrockagentcoreAgentRuntime_LifecycleConfiguration {
            idle_runtime_session_timeout: core::default::Default::default(),
            max_lifetime: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_LifecycleConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreAgentRuntime_LifecycleConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_LifecycleConfigurationRef {
        BedrockagentcoreAgentRuntime_LifecycleConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_LifecycleConfigurationRef {
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
pub struct BedrockagentcoreAgentRuntime_WorkloadIdentityDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_identity_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreAgentRuntime_WorkloadIdentityDetails {
    #[doc = "Set the field `workload_identity_arn`.\n"]
    pub fn set_workload_identity_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.workload_identity_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreAgentRuntime_WorkloadIdentityDetails {
    type O = terrars::BlockAssignable<BedrockagentcoreAgentRuntime_WorkloadIdentityDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_WorkloadIdentityDetails {}

impl BuildBedrockagentcoreAgentRuntime_WorkloadIdentityDetails {
    pub fn build(self) -> BedrockagentcoreAgentRuntime_WorkloadIdentityDetails {
        BedrockagentcoreAgentRuntime_WorkloadIdentityDetails {
            workload_identity_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_WorkloadIdentityDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreAgentRuntime_WorkloadIdentityDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_WorkloadIdentityDetailsRef {
        BedrockagentcoreAgentRuntime_WorkloadIdentityDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_WorkloadIdentityDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `workload_identity_arn` after provisioning.\n"]
    pub fn workload_identity_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workload_identity_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code_S3 {
    bucket: terrars::PrimField<std::string::String>,
    prefix: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code_S3 {
    #[doc = "Set the field `version_id`.\n"]
    pub fn set_version_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code_S3 {
    type O = terrars::BlockAssignable<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code_S3 {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub prefix: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code_S3 {
    pub fn build(self) -> BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code_S3 {
        BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code_S3 {
            bucket: self.bucket,
            prefix: self.prefix,
            version_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code_S3Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code_S3Ref {
        BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code_S3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_CodeDynamic {
    s3: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code_S3>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code_S3>>,
    dynamic: BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_CodeDynamic,
}

impl BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code_S3>,
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

impl terrars::ToListMappable for BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code {
    type O = terrars::BlockAssignable<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code {}

impl BuildBedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code {
    pub fn build(self) -> BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code {
        BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code {
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_CodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_CodeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_CodeRef {
        BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_CodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_CodeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfigurationDynamic {
    code: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration {
    entry_point: terrars::ListField<terrars::PrimField<std::string::String>>,
    runtime: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<std::vec::Vec<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code>>,
    dynamic: BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfigurationDynamic,
}

impl BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_Code>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration {
    #[doc = ""]
    pub entry_point: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub runtime: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration {
    pub fn build(self) -> BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration {
        BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration {
            entry_point: self.entry_point,
            runtime: self.runtime,
            code: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfigurationRef {
        BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entry_point` after provisioning.\n"]
    pub fn entry_point(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entry_point", self.base))
    }

    #[doc = "Get a reference to the value of field `runtime` after provisioning.\n"]
    pub fn runtime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.base))
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::ListRef<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration_CodeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_ContainerConfiguration {
    container_uri: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_ContainerConfiguration { }

impl terrars::ToListMappable for BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_ContainerConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_ContainerConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_AgentRuntimeArtifact_ContainerConfiguration {
    #[doc = ""]
    pub container_uri: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreAgentRuntime_AgentRuntimeArtifact_ContainerConfiguration {
    pub fn build(self) -> BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_ContainerConfiguration {
        BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_ContainerConfiguration {
            container_uri: self.container_uri,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_ContainerConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_ContainerConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_ContainerConfigurationRef {
        BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_ContainerConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_ContainerConfigurationRef {
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
struct BedrockagentcoreAgentRuntime_AgentRuntimeArtifactDynamic {
    code_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration>>,
    container_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_ContainerConfiguration>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_AgentRuntimeArtifact {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_ContainerConfiguration>>,
    dynamic: BedrockagentcoreAgentRuntime_AgentRuntimeArtifactDynamic,
}

impl BedrockagentcoreAgentRuntime_AgentRuntimeArtifact {
    #[doc = "Set the field `code_configuration`.\n"]
    pub fn set_code_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `container_configuration`.\n"]
    pub fn set_container_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_ContainerConfiguration>,
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

impl terrars::ToListMappable for BedrockagentcoreAgentRuntime_AgentRuntimeArtifact {
    type O = terrars::BlockAssignable<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_AgentRuntimeArtifact {}

impl BuildBedrockagentcoreAgentRuntime_AgentRuntimeArtifact {
    pub fn build(self) -> BedrockagentcoreAgentRuntime_AgentRuntimeArtifact {
        BedrockagentcoreAgentRuntime_AgentRuntimeArtifact {
            code_configuration: core::default::Default::default(),
            container_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_AgentRuntimeArtifactRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreAgentRuntime_AgentRuntimeArtifactRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_AgentRuntimeArtifactRef {
        BedrockagentcoreAgentRuntime_AgentRuntimeArtifactRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_AgentRuntimeArtifactRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code_configuration` after provisioning.\n"]
    pub fn code_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_CodeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `container_configuration` after provisioning.\n"]
    pub fn container_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreAgentRuntime_AgentRuntimeArtifact_ContainerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
    arn: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {

}

impl terrars
::ToListMappable for BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
    pub fn build(
        self,
    ) -> BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
            arn: self.arn,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef {
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef {
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
struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationDynamic {
    hosting_environment: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_identities: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosting_environment: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment,
        >,
    >,
    dynamic: BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationDynamic,
}

impl BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
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
                            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment,
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
::ToListMappable for BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {}

impl BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
            workload_identities: core::default::Default::default(),
            hosting_environment: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef {
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef {
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
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hosting_environment", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_value_string: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_value_string_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
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
::ToListMappable for BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {}

impl BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
    pub fn build(
        self,
    ) -> BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
            match_value_string: core::default::Default::default(),
            match_value_string_list: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
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
struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueDynamic {
    claim_match_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    claim_match_operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    claim_match_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue,
        >,
    >,
    dynamic: BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueDynamic,
}

impl BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
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
                            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue,
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
::ToListMappable for BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    #[doc = ""]
    pub claim_match_operator: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    pub fn build(
        self,
    ) -> BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
            claim_match_operator: self.claim_match_operator,
            claim_match_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
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
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.claim_match_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimDynamic {
    authorizing_claim_match_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    inbound_token_claim_name: terrars::PrimField<std::string::String>,
    inbound_token_claim_value_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizing_claim_match_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue,
        >,
    >,
    dynamic: BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimDynamic,
}

impl BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
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
                            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue,
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

impl terrars::ToListMappable for BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    type O =
        terrars
        ::BlockAssignable<BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    #[doc = ""]
    pub inbound_token_claim_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub inbound_token_claim_value_type: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    pub fn build(self) -> BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
            inbound_token_claim_name: self.inbound_token_claim_name,
            inbound_token_claim_value_type: self.inbound_token_claim_value_type,
            authorizing_claim_match_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
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
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizing_claim_match_value", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
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

impl BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
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
::ToListMappable for BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    #[doc = ""]
    pub endpoint_ip_address_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub vpc_identifier: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    pub fn build(
        self,
    ) -> BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
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
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
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
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_configuration_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
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
::ToListMappable for BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {}

impl BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
    pub fn build(
        self,
    ) -> BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
            resource_configuration_identifier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
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
struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointDynamic {
    managed_vpc_resource: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource,
        >,
    >,
    self_managed_lattice_resource: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_vpc_resource: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_managed_lattice_resource: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource,
        >,
    >,
    dynamic: BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointDynamic,
}

impl BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
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
                            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource,
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
                            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource,
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

impl terrars
::ToListMappable for BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
    type O =
        terrars
        ::BlockAssignable<BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {}

impl BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
    pub fn build(self) -> BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
            managed_vpc_resource: core::default::Default::default(),
            self_managed_lattice_resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `managed_vpc_resource` after provisioning.\n"]
    pub fn managed_vpc_resource(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_vpc_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `self_managed_lattice_resource` after provisioning.\n"]
    pub fn self_managed_lattice_resource(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_managed_lattice_resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesDynamic {
    private_endpoint: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
    domain: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint>>,
    dynamic: BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesDynamic,
}

impl BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
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
                            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint,
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
::ToListMappable for BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
    #[doc = ""]
    pub domain: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
    pub fn build(
        self,
    ) -> BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
            domain: self.domain,
            private_endpoint: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef {
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef {
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
    ) -> terrars
    ::ListRef<BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizerDynamic {
    allowed_workload_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration,
        >,
    >,
    custom_claim: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim>,
    >,
    private_endpoint: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint>,
    >,
    private_endpoint_overrides: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer {
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
        ::Vec<BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_claim: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint_overrides: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides>,
    >,
    dynamic: BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizerDynamic,
}

impl BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer {
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
                            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration,
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
                            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim,
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
                            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint,
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
                            BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides,
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

impl terrars::ToListMappable for BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer {
    type O = terrars::BlockAssignable<BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer {
    #[doc = ""]
    pub discovery_url: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer {
    pub fn build(self) -> BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer {
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer {
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
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizerRef {
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizerRef {
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
    ::ListRef<
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_workload_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `private_endpoint` after provisioning.\n"]
    pub fn private_endpoint(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `private_endpoint_overrides` after provisioning.\n"]
    pub fn private_endpoint_overrides(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoint_overrides", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreAgentRuntime_AuthorizerConfigurationDynamic {
    custom_jwt_authorizer: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_jwt_authorizer: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer>>,
    dynamic: BedrockagentcoreAgentRuntime_AuthorizerConfigurationDynamic,
}

impl BedrockagentcoreAgentRuntime_AuthorizerConfiguration {
    #[doc = "Set the field `custom_jwt_authorizer`.\n"]
    pub fn set_custom_jwt_authorizer(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizer>,
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

impl terrars::ToListMappable for BedrockagentcoreAgentRuntime_AuthorizerConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreAgentRuntime_AuthorizerConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration {}

impl BuildBedrockagentcoreAgentRuntime_AuthorizerConfiguration {
    pub fn build(self) -> BedrockagentcoreAgentRuntime_AuthorizerConfiguration {
        BedrockagentcoreAgentRuntime_AuthorizerConfiguration {
            custom_jwt_authorizer: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_AuthorizerConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreAgentRuntime_AuthorizerConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_AuthorizerConfigurationRef {
        BedrockagentcoreAgentRuntime_AuthorizerConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_AuthorizerConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_jwt_authorizer` after provisioning.\n"]
    pub fn custom_jwt_authorizer(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreAgentRuntime_AuthorizerConfiguration_CustomJwtAuthorizerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_jwt_authorizer", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_FilesystemConfiguration_EfsAccessPoint {
    access_point_arn: terrars::PrimField<std::string::String>,
    mount_path: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreAgentRuntime_FilesystemConfiguration_EfsAccessPoint { }

impl terrars::ToListMappable for BedrockagentcoreAgentRuntime_FilesystemConfiguration_EfsAccessPoint {
    type O = terrars::BlockAssignable<BedrockagentcoreAgentRuntime_FilesystemConfiguration_EfsAccessPoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_FilesystemConfiguration_EfsAccessPoint {
    #[doc = ""]
    pub access_point_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub mount_path: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreAgentRuntime_FilesystemConfiguration_EfsAccessPoint {
    pub fn build(self) -> BedrockagentcoreAgentRuntime_FilesystemConfiguration_EfsAccessPoint {
        BedrockagentcoreAgentRuntime_FilesystemConfiguration_EfsAccessPoint {
            access_point_arn: self.access_point_arn,
            mount_path: self.mount_path,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_FilesystemConfiguration_EfsAccessPointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreAgentRuntime_FilesystemConfiguration_EfsAccessPointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_FilesystemConfiguration_EfsAccessPointRef {
        BedrockagentcoreAgentRuntime_FilesystemConfiguration_EfsAccessPointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_FilesystemConfiguration_EfsAccessPointRef {
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
pub struct BedrockagentcoreAgentRuntime_FilesystemConfiguration_S3FilesAccessPoint {
    access_point_arn: terrars::PrimField<std::string::String>,
    mount_path: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreAgentRuntime_FilesystemConfiguration_S3FilesAccessPoint { }

impl terrars::ToListMappable for BedrockagentcoreAgentRuntime_FilesystemConfiguration_S3FilesAccessPoint {
    type O = terrars::BlockAssignable<BedrockagentcoreAgentRuntime_FilesystemConfiguration_S3FilesAccessPoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_FilesystemConfiguration_S3FilesAccessPoint {
    #[doc = ""]
    pub access_point_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub mount_path: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreAgentRuntime_FilesystemConfiguration_S3FilesAccessPoint {
    pub fn build(self) -> BedrockagentcoreAgentRuntime_FilesystemConfiguration_S3FilesAccessPoint {
        BedrockagentcoreAgentRuntime_FilesystemConfiguration_S3FilesAccessPoint {
            access_point_arn: self.access_point_arn,
            mount_path: self.mount_path,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_FilesystemConfiguration_S3FilesAccessPointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreAgentRuntime_FilesystemConfiguration_S3FilesAccessPointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_FilesystemConfiguration_S3FilesAccessPointRef {
        BedrockagentcoreAgentRuntime_FilesystemConfiguration_S3FilesAccessPointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_FilesystemConfiguration_S3FilesAccessPointRef {
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
pub struct BedrockagentcoreAgentRuntime_FilesystemConfiguration_SessionStorage {
    mount_path: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreAgentRuntime_FilesystemConfiguration_SessionStorage { }

impl terrars::ToListMappable for BedrockagentcoreAgentRuntime_FilesystemConfiguration_SessionStorage {
    type O = terrars::BlockAssignable<BedrockagentcoreAgentRuntime_FilesystemConfiguration_SessionStorage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_FilesystemConfiguration_SessionStorage {
    #[doc = ""]
    pub mount_path: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreAgentRuntime_FilesystemConfiguration_SessionStorage {
    pub fn build(self) -> BedrockagentcoreAgentRuntime_FilesystemConfiguration_SessionStorage {
        BedrockagentcoreAgentRuntime_FilesystemConfiguration_SessionStorage { mount_path: self.mount_path }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_FilesystemConfiguration_SessionStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreAgentRuntime_FilesystemConfiguration_SessionStorageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_FilesystemConfiguration_SessionStorageRef {
        BedrockagentcoreAgentRuntime_FilesystemConfiguration_SessionStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_FilesystemConfiguration_SessionStorageRef {
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
struct BedrockagentcoreAgentRuntime_FilesystemConfigurationDynamic {
    efs_access_point: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreAgentRuntime_FilesystemConfiguration_EfsAccessPoint>>,
    s3_files_access_point: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreAgentRuntime_FilesystemConfiguration_S3FilesAccessPoint>>,
    session_storage: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreAgentRuntime_FilesystemConfiguration_SessionStorage>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_FilesystemConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    efs_access_point: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreAgentRuntime_FilesystemConfiguration_EfsAccessPoint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_files_access_point: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreAgentRuntime_FilesystemConfiguration_S3FilesAccessPoint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_storage: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreAgentRuntime_FilesystemConfiguration_SessionStorage>>,
    dynamic: BedrockagentcoreAgentRuntime_FilesystemConfigurationDynamic,
}

impl BedrockagentcoreAgentRuntime_FilesystemConfiguration {
    #[doc = "Set the field `efs_access_point`.\n"]
    pub fn set_efs_access_point(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreAgentRuntime_FilesystemConfiguration_EfsAccessPoint>,
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
                        ::BlockAssignable<BedrockagentcoreAgentRuntime_FilesystemConfiguration_S3FilesAccessPoint>,
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
                        terrars::BlockAssignable<BedrockagentcoreAgentRuntime_FilesystemConfiguration_SessionStorage>,
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

impl terrars::ToListMappable for BedrockagentcoreAgentRuntime_FilesystemConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreAgentRuntime_FilesystemConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_FilesystemConfiguration {}

impl BuildBedrockagentcoreAgentRuntime_FilesystemConfiguration {
    pub fn build(self) -> BedrockagentcoreAgentRuntime_FilesystemConfiguration {
        BedrockagentcoreAgentRuntime_FilesystemConfiguration {
            efs_access_point: core::default::Default::default(),
            s3_files_access_point: core::default::Default::default(),
            session_storage: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_FilesystemConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreAgentRuntime_FilesystemConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_FilesystemConfigurationRef {
        BedrockagentcoreAgentRuntime_FilesystemConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_FilesystemConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `efs_access_point` after provisioning.\n"]
    pub fn efs_access_point(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreAgentRuntime_FilesystemConfiguration_EfsAccessPointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.efs_access_point", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_files_access_point` after provisioning.\n"]
    pub fn s3_files_access_point(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreAgentRuntime_FilesystemConfiguration_S3FilesAccessPointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_files_access_point", self.base))
    }

    #[doc = "Get a reference to the value of field `session_storage` after provisioning.\n"]
    pub fn session_storage(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreAgentRuntime_FilesystemConfiguration_SessionStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.session_storage", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_NetworkConfiguration_NetworkModeConfig {
    security_groups: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreAgentRuntime_NetworkConfiguration_NetworkModeConfig { }

impl terrars::ToListMappable for BedrockagentcoreAgentRuntime_NetworkConfiguration_NetworkModeConfig {
    type O = terrars::BlockAssignable<BedrockagentcoreAgentRuntime_NetworkConfiguration_NetworkModeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_NetworkConfiguration_NetworkModeConfig {
    #[doc = ""]
    pub security_groups: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildBedrockagentcoreAgentRuntime_NetworkConfiguration_NetworkModeConfig {
    pub fn build(self) -> BedrockagentcoreAgentRuntime_NetworkConfiguration_NetworkModeConfig {
        BedrockagentcoreAgentRuntime_NetworkConfiguration_NetworkModeConfig {
            security_groups: self.security_groups,
            subnets: self.subnets,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_NetworkConfiguration_NetworkModeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreAgentRuntime_NetworkConfiguration_NetworkModeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_NetworkConfiguration_NetworkModeConfigRef {
        BedrockagentcoreAgentRuntime_NetworkConfiguration_NetworkModeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_NetworkConfiguration_NetworkModeConfigRef {
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
struct BedrockagentcoreAgentRuntime_NetworkConfigurationDynamic {
    network_mode_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreAgentRuntime_NetworkConfiguration_NetworkModeConfig>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_NetworkConfiguration {
    network_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_mode_config: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreAgentRuntime_NetworkConfiguration_NetworkModeConfig>>,
    dynamic: BedrockagentcoreAgentRuntime_NetworkConfigurationDynamic,
}

impl BedrockagentcoreAgentRuntime_NetworkConfiguration {
    #[doc = "Set the field `network_mode_config`.\n"]
    pub fn set_network_mode_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreAgentRuntime_NetworkConfiguration_NetworkModeConfig>,
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

impl terrars::ToListMappable for BedrockagentcoreAgentRuntime_NetworkConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreAgentRuntime_NetworkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_NetworkConfiguration {
    #[doc = ""]
    pub network_mode: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreAgentRuntime_NetworkConfiguration {
    pub fn build(self) -> BedrockagentcoreAgentRuntime_NetworkConfiguration {
        BedrockagentcoreAgentRuntime_NetworkConfiguration {
            network_mode: self.network_mode,
            network_mode_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_NetworkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreAgentRuntime_NetworkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_NetworkConfigurationRef {
        BedrockagentcoreAgentRuntime_NetworkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_NetworkConfigurationRef {
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
    ) -> terrars::ListRef<BedrockagentcoreAgentRuntime_NetworkConfiguration_NetworkModeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_mode_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_ProtocolConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreAgentRuntime_ProtocolConfiguration {
    #[doc = "Set the field `server_protocol`.\n"]
    pub fn set_server_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_protocol = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreAgentRuntime_ProtocolConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreAgentRuntime_ProtocolConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_ProtocolConfiguration {}

impl BuildBedrockagentcoreAgentRuntime_ProtocolConfiguration {
    pub fn build(self) -> BedrockagentcoreAgentRuntime_ProtocolConfiguration {
        BedrockagentcoreAgentRuntime_ProtocolConfiguration { server_protocol: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_ProtocolConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreAgentRuntime_ProtocolConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_ProtocolConfigurationRef {
        BedrockagentcoreAgentRuntime_ProtocolConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_ProtocolConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `server_protocol` after provisioning.\n"]
    pub fn server_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_protocol", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_RequestHeaderConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_header_allowlist: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl BedrockagentcoreAgentRuntime_RequestHeaderConfiguration {
    #[doc = "Set the field `request_header_allowlist`.\n"]
    pub fn set_request_header_allowlist(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.request_header_allowlist = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreAgentRuntime_RequestHeaderConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreAgentRuntime_RequestHeaderConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_RequestHeaderConfiguration {}

impl BuildBedrockagentcoreAgentRuntime_RequestHeaderConfiguration {
    pub fn build(self) -> BedrockagentcoreAgentRuntime_RequestHeaderConfiguration {
        BedrockagentcoreAgentRuntime_RequestHeaderConfiguration {
            request_header_allowlist: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_RequestHeaderConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreAgentRuntime_RequestHeaderConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreAgentRuntime_RequestHeaderConfigurationRef {
        BedrockagentcoreAgentRuntime_RequestHeaderConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_RequestHeaderConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `request_header_allowlist` after provisioning.\n"]
    pub fn request_header_allowlist(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.request_header_allowlist", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreAgentRuntime_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreAgentRuntime_Timeouts {
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

impl terrars::ToListMappable for BedrockagentcoreAgentRuntime_Timeouts {
    type O = terrars::BlockAssignable<BedrockagentcoreAgentRuntime_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreAgentRuntime_Timeouts {}

impl BuildBedrockagentcoreAgentRuntime_Timeouts {
    pub fn build(self) -> BedrockagentcoreAgentRuntime_Timeouts {
        BedrockagentcoreAgentRuntime_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreAgentRuntime_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreAgentRuntime_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreAgentRuntime_TimeoutsRef {
        BedrockagentcoreAgentRuntime_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreAgentRuntime_TimeoutsRef {
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
