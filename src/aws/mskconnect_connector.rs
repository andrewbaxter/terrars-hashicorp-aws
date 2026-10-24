use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MskconnectConnectorData {
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
    connector_configuration: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    kafkaconnect_version: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    service_execution_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity: std::option::Option<std::vec::Vec<MskconnectConnectorCapacityEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kafka_cluster: std::option::Option<std::vec::Vec<MskconnectConnectorKafkaClusterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kafka_cluster_client_authentication: std
    ::option
    ::Option<std::vec::Vec<MskconnectConnectorKafkaClusterClientAuthenticationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kafka_cluster_encryption_in_transit: std
    ::option
    ::Option<std::vec::Vec<MskconnectConnectorKafkaClusterEncryptionInTransitEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_delivery: std::option::Option<std::vec::Vec<MskconnectConnectorLogDeliveryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plugin: std::option::Option<std::vec::Vec<MskconnectConnectorPluginEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MskconnectConnectorTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_configuration: std::option::Option<std::vec::Vec<MskconnectConnectorWorkerConfigurationEl>>,
    dynamic: MskconnectConnectorDynamic,
}

struct MskconnectConnector_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MskconnectConnectorData>,
}

#[derive(Clone)]
pub struct MskconnectConnector(std::rc::Rc<MskconnectConnector_>);

