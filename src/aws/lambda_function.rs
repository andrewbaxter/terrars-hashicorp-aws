#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LambdaFunctionData_ {
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
    code_sha256: std::option::Option<terrars::PrimField<std::string::String>>,
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
    publish_to: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replace_security_groups_on_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replacement_security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
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
    skip_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_code_hash: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_resource_timeout_for_propagation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_provider_config: std::option::Option<std::vec::Vec<LambdaFunction_CapacityProviderConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dead_letter_config: std::option::Option<std::vec::Vec<LambdaFunction_DeadLetterConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    durable_config: std::option::Option<std::vec::Vec<LambdaFunction_DurableConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<std::vec::Vec<LambdaFunction_Environment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_storage: std::option::Option<std::vec::Vec<LambdaFunction_EphemeralStorage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_config: std::option::Option<std::vec::Vec<LambdaFunction_FileSystemConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_config: std::option::Option<std::vec::Vec<LambdaFunction_ImageConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<LambdaFunction_LoggingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snap_start: std::option::Option<std::vec::Vec<LambdaFunction_SnapStart>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenancy_config: std::option::Option<std::vec::Vec<LambdaFunction_TenancyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LambdaFunction_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tracing_config: std::option::Option<std::vec::Vec<LambdaFunction_TracingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<LambdaFunction_VpcConfig>>,
    dynamic: LambdaFunctionDynamic,
}

