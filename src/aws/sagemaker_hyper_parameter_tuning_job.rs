#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerHyperParameterTuningJobData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autotune: std::option::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_Autotune>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_Config>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SagemakerHyperParameterTuningJob_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    training_job_definition: std::option::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    training_job_definitions: std::option::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinitions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warm_start_config: std::option::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_WarmStartConfig>>,
    dynamic: SagemakerHyperParameterTuningJobDynamic,
}

struct SagemakerHyperParameterTuningJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerHyperParameterTuningJobData_>,
}

#[derive(Clone)]
pub struct SagemakerHyperParameterTuningJob(std::rc::Rc<SagemakerHyperParameterTuningJob_>);

impl SagemakerHyperParameterTuningJob {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autotune`.\n"]
    pub fn set_autotune(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerHyperParameterTuningJob_Autotune>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().autotune = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.autotune = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerHyperParameterTuningJob_Config>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<SagemakerHyperParameterTuningJob_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `training_job_definition`.\n"]
    pub fn set_training_job_definition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().training_job_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.training_job_definition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `training_job_definitions`.\n"]
    pub fn set_training_job_definitions(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinitions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().training_job_definitions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.training_job_definitions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `warm_start_config`.\n"]
    pub fn set_warm_start_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerHyperParameterTuningJob_WarmStartConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().warm_start_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.warm_start_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\n"]
    pub fn failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `autotune` after provisioning.\n"]
    pub fn autotune(&self) -> terrars::ListRef<SagemakerHyperParameterTuningJob_AutotuneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autotune", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<SagemakerHyperParameterTuningJob_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SagemakerHyperParameterTuningJob_TimeoutsRef {
        SagemakerHyperParameterTuningJob_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `training_job_definition` after provisioning.\n"]
    pub fn training_job_definition(&self) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.training_job_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `training_job_definitions` after provisioning.\n"]
    pub fn training_job_definitions(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinitionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.training_job_definitions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_start_config` after provisioning.\n"]
    pub fn warm_start_config(&self) -> terrars::ListRef<SagemakerHyperParameterTuningJob_WarmStartConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warm_start_config", self.extract_ref()))
    }
}

