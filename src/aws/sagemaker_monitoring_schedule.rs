#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerMonitoringScheduleData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_schedule_config: std::option::Option<std::vec::Vec<SagemakerMonitoringSchedule_MonitoringScheduleConfig>>,
    dynamic: SagemakerMonitoringScheduleDynamic,
}

struct SagemakerMonitoringSchedule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerMonitoringScheduleData_>,
}

#[derive(Clone)]
pub struct SagemakerMonitoringSchedule(std::rc::Rc<SagemakerMonitoringSchedule_>);

impl SagemakerMonitoringSchedule {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monitoring_schedule_config`.\n"]
    pub fn set_monitoring_schedule_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerMonitoringSchedule_MonitoringScheduleConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().monitoring_schedule_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.monitoring_schedule_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_schedule_config` after provisioning.\n"]
    pub fn monitoring_schedule_config(
        &self,
    ) -> terrars::ListRef<SagemakerMonitoringSchedule_MonitoringScheduleConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_schedule_config", self.extract_ref()))
    }
}

impl terrars::Referable for SagemakerMonitoringSchedule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerMonitoringSchedule { }

impl terrars::ToListMappable for SagemakerMonitoringSchedule {
    type O = terrars::ListRef<SagemakerMonitoringScheduleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerMonitoringSchedule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_monitoring_schedule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerMonitoringSchedule {
    pub tf_id: std::string::String,
}

impl BuildSagemakerMonitoringSchedule {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerMonitoringSchedule {
        let out = SagemakerMonitoringSchedule(std::rc::Rc::new(SagemakerMonitoringSchedule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerMonitoringScheduleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                monitoring_schedule_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerMonitoringScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerMonitoringScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerMonitoringScheduleRef {
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_schedule_config` after provisioning.\n"]
    pub fn monitoring_schedule_config(
        &self,
    ) -> terrars::ListRef<SagemakerMonitoringSchedule_MonitoringScheduleConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_schedule_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SagemakerMonitoringScheduleDynamic {
    monitoring_schedule_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerMonitoringSchedule_MonitoringScheduleConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_ConstraintsResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_ConstraintsResource {
    #[doc = "Set the field `s3_uri`.\n"]
    pub fn set_s3_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_ConstraintsResource {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_ConstraintsResource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_ConstraintsResource {}

impl BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_ConstraintsResource {
    pub fn build(
        self,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_ConstraintsResource {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_ConstraintsResource {
            s3_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_ConstraintsResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_ConstraintsResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_ConstraintsResourceRef {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_ConstraintsResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_ConstraintsResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_StatisticsResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_StatisticsResource {
    #[doc = "Set the field `s3_uri`.\n"]
    pub fn set_s3_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_StatisticsResource {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_StatisticsResource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_StatisticsResource {}

impl BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_StatisticsResource {
    pub fn build(
        self,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_StatisticsResource {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_StatisticsResource {
            s3_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_StatisticsResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_StatisticsResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_StatisticsResourceRef {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_StatisticsResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_StatisticsResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_BaselineDynamic {
    constraints_resource: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_ConstraintsResource,
        >,
    >,
    statistics_resource: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_StatisticsResource,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    baselining_job_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    constraints_resource: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_ConstraintsResource,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statistics_resource: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_StatisticsResource,
        >,
    >,
    dynamic: SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_BaselineDynamic,
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline {
    #[doc = "Set the field `baselining_job_name`.\n"]
    pub fn set_baselining_job_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.baselining_job_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `constraints_resource`.\n"]
    pub fn set_constraints_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_ConstraintsResource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.constraints_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.constraints_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `statistics_resource`.\n"]
    pub fn set_statistics_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_StatisticsResource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.statistics_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.statistics_resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline {
    type O =
        terrars
        ::BlockAssignable<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline {}

impl BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline {
    pub fn build(self) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline {
            baselining_job_name: core::default::Default::default(),
            constraints_resource: core::default::Default::default(),
            statistics_resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_BaselineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_BaselineRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_BaselineRef {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_BaselineRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_BaselineRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `baselining_job_name` after provisioning.\n"]
    pub fn baselining_job_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.baselining_job_name", self.base))
    }

    #[doc = "Get a reference to the value of field `constraints_resource` after provisioning.\n"]
    pub fn constraints_resource(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_ConstraintsResourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.constraints_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `statistics_resource` after provisioning.\n"]
    pub fn statistics_resource(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline_StatisticsResourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statistics_resource", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringAppSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_arguments: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_entrypoint: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    image_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_analytics_processor_source_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_preprocessor_source_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringAppSpecification {
    #[doc = "Set the field `container_arguments`.\n"]
    pub fn set_container_arguments(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.container_arguments = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_entrypoint`.\n"]
    pub fn set_container_entrypoint(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.container_entrypoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `post_analytics_processor_source_uri`.\n"]
    pub fn set_post_analytics_processor_source_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.post_analytics_processor_source_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_preprocessor_source_uri`.\n"]
    pub fn set_record_preprocessor_source_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.record_preprocessor_source_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringAppSpecification {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringAppSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringAppSpecification {
    #[doc = ""]
    pub image_uri: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringAppSpecification {
    pub fn build(
        self,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringAppSpecification {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringAppSpecification {
            container_arguments: core::default::Default::default(),
            container_entrypoint: core::default::Default::default(),
            image_uri: self.image_uri,
            post_analytics_processor_source_uri: core::default::Default::default(),
            record_preprocessor_source_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringAppSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringAppSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringAppSpecificationRef {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringAppSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringAppSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_arguments` after provisioning.\n"]
    pub fn container_arguments(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_arguments", self.base))
    }

    #[doc = "Get a reference to the value of field `container_entrypoint` after provisioning.\n"]
    pub fn container_entrypoint(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_entrypoint", self.base))
    }

    #[doc = "Get a reference to the value of field `image_uri` after provisioning.\n"]
    pub fn image_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `post_analytics_processor_source_uri` after provisioning.\n"]
    pub fn post_analytics_processor_source_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.post_analytics_processor_source_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `record_preprocessor_source_uri` after provisioning.\n"]
    pub fn record_preprocessor_source_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_preprocessor_source_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Csv {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<terrars::PrimField<bool>>,
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Csv {
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.header = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Csv {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Csv,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Csv {}

impl BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Csv {
    pub fn build(
        self,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Csv {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Csv {
            header: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_CsvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_CsvRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_CsvRef {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_CsvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_CsvRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Json {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    line: std::option::Option<terrars::PrimField<bool>>,
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Json {
    #[doc = "Set the field `line`.\n"]
    pub fn set_line(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.line = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Json {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Json,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Json {}

impl BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Json {
    pub fn build(
        self,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Json {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Json {
            line: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_JsonRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_JsonRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_JsonRef {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_JsonRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_JsonRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `line` after provisioning.\n"]
    pub fn line(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.line", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormatDynamic {
    csv: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Csv,
        >,
    >,
    json: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Json,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Csv,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Json,
        >,
    >,
    dynamic: SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormatDynamic,
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat {
    #[doc = "Set the field `csv`.\n"]
    pub fn set_csv(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Csv,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.csv = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.csv = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `json`.\n"]
    pub fn set_json(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_Json,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.json = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.json = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat {}

impl BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat {
    pub fn build(
        self,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat {
            csv: core::default::Default::default(),
            json: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormatRef {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `csv` after provisioning.\n"]
    pub fn csv(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_CsvRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv", self.base))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat_JsonRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInputDynamic {
    dataset_format: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput {
    data_captured_destination_s3_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time_offset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_features_attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features_attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    local_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    probability_attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    probability_threshold_attribute: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_data_distribution_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_input_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time_offset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_format: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat,
        >,
    >,
    dynamic: SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInputDynamic,
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput {
    #[doc = "Set the field `end_time_offset`.\n"]
    pub fn set_end_time_offset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time_offset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_features_attribute`.\n"]
    pub fn set_exclude_features_attribute(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.exclude_features_attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `features_attribute`.\n"]
    pub fn set_features_attribute(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.features_attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inference_attribute`.\n"]
    pub fn set_inference_attribute(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.inference_attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `probability_attribute`.\n"]
    pub fn set_probability_attribute(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.probability_attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `probability_threshold_attribute`.\n"]
    pub fn set_probability_threshold_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.probability_threshold_attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_data_distribution_type`.\n"]
    pub fn set_s3_data_distribution_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.s3_data_distribution_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_input_mode`.\n"]
    pub fn set_s3_input_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_input_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time_offset`.\n"]
    pub fn set_start_time_offset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time_offset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dataset_format`.\n"]
    pub fn set_dataset_format(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormat,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dataset_format = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dataset_format = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput {
    #[doc = ""]
    pub data_captured_destination_s3_uri: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub local_path: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput {
    pub fn build(
        self,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput {
            data_captured_destination_s3_uri: self.data_captured_destination_s3_uri,
            end_time_offset: core::default::Default::default(),
            exclude_features_attribute: core::default::Default::default(),
            features_attribute: core::default::Default::default(),
            inference_attribute: core::default::Default::default(),
            local_path: self.local_path,
            probability_attribute: core::default::Default::default(),
            probability_threshold_attribute: core::default::Default::default(),
            s3_data_distribution_type: core::default::Default::default(),
            s3_input_mode: core::default::Default::default(),
            start_time_offset: core::default::Default::default(),
            dataset_format: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInputRef {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_captured_destination_s3_uri` after provisioning.\n"]
    pub fn data_captured_destination_s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_captured_destination_s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `end_time_offset` after provisioning.\n"]
    pub fn end_time_offset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time_offset", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_features_attribute` after provisioning.\n"]
    pub fn exclude_features_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_features_attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `features_attribute` after provisioning.\n"]
    pub fn features_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.features_attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `inference_attribute` after provisioning.\n"]
    pub fn inference_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inference_attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `local_path` after provisioning.\n"]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `probability_attribute` after provisioning.\n"]
    pub fn probability_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.probability_attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `probability_threshold_attribute` after provisioning.\n"]
    pub fn probability_threshold_attribute(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.probability_threshold_attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_data_distribution_type` after provisioning.\n"]
    pub fn s3_data_distribution_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_data_distribution_type", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_input_mode` after provisioning.\n"]
    pub fn s3_input_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_input_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time_offset` after provisioning.\n"]
    pub fn start_time_offset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time_offset", self.base))
    }

    #[doc = "Get a reference to the value of field `dataset_format` after provisioning.\n"]
    pub fn dataset_format(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput_DatasetFormatRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataset_format", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_EndpointInput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time_offset: std::option::Option<terrars::PrimField<std::string::String>>,
    endpoint_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_features_attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features_attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    local_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    probability_attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    probability_threshold_attribute: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_data_distribution_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_input_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time_offset: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_EndpointInput {
    #[doc = "Set the field `end_time_offset`.\n"]
    pub fn set_end_time_offset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time_offset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_features_attribute`.\n"]
    pub fn set_exclude_features_attribute(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.exclude_features_attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `features_attribute`.\n"]
    pub fn set_features_attribute(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.features_attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inference_attribute`.\n"]
    pub fn set_inference_attribute(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.inference_attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `probability_attribute`.\n"]
    pub fn set_probability_attribute(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.probability_attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `probability_threshold_attribute`.\n"]
    pub fn set_probability_threshold_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.probability_threshold_attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_data_distribution_type`.\n"]
    pub fn set_s3_data_distribution_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.s3_data_distribution_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_input_mode`.\n"]
    pub fn set_s3_input_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_input_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time_offset`.\n"]
    pub fn set_start_time_offset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time_offset = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_EndpointInput {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_EndpointInput,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_EndpointInput {
    #[doc = ""]
    pub endpoint_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub local_path: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_EndpointInput {
    pub fn build(
        self,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_EndpointInput {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_EndpointInput {
            end_time_offset: core::default::Default::default(),
            endpoint_name: self.endpoint_name,
            exclude_features_attribute: core::default::Default::default(),
            features_attribute: core::default::Default::default(),
            inference_attribute: core::default::Default::default(),
            local_path: self.local_path,
            probability_attribute: core::default::Default::default(),
            probability_threshold_attribute: core::default::Default::default(),
            s3_data_distribution_type: core::default::Default::default(),
            s3_input_mode: core::default::Default::default(),
            start_time_offset: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_EndpointInputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_EndpointInputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_EndpointInputRef {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_EndpointInputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_EndpointInputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_time_offset` after provisioning.\n"]
    pub fn end_time_offset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time_offset", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_name` after provisioning.\n"]
    pub fn endpoint_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_name", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_features_attribute` after provisioning.\n"]
    pub fn exclude_features_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_features_attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `features_attribute` after provisioning.\n"]
    pub fn features_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.features_attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `inference_attribute` after provisioning.\n"]
    pub fn inference_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inference_attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `local_path` after provisioning.\n"]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `probability_attribute` after provisioning.\n"]
    pub fn probability_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.probability_attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `probability_threshold_attribute` after provisioning.\n"]
    pub fn probability_threshold_attribute(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.probability_threshold_attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_data_distribution_type` after provisioning.\n"]
    pub fn s3_data_distribution_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_data_distribution_type", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_input_mode` after provisioning.\n"]
    pub fn s3_input_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_input_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time_offset` after provisioning.\n"]
    pub fn start_time_offset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time_offset", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputsDynamic {
    batch_transform_input: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput,
        >,
    >,
    endpoint_input: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_EndpointInput,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_transform_input: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_input: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_EndpointInput,
        >,
    >,
    dynamic: SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputsDynamic,
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs {
    #[doc = "Set the field `batch_transform_input`.\n"]
    pub fn set_batch_transform_input(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInput,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.batch_transform_input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.batch_transform_input = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `endpoint_input`.\n"]
    pub fn set_endpoint_input(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_EndpointInput,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.endpoint_input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.endpoint_input = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs {}

impl BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs {
    pub fn build(
        self,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs {
            batch_transform_input: core::default::Default::default(),
            endpoint_input: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputsRef {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `batch_transform_input` after provisioning.\n"]
    pub fn batch_transform_input(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_BatchTransformInputRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.batch_transform_input", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_input` after provisioning.\n"]
    pub fn endpoint_input(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs_EndpointInputRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_input", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs_S3Output {
    local_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_upload_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_uri: terrars::PrimField<std::string::String>,
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs_S3Output {
    #[doc = "Set the field `s3_upload_mode`.\n"]
    pub fn set_s3_upload_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_upload_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs_S3Output {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs_S3Output,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs_S3Output {
    #[doc = ""]
    pub local_path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs_S3Output {
    pub fn build(
        self,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs_S3Output {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs_S3Output {
            local_path: self.local_path,
            s3_upload_mode: core::default::Default::default(),
            s3_uri: self.s3_uri,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs_S3OutputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs_S3OutputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs_S3OutputRef {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs_S3OutputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs_S3OutputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `local_path` after provisioning.\n"]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_upload_mode` after provisioning.\n"]
    pub fn s3_upload_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_upload_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputsDynamic {
    s3_output: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs_S3Output,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_output: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs_S3Output,
        >,
    >,
    dynamic: SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputsDynamic,
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs {
    #[doc = "Set the field `s3_output`.\n"]
    pub fn set_s3_output(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs_S3Output,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_output = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_output = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs {}

impl BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs {
    pub fn build(
        self,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs {
            s3_output: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputsRef {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_output` after provisioning.\n"]
    pub fn s3_output(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs_S3OutputRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_output", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfigDynamic {
    monitoring_outputs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_outputs: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs,
        >,
    >,
    dynamic: SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfigDynamic,
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monitoring_outputs`.\n"]
    pub fn set_monitoring_outputs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.monitoring_outputs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.monitoring_outputs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig {}

impl BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig {
    pub fn build(
        self,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig {
            kms_key_id: core::default::Default::default(),
            monitoring_outputs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfigRef {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `monitoring_outputs` after provisioning.\n"]
    pub fn monitoring_outputs(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig_MonitoringOutputsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_outputs", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources_ClusterConfig {
    instance_count: terrars::PrimField<f64>,
    instance_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    volume_size_in_gb: terrars::PrimField<f64>,
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources_ClusterConfig {
    #[doc = "Set the field `volume_kms_key_id`.\n"]
    pub fn set_volume_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources_ClusterConfig {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources_ClusterConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources_ClusterConfig {
    #[doc = ""]
    pub instance_count: terrars::PrimField<f64>,
    #[doc = ""]
    pub instance_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub volume_size_in_gb: terrars::PrimField<f64>,
}

impl BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources_ClusterConfig {
    pub fn build(
        self,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources_ClusterConfig {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources_ClusterConfig {
            instance_count: self.instance_count,
            instance_type: self.instance_type,
            volume_kms_key_id: core::default::Default::default(),
            volume_size_in_gb: self.volume_size_in_gb,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources_ClusterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources_ClusterConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources_ClusterConfigRef {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources_ClusterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources_ClusterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_kms_key_id` after provisioning.\n"]
    pub fn volume_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size_in_gb` after provisioning.\n"]
    pub fn volume_size_in_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_size_in_gb", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResourcesDynamic {
    cluster_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources_ClusterConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources_ClusterConfig,
        >,
    >,
    dynamic: SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResourcesDynamic,
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources {
    #[doc = "Set the field `cluster_config`.\n"]
    pub fn set_cluster_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources_ClusterConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cluster_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cluster_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources {}

impl BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources {
    pub fn build(
        self,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources {
            cluster_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResourcesRef {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources_ClusterConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig_VpcConfig {
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig_VpcConfig { }

impl terrars
::ToListMappable for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig_VpcConfig {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig_VpcConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig_VpcConfig {
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig_VpcConfig {
    pub fn build(
        self,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig_VpcConfig {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig_VpcConfig {
            security_group_ids: self.security_group_ids,
            subnets: self.subnets,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig_VpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig_VpcConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig_VpcConfigRef {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig_VpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig_VpcConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfigDynamic {
    vpc_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig_VpcConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_inter_container_traffic_encryption: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_network_isolation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig_VpcConfig>,
    >,
    dynamic: SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfigDynamic,
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig {
    #[doc = "Set the field `enable_inter_container_traffic_encryption`.\n"]
    pub fn set_enable_inter_container_traffic_encryption(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_inter_container_traffic_encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_network_isolation`.\n"]
    pub fn set_enable_network_isolation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_network_isolation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig_VpcConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig {
    type O =
        terrars
        ::BlockAssignable<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig {}

impl BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig {
    pub fn build(self) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig {
            enable_inter_container_traffic_encryption: core::default::Default::default(),
            enable_network_isolation: core::default::Default::default(),
            vpc_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfigRef {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_inter_container_traffic_encryption` after provisioning.\n"]
    pub fn enable_inter_container_traffic_encryption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_inter_container_traffic_encryption", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enable_network_isolation` after provisioning.\n"]
    pub fn enable_network_isolation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_network_isolation", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig_VpcConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_StoppingCondition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_runtime_in_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_StoppingCondition {
    #[doc = "Set the field `max_runtime_in_seconds`.\n"]
    pub fn set_max_runtime_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_runtime_in_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_StoppingCondition {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_StoppingCondition,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_StoppingCondition {}

impl BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_StoppingCondition {
    pub fn build(
        self,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_StoppingCondition {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_StoppingCondition {
            max_runtime_in_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_StoppingConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_StoppingConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_StoppingConditionRef {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_StoppingConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_StoppingConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_runtime_in_seconds` after provisioning.\n"]
    pub fn max_runtime_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_runtime_in_seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinitionDynamic {
    baseline: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline>,
    >,
    monitoring_app_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringAppSpecification,
        >,
    >,
    monitoring_inputs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs>,
    >,
    monitoring_output_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig,
        >,
    >,
    monitoring_resources: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources,
        >,
    >,
    network_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig>,
    >,
    stopping_condition: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_StoppingCondition>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    baseline: std
    ::option
    ::Option<std::vec::Vec<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_app_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringAppSpecification>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_inputs: std
    ::option
    ::Option<
        std::vec::Vec<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_output_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_resources: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std
    ::option
    ::Option<
        std::vec::Vec<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stopping_condition: std
    ::option
    ::Option<
        std::vec::Vec<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_StoppingCondition>,
    >,
    dynamic: SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinitionDynamic,
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition {
    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `baseline`.\n"]
    pub fn set_baseline(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_Baseline,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.baseline = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.baseline = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monitoring_app_specification`.\n"]
    pub fn set_monitoring_app_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringAppSpecification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.monitoring_app_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.monitoring_app_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monitoring_inputs`.\n"]
    pub fn set_monitoring_inputs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.monitoring_inputs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.monitoring_inputs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monitoring_output_config`.\n"]
    pub fn set_monitoring_output_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.monitoring_output_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.monitoring_output_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monitoring_resources`.\n"]
    pub fn set_monitoring_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResources,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.monitoring_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.monitoring_resources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stopping_condition`.\n"]
    pub fn set_stopping_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_StoppingCondition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.stopping_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.stopping_condition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition {
    type O = terrars::BlockAssignable<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition {
    pub fn build(self) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition {
            environment: core::default::Default::default(),
            role_arn: self.role_arn,
            baseline: core::default::Default::default(),
            monitoring_app_specification: core::default::Default::default(),
            monitoring_inputs: core::default::Default::default(),
            monitoring_output_config: core::default::Default::default(),
            monitoring_resources: core::default::Default::default(),
            network_config: core::default::Default::default(),
            stopping_condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinitionRef {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `baseline` after provisioning.\n"]
    pub fn baseline(
        &self,
    ) -> terrars::ListRef<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_BaselineRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.baseline", self.base))
    }

    #[doc = "Get a reference to the value of field `monitoring_app_specification` after provisioning.\n"]
    pub fn monitoring_app_specification(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringAppSpecificationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_app_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `monitoring_inputs` after provisioning.\n"]
    pub fn monitoring_inputs(
        &self,
    ) -> terrars
    ::ListRef<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringInputsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_inputs", self.base))
    }

    #[doc = "Get a reference to the value of field `monitoring_output_config` after provisioning.\n"]
    pub fn monitoring_output_config(
        &self,
    ) -> terrars
    ::ListRef<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringOutputConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_output_config", self.base))
    }

    #[doc = "Get a reference to the value of field `monitoring_resources` after provisioning.\n"]
    pub fn monitoring_resources(
        &self,
    ) -> terrars
    ::ListRef<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_MonitoringResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_resources", self.base))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(
        &self,
    ) -> terrars
    ::ListRef<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.base))
    }

    #[doc = "Get a reference to the value of field `stopping_condition` after provisioning.\n"]
    pub fn stopping_condition(
        &self,
    ) -> terrars
    ::ListRef<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition_StoppingConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stopping_condition", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_ScheduleConfig {
    schedule_expression: terrars::PrimField<std::string::String>,
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_ScheduleConfig { }

impl terrars::ToListMappable for SagemakerMonitoringSchedule_MonitoringScheduleConfig_ScheduleConfig {
    type O = terrars::BlockAssignable<SagemakerMonitoringSchedule_MonitoringScheduleConfig_ScheduleConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_ScheduleConfig {
    #[doc = ""]
    pub schedule_expression: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig_ScheduleConfig {
    pub fn build(self) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_ScheduleConfig {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_ScheduleConfig {
            schedule_expression: self.schedule_expression,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig_ScheduleConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerMonitoringSchedule_MonitoringScheduleConfig_ScheduleConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig_ScheduleConfigRef {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig_ScheduleConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig_ScheduleConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerMonitoringSchedule_MonitoringScheduleConfigDynamic {
    monitoring_job_definition: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition>>,
    schedule_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerMonitoringSchedule_MonitoringScheduleConfig_ScheduleConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_job_definition_name: std::option::Option<terrars::PrimField<std::string::String>>,
    monitoring_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_job_definition: std
    ::option
    ::Option<std::vec::Vec<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerMonitoringSchedule_MonitoringScheduleConfig_ScheduleConfig>>,
    dynamic: SagemakerMonitoringSchedule_MonitoringScheduleConfigDynamic,
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfig {
    #[doc = "Set the field `monitoring_job_definition_name`.\n"]
    pub fn set_monitoring_job_definition_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.monitoring_job_definition_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monitoring_job_definition`.\n"]
    pub fn set_monitoring_job_definition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.monitoring_job_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.monitoring_job_definition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schedule_config`.\n"]
    pub fn set_schedule_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerMonitoringSchedule_MonitoringScheduleConfig_ScheduleConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schedule_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schedule_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerMonitoringSchedule_MonitoringScheduleConfig {
    type O = terrars::BlockAssignable<SagemakerMonitoringSchedule_MonitoringScheduleConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig {
    #[doc = ""]
    pub monitoring_type: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerMonitoringSchedule_MonitoringScheduleConfig {
    pub fn build(self) -> SagemakerMonitoringSchedule_MonitoringScheduleConfig {
        SagemakerMonitoringSchedule_MonitoringScheduleConfig {
            monitoring_job_definition_name: core::default::Default::default(),
            monitoring_type: self.monitoring_type,
            monitoring_job_definition: core::default::Default::default(),
            schedule_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerMonitoringSchedule_MonitoringScheduleConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerMonitoringSchedule_MonitoringScheduleConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerMonitoringSchedule_MonitoringScheduleConfigRef {
        SagemakerMonitoringSchedule_MonitoringScheduleConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerMonitoringSchedule_MonitoringScheduleConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `monitoring_job_definition_name` after provisioning.\n"]
    pub fn monitoring_job_definition_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring_job_definition_name", self.base))
    }

    #[doc = "Get a reference to the value of field `monitoring_type` after provisioning.\n"]
    pub fn monitoring_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring_type", self.base))
    }

    #[doc = "Get a reference to the value of field `monitoring_job_definition` after provisioning.\n"]
    pub fn monitoring_job_definition(
        &self,
    ) -> terrars::ListRef<SagemakerMonitoringSchedule_MonitoringScheduleConfig_MonitoringJobDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_job_definition", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_config` after provisioning.\n"]
    pub fn schedule_config(
        &self,
    ) -> terrars::ListRef<SagemakerMonitoringSchedule_MonitoringScheduleConfig_ScheduleConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule_config", self.base))
    }
}