impl MskconnectConnector {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity`.\n"]
    pub fn set_capacity(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnectorCapacityEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capacity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kafka_cluster`.\n"]
    pub fn set_kafka_cluster(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnectorKafkaClusterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kafka_cluster = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kafka_cluster = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kafka_cluster_client_authentication`.\n"]
    pub fn set_kafka_cluster_client_authentication(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnectorKafkaClusterClientAuthenticationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kafka_cluster_client_authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kafka_cluster_client_authentication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kafka_cluster_encryption_in_transit`.\n"]
    pub fn set_kafka_cluster_encryption_in_transit(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnectorKafkaClusterEncryptionInTransitEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kafka_cluster_encryption_in_transit = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kafka_cluster_encryption_in_transit = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_delivery`.\n"]
    pub fn set_log_delivery(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnectorLogDeliveryEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_delivery = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_delivery = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `plugin`.\n"]
    pub fn set_plugin(self, v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnectorPluginEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().plugin = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.plugin = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<MskconnectConnectorTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `worker_configuration`.\n"]
    pub fn set_worker_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnectorWorkerConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().worker_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.worker_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connector_configuration` after provisioning.\n"]
    pub fn connector_configuration(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.connector_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafkaconnect_version` after provisioning.\n"]
    pub fn kafkaconnect_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kafkaconnect_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_execution_role_arn` after provisioning.\n"]
    pub fn service_execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> terrars::ListRef<MskconnectConnectorCapacityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_cluster` after provisioning.\n"]
    pub fn kafka_cluster(&self) -> terrars::ListRef<MskconnectConnectorKafkaClusterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kafka_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_cluster_client_authentication` after provisioning.\n"]
    pub fn kafka_cluster_client_authentication(
        &self,
    ) -> terrars::ListRef<MskconnectConnectorKafkaClusterClientAuthenticationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.kafka_cluster_client_authentication", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kafka_cluster_encryption_in_transit` after provisioning.\n"]
    pub fn kafka_cluster_encryption_in_transit(
        &self,
    ) -> terrars::ListRef<MskconnectConnectorKafkaClusterEncryptionInTransitElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.kafka_cluster_encryption_in_transit", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_delivery` after provisioning.\n"]
    pub fn log_delivery(&self) -> terrars::ListRef<MskconnectConnectorLogDeliveryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_delivery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskconnectConnectorTimeoutsElRef {
        MskconnectConnectorTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_configuration` after provisioning.\n"]
    pub fn worker_configuration(&self) -> terrars::ListRef<MskconnectConnectorWorkerConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for MskconnectConnector {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MskconnectConnector { }

impl terrars::ToListMappable for MskconnectConnector {
    type O = terrars::ListRef<MskconnectConnectorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MskconnectConnector_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_mskconnect_connector".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMskconnectConnector {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub connector_configuration: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub kafkaconnect_version: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_execution_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildMskconnectConnector {
    pub fn build(self, stack: &mut terrars::Stack) -> MskconnectConnector {
        let out = MskconnectConnector(std::rc::Rc::new(MskconnectConnector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MskconnectConnectorData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                connector_configuration: self.connector_configuration,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                kafkaconnect_version: self.kafkaconnect_version,
                name: self.name,
                service_execution_role_arn: self.service_execution_role_arn,
                capacity: core::default::Default::default(),
                kafka_cluster: core::default::Default::default(),
                kafka_cluster_client_authentication: core::default::Default::default(),
                kafka_cluster_encryption_in_transit: core::default::Default::default(),
                log_delivery: core::default::Default::default(),
                plugin: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                worker_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MskconnectConnectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnectorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MskconnectConnectorRef {
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

    #[doc = "Get a reference to the value of field `connector_configuration` after provisioning.\n"]
    pub fn connector_configuration(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.connector_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafkaconnect_version` after provisioning.\n"]
    pub fn kafkaconnect_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kafkaconnect_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_execution_role_arn` after provisioning.\n"]
    pub fn service_execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> terrars::ListRef<MskconnectConnectorCapacityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_cluster` after provisioning.\n"]
    pub fn kafka_cluster(&self) -> terrars::ListRef<MskconnectConnectorKafkaClusterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kafka_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_cluster_client_authentication` after provisioning.\n"]
    pub fn kafka_cluster_client_authentication(
        &self,
    ) -> terrars::ListRef<MskconnectConnectorKafkaClusterClientAuthenticationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.kafka_cluster_client_authentication", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kafka_cluster_encryption_in_transit` after provisioning.\n"]
    pub fn kafka_cluster_encryption_in_transit(
        &self,
    ) -> terrars::ListRef<MskconnectConnectorKafkaClusterEncryptionInTransitElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.kafka_cluster_encryption_in_transit", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_delivery` after provisioning.\n"]
    pub fn log_delivery(&self) -> terrars::ListRef<MskconnectConnectorLogDeliveryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_delivery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskconnectConnectorTimeoutsElRef {
        MskconnectConnectorTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_configuration` after provisioning.\n"]
    pub fn worker_configuration(&self) -> terrars::ListRef<MskconnectConnectorWorkerConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_utilization_percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl MskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl {
    #[doc = "Set the field `cpu_utilization_percentage`.\n"]
    pub fn set_cpu_utilization_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_utilization_percentage = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl {
    type O = terrars::BlockAssignable<MskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl {}

impl BuildMskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl {
    pub fn build(self) -> MskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl {
        MskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl {
            cpu_utilization_percentage: core::default::Default::default(),
        }
    }
}

pub struct MskconnectConnectorCapacityElAutoscalingElScaleInPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnectorCapacityElAutoscalingElScaleInPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnectorCapacityElAutoscalingElScaleInPolicyElRef {
        MskconnectConnectorCapacityElAutoscalingElScaleInPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnectorCapacityElAutoscalingElScaleInPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu_utilization_percentage` after provisioning.\n"]
    pub fn cpu_utilization_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_utilization_percentage", self.base))
    }
}

#[derive(Serialize)]
pub struct MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_utilization_percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl {
    #[doc = "Set the field `cpu_utilization_percentage`.\n"]
    pub fn set_cpu_utilization_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_utilization_percentage = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl {
    type O = terrars::BlockAssignable<MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl {}

impl BuildMskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl {
    pub fn build(self) -> MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl {
        MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl {
            cpu_utilization_percentage: core::default::Default::default(),
        }
    }
}

pub struct MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyElRef {
        MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu_utilization_percentage` after provisioning.\n"]
    pub fn cpu_utilization_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_utilization_percentage", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskconnectConnectorCapacityElAutoscalingElDynamic {
    scale_in_policy: std
    ::option
    ::Option<terrars::DynamicBlock<MskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl>>,
    scale_out_policy: std
    ::option
    ::Option<terrars::DynamicBlock<MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl>>,
}

#[derive(Serialize)]
pub struct MskconnectConnectorCapacityElAutoscalingEl {
    max_worker_count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mcu_count: std::option::Option<terrars::PrimField<f64>>,
    min_worker_count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_in_policy: std::option::Option<std::vec::Vec<MskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_out_policy: std::option::Option<std::vec::Vec<MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl>>,
    dynamic: MskconnectConnectorCapacityElAutoscalingElDynamic,
}

impl MskconnectConnectorCapacityElAutoscalingEl {
    #[doc = "Set the field `mcu_count`.\n"]
    pub fn set_mcu_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.mcu_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_in_policy`.\n"]
    pub fn set_scale_in_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scale_in_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scale_in_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scale_out_policy`.\n"]
    pub fn set_scale_out_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scale_out_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scale_out_policy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskconnectConnectorCapacityElAutoscalingEl {
    type O = terrars::BlockAssignable<MskconnectConnectorCapacityElAutoscalingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnectorCapacityElAutoscalingEl {
    #[doc = ""]
    pub max_worker_count: terrars::PrimField<f64>,
    #[doc = ""]
    pub min_worker_count: terrars::PrimField<f64>,
}

impl BuildMskconnectConnectorCapacityElAutoscalingEl {
    pub fn build(self) -> MskconnectConnectorCapacityElAutoscalingEl {
        MskconnectConnectorCapacityElAutoscalingEl {
            max_worker_count: self.max_worker_count,
            mcu_count: core::default::Default::default(),
            min_worker_count: self.min_worker_count,
            scale_in_policy: core::default::Default::default(),
            scale_out_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MskconnectConnectorCapacityElAutoscalingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnectorCapacityElAutoscalingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskconnectConnectorCapacityElAutoscalingElRef {
        MskconnectConnectorCapacityElAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnectorCapacityElAutoscalingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_worker_count` after provisioning.\n"]
    pub fn max_worker_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_worker_count", self.base))
    }

    #[doc = "Get a reference to the value of field `mcu_count` after provisioning.\n"]
    pub fn mcu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mcu_count", self.base))
    }

