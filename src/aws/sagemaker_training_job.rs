#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerTrainingJobData_ {
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
    delete_model_packages_on_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_vpc_enis_on_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_inter_container_traffic_encryption: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_managed_spot_training: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_network_isolation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hyper_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    training_job_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm_specification: std::option::Option<std::vec::Vec<SagemakerTrainingJob_AlgorithmSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    checkpoint_config: std::option::Option<std::vec::Vec<SagemakerTrainingJob_CheckpointConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    debug_hook_config: std::option::Option<std::vec::Vec<SagemakerTrainingJob_DebugHookConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    debug_rule_configurations: std::option::Option<std::vec::Vec<SagemakerTrainingJob_DebugRuleConfigurations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    experiment_config: std::option::Option<std::vec::Vec<SagemakerTrainingJob_ExperimentConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    infra_check_config: std::option::Option<std::vec::Vec<SagemakerTrainingJob_InfraCheckConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_data_config: std::option::Option<std::vec::Vec<SagemakerTrainingJob_InputDataConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mlflow_config: std::option::Option<std::vec::Vec<SagemakerTrainingJob_MlflowConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_package_config: std::option::Option<std::vec::Vec<SagemakerTrainingJob_ModelPackageConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_data_config: std::option::Option<std::vec::Vec<SagemakerTrainingJob_OutputDataConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profiler_config: std::option::Option<std::vec::Vec<SagemakerTrainingJob_ProfilerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profiler_rule_configurations: std::option::Option<std::vec::Vec<SagemakerTrainingJob_ProfilerRuleConfigurations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_debug_config: std::option::Option<std::vec::Vec<SagemakerTrainingJob_RemoteDebugConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_config: std::option::Option<std::vec::Vec<SagemakerTrainingJob_ResourceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_strategy: std::option::Option<std::vec::Vec<SagemakerTrainingJob_RetryStrategy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serverless_job_config: std::option::Option<std::vec::Vec<SagemakerTrainingJob_ServerlessJobConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_chaining_config: std::option::Option<std::vec::Vec<SagemakerTrainingJob_SessionChainingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stopping_condition: std::option::Option<std::vec::Vec<SagemakerTrainingJob_StoppingCondition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tensor_board_output_config: std::option::Option<std::vec::Vec<SagemakerTrainingJob_TensorBoardOutputConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SagemakerTrainingJob_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<SagemakerTrainingJob_VpcConfig>>,
    dynamic: SagemakerTrainingJobDynamic,
}

struct SagemakerTrainingJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerTrainingJobData_>,
}

#[derive(Clone)]
pub struct SagemakerTrainingJob(std::rc::Rc<SagemakerTrainingJob_>);

