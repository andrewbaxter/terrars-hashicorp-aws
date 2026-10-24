use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LambdaEventSourceMappingData {
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
    maximum_batching_window_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_record_age_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_retry_attempts: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallelization_factor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queues: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    starting_position: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    starting_position_timestamp: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topics: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tumbling_window_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_managed_kafka_event_source_config: std
    ::option
    ::Option<std::vec::Vec<LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_config: std::option::Option<std::vec::Vec<LambdaEventSourceMappingDestinationConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_criteria: std::option::Option<std::vec::Vec<LambdaEventSourceMappingFilterCriteriaEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_config: std::option::Option<std::vec::Vec<LambdaEventSourceMappingScalingConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_managed_event_source: std::option::Option<std::vec::Vec<LambdaEventSourceMappingSelfManagedEventSourceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_managed_kafka_event_source_config: std
    ::option
    ::Option<std::vec::Vec<LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_access_configuration: std::option::Option<std::vec::Vec<LambdaEventSourceMappingSourceAccessConfigurationEl>>,
    dynamic: LambdaEventSourceMappingDynamic,
}

struct LambdaEventSourceMapping_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LambdaEventSourceMappingData>,
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
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().queues = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `amazon_managed_kafka_event_source_config`.\n"]
    pub fn set_amazon_managed_kafka_event_source_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaEventSourceMappingDestinationConfigEl>>,
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

    #[doc = "Set the field `filter_criteria`.\n"]
    pub fn set_filter_criteria(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaEventSourceMappingFilterCriteriaEl>>,
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

    #[doc = "Set the field `scaling_config`.\n"]
    pub fn set_scaling_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaEventSourceMappingScalingConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaEventSourceMappingSelfManagedEventSourceEl>>,
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
                    ::Into<terrars::BlockAssignable<LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaEventSourceMappingSourceAccessConfigurationEl>>,
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
    pub fn queues(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.queues", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `topics` after provisioning.\n"]
    pub fn topics(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.topics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tumbling_window_in_seconds` after provisioning.\n"]
    pub fn tumbling_window_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tumbling_window_in_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `amazon_managed_kafka_event_source_config` after provisioning.\n"]
    pub fn amazon_managed_kafka_event_source_config(
        &self,
    ) -> terrars::ListRef<LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.amazon_managed_kafka_event_source_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `destination_config` after provisioning.\n"]
    pub fn destination_config(&self) -> terrars::ListRef<LambdaEventSourceMappingDestinationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_criteria` after provisioning.\n"]
    pub fn filter_criteria(&self) -> terrars::ListRef<LambdaEventSourceMappingFilterCriteriaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_criteria", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> terrars::ListRef<LambdaEventSourceMappingScalingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_managed_event_source` after provisioning.\n"]
    pub fn self_managed_event_source(&self) -> terrars::ListRef<LambdaEventSourceMappingSelfManagedEventSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_managed_event_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_managed_kafka_event_source_config` after provisioning.\n"]
    pub fn self_managed_kafka_event_source_config(
        &self,
    ) -> terrars::ListRef<LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.self_managed_kafka_event_source_config", self.extract_ref()),
        )
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
            data: std::cell::RefCell::new(LambdaEventSourceMappingData {
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
                maximum_batching_window_in_seconds: core::default::Default::default(),
                maximum_record_age_in_seconds: core::default::Default::default(),
                maximum_retry_attempts: core::default::Default::default(),
                parallelization_factor: core::default::Default::default(),
                queues: core::default::Default::default(),
                starting_position: core::default::Default::default(),
                starting_position_timestamp: core::default::Default::default(),
                topics: core::default::Default::default(),
                tumbling_window_in_seconds: core::default::Default::default(),
                amazon_managed_kafka_event_source_config: core::default::Default::default(),
                destination_config: core::default::Default::default(),
                filter_criteria: core::default::Default::default(),
                scaling_config: core::default::Default::default(),
                self_managed_event_source: core::default::Default::default(),
                self_managed_kafka_event_source_config: core::default::Default::default(),
                source_access_configuration: core::default::Default::default(),
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
    pub fn queues(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.queues", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `topics` after provisioning.\n"]
    pub fn topics(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.topics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tumbling_window_in_seconds` after provisioning.\n"]
    pub fn tumbling_window_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tumbling_window_in_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `amazon_managed_kafka_event_source_config` after provisioning.\n"]
    pub fn amazon_managed_kafka_event_source_config(
        &self,
    ) -> terrars::ListRef<LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.amazon_managed_kafka_event_source_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `destination_config` after provisioning.\n"]
    pub fn destination_config(&self) -> terrars::ListRef<LambdaEventSourceMappingDestinationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_criteria` after provisioning.\n"]
    pub fn filter_criteria(&self) -> terrars::ListRef<LambdaEventSourceMappingFilterCriteriaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_criteria", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> terrars::ListRef<LambdaEventSourceMappingScalingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_managed_event_source` after provisioning.\n"]
    pub fn self_managed_event_source(&self) -> terrars::ListRef<LambdaEventSourceMappingSelfManagedEventSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_managed_event_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_managed_kafka_event_source_config` after provisioning.\n"]
    pub fn self_managed_kafka_event_source_config(
        &self,
    ) -> terrars::ListRef<LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.self_managed_kafka_event_source_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigEl {
    #[doc = "Set the field `consumer_group_id`.\n"]
    pub fn set_consumer_group_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.consumer_group_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigEl {
    type O = terrars::BlockAssignable<LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigEl {}

impl BuildLambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigEl {
    pub fn build(self) -> LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigEl {
        LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigEl {
            consumer_group_id: core::default::Default::default(),
        }
    }
}

pub struct LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigElRef {
        LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `consumer_group_id` after provisioning.\n"]
    pub fn consumer_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_group_id", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMappingDestinationConfigElOnFailureEl {
    destination_arn: terrars::PrimField<std::string::String>,
}

impl LambdaEventSourceMappingDestinationConfigElOnFailureEl { }

impl terrars::ToListMappable for LambdaEventSourceMappingDestinationConfigElOnFailureEl {
    type O = terrars::BlockAssignable<LambdaEventSourceMappingDestinationConfigElOnFailureEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMappingDestinationConfigElOnFailureEl {
    #[doc = ""]
    pub destination_arn: terrars::PrimField<std::string::String>,
}

impl BuildLambdaEventSourceMappingDestinationConfigElOnFailureEl {
    pub fn build(self) -> LambdaEventSourceMappingDestinationConfigElOnFailureEl {
        LambdaEventSourceMappingDestinationConfigElOnFailureEl { destination_arn: self.destination_arn }
    }
}

pub struct LambdaEventSourceMappingDestinationConfigElOnFailureElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMappingDestinationConfigElOnFailureElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaEventSourceMappingDestinationConfigElOnFailureElRef {
        LambdaEventSourceMappingDestinationConfigElOnFailureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMappingDestinationConfigElOnFailureElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_arn` after provisioning.\n"]
    pub fn destination_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct LambdaEventSourceMappingDestinationConfigElDynamic {
    on_failure: std::option::Option<terrars::DynamicBlock<LambdaEventSourceMappingDestinationConfigElOnFailureEl>>,
}

#[derive(Serialize)]
pub struct LambdaEventSourceMappingDestinationConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_failure: std::option::Option<std::vec::Vec<LambdaEventSourceMappingDestinationConfigElOnFailureEl>>,
    dynamic: LambdaEventSourceMappingDestinationConfigElDynamic,
}

