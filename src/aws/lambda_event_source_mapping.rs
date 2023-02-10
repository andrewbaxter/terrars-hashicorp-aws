use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LambdaEventSourceMappingData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bisect_batch_on_function_error: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_source_arn: Option<PrimField<String>>,
    function_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_response_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_batching_window_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_record_age_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_retry_attempts: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallelization_factor: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queues: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_position: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_position_timestamp: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topics: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tumbling_window_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amazon_managed_kafka_event_source_config: Option<Vec<LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_config: Option<Vec<LambdaEventSourceMappingDestinationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_criteria: Option<Vec<LambdaEventSourceMappingFilterCriteriaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling_config: Option<Vec<LambdaEventSourceMappingScalingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_managed_event_source: Option<Vec<LambdaEventSourceMappingSelfManagedEventSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_managed_kafka_event_source_config: Option<Vec<LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_access_configuration: Option<Vec<LambdaEventSourceMappingSourceAccessConfigurationEl>>,
    dynamic: LambdaEventSourceMappingDynamic,
}

struct LambdaEventSourceMapping_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LambdaEventSourceMappingData>,
}

#[derive(Clone)]
pub struct LambdaEventSourceMapping(Rc<LambdaEventSourceMapping_>);

impl LambdaEventSourceMapping {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `batch_size`.\n"]
    pub fn set_batch_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().batch_size = Some(v.into());
        self
    }

    #[doc= "Set the field `bisect_batch_on_function_error`.\n"]
    pub fn set_bisect_batch_on_function_error(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().bisect_batch_on_function_error = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `event_source_arn`.\n"]
    pub fn set_event_source_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().event_source_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `function_response_types`.\n"]
    pub fn set_function_response_types(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().function_response_types = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `maximum_batching_window_in_seconds`.\n"]
    pub fn set_maximum_batching_window_in_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().maximum_batching_window_in_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `maximum_record_age_in_seconds`.\n"]
    pub fn set_maximum_record_age_in_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().maximum_record_age_in_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `maximum_retry_attempts`.\n"]
    pub fn set_maximum_retry_attempts(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().maximum_retry_attempts = Some(v.into());
        self
    }

    #[doc= "Set the field `parallelization_factor`.\n"]
    pub fn set_parallelization_factor(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().parallelization_factor = Some(v.into());
        self
    }

    #[doc= "Set the field `queues`.\n"]
    pub fn set_queues(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().queues = Some(v.into());
        self
    }

    #[doc= "Set the field `starting_position`.\n"]
    pub fn set_starting_position(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().starting_position = Some(v.into());
        self
    }

    #[doc= "Set the field `starting_position_timestamp`.\n"]
    pub fn set_starting_position_timestamp(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().starting_position_timestamp = Some(v.into());
        self
    }

    #[doc= "Set the field `topics`.\n"]
    pub fn set_topics(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().topics = Some(v.into());
        self
    }

    #[doc= "Set the field `tumbling_window_in_seconds`.\n"]
    pub fn set_tumbling_window_in_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tumbling_window_in_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `amazon_managed_kafka_event_source_config`.\n"]
    pub fn set_amazon_managed_kafka_event_source_config(
        self,
        v: impl Into<BlockAssignable<LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().amazon_managed_kafka_event_source_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.amazon_managed_kafka_event_source_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `destination_config`.\n"]
    pub fn set_destination_config(
        self,
        v: impl Into<BlockAssignable<LambdaEventSourceMappingDestinationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `filter_criteria`.\n"]
    pub fn set_filter_criteria(self, v: impl Into<BlockAssignable<LambdaEventSourceMappingFilterCriteriaEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter_criteria = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter_criteria = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `scaling_config`.\n"]
    pub fn set_scaling_config(self, v: impl Into<BlockAssignable<LambdaEventSourceMappingScalingConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scaling_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scaling_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `self_managed_event_source`.\n"]
    pub fn set_self_managed_event_source(
        self,
        v: impl Into<BlockAssignable<LambdaEventSourceMappingSelfManagedEventSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().self_managed_event_source = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.self_managed_event_source = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `self_managed_kafka_event_source_config`.\n"]
    pub fn set_self_managed_kafka_event_source_config(
        self,
        v: impl Into<BlockAssignable<LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().self_managed_kafka_event_source_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.self_managed_kafka_event_source_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `source_access_configuration`.\n"]
    pub fn set_source_access_configuration(
        self,
        v: impl Into<BlockAssignable<LambdaEventSourceMappingSourceAccessConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_access_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_access_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bisect_batch_on_function_error` after provisioning.\n"]
    pub fn bisect_batch_on_function_error(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bisect_batch_on_function_error", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `event_source_arn` after provisioning.\n"]
    pub fn event_source_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_source_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.function_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `function_response_types` after provisioning.\n"]
    pub fn function_response_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.function_response_types", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_processing_result` after provisioning.\n"]
    pub fn last_processing_result(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_processing_result", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maximum_batching_window_in_seconds` after provisioning.\n"]
    pub fn maximum_batching_window_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_batching_window_in_seconds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maximum_record_age_in_seconds` after provisioning.\n"]
    pub fn maximum_record_age_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_record_age_in_seconds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maximum_retry_attempts` after provisioning.\n"]
    pub fn maximum_retry_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_retry_attempts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parallelization_factor` after provisioning.\n"]
    pub fn parallelization_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.parallelization_factor", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `queues` after provisioning.\n"]
    pub fn queues(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.queues", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `starting_position` after provisioning.\n"]
    pub fn starting_position(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.starting_position", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `starting_position_timestamp` after provisioning.\n"]
    pub fn starting_position_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.starting_position_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state_transition_reason` after provisioning.\n"]
    pub fn state_transition_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state_transition_reason", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `topics` after provisioning.\n"]
    pub fn topics(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.topics", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tumbling_window_in_seconds` after provisioning.\n"]
    pub fn tumbling_window_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tumbling_window_in_seconds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `amazon_managed_kafka_event_source_config` after provisioning.\n"]
    pub fn amazon_managed_kafka_event_source_config(
        &self,
    ) -> ListRef<LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.amazon_managed_kafka_event_source_config", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `destination_config` after provisioning.\n"]
    pub fn destination_config(&self) -> ListRef<LambdaEventSourceMappingDestinationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filter_criteria` after provisioning.\n"]
    pub fn filter_criteria(&self) -> ListRef<LambdaEventSourceMappingFilterCriteriaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter_criteria", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> ListRef<LambdaEventSourceMappingScalingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_managed_event_source` after provisioning.\n"]
    pub fn self_managed_event_source(&self) -> ListRef<LambdaEventSourceMappingSelfManagedEventSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.self_managed_event_source", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_managed_kafka_event_source_config` after provisioning.\n"]
    pub fn self_managed_kafka_event_source_config(
        &self,
    ) -> ListRef<LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.self_managed_kafka_event_source_config", self.extract_ref()))
    }
}

impl Resource for LambdaEventSourceMapping {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for LambdaEventSourceMapping {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for LambdaEventSourceMapping {
    type O = ListRef<LambdaEventSourceMappingRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for LambdaEventSourceMapping_ {
    fn extract_resource_type(&self) -> String {
        "aws_lambda_event_source_mapping".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLambdaEventSourceMapping {
    pub tf_id: String,
    #[doc= ""]
    pub function_name: PrimField<String>,
}

impl BuildLambdaEventSourceMapping {
    pub fn build(self, stack: &mut Stack) -> LambdaEventSourceMapping {
        let out = LambdaEventSourceMapping(Rc::new(LambdaEventSourceMapping_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LambdaEventSourceMappingData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LambdaEventSourceMappingRef {
    shared: StackShared,
    base: String,
}

impl Ref for LambdaEventSourceMappingRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LambdaEventSourceMappingRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bisect_batch_on_function_error` after provisioning.\n"]
    pub fn bisect_batch_on_function_error(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bisect_batch_on_function_error", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `event_source_arn` after provisioning.\n"]
    pub fn event_source_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_source_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.function_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `function_response_types` after provisioning.\n"]
    pub fn function_response_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.function_response_types", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_processing_result` after provisioning.\n"]
    pub fn last_processing_result(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_processing_result", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maximum_batching_window_in_seconds` after provisioning.\n"]
    pub fn maximum_batching_window_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_batching_window_in_seconds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maximum_record_age_in_seconds` after provisioning.\n"]
    pub fn maximum_record_age_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_record_age_in_seconds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maximum_retry_attempts` after provisioning.\n"]
    pub fn maximum_retry_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_retry_attempts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parallelization_factor` after provisioning.\n"]
    pub fn parallelization_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.parallelization_factor", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `queues` after provisioning.\n"]
    pub fn queues(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.queues", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `starting_position` after provisioning.\n"]
    pub fn starting_position(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.starting_position", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `starting_position_timestamp` after provisioning.\n"]
    pub fn starting_position_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.starting_position_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state_transition_reason` after provisioning.\n"]
    pub fn state_transition_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state_transition_reason", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `topics` after provisioning.\n"]
    pub fn topics(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.topics", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tumbling_window_in_seconds` after provisioning.\n"]
    pub fn tumbling_window_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tumbling_window_in_seconds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `amazon_managed_kafka_event_source_config` after provisioning.\n"]
    pub fn amazon_managed_kafka_event_source_config(
        &self,
    ) -> ListRef<LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.amazon_managed_kafka_event_source_config", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `destination_config` after provisioning.\n"]
    pub fn destination_config(&self) -> ListRef<LambdaEventSourceMappingDestinationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filter_criteria` after provisioning.\n"]
    pub fn filter_criteria(&self) -> ListRef<LambdaEventSourceMappingFilterCriteriaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter_criteria", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> ListRef<LambdaEventSourceMappingScalingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_managed_event_source` after provisioning.\n"]
    pub fn self_managed_event_source(&self) -> ListRef<LambdaEventSourceMappingSelfManagedEventSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.self_managed_event_source", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_managed_kafka_event_source_config` after provisioning.\n"]
    pub fn self_managed_kafka_event_source_config(
        &self,
    ) -> ListRef<LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.self_managed_kafka_event_source_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_group_id: Option<PrimField<String>>,
}

impl LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigEl {
    #[doc= "Set the field `consumer_group_id`.\n"]
    pub fn set_consumer_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consumer_group_id = Some(v.into());
        self
    }
}

impl ToListMappable for LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigEl {
    type O = BlockAssignable<LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigElRef {
    fn new(shared: StackShared, base: String) -> LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigElRef {
        LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `consumer_group_id` after provisioning.\n"]
    pub fn consumer_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.consumer_group_id", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMappingDestinationConfigElOnFailureEl {
    destination_arn: PrimField<String>,
}

impl LambdaEventSourceMappingDestinationConfigElOnFailureEl { }

impl ToListMappable for LambdaEventSourceMappingDestinationConfigElOnFailureEl {
    type O = BlockAssignable<LambdaEventSourceMappingDestinationConfigElOnFailureEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMappingDestinationConfigElOnFailureEl {
    #[doc= ""]
    pub destination_arn: PrimField<String>,
}

impl BuildLambdaEventSourceMappingDestinationConfigElOnFailureEl {
    pub fn build(self) -> LambdaEventSourceMappingDestinationConfigElOnFailureEl {
        LambdaEventSourceMappingDestinationConfigElOnFailureEl { destination_arn: self.destination_arn }
    }
}

pub struct LambdaEventSourceMappingDestinationConfigElOnFailureElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LambdaEventSourceMappingDestinationConfigElOnFailureElRef {
    fn new(shared: StackShared, base: String) -> LambdaEventSourceMappingDestinationConfigElOnFailureElRef {
        LambdaEventSourceMappingDestinationConfigElOnFailureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMappingDestinationConfigElOnFailureElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `destination_arn` after provisioning.\n"]
    pub fn destination_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct LambdaEventSourceMappingDestinationConfigElDynamic {
    on_failure: Option<DynamicBlock<LambdaEventSourceMappingDestinationConfigElOnFailureEl>>,
}

#[derive(Serialize)]
pub struct LambdaEventSourceMappingDestinationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    on_failure: Option<Vec<LambdaEventSourceMappingDestinationConfigElOnFailureEl>>,
    dynamic: LambdaEventSourceMappingDestinationConfigElDynamic,
}

impl LambdaEventSourceMappingDestinationConfigEl {
    #[doc= "Set the field `on_failure`.\n"]
    pub fn set_on_failure(
        mut self,
        v: impl Into<BlockAssignable<LambdaEventSourceMappingDestinationConfigElOnFailureEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.on_failure = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.on_failure = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for LambdaEventSourceMappingDestinationConfigEl {
    type O = BlockAssignable<LambdaEventSourceMappingDestinationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct LambdaEventSourceMappingDestinationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LambdaEventSourceMappingDestinationConfigElRef {
    fn new(shared: StackShared, base: String) -> LambdaEventSourceMappingDestinationConfigElRef {
        LambdaEventSourceMappingDestinationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMappingDestinationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `on_failure` after provisioning.\n"]
    pub fn on_failure(&self) -> ListRef<LambdaEventSourceMappingDestinationConfigElOnFailureElRef> {
        ListRef::new(self.shared().clone(), format!("{}.on_failure", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMappingFilterCriteriaElFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<PrimField<String>>,
}

impl LambdaEventSourceMappingFilterCriteriaElFilterEl {
    #[doc= "Set the field `pattern`.\n"]
    pub fn set_pattern(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pattern = Some(v.into());
        self
    }
}

impl ToListMappable for LambdaEventSourceMappingFilterCriteriaElFilterEl {
    type O = BlockAssignable<LambdaEventSourceMappingFilterCriteriaElFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for LambdaEventSourceMappingFilterCriteriaElFilterElRef {
    fn new(shared: StackShared, base: String) -> LambdaEventSourceMappingFilterCriteriaElFilterElRef {
        LambdaEventSourceMappingFilterCriteriaElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMappingFilterCriteriaElFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize, Default)]
struct LambdaEventSourceMappingFilterCriteriaElDynamic {
    filter: Option<DynamicBlock<LambdaEventSourceMappingFilterCriteriaElFilterEl>>,
}

#[derive(Serialize)]
pub struct LambdaEventSourceMappingFilterCriteriaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<LambdaEventSourceMappingFilterCriteriaElFilterEl>>,
    dynamic: LambdaEventSourceMappingFilterCriteriaElDynamic,
}

impl LambdaEventSourceMappingFilterCriteriaEl {
    #[doc= "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl Into<BlockAssignable<LambdaEventSourceMappingFilterCriteriaElFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for LambdaEventSourceMappingFilterCriteriaEl {
    type O = BlockAssignable<LambdaEventSourceMappingFilterCriteriaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct LambdaEventSourceMappingFilterCriteriaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LambdaEventSourceMappingFilterCriteriaElRef {
    fn new(shared: StackShared, base: String) -> LambdaEventSourceMappingFilterCriteriaElRef {
        LambdaEventSourceMappingFilterCriteriaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMappingFilterCriteriaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMappingScalingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_concurrency: Option<PrimField<f64>>,
}

impl LambdaEventSourceMappingScalingConfigEl {
    #[doc= "Set the field `maximum_concurrency`.\n"]
    pub fn set_maximum_concurrency(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_concurrency = Some(v.into());
        self
    }
}

impl ToListMappable for LambdaEventSourceMappingScalingConfigEl {
    type O = BlockAssignable<LambdaEventSourceMappingScalingConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for LambdaEventSourceMappingScalingConfigElRef {
    fn new(shared: StackShared, base: String) -> LambdaEventSourceMappingScalingConfigElRef {
        LambdaEventSourceMappingScalingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMappingScalingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `maximum_concurrency` after provisioning.\n"]
    pub fn maximum_concurrency(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_concurrency", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMappingSelfManagedEventSourceEl {
    endpoints: RecField<PrimField<String>>,
}

impl LambdaEventSourceMappingSelfManagedEventSourceEl { }

impl ToListMappable for LambdaEventSourceMappingSelfManagedEventSourceEl {
    type O = BlockAssignable<LambdaEventSourceMappingSelfManagedEventSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMappingSelfManagedEventSourceEl {
    #[doc= ""]
    pub endpoints: RecField<PrimField<String>>,
}

impl BuildLambdaEventSourceMappingSelfManagedEventSourceEl {
    pub fn build(self) -> LambdaEventSourceMappingSelfManagedEventSourceEl {
        LambdaEventSourceMappingSelfManagedEventSourceEl { endpoints: self.endpoints }
    }
}

pub struct LambdaEventSourceMappingSelfManagedEventSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LambdaEventSourceMappingSelfManagedEventSourceElRef {
    fn new(shared: StackShared, base: String) -> LambdaEventSourceMappingSelfManagedEventSourceElRef {
        LambdaEventSourceMappingSelfManagedEventSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMappingSelfManagedEventSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.endpoints", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_group_id: Option<PrimField<String>>,
}

impl LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigEl {
    #[doc= "Set the field `consumer_group_id`.\n"]
    pub fn set_consumer_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consumer_group_id = Some(v.into());
        self
    }
}

impl ToListMappable for LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigEl {
    type O = BlockAssignable<LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigElRef {
    fn new(shared: StackShared, base: String) -> LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigElRef {
        LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `consumer_group_id` after provisioning.\n"]
    pub fn consumer_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.consumer_group_id", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaEventSourceMappingSourceAccessConfigurationEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    uri: PrimField<String>,
}

impl LambdaEventSourceMappingSourceAccessConfigurationEl { }

impl ToListMappable for LambdaEventSourceMappingSourceAccessConfigurationEl {
    type O = BlockAssignable<LambdaEventSourceMappingSourceAccessConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaEventSourceMappingSourceAccessConfigurationEl {
    #[doc= ""]
    pub type_: PrimField<String>,
    #[doc= ""]
    pub uri: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for LambdaEventSourceMappingSourceAccessConfigurationElRef {
    fn new(shared: StackShared, base: String) -> LambdaEventSourceMappingSourceAccessConfigurationElRef {
        LambdaEventSourceMappingSourceAccessConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaEventSourceMappingSourceAccessConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct LambdaEventSourceMappingDynamic {
    amazon_managed_kafka_event_source_config: Option<
        DynamicBlock<LambdaEventSourceMappingAmazonManagedKafkaEventSourceConfigEl>,
    >,
    destination_config: Option<DynamicBlock<LambdaEventSourceMappingDestinationConfigEl>>,
    filter_criteria: Option<DynamicBlock<LambdaEventSourceMappingFilterCriteriaEl>>,
    scaling_config: Option<DynamicBlock<LambdaEventSourceMappingScalingConfigEl>>,
    self_managed_event_source: Option<DynamicBlock<LambdaEventSourceMappingSelfManagedEventSourceEl>>,
    self_managed_kafka_event_source_config: Option<
        DynamicBlock<LambdaEventSourceMappingSelfManagedKafkaEventSourceConfigEl>,
    >,
    source_access_configuration: Option<DynamicBlock<LambdaEventSourceMappingSourceAccessConfigurationEl>>,
}
