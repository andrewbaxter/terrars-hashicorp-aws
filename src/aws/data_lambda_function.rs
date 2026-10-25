#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataLambdaFunctionData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    function_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qualifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataLambdaFunction_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLambdaFunctionData_>,
}

#[derive(Clone)]
pub struct DataLambdaFunction(std::rc::Rc<DataLambdaFunction_>);

impl DataLambdaFunction {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `qualifier`.\n"]
    pub fn set_qualifier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().qualifier = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `architectures` after provisioning.\n"]
    pub fn architectures(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.architectures", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_provider_config` after provisioning.\n"]
    pub fn capacity_provider_config(&self) -> terrars::ListRef<DataLambdaFunction_CapacityProviderConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_provider_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code_sha256` after provisioning.\n"]
    pub fn code_sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_sha256", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code_signing_config_arn` after provisioning.\n"]
    pub fn code_signing_config_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_signing_config_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> terrars::ListRef<DataLambdaFunction_DeadLetterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `durable_config` after provisioning.\n"]
    pub fn durable_config(&self) -> terrars::ListRef<DataLambdaFunction_DurableConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.durable_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::ListRef<DataLambdaFunction_EnvironmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> terrars::ListRef<DataLambdaFunction_EphemeralStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_config` after provisioning.\n"]
    pub fn file_system_config(&self) -> terrars::ListRef<DataLambdaFunction_FileSystemConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_system_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `handler` after provisioning.\n"]
    pub fn handler(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.handler", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_uri` after provisioning.\n"]
    pub fn image_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `invoke_arn` after provisioning.\n"]
    pub fn invoke_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invoke_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `layers` after provisioning.\n"]
    pub fn layers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.layers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<DataLambdaFunction_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_size` after provisioning.\n"]
    pub fn memory_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qualified_arn` after provisioning.\n"]
    pub fn qualified_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualified_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qualified_invoke_arn` after provisioning.\n"]
    pub fn qualified_invoke_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualified_invoke_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qualifier` after provisioning.\n"]
    pub fn qualifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reserved_concurrent_executions` after provisioning.\n"]
    pub fn reserved_concurrent_executions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.reserved_concurrent_executions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `response_streaming_invoke_arn` after provisioning.\n"]
    pub fn response_streaming_invoke_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_streaming_invoke_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime` after provisioning.\n"]
    pub fn runtime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signing_job_arn` after provisioning.\n"]
    pub fn signing_job_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signing_job_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signing_profile_version_arn` after provisioning.\n"]
    pub fn signing_profile_version_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signing_profile_version_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_code_hash` after provisioning.\n"]
    pub fn source_code_hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_code_hash", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_code_size` after provisioning.\n"]
    pub fn source_code_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_code_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_kms_key_arn` after provisioning.\n"]
    pub fn source_kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenancy_config` after provisioning.\n"]
    pub fn tenancy_config(&self) -> terrars::ListRef<DataLambdaFunction_TenancyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tenancy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tracing_config` after provisioning.\n"]
    pub fn tracing_config(&self) -> terrars::ListRef<DataLambdaFunction_TracingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tracing_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<DataLambdaFunction_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

impl terrars::Referable for DataLambdaFunction {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataLambdaFunction { }

impl terrars::ToListMappable for DataLambdaFunction {
    type O = terrars::ListRef<DataLambdaFunctionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataLambdaFunction_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_lambda_function".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLambdaFunction {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub function_name: terrars::PrimField<std::string::String>,
}

impl BuildDataLambdaFunction {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLambdaFunction {
        let out = DataLambdaFunction(std::rc::Rc::new(DataLambdaFunction_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLambdaFunctionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                function_name: self.function_name,
                id: core::default::Default::default(),
                qualifier: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLambdaFunctionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdaFunctionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLambdaFunctionRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `architectures` after provisioning.\n"]
    pub fn architectures(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.architectures", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_provider_config` after provisioning.\n"]
    pub fn capacity_provider_config(&self) -> terrars::ListRef<DataLambdaFunction_CapacityProviderConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_provider_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code_sha256` after provisioning.\n"]
    pub fn code_sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_sha256", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code_signing_config_arn` after provisioning.\n"]
    pub fn code_signing_config_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_signing_config_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> terrars::ListRef<DataLambdaFunction_DeadLetterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `durable_config` after provisioning.\n"]
    pub fn durable_config(&self) -> terrars::ListRef<DataLambdaFunction_DurableConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.durable_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::ListRef<DataLambdaFunction_EnvironmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> terrars::ListRef<DataLambdaFunction_EphemeralStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_config` after provisioning.\n"]
    pub fn file_system_config(&self) -> terrars::ListRef<DataLambdaFunction_FileSystemConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_system_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `handler` after provisioning.\n"]
    pub fn handler(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.handler", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_uri` after provisioning.\n"]
    pub fn image_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `invoke_arn` after provisioning.\n"]
    pub fn invoke_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invoke_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `layers` after provisioning.\n"]
    pub fn layers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.layers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<DataLambdaFunction_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_size` after provisioning.\n"]
    pub fn memory_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qualified_arn` after provisioning.\n"]
    pub fn qualified_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualified_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qualified_invoke_arn` after provisioning.\n"]
    pub fn qualified_invoke_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualified_invoke_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qualifier` after provisioning.\n"]
    pub fn qualifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reserved_concurrent_executions` after provisioning.\n"]
    pub fn reserved_concurrent_executions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.reserved_concurrent_executions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `response_streaming_invoke_arn` after provisioning.\n"]
    pub fn response_streaming_invoke_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_streaming_invoke_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime` after provisioning.\n"]
    pub fn runtime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signing_job_arn` after provisioning.\n"]
    pub fn signing_job_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signing_job_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signing_profile_version_arn` after provisioning.\n"]
    pub fn signing_profile_version_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signing_profile_version_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_code_hash` after provisioning.\n"]
    pub fn source_code_hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_code_hash", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_code_size` after provisioning.\n"]
    pub fn source_code_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_code_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_kms_key_arn` after provisioning.\n"]
    pub fn source_kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenancy_config` after provisioning.\n"]
    pub fn tenancy_config(&self) -> terrars::ListRef<DataLambdaFunction_TenancyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tenancy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tracing_config` after provisioning.\n"]
    pub fn tracing_config(&self) -> terrars::ListRef<DataLambdaFunction_TracingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tracing_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<DataLambdaFunction_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_provider_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_environment_memory_gib_per_vcpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_execution_environment_max_concurrency: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfig {
    #[doc = "Set the field `capacity_provider_arn`.\n"]
    pub fn set_capacity_provider_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.capacity_provider_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_environment_memory_gib_per_vcpu`.\n"]
    pub fn set_execution_environment_memory_gib_per_vcpu(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.execution_environment_memory_gib_per_vcpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `per_execution_environment_max_concurrency`.\n"]
    pub fn set_per_execution_environment_max_concurrency(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.per_execution_environment_max_concurrency = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfig {
    type O =
        terrars
        ::BlockAssignable<DataLambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfig {}

impl BuildDataLambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfig {
    pub fn build(self) -> DataLambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfig {
        DataLambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfig {
            capacity_provider_arn: core::default::Default::default(),
            execution_environment_memory_gib_per_vcpu: core::default::Default::default(),
            per_execution_environment_max_concurrency: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfigRef {
        DataLambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_provider_arn` after provisioning.\n"]
    pub fn capacity_provider_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_provider_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_environment_memory_gib_per_vcpu` after provisioning.\n"]
    pub fn execution_environment_memory_gib_per_vcpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_environment_memory_gib_per_vcpu", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `per_execution_environment_max_concurrency` after provisioning.\n"]
    pub fn per_execution_environment_max_concurrency(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.per_execution_environment_max_concurrency", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataLambdaFunction_CapacityProviderConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_managed_instances_capacity_provider_config: std
    ::option
    ::Option<
        terrars::ListField<DataLambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfig>,
    >,
}

impl DataLambdaFunction_CapacityProviderConfig {
    #[doc = "Set the field `lambda_managed_instances_capacity_provider_config`.\n"]
    pub fn set_lambda_managed_instances_capacity_provider_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataLambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfig,
                        >,
                    >,
    ) -> Self {
        self.lambda_managed_instances_capacity_provider_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdaFunction_CapacityProviderConfig {
    type O = terrars::BlockAssignable<DataLambdaFunction_CapacityProviderConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdaFunction_CapacityProviderConfig {}

impl BuildDataLambdaFunction_CapacityProviderConfig {
    pub fn build(self) -> DataLambdaFunction_CapacityProviderConfig {
        DataLambdaFunction_CapacityProviderConfig {
            lambda_managed_instances_capacity_provider_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLambdaFunction_CapacityProviderConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdaFunction_CapacityProviderConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLambdaFunction_CapacityProviderConfigRef {
        DataLambdaFunction_CapacityProviderConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdaFunction_CapacityProviderConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `lambda_managed_instances_capacity_provider_config` after provisioning.\n"]
    pub fn lambda_managed_instances_capacity_provider_config(
        &self,
    ) -> terrars::ListRef<DataLambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.lambda_managed_instances_capacity_provider_config", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataLambdaFunction_DeadLetterConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLambdaFunction_DeadLetterConfig {
    #[doc = "Set the field `target_arn`.\n"]
    pub fn set_target_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdaFunction_DeadLetterConfig {
    type O = terrars::BlockAssignable<DataLambdaFunction_DeadLetterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdaFunction_DeadLetterConfig {}

impl BuildDataLambdaFunction_DeadLetterConfig {
    pub fn build(self) -> DataLambdaFunction_DeadLetterConfig {
        DataLambdaFunction_DeadLetterConfig { target_arn: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataLambdaFunction_DeadLetterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdaFunction_DeadLetterConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLambdaFunction_DeadLetterConfigRef {
        DataLambdaFunction_DeadLetterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdaFunction_DeadLetterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_arn` after provisioning.\n"]
    pub fn target_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLambdaFunction_DurableConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_period: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLambdaFunction_DurableConfig {
    #[doc = "Set the field `execution_timeout`.\n"]
    pub fn set_execution_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.execution_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention_period`.\n"]
    pub fn set_retention_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retention_period = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdaFunction_DurableConfig {
    type O = terrars::BlockAssignable<DataLambdaFunction_DurableConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdaFunction_DurableConfig {}

impl BuildDataLambdaFunction_DurableConfig {
    pub fn build(self) -> DataLambdaFunction_DurableConfig {
        DataLambdaFunction_DurableConfig {
            execution_timeout: core::default::Default::default(),
            retention_period: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLambdaFunction_DurableConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdaFunction_DurableConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLambdaFunction_DurableConfigRef {
        DataLambdaFunction_DurableConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdaFunction_DurableConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execution_timeout` after provisioning.\n"]
    pub fn execution_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_period` after provisioning.\n"]
    pub fn retention_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_period", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLambdaFunction_Environment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataLambdaFunction_Environment {
    #[doc = "Set the field `variables`.\n"]
    pub fn set_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.variables = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdaFunction_Environment {
    type O = terrars::BlockAssignable<DataLambdaFunction_Environment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdaFunction_Environment {}

impl BuildDataLambdaFunction_Environment {
    pub fn build(self) -> DataLambdaFunction_Environment {
        DataLambdaFunction_Environment { variables: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataLambdaFunction_EnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdaFunction_EnvironmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLambdaFunction_EnvironmentRef {
        DataLambdaFunction_EnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdaFunction_EnvironmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `variables` after provisioning.\n"]
    pub fn variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.variables", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLambdaFunction_EphemeralStorage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLambdaFunction_EphemeralStorage {
    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdaFunction_EphemeralStorage {
    type O = terrars::BlockAssignable<DataLambdaFunction_EphemeralStorage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdaFunction_EphemeralStorage {}

impl BuildDataLambdaFunction_EphemeralStorage {
    pub fn build(self) -> DataLambdaFunction_EphemeralStorage {
        DataLambdaFunction_EphemeralStorage { size: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataLambdaFunction_EphemeralStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdaFunction_EphemeralStorageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLambdaFunction_EphemeralStorageRef {
        DataLambdaFunction_EphemeralStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdaFunction_EphemeralStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLambdaFunction_FileSystemConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_mount_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLambdaFunction_FileSystemConfig {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_mount_path`.\n"]
    pub fn set_local_mount_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_mount_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdaFunction_FileSystemConfig {
    type O = terrars::BlockAssignable<DataLambdaFunction_FileSystemConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdaFunction_FileSystemConfig {}

impl BuildDataLambdaFunction_FileSystemConfig {
    pub fn build(self) -> DataLambdaFunction_FileSystemConfig {
        DataLambdaFunction_FileSystemConfig {
            arn: core::default::Default::default(),
            local_mount_path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLambdaFunction_FileSystemConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdaFunction_FileSystemConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLambdaFunction_FileSystemConfigRef {
        DataLambdaFunction_FileSystemConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdaFunction_FileSystemConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `local_mount_path` after provisioning.\n"]
    pub fn local_mount_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_mount_path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLambdaFunction_LoggingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_log_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system_log_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLambdaFunction_LoggingConfig {
    #[doc = "Set the field `application_log_level`.\n"]
    pub fn set_application_log_level(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.application_log_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_format`.\n"]
    pub fn set_log_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_group`.\n"]
    pub fn set_log_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `system_log_level`.\n"]
    pub fn set_system_log_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.system_log_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdaFunction_LoggingConfig {
    type O = terrars::BlockAssignable<DataLambdaFunction_LoggingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdaFunction_LoggingConfig {}

impl BuildDataLambdaFunction_LoggingConfig {
    pub fn build(self) -> DataLambdaFunction_LoggingConfig {
        DataLambdaFunction_LoggingConfig {
            application_log_level: core::default::Default::default(),
            log_format: core::default::Default::default(),
            log_group: core::default::Default::default(),
            system_log_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLambdaFunction_LoggingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdaFunction_LoggingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLambdaFunction_LoggingConfigRef {
        DataLambdaFunction_LoggingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdaFunction_LoggingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_log_level` after provisioning.\n"]
    pub fn application_log_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_log_level", self.base))
    }

    #[doc = "Get a reference to the value of field `log_format` after provisioning.\n"]
    pub fn log_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_format", self.base))
    }

    #[doc = "Get a reference to the value of field `log_group` after provisioning.\n"]
    pub fn log_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group", self.base))
    }

    #[doc = "Get a reference to the value of field `system_log_level` after provisioning.\n"]
    pub fn system_log_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.system_log_level", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLambdaFunction_TenancyConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_isolation_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLambdaFunction_TenancyConfig {
    #[doc = "Set the field `tenant_isolation_mode`.\n"]
    pub fn set_tenant_isolation_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.tenant_isolation_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdaFunction_TenancyConfig {
    type O = terrars::BlockAssignable<DataLambdaFunction_TenancyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdaFunction_TenancyConfig {}

impl BuildDataLambdaFunction_TenancyConfig {
    pub fn build(self) -> DataLambdaFunction_TenancyConfig {
        DataLambdaFunction_TenancyConfig { tenant_isolation_mode: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataLambdaFunction_TenancyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdaFunction_TenancyConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLambdaFunction_TenancyConfigRef {
        DataLambdaFunction_TenancyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdaFunction_TenancyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tenant_isolation_mode` after provisioning.\n"]
    pub fn tenant_isolation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_isolation_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLambdaFunction_TracingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLambdaFunction_TracingConfig {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdaFunction_TracingConfig {
    type O = terrars::BlockAssignable<DataLambdaFunction_TracingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdaFunction_TracingConfig {}

impl BuildDataLambdaFunction_TracingConfig {
    pub fn build(self) -> DataLambdaFunction_TracingConfig {
        DataLambdaFunction_TracingConfig { mode: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataLambdaFunction_TracingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdaFunction_TracingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLambdaFunction_TracingConfigRef {
        DataLambdaFunction_TracingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdaFunction_TracingConfigRef {
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
pub struct DataLambdaFunction_VpcConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_allowed_for_dual_stack: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLambdaFunction_VpcConfig {
    #[doc = "Set the field `ipv6_allowed_for_dual_stack`.\n"]
    pub fn set_ipv6_allowed_for_dual_stack(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ipv6_allowed_for_dual_stack = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnet_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdaFunction_VpcConfig {
    type O = terrars::BlockAssignable<DataLambdaFunction_VpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdaFunction_VpcConfig {}

impl BuildDataLambdaFunction_VpcConfig {
    pub fn build(self) -> DataLambdaFunction_VpcConfig {
        DataLambdaFunction_VpcConfig {
            ipv6_allowed_for_dual_stack: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLambdaFunction_VpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdaFunction_VpcConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLambdaFunction_VpcConfigRef {
        DataLambdaFunction_VpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdaFunction_VpcConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ipv6_allowed_for_dual_stack` after provisioning.\n"]
    pub fn ipv6_allowed_for_dual_stack(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_allowed_for_dual_stack", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}