impl SagemakerTrainingJob {
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
        "Set the field `delete_model_packages_on_destroy`.\nWhether to delete model packages in the configured model package group when destroying the training job."]
    pub fn set_delete_model_packages_on_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_model_packages_on_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete_vpc_enis_on_destroy`.\nWhether to delete detached VPC ENIs that SageMaker may leave behind when destroying the training job."]
    pub fn set_delete_vpc_enis_on_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_vpc_enis_on_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_inter_container_traffic_encryption`.\n"]
    pub fn set_enable_inter_container_traffic_encryption(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().enable_inter_container_traffic_encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_managed_spot_training`.\n"]
    pub fn set_enable_managed_spot_training(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_managed_spot_training = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_network_isolation`.\n"]
    pub fn set_enable_network_isolation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_network_isolation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hyper_parameters`.\n"]
    pub fn set_hyper_parameters(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().hyper_parameters = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `algorithm_specification`.\n"]
    pub fn set_algorithm_specification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_AlgorithmSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().algorithm_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.algorithm_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `checkpoint_config`.\n"]
    pub fn set_checkpoint_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_CheckpointConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().checkpoint_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.checkpoint_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `debug_hook_config`.\n"]
    pub fn set_debug_hook_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_DebugHookConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().debug_hook_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.debug_hook_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `debug_rule_configurations`.\n"]
    pub fn set_debug_rule_configurations(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_DebugRuleConfigurations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().debug_rule_configurations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.debug_rule_configurations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `experiment_config`.\n"]
    pub fn set_experiment_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_ExperimentConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().experiment_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.experiment_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `infra_check_config`.\n"]
    pub fn set_infra_check_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_InfraCheckConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().infra_check_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.infra_check_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_data_config`.\n"]
    pub fn set_input_data_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_InputDataConfig>>,
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

    #[doc = "Set the field `mlflow_config`.\n"]
    pub fn set_mlflow_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_MlflowConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mlflow_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mlflow_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `model_package_config`.\n"]
    pub fn set_model_package_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_ModelPackageConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().model_package_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.model_package_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_data_config`.\n"]
    pub fn set_output_data_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_OutputDataConfig>>,
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

    #[doc = "Set the field `profiler_config`.\n"]
    pub fn set_profiler_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_ProfilerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().profiler_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.profiler_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `profiler_rule_configurations`.\n"]
    pub fn set_profiler_rule_configurations(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_ProfilerRuleConfigurations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().profiler_rule_configurations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.profiler_rule_configurations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote_debug_config`.\n"]
    pub fn set_remote_debug_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_RemoteDebugConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().remote_debug_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.remote_debug_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_config`.\n"]
    pub fn set_resource_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_ResourceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retry_strategy`.\n"]
    pub fn set_retry_strategy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_RetryStrategy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retry_strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retry_strategy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `serverless_job_config`.\n"]
    pub fn set_serverless_job_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_ServerlessJobConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().serverless_job_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.serverless_job_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `session_chaining_config`.\n"]
    pub fn set_session_chaining_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_SessionChainingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().session_chaining_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.session_chaining_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stopping_condition`.\n"]
    pub fn set_stopping_condition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_StoppingCondition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stopping_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stopping_condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tensor_board_output_config`.\n"]
    pub fn set_tensor_board_output_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_TensorBoardOutputConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tensor_board_output_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tensor_board_output_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<SagemakerTrainingJob_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_VpcConfig>>,
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_model_packages_on_destroy` after provisioning.\nWhether to delete model packages in the configured model package group when destroying the training job."]
    pub fn delete_model_packages_on_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_model_packages_on_destroy", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `delete_vpc_enis_on_destroy` after provisioning.\nWhether to delete detached VPC ENIs that SageMaker may leave behind when destroying the training job."]
    pub fn delete_vpc_enis_on_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_vpc_enis_on_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_inter_container_traffic_encryption` after provisioning.\n"]
    pub fn enable_inter_container_traffic_encryption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_inter_container_traffic_encryption", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_managed_spot_training` after provisioning.\n"]
    pub fn enable_managed_spot_training(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_managed_spot_training", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_network_isolation` after provisioning.\n"]
    pub fn enable_network_isolation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_network_isolation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hyper_parameters` after provisioning.\n"]
    pub fn hyper_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.hyper_parameters", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `training_job_name` after provisioning.\n"]
    pub fn training_job_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.training_job_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `algorithm_specification` after provisioning.\n"]
    pub fn algorithm_specification(&self) -> terrars::ListRef<SagemakerTrainingJob_AlgorithmSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.algorithm_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `checkpoint_config` after provisioning.\n"]
    pub fn checkpoint_config(&self) -> terrars::ListRef<SagemakerTrainingJob_CheckpointConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.checkpoint_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `debug_hook_config` after provisioning.\n"]
    pub fn debug_hook_config(&self) -> terrars::ListRef<SagemakerTrainingJob_DebugHookConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.debug_hook_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `debug_rule_configurations` after provisioning.\n"]
    pub fn debug_rule_configurations(&self) -> terrars::ListRef<SagemakerTrainingJob_DebugRuleConfigurationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.debug_rule_configurations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `experiment_config` after provisioning.\n"]
    pub fn experiment_config(&self) -> terrars::ListRef<SagemakerTrainingJob_ExperimentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.experiment_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `infra_check_config` after provisioning.\n"]
    pub fn infra_check_config(&self) -> terrars::ListRef<SagemakerTrainingJob_InfraCheckConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.infra_check_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_data_config` after provisioning.\n"]
    pub fn input_data_config(&self) -> terrars::ListRef<SagemakerTrainingJob_InputDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mlflow_config` after provisioning.\n"]
    pub fn mlflow_config(&self) -> terrars::ListRef<SagemakerTrainingJob_MlflowConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mlflow_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_package_config` after provisioning.\n"]
    pub fn model_package_config(&self) -> terrars::ListRef<SagemakerTrainingJob_ModelPackageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_package_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_data_config` after provisioning.\n"]
    pub fn output_data_config(&self) -> terrars::ListRef<SagemakerTrainingJob_OutputDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profiler_config` after provisioning.\n"]
    pub fn profiler_config(&self) -> terrars::ListRef<SagemakerTrainingJob_ProfilerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.profiler_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profiler_rule_configurations` after provisioning.\n"]
    pub fn profiler_rule_configurations(&self) -> terrars::ListRef<SagemakerTrainingJob_ProfilerRuleConfigurationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.profiler_rule_configurations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_debug_config` after provisioning.\n"]
    pub fn remote_debug_config(&self) -> terrars::ListRef<SagemakerTrainingJob_RemoteDebugConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_debug_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_config` after provisioning.\n"]
    pub fn resource_config(&self) -> terrars::ListRef<SagemakerTrainingJob_ResourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_strategy` after provisioning.\n"]
    pub fn retry_strategy(&self) -> terrars::ListRef<SagemakerTrainingJob_RetryStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `serverless_job_config` after provisioning.\n"]
    pub fn serverless_job_config(&self) -> terrars::ListRef<SagemakerTrainingJob_ServerlessJobConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.serverless_job_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `session_chaining_config` after provisioning.\n"]
    pub fn session_chaining_config(&self) -> terrars::ListRef<SagemakerTrainingJob_SessionChainingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.session_chaining_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stopping_condition` after provisioning.\n"]
    pub fn stopping_condition(&self) -> terrars::ListRef<SagemakerTrainingJob_StoppingConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stopping_condition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tensor_board_output_config` after provisioning.\n"]
    pub fn tensor_board_output_config(&self) -> terrars::ListRef<SagemakerTrainingJob_TensorBoardOutputConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tensor_board_output_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SagemakerTrainingJob_TimeoutsRef {
        SagemakerTrainingJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<SagemakerTrainingJob_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

impl terrars::Referable for SagemakerTrainingJob {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerTrainingJob { }

impl terrars::ToListMappable for SagemakerTrainingJob {
    type O = terrars::ListRef<SagemakerTrainingJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerTrainingJob_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_training_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerTrainingJob {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub training_job_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerTrainingJob {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerTrainingJob {
        let out = SagemakerTrainingJob(std::rc::Rc::new(SagemakerTrainingJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerTrainingJobData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                delete_model_packages_on_destroy: core::default::Default::default(),
                delete_vpc_enis_on_destroy: core::default::Default::default(),
                enable_inter_container_traffic_encryption: core::default::Default::default(),
                enable_managed_spot_training: core::default::Default::default(),
                enable_network_isolation: core::default::Default::default(),
                environment: core::default::Default::default(),
                hyper_parameters: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                training_job_name: self.training_job_name,
                algorithm_specification: core::default::Default::default(),
                checkpoint_config: core::default::Default::default(),
                debug_hook_config: core::default::Default::default(),
                debug_rule_configurations: core::default::Default::default(),
                experiment_config: core::default::Default::default(),
                infra_check_config: core::default::Default::default(),
                input_data_config: core::default::Default::default(),
                mlflow_config: core::default::Default::default(),
                model_package_config: core::default::Default::default(),
                output_data_config: core::default::Default::default(),
                profiler_config: core::default::Default::default(),
                profiler_rule_configurations: core::default::Default::default(),
                remote_debug_config: core::default::Default::default(),
                resource_config: core::default::Default::default(),
                retry_strategy: core::default::Default::default(),
                serverless_job_config: core::default::Default::default(),
                session_chaining_config: core::default::Default::default(),
                stopping_condition: core::default::Default::default(),
                tensor_board_output_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerTrainingJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerTrainingJobRef {
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

    #[doc =
        "Get a reference to the value of field `delete_model_packages_on_destroy` after provisioning.\nWhether to delete model packages in the configured model package group when destroying the training job."]
    pub fn delete_model_packages_on_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_model_packages_on_destroy", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `delete_vpc_enis_on_destroy` after provisioning.\nWhether to delete detached VPC ENIs that SageMaker may leave behind when destroying the training job."]
    pub fn delete_vpc_enis_on_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_vpc_enis_on_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_inter_container_traffic_encryption` after provisioning.\n"]
    pub fn enable_inter_container_traffic_encryption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_inter_container_traffic_encryption", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_managed_spot_training` after provisioning.\n"]
    pub fn enable_managed_spot_training(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_managed_spot_training", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_network_isolation` after provisioning.\n"]
    pub fn enable_network_isolation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_network_isolation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hyper_parameters` after provisioning.\n"]
    pub fn hyper_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.hyper_parameters", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `training_job_name` after provisioning.\n"]
    pub fn training_job_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.training_job_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `algorithm_specification` after provisioning.\n"]
    pub fn algorithm_specification(&self) -> terrars::ListRef<SagemakerTrainingJob_AlgorithmSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.algorithm_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `checkpoint_config` after provisioning.\n"]
    pub fn checkpoint_config(&self) -> terrars::ListRef<SagemakerTrainingJob_CheckpointConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.checkpoint_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `debug_hook_config` after provisioning.\n"]
    pub fn debug_hook_config(&self) -> terrars::ListRef<SagemakerTrainingJob_DebugHookConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.debug_hook_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `debug_rule_configurations` after provisioning.\n"]
    pub fn debug_rule_configurations(&self) -> terrars::ListRef<SagemakerTrainingJob_DebugRuleConfigurationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.debug_rule_configurations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `experiment_config` after provisioning.\n"]
    pub fn experiment_config(&self) -> terrars::ListRef<SagemakerTrainingJob_ExperimentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.experiment_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `infra_check_config` after provisioning.\n"]
    pub fn infra_check_config(&self) -> terrars::ListRef<SagemakerTrainingJob_InfraCheckConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.infra_check_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_data_config` after provisioning.\n"]
    pub fn input_data_config(&self) -> terrars::ListRef<SagemakerTrainingJob_InputDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mlflow_config` after provisioning.\n"]
    pub fn mlflow_config(&self) -> terrars::ListRef<SagemakerTrainingJob_MlflowConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mlflow_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_package_config` after provisioning.\n"]
    pub fn model_package_config(&self) -> terrars::ListRef<SagemakerTrainingJob_ModelPackageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_package_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_data_config` after provisioning.\n"]
    pub fn output_data_config(&self) -> terrars::ListRef<SagemakerTrainingJob_OutputDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profiler_config` after provisioning.\n"]
    pub fn profiler_config(&self) -> terrars::ListRef<SagemakerTrainingJob_ProfilerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.profiler_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profiler_rule_configurations` after provisioning.\n"]
    pub fn profiler_rule_configurations(&self) -> terrars::ListRef<SagemakerTrainingJob_ProfilerRuleConfigurationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.profiler_rule_configurations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_debug_config` after provisioning.\n"]
    pub fn remote_debug_config(&self) -> terrars::ListRef<SagemakerTrainingJob_RemoteDebugConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_debug_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_config` after provisioning.\n"]
    pub fn resource_config(&self) -> terrars::ListRef<SagemakerTrainingJob_ResourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_strategy` after provisioning.\n"]
    pub fn retry_strategy(&self) -> terrars::ListRef<SagemakerTrainingJob_RetryStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `serverless_job_config` after provisioning.\n"]
    pub fn serverless_job_config(&self) -> terrars::ListRef<SagemakerTrainingJob_ServerlessJobConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.serverless_job_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `session_chaining_config` after provisioning.\n"]
    pub fn session_chaining_config(&self) -> terrars::ListRef<SagemakerTrainingJob_SessionChainingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.session_chaining_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stopping_condition` after provisioning.\n"]
    pub fn stopping_condition(&self) -> terrars::ListRef<SagemakerTrainingJob_StoppingConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stopping_condition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tensor_board_output_config` after provisioning.\n"]
    pub fn tensor_board_output_config(&self) -> terrars::ListRef<SagemakerTrainingJob_TensorBoardOutputConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tensor_board_output_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SagemakerTrainingJob_TimeoutsRef {
        SagemakerTrainingJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<SagemakerTrainingJob_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SagemakerTrainingJobDynamic {
    algorithm_specification: std::option::Option<terrars::DynamicBlock<SagemakerTrainingJob_AlgorithmSpecification>>,
    checkpoint_config: std::option::Option<terrars::DynamicBlock<SagemakerTrainingJob_CheckpointConfig>>,
    debug_hook_config: std::option::Option<terrars::DynamicBlock<SagemakerTrainingJob_DebugHookConfig>>,
    debug_rule_configurations: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerTrainingJob_DebugRuleConfigurations>>,
    experiment_config: std::option::Option<terrars::DynamicBlock<SagemakerTrainingJob_ExperimentConfig>>,
    infra_check_config: std::option::Option<terrars::DynamicBlock<SagemakerTrainingJob_InfraCheckConfig>>,
    input_data_config: std::option::Option<terrars::DynamicBlock<SagemakerTrainingJob_InputDataConfig>>,
    mlflow_config: std::option::Option<terrars::DynamicBlock<SagemakerTrainingJob_MlflowConfig>>,
    model_package_config: std::option::Option<terrars::DynamicBlock<SagemakerTrainingJob_ModelPackageConfig>>,
    output_data_config: std::option::Option<terrars::DynamicBlock<SagemakerTrainingJob_OutputDataConfig>>,
    profiler_config: std::option::Option<terrars::DynamicBlock<SagemakerTrainingJob_ProfilerConfig>>,
    profiler_rule_configurations: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerTrainingJob_ProfilerRuleConfigurations>>,
    remote_debug_config: std::option::Option<terrars::DynamicBlock<SagemakerTrainingJob_RemoteDebugConfig>>,
    resource_config: std::option::Option<terrars::DynamicBlock<SagemakerTrainingJob_ResourceConfig>>,
    retry_strategy: std::option::Option<terrars::DynamicBlock<SagemakerTrainingJob_RetryStrategy>>,
    serverless_job_config: std::option::Option<terrars::DynamicBlock<SagemakerTrainingJob_ServerlessJobConfig>>,
    session_chaining_config: std::option::Option<terrars::DynamicBlock<SagemakerTrainingJob_SessionChainingConfig>>,
    stopping_condition: std::option::Option<terrars::DynamicBlock<SagemakerTrainingJob_StoppingCondition>>,
    tensor_board_output_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerTrainingJob_TensorBoardOutputConfig>>,
    vpc_config: std::option::Option<terrars::DynamicBlock<SagemakerTrainingJob_VpcConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_AlgorithmSpecification_MetricDefinitions {
    name: terrars::PrimField<std::string::String>,
    regex: terrars::PrimField<std::string::String>,
}

impl SagemakerTrainingJob_AlgorithmSpecification_MetricDefinitions { }

impl terrars::ToListMappable for SagemakerTrainingJob_AlgorithmSpecification_MetricDefinitions {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_AlgorithmSpecification_MetricDefinitions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_AlgorithmSpecification_MetricDefinitions {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub regex: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerTrainingJob_AlgorithmSpecification_MetricDefinitions {
    pub fn build(self) -> SagemakerTrainingJob_AlgorithmSpecification_MetricDefinitions {
        SagemakerTrainingJob_AlgorithmSpecification_MetricDefinitions {
            name: self.name,
            regex: self.regex,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_AlgorithmSpecification_MetricDefinitionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_AlgorithmSpecification_MetricDefinitionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerTrainingJob_AlgorithmSpecification_MetricDefinitionsRef {
        SagemakerTrainingJob_AlgorithmSpecification_MetricDefinitionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_AlgorithmSpecification_MetricDefinitionsRef {
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

#[derive(Serialize)]
pub struct SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig_TrainingRepositoryAuthConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    training_repository_credentials_provider_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig_TrainingRepositoryAuthConfig {
    #[doc = "Set the field `training_repository_credentials_provider_arn`.\n"]
    pub fn set_training_repository_credentials_provider_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.training_repository_credentials_provider_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig_TrainingRepositoryAuthConfig {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig_TrainingRepositoryAuthConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig_TrainingRepositoryAuthConfig {}

impl BuildSagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig_TrainingRepositoryAuthConfig {
    pub fn build(
        self,
    ) -> SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig_TrainingRepositoryAuthConfig {
        SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig_TrainingRepositoryAuthConfig {
            training_repository_credentials_provider_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig_TrainingRepositoryAuthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig_TrainingRepositoryAuthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig_TrainingRepositoryAuthConfigRef {
        SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig_TrainingRepositoryAuthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig_TrainingRepositoryAuthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `training_repository_credentials_provider_arn` after provisioning.\n"]
    pub fn training_repository_credentials_provider_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.training_repository_credentials_provider_arn", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfigDynamic {
    training_repository_auth_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig_TrainingRepositoryAuthConfig>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    training_repository_access_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    training_repository_auth_config: std
    ::option
    ::Option<
        std::vec::Vec<SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig_TrainingRepositoryAuthConfig>,
    >,
    dynamic: SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfigDynamic,
}

impl SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig {
    #[doc = "Set the field `training_repository_access_mode`.\n"]
    pub fn set_training_repository_access_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.training_repository_access_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `training_repository_auth_config`.\n"]
    pub fn set_training_repository_auth_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig_TrainingRepositoryAuthConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.training_repository_auth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.training_repository_auth_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig {}

impl BuildSagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig {
    pub fn build(self) -> SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig {
        SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig {
            training_repository_access_mode: core::default::Default::default(),
            training_repository_auth_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfigRef {
        SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `training_repository_access_mode` after provisioning.\n"]
    pub fn training_repository_access_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.training_repository_access_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `training_repository_auth_config` after provisioning.\n"]
    pub fn training_repository_auth_config(
        &self,
    ) -> terrars
    ::ListRef<SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig_TrainingRepositoryAuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.training_repository_auth_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerTrainingJob_AlgorithmSpecificationDynamic {
    metric_definitions: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerTrainingJob_AlgorithmSpecification_MetricDefinitions>>,
    training_image_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_AlgorithmSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_arguments: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_entrypoint: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_sagemaker_metrics_time_series: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    training_image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    training_input_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_definitions: std::option::Option<std::vec::Vec<SagemakerTrainingJob_AlgorithmSpecification_MetricDefinitions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    training_image_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig>>,
    dynamic: SagemakerTrainingJob_AlgorithmSpecificationDynamic,
}

impl SagemakerTrainingJob_AlgorithmSpecification {
    #[doc =
        "Set the field `algorithm_name`.\nName or ARN of a SageMaker algorithm resource. Exactly one of `algorithm_name` or `training_image` must be set."]
    pub fn set_algorithm_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.algorithm_name = std::option::Option::Some(v.into());
        self
    }

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

    #[doc =
        "Set the field `enable_sagemaker_metrics_time_series`.\nWhether SageMaker AI should publish time-series metrics. SageMaker enables this automatically for built-in algorithms, supported prebuilt images, and jobs with explicit `metric_definitions`."]
    pub fn set_enable_sagemaker_metrics_time_series(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_sagemaker_metrics_time_series = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `training_image`.\nRegistry path of the training image. Exactly one of `algorithm_name` or `training_image` must be set. Use `metric_definitions` only when you need to extract custom metrics from your own training container logs."]
    pub fn set_training_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.training_image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `training_input_mode`.\n"]
    pub fn set_training_input_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.training_input_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric_definitions`.\n"]
    pub fn set_metric_definitions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerTrainingJob_AlgorithmSpecification_MetricDefinitions>>,
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

    #[doc = "Set the field `training_image_config`.\n"]
    pub fn set_training_image_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.training_image_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.training_image_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerTrainingJob_AlgorithmSpecification {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_AlgorithmSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_AlgorithmSpecification {}

impl BuildSagemakerTrainingJob_AlgorithmSpecification {
    pub fn build(self) -> SagemakerTrainingJob_AlgorithmSpecification {
        SagemakerTrainingJob_AlgorithmSpecification {
            algorithm_name: core::default::Default::default(),
            container_arguments: core::default::Default::default(),
            container_entrypoint: core::default::Default::default(),
            enable_sagemaker_metrics_time_series: core::default::Default::default(),
            training_image: core::default::Default::default(),
            training_input_mode: core::default::Default::default(),
            metric_definitions: core::default::Default::default(),
            training_image_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_AlgorithmSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_AlgorithmSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerTrainingJob_AlgorithmSpecificationRef {
        SagemakerTrainingJob_AlgorithmSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_AlgorithmSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `algorithm_name` after provisioning.\nName or ARN of a SageMaker algorithm resource. Exactly one of `algorithm_name` or `training_image` must be set."]
    pub fn algorithm_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm_name", self.base))
    }

    #[doc = "Get a reference to the value of field `container_arguments` after provisioning.\n"]
    pub fn container_arguments(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_arguments", self.base))
    }

    #[doc = "Get a reference to the value of field `container_entrypoint` after provisioning.\n"]
    pub fn container_entrypoint(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_entrypoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_sagemaker_metrics_time_series` after provisioning.\nWhether SageMaker AI should publish time-series metrics. SageMaker enables this automatically for built-in algorithms, supported prebuilt images, and jobs with explicit `metric_definitions`."]
    pub fn enable_sagemaker_metrics_time_series(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_sagemaker_metrics_time_series", self.base))
    }

    #[doc =
        "Get a reference to the value of field `training_image` after provisioning.\nRegistry path of the training image. Exactly one of `algorithm_name` or `training_image` must be set. Use `metric_definitions` only when you need to extract custom metrics from your own training container logs."]
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
    ) -> terrars::ListRef<SagemakerTrainingJob_AlgorithmSpecification_MetricDefinitionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_definitions", self.base))
    }

    #[doc = "Get a reference to the value of field `training_image_config` after provisioning.\n"]
    pub fn training_image_config(
        &self,
    ) -> terrars::ListRef<SagemakerTrainingJob_AlgorithmSpecification_TrainingImageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.training_image_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_CheckpointConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_path: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_uri: terrars::PrimField<std::string::String>,
}

impl SagemakerTrainingJob_CheckpointConfig {
    #[doc = "Set the field `local_path`.\n"]
    pub fn set_local_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerTrainingJob_CheckpointConfig {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_CheckpointConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_CheckpointConfig {
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerTrainingJob_CheckpointConfig {
    pub fn build(self) -> SagemakerTrainingJob_CheckpointConfig {
        SagemakerTrainingJob_CheckpointConfig {
            local_path: core::default::Default::default(),
            s3_uri: self.s3_uri,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_CheckpointConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_CheckpointConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerTrainingJob_CheckpointConfigRef {
        SagemakerTrainingJob_CheckpointConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_CheckpointConfigRef {
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

#[derive(Serialize)]
pub struct SagemakerTrainingJob_DebugHookConfig_CollectionConfigurations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collection_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collection_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl SagemakerTrainingJob_DebugHookConfig_CollectionConfigurations {
    #[doc = "Set the field `collection_name`.\n"]
    pub fn set_collection_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.collection_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `collection_parameters`.\n"]
    pub fn set_collection_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.collection_parameters = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerTrainingJob_DebugHookConfig_CollectionConfigurations {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_DebugHookConfig_CollectionConfigurations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_DebugHookConfig_CollectionConfigurations {}

impl BuildSagemakerTrainingJob_DebugHookConfig_CollectionConfigurations {
    pub fn build(self) -> SagemakerTrainingJob_DebugHookConfig_CollectionConfigurations {
        SagemakerTrainingJob_DebugHookConfig_CollectionConfigurations {
            collection_name: core::default::Default::default(),
            collection_parameters: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_DebugHookConfig_CollectionConfigurationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_DebugHookConfig_CollectionConfigurationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerTrainingJob_DebugHookConfig_CollectionConfigurationsRef {
        SagemakerTrainingJob_DebugHookConfig_CollectionConfigurationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_DebugHookConfig_CollectionConfigurationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `collection_name` after provisioning.\n"]
    pub fn collection_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_name", self.base))
    }

    #[doc = "Get a reference to the value of field `collection_parameters` after provisioning.\n"]
    pub fn collection_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.collection_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerTrainingJob_DebugHookConfigDynamic {
    collection_configurations: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerTrainingJob_DebugHookConfig_CollectionConfigurations>>,
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_DebugHookConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hook_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_path: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_output_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collection_configurations: std
    ::option
    ::Option<std::vec::Vec<SagemakerTrainingJob_DebugHookConfig_CollectionConfigurations>>,
    dynamic: SagemakerTrainingJob_DebugHookConfigDynamic,
}

impl SagemakerTrainingJob_DebugHookConfig {
    #[doc = "Set the field `hook_parameters`.\n"]
    pub fn set_hook_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.hook_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_path`.\n"]
    pub fn set_local_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `collection_configurations`.\n"]
    pub fn set_collection_configurations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerTrainingJob_DebugHookConfig_CollectionConfigurations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.collection_configurations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.collection_configurations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerTrainingJob_DebugHookConfig {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_DebugHookConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_DebugHookConfig {
    #[doc = ""]
    pub s3_output_path: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerTrainingJob_DebugHookConfig {
    pub fn build(self) -> SagemakerTrainingJob_DebugHookConfig {
        SagemakerTrainingJob_DebugHookConfig {
            hook_parameters: core::default::Default::default(),
            local_path: core::default::Default::default(),
            s3_output_path: self.s3_output_path,
            collection_configurations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_DebugHookConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_DebugHookConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerTrainingJob_DebugHookConfigRef {
        SagemakerTrainingJob_DebugHookConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_DebugHookConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hook_parameters` after provisioning.\n"]
    pub fn hook_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.hook_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `local_path` after provisioning.\n"]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_path` after provisioning.\n"]
    pub fn s3_output_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_output_path", self.base))
    }

    #[doc = "Get a reference to the value of field `collection_configurations` after provisioning.\n"]
    pub fn collection_configurations(
        &self,
    ) -> terrars::ListRef<SagemakerTrainingJob_DebugHookConfig_CollectionConfigurationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.collection_configurations", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_DebugRuleConfigurations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_path: std::option::Option<terrars::PrimField<std::string::String>>,
    rule_configuration_name: terrars::PrimField<std::string::String>,
    rule_evaluator_image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_output_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size_in_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerTrainingJob_DebugRuleConfigurations {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_path`.\n"]
    pub fn set_local_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_parameters`.\n"]
    pub fn set_rule_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.rule_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_output_path`.\n"]
    pub fn set_s3_output_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_output_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size_in_gb`.\n"]
    pub fn set_volume_size_in_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_size_in_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerTrainingJob_DebugRuleConfigurations {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_DebugRuleConfigurations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_DebugRuleConfigurations {
    #[doc = ""]
    pub rule_configuration_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule_evaluator_image: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerTrainingJob_DebugRuleConfigurations {
    pub fn build(self) -> SagemakerTrainingJob_DebugRuleConfigurations {
        SagemakerTrainingJob_DebugRuleConfigurations {
            instance_type: core::default::Default::default(),
            local_path: core::default::Default::default(),
            rule_configuration_name: self.rule_configuration_name,
            rule_evaluator_image: self.rule_evaluator_image,
            rule_parameters: core::default::Default::default(),
            s3_output_path: core::default::Default::default(),
            volume_size_in_gb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_DebugRuleConfigurationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_DebugRuleConfigurationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerTrainingJob_DebugRuleConfigurationsRef {
        SagemakerTrainingJob_DebugRuleConfigurationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_DebugRuleConfigurationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `local_path` after provisioning.\n"]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_configuration_name` after provisioning.\n"]
    pub fn rule_configuration_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_configuration_name", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_evaluator_image` after provisioning.\n"]
    pub fn rule_evaluator_image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_evaluator_image", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_parameters` after provisioning.\n"]
    pub fn rule_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.rule_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_path` after provisioning.\n"]
    pub fn s3_output_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_output_path", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size_in_gb` after provisioning.\n"]
    pub fn volume_size_in_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_size_in_gb", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_ExperimentConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    experiment_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trial_component_display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trial_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerTrainingJob_ExperimentConfig {
    #[doc = "Set the field `experiment_name`.\n"]
    pub fn set_experiment_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.experiment_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_name`.\n"]
    pub fn set_run_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.run_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trial_component_display_name`.\n"]
    pub fn set_trial_component_display_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.trial_component_display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trial_name`.\n"]
    pub fn set_trial_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.trial_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerTrainingJob_ExperimentConfig {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_ExperimentConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_ExperimentConfig {}

impl BuildSagemakerTrainingJob_ExperimentConfig {
    pub fn build(self) -> SagemakerTrainingJob_ExperimentConfig {
        SagemakerTrainingJob_ExperimentConfig {
            experiment_name: core::default::Default::default(),
            run_name: core::default::Default::default(),
            trial_component_display_name: core::default::Default::default(),
            trial_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_ExperimentConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_ExperimentConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerTrainingJob_ExperimentConfigRef {
        SagemakerTrainingJob_ExperimentConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_ExperimentConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `experiment_name` after provisioning.\n"]
    pub fn experiment_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.experiment_name", self.base))
    }

    #[doc = "Get a reference to the value of field `run_name` after provisioning.\n"]
    pub fn run_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_name", self.base))
    }

    #[doc = "Get a reference to the value of field `trial_component_display_name` after provisioning.\n"]
    pub fn trial_component_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trial_component_display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `trial_name` after provisioning.\n"]
    pub fn trial_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trial_name", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_InfraCheckConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_infra_check: std::option::Option<terrars::PrimField<bool>>,
}

impl SagemakerTrainingJob_InfraCheckConfig {
    #[doc = "Set the field `enable_infra_check`.\n"]
    pub fn set_enable_infra_check(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_infra_check = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerTrainingJob_InfraCheckConfig {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_InfraCheckConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_InfraCheckConfig {}

impl BuildSagemakerTrainingJob_InfraCheckConfig {
    pub fn build(self) -> SagemakerTrainingJob_InfraCheckConfig {
        SagemakerTrainingJob_InfraCheckConfig { enable_infra_check: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_InfraCheckConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_InfraCheckConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerTrainingJob_InfraCheckConfigRef {
        SagemakerTrainingJob_InfraCheckConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_InfraCheckConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_infra_check` after provisioning.\n"]
    pub fn enable_infra_check(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_infra_check", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_InputDataConfig_DataSource_FileSystemDataSource {
    directory_path: terrars::PrimField<std::string::String>,
    file_system_access_mode: terrars::PrimField<std::string::String>,
    file_system_id: terrars::PrimField<std::string::String>,
    file_system_type: terrars::PrimField<std::string::String>,
}

impl SagemakerTrainingJob_InputDataConfig_DataSource_FileSystemDataSource { }

impl terrars::ToListMappable for SagemakerTrainingJob_InputDataConfig_DataSource_FileSystemDataSource {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_InputDataConfig_DataSource_FileSystemDataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_InputDataConfig_DataSource_FileSystemDataSource {
    #[doc = ""]
    pub directory_path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub file_system_access_mode: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub file_system_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub file_system_type: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerTrainingJob_InputDataConfig_DataSource_FileSystemDataSource {
    pub fn build(self) -> SagemakerTrainingJob_InputDataConfig_DataSource_FileSystemDataSource {
        SagemakerTrainingJob_InputDataConfig_DataSource_FileSystemDataSource {
            directory_path: self.directory_path,
            file_system_access_mode: self.file_system_access_mode,
            file_system_id: self.file_system_id,
            file_system_type: self.file_system_type,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_InputDataConfig_DataSource_FileSystemDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_InputDataConfig_DataSource_FileSystemDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerTrainingJob_InputDataConfig_DataSource_FileSystemDataSourceRef {
        SagemakerTrainingJob_InputDataConfig_DataSource_FileSystemDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_InputDataConfig_DataSource_FileSystemDataSourceRef {
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
pub struct SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_HubAccessConfig {
    hub_content_arn: terrars::PrimField<std::string::String>,
}

impl SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_HubAccessConfig { }

impl terrars::ToListMappable for SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_HubAccessConfig {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_HubAccessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_HubAccessConfig {
    #[doc = ""]
    pub hub_content_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_HubAccessConfig {
    pub fn build(self) -> SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_HubAccessConfig {
        SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_HubAccessConfig {
            hub_content_arn: self.hub_content_arn,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_HubAccessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_HubAccessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_HubAccessConfigRef {
        SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_HubAccessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_HubAccessConfigRef {
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
pub struct SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig {
    accept_eula: terrars::PrimField<bool>,
}

impl SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig { }

impl terrars::ToListMappable for SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig {
    type O =
        terrars::BlockAssignable<SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig {
    #[doc = ""]
    pub accept_eula: terrars::PrimField<bool>,
}

impl BuildSagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig {
    pub fn build(self) -> SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig {
        SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig {
            accept_eula: self.accept_eula,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_ModelAccessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_ModelAccessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_ModelAccessConfigRef {
        SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_ModelAccessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_ModelAccessConfigRef {
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
struct SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSourceDynamic {
    hub_access_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_HubAccessConfig>>,
    model_access_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_group_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_data_distribution_type: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_data_type: terrars::PrimField<std::string::String>,
    s3_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hub_access_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_HubAccessConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_access_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig>>,
    dynamic: SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSourceDynamic,
}

impl SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource {
    #[doc = "Set the field `attribute_names`.\n"]
    pub fn set_attribute_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.attribute_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_group_names`.\n"]
    pub fn set_instance_group_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
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
                            SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_HubAccessConfig,
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
                            SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_ModelAccessConfig,
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

impl terrars::ToListMappable for SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource {
    #[doc = ""]
    pub s3_data_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource {
    pub fn build(self) -> SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource {
        SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource {
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
pub struct SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSourceRef {
        SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute_names` after provisioning.\n"]
    pub fn attribute_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attribute_names", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_group_names` after provisioning.\n"]
    pub fn instance_group_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_group_names", self.base))
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
    ) -> terrars::ListRef<SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_HubAccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hub_access_config", self.base))
    }

    #[doc = "Get a reference to the value of field `model_access_config` after provisioning.\n"]
    pub fn model_access_config(
        &self,
    ) -> terrars::ListRef<SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource_ModelAccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_access_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerTrainingJob_InputDataConfig_DataSourceDynamic {
    file_system_data_source: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerTrainingJob_InputDataConfig_DataSource_FileSystemDataSource>>,
    s3_data_source: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource>>,
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_InputDataConfig_DataSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_data_source: std
    ::option
    ::Option<std::vec::Vec<SagemakerTrainingJob_InputDataConfig_DataSource_FileSystemDataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_data_source: std::option::Option<std::vec::Vec<SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource>>,
    dynamic: SagemakerTrainingJob_InputDataConfig_DataSourceDynamic,
}

impl SagemakerTrainingJob_InputDataConfig_DataSource {
    #[doc = "Set the field `file_system_data_source`.\n"]
    pub fn set_file_system_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerTrainingJob_InputDataConfig_DataSource_FileSystemDataSource>,
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
                    ::Into<terrars::BlockAssignable<SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSource>>,
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

impl terrars::ToListMappable for SagemakerTrainingJob_InputDataConfig_DataSource {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_InputDataConfig_DataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_InputDataConfig_DataSource {}

impl BuildSagemakerTrainingJob_InputDataConfig_DataSource {
    pub fn build(self) -> SagemakerTrainingJob_InputDataConfig_DataSource {
        SagemakerTrainingJob_InputDataConfig_DataSource {
            file_system_data_source: core::default::Default::default(),
            s3_data_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_InputDataConfig_DataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_InputDataConfig_DataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerTrainingJob_InputDataConfig_DataSourceRef {
        SagemakerTrainingJob_InputDataConfig_DataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_InputDataConfig_DataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_system_data_source` after provisioning.\n"]
    pub fn file_system_data_source(
        &self,
    ) -> terrars::ListRef<SagemakerTrainingJob_InputDataConfig_DataSource_FileSystemDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_system_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_data_source` after provisioning.\n"]
    pub fn s3_data_source(&self) -> terrars::ListRef<SagemakerTrainingJob_InputDataConfig_DataSource_S3DataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_data_source", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_InputDataConfig_ShuffleConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seed: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerTrainingJob_InputDataConfig_ShuffleConfig {
    #[doc = "Set the field `seed`.\n"]
    pub fn set_seed(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seed = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerTrainingJob_InputDataConfig_ShuffleConfig {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_InputDataConfig_ShuffleConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_InputDataConfig_ShuffleConfig {}

impl BuildSagemakerTrainingJob_InputDataConfig_ShuffleConfig {
    pub fn build(self) -> SagemakerTrainingJob_InputDataConfig_ShuffleConfig {
        SagemakerTrainingJob_InputDataConfig_ShuffleConfig { seed: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_InputDataConfig_ShuffleConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_InputDataConfig_ShuffleConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerTrainingJob_InputDataConfig_ShuffleConfigRef {
        SagemakerTrainingJob_InputDataConfig_ShuffleConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_InputDataConfig_ShuffleConfigRef {
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
struct SagemakerTrainingJob_InputDataConfigDynamic {
    data_source: std::option::Option<terrars::DynamicBlock<SagemakerTrainingJob_InputDataConfig_DataSource>>,
    shuffle_config: std::option::Option<terrars::DynamicBlock<SagemakerTrainingJob_InputDataConfig_ShuffleConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_InputDataConfig {
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
    data_source: std::option::Option<std::vec::Vec<SagemakerTrainingJob_InputDataConfig_DataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shuffle_config: std::option::Option<std::vec::Vec<SagemakerTrainingJob_InputDataConfig_ShuffleConfig>>,
    dynamic: SagemakerTrainingJob_InputDataConfigDynamic,
}

impl SagemakerTrainingJob_InputDataConfig {
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
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_InputDataConfig_DataSource>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_InputDataConfig_ShuffleConfig>>,
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

impl terrars::ToListMappable for SagemakerTrainingJob_InputDataConfig {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_InputDataConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_InputDataConfig {
    #[doc = ""]
    pub channel_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerTrainingJob_InputDataConfig {
    pub fn build(self) -> SagemakerTrainingJob_InputDataConfig {
        SagemakerTrainingJob_InputDataConfig {
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
pub struct SagemakerTrainingJob_InputDataConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_InputDataConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerTrainingJob_InputDataConfigRef {
        SagemakerTrainingJob_InputDataConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_InputDataConfigRef {
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
    pub fn data_source(&self) -> terrars::ListRef<SagemakerTrainingJob_InputDataConfig_DataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `shuffle_config` after provisioning.\n"]
    pub fn shuffle_config(&self) -> terrars::ListRef<SagemakerTrainingJob_InputDataConfig_ShuffleConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shuffle_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_MlflowConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mlflow_experiment_name: std::option::Option<terrars::PrimField<std::string::String>>,
    mlflow_resource_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mlflow_run_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerTrainingJob_MlflowConfig {
    #[doc = "Set the field `mlflow_experiment_name`.\n"]
    pub fn set_mlflow_experiment_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.mlflow_experiment_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mlflow_run_name`.\n"]
    pub fn set_mlflow_run_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mlflow_run_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerTrainingJob_MlflowConfig {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_MlflowConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_MlflowConfig {
    #[doc = ""]
    pub mlflow_resource_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerTrainingJob_MlflowConfig {
    pub fn build(self) -> SagemakerTrainingJob_MlflowConfig {
        SagemakerTrainingJob_MlflowConfig {
            mlflow_experiment_name: core::default::Default::default(),
            mlflow_resource_arn: self.mlflow_resource_arn,
            mlflow_run_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_MlflowConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_MlflowConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerTrainingJob_MlflowConfigRef {
        SagemakerTrainingJob_MlflowConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_MlflowConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mlflow_experiment_name` after provisioning.\n"]
    pub fn mlflow_experiment_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mlflow_experiment_name", self.base))
    }

    #[doc = "Get a reference to the value of field `mlflow_resource_arn` after provisioning.\n"]
    pub fn mlflow_resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mlflow_resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `mlflow_run_name` after provisioning.\n"]
    pub fn mlflow_run_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mlflow_run_name", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_ModelPackageConfig {
    model_package_group_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_model_package_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerTrainingJob_ModelPackageConfig {
    #[doc = "Set the field `source_model_package_arn`.\n"]
    pub fn set_source_model_package_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_model_package_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerTrainingJob_ModelPackageConfig {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_ModelPackageConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_ModelPackageConfig {
    #[doc = ""]
    pub model_package_group_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerTrainingJob_ModelPackageConfig {
    pub fn build(self) -> SagemakerTrainingJob_ModelPackageConfig {
        SagemakerTrainingJob_ModelPackageConfig {
            model_package_group_arn: self.model_package_group_arn,
            source_model_package_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_ModelPackageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_ModelPackageConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerTrainingJob_ModelPackageConfigRef {
        SagemakerTrainingJob_ModelPackageConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_ModelPackageConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `model_package_group_arn` after provisioning.\n"]
    pub fn model_package_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_package_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `source_model_package_arn` after provisioning.\n"]
    pub fn source_model_package_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_model_package_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_OutputDataConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_output_path: terrars::PrimField<std::string::String>,
}

impl SagemakerTrainingJob_OutputDataConfig {
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

impl terrars::ToListMappable for SagemakerTrainingJob_OutputDataConfig {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_OutputDataConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_OutputDataConfig {
    #[doc = ""]
    pub s3_output_path: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerTrainingJob_OutputDataConfig {
    pub fn build(self) -> SagemakerTrainingJob_OutputDataConfig {
        SagemakerTrainingJob_OutputDataConfig {
            compression_type: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            s3_output_path: self.s3_output_path,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_OutputDataConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_OutputDataConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerTrainingJob_OutputDataConfigRef {
        SagemakerTrainingJob_OutputDataConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_OutputDataConfigRef {
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
pub struct SagemakerTrainingJob_ProfilerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_profiler: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profiling_interval_in_milliseconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profiling_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_output_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerTrainingJob_ProfilerConfig {
    #[doc = "Set the field `disable_profiler`.\n"]
    pub fn set_disable_profiler(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_profiler = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `profiling_interval_in_milliseconds`.\n"]
    pub fn set_profiling_interval_in_milliseconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.profiling_interval_in_milliseconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `profiling_parameters`.\n"]
    pub fn set_profiling_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.profiling_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_output_path`.\n"]
    pub fn set_s3_output_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_output_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerTrainingJob_ProfilerConfig {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_ProfilerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_ProfilerConfig {}

impl BuildSagemakerTrainingJob_ProfilerConfig {
    pub fn build(self) -> SagemakerTrainingJob_ProfilerConfig {
        SagemakerTrainingJob_ProfilerConfig {
            disable_profiler: core::default::Default::default(),
            profiling_interval_in_milliseconds: core::default::Default::default(),
            profiling_parameters: core::default::Default::default(),
            s3_output_path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_ProfilerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_ProfilerConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerTrainingJob_ProfilerConfigRef {
        SagemakerTrainingJob_ProfilerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_ProfilerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disable_profiler` after provisioning.\n"]
    pub fn disable_profiler(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_profiler", self.base))
    }

    #[doc = "Get a reference to the value of field `profiling_interval_in_milliseconds` after provisioning.\n"]
    pub fn profiling_interval_in_milliseconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profiling_interval_in_milliseconds", self.base))
    }

    #[doc = "Get a reference to the value of field `profiling_parameters` after provisioning.\n"]
    pub fn profiling_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.profiling_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_path` after provisioning.\n"]
    pub fn s3_output_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_output_path", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_ProfilerRuleConfigurations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_path: std::option::Option<terrars::PrimField<std::string::String>>,
    rule_configuration_name: terrars::PrimField<std::string::String>,
    rule_evaluator_image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_output_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size_in_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerTrainingJob_ProfilerRuleConfigurations {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_path`.\n"]
    pub fn set_local_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_parameters`.\n"]
    pub fn set_rule_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.rule_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_output_path`.\n"]
    pub fn set_s3_output_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_output_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size_in_gb`.\n"]
    pub fn set_volume_size_in_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_size_in_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerTrainingJob_ProfilerRuleConfigurations {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_ProfilerRuleConfigurations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_ProfilerRuleConfigurations {
    #[doc = ""]
    pub rule_configuration_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule_evaluator_image: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerTrainingJob_ProfilerRuleConfigurations {
    pub fn build(self) -> SagemakerTrainingJob_ProfilerRuleConfigurations {
        SagemakerTrainingJob_ProfilerRuleConfigurations {
            instance_type: core::default::Default::default(),
            local_path: core::default::Default::default(),
            rule_configuration_name: self.rule_configuration_name,
            rule_evaluator_image: self.rule_evaluator_image,
            rule_parameters: core::default::Default::default(),
            s3_output_path: core::default::Default::default(),
            volume_size_in_gb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_ProfilerRuleConfigurationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_ProfilerRuleConfigurationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerTrainingJob_ProfilerRuleConfigurationsRef {
        SagemakerTrainingJob_ProfilerRuleConfigurationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_ProfilerRuleConfigurationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `local_path` after provisioning.\n"]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_configuration_name` after provisioning.\n"]
    pub fn rule_configuration_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_configuration_name", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_evaluator_image` after provisioning.\n"]
    pub fn rule_evaluator_image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_evaluator_image", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_parameters` after provisioning.\n"]
    pub fn rule_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.rule_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_path` after provisioning.\n"]
    pub fn s3_output_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_output_path", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size_in_gb` after provisioning.\n"]
    pub fn volume_size_in_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_size_in_gb", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_RemoteDebugConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_remote_debug: std::option::Option<terrars::PrimField<bool>>,
}

impl SagemakerTrainingJob_RemoteDebugConfig {
    #[doc = "Set the field `enable_remote_debug`.\n"]
    pub fn set_enable_remote_debug(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_remote_debug = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerTrainingJob_RemoteDebugConfig {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_RemoteDebugConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_RemoteDebugConfig {}

impl BuildSagemakerTrainingJob_RemoteDebugConfig {
    pub fn build(self) -> SagemakerTrainingJob_RemoteDebugConfig {
        SagemakerTrainingJob_RemoteDebugConfig { enable_remote_debug: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_RemoteDebugConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_RemoteDebugConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerTrainingJob_RemoteDebugConfigRef {
        SagemakerTrainingJob_RemoteDebugConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_RemoteDebugConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_remote_debug` after provisioning.\n"]
    pub fn enable_remote_debug(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_remote_debug", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_ResourceConfig_InstanceGroups {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerTrainingJob_ResourceConfig_InstanceGroups {
    #[doc = "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_group_name`.\n"]
    pub fn set_instance_group_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerTrainingJob_ResourceConfig_InstanceGroups {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_ResourceConfig_InstanceGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_ResourceConfig_InstanceGroups {}

impl BuildSagemakerTrainingJob_ResourceConfig_InstanceGroups {
    pub fn build(self) -> SagemakerTrainingJob_ResourceConfig_InstanceGroups {
        SagemakerTrainingJob_ResourceConfig_InstanceGroups {
            instance_count: core::default::Default::default(),
            instance_group_name: core::default::Default::default(),
            instance_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_ResourceConfig_InstanceGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_ResourceConfig_InstanceGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerTrainingJob_ResourceConfig_InstanceGroupsRef {
        SagemakerTrainingJob_ResourceConfig_InstanceGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_ResourceConfig_InstanceGroupsRef {
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
pub struct SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig_PlacementSpecifications {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ultra_server_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig_PlacementSpecifications {
    #[doc = "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ultra_server_id`.\n"]
    pub fn set_ultra_server_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ultra_server_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig_PlacementSpecifications {
    type O =
        terrars
        ::BlockAssignable<SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig_PlacementSpecifications>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_ResourceConfig_InstancePlacementConfig_PlacementSpecifications {}

impl BuildSagemakerTrainingJob_ResourceConfig_InstancePlacementConfig_PlacementSpecifications {
    pub fn build(self) -> SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig_PlacementSpecifications {
        SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig_PlacementSpecifications {
            instance_count: core::default::Default::default(),
            ultra_server_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig_PlacementSpecificationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig_PlacementSpecificationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig_PlacementSpecificationsRef {
        SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig_PlacementSpecificationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig_PlacementSpecificationsRef {
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
struct SagemakerTrainingJob_ResourceConfig_InstancePlacementConfigDynamic {
    placement_specifications: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig_PlacementSpecifications>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_multiple_jobs: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_specifications: std
    ::option
    ::Option<std::vec::Vec<SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig_PlacementSpecifications>>,
    dynamic: SagemakerTrainingJob_ResourceConfig_InstancePlacementConfigDynamic,
}

impl SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig {
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
                            SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig_PlacementSpecifications,
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

impl terrars::ToListMappable for SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_ResourceConfig_InstancePlacementConfig {}

impl BuildSagemakerTrainingJob_ResourceConfig_InstancePlacementConfig {
    pub fn build(self) -> SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig {
        SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig {
            enable_multiple_jobs: core::default::Default::default(),
            placement_specifications: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_ResourceConfig_InstancePlacementConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_ResourceConfig_InstancePlacementConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerTrainingJob_ResourceConfig_InstancePlacementConfigRef {
        SagemakerTrainingJob_ResourceConfig_InstancePlacementConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_ResourceConfig_InstancePlacementConfigRef {
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
    ) -> terrars::ListRef<SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig_PlacementSpecificationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement_specifications", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerTrainingJob_ResourceConfigDynamic {
    instance_groups: std::option::Option<terrars::DynamicBlock<SagemakerTrainingJob_ResourceConfig_InstanceGroups>>,
    instance_placement_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_ResourceConfig {
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
    instance_groups: std::option::Option<std::vec::Vec<SagemakerTrainingJob_ResourceConfig_InstanceGroups>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_placement_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig>>,
    dynamic: SagemakerTrainingJob_ResourceConfigDynamic,
}

impl SagemakerTrainingJob_ResourceConfig {
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
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerTrainingJob_ResourceConfig_InstanceGroups>>,
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
                    ::Into<terrars::BlockAssignable<SagemakerTrainingJob_ResourceConfig_InstancePlacementConfig>>,
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

impl terrars::ToListMappable for SagemakerTrainingJob_ResourceConfig {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_ResourceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_ResourceConfig {}

impl BuildSagemakerTrainingJob_ResourceConfig {
    pub fn build(self) -> SagemakerTrainingJob_ResourceConfig {
        SagemakerTrainingJob_ResourceConfig {
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
pub struct SagemakerTrainingJob_ResourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_ResourceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerTrainingJob_ResourceConfigRef {
        SagemakerTrainingJob_ResourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_ResourceConfigRef {
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
    pub fn instance_groups(&self) -> terrars::ListRef<SagemakerTrainingJob_ResourceConfig_InstanceGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_placement_config` after provisioning.\n"]
    pub fn instance_placement_config(
        &self,
    ) -> terrars::ListRef<SagemakerTrainingJob_ResourceConfig_InstancePlacementConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_placement_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_RetryStrategy {
    maximum_retry_attempts: terrars::PrimField<f64>,
}

impl SagemakerTrainingJob_RetryStrategy { }

impl terrars::ToListMappable for SagemakerTrainingJob_RetryStrategy {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_RetryStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_RetryStrategy {
    #[doc = ""]
    pub maximum_retry_attempts: terrars::PrimField<f64>,
}

impl BuildSagemakerTrainingJob_RetryStrategy {
    pub fn build(self) -> SagemakerTrainingJob_RetryStrategy {
        SagemakerTrainingJob_RetryStrategy { maximum_retry_attempts: self.maximum_retry_attempts }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_RetryStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_RetryStrategyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerTrainingJob_RetryStrategyRef {
        SagemakerTrainingJob_RetryStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_RetryStrategyRef {
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
pub struct SagemakerTrainingJob_ServerlessJobConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accept_eula: std::option::Option<terrars::PrimField<bool>>,
    base_model_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customization_technique: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluator_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    job_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peft: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerTrainingJob_ServerlessJobConfig {
    #[doc = "Set the field `accept_eula`.\n"]
    pub fn set_accept_eula(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.accept_eula = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customization_technique`.\n"]
    pub fn set_customization_technique(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.customization_technique = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `evaluation_type`.\n"]
    pub fn set_evaluation_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.evaluation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `evaluator_arn`.\n"]
    pub fn set_evaluator_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.evaluator_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peft`.\n"]
    pub fn set_peft(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peft = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerTrainingJob_ServerlessJobConfig {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_ServerlessJobConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_ServerlessJobConfig {
    #[doc =
        "Base model ARN in SageMaker Public Hub. SageMaker always selects the latest version of the provided model."]
    pub base_model_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub job_type: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerTrainingJob_ServerlessJobConfig {
    pub fn build(self) -> SagemakerTrainingJob_ServerlessJobConfig {
        SagemakerTrainingJob_ServerlessJobConfig {
            accept_eula: core::default::Default::default(),
            base_model_arn: self.base_model_arn,
            customization_technique: core::default::Default::default(),
            evaluation_type: core::default::Default::default(),
            evaluator_arn: core::default::Default::default(),
            job_type: self.job_type,
            peft: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_ServerlessJobConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_ServerlessJobConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerTrainingJob_ServerlessJobConfigRef {
        SagemakerTrainingJob_ServerlessJobConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_ServerlessJobConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accept_eula` after provisioning.\n"]
    pub fn accept_eula(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accept_eula", self.base))
    }

    #[doc =
        "Get a reference to the value of field `base_model_arn` after provisioning.\nBase model ARN in SageMaker Public Hub. SageMaker always selects the latest version of the provided model."]
    pub fn base_model_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_model_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `customization_technique` after provisioning.\n"]
    pub fn customization_technique(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customization_technique", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluation_type` after provisioning.\n"]
    pub fn evaluation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluator_arn` after provisioning.\n"]
    pub fn evaluator_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluator_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `job_type` after provisioning.\n"]
    pub fn job_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_type", self.base))
    }

    #[doc = "Get a reference to the value of field `peft` after provisioning.\n"]
    pub fn peft(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peft", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_SessionChainingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_session_tag_chaining: std::option::Option<terrars::PrimField<bool>>,
}

impl SagemakerTrainingJob_SessionChainingConfig {
    #[doc = "Set the field `enable_session_tag_chaining`.\n"]
    pub fn set_enable_session_tag_chaining(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_session_tag_chaining = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerTrainingJob_SessionChainingConfig {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_SessionChainingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_SessionChainingConfig {}

impl BuildSagemakerTrainingJob_SessionChainingConfig {
    pub fn build(self) -> SagemakerTrainingJob_SessionChainingConfig {
        SagemakerTrainingJob_SessionChainingConfig { enable_session_tag_chaining: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_SessionChainingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_SessionChainingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerTrainingJob_SessionChainingConfigRef {
        SagemakerTrainingJob_SessionChainingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_SessionChainingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_session_tag_chaining` after provisioning.\n"]
    pub fn enable_session_tag_chaining(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_session_tag_chaining", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_StoppingCondition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pending_time_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_runtime_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_wait_time_in_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerTrainingJob_StoppingCondition {
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

impl terrars::ToListMappable for SagemakerTrainingJob_StoppingCondition {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_StoppingCondition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_StoppingCondition {}

impl BuildSagemakerTrainingJob_StoppingCondition {
    pub fn build(self) -> SagemakerTrainingJob_StoppingCondition {
        SagemakerTrainingJob_StoppingCondition {
            max_pending_time_in_seconds: core::default::Default::default(),
            max_runtime_in_seconds: core::default::Default::default(),
            max_wait_time_in_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_StoppingConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_StoppingConditionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerTrainingJob_StoppingConditionRef {
        SagemakerTrainingJob_StoppingConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_StoppingConditionRef {
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
pub struct SagemakerTrainingJob_TensorBoardOutputConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_path: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_output_path: terrars::PrimField<std::string::String>,
}

impl SagemakerTrainingJob_TensorBoardOutputConfig {
    #[doc = "Set the field `local_path`.\n"]
    pub fn set_local_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerTrainingJob_TensorBoardOutputConfig {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_TensorBoardOutputConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_TensorBoardOutputConfig {
    #[doc = ""]
    pub s3_output_path: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerTrainingJob_TensorBoardOutputConfig {
    pub fn build(self) -> SagemakerTrainingJob_TensorBoardOutputConfig {
        SagemakerTrainingJob_TensorBoardOutputConfig {
            local_path: core::default::Default::default(),
            s3_output_path: self.s3_output_path,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_TensorBoardOutputConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_TensorBoardOutputConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerTrainingJob_TensorBoardOutputConfigRef {
        SagemakerTrainingJob_TensorBoardOutputConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_TensorBoardOutputConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `local_path` after provisioning.\n"]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_path` after provisioning.\n"]
    pub fn s3_output_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_output_path", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerTrainingJob_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerTrainingJob_Timeouts {
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

impl terrars::ToListMappable for SagemakerTrainingJob_Timeouts {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_Timeouts {}

impl BuildSagemakerTrainingJob_Timeouts {
    pub fn build(self) -> SagemakerTrainingJob_Timeouts {
        SagemakerTrainingJob_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerTrainingJob_TimeoutsRef {
        SagemakerTrainingJob_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_TimeoutsRef {
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

#[derive(Serialize)]
pub struct SagemakerTrainingJob_VpcConfig {
    security_group_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    subnets: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl SagemakerTrainingJob_VpcConfig { }

impl terrars::ToListMappable for SagemakerTrainingJob_VpcConfig {
    type O = terrars::BlockAssignable<SagemakerTrainingJob_VpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerTrainingJob_VpcConfig {
    #[doc = ""]
    pub security_group_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnets: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildSagemakerTrainingJob_VpcConfig {
    pub fn build(self) -> SagemakerTrainingJob_VpcConfig {
        SagemakerTrainingJob_VpcConfig {
            security_group_ids: self.security_group_ids,
            subnets: self.subnets,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerTrainingJob_VpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerTrainingJob_VpcConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerTrainingJob_VpcConfigRef {
        SagemakerTrainingJob_VpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerTrainingJob_VpcConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}