impl terrars::Referable for SagemakerHyperParameterTuningJob {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerHyperParameterTuningJob { }

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob {
    type O = terrars::ListRef<SagemakerHyperParameterTuningJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerHyperParameterTuningJob_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_hyper_parameter_tuning_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerHyperParameterTuningJob {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerHyperParameterTuningJob {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerHyperParameterTuningJob {
        let out = SagemakerHyperParameterTuningJob(std::rc::Rc::new(SagemakerHyperParameterTuningJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerHyperParameterTuningJobData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                autotune: core::default::Default::default(),
                config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                training_job_definition: core::default::Default::default(),
                training_job_definitions: core::default::Default::default(),
                warm_start_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerHyperParameterTuningJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerHyperParameterTuningJobRef {
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

    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\n"]
    pub fn failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `autotune` after provisioning.\n"]
    pub fn autotune(&self) -> terrars::ListRef<SagemakerHyperParameterTuningJob_AutotuneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autotune", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<SagemakerHyperParameterTuningJob_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SagemakerHyperParameterTuningJob_TimeoutsRef {
        SagemakerHyperParameterTuningJob_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `training_job_definition` after provisioning.\n"]
    pub fn training_job_definition(&self) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.training_job_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `training_job_definitions` after provisioning.\n"]
    pub fn training_job_definitions(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinitionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.training_job_definitions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_start_config` after provisioning.\n"]
    pub fn warm_start_config(&self) -> terrars::ListRef<SagemakerHyperParameterTuningJob_WarmStartConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warm_start_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SagemakerHyperParameterTuningJobDynamic {
    autotune: std::option::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_Autotune>>,
    config: std::option::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_Config>>,
    training_job_definition: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition>>,
    training_job_definitions: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinitions>>,
    warm_start_config: std::option::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_WarmStartConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_Autotune {
    mode: terrars::PrimField<std::string::String>,
}

impl SagemakerHyperParameterTuningJob_Autotune { }

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_Autotune {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_Autotune>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_Autotune {
    #[doc = ""]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerHyperParameterTuningJob_Autotune {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_Autotune {
        SagemakerHyperParameterTuningJob_Autotune { mode: self.mode }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_AutotuneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_AutotuneRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerHyperParameterTuningJob_AutotuneRef {
        SagemakerHyperParameterTuningJob_AutotuneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_AutotuneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_Config_Objective {
    metric_name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl SagemakerHyperParameterTuningJob_Config_Objective { }

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_Config_Objective {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_Config_Objective>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_Config_Objective {
    #[doc = ""]
    pub metric_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerHyperParameterTuningJob_Config_Objective {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_Config_Objective {
        SagemakerHyperParameterTuningJob_Config_Objective {
            metric_name: self.metric_name,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_Config_ObjectiveRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_Config_ObjectiveRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_Config_ObjectiveRef {
        SagemakerHyperParameterTuningJob_Config_ObjectiveRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_Config_ObjectiveRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_Config_ParameterRanges_AutoParameters {
    name: terrars::PrimField<std::string::String>,
    value_hint: terrars::PrimField<std::string::String>,
}

impl SagemakerHyperParameterTuningJob_Config_ParameterRanges_AutoParameters { }

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_Config_ParameterRanges_AutoParameters {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_Config_ParameterRanges_AutoParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_Config_ParameterRanges_AutoParameters {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value_hint: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerHyperParameterTuningJob_Config_ParameterRanges_AutoParameters {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_Config_ParameterRanges_AutoParameters {
        SagemakerHyperParameterTuningJob_Config_ParameterRanges_AutoParameters {
            name: self.name,
            value_hint: self.value_hint,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_Config_ParameterRanges_AutoParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_Config_ParameterRanges_AutoParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_Config_ParameterRanges_AutoParametersRef {
        SagemakerHyperParameterTuningJob_Config_ParameterRanges_AutoParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_Config_ParameterRanges_AutoParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value_hint` after provisioning.\n"]
    pub fn value_hint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_hint", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_Config_ParameterRanges_CategoricalParameterRanges {
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl SagemakerHyperParameterTuningJob_Config_ParameterRanges_CategoricalParameterRanges { }

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_Config_ParameterRanges_CategoricalParameterRanges {
    type O =
        terrars::BlockAssignable<SagemakerHyperParameterTuningJob_Config_ParameterRanges_CategoricalParameterRanges>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_Config_ParameterRanges_CategoricalParameterRanges {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildSagemakerHyperParameterTuningJob_Config_ParameterRanges_CategoricalParameterRanges {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_Config_ParameterRanges_CategoricalParameterRanges {
        SagemakerHyperParameterTuningJob_Config_ParameterRanges_CategoricalParameterRanges {
            name: self.name,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_Config_ParameterRanges_CategoricalParameterRangesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_Config_ParameterRanges_CategoricalParameterRangesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_Config_ParameterRanges_CategoricalParameterRangesRef {
        SagemakerHyperParameterTuningJob_Config_ParameterRanges_CategoricalParameterRangesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_Config_ParameterRanges_CategoricalParameterRangesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_Config_ParameterRanges_ContinuousParameterRanges {
    max_value: terrars::PrimField<std::string::String>,
    min_value: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerHyperParameterTuningJob_Config_ParameterRanges_ContinuousParameterRanges {
    #[doc = "Set the field `scaling_type`.\n"]
    pub fn set_scaling_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scaling_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_Config_ParameterRanges_ContinuousParameterRanges {
    type O =
        terrars::BlockAssignable<SagemakerHyperParameterTuningJob_Config_ParameterRanges_ContinuousParameterRanges>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_Config_ParameterRanges_ContinuousParameterRanges {
    #[doc = ""]
    pub max_value: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub min_value: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerHyperParameterTuningJob_Config_ParameterRanges_ContinuousParameterRanges {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_Config_ParameterRanges_ContinuousParameterRanges {
        SagemakerHyperParameterTuningJob_Config_ParameterRanges_ContinuousParameterRanges {
            max_value: self.max_value,
            min_value: self.min_value,
            name: self.name,
            scaling_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_Config_ParameterRanges_ContinuousParameterRangesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_Config_ParameterRanges_ContinuousParameterRangesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_Config_ParameterRanges_ContinuousParameterRangesRef {
        SagemakerHyperParameterTuningJob_Config_ParameterRanges_ContinuousParameterRangesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_Config_ParameterRanges_ContinuousParameterRangesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_value` after provisioning.\n"]
    pub fn max_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_value", self.base))
    }

    #[doc = "Get a reference to the value of field `min_value` after provisioning.\n"]
    pub fn min_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_value", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `scaling_type` after provisioning.\n"]
    pub fn scaling_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scaling_type", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_Config_ParameterRanges_IntegerParameterRanges {
    max_value: terrars::PrimField<std::string::String>,
    min_value: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerHyperParameterTuningJob_Config_ParameterRanges_IntegerParameterRanges {
    #[doc = "Set the field `scaling_type`.\n"]
    pub fn set_scaling_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scaling_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_Config_ParameterRanges_IntegerParameterRanges {
    type O =
        terrars::BlockAssignable<SagemakerHyperParameterTuningJob_Config_ParameterRanges_IntegerParameterRanges>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_Config_ParameterRanges_IntegerParameterRanges {
    #[doc = ""]
    pub max_value: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub min_value: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerHyperParameterTuningJob_Config_ParameterRanges_IntegerParameterRanges {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_Config_ParameterRanges_IntegerParameterRanges {
        SagemakerHyperParameterTuningJob_Config_ParameterRanges_IntegerParameterRanges {
            max_value: self.max_value,
            min_value: self.min_value,
            name: self.name,
            scaling_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_Config_ParameterRanges_IntegerParameterRangesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_Config_ParameterRanges_IntegerParameterRangesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_Config_ParameterRanges_IntegerParameterRangesRef {
        SagemakerHyperParameterTuningJob_Config_ParameterRanges_IntegerParameterRangesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_Config_ParameterRanges_IntegerParameterRangesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_value` after provisioning.\n"]
    pub fn max_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_value", self.base))
    }

    #[doc = "Get a reference to the value of field `min_value` after provisioning.\n"]
    pub fn min_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_value", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `scaling_type` after provisioning.\n"]
    pub fn scaling_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scaling_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerHyperParameterTuningJob_Config_ParameterRangesDynamic {
    auto_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_Config_ParameterRanges_AutoParameters>>,
    categorical_parameter_ranges: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerHyperParameterTuningJob_Config_ParameterRanges_CategoricalParameterRanges>,
    >,
    continuous_parameter_ranges: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerHyperParameterTuningJob_Config_ParameterRanges_ContinuousParameterRanges>,
    >,
    integer_parameter_ranges: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_Config_ParameterRanges_IntegerParameterRanges>>,
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_Config_ParameterRanges {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_parameters: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_Config_ParameterRanges_AutoParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    categorical_parameter_ranges: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_Config_ParameterRanges_CategoricalParameterRanges>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    continuous_parameter_ranges: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_Config_ParameterRanges_ContinuousParameterRanges>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integer_parameter_ranges: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_Config_ParameterRanges_IntegerParameterRanges>>,
    dynamic: SagemakerHyperParameterTuningJob_Config_ParameterRangesDynamic,
}

impl SagemakerHyperParameterTuningJob_Config_ParameterRanges {
    #[doc = "Set the field `auto_parameters`.\n"]
    pub fn set_auto_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerHyperParameterTuningJob_Config_ParameterRanges_AutoParameters>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auto_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auto_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `categorical_parameter_ranges`.\n"]
    pub fn set_categorical_parameter_ranges(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_Config_ParameterRanges_CategoricalParameterRanges,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.categorical_parameter_ranges = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.categorical_parameter_ranges = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `continuous_parameter_ranges`.\n"]
    pub fn set_continuous_parameter_ranges(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_Config_ParameterRanges_ContinuousParameterRanges,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.continuous_parameter_ranges = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.continuous_parameter_ranges = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `integer_parameter_ranges`.\n"]
    pub fn set_integer_parameter_ranges(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_Config_ParameterRanges_IntegerParameterRanges,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.integer_parameter_ranges = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.integer_parameter_ranges = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_Config_ParameterRanges {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_Config_ParameterRanges>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_Config_ParameterRanges {}

impl BuildSagemakerHyperParameterTuningJob_Config_ParameterRanges {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_Config_ParameterRanges {
        SagemakerHyperParameterTuningJob_Config_ParameterRanges {
            auto_parameters: core::default::Default::default(),
            categorical_parameter_ranges: core::default::Default::default(),
            continuous_parameter_ranges: core::default::Default::default(),
            integer_parameter_ranges: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_Config_ParameterRangesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_Config_ParameterRangesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_Config_ParameterRangesRef {
        SagemakerHyperParameterTuningJob_Config_ParameterRangesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_Config_ParameterRangesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_parameters` after provisioning.\n"]
    pub fn auto_parameters(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_Config_ParameterRanges_AutoParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `categorical_parameter_ranges` after provisioning.\n"]
    pub fn categorical_parameter_ranges(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_Config_ParameterRanges_CategoricalParameterRangesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.categorical_parameter_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `continuous_parameter_ranges` after provisioning.\n"]
    pub fn continuous_parameter_ranges(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_Config_ParameterRanges_ContinuousParameterRangesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.continuous_parameter_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `integer_parameter_ranges` after provisioning.\n"]
    pub fn integer_parameter_ranges(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_Config_ParameterRanges_IntegerParameterRangesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integer_parameter_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_Config_ResourceLimits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_number_of_training_jobs: std::option::Option<terrars::PrimField<f64>>,
    max_parallel_training_jobs: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_runtime_in_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerHyperParameterTuningJob_Config_ResourceLimits {
    #[doc = "Set the field `max_number_of_training_jobs`.\n"]
    pub fn set_max_number_of_training_jobs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_number_of_training_jobs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_runtime_in_seconds`.\n"]
    pub fn set_max_runtime_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_runtime_in_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_Config_ResourceLimits {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_Config_ResourceLimits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_Config_ResourceLimits {
    #[doc = ""]
    pub max_parallel_training_jobs: terrars::PrimField<f64>,
}

impl BuildSagemakerHyperParameterTuningJob_Config_ResourceLimits {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_Config_ResourceLimits {
        SagemakerHyperParameterTuningJob_Config_ResourceLimits {
            max_number_of_training_jobs: core::default::Default::default(),
            max_parallel_training_jobs: self.max_parallel_training_jobs,
            max_runtime_in_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_Config_ResourceLimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_Config_ResourceLimitsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_Config_ResourceLimitsRef {
        SagemakerHyperParameterTuningJob_Config_ResourceLimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_Config_ResourceLimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_number_of_training_jobs` after provisioning.\n"]
    pub fn max_number_of_training_jobs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_number_of_training_jobs", self.base))
    }

    #[doc = "Get a reference to the value of field `max_parallel_training_jobs` after provisioning.\n"]
    pub fn max_parallel_training_jobs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_parallel_training_jobs", self.base))
    }

    #[doc = "Get a reference to the value of field `max_runtime_in_seconds` after provisioning.\n"]
    pub fn max_runtime_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_runtime_in_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_Config_StrategyConfig_HyperbandStrategyConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_resource: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_resource: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerHyperParameterTuningJob_Config_StrategyConfig_HyperbandStrategyConfig {
    #[doc = "Set the field `max_resource`.\n"]
    pub fn set_max_resource(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_resource = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_resource`.\n"]
    pub fn set_min_resource(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_resource = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_Config_StrategyConfig_HyperbandStrategyConfig {
    type O =
        terrars::BlockAssignable<SagemakerHyperParameterTuningJob_Config_StrategyConfig_HyperbandStrategyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_Config_StrategyConfig_HyperbandStrategyConfig {}

impl BuildSagemakerHyperParameterTuningJob_Config_StrategyConfig_HyperbandStrategyConfig {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_Config_StrategyConfig_HyperbandStrategyConfig {
        SagemakerHyperParameterTuningJob_Config_StrategyConfig_HyperbandStrategyConfig {
            max_resource: core::default::Default::default(),
            min_resource: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_Config_StrategyConfig_HyperbandStrategyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_Config_StrategyConfig_HyperbandStrategyConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_Config_StrategyConfig_HyperbandStrategyConfigRef {
        SagemakerHyperParameterTuningJob_Config_StrategyConfig_HyperbandStrategyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_Config_StrategyConfig_HyperbandStrategyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_resource` after provisioning.\n"]
    pub fn max_resource(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `min_resource` after provisioning.\n"]
    pub fn min_resource(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerHyperParameterTuningJob_Config_StrategyConfigDynamic {
    hyperband_strategy_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_Config_StrategyConfig_HyperbandStrategyConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_Config_StrategyConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hyperband_strategy_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_Config_StrategyConfig_HyperbandStrategyConfig>>,
    dynamic: SagemakerHyperParameterTuningJob_Config_StrategyConfigDynamic,
}

impl SagemakerHyperParameterTuningJob_Config_StrategyConfig {
    #[doc = "Set the field `hyperband_strategy_config`.\n"]
    pub fn set_hyperband_strategy_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_Config_StrategyConfig_HyperbandStrategyConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hyperband_strategy_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hyperband_strategy_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_Config_StrategyConfig {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_Config_StrategyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_Config_StrategyConfig {}

impl BuildSagemakerHyperParameterTuningJob_Config_StrategyConfig {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_Config_StrategyConfig {
        SagemakerHyperParameterTuningJob_Config_StrategyConfig {
            hyperband_strategy_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_Config_StrategyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_Config_StrategyConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_Config_StrategyConfigRef {
        SagemakerHyperParameterTuningJob_Config_StrategyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_Config_StrategyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hyperband_strategy_config` after provisioning.\n"]
    pub fn hyperband_strategy_config(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_Config_StrategyConfig_HyperbandStrategyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hyperband_strategy_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_BestObjectiveNotImproving {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_number_of_training_jobs_not_improving: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_BestObjectiveNotImproving {
    #[doc = "Set the field `max_number_of_training_jobs_not_improving`.\n"]
    pub fn set_max_number_of_training_jobs_not_improving(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.max_number_of_training_jobs_not_improving = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_BestObjectiveNotImproving {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_BestObjectiveNotImproving,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_BestObjectiveNotImproving {}

impl BuildSagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_BestObjectiveNotImproving {
    pub fn build(
        self,
    ) -> SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_BestObjectiveNotImproving {
        SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_BestObjectiveNotImproving {
            max_number_of_training_jobs_not_improving: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_BestObjectiveNotImprovingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_BestObjectiveNotImprovingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_BestObjectiveNotImprovingRef {
        SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_BestObjectiveNotImprovingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_BestObjectiveNotImprovingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_number_of_training_jobs_not_improving` after provisioning.\n"]
    pub fn max_number_of_training_jobs_not_improving(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_number_of_training_jobs_not_improving", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_ConvergenceDetected {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    complete_on_convergence: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_ConvergenceDetected {
    #[doc = "Set the field `complete_on_convergence`.\n"]
    pub fn set_complete_on_convergence(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.complete_on_convergence = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_ConvergenceDetected {
    type O =
        terrars
        ::BlockAssignable<SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_ConvergenceDetected>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_ConvergenceDetected {}

impl BuildSagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_ConvergenceDetected {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_ConvergenceDetected {
        SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_ConvergenceDetected {
            complete_on_convergence: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_ConvergenceDetectedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_ConvergenceDetectedRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_ConvergenceDetectedRef {
        SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_ConvergenceDetectedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_ConvergenceDetectedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `complete_on_convergence` after provisioning.\n"]
    pub fn complete_on_convergence(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.complete_on_convergence", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteriaDynamic {
    best_objective_not_improving: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_BestObjectiveNotImproving>,
    >,
    convergence_detected: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_ConvergenceDetected>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_objective_metric_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    best_objective_not_improving: std
    ::option
    ::Option<
        std::vec::Vec<SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_BestObjectiveNotImproving>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    convergence_detected: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_ConvergenceDetected>>,
    dynamic: SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteriaDynamic,
}

impl SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria {
    #[doc = "Set the field `target_objective_metric_value`.\n"]
    pub fn set_target_objective_metric_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_objective_metric_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `best_objective_not_improving`.\n"]
    pub fn set_best_objective_not_improving(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_BestObjectiveNotImproving,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.best_objective_not_improving = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.best_objective_not_improving = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `convergence_detected`.\n"]
    pub fn set_convergence_detected(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_ConvergenceDetected,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.convergence_detected = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.convergence_detected = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria {}

impl BuildSagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria {
        SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria {
            target_objective_metric_value: core::default::Default::default(),
            best_objective_not_improving: core::default::Default::default(),
            convergence_detected: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteriaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteriaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteriaRef {
        SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteriaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteriaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_objective_metric_value` after provisioning.\n"]
    pub fn target_objective_metric_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_objective_metric_value", self.base))
    }

    #[doc = "Get a reference to the value of field `best_objective_not_improving` after provisioning.\n"]
    pub fn best_objective_not_improving(
        &self,
    ) -> terrars
    ::ListRef<SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_BestObjectiveNotImprovingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.best_objective_not_improving", self.base))
    }

    #[doc = "Get a reference to the value of field `convergence_detected` after provisioning.\n"]
    pub fn convergence_detected(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria_ConvergenceDetectedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.convergence_detected", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerHyperParameterTuningJob_ConfigDynamic {
    objective: std::option::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_Config_Objective>>,
    parameter_ranges: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_Config_ParameterRanges>>,
    resource_limits: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_Config_ResourceLimits>>,
    strategy_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_Config_StrategyConfig>>,
    tuning_job_completion_criteria: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria>>,
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    random_seed: std::option::Option<terrars::PrimField<f64>>,
    strategy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    training_job_early_stopping_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    objective: std::option::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_Config_Objective>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_ranges: std::option::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_Config_ParameterRanges>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_limits: std::option::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_Config_ResourceLimits>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strategy_config: std::option::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_Config_StrategyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tuning_job_completion_criteria: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria>>,
    dynamic: SagemakerHyperParameterTuningJob_ConfigDynamic,
}

impl SagemakerHyperParameterTuningJob_Config {
    #[doc = "Set the field `random_seed`.\n"]
    pub fn set_random_seed(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.random_seed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `training_job_early_stopping_type`.\n"]
    pub fn set_training_job_early_stopping_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.training_job_early_stopping_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `objective`.\n"]
    pub fn set_objective(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerHyperParameterTuningJob_Config_Objective>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.objective = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.objective = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parameter_ranges`.\n"]
    pub fn set_parameter_ranges(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerHyperParameterTuningJob_Config_ParameterRanges>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameter_ranges = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameter_ranges = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_limits`.\n"]
    pub fn set_resource_limits(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerHyperParameterTuningJob_Config_ResourceLimits>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_limits = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_limits = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `strategy_config`.\n"]
    pub fn set_strategy_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerHyperParameterTuningJob_Config_StrategyConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.strategy_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.strategy_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tuning_job_completion_criteria`.\n"]
    pub fn set_tuning_job_completion_criteria(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteria>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tuning_job_completion_criteria = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tuning_job_completion_criteria = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_Config {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_Config {
    #[doc = ""]
    pub strategy: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerHyperParameterTuningJob_Config {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_Config {
        SagemakerHyperParameterTuningJob_Config {
            random_seed: core::default::Default::default(),
            strategy: self.strategy,
            training_job_early_stopping_type: core::default::Default::default(),
            objective: core::default::Default::default(),
            parameter_ranges: core::default::Default::default(),
            resource_limits: core::default::Default::default(),
            strategy_config: core::default::Default::default(),
            tuning_job_completion_criteria: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerHyperParameterTuningJob_ConfigRef {
        SagemakerHyperParameterTuningJob_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `random_seed` after provisioning.\n"]
    pub fn random_seed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.random_seed", self.base))
    }

    #[doc = "Get a reference to the value of field `strategy` after provisioning.\n"]
    pub fn strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `training_job_early_stopping_type` after provisioning.\n"]
    pub fn training_job_early_stopping_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.training_job_early_stopping_type", self.base))
    }

    #[doc = "Get a reference to the value of field `objective` after provisioning.\n"]
    pub fn objective(&self) -> terrars::ListRef<SagemakerHyperParameterTuningJob_Config_ObjectiveRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.objective", self.base))
    }

    #[doc = "Get a reference to the value of field `parameter_ranges` after provisioning.\n"]
    pub fn parameter_ranges(&self) -> terrars::ListRef<SagemakerHyperParameterTuningJob_Config_ParameterRangesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameter_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_limits` after provisioning.\n"]
    pub fn resource_limits(&self) -> terrars::ListRef<SagemakerHyperParameterTuningJob_Config_ResourceLimitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_limits", self.base))
    }

    #[doc = "Get a reference to the value of field `strategy_config` after provisioning.\n"]
    pub fn strategy_config(&self) -> terrars::ListRef<SagemakerHyperParameterTuningJob_Config_StrategyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.strategy_config", self.base))
    }

    #[doc = "Get a reference to the value of field `tuning_job_completion_criteria` after provisioning.\n"]
    pub fn tuning_job_completion_criteria(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_Config_TuningJobCompletionCriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tuning_job_completion_criteria", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerHyperParameterTuningJob_Timeouts {
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
}

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_Timeouts {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_Timeouts {}

impl BuildSagemakerHyperParameterTuningJob_Timeouts {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_Timeouts {
        SagemakerHyperParameterTuningJob_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerHyperParameterTuningJob_TimeoutsRef {
        SagemakerHyperParameterTuningJob_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TimeoutsRef {
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
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_RetryStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_retry_attempts: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_RetryStrategy {
    #[doc = "Set the field `maximum_retry_attempts`.\n"]
    pub fn set_maximum_retry_attempts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_retry_attempts = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_RetryStrategy {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_RetryStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_RetryStrategy {}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_RetryStrategy {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_RetryStrategy {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_RetryStrategy {
            maximum_retry_attempts: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_RetryStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_RetryStrategyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_RetryStrategyRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_RetryStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_RetryStrategyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_retry_attempts` after provisioning.\n"]
    pub fn maximum_retry_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_retry_attempts", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification_MetricDefinitions {
    name: terrars::PrimField<std::string::String>,
    regex: terrars::PrimField<std::string::String>,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification_MetricDefinitions { }

impl terrars
::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification_MetricDefinitions {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification_MetricDefinitions,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification_MetricDefinitions {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub regex: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification_MetricDefinitions {
    pub fn build(
        self,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification_MetricDefinitions {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification_MetricDefinitions {
            name: self.name,
            regex: self.regex,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification_MetricDefinitionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification_MetricDefinitionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification_MetricDefinitionsRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification_MetricDefinitionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification_MetricDefinitionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecificationDynamic {
    metric_definitions: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification_MetricDefinitions,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    training_image: std::option::Option<terrars::PrimField<std::string::String>>,
    training_input_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_definitions: std
    ::option
    ::Option<
        std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification_MetricDefinitions>,
    >,
    dynamic: SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecificationDynamic,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification {
    #[doc = "Set the field `algorithm_name`.\n"]
    pub fn set_algorithm_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.algorithm_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `training_image`.\n"]
    pub fn set_training_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.training_image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric_definitions`.\n"]
    pub fn set_metric_definitions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification_MetricDefinitions,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric_definitions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_definitions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification {
    #[doc = ""]
    pub training_input_mode: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification {
            algorithm_name: core::default::Default::default(),
            training_image: core::default::Default::default(),
            training_input_mode: self.training_input_mode,
            metric_definitions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecificationRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `algorithm_name` after provisioning.\n"]
    pub fn algorithm_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm_name", self.base))
    }

    #[doc = "Get a reference to the value of field `training_image` after provisioning.\n"]
    pub fn training_image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.training_image", self.base))
    }

    #[doc = "Get a reference to the value of field `training_input_mode` after provisioning.\n"]
    pub fn training_input_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.training_input_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_definitions` after provisioning.\n"]
    pub fn metric_definitions(
        &self,
    ) -> terrars
    ::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification_MetricDefinitionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_definitions", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_CheckpointConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_path: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_uri: terrars::PrimField<std::string::String>,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_CheckpointConfig {
    #[doc = "Set the field `local_path`.\n"]
    pub fn set_local_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_CheckpointConfig {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_CheckpointConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_CheckpointConfig {
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_CheckpointConfig {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_CheckpointConfig {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_CheckpointConfig {
            local_path: core::default::Default::default(),
            s3_uri: self.s3_uri,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_CheckpointConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_CheckpointConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_CheckpointConfigRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_CheckpointConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_CheckpointConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `local_path` after provisioning.\n"]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRangesDynamic {
    auto_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_Config_ParameterRanges_AutoParameters>>,
    categorical_parameter_ranges: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerHyperParameterTuningJob_Config_ParameterRanges_CategoricalParameterRanges>,
    >,
    continuous_parameter_ranges: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerHyperParameterTuningJob_Config_ParameterRanges_ContinuousParameterRanges>,
    >,
    integer_parameter_ranges: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_Config_ParameterRanges_IntegerParameterRanges>>,
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRanges {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_parameters: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_Config_ParameterRanges_AutoParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    categorical_parameter_ranges: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_Config_ParameterRanges_CategoricalParameterRanges>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    continuous_parameter_ranges: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_Config_ParameterRanges_ContinuousParameterRanges>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integer_parameter_ranges: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_Config_ParameterRanges_IntegerParameterRanges>>,
    dynamic: SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRangesDynamic,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRanges {
    #[doc = "Set the field `auto_parameters`.\n"]
    pub fn set_auto_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerHyperParameterTuningJob_Config_ParameterRanges_AutoParameters>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auto_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auto_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `categorical_parameter_ranges`.\n"]
    pub fn set_categorical_parameter_ranges(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_Config_ParameterRanges_CategoricalParameterRanges,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.categorical_parameter_ranges = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.categorical_parameter_ranges = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `continuous_parameter_ranges`.\n"]
    pub fn set_continuous_parameter_ranges(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_Config_ParameterRanges_ContinuousParameterRanges,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.continuous_parameter_ranges = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.continuous_parameter_ranges = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `integer_parameter_ranges`.\n"]
    pub fn set_integer_parameter_ranges(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_Config_ParameterRanges_IntegerParameterRanges,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.integer_parameter_ranges = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.integer_parameter_ranges = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRanges {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRanges>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRanges {}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRanges {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRanges {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRanges {
            auto_parameters: core::default::Default::default(),
            categorical_parameter_ranges: core::default::Default::default(),
            continuous_parameter_ranges: core::default::Default::default(),
            integer_parameter_ranges: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRangesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRangesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRangesRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRangesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRangesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_parameters` after provisioning.\n"]
    pub fn auto_parameters(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_Config_ParameterRanges_AutoParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `categorical_parameter_ranges` after provisioning.\n"]
    pub fn categorical_parameter_ranges(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_Config_ParameterRanges_CategoricalParameterRangesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.categorical_parameter_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `continuous_parameter_ranges` after provisioning.\n"]
    pub fn continuous_parameter_ranges(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_Config_ParameterRanges_ContinuousParameterRangesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.continuous_parameter_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `integer_parameter_ranges` after provisioning.\n"]
    pub fn integer_parameter_ranges(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_Config_ParameterRanges_IntegerParameterRangesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integer_parameter_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig_InstanceConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size_in_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig_InstanceConfigs {
    #[doc = "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size_in_gb`.\n"]
    pub fn set_volume_size_in_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_size_in_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig_InstanceConfigs {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig_InstanceConfigs,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig_InstanceConfigs {}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig_InstanceConfigs {
    pub fn build(
        self,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig_InstanceConfigs {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig_InstanceConfigs {
            instance_count: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            volume_size_in_gb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig_InstanceConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig_InstanceConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig_InstanceConfigsRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig_InstanceConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig_InstanceConfigsRef {
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

    #[doc = "Get a reference to the value of field `volume_size_in_gb` after provisioning.\n"]
    pub fn volume_size_in_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_size_in_gb", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfigDynamic {
    instance_configs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig_InstanceConfigs,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocation_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size_in_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_configs: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig_InstanceConfigs,
        >,
    >,
    dynamic: SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfigDynamic,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig {
    #[doc = "Set the field `allocation_strategy`.\n"]
    pub fn set_allocation_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allocation_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_kms_key_id`.\n"]
    pub fn set_volume_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size_in_gb`.\n"]
    pub fn set_volume_size_in_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_size_in_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_configs`.\n"]
    pub fn set_instance_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig_InstanceConfigs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig {
    type O =
        terrars
        ::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig {}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig {
            allocation_strategy: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            volume_kms_key_id: core::default::Default::default(),
            volume_size_in_gb: core::default::Default::default(),
            instance_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfigRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocation_strategy` after provisioning.\n"]
    pub fn allocation_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocation_strategy", self.base))
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

    #[doc = "Get a reference to the value of field `instance_configs` after provisioning.\n"]
    pub fn instance_configs(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig_InstanceConfigsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource {
    directory_path: terrars::PrimField<std::string::String>,
    file_system_access_mode: terrars::PrimField<std::string::String>,
    file_system_id: terrars::PrimField<std::string::String>,
    file_system_type: terrars::PrimField<std::string::String>,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource { }

impl terrars
::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource {
    #[doc = ""]
    pub directory_path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub file_system_access_mode: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub file_system_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub file_system_type: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource {
    pub fn build(
        self,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource {
            directory_path: self.directory_path,
            file_system_access_mode: self.file_system_access_mode,
            file_system_id: self.file_system_id,
            file_system_type: self.file_system_type,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSourceRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `directory_path` after provisioning.\n"]
    pub fn directory_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_path", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_access_mode` after provisioning.\n"]
    pub fn file_system_access_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_access_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_type` after provisioning.\n"]
    pub fn file_system_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_type", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_HubAccessConfig {
    hub_content_arn: terrars::PrimField<std::string::String>,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_HubAccessConfig {

}

impl terrars
::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_HubAccessConfig {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_HubAccessConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_HubAccessConfig {
    #[doc = ""]
    pub hub_content_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_HubAccessConfig {
    pub fn build(
        self,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_HubAccessConfig {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_HubAccessConfig {
            hub_content_arn: self.hub_content_arn,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_HubAccessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_HubAccessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_HubAccessConfigRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_HubAccessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_HubAccessConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hub_content_arn` after provisioning.\n"]
    pub fn hub_content_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hub_content_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig {
    accept_eula: terrars::PrimField<bool>,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig {

}

impl terrars
::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig {
    #[doc = ""]
    pub accept_eula: terrars::PrimField<bool>,
}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig {
    pub fn build(
        self,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig {
            accept_eula: self.accept_eula,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_ModelAccessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_ModelAccessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_ModelAccessConfigRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_ModelAccessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_ModelAccessConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accept_eula` after provisioning.\n"]
    pub fn accept_eula(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accept_eula", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSourceDynamic {
    hub_access_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_HubAccessConfig,
        >,
    >,
    model_access_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_group_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_data_distribution_type: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_data_type: terrars::PrimField<std::string::String>,
    s3_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hub_access_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_HubAccessConfig,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_access_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig,
        >,
    >,
    dynamic: SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSourceDynamic,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource {
    #[doc = "Set the field `attribute_names`.\n"]
    pub fn set_attribute_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.attribute_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_group_names`.\n"]
    pub fn set_instance_group_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.instance_group_names = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `hub_access_config`.\n"]
    pub fn set_hub_access_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_HubAccessConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hub_access_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hub_access_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `model_access_config`.\n"]
    pub fn set_model_access_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model_access_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model_access_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource {
    #[doc = ""]
    pub s3_data_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource {
    pub fn build(
        self,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource {
            attribute_names: core::default::Default::default(),
            instance_group_names: core::default::Default::default(),
            s3_data_distribution_type: core::default::Default::default(),
            s3_data_type: self.s3_data_type,
            s3_uri: self.s3_uri,
            hub_access_config: core::default::Default::default(),
            model_access_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSourceRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute_names` after provisioning.\n"]
    pub fn attribute_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.attribute_names", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_group_names` after provisioning.\n"]
    pub fn instance_group_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.instance_group_names", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_data_distribution_type` after provisioning.\n"]
    pub fn s3_data_distribution_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_data_distribution_type", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_data_type` after provisioning.\n"]
    pub fn s3_data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `hub_access_config` after provisioning.\n"]
    pub fn hub_access_config(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_HubAccessConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hub_access_config", self.base))
    }

    #[doc = "Get a reference to the value of field `model_access_config` after provisioning.\n"]
    pub fn model_access_config(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource_ModelAccessConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_access_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSourceDynamic {
    file_system_data_source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource,
        >,
    >,
    s3_data_source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_data_source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_data_source: std
    ::option
    ::Option<
        std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource>,
    >,
    dynamic: SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSourceDynamic,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource {
    #[doc = "Set the field `file_system_data_source`.\n"]
    pub fn set_file_system_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file_system_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file_system_data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_data_source`.\n"]
    pub fn set_s3_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_data_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource {
    type O =
        terrars::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource {}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource {
            file_system_data_source: core::default::Default::default(),
            s3_data_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSourceRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_system_data_source` after provisioning.\n"]
    pub fn file_system_data_source(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_system_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_data_source` after provisioning.\n"]
    pub fn s3_data_source(
        &self,
    ) -> terrars
    ::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_data_source", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_ShuffleConfig {
    seed: terrars::PrimField<f64>,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_ShuffleConfig { }

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_ShuffleConfig {
    type O =
        terrars
        ::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_ShuffleConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_ShuffleConfig {
    #[doc = ""]
    pub seed: terrars::PrimField<f64>,
}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_ShuffleConfig {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_ShuffleConfig {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_ShuffleConfig { seed: self.seed }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_ShuffleConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_ShuffleConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_ShuffleConfigRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_ShuffleConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_ShuffleConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `seed` after provisioning.\n"]
    pub fn seed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seed", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfigDynamic {
    data_source: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource>,
    >,
    shuffle_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_ShuffleConfig>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig {
    channel_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_wrapper_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shuffle_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_ShuffleConfig>>,
    dynamic: SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfigDynamic,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig {
    #[doc = "Set the field `compression_type`.\n"]
    pub fn set_compression_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compression_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_type`.\n"]
    pub fn set_content_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_mode`.\n"]
    pub fn set_input_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_wrapper_type`.\n"]
    pub fn set_record_wrapper_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.record_wrapper_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source`.\n"]
    pub fn set_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `shuffle_config`.\n"]
    pub fn set_shuffle_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_ShuffleConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.shuffle_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.shuffle_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig {
    #[doc = ""]
    pub channel_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig {
            channel_name: self.channel_name,
            compression_type: core::default::Default::default(),
            content_type: core::default::Default::default(),
            input_mode: core::default::Default::default(),
            record_wrapper_type: core::default::Default::default(),
            data_source: core::default::Default::default(),
            shuffle_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfigRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `channel_name` after provisioning.\n"]
    pub fn channel_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel_name", self.base))
    }

    #[doc = "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_type", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `input_mode` after provisioning.\n"]
    pub fn input_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `record_wrapper_type` after provisioning.\n"]
    pub fn record_wrapper_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_wrapper_type", self.base))
    }

    #[doc = "Get a reference to the value of field `data_source` after provisioning.\n"]
    pub fn data_source(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_DataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `shuffle_config` after provisioning.\n"]
    pub fn shuffle_config(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig_ShuffleConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shuffle_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_OutputDataConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_output_path: terrars::PrimField<std::string::String>,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_OutputDataConfig {
    #[doc = "Set the field `compression_type`.\n"]
    pub fn set_compression_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compression_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_OutputDataConfig {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_OutputDataConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_OutputDataConfig {
    #[doc = ""]
    pub s3_output_path: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_OutputDataConfig {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_OutputDataConfig {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_OutputDataConfig {
            compression_type: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            s3_output_path: self.s3_output_path,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_OutputDataConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_OutputDataConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_OutputDataConfigRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_OutputDataConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_OutputDataConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_type", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_path` after provisioning.\n"]
    pub fn s3_output_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_output_path", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstanceGroups {
    instance_count: terrars::PrimField<f64>,
    instance_group_name: terrars::PrimField<std::string::String>,
    instance_type: terrars::PrimField<std::string::String>,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstanceGroups { }

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstanceGroups {
    type O =
        terrars
        ::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstanceGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstanceGroups {
    #[doc = ""]
    pub instance_count: terrars::PrimField<f64>,
    #[doc = ""]
    pub instance_group_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub instance_type: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstanceGroups {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstanceGroups {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstanceGroups {
            instance_count: self.instance_count,
            instance_group_name: self.instance_group_name,
            instance_type: self.instance_type,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstanceGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstanceGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstanceGroupsRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstanceGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstanceGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_group_name` after provisioning.\n"]
    pub fn instance_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications {
    instance_count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ultra_server_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications {
    #[doc = "Set the field `ultra_server_id`.\n"]
    pub fn set_ultra_server_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ultra_server_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications {
    #[doc = ""]
    pub instance_count: terrars::PrimField<f64>,
}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications {
    pub fn build(
        self,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications {
            instance_count: self.instance_count,
            ultra_server_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecificationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecificationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecificationsRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecificationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecificationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `ultra_server_id` after provisioning.\n"]
    pub fn ultra_server_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ultra_server_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfigDynamic {
    placement_specifications: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_multiple_jobs: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_specifications: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications,
        >,
    >,
    dynamic: SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfigDynamic,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig {
    #[doc = "Set the field `enable_multiple_jobs`.\n"]
    pub fn set_enable_multiple_jobs(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_multiple_jobs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `placement_specifications`.\n"]
    pub fn set_placement_specifications(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.placement_specifications = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.placement_specifications = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig {}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig {
    pub fn build(
        self,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig {
            enable_multiple_jobs: core::default::Default::default(),
            placement_specifications: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfigRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_multiple_jobs` after provisioning.\n"]
    pub fn enable_multiple_jobs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_multiple_jobs", self.base))
    }

    #[doc = "Get a reference to the value of field `placement_specifications` after provisioning.\n"]
    pub fn placement_specifications(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecificationsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement_specifications", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfigDynamic {
    instance_groups: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstanceGroups>,
    >,
    instance_placement_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keep_alive_period_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    training_plan_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size_in_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_groups: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstanceGroups>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_placement_config: std
    ::option
    ::Option<
        std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig>,
    >,
    dynamic: SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfigDynamic,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig {
    #[doc = "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `keep_alive_period_in_seconds`.\n"]
    pub fn set_keep_alive_period_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.keep_alive_period_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `training_plan_arn`.\n"]
    pub fn set_training_plan_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.training_plan_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_kms_key_id`.\n"]
    pub fn set_volume_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size_in_gb`.\n"]
    pub fn set_volume_size_in_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_size_in_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_groups`.\n"]
    pub fn set_instance_groups(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstanceGroups,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_groups = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_groups = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_placement_config`.\n"]
    pub fn set_instance_placement_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_placement_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_placement_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig {}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig {
            instance_count: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            keep_alive_period_in_seconds: core::default::Default::default(),
            training_plan_arn: core::default::Default::default(),
            volume_kms_key_id: core::default::Default::default(),
            volume_size_in_gb: core::default::Default::default(),
            instance_groups: core::default::Default::default(),
            instance_placement_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfigRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfigRef {
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

    #[doc = "Get a reference to the value of field `keep_alive_period_in_seconds` after provisioning.\n"]
    pub fn keep_alive_period_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keep_alive_period_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `training_plan_arn` after provisioning.\n"]
    pub fn training_plan_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.training_plan_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_kms_key_id` after provisioning.\n"]
    pub fn volume_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size_in_gb` after provisioning.\n"]
    pub fn volume_size_in_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_size_in_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_groups` after provisioning.\n"]
    pub fn instance_groups(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstanceGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_placement_config` after provisioning.\n"]
    pub fn instance_placement_config(
        &self,
    ) -> terrars
    ::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig_InstancePlacementConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_placement_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_StoppingCondition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pending_time_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_runtime_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_wait_time_in_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_StoppingCondition {
    #[doc = "Set the field `max_pending_time_in_seconds`.\n"]
    pub fn set_max_pending_time_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_pending_time_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_runtime_in_seconds`.\n"]
    pub fn set_max_runtime_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_runtime_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_wait_time_in_seconds`.\n"]
    pub fn set_max_wait_time_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_wait_time_in_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_StoppingCondition {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_StoppingCondition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_StoppingCondition {}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_StoppingCondition {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_StoppingCondition {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_StoppingCondition {
            max_pending_time_in_seconds: core::default::Default::default(),
            max_runtime_in_seconds: core::default::Default::default(),
            max_wait_time_in_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_StoppingConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_StoppingConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_StoppingConditionRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_StoppingConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_StoppingConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_pending_time_in_seconds` after provisioning.\n"]
    pub fn max_pending_time_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_pending_time_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `max_runtime_in_seconds` after provisioning.\n"]
    pub fn max_runtime_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_runtime_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `max_wait_time_in_seconds` after provisioning.\n"]
    pub fn max_wait_time_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_wait_time_in_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_TuningObjective {
    metric_name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_TuningObjective { }

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_TuningObjective {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_TuningObjective>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_TuningObjective {
    #[doc = ""]
    pub metric_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_TuningObjective {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_TuningObjective {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_TuningObjective {
            metric_name: self.metric_name,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_TuningObjectiveRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_TuningObjectiveRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_TuningObjectiveRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_TuningObjectiveRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_TuningObjectiveRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_VpcConfig {
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_VpcConfig { }

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition_VpcConfig {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_VpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_VpcConfig {
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition_VpcConfig {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_VpcConfig {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_VpcConfig {
            security_group_ids: self.security_group_ids,
            subnets: self.subnets,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition_VpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinition_VpcConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition_VpcConfigRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition_VpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition_VpcConfigRef {
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
struct SagemakerHyperParameterTuningJob_TrainingJobDefinitionDynamic {
    algorithm_specification: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification>>,
    checkpoint_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_CheckpointConfig>>,
    hyper_parameter_ranges: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRanges>>,
    hyper_parameter_tuning_resource_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig>,
    >,
    input_data_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig>>,
    output_data_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_OutputDataConfig>>,
    resource_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig>>,
    stopping_condition: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_StoppingCondition>>,
    tuning_objective: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_TuningObjective>>,
    vpc_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_VpcConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    definition_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_inter_container_traffic_encryption: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_managed_spot_training: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_network_isolation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_strategy: std
    ::option
    ::Option<terrars::ListField<SagemakerHyperParameterTuningJob_TrainingJobDefinition_RetryStrategy>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_hyper_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm_specification: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    checkpoint_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_CheckpointConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hyper_parameter_ranges: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRanges>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hyper_parameter_tuning_resource_config: std
    ::option
    ::Option<
        std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_data_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_data_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_OutputDataConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stopping_condition: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_StoppingCondition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tuning_objective: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_TuningObjective>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_VpcConfig>>,
    dynamic: SagemakerHyperParameterTuningJob_TrainingJobDefinitionDynamic,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinition {
    #[doc = "Set the field `definition_name`.\n"]
    pub fn set_definition_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.definition_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_inter_container_traffic_encryption`.\n"]
    pub fn set_enable_inter_container_traffic_encryption(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_inter_container_traffic_encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_managed_spot_training`.\n"]
    pub fn set_enable_managed_spot_training(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_managed_spot_training = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_network_isolation`.\n"]
    pub fn set_enable_network_isolation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_network_isolation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_strategy`.\n"]
    pub fn set_retry_strategy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<SagemakerHyperParameterTuningJob_TrainingJobDefinition_RetryStrategy>>,
    ) -> Self {
        self.retry_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `static_hyper_parameters`.\n"]
    pub fn set_static_hyper_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.static_hyper_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `algorithm_specification`.\n"]
    pub fn set_algorithm_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.algorithm_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.algorithm_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `checkpoint_config`.\n"]
    pub fn set_checkpoint_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_CheckpointConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.checkpoint_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.checkpoint_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hyper_parameter_ranges`.\n"]
    pub fn set_hyper_parameter_ranges(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRanges>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hyper_parameter_ranges = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hyper_parameter_ranges = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hyper_parameter_tuning_resource_config`.\n"]
    pub fn set_hyper_parameter_tuning_resource_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hyper_parameter_tuning_resource_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hyper_parameter_tuning_resource_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_data_config`.\n"]
    pub fn set_input_data_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_data_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_data_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_data_config`.\n"]
    pub fn set_output_data_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_OutputDataConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output_data_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output_data_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_config`.\n"]
    pub fn set_resource_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_config = std::option::Option::Some(d);
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
                        ::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_StoppingCondition>,
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

    #[doc = "Set the field `tuning_objective`.\n"]
    pub fn set_tuning_objective(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_TuningObjective>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tuning_objective = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tuning_objective = std::option::Option::Some(d);
            },
        }
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
                        terrars::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_VpcConfig>,
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

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinition {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinition {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_TrainingJobDefinition {
        SagemakerHyperParameterTuningJob_TrainingJobDefinition {
            definition_name: core::default::Default::default(),
            enable_inter_container_traffic_encryption: core::default::Default::default(),
            enable_managed_spot_training: core::default::Default::default(),
            enable_network_isolation: core::default::Default::default(),
            environment: core::default::Default::default(),
            retry_strategy: core::default::Default::default(),
            role_arn: self.role_arn,
            static_hyper_parameters: core::default::Default::default(),
            algorithm_specification: core::default::Default::default(),
            checkpoint_config: core::default::Default::default(),
            hyper_parameter_ranges: core::default::Default::default(),
            hyper_parameter_tuning_resource_config: core::default::Default::default(),
            input_data_config: core::default::Default::default(),
            output_data_config: core::default::Default::default(),
            resource_config: core::default::Default::default(),
            stopping_condition: core::default::Default::default(),
            tuning_objective: core::default::Default::default(),
            vpc_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinitionRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `definition_name` after provisioning.\n"]
    pub fn definition_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.definition_name", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_inter_container_traffic_encryption` after provisioning.\n"]
    pub fn enable_inter_container_traffic_encryption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_inter_container_traffic_encryption", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enable_managed_spot_training` after provisioning.\n"]
    pub fn enable_managed_spot_training(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_managed_spot_training", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_network_isolation` after provisioning.\n"]
    pub fn enable_network_isolation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_network_isolation", self.base))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_strategy` after provisioning.\n"]
    pub fn retry_strategy(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_RetryStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `static_hyper_parameters` after provisioning.\n"]
    pub fn static_hyper_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.static_hyper_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `algorithm_specification` after provisioning.\n"]
    pub fn algorithm_specification(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.algorithm_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `checkpoint_config` after provisioning.\n"]
    pub fn checkpoint_config(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_CheckpointConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.checkpoint_config", self.base))
    }

    #[doc = "Get a reference to the value of field `hyper_parameter_ranges` after provisioning.\n"]
    pub fn hyper_parameter_ranges(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRangesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hyper_parameter_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `hyper_parameter_tuning_resource_config` after provisioning.\n"]
    pub fn hyper_parameter_tuning_resource_config(
        &self,
    ) -> terrars
    ::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hyper_parameter_tuning_resource_config", self.base))
    }

    #[doc = "Get a reference to the value of field `input_data_config` after provisioning.\n"]
    pub fn input_data_config(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_data_config", self.base))
    }

    #[doc = "Get a reference to the value of field `output_data_config` after provisioning.\n"]
    pub fn output_data_config(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_OutputDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_data_config", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_config` after provisioning.\n"]
    pub fn resource_config(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_config", self.base))
    }

    #[doc = "Get a reference to the value of field `stopping_condition` after provisioning.\n"]
    pub fn stopping_condition(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_StoppingConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stopping_condition", self.base))
    }

    #[doc = "Get a reference to the value of field `tuning_objective` after provisioning.\n"]
    pub fn tuning_objective(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_TuningObjectiveRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tuning_objective", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerHyperParameterTuningJob_TrainingJobDefinitionsDynamic {
    algorithm_specification: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification>>,
    checkpoint_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_CheckpointConfig>>,
    hyper_parameter_ranges: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRanges>>,
    hyper_parameter_tuning_resource_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig>,
    >,
    input_data_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig>>,
    output_data_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_OutputDataConfig>>,
    resource_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig>>,
    stopping_condition: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_StoppingCondition>>,
    tuning_objective: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_TuningObjective>>,
    vpc_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_TrainingJobDefinition_VpcConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinitions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    definition_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_inter_container_traffic_encryption: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_managed_spot_training: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_network_isolation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_strategy: std
    ::option
    ::Option<terrars::ListField<SagemakerHyperParameterTuningJob_TrainingJobDefinition_RetryStrategy>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_hyper_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm_specification: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    checkpoint_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_CheckpointConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hyper_parameter_ranges: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRanges>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hyper_parameter_tuning_resource_config: std
    ::option
    ::Option<
        std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_data_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_data_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_OutputDataConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stopping_condition: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_StoppingCondition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tuning_objective: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_TuningObjective>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_TrainingJobDefinition_VpcConfig>>,
    dynamic: SagemakerHyperParameterTuningJob_TrainingJobDefinitionsDynamic,
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinitions {
    #[doc = "Set the field `definition_name`.\n"]
    pub fn set_definition_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.definition_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_inter_container_traffic_encryption`.\n"]
    pub fn set_enable_inter_container_traffic_encryption(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_inter_container_traffic_encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_managed_spot_training`.\n"]
    pub fn set_enable_managed_spot_training(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_managed_spot_training = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_network_isolation`.\n"]
    pub fn set_enable_network_isolation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_network_isolation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_strategy`.\n"]
    pub fn set_retry_strategy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<SagemakerHyperParameterTuningJob_TrainingJobDefinition_RetryStrategy>>,
    ) -> Self {
        self.retry_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `static_hyper_parameters`.\n"]
    pub fn set_static_hyper_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.static_hyper_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `algorithm_specification`.\n"]
    pub fn set_algorithm_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.algorithm_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.algorithm_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `checkpoint_config`.\n"]
    pub fn set_checkpoint_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_CheckpointConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.checkpoint_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.checkpoint_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hyper_parameter_ranges`.\n"]
    pub fn set_hyper_parameter_ranges(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRanges>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hyper_parameter_ranges = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hyper_parameter_ranges = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hyper_parameter_tuning_resource_config`.\n"]
    pub fn set_hyper_parameter_tuning_resource_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hyper_parameter_tuning_resource_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hyper_parameter_tuning_resource_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_data_config`.\n"]
    pub fn set_input_data_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_data_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_data_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_data_config`.\n"]
    pub fn set_output_data_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_OutputDataConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output_data_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output_data_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_config`.\n"]
    pub fn set_resource_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_config = std::option::Option::Some(d);
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
                        ::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_StoppingCondition>,
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

    #[doc = "Set the field `tuning_objective`.\n"]
    pub fn set_tuning_objective(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_TuningObjective>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tuning_objective = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tuning_objective = std::option::Option::Some(d);
            },
        }
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
                        terrars::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinition_VpcConfig>,
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

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_TrainingJobDefinitions {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_TrainingJobDefinitions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_TrainingJobDefinitions {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerHyperParameterTuningJob_TrainingJobDefinitions {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_TrainingJobDefinitions {
        SagemakerHyperParameterTuningJob_TrainingJobDefinitions {
            definition_name: core::default::Default::default(),
            enable_inter_container_traffic_encryption: core::default::Default::default(),
            enable_managed_spot_training: core::default::Default::default(),
            enable_network_isolation: core::default::Default::default(),
            environment: core::default::Default::default(),
            retry_strategy: core::default::Default::default(),
            role_arn: self.role_arn,
            static_hyper_parameters: core::default::Default::default(),
            algorithm_specification: core::default::Default::default(),
            checkpoint_config: core::default::Default::default(),
            hyper_parameter_ranges: core::default::Default::default(),
            hyper_parameter_tuning_resource_config: core::default::Default::default(),
            input_data_config: core::default::Default::default(),
            output_data_config: core::default::Default::default(),
            resource_config: core::default::Default::default(),
            stopping_condition: core::default::Default::default(),
            tuning_objective: core::default::Default::default(),
            vpc_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_TrainingJobDefinitionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_TrainingJobDefinitionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_TrainingJobDefinitionsRef {
        SagemakerHyperParameterTuningJob_TrainingJobDefinitionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_TrainingJobDefinitionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `definition_name` after provisioning.\n"]
    pub fn definition_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.definition_name", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_inter_container_traffic_encryption` after provisioning.\n"]
    pub fn enable_inter_container_traffic_encryption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_inter_container_traffic_encryption", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enable_managed_spot_training` after provisioning.\n"]
    pub fn enable_managed_spot_training(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_managed_spot_training", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_network_isolation` after provisioning.\n"]
    pub fn enable_network_isolation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_network_isolation", self.base))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_strategy` after provisioning.\n"]
    pub fn retry_strategy(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_RetryStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `static_hyper_parameters` after provisioning.\n"]
    pub fn static_hyper_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.static_hyper_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `algorithm_specification` after provisioning.\n"]
    pub fn algorithm_specification(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_AlgorithmSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.algorithm_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `checkpoint_config` after provisioning.\n"]
    pub fn checkpoint_config(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_CheckpointConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.checkpoint_config", self.base))
    }

    #[doc = "Get a reference to the value of field `hyper_parameter_ranges` after provisioning.\n"]
    pub fn hyper_parameter_ranges(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterRangesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hyper_parameter_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `hyper_parameter_tuning_resource_config` after provisioning.\n"]
    pub fn hyper_parameter_tuning_resource_config(
        &self,
    ) -> terrars
    ::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_HyperParameterTuningResourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hyper_parameter_tuning_resource_config", self.base))
    }

    #[doc = "Get a reference to the value of field `input_data_config` after provisioning.\n"]
    pub fn input_data_config(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_InputDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_data_config", self.base))
    }

    #[doc = "Get a reference to the value of field `output_data_config` after provisioning.\n"]
    pub fn output_data_config(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_OutputDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_data_config", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_config` after provisioning.\n"]
    pub fn resource_config(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_ResourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_config", self.base))
    }

    #[doc = "Get a reference to the value of field `stopping_condition` after provisioning.\n"]
    pub fn stopping_condition(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_StoppingConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stopping_condition", self.base))
    }

    #[doc = "Get a reference to the value of field `tuning_objective` after provisioning.\n"]
    pub fn tuning_objective(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_TuningObjectiveRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tuning_objective", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<SagemakerHyperParameterTuningJob_TrainingJobDefinition_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_WarmStartConfig_ParentHyperParameterTuningJobs {
    name: terrars::PrimField<std::string::String>,
}

impl SagemakerHyperParameterTuningJob_WarmStartConfig_ParentHyperParameterTuningJobs { }

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_WarmStartConfig_ParentHyperParameterTuningJobs {
    type O =
        terrars::BlockAssignable<SagemakerHyperParameterTuningJob_WarmStartConfig_ParentHyperParameterTuningJobs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_WarmStartConfig_ParentHyperParameterTuningJobs {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerHyperParameterTuningJob_WarmStartConfig_ParentHyperParameterTuningJobs {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_WarmStartConfig_ParentHyperParameterTuningJobs {
        SagemakerHyperParameterTuningJob_WarmStartConfig_ParentHyperParameterTuningJobs { name: self.name }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_WarmStartConfig_ParentHyperParameterTuningJobsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_WarmStartConfig_ParentHyperParameterTuningJobsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_WarmStartConfig_ParentHyperParameterTuningJobsRef {
        SagemakerHyperParameterTuningJob_WarmStartConfig_ParentHyperParameterTuningJobsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_WarmStartConfig_ParentHyperParameterTuningJobsRef {
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
struct SagemakerHyperParameterTuningJob_WarmStartConfigDynamic {
    parent_hyper_parameter_tuning_jobs: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerHyperParameterTuningJob_WarmStartConfig_ParentHyperParameterTuningJobs>>,
}

#[derive(Serialize)]
pub struct SagemakerHyperParameterTuningJob_WarmStartConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warm_start_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent_hyper_parameter_tuning_jobs: std
    ::option
    ::Option<std::vec::Vec<SagemakerHyperParameterTuningJob_WarmStartConfig_ParentHyperParameterTuningJobs>>,
    dynamic: SagemakerHyperParameterTuningJob_WarmStartConfigDynamic,
}

impl SagemakerHyperParameterTuningJob_WarmStartConfig {
    #[doc = "Set the field `warm_start_type`.\n"]
    pub fn set_warm_start_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.warm_start_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parent_hyper_parameter_tuning_jobs`.\n"]
    pub fn set_parent_hyper_parameter_tuning_jobs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerHyperParameterTuningJob_WarmStartConfig_ParentHyperParameterTuningJobs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parent_hyper_parameter_tuning_jobs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parent_hyper_parameter_tuning_jobs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerHyperParameterTuningJob_WarmStartConfig {
    type O = terrars::BlockAssignable<SagemakerHyperParameterTuningJob_WarmStartConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHyperParameterTuningJob_WarmStartConfig {}

impl BuildSagemakerHyperParameterTuningJob_WarmStartConfig {
    pub fn build(self) -> SagemakerHyperParameterTuningJob_WarmStartConfig {
        SagemakerHyperParameterTuningJob_WarmStartConfig {
            warm_start_type: core::default::Default::default(),
            parent_hyper_parameter_tuning_jobs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerHyperParameterTuningJob_WarmStartConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerHyperParameterTuningJob_WarmStartConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerHyperParameterTuningJob_WarmStartConfigRef {
        SagemakerHyperParameterTuningJob_WarmStartConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHyperParameterTuningJob_WarmStartConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `warm_start_type` after provisioning.\n"]
    pub fn warm_start_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warm_start_type", self.base))
    }

    #[doc = "Get a reference to the value of field `parent_hyper_parameter_tuning_jobs` after provisioning.\n"]
    pub fn parent_hyper_parameter_tuning_jobs(
        &self,
    ) -> terrars::ListRef<SagemakerHyperParameterTuningJob_WarmStartConfig_ParentHyperParameterTuningJobsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parent_hyper_parameter_tuning_jobs", self.base))
    }
}
