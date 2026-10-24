use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataLambdaFunctionData {
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
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataLambdaFunction_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLambdaFunctionData>,
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

    #[doc = "Get a reference to the value of field `code_signing_config_arn` after provisioning.\n"]
    pub fn code_signing_config_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_signing_config_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> terrars::ListRef<DataLambdaFunctionDeadLetterConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::ListRef<DataLambdaFunctionEnvironmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> terrars::ListRef<DataLambdaFunctionEphemeralStorageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_config` after provisioning.\n"]
    pub fn file_system_config(&self) -> terrars::ListRef<DataLambdaFunctionFileSystemConfigElRef> {
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

    #[doc = "Get a reference to the value of field `reserved_concurrent_executions` after provisioning.\n"]
    pub fn reserved_concurrent_executions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.reserved_concurrent_executions", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tracing_config` after provisioning.\n"]
    pub fn tracing_config(&self) -> terrars::ListRef<DataLambdaFunctionTracingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tracing_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<DataLambdaFunctionVpcConfigElRef> {
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
            data: std::cell::RefCell::new(DataLambdaFunctionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                function_name: self.function_name,
                id: core::default::Default::default(),
                qualifier: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `code_signing_config_arn` after provisioning.\n"]
    pub fn code_signing_config_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_signing_config_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> terrars::ListRef<DataLambdaFunctionDeadLetterConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::ListRef<DataLambdaFunctionEnvironmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> terrars::ListRef<DataLambdaFunctionEphemeralStorageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_config` after provisioning.\n"]
    pub fn file_system_config(&self) -> terrars::ListRef<DataLambdaFunctionFileSystemConfigElRef> {
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

    #[doc = "Get a reference to the value of field `reserved_concurrent_executions` after provisioning.\n"]
    pub fn reserved_concurrent_executions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.reserved_concurrent_executions", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tracing_config` after provisioning.\n"]
    pub fn tracing_config(&self) -> terrars::ListRef<DataLambdaFunctionTracingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tracing_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<DataLambdaFunctionVpcConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLambdaFunctionDeadLetterConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLambdaFunctionDeadLetterConfigEl {
    #[doc = "Set the field `target_arn`.\n"]
    pub fn set_target_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdaFunctionDeadLetterConfigEl {
    type O = terrars::BlockAssignable<DataLambdaFunctionDeadLetterConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdaFunctionDeadLetterConfigEl {}

impl BuildDataLambdaFunctionDeadLetterConfigEl {
    pub fn build(self) -> DataLambdaFunctionDeadLetterConfigEl {
        DataLambdaFunctionDeadLetterConfigEl { target_arn: core::default::Default::default() }
    }
}

pub struct DataLambdaFunctionDeadLetterConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdaFunctionDeadLetterConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLambdaFunctionDeadLetterConfigElRef {
        DataLambdaFunctionDeadLetterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdaFunctionDeadLetterConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_arn` after provisioning.\n"]
    pub fn target_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLambdaFunctionEnvironmentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataLambdaFunctionEnvironmentEl {
    #[doc = "Set the field `variables`.\n"]
    pub fn set_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.variables = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdaFunctionEnvironmentEl {
    type O = terrars::BlockAssignable<DataLambdaFunctionEnvironmentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdaFunctionEnvironmentEl {}

impl BuildDataLambdaFunctionEnvironmentEl {
    pub fn build(self) -> DataLambdaFunctionEnvironmentEl {
        DataLambdaFunctionEnvironmentEl { variables: core::default::Default::default() }
    }
}

pub struct DataLambdaFunctionEnvironmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdaFunctionEnvironmentElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLambdaFunctionEnvironmentElRef {
        DataLambdaFunctionEnvironmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdaFunctionEnvironmentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `variables` after provisioning.\n"]
    pub fn variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.variables", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLambdaFunctionEphemeralStorageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLambdaFunctionEphemeralStorageEl {
    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdaFunctionEphemeralStorageEl {
    type O = terrars::BlockAssignable<DataLambdaFunctionEphemeralStorageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdaFunctionEphemeralStorageEl {}

impl BuildDataLambdaFunctionEphemeralStorageEl {
    pub fn build(self) -> DataLambdaFunctionEphemeralStorageEl {
        DataLambdaFunctionEphemeralStorageEl { size: core::default::Default::default() }
    }
}

pub struct DataLambdaFunctionEphemeralStorageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdaFunctionEphemeralStorageElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLambdaFunctionEphemeralStorageElRef {
        DataLambdaFunctionEphemeralStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdaFunctionEphemeralStorageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLambdaFunctionFileSystemConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_mount_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLambdaFunctionFileSystemConfigEl {
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

impl terrars::ToListMappable for DataLambdaFunctionFileSystemConfigEl {
    type O = terrars::BlockAssignable<DataLambdaFunctionFileSystemConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdaFunctionFileSystemConfigEl {}

impl BuildDataLambdaFunctionFileSystemConfigEl {
    pub fn build(self) -> DataLambdaFunctionFileSystemConfigEl {
        DataLambdaFunctionFileSystemConfigEl {
            arn: core::default::Default::default(),
            local_mount_path: core::default::Default::default(),
        }
    }
}

pub struct DataLambdaFunctionFileSystemConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdaFunctionFileSystemConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLambdaFunctionFileSystemConfigElRef {
        DataLambdaFunctionFileSystemConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdaFunctionFileSystemConfigElRef {
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
pub struct DataLambdaFunctionTracingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLambdaFunctionTracingConfigEl {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdaFunctionTracingConfigEl {
    type O = terrars::BlockAssignable<DataLambdaFunctionTracingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdaFunctionTracingConfigEl {}

impl BuildDataLambdaFunctionTracingConfigEl {
    pub fn build(self) -> DataLambdaFunctionTracingConfigEl {
        DataLambdaFunctionTracingConfigEl { mode: core::default::Default::default() }
    }
}

pub struct DataLambdaFunctionTracingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdaFunctionTracingConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLambdaFunctionTracingConfigElRef {
        DataLambdaFunctionTracingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdaFunctionTracingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLambdaFunctionVpcConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLambdaFunctionVpcConfigEl {
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

impl terrars::ToListMappable for DataLambdaFunctionVpcConfigEl {
    type O = terrars::BlockAssignable<DataLambdaFunctionVpcConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdaFunctionVpcConfigEl {}

impl BuildDataLambdaFunctionVpcConfigEl {
    pub fn build(self) -> DataLambdaFunctionVpcConfigEl {
        DataLambdaFunctionVpcConfigEl {
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}

pub struct DataLambdaFunctionVpcConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdaFunctionVpcConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLambdaFunctionVpcConfigElRef {
        DataLambdaFunctionVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdaFunctionVpcConfigElRef {
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

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}
