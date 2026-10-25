#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerLabelingJobData_ {
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
    label_attribute_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label_category_config_s3_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    labeling_job_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stopping_conditions: std::option::Option<terrars::ListField<SagemakerLabelingJob_StoppingConditions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    human_task_config: std::option::Option<std::vec::Vec<SagemakerLabelingJob_HumanTaskConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_config: std::option::Option<std::vec::Vec<SagemakerLabelingJob_InputConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labeling_job_algorithms_config: std::option::Option<std::vec::Vec<SagemakerLabelingJob_LabelingJobAlgorithmsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_config: std::option::Option<std::vec::Vec<SagemakerLabelingJob_OutputConfig>>,
    dynamic: SagemakerLabelingJobDynamic,
}

struct SagemakerLabelingJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerLabelingJobData_>,
}

#[derive(Clone)]
pub struct SagemakerLabelingJob(std::rc::Rc<SagemakerLabelingJob_>);

impl SagemakerLabelingJob {
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

    #[doc = "Set the field `label_category_config_s3_uri`.\n"]
    pub fn set_label_category_config_s3_uri(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().label_category_config_s3_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stopping_conditions`.\n"]
    pub fn set_stopping_conditions(
        self,
        v: impl std::convert::Into<terrars::ListField<SagemakerLabelingJob_StoppingConditions>>,
    ) -> Self {
        self.0.data.borrow_mut().stopping_conditions = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `human_task_config`.\n"]
    pub fn set_human_task_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerLabelingJob_HumanTaskConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().human_task_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.human_task_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_config`.\n"]
    pub fn set_input_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerLabelingJob_InputConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `labeling_job_algorithms_config`.\n"]
    pub fn set_labeling_job_algorithms_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerLabelingJob_LabelingJobAlgorithmsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().labeling_job_algorithms_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.labeling_job_algorithms_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_config`.\n"]
    pub fn set_output_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerLabelingJob_OutputConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().output_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.output_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\n"]
    pub fn failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_reference_code` after provisioning.\n"]
    pub fn job_reference_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_reference_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `label_attribute_name` after provisioning.\n"]
    pub fn label_attribute_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_attribute_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `label_category_config_s3_uri` after provisioning.\n"]
    pub fn label_category_config_s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_category_config_s3_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `label_counters` after provisioning.\n"]
    pub fn label_counters(&self) -> terrars::ListRef<SagemakerLabelingJob_LabelCountersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.label_counters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `labeling_job_arn` after provisioning.\n"]
    pub fn labeling_job_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.labeling_job_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `labeling_job_name` after provisioning.\n"]
    pub fn labeling_job_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.labeling_job_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `labeling_job_status` after provisioning.\n"]
    pub fn labeling_job_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.labeling_job_status", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `stopping_conditions` after provisioning.\n"]
    pub fn stopping_conditions(&self) -> terrars::ListRef<SagemakerLabelingJob_StoppingConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stopping_conditions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `human_task_config` after provisioning.\n"]
    pub fn human_task_config(&self) -> terrars::ListRef<SagemakerLabelingJob_HumanTaskConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.human_task_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_config` after provisioning.\n"]
    pub fn input_config(&self) -> terrars::ListRef<SagemakerLabelingJob_InputConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `labeling_job_algorithms_config` after provisioning.\n"]
    pub fn labeling_job_algorithms_config(
        &self,
    ) -> terrars::ListRef<SagemakerLabelingJob_LabelingJobAlgorithmsConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.labeling_job_algorithms_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `output_config` after provisioning.\n"]
    pub fn output_config(&self) -> terrars::ListRef<SagemakerLabelingJob_OutputConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_config", self.extract_ref()))
    }
}

impl terrars::Referable for SagemakerLabelingJob {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerLabelingJob { }

impl terrars::ToListMappable for SagemakerLabelingJob {
    type O = terrars::ListRef<SagemakerLabelingJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerLabelingJob_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_labeling_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerLabelingJob {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub label_attribute_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub labeling_job_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerLabelingJob {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerLabelingJob {
        let out = SagemakerLabelingJob(std::rc::Rc::new(SagemakerLabelingJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerLabelingJobData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                label_attribute_name: self.label_attribute_name,
                label_category_config_s3_uri: core::default::Default::default(),
                labeling_job_name: self.labeling_job_name,
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                stopping_conditions: core::default::Default::default(),
                tags: core::default::Default::default(),
                human_task_config: core::default::Default::default(),
                input_config: core::default::Default::default(),
                labeling_job_algorithms_config: core::default::Default::default(),
                output_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerLabelingJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerLabelingJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerLabelingJobRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\n"]
    pub fn failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_reference_code` after provisioning.\n"]
    pub fn job_reference_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_reference_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `label_attribute_name` after provisioning.\n"]
    pub fn label_attribute_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_attribute_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `label_category_config_s3_uri` after provisioning.\n"]
    pub fn label_category_config_s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_category_config_s3_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `label_counters` after provisioning.\n"]
    pub fn label_counters(&self) -> terrars::ListRef<SagemakerLabelingJob_LabelCountersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.label_counters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `labeling_job_arn` after provisioning.\n"]
    pub fn labeling_job_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.labeling_job_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `labeling_job_name` after provisioning.\n"]
    pub fn labeling_job_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.labeling_job_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `labeling_job_status` after provisioning.\n"]
    pub fn labeling_job_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.labeling_job_status", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `stopping_conditions` after provisioning.\n"]
    pub fn stopping_conditions(&self) -> terrars::ListRef<SagemakerLabelingJob_StoppingConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stopping_conditions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `human_task_config` after provisioning.\n"]
    pub fn human_task_config(&self) -> terrars::ListRef<SagemakerLabelingJob_HumanTaskConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.human_task_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_config` after provisioning.\n"]
    pub fn input_config(&self) -> terrars::ListRef<SagemakerLabelingJob_InputConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `labeling_job_algorithms_config` after provisioning.\n"]
    pub fn labeling_job_algorithms_config(
        &self,
    ) -> terrars::ListRef<SagemakerLabelingJob_LabelingJobAlgorithmsConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.labeling_job_algorithms_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `output_config` after provisioning.\n"]
    pub fn output_config(&self) -> terrars::ListRef<SagemakerLabelingJob_OutputConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SagemakerLabelingJobDynamic {
    human_task_config: std::option::Option<terrars::DynamicBlock<SagemakerLabelingJob_HumanTaskConfig>>,
    input_config: std::option::Option<terrars::DynamicBlock<SagemakerLabelingJob_InputConfig>>,
    labeling_job_algorithms_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerLabelingJob_LabelingJobAlgorithmsConfig>>,
    output_config: std::option::Option<terrars::DynamicBlock<SagemakerLabelingJob_OutputConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerLabelingJob_LabelCounters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failed_non_retryable_error: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    human_labeled: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_labeled: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_labeled: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unlabeled: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerLabelingJob_LabelCounters {
    #[doc = "Set the field `failed_non_retryable_error`.\n"]
    pub fn set_failed_non_retryable_error(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failed_non_retryable_error = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `human_labeled`.\n"]
    pub fn set_human_labeled(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.human_labeled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_labeled`.\n"]
    pub fn set_machine_labeled(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.machine_labeled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_labeled`.\n"]
    pub fn set_total_labeled(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_labeled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unlabeled`.\n"]
    pub fn set_unlabeled(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.unlabeled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerLabelingJob_LabelCounters {
    type O = terrars::BlockAssignable<SagemakerLabelingJob_LabelCounters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerLabelingJob_LabelCounters {}

impl BuildSagemakerLabelingJob_LabelCounters {
    pub fn build(self) -> SagemakerLabelingJob_LabelCounters {
        SagemakerLabelingJob_LabelCounters {
            failed_non_retryable_error: core::default::Default::default(),
            human_labeled: core::default::Default::default(),
            machine_labeled: core::default::Default::default(),
            total_labeled: core::default::Default::default(),
            unlabeled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerLabelingJob_LabelCountersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerLabelingJob_LabelCountersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerLabelingJob_LabelCountersRef {
        SagemakerLabelingJob_LabelCountersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerLabelingJob_LabelCountersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failed_non_retryable_error` after provisioning.\n"]
    pub fn failed_non_retryable_error(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failed_non_retryable_error", self.base))
    }

    #[doc = "Get a reference to the value of field `human_labeled` after provisioning.\n"]
    pub fn human_labeled(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.human_labeled", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_labeled` after provisioning.\n"]
    pub fn machine_labeled(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_labeled", self.base))
    }

    #[doc = "Get a reference to the value of field `total_labeled` after provisioning.\n"]
    pub fn total_labeled(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_labeled", self.base))
    }

    #[doc = "Get a reference to the value of field `unlabeled` after provisioning.\n"]
    pub fn unlabeled(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unlabeled", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerLabelingJob_StoppingConditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_human_labeled_object_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_percentage_of_input_dataset_labeled: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerLabelingJob_StoppingConditions {
    #[doc = "Set the field `max_human_labeled_object_count`.\n"]
    pub fn set_max_human_labeled_object_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_human_labeled_object_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_percentage_of_input_dataset_labeled`.\n"]
    pub fn set_max_percentage_of_input_dataset_labeled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.max_percentage_of_input_dataset_labeled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerLabelingJob_StoppingConditions {
    type O = terrars::BlockAssignable<SagemakerLabelingJob_StoppingConditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerLabelingJob_StoppingConditions {}

impl BuildSagemakerLabelingJob_StoppingConditions {
    pub fn build(self) -> SagemakerLabelingJob_StoppingConditions {
        SagemakerLabelingJob_StoppingConditions {
            max_human_labeled_object_count: core::default::Default::default(),
            max_percentage_of_input_dataset_labeled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerLabelingJob_StoppingConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerLabelingJob_StoppingConditionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerLabelingJob_StoppingConditionsRef {
        SagemakerLabelingJob_StoppingConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerLabelingJob_StoppingConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_human_labeled_object_count` after provisioning.\n"]
    pub fn max_human_labeled_object_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_human_labeled_object_count", self.base))
    }

    #[doc = "Get a reference to the value of field `max_percentage_of_input_dataset_labeled` after provisioning.\n"]
    pub fn max_percentage_of_input_dataset_labeled(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_percentage_of_input_dataset_labeled", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerLabelingJob_HumanTaskConfig_AnnotationConsolidationConfig {
    annotation_consolidation_lambda_arn: terrars::PrimField<std::string::String>,
}

impl SagemakerLabelingJob_HumanTaskConfig_AnnotationConsolidationConfig { }

impl terrars::ToListMappable for SagemakerLabelingJob_HumanTaskConfig_AnnotationConsolidationConfig {
    type O = terrars::BlockAssignable<SagemakerLabelingJob_HumanTaskConfig_AnnotationConsolidationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerLabelingJob_HumanTaskConfig_AnnotationConsolidationConfig {
    #[doc = ""]
    pub annotation_consolidation_lambda_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerLabelingJob_HumanTaskConfig_AnnotationConsolidationConfig {
    pub fn build(self) -> SagemakerLabelingJob_HumanTaskConfig_AnnotationConsolidationConfig {
        SagemakerLabelingJob_HumanTaskConfig_AnnotationConsolidationConfig {
            annotation_consolidation_lambda_arn: self.annotation_consolidation_lambda_arn,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerLabelingJob_HumanTaskConfig_AnnotationConsolidationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerLabelingJob_HumanTaskConfig_AnnotationConsolidationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerLabelingJob_HumanTaskConfig_AnnotationConsolidationConfigRef {
        SagemakerLabelingJob_HumanTaskConfig_AnnotationConsolidationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerLabelingJob_HumanTaskConfig_AnnotationConsolidationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `annotation_consolidation_lambda_arn` after provisioning.\n"]
    pub fn annotation_consolidation_lambda_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.annotation_consolidation_lambda_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice_AmountInUsd {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cents: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dollars: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenth_fractions_of_a_cent: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice_AmountInUsd {
    #[doc = "Set the field `cents`.\n"]
    pub fn set_cents(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cents = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dollars`.\n"]
    pub fn set_dollars(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.dollars = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenth_fractions_of_a_cent`.\n"]
    pub fn set_tenth_fractions_of_a_cent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tenth_fractions_of_a_cent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice_AmountInUsd {
    type O = terrars::BlockAssignable<SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice_AmountInUsd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice_AmountInUsd {}

impl BuildSagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice_AmountInUsd {
    pub fn build(self) -> SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice_AmountInUsd {
        SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice_AmountInUsd {
            cents: core::default::Default::default(),
            dollars: core::default::Default::default(),
            tenth_fractions_of_a_cent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice_AmountInUsdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice_AmountInUsdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice_AmountInUsdRef {
        SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice_AmountInUsdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice_AmountInUsdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cents` after provisioning.\n"]
    pub fn cents(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cents", self.base))
    }

    #[doc = "Get a reference to the value of field `dollars` after provisioning.\n"]
    pub fn dollars(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dollars", self.base))
    }

    #[doc = "Get a reference to the value of field `tenth_fractions_of_a_cent` after provisioning.\n"]
    pub fn tenth_fractions_of_a_cent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenth_fractions_of_a_cent", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPriceDynamic {
    amount_in_usd: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice_AmountInUsd>>,
}

#[derive(Serialize)]
pub struct SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amount_in_usd: std
    ::option
    ::Option<std::vec::Vec<SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice_AmountInUsd>>,
    dynamic: SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPriceDynamic,
}

impl SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice {
    #[doc = "Set the field `amount_in_usd`.\n"]
    pub fn set_amount_in_usd(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice_AmountInUsd>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amount_in_usd = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amount_in_usd = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice {
    type O = terrars::BlockAssignable<SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice {}

impl BuildSagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice {
    pub fn build(self) -> SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice {
        SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice {
            amount_in_usd: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPriceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPriceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPriceRef {
        SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPriceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPriceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amount_in_usd` after provisioning.\n"]
    pub fn amount_in_usd(
        &self,
    ) -> terrars::ListRef<SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice_AmountInUsdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amount_in_usd", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerLabelingJob_HumanTaskConfig_UiConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    human_task_ui_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ui_template_s3_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerLabelingJob_HumanTaskConfig_UiConfig {
    #[doc = "Set the field `human_task_ui_arn`.\n"]
    pub fn set_human_task_ui_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.human_task_ui_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ui_template_s3_uri`.\n"]
    pub fn set_ui_template_s3_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ui_template_s3_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerLabelingJob_HumanTaskConfig_UiConfig {
    type O = terrars::BlockAssignable<SagemakerLabelingJob_HumanTaskConfig_UiConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerLabelingJob_HumanTaskConfig_UiConfig {}

impl BuildSagemakerLabelingJob_HumanTaskConfig_UiConfig {
    pub fn build(self) -> SagemakerLabelingJob_HumanTaskConfig_UiConfig {
        SagemakerLabelingJob_HumanTaskConfig_UiConfig {
            human_task_ui_arn: core::default::Default::default(),
            ui_template_s3_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerLabelingJob_HumanTaskConfig_UiConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerLabelingJob_HumanTaskConfig_UiConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerLabelingJob_HumanTaskConfig_UiConfigRef {
        SagemakerLabelingJob_HumanTaskConfig_UiConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerLabelingJob_HumanTaskConfig_UiConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `human_task_ui_arn` after provisioning.\n"]
    pub fn human_task_ui_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.human_task_ui_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `ui_template_s3_uri` after provisioning.\n"]
    pub fn ui_template_s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ui_template_s3_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerLabelingJob_HumanTaskConfigDynamic {
    annotation_consolidation_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerLabelingJob_HumanTaskConfig_AnnotationConsolidationConfig>>,
    public_workforce_task_price: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice>>,
    ui_config: std::option::Option<terrars::DynamicBlock<SagemakerLabelingJob_HumanTaskConfig_UiConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerLabelingJob_HumanTaskConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_task_count: std::option::Option<terrars::PrimField<f64>>,
    number_of_human_workers_per_data_object: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pre_human_task_lambda_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_availability_lifetime_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    task_description: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_keywords: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    task_time_limit_in_seconds: terrars::PrimField<f64>,
    task_title: terrars::PrimField<std::string::String>,
    workteam_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    annotation_consolidation_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerLabelingJob_HumanTaskConfig_AnnotationConsolidationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_workforce_task_price: std
    ::option
    ::Option<std::vec::Vec<SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ui_config: std::option::Option<std::vec::Vec<SagemakerLabelingJob_HumanTaskConfig_UiConfig>>,
    dynamic: SagemakerLabelingJob_HumanTaskConfigDynamic,
}

impl SagemakerLabelingJob_HumanTaskConfig {
    #[doc = "Set the field `max_concurrent_task_count`.\n"]
    pub fn set_max_concurrent_task_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrent_task_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pre_human_task_lambda_arn`.\n"]
    pub fn set_pre_human_task_lambda_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pre_human_task_lambda_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `task_availability_lifetime_in_seconds`.\n"]
    pub fn set_task_availability_lifetime_in_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.task_availability_lifetime_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `task_keywords`.\n"]
    pub fn set_task_keywords(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.task_keywords = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `annotation_consolidation_config`.\n"]
    pub fn set_annotation_consolidation_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerLabelingJob_HumanTaskConfig_AnnotationConsolidationConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.annotation_consolidation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.annotation_consolidation_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `public_workforce_task_price`.\n"]
    pub fn set_public_workforce_task_price(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPrice>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.public_workforce_task_price = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.public_workforce_task_price = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ui_config`.\n"]
    pub fn set_ui_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerLabelingJob_HumanTaskConfig_UiConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ui_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ui_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerLabelingJob_HumanTaskConfig {
    type O = terrars::BlockAssignable<SagemakerLabelingJob_HumanTaskConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerLabelingJob_HumanTaskConfig {
    #[doc = ""]
    pub number_of_human_workers_per_data_object: terrars::PrimField<f64>,
    #[doc = ""]
    pub task_description: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub task_time_limit_in_seconds: terrars::PrimField<f64>,
    #[doc = ""]
    pub task_title: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub workteam_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerLabelingJob_HumanTaskConfig {
    pub fn build(self) -> SagemakerLabelingJob_HumanTaskConfig {
        SagemakerLabelingJob_HumanTaskConfig {
            max_concurrent_task_count: core::default::Default::default(),
            number_of_human_workers_per_data_object: self.number_of_human_workers_per_data_object,
            pre_human_task_lambda_arn: core::default::Default::default(),
            task_availability_lifetime_in_seconds: core::default::Default::default(),
            task_description: self.task_description,
            task_keywords: core::default::Default::default(),
            task_time_limit_in_seconds: self.task_time_limit_in_seconds,
            task_title: self.task_title,
            workteam_arn: self.workteam_arn,
            annotation_consolidation_config: core::default::Default::default(),
            public_workforce_task_price: core::default::Default::default(),
            ui_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerLabelingJob_HumanTaskConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerLabelingJob_HumanTaskConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerLabelingJob_HumanTaskConfigRef {
        SagemakerLabelingJob_HumanTaskConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerLabelingJob_HumanTaskConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_concurrent_task_count` after provisioning.\n"]
    pub fn max_concurrent_task_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_task_count", self.base))
    }

    #[doc = "Get a reference to the value of field `number_of_human_workers_per_data_object` after provisioning.\n"]
    pub fn number_of_human_workers_per_data_object(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.number_of_human_workers_per_data_object", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `pre_human_task_lambda_arn` after provisioning.\n"]
    pub fn pre_human_task_lambda_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pre_human_task_lambda_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `task_availability_lifetime_in_seconds` after provisioning.\n"]
    pub fn task_availability_lifetime_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_availability_lifetime_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `task_description` after provisioning.\n"]
    pub fn task_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_description", self.base))
    }

    #[doc = "Get a reference to the value of field `task_keywords` after provisioning.\n"]
    pub fn task_keywords(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.task_keywords", self.base))
    }

    #[doc = "Get a reference to the value of field `task_time_limit_in_seconds` after provisioning.\n"]
    pub fn task_time_limit_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_time_limit_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `task_title` after provisioning.\n"]
    pub fn task_title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_title", self.base))
    }

    #[doc = "Get a reference to the value of field `workteam_arn` after provisioning.\n"]
    pub fn workteam_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workteam_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `annotation_consolidation_config` after provisioning.\n"]
    pub fn annotation_consolidation_config(
        &self,
    ) -> terrars::ListRef<SagemakerLabelingJob_HumanTaskConfig_AnnotationConsolidationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.annotation_consolidation_config", self.base))
    }

    #[doc = "Get a reference to the value of field `public_workforce_task_price` after provisioning.\n"]
    pub fn public_workforce_task_price(
        &self,
    ) -> terrars::ListRef<SagemakerLabelingJob_HumanTaskConfig_PublicWorkforceTaskPriceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_workforce_task_price", self.base))
    }

    #[doc = "Get a reference to the value of field `ui_config` after provisioning.\n"]
    pub fn ui_config(&self) -> terrars::ListRef<SagemakerLabelingJob_HumanTaskConfig_UiConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ui_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerLabelingJob_InputConfig_DataAttributes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_classifiers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl SagemakerLabelingJob_InputConfig_DataAttributes {
    #[doc = "Set the field `content_classifiers`.\n"]
    pub fn set_content_classifiers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.content_classifiers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerLabelingJob_InputConfig_DataAttributes {
    type O = terrars::BlockAssignable<SagemakerLabelingJob_InputConfig_DataAttributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerLabelingJob_InputConfig_DataAttributes {}

impl BuildSagemakerLabelingJob_InputConfig_DataAttributes {
    pub fn build(self) -> SagemakerLabelingJob_InputConfig_DataAttributes {
        SagemakerLabelingJob_InputConfig_DataAttributes { content_classifiers: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct SagemakerLabelingJob_InputConfig_DataAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerLabelingJob_InputConfig_DataAttributesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerLabelingJob_InputConfig_DataAttributesRef {
        SagemakerLabelingJob_InputConfig_DataAttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerLabelingJob_InputConfig_DataAttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_classifiers` after provisioning.\n"]
    pub fn content_classifiers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.content_classifiers", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerLabelingJob_InputConfig_DataSource_S3DataSource {
    manifest_s3_uri: terrars::PrimField<std::string::String>,
}

impl SagemakerLabelingJob_InputConfig_DataSource_S3DataSource { }

impl terrars::ToListMappable for SagemakerLabelingJob_InputConfig_DataSource_S3DataSource {
    type O = terrars::BlockAssignable<SagemakerLabelingJob_InputConfig_DataSource_S3DataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerLabelingJob_InputConfig_DataSource_S3DataSource {
    #[doc = ""]
    pub manifest_s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerLabelingJob_InputConfig_DataSource_S3DataSource {
    pub fn build(self) -> SagemakerLabelingJob_InputConfig_DataSource_S3DataSource {
        SagemakerLabelingJob_InputConfig_DataSource_S3DataSource { manifest_s3_uri: self.manifest_s3_uri }
    }
}

#[allow(dead_code)]
pub struct SagemakerLabelingJob_InputConfig_DataSource_S3DataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerLabelingJob_InputConfig_DataSource_S3DataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerLabelingJob_InputConfig_DataSource_S3DataSourceRef {
        SagemakerLabelingJob_InputConfig_DataSource_S3DataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerLabelingJob_InputConfig_DataSource_S3DataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `manifest_s3_uri` after provisioning.\n"]
    pub fn manifest_s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manifest_s3_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerLabelingJob_InputConfig_DataSource_SnsDataSource {
    sns_topic_arn: terrars::PrimField<std::string::String>,
}

impl SagemakerLabelingJob_InputConfig_DataSource_SnsDataSource { }

impl terrars::ToListMappable for SagemakerLabelingJob_InputConfig_DataSource_SnsDataSource {
    type O = terrars::BlockAssignable<SagemakerLabelingJob_InputConfig_DataSource_SnsDataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerLabelingJob_InputConfig_DataSource_SnsDataSource {
    #[doc = ""]
    pub sns_topic_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerLabelingJob_InputConfig_DataSource_SnsDataSource {
    pub fn build(self) -> SagemakerLabelingJob_InputConfig_DataSource_SnsDataSource {
        SagemakerLabelingJob_InputConfig_DataSource_SnsDataSource { sns_topic_arn: self.sns_topic_arn }
    }
}

#[allow(dead_code)]
pub struct SagemakerLabelingJob_InputConfig_DataSource_SnsDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerLabelingJob_InputConfig_DataSource_SnsDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerLabelingJob_InputConfig_DataSource_SnsDataSourceRef {
        SagemakerLabelingJob_InputConfig_DataSource_SnsDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerLabelingJob_InputConfig_DataSource_SnsDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sns_topic_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerLabelingJob_InputConfig_DataSourceDynamic {
    s3_data_source: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerLabelingJob_InputConfig_DataSource_S3DataSource>>,
    sns_data_source: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerLabelingJob_InputConfig_DataSource_SnsDataSource>>,
}

#[derive(Serialize)]
pub struct SagemakerLabelingJob_InputConfig_DataSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_data_source: std::option::Option<std::vec::Vec<SagemakerLabelingJob_InputConfig_DataSource_S3DataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sns_data_source: std::option::Option<std::vec::Vec<SagemakerLabelingJob_InputConfig_DataSource_SnsDataSource>>,
    dynamic: SagemakerLabelingJob_InputConfig_DataSourceDynamic,
}

impl SagemakerLabelingJob_InputConfig_DataSource {
    #[doc = "Set the field `s3_data_source`.\n"]
    pub fn set_s3_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerLabelingJob_InputConfig_DataSource_S3DataSource>>,
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

    #[doc = "Set the field `sns_data_source`.\n"]
    pub fn set_sns_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerLabelingJob_InputConfig_DataSource_SnsDataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sns_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sns_data_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerLabelingJob_InputConfig_DataSource {
    type O = terrars::BlockAssignable<SagemakerLabelingJob_InputConfig_DataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerLabelingJob_InputConfig_DataSource {}

impl BuildSagemakerLabelingJob_InputConfig_DataSource {
    pub fn build(self) -> SagemakerLabelingJob_InputConfig_DataSource {
        SagemakerLabelingJob_InputConfig_DataSource {
            s3_data_source: core::default::Default::default(),
            sns_data_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerLabelingJob_InputConfig_DataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerLabelingJob_InputConfig_DataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerLabelingJob_InputConfig_DataSourceRef {
        SagemakerLabelingJob_InputConfig_DataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerLabelingJob_InputConfig_DataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_data_source` after provisioning.\n"]
    pub fn s3_data_source(&self) -> terrars::ListRef<SagemakerLabelingJob_InputConfig_DataSource_S3DataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `sns_data_source` after provisioning.\n"]
    pub fn sns_data_source(&self) -> terrars::ListRef<SagemakerLabelingJob_InputConfig_DataSource_SnsDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sns_data_source", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerLabelingJob_InputConfigDynamic {
    data_attributes: std::option::Option<terrars::DynamicBlock<SagemakerLabelingJob_InputConfig_DataAttributes>>,
    data_source: std::option::Option<terrars::DynamicBlock<SagemakerLabelingJob_InputConfig_DataSource>>,
}

#[derive(Serialize)]
pub struct SagemakerLabelingJob_InputConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_attributes: std::option::Option<std::vec::Vec<SagemakerLabelingJob_InputConfig_DataAttributes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source: std::option::Option<std::vec::Vec<SagemakerLabelingJob_InputConfig_DataSource>>,
    dynamic: SagemakerLabelingJob_InputConfigDynamic,
}

impl SagemakerLabelingJob_InputConfig {
    #[doc = "Set the field `data_attributes`.\n"]
    pub fn set_data_attributes(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerLabelingJob_InputConfig_DataAttributes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_attributes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_source`.\n"]
    pub fn set_data_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerLabelingJob_InputConfig_DataSource>>,
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
}

impl terrars::ToListMappable for SagemakerLabelingJob_InputConfig {
    type O = terrars::BlockAssignable<SagemakerLabelingJob_InputConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerLabelingJob_InputConfig {}

impl BuildSagemakerLabelingJob_InputConfig {
    pub fn build(self) -> SagemakerLabelingJob_InputConfig {
        SagemakerLabelingJob_InputConfig {
            data_attributes: core::default::Default::default(),
            data_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerLabelingJob_InputConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerLabelingJob_InputConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerLabelingJob_InputConfigRef {
        SagemakerLabelingJob_InputConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerLabelingJob_InputConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_attributes` after provisioning.\n"]
    pub fn data_attributes(&self) -> terrars::ListRef<SagemakerLabelingJob_InputConfig_DataAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_attributes", self.base))
    }

    #[doc = "Get a reference to the value of field `data_source` after provisioning.\n"]
    pub fn data_source(&self) -> terrars::ListRef<SagemakerLabelingJob_InputConfig_DataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_source", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig_VpcConfig {
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig_VpcConfig { }

impl terrars::ToListMappable for SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig_VpcConfig {
    type O =
        terrars
        ::BlockAssignable<SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig_VpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig_VpcConfig {
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildSagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig_VpcConfig {
    pub fn build(self) -> SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig_VpcConfig {
        SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig_VpcConfig {
            security_group_ids: self.security_group_ids,
            subnets: self.subnets,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig_VpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig_VpcConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig_VpcConfigRef {
        SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig_VpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig_VpcConfigRef {
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
struct SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfigDynamic {
    vpc_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig_VpcConfig>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig_VpcConfig>>,
    dynamic: SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfigDynamic,
}

impl SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig {
    #[doc = "Set the field `volume_kms_key_id`.\n"]
    pub fn set_volume_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_kms_key_id = std::option::Option::Some(v.into());
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
                            SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig_VpcConfig,
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

impl terrars::ToListMappable for SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig {
    type O = terrars::BlockAssignable<SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig {}

impl BuildSagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig {
    pub fn build(self) -> SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig {
        SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig {
            volume_kms_key_id: core::default::Default::default(),
            vpc_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfigRef {
        SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `volume_kms_key_id` after provisioning.\n"]
    pub fn volume_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(
        &self,
    ) -> terrars::ListRef<SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerLabelingJob_LabelingJobAlgorithmsConfigDynamic {
    labeling_job_resource_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerLabelingJob_LabelingJobAlgorithmsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_active_learning_model_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    labeling_job_algorithm_specification_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labeling_job_resource_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig>>,
    dynamic: SagemakerLabelingJob_LabelingJobAlgorithmsConfigDynamic,
}

impl SagemakerLabelingJob_LabelingJobAlgorithmsConfig {
    #[doc = "Set the field `initial_active_learning_model_arn`.\n"]
    pub fn set_initial_active_learning_model_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.initial_active_learning_model_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labeling_job_resource_config`.\n"]
    pub fn set_labeling_job_resource_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.labeling_job_resource_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.labeling_job_resource_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerLabelingJob_LabelingJobAlgorithmsConfig {
    type O = terrars::BlockAssignable<SagemakerLabelingJob_LabelingJobAlgorithmsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerLabelingJob_LabelingJobAlgorithmsConfig {
    #[doc = ""]
    pub labeling_job_algorithm_specification_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerLabelingJob_LabelingJobAlgorithmsConfig {
    pub fn build(self) -> SagemakerLabelingJob_LabelingJobAlgorithmsConfig {
        SagemakerLabelingJob_LabelingJobAlgorithmsConfig {
            initial_active_learning_model_arn: core::default::Default::default(),
            labeling_job_algorithm_specification_arn: self.labeling_job_algorithm_specification_arn,
            labeling_job_resource_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerLabelingJob_LabelingJobAlgorithmsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerLabelingJob_LabelingJobAlgorithmsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerLabelingJob_LabelingJobAlgorithmsConfigRef {
        SagemakerLabelingJob_LabelingJobAlgorithmsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerLabelingJob_LabelingJobAlgorithmsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `initial_active_learning_model_arn` after provisioning.\n"]
    pub fn initial_active_learning_model_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_active_learning_model_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `labeling_job_algorithm_specification_arn` after provisioning.\n"]
    pub fn labeling_job_algorithm_specification_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.labeling_job_algorithm_specification_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `labeling_job_resource_config` after provisioning.\n"]
    pub fn labeling_job_resource_config(
        &self,
    ) -> terrars::ListRef<SagemakerLabelingJob_LabelingJobAlgorithmsConfig_LabelingJobResourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.labeling_job_resource_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerLabelingJob_OutputConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_output_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sns_topic_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerLabelingJob_OutputConfig {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sns_topic_arn`.\n"]
    pub fn set_sns_topic_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sns_topic_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerLabelingJob_OutputConfig {
    type O = terrars::BlockAssignable<SagemakerLabelingJob_OutputConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerLabelingJob_OutputConfig {
    #[doc = ""]
    pub s3_output_path: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerLabelingJob_OutputConfig {
    pub fn build(self) -> SagemakerLabelingJob_OutputConfig {
        SagemakerLabelingJob_OutputConfig {
            kms_key_id: core::default::Default::default(),
            s3_output_path: self.s3_output_path,
            sns_topic_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerLabelingJob_OutputConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerLabelingJob_OutputConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerLabelingJob_OutputConfigRef {
        SagemakerLabelingJob_OutputConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerLabelingJob_OutputConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_path` after provisioning.\n"]
    pub fn s3_output_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_output_path", self.base))
    }

    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sns_topic_arn", self.base))
    }
}
