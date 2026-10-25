#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MskconnectConnectorData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    service_execution_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity: std::option::Option<std::vec::Vec<MskconnectConnector_Capacity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kafka_cluster: std::option::Option<std::vec::Vec<MskconnectConnector_KafkaCluster>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kafka_cluster_client_authentication: std
    ::option
    ::Option<std::vec::Vec<MskconnectConnector_KafkaClusterClientAuthentication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kafka_cluster_encryption_in_transit: std
    ::option
    ::Option<std::vec::Vec<MskconnectConnector_KafkaClusterEncryptionInTransit>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_delivery: std::option::Option<std::vec::Vec<MskconnectConnector_LogDelivery>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plugin: std::option::Option<std::vec::Vec<MskconnectConnector_Plugin>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MskconnectConnector_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_configuration: std::option::Option<std::vec::Vec<MskconnectConnector_WorkerConfiguration>>,
    dynamic: MskconnectConnectorDynamic,
}

struct MskconnectConnector_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MskconnectConnectorData_>,
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity`.\n"]
    pub fn set_capacity(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnector_Capacity>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnector_KafkaCluster>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnector_KafkaClusterClientAuthentication>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnector_KafkaClusterEncryptionInTransit>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnector_LogDelivery>>,
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
    pub fn set_plugin(self, v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnector_Plugin>>) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<MskconnectConnector_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `worker_configuration`.\n"]
    pub fn set_worker_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnector_WorkerConfiguration>>,
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_execution_role_arn` after provisioning.\n"]
    pub fn service_execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> terrars::ListRef<MskconnectConnector_CapacityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_cluster` after provisioning.\n"]
    pub fn kafka_cluster(&self) -> terrars::ListRef<MskconnectConnector_KafkaClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kafka_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_cluster_client_authentication` after provisioning.\n"]
    pub fn kafka_cluster_client_authentication(
        &self,
    ) -> terrars::ListRef<MskconnectConnector_KafkaClusterClientAuthenticationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.kafka_cluster_client_authentication", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kafka_cluster_encryption_in_transit` after provisioning.\n"]
    pub fn kafka_cluster_encryption_in_transit(
        &self,
    ) -> terrars::ListRef<MskconnectConnector_KafkaClusterEncryptionInTransitRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.kafka_cluster_encryption_in_transit", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_delivery` after provisioning.\n"]
    pub fn log_delivery(&self) -> terrars::ListRef<MskconnectConnector_LogDeliveryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_delivery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskconnectConnector_TimeoutsRef {
        MskconnectConnector_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_configuration` after provisioning.\n"]
    pub fn worker_configuration(&self) -> terrars::ListRef<MskconnectConnector_WorkerConfigurationRef> {
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
            data: std::cell::RefCell::new(MskconnectConnectorData_ {
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
                region: core::default::Default::default(),
                service_execution_role_arn: self.service_execution_role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_execution_role_arn` after provisioning.\n"]
    pub fn service_execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> terrars::ListRef<MskconnectConnector_CapacityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_cluster` after provisioning.\n"]
    pub fn kafka_cluster(&self) -> terrars::ListRef<MskconnectConnector_KafkaClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kafka_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_cluster_client_authentication` after provisioning.\n"]
    pub fn kafka_cluster_client_authentication(
        &self,
    ) -> terrars::ListRef<MskconnectConnector_KafkaClusterClientAuthenticationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.kafka_cluster_client_authentication", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kafka_cluster_encryption_in_transit` after provisioning.\n"]
    pub fn kafka_cluster_encryption_in_transit(
        &self,
    ) -> terrars::ListRef<MskconnectConnector_KafkaClusterEncryptionInTransitRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.kafka_cluster_encryption_in_transit", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_delivery` after provisioning.\n"]
    pub fn log_delivery(&self) -> terrars::ListRef<MskconnectConnector_LogDeliveryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_delivery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskconnectConnector_TimeoutsRef {
        MskconnectConnector_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_configuration` after provisioning.\n"]
    pub fn worker_configuration(&self) -> terrars::ListRef<MskconnectConnector_WorkerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct MskconnectConnectorDynamic {
    capacity: std::option::Option<terrars::DynamicBlock<MskconnectConnector_Capacity>>,
    kafka_cluster: std::option::Option<terrars::DynamicBlock<MskconnectConnector_KafkaCluster>>,
    kafka_cluster_client_authentication: std
    ::option
    ::Option<terrars::DynamicBlock<MskconnectConnector_KafkaClusterClientAuthentication>>,
    kafka_cluster_encryption_in_transit: std
    ::option
    ::Option<terrars::DynamicBlock<MskconnectConnector_KafkaClusterEncryptionInTransit>>,
    log_delivery: std::option::Option<terrars::DynamicBlock<MskconnectConnector_LogDelivery>>,
    plugin: std::option::Option<terrars::DynamicBlock<MskconnectConnector_Plugin>>,
    worker_configuration: std::option::Option<terrars::DynamicBlock<MskconnectConnector_WorkerConfiguration>>,
}

#[derive(Serialize)]
pub struct MskconnectConnector_Capacity_Autoscaling_ScaleInPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_utilization_percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl MskconnectConnector_Capacity_Autoscaling_ScaleInPolicy {
    #[doc = "Set the field `cpu_utilization_percentage`.\n"]
    pub fn set_cpu_utilization_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_utilization_percentage = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskconnectConnector_Capacity_Autoscaling_ScaleInPolicy {
    type O = terrars::BlockAssignable<MskconnectConnector_Capacity_Autoscaling_ScaleInPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnector_Capacity_Autoscaling_ScaleInPolicy {}

impl BuildMskconnectConnector_Capacity_Autoscaling_ScaleInPolicy {
    pub fn build(self) -> MskconnectConnector_Capacity_Autoscaling_ScaleInPolicy {
        MskconnectConnector_Capacity_Autoscaling_ScaleInPolicy {
            cpu_utilization_percentage: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskconnectConnector_Capacity_Autoscaling_ScaleInPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnector_Capacity_Autoscaling_ScaleInPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnector_Capacity_Autoscaling_ScaleInPolicyRef {
        MskconnectConnector_Capacity_Autoscaling_ScaleInPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnector_Capacity_Autoscaling_ScaleInPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu_utilization_percentage` after provisioning.\n"]
    pub fn cpu_utilization_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_utilization_percentage", self.base))
    }
}