    #[doc = "Get a reference to the value of field `min_worker_count` after provisioning.\n"]
    pub fn min_worker_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_worker_count", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_in_policy` after provisioning.\n"]
    pub fn scale_in_policy(&self) -> terrars::ListRef<MskconnectConnectorCapacityElAutoscalingElScaleInPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scale_in_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_out_policy` after provisioning.\n"]
    pub fn scale_out_policy(&self) -> terrars::ListRef<MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scale_out_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct MskconnectConnectorCapacityElProvisionedCapacityEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mcu_count: std::option::Option<terrars::PrimField<f64>>,
    worker_count: terrars::PrimField<f64>,
}

impl MskconnectConnectorCapacityElProvisionedCapacityEl {
    #[doc = "Set the field `mcu_count`.\n"]
    pub fn set_mcu_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.mcu_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskconnectConnectorCapacityElProvisionedCapacityEl {
    type O = terrars::BlockAssignable<MskconnectConnectorCapacityElProvisionedCapacityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnectorCapacityElProvisionedCapacityEl {
    #[doc = ""]
    pub worker_count: terrars::PrimField<f64>,
}

impl BuildMskconnectConnectorCapacityElProvisionedCapacityEl {
    pub fn build(self) -> MskconnectConnectorCapacityElProvisionedCapacityEl {
        MskconnectConnectorCapacityElProvisionedCapacityEl {
            mcu_count: core::default::Default::default(),
            worker_count: self.worker_count,
        }
    }
}

pub struct MskconnectConnectorCapacityElProvisionedCapacityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnectorCapacityElProvisionedCapacityElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnectorCapacityElProvisionedCapacityElRef {
        MskconnectConnectorCapacityElProvisionedCapacityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnectorCapacityElProvisionedCapacityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mcu_count` after provisioning.\n"]
    pub fn mcu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mcu_count", self.base))
    }

    #[doc = "Get a reference to the value of field `worker_count` after provisioning.\n"]
    pub fn worker_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_count", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskconnectConnectorCapacityElDynamic {
    autoscaling: std::option::Option<terrars::DynamicBlock<MskconnectConnectorCapacityElAutoscalingEl>>,
    provisioned_capacity: std
    ::option
    ::Option<terrars::DynamicBlock<MskconnectConnectorCapacityElProvisionedCapacityEl>>,
}

#[derive(Serialize)]
pub struct MskconnectConnectorCapacityEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling: std::option::Option<std::vec::Vec<MskconnectConnectorCapacityElAutoscalingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_capacity: std::option::Option<std::vec::Vec<MskconnectConnectorCapacityElProvisionedCapacityEl>>,
    dynamic: MskconnectConnectorCapacityElDynamic,
}