struct LambdaFunction_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LambdaFunctionData_>,
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

    #[doc = "Set the field `code_sha256`.\n"]
    pub fn set_code_sha256(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().code_sha256 = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `publish_to`.\n"]
    pub fn set_publish_to(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().publish_to = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replace_security_groups_on_destroy`.\n"]
    pub fn set_replace_security_groups_on_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().replace_security_groups_on_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replacement_security_group_ids`.\n"]
    pub fn set_replacement_security_group_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().replacement_security_group_ids = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `skip_destroy`.\n"]
    pub fn set_skip_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_code_hash`.\n"]
    pub fn set_source_code_hash(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source_code_hash = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_kms_key_arn`.\n"]
    pub fn set_source_kms_key_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source_kms_key_arn = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `use_resource_timeout_for_propagation`.\n"]
    pub fn set_use_resource_timeout_for_propagation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_resource_timeout_for_propagation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_provider_config`.\n"]
    pub fn set_capacity_provider_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunction_CapacityProviderConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity_provider_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capacity_provider_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dead_letter_config`.\n"]
    pub fn set_dead_letter_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunction_DeadLetterConfig>>,
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

    #[doc = "Set the field `durable_config`.\n"]
    pub fn set_durable_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunction_DurableConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().durable_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.durable_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunction_Environment>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunction_EphemeralStorage>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunction_FileSystemConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunction_ImageConfig>>,
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

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunction_LoggingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `snap_start`.\n"]
    pub fn set_snap_start(self, v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunction_SnapStart>>) -> Self {
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

    #[doc = "Set the field `tenancy_config`.\n"]
    pub fn set_tenancy_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunction_TenancyConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tenancy_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tenancy_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<LambdaFunction_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tracing_config`.\n"]
    pub fn set_tracing_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunction_TracingConfig>>,
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
    pub fn set_vpc_config(self, v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunction_VpcConfig>>) -> Self {
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

    #[doc = "Get a reference to the value of field `code_sha256` after provisioning.\n"]
    pub fn code_sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_sha256", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `publish_to` after provisioning.\n"]
    pub fn publish_to(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publish_to", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qualified_arn` after provisioning.\n"]
    pub fn qualified_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualified_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qualified_invoke_arn` after provisioning.\n"]
    pub fn qualified_invoke_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualified_invoke_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replace_security_groups_on_destroy` after provisioning.\n"]
    pub fn replace_security_groups_on_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.replace_security_groups_on_destroy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replacement_security_group_ids` after provisioning.\n"]
    pub fn replacement_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.replacement_security_group_ids", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `skip_destroy` after provisioning.\n"]
    pub fn skip_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_destroy", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_resource_timeout_for_propagation` after provisioning.\n"]
    pub fn use_resource_timeout_for_propagation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_resource_timeout_for_propagation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_provider_config` after provisioning.\n"]
    pub fn capacity_provider_config(&self) -> terrars::ListRef<LambdaFunction_CapacityProviderConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_provider_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> terrars::ListRef<LambdaFunction_DeadLetterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `durable_config` after provisioning.\n"]
    pub fn durable_config(&self) -> terrars::ListRef<LambdaFunction_DurableConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.durable_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::ListRef<LambdaFunction_EnvironmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> terrars::ListRef<LambdaFunction_EphemeralStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_config` after provisioning.\n"]
    pub fn file_system_config(&self) -> terrars::ListRef<LambdaFunction_FileSystemConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_system_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_config` after provisioning.\n"]
    pub fn image_config(&self) -> terrars::ListRef<LambdaFunction_ImageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<LambdaFunction_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snap_start` after provisioning.\n"]
    pub fn snap_start(&self) -> terrars::ListRef<LambdaFunction_SnapStartRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snap_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenancy_config` after provisioning.\n"]
    pub fn tenancy_config(&self) -> terrars::ListRef<LambdaFunction_TenancyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tenancy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LambdaFunction_TimeoutsRef {
        LambdaFunction_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tracing_config` after provisioning.\n"]
    pub fn tracing_config(&self) -> terrars::ListRef<LambdaFunction_TracingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tracing_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<LambdaFunction_VpcConfigRef> {
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
            data: std::cell::RefCell::new(LambdaFunctionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                architectures: core::default::Default::default(),
                code_sha256: core::default::Default::default(),
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
                publish_to: core::default::Default::default(),
                region: core::default::Default::default(),
                replace_security_groups_on_destroy: core::default::Default::default(),
                replacement_security_group_ids: core::default::Default::default(),
                reserved_concurrent_executions: core::default::Default::default(),
                role: self.role,
                runtime: core::default::Default::default(),
                s3_bucket: core::default::Default::default(),
                s3_key: core::default::Default::default(),
                s3_object_version: core::default::Default::default(),
                skip_destroy: core::default::Default::default(),
                source_code_hash: core::default::Default::default(),
                source_kms_key_arn: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timeout: core::default::Default::default(),
                use_resource_timeout_for_propagation: core::default::Default::default(),
                capacity_provider_config: core::default::Default::default(),
                dead_letter_config: core::default::Default::default(),
                durable_config: core::default::Default::default(),
                environment: core::default::Default::default(),
                ephemeral_storage: core::default::Default::default(),
                file_system_config: core::default::Default::default(),
                image_config: core::default::Default::default(),
                logging_config: core::default::Default::default(),
                snap_start: core::default::Default::default(),
                tenancy_config: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `code_sha256` after provisioning.\n"]
    pub fn code_sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_sha256", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `publish_to` after provisioning.\n"]
    pub fn publish_to(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publish_to", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qualified_arn` after provisioning.\n"]
    pub fn qualified_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualified_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qualified_invoke_arn` after provisioning.\n"]
    pub fn qualified_invoke_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualified_invoke_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replace_security_groups_on_destroy` after provisioning.\n"]
    pub fn replace_security_groups_on_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.replace_security_groups_on_destroy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replacement_security_group_ids` after provisioning.\n"]
    pub fn replacement_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.replacement_security_group_ids", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `skip_destroy` after provisioning.\n"]
    pub fn skip_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_destroy", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_resource_timeout_for_propagation` after provisioning.\n"]
    pub fn use_resource_timeout_for_propagation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_resource_timeout_for_propagation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_provider_config` after provisioning.\n"]
    pub fn capacity_provider_config(&self) -> terrars::ListRef<LambdaFunction_CapacityProviderConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_provider_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> terrars::ListRef<LambdaFunction_DeadLetterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `durable_config` after provisioning.\n"]
    pub fn durable_config(&self) -> terrars::ListRef<LambdaFunction_DurableConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.durable_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::ListRef<LambdaFunction_EnvironmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> terrars::ListRef<LambdaFunction_EphemeralStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_config` after provisioning.\n"]
    pub fn file_system_config(&self) -> terrars::ListRef<LambdaFunction_FileSystemConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_system_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_config` after provisioning.\n"]
    pub fn image_config(&self) -> terrars::ListRef<LambdaFunction_ImageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<LambdaFunction_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snap_start` after provisioning.\n"]
    pub fn snap_start(&self) -> terrars::ListRef<LambdaFunction_SnapStartRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snap_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenancy_config` after provisioning.\n"]
    pub fn tenancy_config(&self) -> terrars::ListRef<LambdaFunction_TenancyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tenancy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LambdaFunction_TimeoutsRef {
        LambdaFunction_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tracing_config` after provisioning.\n"]
    pub fn tracing_config(&self) -> terrars::ListRef<LambdaFunction_TracingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tracing_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<LambdaFunction_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct LambdaFunctionDynamic {
    capacity_provider_config: std::option::Option<terrars::DynamicBlock<LambdaFunction_CapacityProviderConfig>>,
    dead_letter_config: std::option::Option<terrars::DynamicBlock<LambdaFunction_DeadLetterConfig>>,
    durable_config: std::option::Option<terrars::DynamicBlock<LambdaFunction_DurableConfig>>,
    environment: std::option::Option<terrars::DynamicBlock<LambdaFunction_Environment>>,
    ephemeral_storage: std::option::Option<terrars::DynamicBlock<LambdaFunction_EphemeralStorage>>,
    file_system_config: std::option::Option<terrars::DynamicBlock<LambdaFunction_FileSystemConfig>>,
    image_config: std::option::Option<terrars::DynamicBlock<LambdaFunction_ImageConfig>>,
    logging_config: std::option::Option<terrars::DynamicBlock<LambdaFunction_LoggingConfig>>,
    snap_start: std::option::Option<terrars::DynamicBlock<LambdaFunction_SnapStart>>,
    tenancy_config: std::option::Option<terrars::DynamicBlock<LambdaFunction_TenancyConfig>>,
    tracing_config: std::option::Option<terrars::DynamicBlock<LambdaFunction_TracingConfig>>,
    vpc_config: std::option::Option<terrars::DynamicBlock<LambdaFunction_VpcConfig>>,
}

#[derive(Serialize)]
pub struct LambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfig {
    capacity_provider_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_environment_memory_gib_per_vcpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_execution_environment_max_concurrency: std::option::Option<terrars::PrimField<f64>>,
}

impl LambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfig {
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

impl terrars::ToListMappable for LambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfig {
    type O =
        terrars::BlockAssignable<LambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfig {
    #[doc = ""]
    pub capacity_provider_arn: terrars::PrimField<std::string::String>,
}

impl BuildLambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfig {
    pub fn build(self) -> LambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfig {
        LambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfig {
            capacity_provider_arn: self.capacity_provider_arn,
            execution_environment_memory_gib_per_vcpu: core::default::Default::default(),
            per_execution_environment_max_concurrency: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfigRef {
        LambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfigRef {
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

#[derive(Serialize, Default)]
struct LambdaFunction_CapacityProviderConfigDynamic {
    lambda_managed_instances_capacity_provider_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<LambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfig>,
    >,
}

#[derive(Serialize)]
pub struct LambdaFunction_CapacityProviderConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_managed_instances_capacity_provider_config: std
    ::option
    ::Option<std::vec::Vec<LambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfig>>,
    dynamic: LambdaFunction_CapacityProviderConfigDynamic,
}

impl LambdaFunction_CapacityProviderConfig {
    #[doc = "Set the field `lambda_managed_instances_capacity_provider_config`.\n"]
    pub fn set_lambda_managed_instances_capacity_provider_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            LambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda_managed_instances_capacity_provider_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_managed_instances_capacity_provider_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LambdaFunction_CapacityProviderConfig {
    type O = terrars::BlockAssignable<LambdaFunction_CapacityProviderConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunction_CapacityProviderConfig {}

impl BuildLambdaFunction_CapacityProviderConfig {
    pub fn build(self) -> LambdaFunction_CapacityProviderConfig {
        LambdaFunction_CapacityProviderConfig {
            lambda_managed_instances_capacity_provider_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaFunction_CapacityProviderConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunction_CapacityProviderConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunction_CapacityProviderConfigRef {
        LambdaFunction_CapacityProviderConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunction_CapacityProviderConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `lambda_managed_instances_capacity_provider_config` after provisioning.\n"]
    pub fn lambda_managed_instances_capacity_provider_config(
        &self,
    ) -> terrars::ListRef<LambdaFunction_CapacityProviderConfig_LambdaManagedInstancesCapacityProviderConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.lambda_managed_instances_capacity_provider_config", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct LambdaFunction_DeadLetterConfig {
    target_arn: terrars::PrimField<std::string::String>,
}

impl LambdaFunction_DeadLetterConfig { }

impl terrars::ToListMappable for LambdaFunction_DeadLetterConfig {
    type O = terrars::BlockAssignable<LambdaFunction_DeadLetterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunction_DeadLetterConfig {
    #[doc = ""]
    pub target_arn: terrars::PrimField<std::string::String>,
}

impl BuildLambdaFunction_DeadLetterConfig {
    pub fn build(self) -> LambdaFunction_DeadLetterConfig {
        LambdaFunction_DeadLetterConfig { target_arn: self.target_arn }
    }
}

#[allow(dead_code)]
pub struct LambdaFunction_DeadLetterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunction_DeadLetterConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunction_DeadLetterConfigRef {
        LambdaFunction_DeadLetterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunction_DeadLetterConfigRef {
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
pub struct LambdaFunction_DurableConfig {
    execution_timeout: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_period: std::option::Option<terrars::PrimField<f64>>,
}

impl LambdaFunction_DurableConfig {
    #[doc = "Set the field `retention_period`.\n"]
    pub fn set_retention_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retention_period = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaFunction_DurableConfig {
    type O = terrars::BlockAssignable<LambdaFunction_DurableConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunction_DurableConfig {
    #[doc = ""]
    pub execution_timeout: terrars::PrimField<f64>,
}

impl BuildLambdaFunction_DurableConfig {
    pub fn build(self) -> LambdaFunction_DurableConfig {
        LambdaFunction_DurableConfig {
            execution_timeout: self.execution_timeout,
            retention_period: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaFunction_DurableConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunction_DurableConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunction_DurableConfigRef {
        LambdaFunction_DurableConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunction_DurableConfigRef {
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
pub struct LambdaFunction_Environment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl LambdaFunction_Environment {
    #[doc = "Set the field `variables`.\n"]
    pub fn set_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.variables = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaFunction_Environment {
    type O = terrars::BlockAssignable<LambdaFunction_Environment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunction_Environment {}

impl BuildLambdaFunction_Environment {
    pub fn build(self) -> LambdaFunction_Environment {
        LambdaFunction_Environment { variables: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct LambdaFunction_EnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunction_EnvironmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunction_EnvironmentRef {
        LambdaFunction_EnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunction_EnvironmentRef {
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
pub struct LambdaFunction_EphemeralStorage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<f64>>,
}

impl LambdaFunction_EphemeralStorage {
    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaFunction_EphemeralStorage {
    type O = terrars::BlockAssignable<LambdaFunction_EphemeralStorage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunction_EphemeralStorage {}

impl BuildLambdaFunction_EphemeralStorage {
    pub fn build(self) -> LambdaFunction_EphemeralStorage {
        LambdaFunction_EphemeralStorage { size: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct LambdaFunction_EphemeralStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunction_EphemeralStorageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunction_EphemeralStorageRef {
        LambdaFunction_EphemeralStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunction_EphemeralStorageRef {
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
pub struct LambdaFunction_FileSystemConfig {
    arn: terrars::PrimField<std::string::String>,
    local_mount_path: terrars::PrimField<std::string::String>,
}

impl LambdaFunction_FileSystemConfig { }

impl terrars::ToListMappable for LambdaFunction_FileSystemConfig {
    type O = terrars::BlockAssignable<LambdaFunction_FileSystemConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunction_FileSystemConfig {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub local_mount_path: terrars::PrimField<std::string::String>,
}

impl BuildLambdaFunction_FileSystemConfig {
    pub fn build(self) -> LambdaFunction_FileSystemConfig {
        LambdaFunction_FileSystemConfig {
            arn: self.arn,
            local_mount_path: self.local_mount_path,
        }
    }
}

#[allow(dead_code)]
pub struct LambdaFunction_FileSystemConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunction_FileSystemConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunction_FileSystemConfigRef {
        LambdaFunction_FileSystemConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunction_FileSystemConfigRef {
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
pub struct LambdaFunction_ImageConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry_point: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    working_directory: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LambdaFunction_ImageConfig {
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

impl terrars::ToListMappable for LambdaFunction_ImageConfig {
    type O = terrars::BlockAssignable<LambdaFunction_ImageConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunction_ImageConfig {}

impl BuildLambdaFunction_ImageConfig {
    pub fn build(self) -> LambdaFunction_ImageConfig {
        LambdaFunction_ImageConfig {
            command: core::default::Default::default(),
            entry_point: core::default::Default::default(),
            working_directory: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaFunction_ImageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunction_ImageConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunction_ImageConfigRef {
        LambdaFunction_ImageConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunction_ImageConfigRef {
    #[allow(dead_code)]
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
pub struct LambdaFunction_LoggingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_log_level: std::option::Option<terrars::PrimField<std::string::String>>,
    log_format: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system_log_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LambdaFunction_LoggingConfig {
    #[doc = "Set the field `application_log_level`.\n"]
    pub fn set_application_log_level(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.application_log_level = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for LambdaFunction_LoggingConfig {
    type O = terrars::BlockAssignable<LambdaFunction_LoggingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunction_LoggingConfig {
    #[doc = ""]
    pub log_format: terrars::PrimField<std::string::String>,
}

impl BuildLambdaFunction_LoggingConfig {
    pub fn build(self) -> LambdaFunction_LoggingConfig {
        LambdaFunction_LoggingConfig {
            application_log_level: core::default::Default::default(),
            log_format: self.log_format,
            log_group: core::default::Default::default(),
            system_log_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaFunction_LoggingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunction_LoggingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunction_LoggingConfigRef {
        LambdaFunction_LoggingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunction_LoggingConfigRef {
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
pub struct LambdaFunction_SnapStart {
    apply_on: terrars::PrimField<std::string::String>,
}

impl LambdaFunction_SnapStart { }

impl terrars::ToListMappable for LambdaFunction_SnapStart {
    type O = terrars::BlockAssignable<LambdaFunction_SnapStart>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunction_SnapStart {
    #[doc = ""]
    pub apply_on: terrars::PrimField<std::string::String>,
}

impl BuildLambdaFunction_SnapStart {
    pub fn build(self) -> LambdaFunction_SnapStart {
        LambdaFunction_SnapStart { apply_on: self.apply_on }
    }
}

#[allow(dead_code)]
pub struct LambdaFunction_SnapStartRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunction_SnapStartRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunction_SnapStartRef {
        LambdaFunction_SnapStartRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunction_SnapStartRef {
    #[allow(dead_code)]
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
pub struct LambdaFunction_TenancyConfig {
    tenant_isolation_mode: terrars::PrimField<std::string::String>,
}

impl LambdaFunction_TenancyConfig { }

impl terrars::ToListMappable for LambdaFunction_TenancyConfig {
    type O = terrars::BlockAssignable<LambdaFunction_TenancyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunction_TenancyConfig {
    #[doc = ""]
    pub tenant_isolation_mode: terrars::PrimField<std::string::String>,
}

impl BuildLambdaFunction_TenancyConfig {
    pub fn build(self) -> LambdaFunction_TenancyConfig {
        LambdaFunction_TenancyConfig { tenant_isolation_mode: self.tenant_isolation_mode }
    }
}

#[allow(dead_code)]
pub struct LambdaFunction_TenancyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunction_TenancyConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunction_TenancyConfigRef {
        LambdaFunction_TenancyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunction_TenancyConfigRef {
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
pub struct LambdaFunction_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LambdaFunction_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaFunction_Timeouts {
    type O = terrars::BlockAssignable<LambdaFunction_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunction_Timeouts {}

impl BuildLambdaFunction_Timeouts {
    pub fn build(self) -> LambdaFunction_Timeouts {
        LambdaFunction_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaFunction_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunction_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunction_TimeoutsRef {
        LambdaFunction_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunction_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaFunction_TracingConfig {
    mode: terrars::PrimField<std::string::String>,
}

impl LambdaFunction_TracingConfig { }

impl terrars::ToListMappable for LambdaFunction_TracingConfig {
    type O = terrars::BlockAssignable<LambdaFunction_TracingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunction_TracingConfig {
    #[doc = ""]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildLambdaFunction_TracingConfig {
    pub fn build(self) -> LambdaFunction_TracingConfig {
        LambdaFunction_TracingConfig { mode: self.mode }
    }
}

#[allow(dead_code)]
pub struct LambdaFunction_TracingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunction_TracingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunction_TracingConfigRef {
        LambdaFunction_TracingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunction_TracingConfigRef {
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
pub struct LambdaFunction_VpcConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_allowed_for_dual_stack: std::option::Option<terrars::PrimField<bool>>,
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl LambdaFunction_VpcConfig {
    #[doc = "Set the field `ipv6_allowed_for_dual_stack`.\n"]
    pub fn set_ipv6_allowed_for_dual_stack(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ipv6_allowed_for_dual_stack = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaFunction_VpcConfig {
    type O = terrars::BlockAssignable<LambdaFunction_VpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunction_VpcConfig {
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildLambdaFunction_VpcConfig {
    pub fn build(self) -> LambdaFunction_VpcConfig {
        LambdaFunction_VpcConfig {
            ipv6_allowed_for_dual_stack: core::default::Default::default(),
            security_group_ids: self.security_group_ids,
            subnet_ids: self.subnet_ids,
        }
    }
}

#[allow(dead_code)]
pub struct LambdaFunction_VpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunction_VpcConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunction_VpcConfigRef {
        LambdaFunction_VpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunction_VpcConfigRef {
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
