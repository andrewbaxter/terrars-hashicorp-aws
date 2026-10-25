#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LambdaEventSourceMappingData_ {
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
    batch_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bisect_batch_on_function_error: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_source_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    function_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function_response_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_batching_window_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_record_age_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_retry_attempts: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallelization_factor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queues: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    starting_position: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    starting_position_timestamp: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topics: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tumbling_window_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_resource_timeout_for_propagation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_managed_kafka_event_source_config: std
    ::option
    ::Option<std::vec::Vec<LambdaEventSourceMapping_AmazonManagedKafkaEventSourceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_config: std::option::Option<std::vec::Vec<LambdaEventSourceMapping_DestinationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_db_event_source_config: std
    ::option
    ::Option<std::vec::Vec<LambdaEventSourceMapping_DocumentDbEventSourceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_criteria: std::option::Option<std::vec::Vec<LambdaEventSourceMapping_FilterCriteria>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metrics_config: std::option::Option<std::vec::Vec<LambdaEventSourceMapping_MetricsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_poller_config: std::option::Option<std::vec::Vec<LambdaEventSourceMapping_ProvisionedPollerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_config: std::option::Option<std::vec::Vec<LambdaEventSourceMapping_ScalingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_managed_event_source: std::option::Option<std::vec::Vec<LambdaEventSourceMapping_SelfManagedEventSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_managed_kafka_event_source_config: std
    ::option
    ::Option<std::vec::Vec<LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_access_configuration: std::option::Option<std::vec::Vec<LambdaEventSourceMapping_SourceAccessConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LambdaEventSourceMapping_Timeouts>,
    dynamic: LambdaEventSourceMappingDynamic,
}

struct LambdaEventSourceMapping_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LambdaEventSourceMappingData_>,
}

#[derive(Clone)]
pub struct LambdaEventSourceMapping(std::rc::Rc<LambdaEventSourceMapping_>);

impl LambdaEventSourceMapping {
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