impl MskconnectConnectorCapacityEl {
    #[doc = "Set the field `autoscaling`.\n"]
    pub fn set_autoscaling(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnectorCapacityElAutoscalingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autoscaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `provisioned_capacity`.\n"]
    pub fn set_provisioned_capacity(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnectorCapacityElProvisionedCapacityEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.provisioned_capacity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.provisioned_capacity = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskconnectConnectorCapacityEl {
    type O = terrars::BlockAssignable<MskconnectConnectorCapacityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnectorCapacityEl {}

impl BuildMskconnectConnectorCapacityEl {
    pub fn build(self) -> MskconnectConnectorCapacityEl {
        MskconnectConnectorCapacityEl {
            autoscaling: core::default::Default::default(),
            provisioned_capacity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MskconnectConnectorCapacityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnectorCapacityElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskconnectConnectorCapacityElRef {
        MskconnectConnectorCapacityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnectorCapacityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> terrars::ListRef<MskconnectConnectorCapacityElAutoscalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioned_capacity` after provisioning.\n"]
    pub fn provisioned_capacity(&self) -> terrars::ListRef<MskconnectConnectorCapacityElProvisionedCapacityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provisioned_capacity", self.base))
    }
}

#[derive(Serialize)]
pub struct MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl {
    security_groups: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl { }

impl terrars::ToListMappable for MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl {
    type O = terrars::BlockAssignable<MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl {
    #[doc = ""]
    pub security_groups: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildMskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl {
    pub fn build(self) -> MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl {
        MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl {
            security_groups: self.security_groups,
            subnets: self.subnets,
        }
    }
}

pub struct MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcElRef {
        MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskconnectConnectorKafkaClusterElApacheKafkaClusterElDynamic {
    vpc: std::option::Option<terrars::DynamicBlock<MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl>>,
}

#[derive(Serialize)]
pub struct MskconnectConnectorKafkaClusterElApacheKafkaClusterEl {
    bootstrap_servers: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<std::vec::Vec<MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl>>,
    dynamic: MskconnectConnectorKafkaClusterElApacheKafkaClusterElDynamic,
}

impl MskconnectConnectorKafkaClusterElApacheKafkaClusterEl {
    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskconnectConnectorKafkaClusterElApacheKafkaClusterEl {
    type O = terrars::BlockAssignable<MskconnectConnectorKafkaClusterElApacheKafkaClusterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnectorKafkaClusterElApacheKafkaClusterEl {
    #[doc = ""]
    pub bootstrap_servers: terrars::PrimField<std::string::String>,
}

impl BuildMskconnectConnectorKafkaClusterElApacheKafkaClusterEl {
    pub fn build(self) -> MskconnectConnectorKafkaClusterElApacheKafkaClusterEl {
        MskconnectConnectorKafkaClusterElApacheKafkaClusterEl {
            bootstrap_servers: self.bootstrap_servers,
            vpc: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MskconnectConnectorKafkaClusterElApacheKafkaClusterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnectorKafkaClusterElApacheKafkaClusterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnectorKafkaClusterElApacheKafkaClusterElRef {
        MskconnectConnectorKafkaClusterElApacheKafkaClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnectorKafkaClusterElApacheKafkaClusterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bootstrap_servers` after provisioning.\n"]
    pub fn bootstrap_servers(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_servers", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskconnectConnectorKafkaClusterElDynamic {
    apache_kafka_cluster: std
    ::option
    ::Option<terrars::DynamicBlock<MskconnectConnectorKafkaClusterElApacheKafkaClusterEl>>,
}

#[derive(Serialize)]
pub struct MskconnectConnectorKafkaClusterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    apache_kafka_cluster: std::option::Option<std::vec::Vec<MskconnectConnectorKafkaClusterElApacheKafkaClusterEl>>,
    dynamic: MskconnectConnectorKafkaClusterElDynamic,
}

impl MskconnectConnectorKafkaClusterEl {
    #[doc = "Set the field `apache_kafka_cluster`.\n"]
    pub fn set_apache_kafka_cluster(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnectorKafkaClusterElApacheKafkaClusterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.apache_kafka_cluster = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.apache_kafka_cluster = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskconnectConnectorKafkaClusterEl {
    type O = terrars::BlockAssignable<MskconnectConnectorKafkaClusterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnectorKafkaClusterEl {}

impl BuildMskconnectConnectorKafkaClusterEl {
    pub fn build(self) -> MskconnectConnectorKafkaClusterEl {
        MskconnectConnectorKafkaClusterEl {
            apache_kafka_cluster: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MskconnectConnectorKafkaClusterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnectorKafkaClusterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskconnectConnectorKafkaClusterElRef {
        MskconnectConnectorKafkaClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnectorKafkaClusterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apache_kafka_cluster` after provisioning.\n"]
    pub fn apache_kafka_cluster(&self) -> terrars::ListRef<MskconnectConnectorKafkaClusterElApacheKafkaClusterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.apache_kafka_cluster", self.base))
    }
}

#[derive(Serialize)]
pub struct MskconnectConnectorKafkaClusterClientAuthenticationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskconnectConnectorKafkaClusterClientAuthenticationEl {
    #[doc = "Set the field `authentication_type`.\n"]
    pub fn set_authentication_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authentication_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskconnectConnectorKafkaClusterClientAuthenticationEl {
    type O = terrars::BlockAssignable<MskconnectConnectorKafkaClusterClientAuthenticationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnectorKafkaClusterClientAuthenticationEl {}

impl BuildMskconnectConnectorKafkaClusterClientAuthenticationEl {
    pub fn build(self) -> MskconnectConnectorKafkaClusterClientAuthenticationEl {
        MskconnectConnectorKafkaClusterClientAuthenticationEl {
            authentication_type: core::default::Default::default(),
        }
    }
}

pub struct MskconnectConnectorKafkaClusterClientAuthenticationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnectorKafkaClusterClientAuthenticationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnectorKafkaClusterClientAuthenticationElRef {
        MskconnectConnectorKafkaClusterClientAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnectorKafkaClusterClientAuthenticationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication_type` after provisioning.\n"]
    pub fn authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_type", self.base))
    }
}

#[derive(Serialize)]
pub struct MskconnectConnectorKafkaClusterEncryptionInTransitEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskconnectConnectorKafkaClusterEncryptionInTransitEl {
    #[doc = "Set the field `encryption_type`.\n"]
    pub fn set_encryption_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encryption_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskconnectConnectorKafkaClusterEncryptionInTransitEl {
    type O = terrars::BlockAssignable<MskconnectConnectorKafkaClusterEncryptionInTransitEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnectorKafkaClusterEncryptionInTransitEl {}

impl BuildMskconnectConnectorKafkaClusterEncryptionInTransitEl {
    pub fn build(self) -> MskconnectConnectorKafkaClusterEncryptionInTransitEl {
        MskconnectConnectorKafkaClusterEncryptionInTransitEl { encryption_type: core::default::Default::default() }
    }
}

pub struct MskconnectConnectorKafkaClusterEncryptionInTransitElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnectorKafkaClusterEncryptionInTransitElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnectorKafkaClusterEncryptionInTransitElRef {
        MskconnectConnectorKafkaClusterEncryptionInTransitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnectorKafkaClusterEncryptionInTransitElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\n"]
    pub fn encryption_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_type", self.base))
    }
}

#[derive(Serialize)]
pub struct MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl {
    #[doc = "Set the field `log_group`.\n"]
    pub fn set_log_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_group = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl {
    type O = terrars::BlockAssignable<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildMskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl {
    pub fn build(self) -> MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl {
        MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl {
            enabled: self.enabled,
            log_group: core::default::Default::default(),
        }
    }
}

pub struct MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsElRef {
        MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_group` after provisioning.\n"]
    pub fn log_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group", self.base))
    }
}

#[derive(Serialize)]
pub struct MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delivery_stream: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
}

impl MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl {
    #[doc = "Set the field `delivery_stream`.\n"]
    pub fn set_delivery_stream(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delivery_stream = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl {
    type O = terrars::BlockAssignable<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildMskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl {
    pub fn build(self) -> MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl {
        MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl {
            delivery_stream: core::default::Default::default(),
            enabled: self.enabled,
        }
    }
}

pub struct MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseElRef {
        MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delivery_stream` after provisioning.\n"]
    pub fn delivery_stream(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delivery_stream", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El {
    type O = terrars::BlockAssignable<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildMskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El {
    pub fn build(self) -> MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El {
        MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El {
            bucket: core::default::Default::default(),
            enabled: self.enabled,
            prefix: core::default::Default::default(),
        }
    }
}

pub struct MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3ElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3ElRef {
        MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3ElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskconnectConnectorLogDeliveryElWorkerLogDeliveryElDynamic {
    cloudwatch_logs: std
    ::option
    ::Option<terrars::DynamicBlock<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl>>,
    firehose: std
    ::option
    ::Option<terrars::DynamicBlock<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl>>,
    s3: std::option::Option<terrars::DynamicBlock<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El>>,
}

#[derive(Serialize)]
pub struct MskconnectConnectorLogDeliveryElWorkerLogDeliveryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs: std
    ::option
    ::Option<std::vec::Vec<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firehose: std::option::Option<std::vec::Vec<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El>>,
    dynamic: MskconnectConnectorLogDeliveryElWorkerLogDeliveryElDynamic,
}

impl MskconnectConnectorLogDeliveryElWorkerLogDeliveryEl {
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `firehose`.\n"]
    pub fn set_firehose(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.firehose = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.firehose = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskconnectConnectorLogDeliveryElWorkerLogDeliveryEl {
    type O = terrars::BlockAssignable<MskconnectConnectorLogDeliveryElWorkerLogDeliveryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnectorLogDeliveryElWorkerLogDeliveryEl {}

impl BuildMskconnectConnectorLogDeliveryElWorkerLogDeliveryEl {
    pub fn build(self) -> MskconnectConnectorLogDeliveryElWorkerLogDeliveryEl {
        MskconnectConnectorLogDeliveryElWorkerLogDeliveryEl {
            cloudwatch_logs: core::default::Default::default(),
            firehose: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MskconnectConnectorLogDeliveryElWorkerLogDeliveryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnectorLogDeliveryElWorkerLogDeliveryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnectorLogDeliveryElWorkerLogDeliveryElRef {
        MskconnectConnectorLogDeliveryElWorkerLogDeliveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnectorLogDeliveryElWorkerLogDeliveryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(
        &self,
    ) -> terrars::ListRef<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `firehose` after provisioning.\n"]
    pub fn firehose(&self) -> terrars::ListRef<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firehose", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3ElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskconnectConnectorLogDeliveryElDynamic {
    worker_log_delivery: std
    ::option
    ::Option<terrars::DynamicBlock<MskconnectConnectorLogDeliveryElWorkerLogDeliveryEl>>,
}

#[derive(Serialize)]
pub struct MskconnectConnectorLogDeliveryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_log_delivery: std::option::Option<std::vec::Vec<MskconnectConnectorLogDeliveryElWorkerLogDeliveryEl>>,
    dynamic: MskconnectConnectorLogDeliveryElDynamic,
}

impl MskconnectConnectorLogDeliveryEl {
    #[doc = "Set the field `worker_log_delivery`.\n"]
    pub fn set_worker_log_delivery(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnectorLogDeliveryElWorkerLogDeliveryEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.worker_log_delivery = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.worker_log_delivery = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskconnectConnectorLogDeliveryEl {
    type O = terrars::BlockAssignable<MskconnectConnectorLogDeliveryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnectorLogDeliveryEl {}

impl BuildMskconnectConnectorLogDeliveryEl {
    pub fn build(self) -> MskconnectConnectorLogDeliveryEl {
        MskconnectConnectorLogDeliveryEl {
            worker_log_delivery: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MskconnectConnectorLogDeliveryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnectorLogDeliveryElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskconnectConnectorLogDeliveryElRef {
        MskconnectConnectorLogDeliveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnectorLogDeliveryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `worker_log_delivery` after provisioning.\n"]
    pub fn worker_log_delivery(&self) -> terrars::ListRef<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker_log_delivery", self.base))
    }
}

#[derive(Serialize)]
pub struct MskconnectConnectorPluginElCustomPluginEl {
    arn: terrars::PrimField<std::string::String>,
    revision: terrars::PrimField<f64>,
}

impl MskconnectConnectorPluginElCustomPluginEl { }

impl terrars::ToListMappable for MskconnectConnectorPluginElCustomPluginEl {
    type O = terrars::BlockAssignable<MskconnectConnectorPluginElCustomPluginEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnectorPluginElCustomPluginEl {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub revision: terrars::PrimField<f64>,
}

impl BuildMskconnectConnectorPluginElCustomPluginEl {
    pub fn build(self) -> MskconnectConnectorPluginElCustomPluginEl {
        MskconnectConnectorPluginElCustomPluginEl {
            arn: self.arn,
            revision: self.revision,
        }
    }
}

pub struct MskconnectConnectorPluginElCustomPluginElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnectorPluginElCustomPluginElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskconnectConnectorPluginElCustomPluginElRef {
        MskconnectConnectorPluginElCustomPluginElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnectorPluginElCustomPluginElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskconnectConnectorPluginElDynamic {
    custom_plugin: std::option::Option<terrars::DynamicBlock<MskconnectConnectorPluginElCustomPluginEl>>,
}

#[derive(Serialize)]
pub struct MskconnectConnectorPluginEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_plugin: std::option::Option<std::vec::Vec<MskconnectConnectorPluginElCustomPluginEl>>,
    dynamic: MskconnectConnectorPluginElDynamic,
}

impl MskconnectConnectorPluginEl {
    #[doc = "Set the field `custom_plugin`.\n"]
    pub fn set_custom_plugin(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnectorPluginElCustomPluginEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_plugin = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_plugin = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskconnectConnectorPluginEl {
    type O = terrars::BlockAssignable<MskconnectConnectorPluginEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnectorPluginEl {}

impl BuildMskconnectConnectorPluginEl {
    pub fn build(self) -> MskconnectConnectorPluginEl {
        MskconnectConnectorPluginEl {
            custom_plugin: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MskconnectConnectorPluginElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnectorPluginElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskconnectConnectorPluginElRef {
        MskconnectConnectorPluginElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnectorPluginElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_plugin` after provisioning.\n"]
    pub fn custom_plugin(&self) -> terrars::ListRef<MskconnectConnectorPluginElCustomPluginElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_plugin", self.base))
    }
}

#[derive(Serialize)]
pub struct MskconnectConnectorTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskconnectConnectorTimeoutsEl {
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

impl terrars::ToListMappable for MskconnectConnectorTimeoutsEl {
    type O = terrars::BlockAssignable<MskconnectConnectorTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnectorTimeoutsEl {}

impl BuildMskconnectConnectorTimeoutsEl {
    pub fn build(self) -> MskconnectConnectorTimeoutsEl {
        MskconnectConnectorTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct MskconnectConnectorTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnectorTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskconnectConnectorTimeoutsElRef {
        MskconnectConnectorTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnectorTimeoutsElRef {
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
pub struct MskconnectConnectorWorkerConfigurationEl {
    arn: terrars::PrimField<std::string::String>,
    revision: terrars::PrimField<f64>,
}

impl MskconnectConnectorWorkerConfigurationEl { }

impl terrars::ToListMappable for MskconnectConnectorWorkerConfigurationEl {
    type O = terrars::BlockAssignable<MskconnectConnectorWorkerConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnectorWorkerConfigurationEl {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub revision: terrars::PrimField<f64>,
}

impl BuildMskconnectConnectorWorkerConfigurationEl {
    pub fn build(self) -> MskconnectConnectorWorkerConfigurationEl {
        MskconnectConnectorWorkerConfigurationEl {
            arn: self.arn,
            revision: self.revision,
        }
    }
}

pub struct MskconnectConnectorWorkerConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnectorWorkerConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskconnectConnectorWorkerConfigurationElRef {
        MskconnectConnectorWorkerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnectorWorkerConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskconnectConnectorDynamic {
    capacity: std::option::Option<terrars::DynamicBlock<MskconnectConnectorCapacityEl>>,
    kafka_cluster: std::option::Option<terrars::DynamicBlock<MskconnectConnectorKafkaClusterEl>>,
    kafka_cluster_client_authentication: std
    ::option
    ::Option<terrars::DynamicBlock<MskconnectConnectorKafkaClusterClientAuthenticationEl>>,
    kafka_cluster_encryption_in_transit: std
    ::option
    ::Option<terrars::DynamicBlock<MskconnectConnectorKafkaClusterEncryptionInTransitEl>>,
    log_delivery: std::option::Option<terrars::DynamicBlock<MskconnectConnectorLogDeliveryEl>>,
    plugin: std::option::Option<terrars::DynamicBlock<MskconnectConnectorPluginEl>>,
    worker_configuration: std::option::Option<terrars::DynamicBlock<MskconnectConnectorWorkerConfigurationEl>>,
}