#[derive(Serialize)]
pub struct MskconnectConnector_Capacity_Autoscaling_ScaleOutPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_utilization_percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl MskconnectConnector_Capacity_Autoscaling_ScaleOutPolicy {
    #[doc = "Set the field `cpu_utilization_percentage`.\n"]
    pub fn set_cpu_utilization_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_utilization_percentage = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskconnectConnector_Capacity_Autoscaling_ScaleOutPolicy {
    type O = terrars::BlockAssignable<MskconnectConnector_Capacity_Autoscaling_ScaleOutPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnector_Capacity_Autoscaling_ScaleOutPolicy {}

impl BuildMskconnectConnector_Capacity_Autoscaling_ScaleOutPolicy {
    pub fn build(self) -> MskconnectConnector_Capacity_Autoscaling_ScaleOutPolicy {
        MskconnectConnector_Capacity_Autoscaling_ScaleOutPolicy {
            cpu_utilization_percentage: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskconnectConnector_Capacity_Autoscaling_ScaleOutPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnector_Capacity_Autoscaling_ScaleOutPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnector_Capacity_Autoscaling_ScaleOutPolicyRef {
        MskconnectConnector_Capacity_Autoscaling_ScaleOutPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnector_Capacity_Autoscaling_ScaleOutPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu_utilization_percentage` after provisioning.\n"]
    pub fn cpu_utilization_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_utilization_percentage", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskconnectConnector_Capacity_AutoscalingDynamic {
    scale_in_policy: std
    ::option
    ::Option<terrars::DynamicBlock<MskconnectConnector_Capacity_Autoscaling_ScaleInPolicy>>,
    scale_out_policy: std
    ::option
    ::Option<terrars::DynamicBlock<MskconnectConnector_Capacity_Autoscaling_ScaleOutPolicy>>,
}

#[derive(Serialize)]
pub struct MskconnectConnector_Capacity_Autoscaling {
    max_worker_count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mcu_count: std::option::Option<terrars::PrimField<f64>>,
    min_worker_count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_in_policy: std::option::Option<std::vec::Vec<MskconnectConnector_Capacity_Autoscaling_ScaleInPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_out_policy: std::option::Option<std::vec::Vec<MskconnectConnector_Capacity_Autoscaling_ScaleOutPolicy>>,
    dynamic: MskconnectConnector_Capacity_AutoscalingDynamic,
}

impl MskconnectConnector_Capacity_Autoscaling {
    #[doc = "Set the field `mcu_count`.\n"]
    pub fn set_mcu_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.mcu_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_in_policy`.\n"]
    pub fn set_scale_in_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnector_Capacity_Autoscaling_ScaleInPolicy>>,
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
                    ::Into<terrars::BlockAssignable<MskconnectConnector_Capacity_Autoscaling_ScaleOutPolicy>>,
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

impl terrars::ToListMappable for MskconnectConnector_Capacity_Autoscaling {
    type O = terrars::BlockAssignable<MskconnectConnector_Capacity_Autoscaling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnector_Capacity_Autoscaling {
    #[doc = ""]
    pub max_worker_count: terrars::PrimField<f64>,
    #[doc = ""]
    pub min_worker_count: terrars::PrimField<f64>,
}

impl BuildMskconnectConnector_Capacity_Autoscaling {
    pub fn build(self) -> MskconnectConnector_Capacity_Autoscaling {
        MskconnectConnector_Capacity_Autoscaling {
            max_worker_count: self.max_worker_count,
            mcu_count: core::default::Default::default(),
            min_worker_count: self.min_worker_count,
            scale_in_policy: core::default::Default::default(),
            scale_out_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskconnectConnector_Capacity_AutoscalingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnector_Capacity_AutoscalingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskconnectConnector_Capacity_AutoscalingRef {
        MskconnectConnector_Capacity_AutoscalingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnector_Capacity_AutoscalingRef {
    #[allow(dead_code)]
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
    pub fn scale_in_policy(&self) -> terrars::ListRef<MskconnectConnector_Capacity_Autoscaling_ScaleInPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scale_in_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_out_policy` after provisioning.\n"]
    pub fn scale_out_policy(&self) -> terrars::ListRef<MskconnectConnector_Capacity_Autoscaling_ScaleOutPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scale_out_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct MskconnectConnector_Capacity_ProvisionedCapacity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mcu_count: std::option::Option<terrars::PrimField<f64>>,
    worker_count: terrars::PrimField<f64>,
}

impl MskconnectConnector_Capacity_ProvisionedCapacity {
    #[doc = "Set the field `mcu_count`.\n"]
    pub fn set_mcu_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.mcu_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskconnectConnector_Capacity_ProvisionedCapacity {
    type O = terrars::BlockAssignable<MskconnectConnector_Capacity_ProvisionedCapacity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnector_Capacity_ProvisionedCapacity {
    #[doc = ""]
    pub worker_count: terrars::PrimField<f64>,
}

impl BuildMskconnectConnector_Capacity_ProvisionedCapacity {
    pub fn build(self) -> MskconnectConnector_Capacity_ProvisionedCapacity {
        MskconnectConnector_Capacity_ProvisionedCapacity {
            mcu_count: core::default::Default::default(),
            worker_count: self.worker_count,
        }
    }
}

#[allow(dead_code)]
pub struct MskconnectConnector_Capacity_ProvisionedCapacityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnector_Capacity_ProvisionedCapacityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnector_Capacity_ProvisionedCapacityRef {
        MskconnectConnector_Capacity_ProvisionedCapacityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnector_Capacity_ProvisionedCapacityRef {
    #[allow(dead_code)]
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
struct MskconnectConnector_CapacityDynamic {
    autoscaling: std::option::Option<terrars::DynamicBlock<MskconnectConnector_Capacity_Autoscaling>>,
    provisioned_capacity: std
    ::option
    ::Option<terrars::DynamicBlock<MskconnectConnector_Capacity_ProvisionedCapacity>>,
}

#[derive(Serialize)]
pub struct MskconnectConnector_Capacity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling: std::option::Option<std::vec::Vec<MskconnectConnector_Capacity_Autoscaling>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_capacity: std::option::Option<std::vec::Vec<MskconnectConnector_Capacity_ProvisionedCapacity>>,
    dynamic: MskconnectConnector_CapacityDynamic,
}

impl MskconnectConnector_Capacity {
    #[doc = "Set the field `autoscaling`.\n"]
    pub fn set_autoscaling(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnector_Capacity_Autoscaling>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnector_Capacity_ProvisionedCapacity>>,
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

impl terrars::ToListMappable for MskconnectConnector_Capacity {
    type O = terrars::BlockAssignable<MskconnectConnector_Capacity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnector_Capacity {}

impl BuildMskconnectConnector_Capacity {
    pub fn build(self) -> MskconnectConnector_Capacity {
        MskconnectConnector_Capacity {
            autoscaling: core::default::Default::default(),
            provisioned_capacity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskconnectConnector_CapacityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnector_CapacityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskconnectConnector_CapacityRef {
        MskconnectConnector_CapacityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnector_CapacityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> terrars::ListRef<MskconnectConnector_Capacity_AutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioned_capacity` after provisioning.\n"]
    pub fn provisioned_capacity(&self) -> terrars::ListRef<MskconnectConnector_Capacity_ProvisionedCapacityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provisioned_capacity", self.base))
    }
}

#[derive(Serialize)]
pub struct MskconnectConnector_KafkaCluster_ApacheKafkaCluster_Vpc {
    security_groups: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl MskconnectConnector_KafkaCluster_ApacheKafkaCluster_Vpc { }

impl terrars::ToListMappable for MskconnectConnector_KafkaCluster_ApacheKafkaCluster_Vpc {
    type O = terrars::BlockAssignable<MskconnectConnector_KafkaCluster_ApacheKafkaCluster_Vpc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnector_KafkaCluster_ApacheKafkaCluster_Vpc {
    #[doc = ""]
    pub security_groups: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildMskconnectConnector_KafkaCluster_ApacheKafkaCluster_Vpc {
    pub fn build(self) -> MskconnectConnector_KafkaCluster_ApacheKafkaCluster_Vpc {
        MskconnectConnector_KafkaCluster_ApacheKafkaCluster_Vpc {
            security_groups: self.security_groups,
            subnets: self.subnets,
        }
    }
}

#[allow(dead_code)]
pub struct MskconnectConnector_KafkaCluster_ApacheKafkaCluster_VpcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnector_KafkaCluster_ApacheKafkaCluster_VpcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnector_KafkaCluster_ApacheKafkaCluster_VpcRef {
        MskconnectConnector_KafkaCluster_ApacheKafkaCluster_VpcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnector_KafkaCluster_ApacheKafkaCluster_VpcRef {
    #[allow(dead_code)]
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
struct MskconnectConnector_KafkaCluster_ApacheKafkaClusterDynamic {
    vpc: std::option::Option<terrars::DynamicBlock<MskconnectConnector_KafkaCluster_ApacheKafkaCluster_Vpc>>,
}

#[derive(Serialize)]
pub struct MskconnectConnector_KafkaCluster_ApacheKafkaCluster {
    bootstrap_servers: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<std::vec::Vec<MskconnectConnector_KafkaCluster_ApacheKafkaCluster_Vpc>>,
    dynamic: MskconnectConnector_KafkaCluster_ApacheKafkaClusterDynamic,
}

impl MskconnectConnector_KafkaCluster_ApacheKafkaCluster {
    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MskconnectConnector_KafkaCluster_ApacheKafkaCluster_Vpc>>,
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

impl terrars::ToListMappable for MskconnectConnector_KafkaCluster_ApacheKafkaCluster {
    type O = terrars::BlockAssignable<MskconnectConnector_KafkaCluster_ApacheKafkaCluster>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnector_KafkaCluster_ApacheKafkaCluster {
    #[doc = ""]
    pub bootstrap_servers: terrars::PrimField<std::string::String>,
}

impl BuildMskconnectConnector_KafkaCluster_ApacheKafkaCluster {
    pub fn build(self) -> MskconnectConnector_KafkaCluster_ApacheKafkaCluster {
        MskconnectConnector_KafkaCluster_ApacheKafkaCluster {
            bootstrap_servers: self.bootstrap_servers,
            vpc: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskconnectConnector_KafkaCluster_ApacheKafkaClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnector_KafkaCluster_ApacheKafkaClusterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnector_KafkaCluster_ApacheKafkaClusterRef {
        MskconnectConnector_KafkaCluster_ApacheKafkaClusterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnector_KafkaCluster_ApacheKafkaClusterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bootstrap_servers` after provisioning.\n"]
    pub fn bootstrap_servers(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_servers", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<MskconnectConnector_KafkaCluster_ApacheKafkaCluster_VpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskconnectConnector_KafkaClusterDynamic {
    apache_kafka_cluster: std
    ::option
    ::Option<terrars::DynamicBlock<MskconnectConnector_KafkaCluster_ApacheKafkaCluster>>,
}

#[derive(Serialize)]
pub struct MskconnectConnector_KafkaCluster {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    apache_kafka_cluster: std::option::Option<std::vec::Vec<MskconnectConnector_KafkaCluster_ApacheKafkaCluster>>,
    dynamic: MskconnectConnector_KafkaClusterDynamic,
}

impl MskconnectConnector_KafkaCluster {
    #[doc = "Set the field `apache_kafka_cluster`.\n"]
    pub fn set_apache_kafka_cluster(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnector_KafkaCluster_ApacheKafkaCluster>>,
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

impl terrars::ToListMappable for MskconnectConnector_KafkaCluster {
    type O = terrars::BlockAssignable<MskconnectConnector_KafkaCluster>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnector_KafkaCluster {}

impl BuildMskconnectConnector_KafkaCluster {
    pub fn build(self) -> MskconnectConnector_KafkaCluster {
        MskconnectConnector_KafkaCluster {
            apache_kafka_cluster: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskconnectConnector_KafkaClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnector_KafkaClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskconnectConnector_KafkaClusterRef {
        MskconnectConnector_KafkaClusterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnector_KafkaClusterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apache_kafka_cluster` after provisioning.\n"]
    pub fn apache_kafka_cluster(&self) -> terrars::ListRef<MskconnectConnector_KafkaCluster_ApacheKafkaClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.apache_kafka_cluster", self.base))
    }
}

#[derive(Serialize)]
pub struct MskconnectConnector_KafkaClusterClientAuthentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskconnectConnector_KafkaClusterClientAuthentication {
    #[doc = "Set the field `authentication_type`.\n"]
    pub fn set_authentication_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authentication_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskconnectConnector_KafkaClusterClientAuthentication {
    type O = terrars::BlockAssignable<MskconnectConnector_KafkaClusterClientAuthentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnector_KafkaClusterClientAuthentication {}

impl BuildMskconnectConnector_KafkaClusterClientAuthentication {
    pub fn build(self) -> MskconnectConnector_KafkaClusterClientAuthentication {
        MskconnectConnector_KafkaClusterClientAuthentication {
            authentication_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskconnectConnector_KafkaClusterClientAuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnector_KafkaClusterClientAuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnector_KafkaClusterClientAuthenticationRef {
        MskconnectConnector_KafkaClusterClientAuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnector_KafkaClusterClientAuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication_type` after provisioning.\n"]
    pub fn authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_type", self.base))
    }
}

#[derive(Serialize)]
pub struct MskconnectConnector_KafkaClusterEncryptionInTransit {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskconnectConnector_KafkaClusterEncryptionInTransit {
    #[doc = "Set the field `encryption_type`.\n"]
    pub fn set_encryption_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encryption_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskconnectConnector_KafkaClusterEncryptionInTransit {
    type O = terrars::BlockAssignable<MskconnectConnector_KafkaClusterEncryptionInTransit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnector_KafkaClusterEncryptionInTransit {}

impl BuildMskconnectConnector_KafkaClusterEncryptionInTransit {
    pub fn build(self) -> MskconnectConnector_KafkaClusterEncryptionInTransit {
        MskconnectConnector_KafkaClusterEncryptionInTransit { encryption_type: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MskconnectConnector_KafkaClusterEncryptionInTransitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnector_KafkaClusterEncryptionInTransitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnector_KafkaClusterEncryptionInTransitRef {
        MskconnectConnector_KafkaClusterEncryptionInTransitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnector_KafkaClusterEncryptionInTransitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\n"]
    pub fn encryption_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_type", self.base))
    }
}

#[derive(Serialize)]
pub struct MskconnectConnector_LogDelivery_WorkerLogDelivery_CloudwatchLogs {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskconnectConnector_LogDelivery_WorkerLogDelivery_CloudwatchLogs {
    #[doc = "Set the field `log_group`.\n"]
    pub fn set_log_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_group = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskconnectConnector_LogDelivery_WorkerLogDelivery_CloudwatchLogs {
    type O = terrars::BlockAssignable<MskconnectConnector_LogDelivery_WorkerLogDelivery_CloudwatchLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnector_LogDelivery_WorkerLogDelivery_CloudwatchLogs {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildMskconnectConnector_LogDelivery_WorkerLogDelivery_CloudwatchLogs {
    pub fn build(self) -> MskconnectConnector_LogDelivery_WorkerLogDelivery_CloudwatchLogs {
        MskconnectConnector_LogDelivery_WorkerLogDelivery_CloudwatchLogs {
            enabled: self.enabled,
            log_group: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskconnectConnector_LogDelivery_WorkerLogDelivery_CloudwatchLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnector_LogDelivery_WorkerLogDelivery_CloudwatchLogsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnector_LogDelivery_WorkerLogDelivery_CloudwatchLogsRef {
        MskconnectConnector_LogDelivery_WorkerLogDelivery_CloudwatchLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnector_LogDelivery_WorkerLogDelivery_CloudwatchLogsRef {
    #[allow(dead_code)]
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
pub struct MskconnectConnector_LogDelivery_WorkerLogDelivery_Firehose {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delivery_stream: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
}

impl MskconnectConnector_LogDelivery_WorkerLogDelivery_Firehose {
    #[doc = "Set the field `delivery_stream`.\n"]
    pub fn set_delivery_stream(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delivery_stream = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskconnectConnector_LogDelivery_WorkerLogDelivery_Firehose {
    type O = terrars::BlockAssignable<MskconnectConnector_LogDelivery_WorkerLogDelivery_Firehose>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnector_LogDelivery_WorkerLogDelivery_Firehose {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildMskconnectConnector_LogDelivery_WorkerLogDelivery_Firehose {
    pub fn build(self) -> MskconnectConnector_LogDelivery_WorkerLogDelivery_Firehose {
        MskconnectConnector_LogDelivery_WorkerLogDelivery_Firehose {
            delivery_stream: core::default::Default::default(),
            enabled: self.enabled,
        }
    }
}

#[allow(dead_code)]
pub struct MskconnectConnector_LogDelivery_WorkerLogDelivery_FirehoseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnector_LogDelivery_WorkerLogDelivery_FirehoseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnector_LogDelivery_WorkerLogDelivery_FirehoseRef {
        MskconnectConnector_LogDelivery_WorkerLogDelivery_FirehoseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnector_LogDelivery_WorkerLogDelivery_FirehoseRef {
    #[allow(dead_code)]
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
pub struct MskconnectConnector_LogDelivery_WorkerLogDelivery_S3 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskconnectConnector_LogDelivery_WorkerLogDelivery_S3 {
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

impl terrars::ToListMappable for MskconnectConnector_LogDelivery_WorkerLogDelivery_S3 {
    type O = terrars::BlockAssignable<MskconnectConnector_LogDelivery_WorkerLogDelivery_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnector_LogDelivery_WorkerLogDelivery_S3 {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildMskconnectConnector_LogDelivery_WorkerLogDelivery_S3 {
    pub fn build(self) -> MskconnectConnector_LogDelivery_WorkerLogDelivery_S3 {
        MskconnectConnector_LogDelivery_WorkerLogDelivery_S3 {
            bucket: core::default::Default::default(),
            enabled: self.enabled,
            prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskconnectConnector_LogDelivery_WorkerLogDelivery_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnector_LogDelivery_WorkerLogDelivery_S3Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnector_LogDelivery_WorkerLogDelivery_S3Ref {
        MskconnectConnector_LogDelivery_WorkerLogDelivery_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnector_LogDelivery_WorkerLogDelivery_S3Ref {
    #[allow(dead_code)]
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
struct MskconnectConnector_LogDelivery_WorkerLogDeliveryDynamic {
    cloudwatch_logs: std
    ::option
    ::Option<terrars::DynamicBlock<MskconnectConnector_LogDelivery_WorkerLogDelivery_CloudwatchLogs>>,
    firehose: std::option::Option<terrars::DynamicBlock<MskconnectConnector_LogDelivery_WorkerLogDelivery_Firehose>>,
    s3: std::option::Option<terrars::DynamicBlock<MskconnectConnector_LogDelivery_WorkerLogDelivery_S3>>,
}

#[derive(Serialize)]
pub struct MskconnectConnector_LogDelivery_WorkerLogDelivery {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs: std::option::Option<std::vec::Vec<MskconnectConnector_LogDelivery_WorkerLogDelivery_CloudwatchLogs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firehose: std::option::Option<std::vec::Vec<MskconnectConnector_LogDelivery_WorkerLogDelivery_Firehose>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<MskconnectConnector_LogDelivery_WorkerLogDelivery_S3>>,
    dynamic: MskconnectConnector_LogDelivery_WorkerLogDeliveryDynamic,
}

impl MskconnectConnector_LogDelivery_WorkerLogDelivery {
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MskconnectConnector_LogDelivery_WorkerLogDelivery_CloudwatchLogs>,
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
                    ::Into<terrars::BlockAssignable<MskconnectConnector_LogDelivery_WorkerLogDelivery_Firehose>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnector_LogDelivery_WorkerLogDelivery_S3>>,
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

impl terrars::ToListMappable for MskconnectConnector_LogDelivery_WorkerLogDelivery {
    type O = terrars::BlockAssignable<MskconnectConnector_LogDelivery_WorkerLogDelivery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnector_LogDelivery_WorkerLogDelivery {}

impl BuildMskconnectConnector_LogDelivery_WorkerLogDelivery {
    pub fn build(self) -> MskconnectConnector_LogDelivery_WorkerLogDelivery {
        MskconnectConnector_LogDelivery_WorkerLogDelivery {
            cloudwatch_logs: core::default::Default::default(),
            firehose: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskconnectConnector_LogDelivery_WorkerLogDeliveryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnector_LogDelivery_WorkerLogDeliveryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskconnectConnector_LogDelivery_WorkerLogDeliveryRef {
        MskconnectConnector_LogDelivery_WorkerLogDeliveryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnector_LogDelivery_WorkerLogDeliveryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(
        &self,
    ) -> terrars::ListRef<MskconnectConnector_LogDelivery_WorkerLogDelivery_CloudwatchLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `firehose` after provisioning.\n"]
    pub fn firehose(&self) -> terrars::ListRef<MskconnectConnector_LogDelivery_WorkerLogDelivery_FirehoseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firehose", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<MskconnectConnector_LogDelivery_WorkerLogDelivery_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskconnectConnector_LogDeliveryDynamic {
    worker_log_delivery: std
    ::option
    ::Option<terrars::DynamicBlock<MskconnectConnector_LogDelivery_WorkerLogDelivery>>,
}

#[derive(Serialize)]
pub struct MskconnectConnector_LogDelivery {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_log_delivery: std::option::Option<std::vec::Vec<MskconnectConnector_LogDelivery_WorkerLogDelivery>>,
    dynamic: MskconnectConnector_LogDeliveryDynamic,
}

impl MskconnectConnector_LogDelivery {
    #[doc = "Set the field `worker_log_delivery`.\n"]
    pub fn set_worker_log_delivery(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnector_LogDelivery_WorkerLogDelivery>>,
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

impl terrars::ToListMappable for MskconnectConnector_LogDelivery {
    type O = terrars::BlockAssignable<MskconnectConnector_LogDelivery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnector_LogDelivery {}

impl BuildMskconnectConnector_LogDelivery {
    pub fn build(self) -> MskconnectConnector_LogDelivery {
        MskconnectConnector_LogDelivery {
            worker_log_delivery: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskconnectConnector_LogDeliveryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnector_LogDeliveryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskconnectConnector_LogDeliveryRef {
        MskconnectConnector_LogDeliveryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnector_LogDeliveryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `worker_log_delivery` after provisioning.\n"]
    pub fn worker_log_delivery(&self) -> terrars::ListRef<MskconnectConnector_LogDelivery_WorkerLogDeliveryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker_log_delivery", self.base))
    }
}

#[derive(Serialize)]
pub struct MskconnectConnector_Plugin_CustomPlugin {
    arn: terrars::PrimField<std::string::String>,
    revision: terrars::PrimField<f64>,
}

impl MskconnectConnector_Plugin_CustomPlugin { }

impl terrars::ToListMappable for MskconnectConnector_Plugin_CustomPlugin {
    type O = terrars::BlockAssignable<MskconnectConnector_Plugin_CustomPlugin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnector_Plugin_CustomPlugin {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub revision: terrars::PrimField<f64>,
}

impl BuildMskconnectConnector_Plugin_CustomPlugin {
    pub fn build(self) -> MskconnectConnector_Plugin_CustomPlugin {
        MskconnectConnector_Plugin_CustomPlugin {
            arn: self.arn,
            revision: self.revision,
        }
    }
}

#[allow(dead_code)]
pub struct MskconnectConnector_Plugin_CustomPluginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnector_Plugin_CustomPluginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskconnectConnector_Plugin_CustomPluginRef {
        MskconnectConnector_Plugin_CustomPluginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnector_Plugin_CustomPluginRef {
    #[allow(dead_code)]
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
struct MskconnectConnector_PluginDynamic {
    custom_plugin: std::option::Option<terrars::DynamicBlock<MskconnectConnector_Plugin_CustomPlugin>>,
}

#[derive(Serialize)]
pub struct MskconnectConnector_Plugin {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_plugin: std::option::Option<std::vec::Vec<MskconnectConnector_Plugin_CustomPlugin>>,
    dynamic: MskconnectConnector_PluginDynamic,
}

impl MskconnectConnector_Plugin {
    #[doc = "Set the field `custom_plugin`.\n"]
    pub fn set_custom_plugin(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectConnector_Plugin_CustomPlugin>>,
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

impl terrars::ToListMappable for MskconnectConnector_Plugin {
    type O = terrars::BlockAssignable<MskconnectConnector_Plugin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnector_Plugin {}

impl BuildMskconnectConnector_Plugin {
    pub fn build(self) -> MskconnectConnector_Plugin {
        MskconnectConnector_Plugin {
            custom_plugin: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskconnectConnector_PluginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnector_PluginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskconnectConnector_PluginRef {
        MskconnectConnector_PluginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnector_PluginRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_plugin` after provisioning.\n"]
    pub fn custom_plugin(&self) -> terrars::ListRef<MskconnectConnector_Plugin_CustomPluginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_plugin", self.base))
    }
}

#[derive(Serialize)]
pub struct MskconnectConnector_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskconnectConnector_Timeouts {
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

impl terrars::ToListMappable for MskconnectConnector_Timeouts {
    type O = terrars::BlockAssignable<MskconnectConnector_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnector_Timeouts {}

impl BuildMskconnectConnector_Timeouts {
    pub fn build(self) -> MskconnectConnector_Timeouts {
        MskconnectConnector_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskconnectConnector_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnector_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskconnectConnector_TimeoutsRef {
        MskconnectConnector_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnector_TimeoutsRef {
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
pub struct MskconnectConnector_WorkerConfiguration {
    arn: terrars::PrimField<std::string::String>,
    revision: terrars::PrimField<f64>,
}

impl MskconnectConnector_WorkerConfiguration { }

impl terrars::ToListMappable for MskconnectConnector_WorkerConfiguration {
    type O = terrars::BlockAssignable<MskconnectConnector_WorkerConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectConnector_WorkerConfiguration {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub revision: terrars::PrimField<f64>,
}

impl BuildMskconnectConnector_WorkerConfiguration {
    pub fn build(self) -> MskconnectConnector_WorkerConfiguration {
        MskconnectConnector_WorkerConfiguration {
            arn: self.arn,
            revision: self.revision,
        }
    }
}

#[allow(dead_code)]
pub struct MskconnectConnector_WorkerConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectConnector_WorkerConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskconnectConnector_WorkerConfigurationRef {
        MskconnectConnector_WorkerConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectConnector_WorkerConfigurationRef {
    #[allow(dead_code)]
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
