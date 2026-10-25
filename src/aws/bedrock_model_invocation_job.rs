#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockModelInvocationJobData_ {
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
    job_name: terrars::PrimField<std::string::String>,
    model_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_duration_in_hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_data_config: std::option::Option<std::vec::Vec<BedrockModelInvocationJob_InputDataConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_data_config: std::option::Option<std::vec::Vec<BedrockModelInvocationJob_OutputDataConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockModelInvocationJob_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<BedrockModelInvocationJob_VpcConfig>>,
    dynamic: BedrockModelInvocationJobDynamic,
}

struct BedrockModelInvocationJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockModelInvocationJobData_>,
}

#[derive(Clone)]
pub struct BedrockModelInvocationJob(std::rc::Rc<BedrockModelInvocationJob_>);

impl BedrockModelInvocationJob {
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

    #[doc = "Set the field `skip_destroy`.\n"]
    pub fn set_skip_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_duration_in_hours`.\n"]
    pub fn set_timeout_duration_in_hours(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout_duration_in_hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_data_config`.\n"]
    pub fn set_input_data_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockModelInvocationJob_InputDataConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input_data_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input_data_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_data_config`.\n"]
    pub fn set_output_data_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockModelInvocationJob_OutputDataConfig>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockModelInvocationJob_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockModelInvocationJob_VpcConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_record_count` after provisioning.\n"]
    pub fn error_record_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_record_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_arn` after provisioning.\n"]
    pub fn job_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_expiration_time` after provisioning.\n"]
    pub fn job_expiration_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_expiration_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_name` after provisioning.\n"]
    pub fn job_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_invocation_type` after provisioning.\n"]
    pub fn model_invocation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_invocation_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `processed_record_count` after provisioning.\n"]
    pub fn processed_record_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.processed_record_count", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `submit_time` after provisioning.\n"]
    pub fn submit_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.submit_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `success_record_count` after provisioning.\n"]
    pub fn success_record_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_record_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout_duration_in_hours` after provisioning.\n"]
    pub fn timeout_duration_in_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_duration_in_hours", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `total_record_count` after provisioning.\n"]
    pub fn total_record_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_record_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_data_config` after provisioning.\n"]
    pub fn input_data_config(&self) -> terrars::ListRef<BedrockModelInvocationJob_InputDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_data_config` after provisioning.\n"]
    pub fn output_data_config(&self) -> terrars::ListRef<BedrockModelInvocationJob_OutputDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockModelInvocationJob_TimeoutsRef {
        BedrockModelInvocationJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<BedrockModelInvocationJob_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

impl terrars::Referable for BedrockModelInvocationJob {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockModelInvocationJob { }

impl terrars::ToListMappable for BedrockModelInvocationJob {
    type O = terrars::ListRef<BedrockModelInvocationJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockModelInvocationJob_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrock_model_invocation_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockModelInvocationJob {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub job_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub model_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockModelInvocationJob {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockModelInvocationJob {
        let out = BedrockModelInvocationJob(std::rc::Rc::new(BedrockModelInvocationJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockModelInvocationJobData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                job_name: self.job_name,
                model_id: self.model_id,
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                skip_destroy: core::default::Default::default(),
                timeout_duration_in_hours: core::default::Default::default(),
                input_data_config: core::default::Default::default(),
                output_data_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockModelInvocationJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockModelInvocationJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockModelInvocationJobRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_record_count` after provisioning.\n"]
    pub fn error_record_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_record_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_arn` after provisioning.\n"]
    pub fn job_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_expiration_time` after provisioning.\n"]
    pub fn job_expiration_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_expiration_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_name` after provisioning.\n"]
    pub fn job_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_invocation_type` after provisioning.\n"]
    pub fn model_invocation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_invocation_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `processed_record_count` after provisioning.\n"]
    pub fn processed_record_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.processed_record_count", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `submit_time` after provisioning.\n"]
    pub fn submit_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.submit_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `success_record_count` after provisioning.\n"]
    pub fn success_record_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_record_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout_duration_in_hours` after provisioning.\n"]
    pub fn timeout_duration_in_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_duration_in_hours", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `total_record_count` after provisioning.\n"]
    pub fn total_record_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_record_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_data_config` after provisioning.\n"]
    pub fn input_data_config(&self) -> terrars::ListRef<BedrockModelInvocationJob_InputDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_data_config` after provisioning.\n"]
    pub fn output_data_config(&self) -> terrars::ListRef<BedrockModelInvocationJob_OutputDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockModelInvocationJob_TimeoutsRef {
        BedrockModelInvocationJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<BedrockModelInvocationJob_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BedrockModelInvocationJobDynamic {
    input_data_config: std::option::Option<terrars::DynamicBlock<BedrockModelInvocationJob_InputDataConfig>>,
    output_data_config: std::option::Option<terrars::DynamicBlock<BedrockModelInvocationJob_OutputDataConfig>>,
    vpc_config: std::option::Option<terrars::DynamicBlock<BedrockModelInvocationJob_VpcConfig>>,
}

#[derive(Serialize)]
pub struct BedrockModelInvocationJob_InputDataConfig_S3InputDataConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_bucket_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_input_format: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_uri: terrars::PrimField<std::string::String>,
}

impl BedrockModelInvocationJob_InputDataConfig_S3InputDataConfig {
    #[doc = "Set the field `s3_bucket_owner`.\n"]
    pub fn set_s3_bucket_owner(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_bucket_owner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_input_format`.\n"]
    pub fn set_s3_input_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_input_format = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockModelInvocationJob_InputDataConfig_S3InputDataConfig {
    type O = terrars::BlockAssignable<BedrockModelInvocationJob_InputDataConfig_S3InputDataConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockModelInvocationJob_InputDataConfig_S3InputDataConfig {
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildBedrockModelInvocationJob_InputDataConfig_S3InputDataConfig {
    pub fn build(self) -> BedrockModelInvocationJob_InputDataConfig_S3InputDataConfig {
        BedrockModelInvocationJob_InputDataConfig_S3InputDataConfig {
            s3_bucket_owner: core::default::Default::default(),
            s3_input_format: core::default::Default::default(),
            s3_uri: self.s3_uri,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockModelInvocationJob_InputDataConfig_S3InputDataConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockModelInvocationJob_InputDataConfig_S3InputDataConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockModelInvocationJob_InputDataConfig_S3InputDataConfigRef {
        BedrockModelInvocationJob_InputDataConfig_S3InputDataConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockModelInvocationJob_InputDataConfig_S3InputDataConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_bucket_owner` after provisioning.\n"]
    pub fn s3_bucket_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_owner", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_input_format` after provisioning.\n"]
    pub fn s3_input_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_input_format", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockModelInvocationJob_InputDataConfigDynamic {
    s3_input_data_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockModelInvocationJob_InputDataConfig_S3InputDataConfig>>,
}

#[derive(Serialize)]
pub struct BedrockModelInvocationJob_InputDataConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_input_data_config: std::option::Option<std::vec::Vec<BedrockModelInvocationJob_InputDataConfig_S3InputDataConfig>>,
    dynamic: BedrockModelInvocationJob_InputDataConfigDynamic,
}

impl BedrockModelInvocationJob_InputDataConfig {
    #[doc = "Set the field `s3_input_data_config`.\n"]
    pub fn set_s3_input_data_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockModelInvocationJob_InputDataConfig_S3InputDataConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_input_data_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_input_data_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockModelInvocationJob_InputDataConfig {
    type O = terrars::BlockAssignable<BedrockModelInvocationJob_InputDataConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockModelInvocationJob_InputDataConfig {}

impl BuildBedrockModelInvocationJob_InputDataConfig {
    pub fn build(self) -> BedrockModelInvocationJob_InputDataConfig {
        BedrockModelInvocationJob_InputDataConfig {
            s3_input_data_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockModelInvocationJob_InputDataConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockModelInvocationJob_InputDataConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockModelInvocationJob_InputDataConfigRef {
        BedrockModelInvocationJob_InputDataConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockModelInvocationJob_InputDataConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_input_data_config` after provisioning.\n"]
    pub fn s3_input_data_config(
        &self,
    ) -> terrars::ListRef<BedrockModelInvocationJob_InputDataConfig_S3InputDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_input_data_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockModelInvocationJob_OutputDataConfig_S3OutputDataConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_bucket_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_encryption_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_uri: terrars::PrimField<std::string::String>,
}

impl BedrockModelInvocationJob_OutputDataConfig_S3OutputDataConfig {
    #[doc = "Set the field `s3_bucket_owner`.\n"]
    pub fn set_s3_bucket_owner(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_bucket_owner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_encryption_key_id`.\n"]
    pub fn set_s3_encryption_key_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.s3_encryption_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockModelInvocationJob_OutputDataConfig_S3OutputDataConfig {
    type O = terrars::BlockAssignable<BedrockModelInvocationJob_OutputDataConfig_S3OutputDataConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockModelInvocationJob_OutputDataConfig_S3OutputDataConfig {
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildBedrockModelInvocationJob_OutputDataConfig_S3OutputDataConfig {
    pub fn build(self) -> BedrockModelInvocationJob_OutputDataConfig_S3OutputDataConfig {
        BedrockModelInvocationJob_OutputDataConfig_S3OutputDataConfig {
            s3_bucket_owner: core::default::Default::default(),
            s3_encryption_key_id: core::default::Default::default(),
            s3_uri: self.s3_uri,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockModelInvocationJob_OutputDataConfig_S3OutputDataConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockModelInvocationJob_OutputDataConfig_S3OutputDataConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockModelInvocationJob_OutputDataConfig_S3OutputDataConfigRef {
        BedrockModelInvocationJob_OutputDataConfig_S3OutputDataConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockModelInvocationJob_OutputDataConfig_S3OutputDataConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_bucket_owner` after provisioning.\n"]
    pub fn s3_bucket_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_owner", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_encryption_key_id` after provisioning.\n"]
    pub fn s3_encryption_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_encryption_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockModelInvocationJob_OutputDataConfigDynamic {
    s3_output_data_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockModelInvocationJob_OutputDataConfig_S3OutputDataConfig>>,
}

#[derive(Serialize)]
pub struct BedrockModelInvocationJob_OutputDataConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_output_data_config: std
    ::option
    ::Option<std::vec::Vec<BedrockModelInvocationJob_OutputDataConfig_S3OutputDataConfig>>,
    dynamic: BedrockModelInvocationJob_OutputDataConfigDynamic,
}

impl BedrockModelInvocationJob_OutputDataConfig {
    #[doc = "Set the field `s3_output_data_config`.\n"]
    pub fn set_s3_output_data_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockModelInvocationJob_OutputDataConfig_S3OutputDataConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_output_data_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_output_data_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockModelInvocationJob_OutputDataConfig {
    type O = terrars::BlockAssignable<BedrockModelInvocationJob_OutputDataConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockModelInvocationJob_OutputDataConfig {}

impl BuildBedrockModelInvocationJob_OutputDataConfig {
    pub fn build(self) -> BedrockModelInvocationJob_OutputDataConfig {
        BedrockModelInvocationJob_OutputDataConfig {
            s3_output_data_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockModelInvocationJob_OutputDataConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockModelInvocationJob_OutputDataConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockModelInvocationJob_OutputDataConfigRef {
        BedrockModelInvocationJob_OutputDataConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockModelInvocationJob_OutputDataConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_output_data_config` after provisioning.\n"]
    pub fn s3_output_data_config(
        &self,
    ) -> terrars::ListRef<BedrockModelInvocationJob_OutputDataConfig_S3OutputDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_output_data_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockModelInvocationJob_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockModelInvocationJob_Timeouts {
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

impl terrars::ToListMappable for BedrockModelInvocationJob_Timeouts {
    type O = terrars::BlockAssignable<BedrockModelInvocationJob_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockModelInvocationJob_Timeouts {}

impl BuildBedrockModelInvocationJob_Timeouts {
    pub fn build(self) -> BedrockModelInvocationJob_Timeouts {
        BedrockModelInvocationJob_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockModelInvocationJob_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockModelInvocationJob_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockModelInvocationJob_TimeoutsRef {
        BedrockModelInvocationJob_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockModelInvocationJob_TimeoutsRef {
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
pub struct BedrockModelInvocationJob_VpcConfig {
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BedrockModelInvocationJob_VpcConfig { }

impl terrars::ToListMappable for BedrockModelInvocationJob_VpcConfig {
    type O = terrars::BlockAssignable<BedrockModelInvocationJob_VpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockModelInvocationJob_VpcConfig {
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildBedrockModelInvocationJob_VpcConfig {
    pub fn build(self) -> BedrockModelInvocationJob_VpcConfig {
        BedrockModelInvocationJob_VpcConfig {
            security_group_ids: self.security_group_ids,
            subnet_ids: self.subnet_ids,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockModelInvocationJob_VpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockModelInvocationJob_VpcConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockModelInvocationJob_VpcConfigRef {
        BedrockModelInvocationJob_VpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockModelInvocationJob_VpcConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}