    #[doc = "Set the field `batch_size`.\n"]
    pub fn set_batch_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().batch_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bisect_batch_on_function_error`.\n"]
    pub fn set_bisect_batch_on_function_error(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().bisect_batch_on_function_error = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event_source_arn`.\n"]
    pub fn set_event_source_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().event_source_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `function_response_types`.\n"]
    pub fn set_function_response_types(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().function_response_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_batching_window_in_seconds`.\n"]
    pub fn set_maximum_batching_window_in_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().maximum_batching_window_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_record_age_in_seconds`.\n"]
    pub fn set_maximum_record_age_in_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().maximum_record_age_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_retry_attempts`.\n"]
    pub fn set_maximum_retry_attempts(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().maximum_retry_attempts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parallelization_factor`.\n"]
    pub fn set_parallelization_factor(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().parallelization_factor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `queues`.\n"]
    pub fn set_queues(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().queues = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `starting_position`.\n"]
    pub fn set_starting_position(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().starting_position = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `starting_position_timestamp`.\n"]
    pub fn set_starting_position_timestamp(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().starting_position_timestamp = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `topics`.\n"]
    pub fn set_topics(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().topics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tumbling_window_in_seconds`.\n"]
    pub fn set_tumbling_window_in_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tumbling_window_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_resource_timeout_for_propagation`.\n"]
    pub fn set_use_resource_timeout_for_propagation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_resource_timeout_for_propagation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `amazon_managed_kafka_event_source_config`.\n"]
    pub fn set_amazon_managed_kafka_event_source_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<LambdaEventSourceMapping_AmazonManagedKafkaEventSourceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().amazon_managed_kafka_event_source_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.amazon_managed_kafka_event_source_config =
                    std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destination_config`.\n"]
    pub fn set_destination_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaEventSourceMapping_DestinationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `document_db_event_source_config`.\n"]
    pub fn set_document_db_event_source_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaEventSourceMapping_DocumentDbEventSourceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().document_db_event_source_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.document_db_event_source_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter_criteria`.\n"]
    pub fn set_filter_criteria(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaEventSourceMapping_FilterCriteria>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter_criteria = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter_criteria = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metrics_config`.\n"]
    pub fn set_metrics_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaEventSourceMapping_MetricsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metrics_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metrics_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `provisioned_poller_config`.\n"]
    pub fn set_provisioned_poller_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaEventSourceMapping_ProvisionedPollerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().provisioned_poller_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.provisioned_poller_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scaling_config`.\n"]
    pub fn set_scaling_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaEventSourceMapping_ScalingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scaling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scaling_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `self_managed_event_source`.\n"]
    pub fn set_self_managed_event_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaEventSourceMapping_SelfManagedEventSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().self_managed_event_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.self_managed_event_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `self_managed_kafka_event_source_config`.\n"]
    pub fn set_self_managed_kafka_event_source_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().self_managed_kafka_event_source_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.self_managed_kafka_event_source_config =
                    std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_access_configuration`.\n"]
    pub fn set_source_access_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaEventSourceMapping_SourceAccessConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_access_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_access_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<LambdaEventSourceMapping_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bisect_batch_on_function_error` after provisioning.\n"]
    pub fn bisect_batch_on_function_error(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bisect_batch_on_function_error", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_source_arn` after provisioning.\n"]
    pub fn event_source_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_source_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_response_types` after provisioning.\n"]
    pub fn function_response_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.function_response_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_processing_result` after provisioning.\n"]
    pub fn last_processing_result(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_processing_result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_batching_window_in_seconds` after provisioning.\n"]
    pub fn maximum_batching_window_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_batching_window_in_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `maximum_record_age_in_seconds` after provisioning.\n"]
    pub fn maximum_record_age_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_record_age_in_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `maximum_retry_attempts` after provisioning.\n"]
    pub fn maximum_retry_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_retry_attempts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parallelization_factor` after provisioning.\n"]
    pub fn parallelization_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parallelization_factor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queues` after provisioning.\n"]
    pub fn queues(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.queues", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `starting_position` after provisioning.\n"]
    pub fn starting_position(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.starting_position", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `starting_position_timestamp` after provisioning.\n"]
    pub fn starting_position_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.starting_position_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state_transition_reason` after provisioning.\n"]
    pub fn state_transition_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_transition_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `topics` after provisioning.\n"]
    pub fn topics(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.topics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tumbling_window_in_seconds` after provisioning.\n"]
    pub fn tumbling_window_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tumbling_window_in_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_resource_timeout_for_propagation` after provisioning.\n"]
    pub fn use_resource_timeout_for_propagation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_resource_timeout_for_propagation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `amazon_managed_kafka_event_source_config` after provisioning.\n"]
    pub fn amazon_managed_kafka_event_source_config(
        &self,
    ) -> terrars::ListRef<LambdaEventSourceMapping_AmazonManagedKafkaEventSourceConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.amazon_managed_kafka_event_source_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `destination_config` after provisioning.\n"]
    pub fn destination_config(&self) -> terrars::ListRef<LambdaEventSourceMapping_DestinationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `document_db_event_source_config` after provisioning.\n"]
    pub fn document_db_event_source_config(
        &self,
    ) -> terrars::ListRef<LambdaEventSourceMapping_DocumentDbEventSourceConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.document_db_event_source_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filter_criteria` after provisioning.\n"]
    pub fn filter_criteria(&self) -> terrars::ListRef<LambdaEventSourceMapping_FilterCriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_criteria", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metrics_config` after provisioning.\n"]
    pub fn metrics_config(&self) -> terrars::ListRef<LambdaEventSourceMapping_MetricsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metrics_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioned_poller_config` after provisioning.\n"]
    pub fn provisioned_poller_config(&self) -> terrars::ListRef<LambdaEventSourceMapping_ProvisionedPollerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provisioned_poller_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> terrars::ListRef<LambdaEventSourceMapping_ScalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_managed_event_source` after provisioning.\n"]
    pub fn self_managed_event_source(&self) -> terrars::ListRef<LambdaEventSourceMapping_SelfManagedEventSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_managed_event_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_managed_kafka_event_source_config` after provisioning.\n"]
    pub fn self_managed_kafka_event_source_config(
        &self,
    ) -> terrars::ListRef<LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.self_managed_kafka_event_source_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LambdaEventSourceMapping_TimeoutsRef {
        LambdaEventSourceMapping_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for LambdaEventSourceMapping {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LambdaEventSourceMapping { }

impl terrars::ToListMappable for LambdaEventSourceMapping {
    type O = terrars::ListRef<LambdaEventSourceMappingRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LambdaEventSourceMapping_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lambda_event_source_mapping".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLambdaEventSourceMapping {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub function_name: terrars::PrimField<std::string::String>,
}

impl BuildLambdaEventSourceMapping {
    pub fn build(self, stack: &mut terrars::Stack) -> LambdaEventSourceMapping {
        let out = LambdaEventSourceMapping(std::rc::Rc::new(LambdaEventSourceMapping_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LambdaEventSourceMappingData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                batch_size: core::default::Default::default(),
                bisect_batch_on_function_error: core::default::Default::default(),
                enabled: core::default::Default::default(),
                event_source_arn: core::default::Default::default(),
                function_name: self.function_name,
                function_response_types: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                maximum_batching_window_in_seconds: core::default::Default::default(),
                maximum_record_age_in_seconds: core::default::Default::default(),
                maximum_retry_attempts: core::default::Default::default(),
                parallelization_factor: core::default::Default::default(),
                queues: core::default::Default::default(),
                region: core::default::Default::default(),
                starting_position: core::default::Default::default(),
                starting_position_timestamp: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                topics: core::default::Default::default(),
                tumbling_window_in_seconds: core::default::Default::default(),
                use_resource_timeout_for_propagation: core::default::Default::default(),
                amazon_managed_kafka_event_source_config: core::default::Default::default(),
                destination_config: core::default::Default::default(),
                document_db_event_source_config: core::default::Default::default(),
                filter_criteria: core::default::Default::default(),
                metrics_config: core::default::Default::default(),
                provisioned_poller_config: core::default::Default::default(),
                scaling_config: core::default::Default::default(),
                self_managed_event_source: core::default::Default::default(),
                self_managed_kafka_event_source_config: core::default::Default::default(),
                source_access_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LambdaEventSourceMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMappingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LambdaEventSourceMappingRef {
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

    #[doc = "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bisect_batch_on_function_error` after provisioning.\n"]
    pub fn bisect_batch_on_function_error(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bisect_batch_on_function_error", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_source_arn` after provisioning.\n"]
    pub fn event_source_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_source_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_response_types` after provisioning.\n"]
    pub fn function_response_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.function_response_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_processing_result` after provisioning.\n"]
    pub fn last_processing_result(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_processing_result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_batching_window_in_seconds` after provisioning.\n"]
    pub fn maximum_batching_window_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_batching_window_in_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `maximum_record_age_in_seconds` after provisioning.\n"]
    pub fn maximum_record_age_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_record_age_in_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `maximum_retry_attempts` after provisioning.\n"]
    pub fn maximum_retry_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_retry_attempts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parallelization_factor` after provisioning.\n"]
    pub fn parallelization_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parallelization_factor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queues` after provisioning.\n"]
    pub fn queues(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.queues", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `starting_position` after provisioning.\n"]
    pub fn starting_position(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.starting_position", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `starting_position_timestamp` after provisioning.\n"]
    pub fn starting_position_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.starting_position_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state_transition_reason` after provisioning.\n"]
    pub fn state_transition_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_transition_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `topics` after provisioning.\n"]
    pub fn topics(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.topics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tumbling_window_in_seconds` after provisioning.\n"]
    pub fn tumbling_window_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tumbling_window_in_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_resource_timeout_for_propagation` after provisioning.\n"]
    pub fn use_resource_timeout_for_propagation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_resource_timeout_for_propagation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `amazon_managed_kafka_event_source_config` after provisioning.\n"]
    pub fn amazon_managed_kafka_event_source_config(
        &self,
    ) -> terrars::ListRef<LambdaEventSourceMapping_AmazonManagedKafkaEventSourceConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.amazon_managed_kafka_event_source_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `destination_config` after provisioning.\n"]
    pub fn destination_config(&self) -> terrars::ListRef<LambdaEventSourceMapping_DestinationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `document_db_event_source_config` after provisioning.\n"]
    pub fn document_db_event_source_config(
        &self,
    ) -> terrars::ListRef<LambdaEventSourceMapping_DocumentDbEventSourceConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.document_db_event_source_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filter_criteria` after provisioning.\n"]
    pub fn filter_criteria(&self) -> terrars::ListRef<LambdaEventSourceMapping_FilterCriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_criteria", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metrics_config` after provisioning.\n"]
    pub fn metrics_config(&self) -> terrars::ListRef<LambdaEventSourceMapping_MetricsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metrics_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioned_poller_config` after provisioning.\n"]
    pub fn provisioned_poller_config(&self) -> terrars::ListRef<LambdaEventSourceMapping_ProvisionedPollerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provisioned_poller_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> terrars::ListRef<LambdaEventSourceMapping_ScalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_managed_event_source` after provisioning.\n"]
    pub fn self_managed_event_source(&self) -> terrars::ListRef<LambdaEventSourceMapping_SelfManagedEventSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_managed_event_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_managed_kafka_event_source_config` after provisioning.\n"]
    pub fn self_managed_kafka_event_source_config(
        &self,
    ) -> terrars::ListRef<LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.self_managed_kafka_event_source_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LambdaEventSourceMapping_TimeoutsRef {
        LambdaEventSourceMapping_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct LambdaEventSourceMappingDynamic {
    amazon_managed_kafka_event_source_config: std
    ::option
    ::Option<terrars::DynamicBlock<LambdaEventSourceMapping_AmazonManagedKafkaEventSourceConfig>>,
    destination_config: std::option::Option<terrars::DynamicBlock<LambdaEventSourceMapping_DestinationConfig>>,
    document_db_event_source_config: std
    ::option
    ::Option<terrars::DynamicBlock<LambdaEventSourceMapping_DocumentDbEventSourceConfig>>,
    filter_criteria: std::option::Option<terrars::DynamicBlock<LambdaEventSourceMapping_FilterCriteria>>,
    metrics_config: std::option::Option<terrars::DynamicBlock<LambdaEventSourceMapping_MetricsConfig>>,
    provisioned_poller_config: std
    ::option
    ::Option<terrars::DynamicBlock<LambdaEventSourceMapping_ProvisionedPollerConfig>>,
    scaling_config: std::option::Option<terrars::DynamicBlock<LambdaEventSourceMapping_ScalingConfig>>,
    self_managed_event_source: std
    ::option
    ::Option<terrars::DynamicBlock<LambdaEventSourceMapping_SelfManagedEventSource>>,
    self_managed_kafka_event_source_config: std
    ::option
    ::Option<terrars::DynamicBlock<LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig>>,
    source_access_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<LambdaEventSourceMapping_SourceAccessConfiguration>>,
}

#[derive(Serialize)]
pub struct LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_AccessConfig {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_AccessConfig {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_AccessConfig {
    type O =
        terrars
        ::BlockAssignable<
            LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_AccessConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_AccessConfig {}

impl BuildLambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_AccessConfig {
    pub fn build(
        self,
    ) -> LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_AccessConfig {
        LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_AccessConfig {
            type_: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_AccessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_AccessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_AccessConfigRef {
        LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_AccessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_AccessConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_SchemaValidationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_SchemaValidationConfig {
    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_SchemaValidationConfig {
    type O =
        terrars
        ::BlockAssignable<
            LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_SchemaValidationConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_SchemaValidationConfig {}

impl BuildLambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_SchemaValidationConfig {
    pub fn build(
        self,
    ) -> LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_SchemaValidationConfig {
        LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_SchemaValidationConfig {
            attribute: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_SchemaValidationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_SchemaValidationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_SchemaValidationConfigRef {
        LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_SchemaValidationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_SchemaValidationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }
}

#[derive(Serialize, Default)]
struct LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfigDynamic {
    access_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_AccessConfig>,
    >,
    schema_validation_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_SchemaValidationConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_record_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_registry_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_config: std
    ::option
    ::Option<
        std::vec::Vec<LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_AccessConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_validation_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_SchemaValidationConfig>,
    >,
    dynamic: LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfigDynamic,
}

impl LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig {
    #[doc = "Set the field `event_record_format`.\n"]
    pub fn set_event_record_format(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.event_record_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema_registry_uri`.\n"]
    pub fn set_schema_registry_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.schema_registry_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_config`.\n"]
    pub fn set_access_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_AccessConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schema_validation_config`.\n"]
    pub fn set_schema_validation_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig_SchemaValidationConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema_validation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_validation_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig {
    type O =
        terrars::BlockAssignable<LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig {}

impl BuildLambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig {
    pub fn build(self) -> LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig {
        LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig {
            event_record_format: core::default::Default::default(),
            schema_registry_uri: core::default::Default::default(),
            access_config: core::default::Default::default(),
            schema_validation_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfigRef {
        LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_record_format` after provisioning.\n"]
    pub fn event_record_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_record_format", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_registry_uri` after provisioning.\n"]
    pub fn schema_registry_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema_registry_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct LambdaEventSourceMapping_AmazonManagedKafkaEventSourceConfigDynamic {
    schema_registry_config: std
    ::option
    ::Option<terrars::DynamicBlock<LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig>>,
}

#[derive(Serialize)]
pub struct LambdaEventSourceMapping_AmazonManagedKafkaEventSourceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_registry_config: std
    ::option
    ::Option<std::vec::Vec<LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig>>,
    dynamic: LambdaEventSourceMapping_AmazonManagedKafkaEventSourceConfigDynamic,
}

impl LambdaEventSourceMapping_AmazonManagedKafkaEventSourceConfig {
    #[doc = "Set the field `consumer_group_id`.\n"]
    pub fn set_consumer_group_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.consumer_group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema_registry_config`.\n"]
    pub fn set_schema_registry_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema_registry_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_registry_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LambdaEventSourceMapping_AmazonManagedKafkaEventSourceConfig {
    type O = terrars::BlockAssignable<LambdaEventSourceMapping_AmazonManagedKafkaEventSourceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMapping_AmazonManagedKafkaEventSourceConfig {}

impl BuildLambdaEventSourceMapping_AmazonManagedKafkaEventSourceConfig {
    pub fn build(self) -> LambdaEventSourceMapping_AmazonManagedKafkaEventSourceConfig {
        LambdaEventSourceMapping_AmazonManagedKafkaEventSourceConfig {
            consumer_group_id: core::default::Default::default(),
            schema_registry_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaEventSourceMapping_AmazonManagedKafkaEventSourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMapping_AmazonManagedKafkaEventSourceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaEventSourceMapping_AmazonManagedKafkaEventSourceConfigRef {
        LambdaEventSourceMapping_AmazonManagedKafkaEventSourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMapping_AmazonManagedKafkaEventSourceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `consumer_group_id` after provisioning.\n"]
    pub fn consumer_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_registry_config` after provisioning.\n"]
    pub fn schema_registry_config(
        &self,
    ) -> terrars::ListRef<LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_registry_config", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMapping_DestinationConfig_OnFailure {
    destination_arn: terrars::PrimField<std::string::String>,
}

impl LambdaEventSourceMapping_DestinationConfig_OnFailure { }

impl terrars::ToListMappable for LambdaEventSourceMapping_DestinationConfig_OnFailure {
    type O = terrars::BlockAssignable<LambdaEventSourceMapping_DestinationConfig_OnFailure>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMapping_DestinationConfig_OnFailure {
    #[doc = ""]
    pub destination_arn: terrars::PrimField<std::string::String>,
}

impl BuildLambdaEventSourceMapping_DestinationConfig_OnFailure {
    pub fn build(self) -> LambdaEventSourceMapping_DestinationConfig_OnFailure {
        LambdaEventSourceMapping_DestinationConfig_OnFailure { destination_arn: self.destination_arn }
    }
}

#[allow(dead_code)]
pub struct LambdaEventSourceMapping_DestinationConfig_OnFailureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMapping_DestinationConfig_OnFailureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaEventSourceMapping_DestinationConfig_OnFailureRef {
        LambdaEventSourceMapping_DestinationConfig_OnFailureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMapping_DestinationConfig_OnFailureRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_arn` after provisioning.\n"]
    pub fn destination_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct LambdaEventSourceMapping_DestinationConfigDynamic {
    on_failure: std::option::Option<terrars::DynamicBlock<LambdaEventSourceMapping_DestinationConfig_OnFailure>>,
}

#[derive(Serialize)]
pub struct LambdaEventSourceMapping_DestinationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_failure: std::option::Option<std::vec::Vec<LambdaEventSourceMapping_DestinationConfig_OnFailure>>,
    dynamic: LambdaEventSourceMapping_DestinationConfigDynamic,
}

impl LambdaEventSourceMapping_DestinationConfig {
    #[doc = "Set the field `on_failure`.\n"]
    pub fn set_on_failure(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaEventSourceMapping_DestinationConfig_OnFailure>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.on_failure = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.on_failure = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LambdaEventSourceMapping_DestinationConfig {
    type O = terrars::BlockAssignable<LambdaEventSourceMapping_DestinationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMapping_DestinationConfig {}

impl BuildLambdaEventSourceMapping_DestinationConfig {
    pub fn build(self) -> LambdaEventSourceMapping_DestinationConfig {
        LambdaEventSourceMapping_DestinationConfig {
            on_failure: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaEventSourceMapping_DestinationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMapping_DestinationConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaEventSourceMapping_DestinationConfigRef {
        LambdaEventSourceMapping_DestinationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMapping_DestinationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `on_failure` after provisioning.\n"]
    pub fn on_failure(&self) -> terrars::ListRef<LambdaEventSourceMapping_DestinationConfig_OnFailureRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_failure", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMapping_DocumentDbEventSourceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collection_name: std::option::Option<terrars::PrimField<std::string::String>>,
    database_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    full_document: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LambdaEventSourceMapping_DocumentDbEventSourceConfig {
    #[doc = "Set the field `collection_name`.\n"]
    pub fn set_collection_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.collection_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `full_document`.\n"]
    pub fn set_full_document(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.full_document = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaEventSourceMapping_DocumentDbEventSourceConfig {
    type O = terrars::BlockAssignable<LambdaEventSourceMapping_DocumentDbEventSourceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMapping_DocumentDbEventSourceConfig {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
}

impl BuildLambdaEventSourceMapping_DocumentDbEventSourceConfig {
    pub fn build(self) -> LambdaEventSourceMapping_DocumentDbEventSourceConfig {
        LambdaEventSourceMapping_DocumentDbEventSourceConfig {
            collection_name: core::default::Default::default(),
            database_name: self.database_name,
            full_document: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaEventSourceMapping_DocumentDbEventSourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMapping_DocumentDbEventSourceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaEventSourceMapping_DocumentDbEventSourceConfigRef {
        LambdaEventSourceMapping_DocumentDbEventSourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMapping_DocumentDbEventSourceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `collection_name` after provisioning.\n"]
    pub fn collection_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_name", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `full_document` after provisioning.\n"]
    pub fn full_document(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.full_document", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMapping_FilterCriteria_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pattern: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LambdaEventSourceMapping_FilterCriteria_Filter {
    #[doc = "Set the field `pattern`.\n"]
    pub fn set_pattern(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pattern = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaEventSourceMapping_FilterCriteria_Filter {
    type O = terrars::BlockAssignable<LambdaEventSourceMapping_FilterCriteria_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMapping_FilterCriteria_Filter {}

impl BuildLambdaEventSourceMapping_FilterCriteria_Filter {
    pub fn build(self) -> LambdaEventSourceMapping_FilterCriteria_Filter {
        LambdaEventSourceMapping_FilterCriteria_Filter { pattern: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct LambdaEventSourceMapping_FilterCriteria_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMapping_FilterCriteria_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaEventSourceMapping_FilterCriteria_FilterRef {
        LambdaEventSourceMapping_FilterCriteria_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMapping_FilterCriteria_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize, Default)]
struct LambdaEventSourceMapping_FilterCriteriaDynamic {
    filter: std::option::Option<terrars::DynamicBlock<LambdaEventSourceMapping_FilterCriteria_Filter>>,
}

#[derive(Serialize)]
pub struct LambdaEventSourceMapping_FilterCriteria {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<LambdaEventSourceMapping_FilterCriteria_Filter>>,
    dynamic: LambdaEventSourceMapping_FilterCriteriaDynamic,
}

impl LambdaEventSourceMapping_FilterCriteria {
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaEventSourceMapping_FilterCriteria_Filter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LambdaEventSourceMapping_FilterCriteria {
    type O = terrars::BlockAssignable<LambdaEventSourceMapping_FilterCriteria>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMapping_FilterCriteria {}

impl BuildLambdaEventSourceMapping_FilterCriteria {
    pub fn build(self) -> LambdaEventSourceMapping_FilterCriteria {
        LambdaEventSourceMapping_FilterCriteria {
            filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaEventSourceMapping_FilterCriteriaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMapping_FilterCriteriaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaEventSourceMapping_FilterCriteriaRef {
        LambdaEventSourceMapping_FilterCriteriaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMapping_FilterCriteriaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMapping_MetricsConfig {
    metrics: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl LambdaEventSourceMapping_MetricsConfig { }

impl terrars::ToListMappable for LambdaEventSourceMapping_MetricsConfig {
    type O = terrars::BlockAssignable<LambdaEventSourceMapping_MetricsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMapping_MetricsConfig {
    #[doc = ""]
    pub metrics: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildLambdaEventSourceMapping_MetricsConfig {
    pub fn build(self) -> LambdaEventSourceMapping_MetricsConfig {
        LambdaEventSourceMapping_MetricsConfig { metrics: self.metrics }
    }
}

#[allow(dead_code)]
pub struct LambdaEventSourceMapping_MetricsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMapping_MetricsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaEventSourceMapping_MetricsConfigRef {
        LambdaEventSourceMapping_MetricsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMapping_MetricsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metrics` after provisioning.\n"]
    pub fn metrics(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.metrics", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMapping_ProvisionedPollerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_pollers: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_pollers: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    poller_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LambdaEventSourceMapping_ProvisionedPollerConfig {
    #[doc = "Set the field `maximum_pollers`.\n"]
    pub fn set_maximum_pollers(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_pollers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minimum_pollers`.\n"]
    pub fn set_minimum_pollers(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minimum_pollers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `poller_group_name`.\n"]
    pub fn set_poller_group_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.poller_group_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaEventSourceMapping_ProvisionedPollerConfig {
    type O = terrars::BlockAssignable<LambdaEventSourceMapping_ProvisionedPollerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMapping_ProvisionedPollerConfig {}

impl BuildLambdaEventSourceMapping_ProvisionedPollerConfig {
    pub fn build(self) -> LambdaEventSourceMapping_ProvisionedPollerConfig {
        LambdaEventSourceMapping_ProvisionedPollerConfig {
            maximum_pollers: core::default::Default::default(),
            minimum_pollers: core::default::Default::default(),
            poller_group_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaEventSourceMapping_ProvisionedPollerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMapping_ProvisionedPollerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaEventSourceMapping_ProvisionedPollerConfigRef {
        LambdaEventSourceMapping_ProvisionedPollerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMapping_ProvisionedPollerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_pollers` after provisioning.\n"]
    pub fn maximum_pollers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_pollers", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum_pollers` after provisioning.\n"]
    pub fn minimum_pollers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_pollers", self.base))
    }

    #[doc = "Get a reference to the value of field `poller_group_name` after provisioning.\n"]
    pub fn poller_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.poller_group_name", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMapping_ScalingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_concurrency: std::option::Option<terrars::PrimField<f64>>,
}

impl LambdaEventSourceMapping_ScalingConfig {
    #[doc = "Set the field `maximum_concurrency`.\n"]
    pub fn set_maximum_concurrency(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_concurrency = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaEventSourceMapping_ScalingConfig {
    type O = terrars::BlockAssignable<LambdaEventSourceMapping_ScalingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMapping_ScalingConfig {}

impl BuildLambdaEventSourceMapping_ScalingConfig {
    pub fn build(self) -> LambdaEventSourceMapping_ScalingConfig {
        LambdaEventSourceMapping_ScalingConfig { maximum_concurrency: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct LambdaEventSourceMapping_ScalingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMapping_ScalingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaEventSourceMapping_ScalingConfigRef {
        LambdaEventSourceMapping_ScalingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMapping_ScalingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_concurrency` after provisioning.\n"]
    pub fn maximum_concurrency(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_concurrency", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMapping_SelfManagedEventSource {
    endpoints: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl LambdaEventSourceMapping_SelfManagedEventSource { }

impl terrars::ToListMappable for LambdaEventSourceMapping_SelfManagedEventSource {
    type O = terrars::BlockAssignable<LambdaEventSourceMapping_SelfManagedEventSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMapping_SelfManagedEventSource {
    #[doc = ""]
    pub endpoints: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildLambdaEventSourceMapping_SelfManagedEventSource {
    pub fn build(self) -> LambdaEventSourceMapping_SelfManagedEventSource {
        LambdaEventSourceMapping_SelfManagedEventSource { endpoints: self.endpoints }
    }
}

#[allow(dead_code)]
pub struct LambdaEventSourceMapping_SelfManagedEventSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMapping_SelfManagedEventSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaEventSourceMapping_SelfManagedEventSourceRef {
        LambdaEventSourceMapping_SelfManagedEventSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMapping_SelfManagedEventSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.endpoints", self.base))
    }
}

#[derive(Serialize, Default)]
struct LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfigDynamic {
    schema_registry_config: std
    ::option
    ::Option<terrars::DynamicBlock<LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig>>,
}

#[derive(Serialize)]
pub struct LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_registry_config: std
    ::option
    ::Option<std::vec::Vec<LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig>>,
    dynamic: LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfigDynamic,
}

impl LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig {
    #[doc = "Set the field `consumer_group_id`.\n"]
    pub fn set_consumer_group_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.consumer_group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema_registry_config`.\n"]
    pub fn set_schema_registry_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema_registry_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_registry_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig {
    type O = terrars::BlockAssignable<LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig {}

impl BuildLambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig {
    pub fn build(self) -> LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig {
        LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig {
            consumer_group_id: core::default::Default::default(),
            schema_registry_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfigRef {
        LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `consumer_group_id` after provisioning.\n"]
    pub fn consumer_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_registry_config` after provisioning.\n"]
    pub fn schema_registry_config(
        &self,
    ) -> terrars::ListRef<LambdaEventSourceMapping_SelfManagedKafkaEventSourceConfig_SchemaRegistryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_registry_config", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMapping_SourceAccessConfiguration {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    uri: terrars::PrimField<std::string::String>,
}

impl LambdaEventSourceMapping_SourceAccessConfiguration { }

impl terrars::ToListMappable for LambdaEventSourceMapping_SourceAccessConfiguration {
    type O = terrars::BlockAssignable<LambdaEventSourceMapping_SourceAccessConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMapping_SourceAccessConfiguration {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildLambdaEventSourceMapping_SourceAccessConfiguration {
    pub fn build(self) -> LambdaEventSourceMapping_SourceAccessConfiguration {
        LambdaEventSourceMapping_SourceAccessConfiguration {
            type_: self.type_,
            uri: self.uri,
        }
    }
}

#[allow(dead_code)]
pub struct LambdaEventSourceMapping_SourceAccessConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMapping_SourceAccessConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaEventSourceMapping_SourceAccessConfigurationRef {
        LambdaEventSourceMapping_SourceAccessConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMapping_SourceAccessConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMapping_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LambdaEventSourceMapping_Timeouts {
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

impl terrars::ToListMappable for LambdaEventSourceMapping_Timeouts {
    type O = terrars::BlockAssignable<LambdaEventSourceMapping_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMapping_Timeouts {}

impl BuildLambdaEventSourceMapping_Timeouts {
    pub fn build(self) -> LambdaEventSourceMapping_Timeouts {
        LambdaEventSourceMapping_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaEventSourceMapping_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMapping_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaEventSourceMapping_TimeoutsRef {
        LambdaEventSourceMapping_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMapping_TimeoutsRef {
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
