#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataBedrockagentAgentVersionsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    agent_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_version_summaries: std::option::Option<std::vec::Vec<DataBedrockagentAgentVersions_AgentVersionSummaries>>,
    dynamic: DataBedrockagentAgentVersionsDynamic,
}

struct DataBedrockagentAgentVersions_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBedrockagentAgentVersionsData_>,
}

#[derive(Clone)]
pub struct DataBedrockagentAgentVersions(std::rc::Rc<DataBedrockagentAgentVersions_>);

impl DataBedrockagentAgentVersions {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `agent_version_summaries`.\n"]
    pub fn set_agent_version_summaries(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataBedrockagentAgentVersions_AgentVersionSummaries>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().agent_version_summaries = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.agent_version_summaries = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_version_summaries` after provisioning.\n"]
    pub fn agent_version_summaries(&self) -> terrars::ListRef<DataBedrockagentAgentVersions_AgentVersionSummariesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_version_summaries", self.extract_ref()))
    }
}

impl terrars::Referable for DataBedrockagentAgentVersions {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBedrockagentAgentVersions { }

impl terrars::ToListMappable for DataBedrockagentAgentVersions {
    type O = terrars::ListRef<DataBedrockagentAgentVersionsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBedrockagentAgentVersions_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_bedrockagent_agent_versions".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBedrockagentAgentVersions {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub agent_id: terrars::PrimField<std::string::String>,
}

impl BuildDataBedrockagentAgentVersions {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBedrockagentAgentVersions {
        let out = DataBedrockagentAgentVersions(std::rc::Rc::new(DataBedrockagentAgentVersions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBedrockagentAgentVersionsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                agent_id: self.agent_id,
                region: core::default::Default::default(),
                agent_version_summaries: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBedrockagentAgentVersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockagentAgentVersionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBedrockagentAgentVersionsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_version_summaries` after provisioning.\n"]
    pub fn agent_version_summaries(&self) -> terrars::ListRef<DataBedrockagentAgentVersions_AgentVersionSummariesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_version_summaries", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataBedrockagentAgentVersionsDynamic {
    agent_version_summaries: std
    ::option
    ::Option<terrars::DynamicBlock<DataBedrockagentAgentVersions_AgentVersionSummaries>>,
}

#[derive(Serialize)]
pub struct DataBedrockagentAgentVersions_AgentVersionSummaries_GuardrailConfiguration {}

impl DataBedrockagentAgentVersions_AgentVersionSummaries_GuardrailConfiguration { }

impl terrars::ToListMappable for DataBedrockagentAgentVersions_AgentVersionSummaries_GuardrailConfiguration {
    type O = terrars::BlockAssignable<DataBedrockagentAgentVersions_AgentVersionSummaries_GuardrailConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockagentAgentVersions_AgentVersionSummaries_GuardrailConfiguration {}

impl BuildDataBedrockagentAgentVersions_AgentVersionSummaries_GuardrailConfiguration {
    pub fn build(self) -> DataBedrockagentAgentVersions_AgentVersionSummaries_GuardrailConfiguration {
        DataBedrockagentAgentVersions_AgentVersionSummaries_GuardrailConfiguration {}
    }
}

#[allow(dead_code)]
pub struct DataBedrockagentAgentVersions_AgentVersionSummaries_GuardrailConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockagentAgentVersions_AgentVersionSummaries_GuardrailConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBedrockagentAgentVersions_AgentVersionSummaries_GuardrailConfigurationRef {
        DataBedrockagentAgentVersions_AgentVersionSummaries_GuardrailConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockagentAgentVersions_AgentVersionSummaries_GuardrailConfigurationRef {
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

#[derive(Serialize, Default)]
struct DataBedrockagentAgentVersions_AgentVersionSummariesDynamic {
    guardrail_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<DataBedrockagentAgentVersions_AgentVersionSummaries_GuardrailConfiguration>>,
}

#[derive(Serialize)]
pub struct DataBedrockagentAgentVersions_AgentVersionSummaries {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guardrail_configuration: std
    ::option
    ::Option<std::vec::Vec<DataBedrockagentAgentVersions_AgentVersionSummaries_GuardrailConfiguration>>,
    dynamic: DataBedrockagentAgentVersions_AgentVersionSummariesDynamic,
}

impl DataBedrockagentAgentVersions_AgentVersionSummaries {
    #[doc = "Set the field `guardrail_configuration`.\n"]
    pub fn set_guardrail_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataBedrockagentAgentVersions_AgentVersionSummaries_GuardrailConfiguration>,
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
}

impl terrars::ToListMappable for DataBedrockagentAgentVersions_AgentVersionSummaries {
    type O = terrars::BlockAssignable<DataBedrockagentAgentVersions_AgentVersionSummaries>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockagentAgentVersions_AgentVersionSummaries {}

impl BuildDataBedrockagentAgentVersions_AgentVersionSummaries {
    pub fn build(self) -> DataBedrockagentAgentVersions_AgentVersionSummaries {
        DataBedrockagentAgentVersions_AgentVersionSummaries {
            guardrail_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBedrockagentAgentVersions_AgentVersionSummariesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockagentAgentVersions_AgentVersionSummariesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBedrockagentAgentVersions_AgentVersionSummariesRef {
        DataBedrockagentAgentVersions_AgentVersionSummariesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockagentAgentVersions_AgentVersionSummariesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_name` after provisioning.\n"]
    pub fn agent_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_name", self.base))
    }

    #[doc = "Get a reference to the value of field `agent_status` after provisioning.\n"]
    pub fn agent_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_status", self.base))
    }

    #[doc = "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_version", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `guardrail_configuration` after provisioning.\n"]
    pub fn guardrail_configuration(
        &self,
    ) -> terrars::ListRef<DataBedrockagentAgentVersions_AgentVersionSummaries_GuardrailConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guardrail_configuration", self.base))
    }
}