impl LambdaEventSourceMappingDestinationConfigEl {
    #[doc = "Set the field `on_failure`.\n"]
    pub fn set_on_failure(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaEventSourceMappingDestinationConfigElOnFailureEl>>,
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

impl terrars::ToListMappable for LambdaEventSourceMappingDestinationConfigEl {
    type O = terrars::BlockAssignable<LambdaEventSourceMappingDestinationConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMappingDestinationConfigEl {}

impl BuildLambdaEventSourceMappingDestinationConfigEl {
    pub fn build(self) -> LambdaEventSourceMappingDestinationConfigEl {
        LambdaEventSourceMappingDestinationConfigEl {
            on_failure: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LambdaEventSourceMappingDestinationConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMappingDestinationConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaEventSourceMappingDestinationConfigElRef {
        LambdaEventSourceMappingDestinationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMappingDestinationConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `on_failure` after provisioning.\n"]
    pub fn on_failure(&self) -> terrars::ListRef<LambdaEventSourceMappingDestinationConfigElOnFailureElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_failure", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMappingFilterCriteriaElFilterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pattern: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LambdaEventSourceMappingFilterCriteriaElFilterEl {
    #[doc = "Set the field `pattern`.\n"]
    pub fn set_pattern(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pattern = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaEventSourceMappingFilterCriteriaElFilterEl {
    type O = terrars::BlockAssignable<LambdaEventSourceMappingFilterCriteriaElFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMappingFilterCriteriaElFilterEl {}

impl BuildLambdaEventSourceMappingFilterCriteriaElFilterEl {
    pub fn build(self) -> LambdaEventSourceMappingFilterCriteriaElFilterEl {
        LambdaEventSourceMappingFilterCriteriaElFilterEl { pattern: core::default::Default::default() }
    }
}

pub struct LambdaEventSourceMappingFilterCriteriaElFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMappingFilterCriteriaElFilterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaEventSourceMappingFilterCriteriaElFilterElRef {
        LambdaEventSourceMappingFilterCriteriaElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMappingFilterCriteriaElFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize, Default)]
struct LambdaEventSourceMappingFilterCriteriaElDynamic {
    filter: std::option::Option<terrars::DynamicBlock<LambdaEventSourceMappingFilterCriteriaElFilterEl>>,
}

#[derive(Serialize)]
pub struct LambdaEventSourceMappingFilterCriteriaEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<LambdaEventSourceMappingFilterCriteriaElFilterEl>>,
    dynamic: LambdaEventSourceMappingFilterCriteriaElDynamic,
}

impl LambdaEventSourceMappingFilterCriteriaEl {
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaEventSourceMappingFilterCriteriaElFilterEl>>,
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

impl terrars::ToListMappable for LambdaEventSourceMappingFilterCriteriaEl {
    type O = terrars::BlockAssignable<LambdaEventSourceMappingFilterCriteriaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMappingFilterCriteriaEl {}

impl BuildLambdaEventSourceMappingFilterCriteriaEl {
    pub fn build(self) -> LambdaEventSourceMappingFilterCriteriaEl {
        LambdaEventSourceMappingFilterCriteriaEl {
            filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LambdaEventSourceMappingFilterCriteriaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMappingFilterCriteriaElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaEventSourceMappingFilterCriteriaElRef {
        LambdaEventSourceMappingFilterCriteriaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMappingFilterCriteriaElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMappingScalingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_concurrency: std::option::Option<terrars::PrimField<f64>>,
}

impl LambdaEventSourceMappingScalingConfigEl {
    #[doc = "Set the field `maximum_concurrency`.\n"]
    pub fn set_maximum_concurrency(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_concurrency = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaEventSourceMappingScalingConfigEl {
    type O = terrars::BlockAssignable<LambdaEventSourceMappingScalingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMappingScalingConfigEl {}

impl BuildLambdaEventSourceMappingScalingConfigEl {
    pub fn build(self) -> LambdaEventSourceMappingScalingConfigEl {
        LambdaEventSourceMappingScalingConfigEl { maximum_concurrency: core::default::Default::default() }
    }
}

pub struct LambdaEventSourceMappingScalingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMappingScalingConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaEventSourceMappingScalingConfigElRef {
        LambdaEventSourceMappingScalingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMappingScalingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_concurrency` after provisioning.\n"]
    pub fn maximum_concurrency(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_concurrency", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMappingSelfManagedEventSourceEl {
    endpoints: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl LambdaEventSourceMappingSelfManagedEventSourceEl { }

impl terrars::ToListMappable for LambdaEventSourceMappingSelfManagedEventSourceEl {
    type O = terrars::BlockAssignable<LambdaEventSourceMappingSelfManagedEventSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMappingSelfManagedEventSourceEl {
    #[doc = ""]
    pub endpoints: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildLambdaEventSourceMappingSelfManagedEventSourceEl {
    pub fn build(self) -> LambdaEventSourceMappingSelfManagedEventSourceEl {
        LambdaEventSourceMappingSelfManagedEventSourceEl { endpoints: self.endpoints }
    }
}

pub struct LambdaEventSourceMappingSelfManagedEventSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMappingSelfManagedEventSourceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaEventSourceMappingSelfManagedEventSourceElRef {
        LambdaEventSourceMappingSelfManagedEventSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMappingSelfManagedEventSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.endpoints", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigEl {
    #[doc = "Set the field `consumer_group_id`.\n"]
    pub fn set_consumer_group_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.consumer_group_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigEl {
    type O = terrars::BlockAssignable<LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMappingSelfManagedKafkaEventSourceConfigEl {}

impl BuildLambdaEventSourceMappingSelfManagedKafkaEventSourceConfigEl {
    pub fn build(self) -> LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigEl {
        LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigEl {
            consumer_group_id: core::default::Default::default(),
        }
    }
}

pub struct LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigElRef {
        LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `consumer_group_id` after provisioning.\n"]
    pub fn consumer_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_group_id", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMappingSourceAccessConfigurationEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    uri: terrars::PrimField<std::string::String>,
}

impl LambdaEventSourceMappingSourceAccessConfigurationEl { }

impl terrars::ToListMappable for LambdaEventSourceMappingSourceAccessConfigurationEl {
    type O = terrars::BlockAssignable<LambdaEventSourceMappingSourceAccessConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMappingSourceAccessConfigurationEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildLambdaEventSourceMappingSourceAccessConfigurationEl {
    pub fn build(self) -> LambdaEventSourceMappingSourceAccessConfigurationEl {
        LambdaEventSourceMappingSourceAccessConfigurationEl {
            type_: self.type_,
            uri: self.uri,
        }
    }
}

pub struct LambdaEventSourceMappingSourceAccessConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaEventSourceMappingSourceAccessConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaEventSourceMappingSourceAccessConfigurationElRef {
        LambdaEventSourceMappingSourceAccessConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMappingSourceAccessConfigurationElRef {
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

#[derive(Serialize, Default)]
struct LambdaEventSourceMappingDynamic {
    amazon_managed_kafka_event_source_config: std
    ::option
    ::Option<terrars::DynamicBlock<LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigEl>>,
    destination_config: std::option::Option<terrars::DynamicBlock<LambdaEventSourceMappingDestinationConfigEl>>,
    filter_criteria: std::option::Option<terrars::DynamicBlock<LambdaEventSourceMappingFilterCriteriaEl>>,
    scaling_config: std::option::Option<terrars::DynamicBlock<LambdaEventSourceMappingScalingConfigEl>>,
    self_managed_event_source: std
    ::option
    ::Option<terrars::DynamicBlock<LambdaEventSourceMappingSelfManagedEventSourceEl>>,
    self_managed_kafka_event_source_config: std
    ::option
    ::Option<terrars::DynamicBlock<LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigEl>>,
    source_access_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<LambdaEventSourceMappingSourceAccessConfigurationEl>>,
}
