use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LambdaFunctionData {
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
    architectures: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_signing_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filename: std::option::Option<terrars::PrimField<std::string::String>>,
    function_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    handler: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    layers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    package_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reserved_concurrent_executions: std::option::Option<terrars::PrimField<f64>>,
    role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_object_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_code_hash: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dead_letter_config: std::option::Option<std::vec::Vec<LambdaFunctionDeadLetterConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<std::vec::Vec<LambdaFunctionEnvironmentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_storage: std::option::Option<std::vec::Vec<LambdaFunctionEphemeralStorageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_config: std::option::Option<std::vec::Vec<LambdaFunctionFileSystemConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_config: std::option::Option<std::vec::Vec<LambdaFunctionImageConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snap_start: std::option::Option<std::vec::Vec<LambdaFunctionSnapStartEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LambdaFunctionTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tracing_config: std::option::Option<std::vec::Vec<LambdaFunctionTracingConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<LambdaFunctionVpcConfigEl>>,
    dynamic: LambdaFunctionDynamic,
}

struct LambdaFunction_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LambdaFunctionData>,
}

#[derive(Clone)]
pub struct LambdaFunction(std::rc::Rc<LambdaFunction_>);

impl LambdaFunction {
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

    #[doc = "Set the field `architectures`.\n"]
    pub fn set_architectures(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().architectures = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `code_signing_config_arn`.\n"]
    pub fn set_code_signing_config_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().code_signing_config_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filename`.\n"]
    pub fn set_filename(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().filename = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `handler`.\n"]
    pub fn set_handler(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().handler = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_uri`.\n"]
    pub fn set_image_uri(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().image_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `layers`.\n"]
    pub fn set_layers(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().layers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_size`.\n"]
    pub fn set_memory_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().memory_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `package_type`.\n"]
    pub fn set_package_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().package_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `publish`.\n"]
    pub fn set_publish(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().publish = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reserved_concurrent_executions`.\n"]
    pub fn set_reserved_concurrent_executions(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().reserved_concurrent_executions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `runtime`.\n"]
    pub fn set_runtime(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().runtime = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_bucket`.\n"]
    pub fn set_s3_bucket(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().s3_bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_key`.\n"]
    pub fn set_s3_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().s3_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_object_version`.\n"]
    pub fn set_s3_object_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().s3_object_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_code_hash`.\n"]
    pub fn set_source_code_hash(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source_code_hash = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dead_letter_config`.\n"]
    pub fn set_dead_letter_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunctionDeadLetterConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dead_letter_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dead_letter_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunctionEnvironmentEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().environment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.environment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ephemeral_storage`.\n"]
    pub fn set_ephemeral_storage(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunctionEphemeralStorageEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ephemeral_storage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ephemeral_storage = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `file_system_config`.\n"]
    pub fn set_file_system_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunctionFileSystemConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().file_system_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.file_system_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `image_config`.\n"]
    pub fn set_image_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunctionImageConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().image_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.image_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `snap_start`.\n"]
    pub fn set_snap_start(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunctionSnapStartEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().snap_start = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.snap_start = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<LambdaFunctionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tracing_config`.\n"]
    pub fn set_tracing_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunctionTracingConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tracing_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tracing_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunctionVpcConfigEl>>,
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filename` after provisioning.\n"]
    pub fn filename(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filename", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `package_type` after provisioning.\n"]
    pub fn package_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.package_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publish` after provisioning.\n"]
    pub fn publish(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publish", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qualified_arn` after provisioning.\n"]
    pub fn qualified_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualified_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qualified_invoke_arn` after provisioning.\n"]
    pub fn qualified_invoke_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualified_invoke_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_key` after provisioning.\n"]
    pub fn s3_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_object_version` after provisioning.\n"]
    pub fn s3_object_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_object_version", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> terrars::ListRef<LambdaFunctionDeadLetterConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::ListRef<LambdaFunctionEnvironmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> terrars::ListRef<LambdaFunctionEphemeralStorageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_config` after provisioning.\n"]
    pub fn file_system_config(&self) -> terrars::ListRef<LambdaFunctionFileSystemConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_system_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_config` after provisioning.\n"]
    pub fn image_config(&self) -> terrars::ListRef<LambdaFunctionImageConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snap_start` after provisioning.\n"]
    pub fn snap_start(&self) -> terrars::ListRef<LambdaFunctionSnapStartElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snap_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LambdaFunctionTimeoutsElRef {
        LambdaFunctionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tracing_config` after provisioning.\n"]
    pub fn tracing_config(&self) -> terrars::ListRef<LambdaFunctionTracingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tracing_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<LambdaFunctionVpcConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

impl terrars::Referable for LambdaFunction {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LambdaFunction { }

impl terrars::ToListMappable for LambdaFunction {
    type O = terrars::ListRef<LambdaFunctionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LambdaFunction_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lambda_function".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLambdaFunction {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub function_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role: terrars::PrimField<std::string::String>,
}

impl BuildLambdaFunction {
    pub fn build(self, stack: &mut terrars::Stack) -> LambdaFunction {
        let out = LambdaFunction(std::rc::Rc::new(LambdaFunction_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LambdaFunctionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                architectures: core::default::Default::default(),
                code_signing_config_arn: core::default::Default::default(),
                description: core::default::Default::default(),
                filename: core::default::Default::default(),
                function_name: self.function_name,
                handler: core::default::Default::default(),
                id: core::default::Default::default(),
                image_uri: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                layers: core::default::Default::default(),
                memory_size: core::default::Default::default(),
                package_type: core::default::Default::default(),
                publish: core::default::Default::default(),
                reserved_concurrent_executions: core::default::Default::default(),
                role: self.role,
                runtime: core::default::Default::default(),
                s3_bucket: core::default::Default::default(),
                s3_key: core::default::Default::default(),
                s3_object_version: core::default::Default::default(),
                source_code_hash: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timeout: core::default::Default::default(),
                dead_letter_config: core::default::Default::default(),
                environment: core::default::Default::default(),
                ephemeral_storage: core::default::Default::default(),
                file_system_config: core::default::Default::default(),
                image_config: core::default::Default::default(),
                snap_start: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                tracing_config: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LambdaFunctionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunctionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LambdaFunctionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filename` after provisioning.\n"]
    pub fn filename(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filename", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `package_type` after provisioning.\n"]
    pub fn package_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.package_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publish` after provisioning.\n"]
    pub fn publish(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publish", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qualified_arn` after provisioning.\n"]
    pub fn qualified_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualified_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qualified_invoke_arn` after provisioning.\n"]
    pub fn qualified_invoke_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualified_invoke_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_key` after provisioning.\n"]
    pub fn s3_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_object_version` after provisioning.\n"]
    pub fn s3_object_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_object_version", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> terrars::ListRef<LambdaFunctionDeadLetterConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::ListRef<LambdaFunctionEnvironmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> terrars::ListRef<LambdaFunctionEphemeralStorageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_config` after provisioning.\n"]
    pub fn file_system_config(&self) -> terrars::ListRef<LambdaFunctionFileSystemConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_system_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_config` after provisioning.\n"]
    pub fn image_config(&self) -> terrars::ListRef<LambdaFunctionImageConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snap_start` after provisioning.\n"]
    pub fn snap_start(&self) -> terrars::ListRef<LambdaFunctionSnapStartElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snap_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LambdaFunctionTimeoutsElRef {
        LambdaFunctionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tracing_config` after provisioning.\n"]
    pub fn tracing_config(&self) -> terrars::ListRef<LambdaFunctionTracingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tracing_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<LambdaFunctionVpcConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LambdaFunctionDeadLetterConfigEl {
    target_arn: terrars::PrimField<std::string::String>,
}

impl LambdaFunctionDeadLetterConfigEl { }

impl terrars::ToListMappable for LambdaFunctionDeadLetterConfigEl {
    type O = terrars::BlockAssignable<LambdaFunctionDeadLetterConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunctionDeadLetterConfigEl {
    #[doc = ""]
    pub target_arn: terrars::PrimField<std::string::String>,
}

impl BuildLambdaFunctionDeadLetterConfigEl {
    pub fn build(self) -> LambdaFunctionDeadLetterConfigEl {
        LambdaFunctionDeadLetterConfigEl { target_arn: self.target_arn }
    }
}

pub struct LambdaFunctionDeadLetterConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunctionDeadLetterConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunctionDeadLetterConfigElRef {
        LambdaFunctionDeadLetterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunctionDeadLetterConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_arn` after provisioning.\n"]
    pub fn target_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaFunctionEnvironmentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl LambdaFunctionEnvironmentEl {
    #[doc = "Set the field `variables`.\n"]
    pub fn set_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.variables = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaFunctionEnvironmentEl {
    type O = terrars::BlockAssignable<LambdaFunctionEnvironmentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunctionEnvironmentEl {}

impl BuildLambdaFunctionEnvironmentEl {
    pub fn build(self) -> LambdaFunctionEnvironmentEl {
        LambdaFunctionEnvironmentEl { variables: core::default::Default::default() }
    }
}

pub struct LambdaFunctionEnvironmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunctionEnvironmentElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunctionEnvironmentElRef {
        LambdaFunctionEnvironmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunctionEnvironmentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `variables` after provisioning.\n"]
    pub fn variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.variables", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaFunctionEphemeralStorageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<f64>>,
}

impl LambdaFunctionEphemeralStorageEl {
    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaFunctionEphemeralStorageEl {
    type O = terrars::BlockAssignable<LambdaFunctionEphemeralStorageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunctionEphemeralStorageEl {}

impl BuildLambdaFunctionEphemeralStorageEl {
    pub fn build(self) -> LambdaFunctionEphemeralStorageEl {
        LambdaFunctionEphemeralStorageEl { size: core::default::Default::default() }
    }
}

pub struct LambdaFunctionEphemeralStorageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunctionEphemeralStorageElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunctionEphemeralStorageElRef {
        LambdaFunctionEphemeralStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunctionEphemeralStorageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaFunctionFileSystemConfigEl {
    arn: terrars::PrimField<std::string::String>,
    local_mount_path: terrars::PrimField<std::string::String>,
}

impl LambdaFunctionFileSystemConfigEl { }

impl terrars::ToListMappable for LambdaFunctionFileSystemConfigEl {
    type O = terrars::BlockAssignable<LambdaFunctionFileSystemConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunctionFileSystemConfigEl {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub local_mount_path: terrars::PrimField<std::string::String>,
}

impl BuildLambdaFunctionFileSystemConfigEl {
    pub fn build(self) -> LambdaFunctionFileSystemConfigEl {
        LambdaFunctionFileSystemConfigEl {
            arn: self.arn,
            local_mount_path: self.local_mount_path,
        }
    }
}

pub struct LambdaFunctionFileSystemConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunctionFileSystemConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunctionFileSystemConfigElRef {
        LambdaFunctionFileSystemConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunctionFileSystemConfigElRef {
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
pub struct LambdaFunctionImageConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry_point: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    working_directory: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LambdaFunctionImageConfigEl {
    #[doc = "Set the field `command`.\n"]
    pub fn set_command(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entry_point`.\n"]
    pub fn set_entry_point(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.entry_point = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `working_directory`.\n"]
    pub fn set_working_directory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.working_directory = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaFunctionImageConfigEl {
    type O = terrars::BlockAssignable<LambdaFunctionImageConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunctionImageConfigEl {}

impl BuildLambdaFunctionImageConfigEl {
    pub fn build(self) -> LambdaFunctionImageConfigEl {
        LambdaFunctionImageConfigEl {
            command: core::default::Default::default(),
            entry_point: core::default::Default::default(),
            working_directory: core::default::Default::default(),
        }
    }
}

pub struct LambdaFunctionImageConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunctionImageConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunctionImageConfigElRef {
        LambdaFunctionImageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunctionImageConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc = "Get a reference to the value of field `entry_point` after provisioning.\n"]
    pub fn entry_point(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entry_point", self.base))
    }

    #[doc = "Get a reference to the value of field `working_directory` after provisioning.\n"]
    pub fn working_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.working_directory", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaFunctionSnapStartEl {
    apply_on: terrars::PrimField<std::string::String>,
}

impl LambdaFunctionSnapStartEl { }

impl terrars::ToListMappable for LambdaFunctionSnapStartEl {
    type O = terrars::BlockAssignable<LambdaFunctionSnapStartEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunctionSnapStartEl {
    #[doc = ""]
    pub apply_on: terrars::PrimField<std::string::String>,
}

impl BuildLambdaFunctionSnapStartEl {
    pub fn build(self) -> LambdaFunctionSnapStartEl {
        LambdaFunctionSnapStartEl { apply_on: self.apply_on }
    }
}

pub struct LambdaFunctionSnapStartElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunctionSnapStartElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunctionSnapStartElRef {
        LambdaFunctionSnapStartElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunctionSnapStartElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apply_on` after provisioning.\n"]
    pub fn apply_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apply_on", self.base))
    }

    #[doc = "Get a reference to the value of field `optimization_status` after provisioning.\n"]
    pub fn optimization_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.optimization_status", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaFunctionTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LambdaFunctionTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaFunctionTimeoutsEl {
    type O = terrars::BlockAssignable<LambdaFunctionTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunctionTimeoutsEl {}

impl BuildLambdaFunctionTimeoutsEl {
    pub fn build(self) -> LambdaFunctionTimeoutsEl {
        LambdaFunctionTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct LambdaFunctionTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunctionTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunctionTimeoutsElRef {
        LambdaFunctionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunctionTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaFunctionTracingConfigEl {
    mode: terrars::PrimField<std::string::String>,
}

impl LambdaFunctionTracingConfigEl { }

impl terrars::ToListMappable for LambdaFunctionTracingConfigEl {
    type O = terrars::BlockAssignable<LambdaFunctionTracingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunctionTracingConfigEl {
    #[doc = ""]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildLambdaFunctionTracingConfigEl {
    pub fn build(self) -> LambdaFunctionTracingConfigEl {
        LambdaFunctionTracingConfigEl { mode: self.mode }
    }
}

pub struct LambdaFunctionTracingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunctionTracingConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunctionTracingConfigElRef {
        LambdaFunctionTracingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunctionTracingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaFunctionVpcConfigEl {
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl LambdaFunctionVpcConfigEl { }

impl terrars::ToListMappable for LambdaFunctionVpcConfigEl {
    type O = terrars::BlockAssignable<LambdaFunctionVpcConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunctionVpcConfigEl {
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildLambdaFunctionVpcConfigEl {
    pub fn build(self) -> LambdaFunctionVpcConfigEl {
        LambdaFunctionVpcConfigEl {
            security_group_ids: self.security_group_ids,
            subnet_ids: self.subnet_ids,
        }
    }
}

pub struct LambdaFunctionVpcConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunctionVpcConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunctionVpcConfigElRef {
        LambdaFunctionVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunctionVpcConfigElRef {
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

#[derive(Serialize, Default)]
struct LambdaFunctionDynamic {
    dead_letter_config: std::option::Option<terrars::DynamicBlock<LambdaFunctionDeadLetterConfigEl>>,
    environment: std::option::Option<terrars::DynamicBlock<LambdaFunctionEnvironmentEl>>,
    ephemeral_storage: std::option::Option<terrars::DynamicBlock<LambdaFunctionEphemeralStorageEl>>,
    file_system_config: std::option::Option<terrars::DynamicBlock<LambdaFunctionFileSystemConfigEl>>,
    image_config: std::option::Option<terrars::DynamicBlock<LambdaFunctionImageConfigEl>>,
    snap_start: std::option::Option<terrars::DynamicBlock<LambdaFunctionSnapStartEl>>,
    tracing_config: std::option::Option<terrars::DynamicBlock<LambdaFunctionTracingConfigEl>>,
    vpc_config: std::option::Option<terrars::DynamicBlock<LambdaFunctionVpcConfigEl>>,
}
