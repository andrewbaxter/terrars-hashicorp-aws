#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockEvaluationJobData_ {
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
    application_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_encryption_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_description: std::option::Option<terrars::PrimField<std::string::String>>,
    job_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_config: std::option::Option<std::vec::Vec<BedrockEvaluationJob_EvaluationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_config: std::option::Option<std::vec::Vec<BedrockEvaluationJob_InferenceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_data_config: std::option::Option<std::vec::Vec<BedrockEvaluationJob_OutputDataConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockEvaluationJob_Timeouts>,
    dynamic: BedrockEvaluationJobDynamic,
}

struct BedrockEvaluationJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockEvaluationJobData_>,
}

#[derive(Clone)]
pub struct BedrockEvaluationJob(std::rc::Rc<BedrockEvaluationJob_>);

impl BedrockEvaluationJob {
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

    #[doc = "Set the field `application_type`.\n"]
    pub fn set_application_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().application_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer_encryption_key_id`.\n"]
    pub fn set_customer_encryption_key_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().customer_encryption_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `job_description`.\n"]
    pub fn set_job_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().job_description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `skip_destroy`.\n"]
    pub fn set_skip_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_destroy = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `evaluation_config`.\n"]
    pub fn set_evaluation_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockEvaluationJob_EvaluationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().evaluation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.evaluation_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inference_config`.\n"]
    pub fn set_inference_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockEvaluationJob_InferenceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().inference_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.inference_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_data_config`.\n"]
    pub fn set_output_data_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockEvaluationJob_OutputDataConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().output_data_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.output_data_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockEvaluationJob_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `application_type` after provisioning.\n"]
    pub fn application_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_encryption_key_id` after provisioning.\n"]
    pub fn customer_encryption_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_encryption_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_messages` after provisioning.\n"]
    pub fn failure_messages(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failure_messages", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_arn` after provisioning.\n"]
    pub fn job_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_description` after provisioning.\n"]
    pub fn job_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_name` after provisioning.\n"]
    pub fn job_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_type` after provisioning.\n"]
    pub fn job_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\n"]
    pub fn last_modified_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `skip_destroy` after provisioning.\n"]
    pub fn skip_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_destroy", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `evaluation_config` after provisioning.\n"]
    pub fn evaluation_config(&self) -> terrars::ListRef<BedrockEvaluationJob_EvaluationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluation_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inference_config` after provisioning.\n"]
    pub fn inference_config(&self) -> terrars::ListRef<BedrockEvaluationJob_InferenceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inference_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_data_config` after provisioning.\n"]
    pub fn output_data_config(&self) -> terrars::ListRef<BedrockEvaluationJob_OutputDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockEvaluationJob_TimeoutsRef {
        BedrockEvaluationJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BedrockEvaluationJob {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockEvaluationJob { }

impl terrars::ToListMappable for BedrockEvaluationJob {
    type O = terrars::ListRef<BedrockEvaluationJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockEvaluationJob_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrock_evaluation_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockEvaluationJob {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub job_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockEvaluationJob {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockEvaluationJob {
        let out = BedrockEvaluationJob(std::rc::Rc::new(BedrockEvaluationJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockEvaluationJobData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                application_type: core::default::Default::default(),
                customer_encryption_key_id: core::default::Default::default(),
                job_description: core::default::Default::default(),
                job_name: self.job_name,
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                skip_destroy: core::default::Default::default(),
                tags: core::default::Default::default(),
                evaluation_config: core::default::Default::default(),
                inference_config: core::default::Default::default(),
                output_data_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockEvaluationJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockEvaluationJobRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_type` after provisioning.\n"]
    pub fn application_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_encryption_key_id` after provisioning.\n"]
    pub fn customer_encryption_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_encryption_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_messages` after provisioning.\n"]
    pub fn failure_messages(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failure_messages", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_arn` after provisioning.\n"]
    pub fn job_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_description` after provisioning.\n"]
    pub fn job_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_name` after provisioning.\n"]
    pub fn job_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_type` after provisioning.\n"]
    pub fn job_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\n"]
    pub fn last_modified_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `skip_destroy` after provisioning.\n"]
    pub fn skip_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_destroy", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `evaluation_config` after provisioning.\n"]
    pub fn evaluation_config(&self) -> terrars::ListRef<BedrockEvaluationJob_EvaluationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluation_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inference_config` after provisioning.\n"]
    pub fn inference_config(&self) -> terrars::ListRef<BedrockEvaluationJob_InferenceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inference_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_data_config` after provisioning.\n"]
    pub fn output_data_config(&self) -> terrars::ListRef<BedrockEvaluationJob_OutputDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockEvaluationJob_TimeoutsRef {
        BedrockEvaluationJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BedrockEvaluationJobDynamic {
    evaluation_config: std::option::Option<terrars::DynamicBlock<BedrockEvaluationJob_EvaluationConfig>>,
    inference_config: std::option::Option<terrars::DynamicBlock<BedrockEvaluationJob_InferenceConfig>>,
    output_data_config: std::option::Option<terrars::DynamicBlock<BedrockEvaluationJob_OutputDataConfig>>,
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale_Value {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    float_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale_Value {
    #[doc = "Set the field `float_value`.\n"]
    pub fn set_float_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.float_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_value`.\n"]
    pub fn set_string_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.string_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale_Value {
    type O =
        terrars
        ::BlockAssignable<
            BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale_Value,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale_Value {}

impl BuildBedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale_Value {
    pub fn build(
        self,
    ) -> BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale_Value {
        BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale_Value {
            float_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale_ValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale_ValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale_ValueRef {
        BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale_ValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale_ValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `float_value` after provisioning.\n"]
    pub fn float_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.float_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_value` after provisioning.\n"]
    pub fn string_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScaleDynamic {
    value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale_Value,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale {
    definition: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale_Value,
        >,
    >,
    dynamic: BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScaleDynamic,
}

impl BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale_Value,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale {
    type O =
        terrars
        ::BlockAssignable<
            BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale {
    #[doc = ""]
    pub definition: terrars::PrimField<std::string::String>,
}

impl BuildBedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale {
    pub fn build(
        self,
    ) -> BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale {
        BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale {
            definition: self.definition,
            value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScaleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScaleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScaleRef {
        BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScaleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScaleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.definition", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale_ValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinitionDynamic {
    rating_scale: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition {
    instructions: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rating_scale: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale,
        >,
    >,
    dynamic: BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinitionDynamic,
}

impl BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition {
    #[doc = "Set the field `rating_scale`.\n"]
    pub fn set_rating_scale(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScale,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rating_scale = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rating_scale = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition {
    type O =
        terrars
        ::BlockAssignable<
            BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition {
    #[doc = ""]
    pub instructions: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition {
    pub fn build(
        self,
    ) -> BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition {
        BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition {
            instructions: self.instructions,
            name: self.name,
            rating_scale: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinitionRef {
        BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instructions` after provisioning.\n"]
    pub fn instructions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instructions", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `rating_scale` after provisioning.\n"]
    pub fn rating_scale(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition_RatingScaleRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rating_scale", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetricDynamic {
    custom_metric_definition: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_metric_definition: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition>,
    >,
    dynamic: BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetricDynamic,
}

impl BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric {
    #[doc = "Set the field `custom_metric_definition`.\n"]
    pub fn set_custom_metric_definition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_metric_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_metric_definition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric {
    type O =
        terrars::BlockAssignable<BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric {}

impl BuildBedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric {
    pub fn build(self) -> BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric {
        BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric {
            custom_metric_definition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetricRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetricRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetricRef {
        BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetricRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetricRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_metric_definition` after provisioning.\n"]
    pub fn custom_metric_definition(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric_CustomMetricDefinitionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_metric_definition", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig_BedrockEvaluatorModel {
    model_identifier: terrars::PrimField<std::string::String>,
}

impl BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig_BedrockEvaluatorModel { }

impl terrars
::ToListMappable for BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig_BedrockEvaluatorModel {
    type O =
        terrars
        ::BlockAssignable<BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig_BedrockEvaluatorModel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig_BedrockEvaluatorModel {
    #[doc = ""]
    pub model_identifier: terrars::PrimField<std::string::String>,
}

impl BuildBedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig_BedrockEvaluatorModel {
    pub fn build(self) -> BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig_BedrockEvaluatorModel {
        BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig_BedrockEvaluatorModel {
            model_identifier: self.model_identifier,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig_BedrockEvaluatorModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig_BedrockEvaluatorModelRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig_BedrockEvaluatorModelRef {
        BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig_BedrockEvaluatorModelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig_BedrockEvaluatorModelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `model_identifier` after provisioning.\n"]
    pub fn model_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_identifier", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfigDynamic {
    bedrock_evaluator_model: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig_BedrockEvaluatorModel>,
    >,
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bedrock_evaluator_model: std
    ::option
    ::Option<
        std::vec::Vec<BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig_BedrockEvaluatorModel>,
    >,
    dynamic: BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfigDynamic,
}

impl BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig {
    #[doc = "Set the field `bedrock_evaluator_model`.\n"]
    pub fn set_bedrock_evaluator_model(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig_BedrockEvaluatorModel,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bedrock_evaluator_model = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bedrock_evaluator_model = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig {
    type O = terrars::BlockAssignable<BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig {}

impl BuildBedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig {
    pub fn build(self) -> BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig {
        BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig {
            bedrock_evaluator_model: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfigRef {
        BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bedrock_evaluator_model` after provisioning.\n"]
    pub fn bedrock_evaluator_model(
        &self,
    ) -> terrars
    ::ListRef<BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig_BedrockEvaluatorModelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bedrock_evaluator_model", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfigDynamic {
    custom_metric: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric>>,
    evaluator_model_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig>>,
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_metric: std
    ::option
    ::Option<std::vec::Vec<BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluator_model_config: std
    ::option
    ::Option<std::vec::Vec<BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig>>,
    dynamic: BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfigDynamic,
}

impl BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig {
    #[doc = "Set the field `custom_metric`.\n"]
    pub fn set_custom_metric(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetric,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_metric = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_metric = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `evaluator_model_config`.\n"]
    pub fn set_evaluator_model_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.evaluator_model_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.evaluator_model_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig {
    type O = terrars::BlockAssignable<BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig {}

impl BuildBedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig {
    pub fn build(self) -> BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig {
        BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig {
            custom_metric: core::default::Default::default(),
            evaluator_model_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfigRef {
        BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_metric` after provisioning.\n"]
    pub fn custom_metric(
        &self,
    ) -> terrars::ListRef<BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig_CustomMetricRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_metric", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluator_model_config` after provisioning.\n"]
    pub fn evaluator_model_config(
        &self,
    ) -> terrars::ListRef<BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluator_model_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset_DatasetLocation {
    s3_uri: terrars::PrimField<std::string::String>,
}

impl BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset_DatasetLocation { }

impl terrars
::ToListMappable for BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset_DatasetLocation {
    type O =
        terrars
        ::BlockAssignable<BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset_DatasetLocation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset_DatasetLocation {
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildBedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset_DatasetLocation {
    pub fn build(self) -> BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset_DatasetLocation {
        BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset_DatasetLocation {
            s3_uri: self.s3_uri,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset_DatasetLocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset_DatasetLocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset_DatasetLocationRef {
        BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset_DatasetLocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset_DatasetLocationRef {
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
struct BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_DatasetDynamic {
    dataset_location: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset_DatasetLocation>,
    >,
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_location: std
    ::option
    ::Option<std::vec::Vec<BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset_DatasetLocation>>,
    dynamic: BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_DatasetDynamic,
}

impl BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset {
    #[doc = "Set the field `dataset_location`.\n"]
    pub fn set_dataset_location(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset_DatasetLocation,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dataset_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dataset_location = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset {
    type O = terrars::BlockAssignable<BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset {
    pub fn build(self) -> BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset {
        BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset {
            name: self.name,
            dataset_location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_DatasetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_DatasetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_DatasetRef {
        BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_DatasetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_DatasetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `dataset_location` after provisioning.\n"]
    pub fn dataset_location(
        &self,
    ) -> terrars::ListRef<BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset_DatasetLocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataset_location", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfigDynamic {
    dataset: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset>>,
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig {
    metric_names: terrars::ListField<terrars::PrimField<std::string::String>>,
    task_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset: std::option::Option<std::vec::Vec<BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset>>,
    dynamic: BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfigDynamic,
}

impl BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig {
    #[doc = "Set the field `dataset`.\n"]
    pub fn set_dataset(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_Dataset>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dataset = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dataset = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig {
    type O = terrars::BlockAssignable<BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig {
    #[doc = ""]
    pub metric_names: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub task_type: terrars::PrimField<std::string::String>,
}

impl BuildBedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig {
    pub fn build(self) -> BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig {
        BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig {
            metric_names: self.metric_names,
            task_type: self.task_type,
            dataset: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfigRef {
        BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_names` after provisioning.\n"]
    pub fn metric_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_names", self.base))
    }

    #[doc = "Get a reference to the value of field `task_type` after provisioning.\n"]
    pub fn task_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_type", self.base))
    }

    #[doc = "Get a reference to the value of field `dataset` after provisioning.\n"]
    pub fn dataset(
        &self,
    ) -> terrars::ListRef<BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig_DatasetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataset", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockEvaluationJob_EvaluationConfig_AutomatedDynamic {
    custom_metric_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig>>,
    dataset_metric_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig>>,
    evaluator_model_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig>>,
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_EvaluationConfig_Automated {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_metric_config: std
    ::option
    ::Option<std::vec::Vec<BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_metric_config: std
    ::option
    ::Option<std::vec::Vec<BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluator_model_config: std
    ::option
    ::Option<std::vec::Vec<BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig>>,
    dynamic: BedrockEvaluationJob_EvaluationConfig_AutomatedDynamic,
}

impl BedrockEvaluationJob_EvaluationConfig_Automated {
    #[doc = "Set the field `custom_metric_config`.\n"]
    pub fn set_custom_metric_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_metric_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_metric_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dataset_metric_config`.\n"]
    pub fn set_dataset_metric_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dataset_metric_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dataset_metric_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `evaluator_model_config`.\n"]
    pub fn set_evaluator_model_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.evaluator_model_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.evaluator_model_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockEvaluationJob_EvaluationConfig_Automated {
    type O = terrars::BlockAssignable<BedrockEvaluationJob_EvaluationConfig_Automated>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_EvaluationConfig_Automated {}

impl BuildBedrockEvaluationJob_EvaluationConfig_Automated {
    pub fn build(self) -> BedrockEvaluationJob_EvaluationConfig_Automated {
        BedrockEvaluationJob_EvaluationConfig_Automated {
            custom_metric_config: core::default::Default::default(),
            dataset_metric_config: core::default::Default::default(),
            evaluator_model_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_EvaluationConfig_AutomatedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_EvaluationConfig_AutomatedRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_EvaluationConfig_AutomatedRef {
        BedrockEvaluationJob_EvaluationConfig_AutomatedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_EvaluationConfig_AutomatedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_metric_config` after provisioning.\n"]
    pub fn custom_metric_config(
        &self,
    ) -> terrars::ListRef<BedrockEvaluationJob_EvaluationConfig_Automated_CustomMetricConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_metric_config", self.base))
    }

    #[doc = "Get a reference to the value of field `dataset_metric_config` after provisioning.\n"]
    pub fn dataset_metric_config(
        &self,
    ) -> terrars::ListRef<BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataset_metric_config", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluator_model_config` after provisioning.\n"]
    pub fn evaluator_model_config(
        &self,
    ) -> terrars::ListRef<BedrockEvaluationJob_EvaluationConfig_Automated_EvaluatorModelConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluator_model_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_EvaluationConfig_Human_CustomMetric {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    rating_method: terrars::PrimField<std::string::String>,
}

impl BedrockEvaluationJob_EvaluationConfig_Human_CustomMetric {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockEvaluationJob_EvaluationConfig_Human_CustomMetric {
    type O = terrars::BlockAssignable<BedrockEvaluationJob_EvaluationConfig_Human_CustomMetric>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_EvaluationConfig_Human_CustomMetric {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rating_method: terrars::PrimField<std::string::String>,
}

impl BuildBedrockEvaluationJob_EvaluationConfig_Human_CustomMetric {
    pub fn build(self) -> BedrockEvaluationJob_EvaluationConfig_Human_CustomMetric {
        BedrockEvaluationJob_EvaluationConfig_Human_CustomMetric {
            description: core::default::Default::default(),
            name: self.name,
            rating_method: self.rating_method,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_EvaluationConfig_Human_CustomMetricRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_EvaluationConfig_Human_CustomMetricRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_EvaluationConfig_Human_CustomMetricRef {
        BedrockEvaluationJob_EvaluationConfig_Human_CustomMetricRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_EvaluationConfig_Human_CustomMetricRef {
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

    #[doc = "Get a reference to the value of field `rating_method` after provisioning.\n"]
    pub fn rating_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rating_method", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_EvaluationConfig_Human_HumanWorkflowConfig {
    flow_definition_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instructions: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockEvaluationJob_EvaluationConfig_Human_HumanWorkflowConfig {
    #[doc = "Set the field `instructions`.\n"]
    pub fn set_instructions(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instructions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockEvaluationJob_EvaluationConfig_Human_HumanWorkflowConfig {
    type O = terrars::BlockAssignable<BedrockEvaluationJob_EvaluationConfig_Human_HumanWorkflowConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_EvaluationConfig_Human_HumanWorkflowConfig {
    #[doc = ""]
    pub flow_definition_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockEvaluationJob_EvaluationConfig_Human_HumanWorkflowConfig {
    pub fn build(self) -> BedrockEvaluationJob_EvaluationConfig_Human_HumanWorkflowConfig {
        BedrockEvaluationJob_EvaluationConfig_Human_HumanWorkflowConfig {
            flow_definition_arn: self.flow_definition_arn,
            instructions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_EvaluationConfig_Human_HumanWorkflowConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_EvaluationConfig_Human_HumanWorkflowConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_EvaluationConfig_Human_HumanWorkflowConfigRef {
        BedrockEvaluationJob_EvaluationConfig_Human_HumanWorkflowConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_EvaluationConfig_Human_HumanWorkflowConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `flow_definition_arn` after provisioning.\n"]
    pub fn flow_definition_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flow_definition_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `instructions` after provisioning.\n"]
    pub fn instructions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instructions", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockEvaluationJob_EvaluationConfig_HumanDynamic {
    custom_metric: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockEvaluationJob_EvaluationConfig_Human_CustomMetric>>,
    dataset_metric_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig>>,
    human_workflow_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockEvaluationJob_EvaluationConfig_Human_HumanWorkflowConfig>>,
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_EvaluationConfig_Human {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_metric: std::option::Option<std::vec::Vec<BedrockEvaluationJob_EvaluationConfig_Human_CustomMetric>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_metric_config: std
    ::option
    ::Option<std::vec::Vec<BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    human_workflow_config: std
    ::option
    ::Option<std::vec::Vec<BedrockEvaluationJob_EvaluationConfig_Human_HumanWorkflowConfig>>,
    dynamic: BedrockEvaluationJob_EvaluationConfig_HumanDynamic,
}

impl BedrockEvaluationJob_EvaluationConfig_Human {
    #[doc = "Set the field `custom_metric`.\n"]
    pub fn set_custom_metric(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockEvaluationJob_EvaluationConfig_Human_CustomMetric>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_metric = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_metric = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dataset_metric_config`.\n"]
    pub fn set_dataset_metric_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dataset_metric_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dataset_metric_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `human_workflow_config`.\n"]
    pub fn set_human_workflow_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockEvaluationJob_EvaluationConfig_Human_HumanWorkflowConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.human_workflow_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.human_workflow_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockEvaluationJob_EvaluationConfig_Human {
    type O = terrars::BlockAssignable<BedrockEvaluationJob_EvaluationConfig_Human>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_EvaluationConfig_Human {}

impl BuildBedrockEvaluationJob_EvaluationConfig_Human {
    pub fn build(self) -> BedrockEvaluationJob_EvaluationConfig_Human {
        BedrockEvaluationJob_EvaluationConfig_Human {
            custom_metric: core::default::Default::default(),
            dataset_metric_config: core::default::Default::default(),
            human_workflow_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_EvaluationConfig_HumanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_EvaluationConfig_HumanRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_EvaluationConfig_HumanRef {
        BedrockEvaluationJob_EvaluationConfig_HumanRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_EvaluationConfig_HumanRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_metric` after provisioning.\n"]
    pub fn custom_metric(&self) -> terrars::ListRef<BedrockEvaluationJob_EvaluationConfig_Human_CustomMetricRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_metric", self.base))
    }

    #[doc = "Get a reference to the value of field `dataset_metric_config` after provisioning.\n"]
    pub fn dataset_metric_config(
        &self,
    ) -> terrars::ListRef<BedrockEvaluationJob_EvaluationConfig_Human_DatasetMetricConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataset_metric_config", self.base))
    }

    #[doc = "Get a reference to the value of field `human_workflow_config` after provisioning.\n"]
    pub fn human_workflow_config(
        &self,
    ) -> terrars::ListRef<BedrockEvaluationJob_EvaluationConfig_Human_HumanWorkflowConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.human_workflow_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockEvaluationJob_EvaluationConfigDynamic {
    automated: std::option::Option<terrars::DynamicBlock<BedrockEvaluationJob_EvaluationConfig_Automated>>,
    human: std::option::Option<terrars::DynamicBlock<BedrockEvaluationJob_EvaluationConfig_Human>>,
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_EvaluationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automated: std::option::Option<std::vec::Vec<BedrockEvaluationJob_EvaluationConfig_Automated>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    human: std::option::Option<std::vec::Vec<BedrockEvaluationJob_EvaluationConfig_Human>>,
    dynamic: BedrockEvaluationJob_EvaluationConfigDynamic,
}

impl BedrockEvaluationJob_EvaluationConfig {
    #[doc = "Set the field `automated`.\n"]
    pub fn set_automated(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockEvaluationJob_EvaluationConfig_Automated>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.automated = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.automated = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `human`.\n"]
    pub fn set_human(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockEvaluationJob_EvaluationConfig_Human>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.human = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.human = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockEvaluationJob_EvaluationConfig {
    type O = terrars::BlockAssignable<BedrockEvaluationJob_EvaluationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_EvaluationConfig {}

impl BuildBedrockEvaluationJob_EvaluationConfig {
    pub fn build(self) -> BedrockEvaluationJob_EvaluationConfig {
        BedrockEvaluationJob_EvaluationConfig {
            automated: core::default::Default::default(),
            human: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_EvaluationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_EvaluationConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockEvaluationJob_EvaluationConfigRef {
        BedrockEvaluationJob_EvaluationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_EvaluationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `automated` after provisioning.\n"]
    pub fn automated(&self) -> terrars::ListRef<BedrockEvaluationJob_EvaluationConfig_AutomatedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automated", self.base))
    }

    #[doc = "Get a reference to the value of field `human` after provisioning.\n"]
    pub fn human(&self) -> terrars::ListRef<BedrockEvaluationJob_EvaluationConfig_HumanRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.human", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_InferenceConfig_Model_BedrockModel_PerformanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latency: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockEvaluationJob_InferenceConfig_Model_BedrockModel_PerformanceConfig {
    #[doc = "Set the field `latency`.\n"]
    pub fn set_latency(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.latency = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockEvaluationJob_InferenceConfig_Model_BedrockModel_PerformanceConfig {
    type O = terrars::BlockAssignable<BedrockEvaluationJob_InferenceConfig_Model_BedrockModel_PerformanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_InferenceConfig_Model_BedrockModel_PerformanceConfig {}

impl BuildBedrockEvaluationJob_InferenceConfig_Model_BedrockModel_PerformanceConfig {
    pub fn build(self) -> BedrockEvaluationJob_InferenceConfig_Model_BedrockModel_PerformanceConfig {
        BedrockEvaluationJob_InferenceConfig_Model_BedrockModel_PerformanceConfig {
            latency: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_InferenceConfig_Model_BedrockModel_PerformanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_InferenceConfig_Model_BedrockModel_PerformanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_InferenceConfig_Model_BedrockModel_PerformanceConfigRef {
        BedrockEvaluationJob_InferenceConfig_Model_BedrockModel_PerformanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_InferenceConfig_Model_BedrockModel_PerformanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `latency` after provisioning.\n"]
    pub fn latency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latency", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockEvaluationJob_InferenceConfig_Model_BedrockModelDynamic {
    performance_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockEvaluationJob_InferenceConfig_Model_BedrockModel_PerformanceConfig>>,
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_InferenceConfig_Model_BedrockModel {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_params: std::option::Option<terrars::PrimField<std::string::String>>,
    model_identifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    performance_config: std
    ::option
    ::Option<std::vec::Vec<BedrockEvaluationJob_InferenceConfig_Model_BedrockModel_PerformanceConfig>>,
    dynamic: BedrockEvaluationJob_InferenceConfig_Model_BedrockModelDynamic,
}

impl BedrockEvaluationJob_InferenceConfig_Model_BedrockModel {
    #[doc = "Set the field `inference_params`.\n"]
    pub fn set_inference_params(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.inference_params = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `performance_config`.\n"]
    pub fn set_performance_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockEvaluationJob_InferenceConfig_Model_BedrockModel_PerformanceConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.performance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.performance_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockEvaluationJob_InferenceConfig_Model_BedrockModel {
    type O = terrars::BlockAssignable<BedrockEvaluationJob_InferenceConfig_Model_BedrockModel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_InferenceConfig_Model_BedrockModel {
    #[doc = ""]
    pub model_identifier: terrars::PrimField<std::string::String>,
}

impl BuildBedrockEvaluationJob_InferenceConfig_Model_BedrockModel {
    pub fn build(self) -> BedrockEvaluationJob_InferenceConfig_Model_BedrockModel {
        BedrockEvaluationJob_InferenceConfig_Model_BedrockModel {
            inference_params: core::default::Default::default(),
            model_identifier: self.model_identifier,
            performance_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_InferenceConfig_Model_BedrockModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_InferenceConfig_Model_BedrockModelRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_InferenceConfig_Model_BedrockModelRef {
        BedrockEvaluationJob_InferenceConfig_Model_BedrockModelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_InferenceConfig_Model_BedrockModelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `inference_params` after provisioning.\n"]
    pub fn inference_params(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inference_params", self.base))
    }

    #[doc = "Get a reference to the value of field `model_identifier` after provisioning.\n"]
    pub fn model_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `performance_config` after provisioning.\n"]
    pub fn performance_config(
        &self,
    ) -> terrars::ListRef<BedrockEvaluationJob_InferenceConfig_Model_BedrockModel_PerformanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.performance_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_InferenceConfig_Model_PrecomputedInferenceSource {
    inference_source_identifier: terrars::PrimField<std::string::String>,
}

impl BedrockEvaluationJob_InferenceConfig_Model_PrecomputedInferenceSource { }

impl terrars::ToListMappable for BedrockEvaluationJob_InferenceConfig_Model_PrecomputedInferenceSource {
    type O = terrars::BlockAssignable<BedrockEvaluationJob_InferenceConfig_Model_PrecomputedInferenceSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_InferenceConfig_Model_PrecomputedInferenceSource {
    #[doc = ""]
    pub inference_source_identifier: terrars::PrimField<std::string::String>,
}

impl BuildBedrockEvaluationJob_InferenceConfig_Model_PrecomputedInferenceSource {
    pub fn build(self) -> BedrockEvaluationJob_InferenceConfig_Model_PrecomputedInferenceSource {
        BedrockEvaluationJob_InferenceConfig_Model_PrecomputedInferenceSource {
            inference_source_identifier: self.inference_source_identifier,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_InferenceConfig_Model_PrecomputedInferenceSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_InferenceConfig_Model_PrecomputedInferenceSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_InferenceConfig_Model_PrecomputedInferenceSourceRef {
        BedrockEvaluationJob_InferenceConfig_Model_PrecomputedInferenceSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_InferenceConfig_Model_PrecomputedInferenceSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `inference_source_identifier` after provisioning.\n"]
    pub fn inference_source_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inference_source_identifier", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockEvaluationJob_InferenceConfig_ModelDynamic {
    bedrock_model: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockEvaluationJob_InferenceConfig_Model_BedrockModel>>,
    precomputed_inference_source: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockEvaluationJob_InferenceConfig_Model_PrecomputedInferenceSource>>,
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_InferenceConfig_Model {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bedrock_model: std::option::Option<std::vec::Vec<BedrockEvaluationJob_InferenceConfig_Model_BedrockModel>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    precomputed_inference_source: std
    ::option
    ::Option<std::vec::Vec<BedrockEvaluationJob_InferenceConfig_Model_PrecomputedInferenceSource>>,
    dynamic: BedrockEvaluationJob_InferenceConfig_ModelDynamic,
}

impl BedrockEvaluationJob_InferenceConfig_Model {
    #[doc = "Set the field `bedrock_model`.\n"]
    pub fn set_bedrock_model(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockEvaluationJob_InferenceConfig_Model_BedrockModel>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bedrock_model = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bedrock_model = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `precomputed_inference_source`.\n"]
    pub fn set_precomputed_inference_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockEvaluationJob_InferenceConfig_Model_PrecomputedInferenceSource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.precomputed_inference_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.precomputed_inference_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockEvaluationJob_InferenceConfig_Model {
    type O = terrars::BlockAssignable<BedrockEvaluationJob_InferenceConfig_Model>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_InferenceConfig_Model {}

impl BuildBedrockEvaluationJob_InferenceConfig_Model {
    pub fn build(self) -> BedrockEvaluationJob_InferenceConfig_Model {
        BedrockEvaluationJob_InferenceConfig_Model {
            bedrock_model: core::default::Default::default(),
            precomputed_inference_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_InferenceConfig_ModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_InferenceConfig_ModelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockEvaluationJob_InferenceConfig_ModelRef {
        BedrockEvaluationJob_InferenceConfig_ModelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_InferenceConfig_ModelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bedrock_model` after provisioning.\n"]
    pub fn bedrock_model(&self) -> terrars::ListRef<BedrockEvaluationJob_InferenceConfig_Model_BedrockModelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bedrock_model", self.base))
    }

    #[doc = "Get a reference to the value of field `precomputed_inference_source` after provisioning.\n"]
    pub fn precomputed_inference_source(
        &self,
    ) -> terrars::ListRef<BedrockEvaluationJob_InferenceConfig_Model_PrecomputedInferenceSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.precomputed_inference_source", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration_VectorSearchConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_of_results: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration_VectorSearchConfiguration {
    #[doc = "Set the field `number_of_results`.\n"]
    pub fn set_number_of_results(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.number_of_results = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration_VectorSearchConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration_VectorSearchConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration_VectorSearchConfiguration {}

impl BuildBedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration_VectorSearchConfiguration {
    pub fn build(
        self,
    ) -> BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration_VectorSearchConfiguration {
        BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration_VectorSearchConfiguration {
            number_of_results: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration_VectorSearchConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration_VectorSearchConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration_VectorSearchConfigurationRef {
        BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration_VectorSearchConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration_VectorSearchConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `number_of_results` after provisioning.\n"]
    pub fn number_of_results(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_results", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfigurationDynamic {
    vector_search_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration_VectorSearchConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vector_search_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration_VectorSearchConfiguration,
        >,
    >,
    dynamic: BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfigurationDynamic,
}

impl BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration {
    #[doc = "Set the field `vector_search_configuration`.\n"]
    pub fn set_vector_search_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration_VectorSearchConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vector_search_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vector_search_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration {}

impl BuildBedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration {
    pub fn build(
        self,
    ) -> BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration {
        BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration {
            vector_search_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfigurationRef {
        BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `vector_search_configuration` after provisioning.\n"]
    pub fn vector_search_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration_VectorSearchConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vector_search_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfigDynamic {
    retrieval_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig {
    knowledge_base_id: terrars::PrimField<std::string::String>,
    model_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retrieval_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration,
        >,
    >,
    dynamic: BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfigDynamic,
}

impl BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig {
    #[doc = "Set the field `retrieval_configuration`.\n"]
    pub fn set_retrieval_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retrieval_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retrieval_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig {
    type O =
        terrars
        ::BlockAssignable<
            BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig {
    #[doc = ""]
    pub knowledge_base_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub model_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig {
    pub fn build(
        self,
    ) -> BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig {
        BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig {
            knowledge_base_id: self.knowledge_base_id,
            model_arn: self.model_arn,
            retrieval_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfigRef {
        BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `knowledge_base_id` after provisioning.\n"]
    pub fn knowledge_base_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.knowledge_base_id", self.base))
    }

    #[doc = "Get a reference to the value of field `model_arn` after provisioning.\n"]
    pub fn model_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `retrieval_configuration` after provisioning.\n"]
    pub fn retrieval_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retrieval_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig_KnowledgeBaseRetrievalConfigurationDynamic {
    vector_search_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration_VectorSearchConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig_KnowledgeBaseRetrievalConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vector_search_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration_VectorSearchConfiguration,
        >,
    >,
    dynamic: BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig_KnowledgeBaseRetrievalConfigurationDynamic,
}

impl BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig_KnowledgeBaseRetrievalConfiguration {
    #[doc = "Set the field `vector_search_configuration`.\n"]
    pub fn set_vector_search_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration_VectorSearchConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vector_search_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vector_search_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig_KnowledgeBaseRetrievalConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig_KnowledgeBaseRetrievalConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig_KnowledgeBaseRetrievalConfiguration {}

impl BuildBedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig_KnowledgeBaseRetrievalConfiguration {
    pub fn build(
        self,
    ) -> BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig_KnowledgeBaseRetrievalConfiguration {
        BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig_KnowledgeBaseRetrievalConfiguration {
            vector_search_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig_KnowledgeBaseRetrievalConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig_KnowledgeBaseRetrievalConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig_KnowledgeBaseRetrievalConfigurationRef {
        BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig_KnowledgeBaseRetrievalConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig_KnowledgeBaseRetrievalConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `vector_search_configuration` after provisioning.\n"]
    pub fn vector_search_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig_RetrievalConfiguration_VectorSearchConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vector_search_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfigDynamic {
    knowledge_base_retrieval_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig_KnowledgeBaseRetrievalConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig {
    knowledge_base_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    knowledge_base_retrieval_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig_KnowledgeBaseRetrievalConfiguration,
        >,
    >,
    dynamic: BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfigDynamic,
}

impl BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig {
    #[doc = "Set the field `knowledge_base_retrieval_configuration`.\n"]
    pub fn set_knowledge_base_retrieval_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig_KnowledgeBaseRetrievalConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.knowledge_base_retrieval_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.knowledge_base_retrieval_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig {
    type O =
        terrars::BlockAssignable<BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig {
    #[doc = ""]
    pub knowledge_base_id: terrars::PrimField<std::string::String>,
}

impl BuildBedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig {
    pub fn build(self) -> BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig {
        BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig {
            knowledge_base_id: self.knowledge_base_id,
            knowledge_base_retrieval_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfigRef {
        BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `knowledge_base_id` after provisioning.\n"]
    pub fn knowledge_base_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.knowledge_base_id", self.base))
    }

    #[doc = "Get a reference to the value of field `knowledge_base_retrieval_configuration` after provisioning.\n"]
    pub fn knowledge_base_retrieval_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig_KnowledgeBaseRetrievalConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.knowledge_base_retrieval_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfigDynamic {
    retrieve_and_generate_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig>,
    >,
    retrieve_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig>,
    >,
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retrieve_and_generate_config: std
    ::option
    ::Option<
        std::vec::Vec<BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retrieve_config: std
    ::option
    ::Option<std::vec::Vec<BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig>>,
    dynamic: BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfigDynamic,
}

impl BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig {
    #[doc = "Set the field `retrieve_and_generate_config`.\n"]
    pub fn set_retrieve_and_generate_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retrieve_and_generate_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retrieve_and_generate_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retrieve_config`.\n"]
    pub fn set_retrieve_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retrieve_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retrieve_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig {
    type O = terrars::BlockAssignable<BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig {}

impl BuildBedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig {
    pub fn build(self) -> BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig {
        BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig {
            retrieve_and_generate_config: core::default::Default::default(),
            retrieve_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfigRef {
        BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `retrieve_and_generate_config` after provisioning.\n"]
    pub fn retrieve_and_generate_config(
        &self,
    ) -> terrars
    ::ListRef<BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveAndGenerateConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retrieve_and_generate_config", self.base))
    }

    #[doc = "Get a reference to the value of field `retrieve_config` after provisioning.\n"]
    pub fn retrieve_config(
        &self,
    ) -> terrars::ListRef<BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig_RetrieveConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retrieve_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveAndGenerateSourceConfig {
    rag_source_identifier: terrars::PrimField<std::string::String>,
}

impl BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveAndGenerateSourceConfig { }

impl terrars
::ToListMappable for BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveAndGenerateSourceConfig {
    type O =
        terrars
        ::BlockAssignable<
            BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveAndGenerateSourceConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveAndGenerateSourceConfig {
    #[doc = ""]
    pub rag_source_identifier: terrars::PrimField<std::string::String>,
}

impl BuildBedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveAndGenerateSourceConfig {
    pub fn build(
        self,
    ) -> BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveAndGenerateSourceConfig {
        BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveAndGenerateSourceConfig {
            rag_source_identifier: self.rag_source_identifier,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveAndGenerateSourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveAndGenerateSourceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveAndGenerateSourceConfigRef {
        BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveAndGenerateSourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveAndGenerateSourceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rag_source_identifier` after provisioning.\n"]
    pub fn rag_source_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rag_source_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveSourceConfig {
    rag_source_identifier: terrars::PrimField<std::string::String>,
}

impl BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveSourceConfig { }

impl terrars
::ToListMappable for BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveSourceConfig {
    type O =
        terrars
        ::BlockAssignable<
            BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveSourceConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveSourceConfig {
    #[doc = ""]
    pub rag_source_identifier: terrars::PrimField<std::string::String>,
}

impl BuildBedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveSourceConfig {
    pub fn build(
        self,
    ) -> BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveSourceConfig {
        BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveSourceConfig {
            rag_source_identifier: self.rag_source_identifier,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveSourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveSourceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveSourceConfigRef {
        BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveSourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveSourceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rag_source_identifier` after provisioning.\n"]
    pub fn rag_source_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rag_source_identifier", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfigDynamic {
    retrieve_and_generate_source_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveAndGenerateSourceConfig,
        >,
    >,
    retrieve_source_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveSourceConfig>,
    >,
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retrieve_and_generate_source_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveAndGenerateSourceConfig,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retrieve_source_config: std
    ::option
    ::Option<
        std::vec::Vec<BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveSourceConfig>,
    >,
    dynamic: BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfigDynamic,
}

impl BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig {
    #[doc = "Set the field `retrieve_and_generate_source_config`.\n"]
    pub fn set_retrieve_and_generate_source_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveAndGenerateSourceConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retrieve_and_generate_source_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retrieve_and_generate_source_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retrieve_source_config`.\n"]
    pub fn set_retrieve_source_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveSourceConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retrieve_source_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retrieve_source_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig {
    type O = terrars::BlockAssignable<BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig {}

impl BuildBedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig {
    pub fn build(self) -> BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig {
        BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig {
            retrieve_and_generate_source_config: core::default::Default::default(),
            retrieve_source_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfigRef {
        BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `retrieve_and_generate_source_config` after provisioning.\n"]
    pub fn retrieve_and_generate_source_config(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveAndGenerateSourceConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retrieve_and_generate_source_config", self.base))
    }

    #[doc = "Get a reference to the value of field `retrieve_source_config` after provisioning.\n"]
    pub fn retrieve_source_config(
        &self,
    ) -> terrars
    ::ListRef<BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig_RetrieveSourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retrieve_source_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockEvaluationJob_InferenceConfig_RagConfigDynamic {
    knowledge_base_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig>>,
    precomputed_rag_source_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig>>,
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_InferenceConfig_RagConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    knowledge_base_config: std
    ::option
    ::Option<std::vec::Vec<BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    precomputed_rag_source_config: std
    ::option
    ::Option<std::vec::Vec<BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig>>,
    dynamic: BedrockEvaluationJob_InferenceConfig_RagConfigDynamic,
}

impl BedrockEvaluationJob_InferenceConfig_RagConfig {
    #[doc = "Set the field `knowledge_base_config`.\n"]
    pub fn set_knowledge_base_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.knowledge_base_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.knowledge_base_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `precomputed_rag_source_config`.\n"]
    pub fn set_precomputed_rag_source_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.precomputed_rag_source_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.precomputed_rag_source_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockEvaluationJob_InferenceConfig_RagConfig {
    type O = terrars::BlockAssignable<BedrockEvaluationJob_InferenceConfig_RagConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_InferenceConfig_RagConfig {}

impl BuildBedrockEvaluationJob_InferenceConfig_RagConfig {
    pub fn build(self) -> BedrockEvaluationJob_InferenceConfig_RagConfig {
        BedrockEvaluationJob_InferenceConfig_RagConfig {
            knowledge_base_config: core::default::Default::default(),
            precomputed_rag_source_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_InferenceConfig_RagConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_InferenceConfig_RagConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockEvaluationJob_InferenceConfig_RagConfigRef {
        BedrockEvaluationJob_InferenceConfig_RagConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_InferenceConfig_RagConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `knowledge_base_config` after provisioning.\n"]
    pub fn knowledge_base_config(
        &self,
    ) -> terrars::ListRef<BedrockEvaluationJob_InferenceConfig_RagConfig_KnowledgeBaseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.knowledge_base_config", self.base))
    }

    #[doc = "Get a reference to the value of field `precomputed_rag_source_config` after provisioning.\n"]
    pub fn precomputed_rag_source_config(
        &self,
    ) -> terrars::ListRef<BedrockEvaluationJob_InferenceConfig_RagConfig_PrecomputedRagSourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.precomputed_rag_source_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockEvaluationJob_InferenceConfigDynamic {
    model: std::option::Option<terrars::DynamicBlock<BedrockEvaluationJob_InferenceConfig_Model>>,
    rag_config: std::option::Option<terrars::DynamicBlock<BedrockEvaluationJob_InferenceConfig_RagConfig>>,
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_InferenceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std::option::Option<std::vec::Vec<BedrockEvaluationJob_InferenceConfig_Model>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rag_config: std::option::Option<std::vec::Vec<BedrockEvaluationJob_InferenceConfig_RagConfig>>,
    dynamic: BedrockEvaluationJob_InferenceConfigDynamic,
}

impl BedrockEvaluationJob_InferenceConfig {
    #[doc = "Set the field `model`.\n"]
    pub fn set_model(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockEvaluationJob_InferenceConfig_Model>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rag_config`.\n"]
    pub fn set_rag_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockEvaluationJob_InferenceConfig_RagConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rag_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rag_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockEvaluationJob_InferenceConfig {
    type O = terrars::BlockAssignable<BedrockEvaluationJob_InferenceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_InferenceConfig {}

impl BuildBedrockEvaluationJob_InferenceConfig {
    pub fn build(self) -> BedrockEvaluationJob_InferenceConfig {
        BedrockEvaluationJob_InferenceConfig {
            model: core::default::Default::default(),
            rag_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_InferenceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_InferenceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockEvaluationJob_InferenceConfigRef {
        BedrockEvaluationJob_InferenceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_InferenceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> terrars::ListRef<BedrockEvaluationJob_InferenceConfig_ModelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model", self.base))
    }

    #[doc = "Get a reference to the value of field `rag_config` after provisioning.\n"]
    pub fn rag_config(&self) -> terrars::ListRef<BedrockEvaluationJob_InferenceConfig_RagConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rag_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockEvaluationJob_OutputDataConfig {
    s3_uri: terrars::PrimField<std::string::String>,
}

impl BedrockEvaluationJob_OutputDataConfig { }

impl terrars::ToListMappable for BedrockEvaluationJob_OutputDataConfig {
    type O = terrars::BlockAssignable<BedrockEvaluationJob_OutputDataConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_OutputDataConfig {
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildBedrockEvaluationJob_OutputDataConfig {
    pub fn build(self) -> BedrockEvaluationJob_OutputDataConfig {
        BedrockEvaluationJob_OutputDataConfig { s3_uri: self.s3_uri }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_OutputDataConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_OutputDataConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockEvaluationJob_OutputDataConfigRef {
        BedrockEvaluationJob_OutputDataConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_OutputDataConfigRef {
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
pub struct BedrockEvaluationJob_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockEvaluationJob_Timeouts {
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

impl terrars::ToListMappable for BedrockEvaluationJob_Timeouts {
    type O = terrars::BlockAssignable<BedrockEvaluationJob_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockEvaluationJob_Timeouts {}

impl BuildBedrockEvaluationJob_Timeouts {
    pub fn build(self) -> BedrockEvaluationJob_Timeouts {
        BedrockEvaluationJob_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockEvaluationJob_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockEvaluationJob_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockEvaluationJob_TimeoutsRef {
        BedrockEvaluationJob_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockEvaluationJob_TimeoutsRef {
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
