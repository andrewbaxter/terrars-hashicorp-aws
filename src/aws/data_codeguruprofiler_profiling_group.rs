#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCodeguruprofilerProfilingGroupData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataCodeguruprofilerProfilingGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCodeguruprofilerProfilingGroupData_>,
}

#[derive(Clone)]
pub struct DataCodeguruprofilerProfilingGroup(std::rc::Rc<DataCodeguruprofilerProfilingGroup_>);

impl DataCodeguruprofilerProfilingGroup {
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

    #[doc = "Get a reference to the value of field `agent_orchestration_config` after provisioning.\n"]
    pub fn agent_orchestration_config(
        &self,
    ) -> terrars::ListRef<DataCodeguruprofilerProfilingGroup_AgentOrchestrationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_orchestration_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_platform` after provisioning.\n"]
    pub fn compute_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profiling_status` after provisioning.\n"]
    pub fn profiling_status(&self) -> terrars::ListRef<DataCodeguruprofilerProfilingGroup_ProfilingStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.profiling_status", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl terrars::Referable for DataCodeguruprofilerProfilingGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCodeguruprofilerProfilingGroup { }

impl terrars::ToListMappable for DataCodeguruprofilerProfilingGroup {
    type O = terrars::ListRef<DataCodeguruprofilerProfilingGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCodeguruprofilerProfilingGroup_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_codeguruprofiler_profiling_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCodeguruprofilerProfilingGroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataCodeguruprofilerProfilingGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCodeguruprofilerProfilingGroup {
        let out = DataCodeguruprofilerProfilingGroup(std::rc::Rc::new(DataCodeguruprofilerProfilingGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCodeguruprofilerProfilingGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                name: self.name,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCodeguruprofilerProfilingGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCodeguruprofilerProfilingGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCodeguruprofilerProfilingGroupRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `agent_orchestration_config` after provisioning.\n"]
    pub fn agent_orchestration_config(
        &self,
    ) -> terrars::ListRef<DataCodeguruprofilerProfilingGroup_AgentOrchestrationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_orchestration_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_platform` after provisioning.\n"]
    pub fn compute_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profiling_status` after provisioning.\n"]
    pub fn profiling_status(&self) -> terrars::ListRef<DataCodeguruprofilerProfilingGroup_ProfilingStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.profiling_status", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCodeguruprofilerProfilingGroup_AgentOrchestrationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profiling_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCodeguruprofilerProfilingGroup_AgentOrchestrationConfig {
    #[doc = "Set the field `profiling_enabled`.\n"]
    pub fn set_profiling_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.profiling_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCodeguruprofilerProfilingGroup_AgentOrchestrationConfig {
    type O = terrars::BlockAssignable<DataCodeguruprofilerProfilingGroup_AgentOrchestrationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCodeguruprofilerProfilingGroup_AgentOrchestrationConfig {}

impl BuildDataCodeguruprofilerProfilingGroup_AgentOrchestrationConfig {
    pub fn build(self) -> DataCodeguruprofilerProfilingGroup_AgentOrchestrationConfig {
        DataCodeguruprofilerProfilingGroup_AgentOrchestrationConfig {
            profiling_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCodeguruprofilerProfilingGroup_AgentOrchestrationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCodeguruprofilerProfilingGroup_AgentOrchestrationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCodeguruprofilerProfilingGroup_AgentOrchestrationConfigRef {
        DataCodeguruprofilerProfilingGroup_AgentOrchestrationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCodeguruprofilerProfilingGroup_AgentOrchestrationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `profiling_enabled` after provisioning.\n"]
    pub fn profiling_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profiling_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCodeguruprofilerProfilingGroup_ProfilingStatus_LatestAggregatedProfile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCodeguruprofilerProfilingGroup_ProfilingStatus_LatestAggregatedProfile {
    #[doc = "Set the field `period`.\n"]
    pub fn set_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCodeguruprofilerProfilingGroup_ProfilingStatus_LatestAggregatedProfile {
    type O = terrars::BlockAssignable<DataCodeguruprofilerProfilingGroup_ProfilingStatus_LatestAggregatedProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCodeguruprofilerProfilingGroup_ProfilingStatus_LatestAggregatedProfile {}

impl BuildDataCodeguruprofilerProfilingGroup_ProfilingStatus_LatestAggregatedProfile {
    pub fn build(self) -> DataCodeguruprofilerProfilingGroup_ProfilingStatus_LatestAggregatedProfile {
        DataCodeguruprofilerProfilingGroup_ProfilingStatus_LatestAggregatedProfile {
            period: core::default::Default::default(),
            start: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCodeguruprofilerProfilingGroup_ProfilingStatus_LatestAggregatedProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCodeguruprofilerProfilingGroup_ProfilingStatus_LatestAggregatedProfileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCodeguruprofilerProfilingGroup_ProfilingStatus_LatestAggregatedProfileRef {
        DataCodeguruprofilerProfilingGroup_ProfilingStatus_LatestAggregatedProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCodeguruprofilerProfilingGroup_ProfilingStatus_LatestAggregatedProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCodeguruprofilerProfilingGroup_ProfilingStatus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latest_agent_orchestrated_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latest_agent_profile_reported_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latest_aggregated_profile: std
    ::option
    ::Option<terrars::ListField<DataCodeguruprofilerProfilingGroup_ProfilingStatus_LatestAggregatedProfile>>,
}

impl DataCodeguruprofilerProfilingGroup_ProfilingStatus {
    #[doc = "Set the field `latest_agent_orchestrated_at`.\n"]
    pub fn set_latest_agent_orchestrated_at(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.latest_agent_orchestrated_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `latest_agent_profile_reported_at`.\n"]
    pub fn set_latest_agent_profile_reported_at(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.latest_agent_profile_reported_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `latest_aggregated_profile`.\n"]
    pub fn set_latest_aggregated_profile(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataCodeguruprofilerProfilingGroup_ProfilingStatus_LatestAggregatedProfile>,
                    >,
    ) -> Self {
        self.latest_aggregated_profile = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCodeguruprofilerProfilingGroup_ProfilingStatus {
    type O = terrars::BlockAssignable<DataCodeguruprofilerProfilingGroup_ProfilingStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCodeguruprofilerProfilingGroup_ProfilingStatus {}

impl BuildDataCodeguruprofilerProfilingGroup_ProfilingStatus {
    pub fn build(self) -> DataCodeguruprofilerProfilingGroup_ProfilingStatus {
        DataCodeguruprofilerProfilingGroup_ProfilingStatus {
            latest_agent_orchestrated_at: core::default::Default::default(),
            latest_agent_profile_reported_at: core::default::Default::default(),
            latest_aggregated_profile: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCodeguruprofilerProfilingGroup_ProfilingStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCodeguruprofilerProfilingGroup_ProfilingStatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCodeguruprofilerProfilingGroup_ProfilingStatusRef {
        DataCodeguruprofilerProfilingGroup_ProfilingStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCodeguruprofilerProfilingGroup_ProfilingStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `latest_agent_orchestrated_at` after provisioning.\n"]
    pub fn latest_agent_orchestrated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_agent_orchestrated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `latest_agent_profile_reported_at` after provisioning.\n"]
    pub fn latest_agent_profile_reported_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_agent_profile_reported_at", self.base))
    }

    #[doc = "Get a reference to the value of field `latest_aggregated_profile` after provisioning.\n"]
    pub fn latest_aggregated_profile(
        &self,
    ) -> terrars::ListRef<DataCodeguruprofilerProfilingGroup_ProfilingStatus_LatestAggregatedProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.latest_aggregated_profile", self.base))
    }
}
