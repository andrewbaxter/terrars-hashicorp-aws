#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MskClusterData_ {
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
    cluster_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enhanced_monitoring: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    kafka_version: terrars::PrimField<std::string::String>,
    number_of_broker_nodes: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    broker_node_group_info: std::option::Option<std::vec::Vec<MskCluster_BrokerNodeGroupInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_authentication: std::option::Option<std::vec::Vec<MskCluster_ClientAuthentication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration_info: std::option::Option<std::vec::Vec<MskCluster_ConfigurationInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_info: std::option::Option<std::vec::Vec<MskCluster_EncryptionInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_info: std::option::Option<std::vec::Vec<MskCluster_LoggingInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_monitoring: std::option::Option<std::vec::Vec<MskCluster_OpenMonitoring>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rebalancing: std::option::Option<std::vec::Vec<MskCluster_Rebalancing>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MskCluster_Timeouts>,
    dynamic: MskClusterDynamic,
}

struct MskCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MskClusterData_>,
}

#[derive(Clone)]
pub struct MskCluster(std::rc::Rc<MskCluster_>);

impl MskCluster {
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

    #[doc = "Set the field `enhanced_monitoring`.\n"]
    pub fn set_enhanced_monitoring(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().enhanced_monitoring = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `storage_mode`.\n"]
    pub fn set_storage_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().storage_mode = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `broker_node_group_info`.\n"]
    pub fn set_broker_node_group_info(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskCluster_BrokerNodeGroupInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().broker_node_group_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.broker_node_group_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `client_authentication`.\n"]
    pub fn set_client_authentication(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskCluster_ClientAuthentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_authentication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `configuration_info`.\n"]
    pub fn set_configuration_info(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskCluster_ConfigurationInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_info`.\n"]
    pub fn set_encryption_info(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskCluster_EncryptionInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logging_info`.\n"]
    pub fn set_logging_info(self, v: impl std::convert::Into<terrars::BlockAssignable<MskCluster_LoggingInfo>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `open_monitoring`.\n"]
    pub fn set_open_monitoring(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskCluster_OpenMonitoring>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().open_monitoring = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.open_monitoring = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rebalancing`.\n"]
    pub fn set_rebalancing(self, v: impl std::convert::Into<terrars::BlockAssignable<MskCluster_Rebalancing>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rebalancing = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rebalancing = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<MskCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers` after provisioning.\n"]
    pub fn bootstrap_brokers(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_ipv6` after provisioning.\n"]
    pub fn bootstrap_brokers_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_public_sasl_iam(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_public_sasl_iam", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_public_sasl_scram(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_public_sasl_scram", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_public_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_public_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_iam(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_sasl_iam", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_iam_ipv6` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_iam_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_sasl_iam_ipv6", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_scram(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_sasl_scram", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_scram_ipv6` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_scram_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_sasl_scram_ipv6", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_tls_ipv6` after provisioning.\n"]
    pub fn bootstrap_brokers_tls_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_tls_ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_vpc_connectivity_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_vpc_connectivity_sasl_iam(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_vpc_connectivity_sasl_iam", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `bootstrap_brokers_vpc_connectivity_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_vpc_connectivity_sasl_scram(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_vpc_connectivity_sasl_scram", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_vpc_connectivity_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_vpc_connectivity_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_vpc_connectivity_tls", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_uuid` after provisioning.\n"]
    pub fn cluster_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `current_version` after provisioning.\n"]
    pub fn current_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_action_status` after provisioning.\n"]
    pub fn customer_action_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_action_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enhanced_monitoring` after provisioning.\n"]
    pub fn enhanced_monitoring(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enhanced_monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_version` after provisioning.\n"]
    pub fn kafka_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kafka_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_broker_nodes` after provisioning.\n"]
    pub fn number_of_broker_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_broker_nodes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_mode` after provisioning.\n"]
    pub fn storage_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zookeeper_connect_string` after provisioning.\n"]
    pub fn zookeeper_connect_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zookeeper_connect_string", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zookeeper_connect_string_tls` after provisioning.\n"]
    pub fn zookeeper_connect_string_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zookeeper_connect_string_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `broker_node_group_info` after provisioning.\n"]
    pub fn broker_node_group_info(&self) -> terrars::ListRef<MskCluster_BrokerNodeGroupInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.broker_node_group_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_authentication` after provisioning.\n"]
    pub fn client_authentication(&self) -> terrars::ListRef<MskCluster_ClientAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_info` after provisioning.\n"]
    pub fn configuration_info(&self) -> terrars::ListRef<MskCluster_ConfigurationInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_info` after provisioning.\n"]
    pub fn encryption_info(&self) -> terrars::ListRef<MskCluster_EncryptionInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_info` after provisioning.\n"]
    pub fn logging_info(&self) -> terrars::ListRef<MskCluster_LoggingInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `open_monitoring` after provisioning.\n"]
    pub fn open_monitoring(&self) -> terrars::ListRef<MskCluster_OpenMonitoringRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rebalancing` after provisioning.\n"]
    pub fn rebalancing(&self) -> terrars::ListRef<MskCluster_RebalancingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rebalancing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskCluster_TimeoutsRef {
        MskCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for MskCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MskCluster { }

impl terrars::ToListMappable for MskCluster {
    type O = terrars::ListRef<MskClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MskCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_msk_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMskCluster {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub kafka_version: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub number_of_broker_nodes: terrars::PrimField<f64>,
}

impl BuildMskCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> MskCluster {
        let out = MskCluster(std::rc::Rc::new(MskCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MskClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cluster_name: self.cluster_name,
                enhanced_monitoring: core::default::Default::default(),
                id: core::default::Default::default(),
                kafka_version: self.kafka_version,
                number_of_broker_nodes: self.number_of_broker_nodes,
                region: core::default::Default::default(),
                storage_mode: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                broker_node_group_info: core::default::Default::default(),
                client_authentication: core::default::Default::default(),
                configuration_info: core::default::Default::default(),
                encryption_info: core::default::Default::default(),
                logging_info: core::default::Default::default(),
                open_monitoring: core::default::Default::default(),
                rebalancing: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MskClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MskClusterRef {
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

    #[doc = "Get a reference to the value of field `bootstrap_brokers` after provisioning.\n"]
    pub fn bootstrap_brokers(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_ipv6` after provisioning.\n"]
    pub fn bootstrap_brokers_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_public_sasl_iam(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_public_sasl_iam", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_public_sasl_scram(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_public_sasl_scram", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_public_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_public_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_iam(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_sasl_iam", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_iam_ipv6` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_iam_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_sasl_iam_ipv6", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_scram(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_sasl_scram", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_scram_ipv6` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_scram_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_sasl_scram_ipv6", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_tls_ipv6` after provisioning.\n"]
    pub fn bootstrap_brokers_tls_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_tls_ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_vpc_connectivity_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_vpc_connectivity_sasl_iam(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_vpc_connectivity_sasl_iam", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `bootstrap_brokers_vpc_connectivity_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_vpc_connectivity_sasl_scram(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_vpc_connectivity_sasl_scram", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_vpc_connectivity_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_vpc_connectivity_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_vpc_connectivity_tls", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_uuid` after provisioning.\n"]
    pub fn cluster_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `current_version` after provisioning.\n"]
    pub fn current_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_action_status` after provisioning.\n"]
    pub fn customer_action_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_action_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enhanced_monitoring` after provisioning.\n"]
    pub fn enhanced_monitoring(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enhanced_monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_version` after provisioning.\n"]
    pub fn kafka_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kafka_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_broker_nodes` after provisioning.\n"]
    pub fn number_of_broker_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_broker_nodes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_mode` after provisioning.\n"]
    pub fn storage_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zookeeper_connect_string` after provisioning.\n"]
    pub fn zookeeper_connect_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zookeeper_connect_string", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zookeeper_connect_string_tls` after provisioning.\n"]
    pub fn zookeeper_connect_string_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zookeeper_connect_string_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `broker_node_group_info` after provisioning.\n"]
    pub fn broker_node_group_info(&self) -> terrars::ListRef<MskCluster_BrokerNodeGroupInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.broker_node_group_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_authentication` after provisioning.\n"]
    pub fn client_authentication(&self) -> terrars::ListRef<MskCluster_ClientAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_info` after provisioning.\n"]
    pub fn configuration_info(&self) -> terrars::ListRef<MskCluster_ConfigurationInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_info` after provisioning.\n"]
    pub fn encryption_info(&self) -> terrars::ListRef<MskCluster_EncryptionInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_info` after provisioning.\n"]
    pub fn logging_info(&self) -> terrars::ListRef<MskCluster_LoggingInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `open_monitoring` after provisioning.\n"]
    pub fn open_monitoring(&self) -> terrars::ListRef<MskCluster_OpenMonitoringRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rebalancing` after provisioning.\n"]
    pub fn rebalancing(&self) -> terrars::ListRef<MskCluster_RebalancingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rebalancing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskCluster_TimeoutsRef {
        MskCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct MskClusterDynamic {
    broker_node_group_info: std::option::Option<terrars::DynamicBlock<MskCluster_BrokerNodeGroupInfo>>,
    client_authentication: std::option::Option<terrars::DynamicBlock<MskCluster_ClientAuthentication>>,
    configuration_info: std::option::Option<terrars::DynamicBlock<MskCluster_ConfigurationInfo>>,
    encryption_info: std::option::Option<terrars::DynamicBlock<MskCluster_EncryptionInfo>>,
    logging_info: std::option::Option<terrars::DynamicBlock<MskCluster_LoggingInfo>>,
    open_monitoring: std::option::Option<terrars::DynamicBlock<MskCluster_OpenMonitoring>>,
    rebalancing: std::option::Option<terrars::DynamicBlock<MskCluster_Rebalancing>>,
}

#[derive(Serialize)]
pub struct MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccess {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccess {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccess {
    type O = terrars::BlockAssignable<MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccess>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccess {}

impl BuildMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccess {
    pub fn build(self) -> MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccess {
        MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccess { type_: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccessRef {
        MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct MskCluster_ClientAuthentication_Sasl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scram: std::option::Option<terrars::PrimField<bool>>,
}

impl MskCluster_ClientAuthentication_Sasl {
    #[doc = "Set the field `iam`.\n"]
    pub fn set_iam(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.iam = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scram`.\n"]
    pub fn set_scram(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.scram = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskCluster_ClientAuthentication_Sasl {
    type O = terrars::BlockAssignable<MskCluster_ClientAuthentication_Sasl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_ClientAuthentication_Sasl {}

impl BuildMskCluster_ClientAuthentication_Sasl {
    pub fn build(self) -> MskCluster_ClientAuthentication_Sasl {
        MskCluster_ClientAuthentication_Sasl {
            iam: core::default::Default::default(),
            scram: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskCluster_ClientAuthentication_SaslRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_ClientAuthentication_SaslRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskCluster_ClientAuthentication_SaslRef {
        MskCluster_ClientAuthentication_SaslRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_ClientAuthentication_SaslRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iam` after provisioning.\n"]
    pub fn iam(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam", self.base))
    }

    #[doc = "Get a reference to the value of field `scram` after provisioning.\n"]
    pub fn scram(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scram", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthenticationDynamic {
    sasl: std::option::Option<terrars::DynamicBlock<MskCluster_ClientAuthentication_Sasl>>,
}

#[derive(Serialize)]
pub struct MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sasl: std::option::Option<std::vec::Vec<MskCluster_ClientAuthentication_Sasl>>,
    dynamic: MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthenticationDynamic,
}

impl MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication {
    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.tls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sasl`.\n"]
    pub fn set_sasl(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskCluster_ClientAuthentication_Sasl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sasl = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sasl = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication {
    type O =
        terrars
        ::BlockAssignable<MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication {}

impl BuildMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication {
    pub fn build(self) -> MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication {
        MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication {
            tls: core::default::Default::default(),
            sasl: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthenticationRef {
        MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls", self.base))
    }

    #[doc = "Get a reference to the value of field `sasl` after provisioning.\n"]
    pub fn sasl(&self) -> terrars::ListRef<MskCluster_ClientAuthentication_SaslRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sasl", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivityDynamic {
    client_authentication: std
    ::option
    ::Option<
        terrars::DynamicBlock<MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication>,
    >,
}

#[derive(Serialize)]
pub struct MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_authentication: std
    ::option
    ::Option<std::vec::Vec<MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication>>,
    dynamic: MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivityDynamic,
}

impl MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity {
    #[doc = "Set the field `client_authentication`.\n"]
    pub fn set_client_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity {
    type O = terrars::BlockAssignable<MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity {}

impl BuildMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity {
    pub fn build(self) -> MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity {
        MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity {
            client_authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivityRef {
        MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_authentication` after provisioning.\n"]
    pub fn client_authentication(
        &self,
    ) -> terrars::ListRef<MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskCluster_BrokerNodeGroupInfo_ConnectivityInfoDynamic {
    public_access: std
    ::option
    ::Option<terrars::DynamicBlock<MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccess>>,
    vpc_connectivity: std
    ::option
    ::Option<terrars::DynamicBlock<MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity>>,
}

#[derive(Serialize)]
pub struct MskCluster_BrokerNodeGroupInfo_ConnectivityInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_access: std::option::Option<std::vec::Vec<MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccess>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_connectivity: std::option::Option<std::vec::Vec<MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity>>,
    dynamic: MskCluster_BrokerNodeGroupInfo_ConnectivityInfoDynamic,
}

impl MskCluster_BrokerNodeGroupInfo_ConnectivityInfo {
    #[doc = "Set the field `network_type`.\n"]
    pub fn set_network_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_access`.\n"]
    pub fn set_public_access(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccess>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.public_access = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.public_access = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_connectivity`.\n"]
    pub fn set_vpc_connectivity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_connectivity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_connectivity = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskCluster_BrokerNodeGroupInfo_ConnectivityInfo {
    type O = terrars::BlockAssignable<MskCluster_BrokerNodeGroupInfo_ConnectivityInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_BrokerNodeGroupInfo_ConnectivityInfo {}

impl BuildMskCluster_BrokerNodeGroupInfo_ConnectivityInfo {
    pub fn build(self) -> MskCluster_BrokerNodeGroupInfo_ConnectivityInfo {
        MskCluster_BrokerNodeGroupInfo_ConnectivityInfo {
            network_type: core::default::Default::default(),
            public_access: core::default::Default::default(),
            vpc_connectivity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskCluster_BrokerNodeGroupInfo_ConnectivityInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_BrokerNodeGroupInfo_ConnectivityInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskCluster_BrokerNodeGroupInfo_ConnectivityInfoRef {
        MskCluster_BrokerNodeGroupInfo_ConnectivityInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_BrokerNodeGroupInfo_ConnectivityInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_type` after provisioning.\n"]
    pub fn network_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_type", self.base))
    }

    #[doc = "Get a reference to the value of field `public_access` after provisioning.\n"]
    pub fn public_access(&self) -> terrars::ListRef<MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_access", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_connectivity` after provisioning.\n"]
    pub fn vpc_connectivity(
        &self,
    ) -> terrars::ListRef<MskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_connectivity", self.base))
    }
}

#[derive(Serialize)]
pub struct MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_throughput: std::option::Option<terrars::PrimField<f64>>,
}

impl MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughput {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_throughput`.\n"]
    pub fn set_volume_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_throughput = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughput {
    type O =
        terrars::BlockAssignable<MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughput {}

impl BuildMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughput {
    pub fn build(self) -> MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughput {
        MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughput {
            enabled: core::default::Default::default(),
            volume_throughput: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughputRef {
        MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_throughput` after provisioning.\n"]
    pub fn volume_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_throughput", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfoDynamic {
    provisioned_throughput: std
    ::option
    ::Option<terrars::DynamicBlock<MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughput>>,
}

#[derive(Serialize)]
pub struct MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_throughput: std
    ::option
    ::Option<std::vec::Vec<MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughput>>,
    dynamic: MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfoDynamic,
}

impl MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo {
    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provisioned_throughput`.\n"]
    pub fn set_provisioned_throughput(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughput,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.provisioned_throughput = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.provisioned_throughput = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo {
    type O = terrars::BlockAssignable<MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo {}

impl BuildMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo {
    pub fn build(self) -> MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo {
        MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo {
            volume_size: core::default::Default::default(),
            provisioned_throughput: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfoRef {
        MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioned_throughput` after provisioning.\n"]
    pub fn provisioned_throughput(
        &self,
    ) -> terrars::ListRef<MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provisioned_throughput", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskCluster_BrokerNodeGroupInfo_StorageInfoDynamic {
    ebs_storage_info: std
    ::option
    ::Option<terrars::DynamicBlock<MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo>>,
}

#[derive(Serialize)]
pub struct MskCluster_BrokerNodeGroupInfo_StorageInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_storage_info: std::option::Option<std::vec::Vec<MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo>>,
    dynamic: MskCluster_BrokerNodeGroupInfo_StorageInfoDynamic,
}

impl MskCluster_BrokerNodeGroupInfo_StorageInfo {
    #[doc = "Set the field `ebs_storage_info`.\n"]
    pub fn set_ebs_storage_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ebs_storage_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ebs_storage_info = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskCluster_BrokerNodeGroupInfo_StorageInfo {
    type O = terrars::BlockAssignable<MskCluster_BrokerNodeGroupInfo_StorageInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_BrokerNodeGroupInfo_StorageInfo {}

impl BuildMskCluster_BrokerNodeGroupInfo_StorageInfo {
    pub fn build(self) -> MskCluster_BrokerNodeGroupInfo_StorageInfo {
        MskCluster_BrokerNodeGroupInfo_StorageInfo {
            ebs_storage_info: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskCluster_BrokerNodeGroupInfo_StorageInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_BrokerNodeGroupInfo_StorageInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskCluster_BrokerNodeGroupInfo_StorageInfoRef {
        MskCluster_BrokerNodeGroupInfo_StorageInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_BrokerNodeGroupInfo_StorageInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ebs_storage_info` after provisioning.\n"]
    pub fn ebs_storage_info(&self) -> terrars::ListRef<MskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs_storage_info", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskCluster_BrokerNodeGroupInfoDynamic {
    connectivity_info: std::option::Option<terrars::DynamicBlock<MskCluster_BrokerNodeGroupInfo_ConnectivityInfo>>,
    storage_info: std::option::Option<terrars::DynamicBlock<MskCluster_BrokerNodeGroupInfo_StorageInfo>>,
}

#[derive(Serialize)]
pub struct MskCluster_BrokerNodeGroupInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    az_distribution: std::option::Option<terrars::PrimField<std::string::String>>,
    client_subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
    instance_type: terrars::PrimField<std::string::String>,
    security_groups: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connectivity_info: std::option::Option<std::vec::Vec<MskCluster_BrokerNodeGroupInfo_ConnectivityInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_info: std::option::Option<std::vec::Vec<MskCluster_BrokerNodeGroupInfo_StorageInfo>>,
    dynamic: MskCluster_BrokerNodeGroupInfoDynamic,
}

impl MskCluster_BrokerNodeGroupInfo {
    #[doc = "Set the field `az_distribution`.\n"]
    pub fn set_az_distribution(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.az_distribution = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connectivity_info`.\n"]
    pub fn set_connectivity_info(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskCluster_BrokerNodeGroupInfo_ConnectivityInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.connectivity_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.connectivity_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_info`.\n"]
    pub fn set_storage_info(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskCluster_BrokerNodeGroupInfo_StorageInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_info = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskCluster_BrokerNodeGroupInfo {
    type O = terrars::BlockAssignable<MskCluster_BrokerNodeGroupInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_BrokerNodeGroupInfo {
    #[doc = ""]
    pub client_subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub instance_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub security_groups: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildMskCluster_BrokerNodeGroupInfo {
    pub fn build(self) -> MskCluster_BrokerNodeGroupInfo {
        MskCluster_BrokerNodeGroupInfo {
            az_distribution: core::default::Default::default(),
            client_subnets: self.client_subnets,
            instance_type: self.instance_type,
            security_groups: self.security_groups,
            connectivity_info: core::default::Default::default(),
            storage_info: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskCluster_BrokerNodeGroupInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_BrokerNodeGroupInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskCluster_BrokerNodeGroupInfoRef {
        MskCluster_BrokerNodeGroupInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_BrokerNodeGroupInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `az_distribution` after provisioning.\n"]
    pub fn az_distribution(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.az_distribution", self.base))
    }

    #[doc = "Get a reference to the value of field `client_subnets` after provisioning.\n"]
    pub fn client_subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.client_subnets", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `connectivity_info` after provisioning.\n"]
    pub fn connectivity_info(&self) -> terrars::ListRef<MskCluster_BrokerNodeGroupInfo_ConnectivityInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connectivity_info", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_info` after provisioning.\n"]
    pub fn storage_info(&self) -> terrars::ListRef<MskCluster_BrokerNodeGroupInfo_StorageInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_info", self.base))
    }
}

#[derive(Serialize)]
pub struct MskCluster_ClientAuthentication_Tls {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_authority_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl MskCluster_ClientAuthentication_Tls {
    #[doc = "Set the field `certificate_authority_arns`.\n"]
    pub fn set_certificate_authority_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.certificate_authority_arns = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskCluster_ClientAuthentication_Tls {
    type O = terrars::BlockAssignable<MskCluster_ClientAuthentication_Tls>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_ClientAuthentication_Tls {}

impl BuildMskCluster_ClientAuthentication_Tls {
    pub fn build(self) -> MskCluster_ClientAuthentication_Tls {
        MskCluster_ClientAuthentication_Tls { certificate_authority_arns: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MskCluster_ClientAuthentication_TlsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_ClientAuthentication_TlsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskCluster_ClientAuthentication_TlsRef {
        MskCluster_ClientAuthentication_TlsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_ClientAuthentication_TlsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority_arns` after provisioning.\n"]
    pub fn certificate_authority_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.certificate_authority_arns", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskCluster_ClientAuthenticationDynamic {
    sasl: std::option::Option<terrars::DynamicBlock<MskCluster_ClientAuthentication_Sasl>>,
    tls: std::option::Option<terrars::DynamicBlock<MskCluster_ClientAuthentication_Tls>>,
}

#[derive(Serialize)]
pub struct MskCluster_ClientAuthentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unauthenticated: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sasl: std::option::Option<std::vec::Vec<MskCluster_ClientAuthentication_Sasl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls: std::option::Option<std::vec::Vec<MskCluster_ClientAuthentication_Tls>>,
    dynamic: MskCluster_ClientAuthenticationDynamic,
}

impl MskCluster_ClientAuthentication {
    #[doc = "Set the field `unauthenticated`.\n"]
    pub fn set_unauthenticated(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.unauthenticated = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sasl`.\n"]
    pub fn set_sasl(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskCluster_ClientAuthentication_Sasl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sasl = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sasl = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskCluster_ClientAuthentication_Tls>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tls = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tls = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskCluster_ClientAuthentication {
    type O = terrars::BlockAssignable<MskCluster_ClientAuthentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_ClientAuthentication {}

impl BuildMskCluster_ClientAuthentication {
    pub fn build(self) -> MskCluster_ClientAuthentication {
        MskCluster_ClientAuthentication {
            unauthenticated: core::default::Default::default(),
            sasl: core::default::Default::default(),
            tls: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskCluster_ClientAuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_ClientAuthenticationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskCluster_ClientAuthenticationRef {
        MskCluster_ClientAuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_ClientAuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unauthenticated` after provisioning.\n"]
    pub fn unauthenticated(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unauthenticated", self.base))
    }

    #[doc = "Get a reference to the value of field `sasl` after provisioning.\n"]
    pub fn sasl(&self) -> terrars::ListRef<MskCluster_ClientAuthentication_SaslRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sasl", self.base))
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> terrars::ListRef<MskCluster_ClientAuthentication_TlsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize)]
pub struct MskCluster_ConfigurationInfo {
    arn: terrars::PrimField<std::string::String>,
    revision: terrars::PrimField<f64>,
}

impl MskCluster_ConfigurationInfo { }

impl terrars::ToListMappable for MskCluster_ConfigurationInfo {
    type O = terrars::BlockAssignable<MskCluster_ConfigurationInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_ConfigurationInfo {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub revision: terrars::PrimField<f64>,
}

impl BuildMskCluster_ConfigurationInfo {
    pub fn build(self) -> MskCluster_ConfigurationInfo {
        MskCluster_ConfigurationInfo {
            arn: self.arn,
            revision: self.revision,
        }
    }
}

#[allow(dead_code)]
pub struct MskCluster_ConfigurationInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_ConfigurationInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskCluster_ConfigurationInfoRef {
        MskCluster_ConfigurationInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_ConfigurationInfoRef {
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

#[derive(Serialize)]
pub struct MskCluster_EncryptionInfo_EncryptionInTransit {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_broker: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    in_cluster: std::option::Option<terrars::PrimField<bool>>,
}

impl MskCluster_EncryptionInfo_EncryptionInTransit {
    #[doc = "Set the field `client_broker`.\n"]
    pub fn set_client_broker(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_broker = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `in_cluster`.\n"]
    pub fn set_in_cluster(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.in_cluster = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskCluster_EncryptionInfo_EncryptionInTransit {
    type O = terrars::BlockAssignable<MskCluster_EncryptionInfo_EncryptionInTransit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_EncryptionInfo_EncryptionInTransit {}

impl BuildMskCluster_EncryptionInfo_EncryptionInTransit {
    pub fn build(self) -> MskCluster_EncryptionInfo_EncryptionInTransit {
        MskCluster_EncryptionInfo_EncryptionInTransit {
            client_broker: core::default::Default::default(),
            in_cluster: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskCluster_EncryptionInfo_EncryptionInTransitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_EncryptionInfo_EncryptionInTransitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskCluster_EncryptionInfo_EncryptionInTransitRef {
        MskCluster_EncryptionInfo_EncryptionInTransitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_EncryptionInfo_EncryptionInTransitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_broker` after provisioning.\n"]
    pub fn client_broker(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_broker", self.base))
    }

    #[doc = "Get a reference to the value of field `in_cluster` after provisioning.\n"]
    pub fn in_cluster(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.in_cluster", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskCluster_EncryptionInfoDynamic {
    encryption_in_transit: std::option::Option<terrars::DynamicBlock<MskCluster_EncryptionInfo_EncryptionInTransit>>,
}

#[derive(Serialize)]
pub struct MskCluster_EncryptionInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_at_rest_kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_in_transit: std::option::Option<std::vec::Vec<MskCluster_EncryptionInfo_EncryptionInTransit>>,
    dynamic: MskCluster_EncryptionInfoDynamic,
}

impl MskCluster_EncryptionInfo {
    #[doc = "Set the field `encryption_at_rest_kms_key_arn`.\n"]
    pub fn set_encryption_at_rest_kms_key_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.encryption_at_rest_kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_in_transit`.\n"]
    pub fn set_encryption_in_transit(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskCluster_EncryptionInfo_EncryptionInTransit>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_in_transit = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_in_transit = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskCluster_EncryptionInfo {
    type O = terrars::BlockAssignable<MskCluster_EncryptionInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_EncryptionInfo {}

impl BuildMskCluster_EncryptionInfo {
    pub fn build(self) -> MskCluster_EncryptionInfo {
        MskCluster_EncryptionInfo {
            encryption_at_rest_kms_key_arn: core::default::Default::default(),
            encryption_in_transit: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskCluster_EncryptionInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_EncryptionInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskCluster_EncryptionInfoRef {
        MskCluster_EncryptionInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_EncryptionInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_at_rest_kms_key_arn` after provisioning.\n"]
    pub fn encryption_at_rest_kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_at_rest_kms_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_in_transit` after provisioning.\n"]
    pub fn encryption_in_transit(&self) -> terrars::ListRef<MskCluster_EncryptionInfo_EncryptionInTransitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_in_transit", self.base))
    }
}

#[derive(Serialize)]
pub struct MskCluster_LoggingInfo_BrokerLogs_CloudwatchLogs {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskCluster_LoggingInfo_BrokerLogs_CloudwatchLogs {
    #[doc = "Set the field `log_group`.\n"]
    pub fn set_log_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_group = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskCluster_LoggingInfo_BrokerLogs_CloudwatchLogs {
    type O = terrars::BlockAssignable<MskCluster_LoggingInfo_BrokerLogs_CloudwatchLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_LoggingInfo_BrokerLogs_CloudwatchLogs {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildMskCluster_LoggingInfo_BrokerLogs_CloudwatchLogs {
    pub fn build(self) -> MskCluster_LoggingInfo_BrokerLogs_CloudwatchLogs {
        MskCluster_LoggingInfo_BrokerLogs_CloudwatchLogs {
            enabled: self.enabled,
            log_group: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskCluster_LoggingInfo_BrokerLogs_CloudwatchLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_LoggingInfo_BrokerLogs_CloudwatchLogsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskCluster_LoggingInfo_BrokerLogs_CloudwatchLogsRef {
        MskCluster_LoggingInfo_BrokerLogs_CloudwatchLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_LoggingInfo_BrokerLogs_CloudwatchLogsRef {
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
pub struct MskCluster_LoggingInfo_BrokerLogs_Firehose {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delivery_stream: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
}

impl MskCluster_LoggingInfo_BrokerLogs_Firehose {
    #[doc = "Set the field `delivery_stream`.\n"]
    pub fn set_delivery_stream(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delivery_stream = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskCluster_LoggingInfo_BrokerLogs_Firehose {
    type O = terrars::BlockAssignable<MskCluster_LoggingInfo_BrokerLogs_Firehose>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_LoggingInfo_BrokerLogs_Firehose {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildMskCluster_LoggingInfo_BrokerLogs_Firehose {
    pub fn build(self) -> MskCluster_LoggingInfo_BrokerLogs_Firehose {
        MskCluster_LoggingInfo_BrokerLogs_Firehose {
            delivery_stream: core::default::Default::default(),
            enabled: self.enabled,
        }
    }
}

#[allow(dead_code)]
pub struct MskCluster_LoggingInfo_BrokerLogs_FirehoseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_LoggingInfo_BrokerLogs_FirehoseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskCluster_LoggingInfo_BrokerLogs_FirehoseRef {
        MskCluster_LoggingInfo_BrokerLogs_FirehoseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_LoggingInfo_BrokerLogs_FirehoseRef {
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
pub struct MskCluster_LoggingInfo_BrokerLogs_S3 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskCluster_LoggingInfo_BrokerLogs_S3 {
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

impl terrars::ToListMappable for MskCluster_LoggingInfo_BrokerLogs_S3 {
    type O = terrars::BlockAssignable<MskCluster_LoggingInfo_BrokerLogs_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_LoggingInfo_BrokerLogs_S3 {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildMskCluster_LoggingInfo_BrokerLogs_S3 {
    pub fn build(self) -> MskCluster_LoggingInfo_BrokerLogs_S3 {
        MskCluster_LoggingInfo_BrokerLogs_S3 {
            bucket: core::default::Default::default(),
            enabled: self.enabled,
            prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskCluster_LoggingInfo_BrokerLogs_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_LoggingInfo_BrokerLogs_S3Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskCluster_LoggingInfo_BrokerLogs_S3Ref {
        MskCluster_LoggingInfo_BrokerLogs_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_LoggingInfo_BrokerLogs_S3Ref {
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
struct MskCluster_LoggingInfo_BrokerLogsDynamic {
    cloudwatch_logs: std::option::Option<terrars::DynamicBlock<MskCluster_LoggingInfo_BrokerLogs_CloudwatchLogs>>,
    firehose: std::option::Option<terrars::DynamicBlock<MskCluster_LoggingInfo_BrokerLogs_Firehose>>,
    s3: std::option::Option<terrars::DynamicBlock<MskCluster_LoggingInfo_BrokerLogs_S3>>,
}

#[derive(Serialize)]
pub struct MskCluster_LoggingInfo_BrokerLogs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs: std::option::Option<std::vec::Vec<MskCluster_LoggingInfo_BrokerLogs_CloudwatchLogs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firehose: std::option::Option<std::vec::Vec<MskCluster_LoggingInfo_BrokerLogs_Firehose>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<MskCluster_LoggingInfo_BrokerLogs_S3>>,
    dynamic: MskCluster_LoggingInfo_BrokerLogsDynamic,
}

impl MskCluster_LoggingInfo_BrokerLogs {
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskCluster_LoggingInfo_BrokerLogs_CloudwatchLogs>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MskCluster_LoggingInfo_BrokerLogs_Firehose>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MskCluster_LoggingInfo_BrokerLogs_S3>>,
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

impl terrars::ToListMappable for MskCluster_LoggingInfo_BrokerLogs {
    type O = terrars::BlockAssignable<MskCluster_LoggingInfo_BrokerLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_LoggingInfo_BrokerLogs {}

impl BuildMskCluster_LoggingInfo_BrokerLogs {
    pub fn build(self) -> MskCluster_LoggingInfo_BrokerLogs {
        MskCluster_LoggingInfo_BrokerLogs {
            cloudwatch_logs: core::default::Default::default(),
            firehose: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskCluster_LoggingInfo_BrokerLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_LoggingInfo_BrokerLogsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskCluster_LoggingInfo_BrokerLogsRef {
        MskCluster_LoggingInfo_BrokerLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_LoggingInfo_BrokerLogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(&self) -> terrars::ListRef<MskCluster_LoggingInfo_BrokerLogs_CloudwatchLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `firehose` after provisioning.\n"]
    pub fn firehose(&self) -> terrars::ListRef<MskCluster_LoggingInfo_BrokerLogs_FirehoseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firehose", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<MskCluster_LoggingInfo_BrokerLogs_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskCluster_LoggingInfoDynamic {
    broker_logs: std::option::Option<terrars::DynamicBlock<MskCluster_LoggingInfo_BrokerLogs>>,
}

#[derive(Serialize)]
pub struct MskCluster_LoggingInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    broker_logs: std::option::Option<std::vec::Vec<MskCluster_LoggingInfo_BrokerLogs>>,
    dynamic: MskCluster_LoggingInfoDynamic,
}

impl MskCluster_LoggingInfo {
    #[doc = "Set the field `broker_logs`.\n"]
    pub fn set_broker_logs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskCluster_LoggingInfo_BrokerLogs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.broker_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.broker_logs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskCluster_LoggingInfo {
    type O = terrars::BlockAssignable<MskCluster_LoggingInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_LoggingInfo {}

impl BuildMskCluster_LoggingInfo {
    pub fn build(self) -> MskCluster_LoggingInfo {
        MskCluster_LoggingInfo {
            broker_logs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskCluster_LoggingInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_LoggingInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskCluster_LoggingInfoRef {
        MskCluster_LoggingInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_LoggingInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `broker_logs` after provisioning.\n"]
    pub fn broker_logs(&self) -> terrars::ListRef<MskCluster_LoggingInfo_BrokerLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.broker_logs", self.base))
    }
}

#[derive(Serialize)]
pub struct MskCluster_OpenMonitoring_Prometheus_JmxExporter {
    enabled_in_broker: terrars::PrimField<bool>,
}

impl MskCluster_OpenMonitoring_Prometheus_JmxExporter { }

impl terrars::ToListMappable for MskCluster_OpenMonitoring_Prometheus_JmxExporter {
    type O = terrars::BlockAssignable<MskCluster_OpenMonitoring_Prometheus_JmxExporter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_OpenMonitoring_Prometheus_JmxExporter {
    #[doc = ""]
    pub enabled_in_broker: terrars::PrimField<bool>,
}

impl BuildMskCluster_OpenMonitoring_Prometheus_JmxExporter {
    pub fn build(self) -> MskCluster_OpenMonitoring_Prometheus_JmxExporter {
        MskCluster_OpenMonitoring_Prometheus_JmxExporter { enabled_in_broker: self.enabled_in_broker }
    }
}

#[allow(dead_code)]
pub struct MskCluster_OpenMonitoring_Prometheus_JmxExporterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_OpenMonitoring_Prometheus_JmxExporterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskCluster_OpenMonitoring_Prometheus_JmxExporterRef {
        MskCluster_OpenMonitoring_Prometheus_JmxExporterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_OpenMonitoring_Prometheus_JmxExporterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled_in_broker` after provisioning.\n"]
    pub fn enabled_in_broker(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled_in_broker", self.base))
    }
}

#[derive(Serialize)]
pub struct MskCluster_OpenMonitoring_Prometheus_NodeExporter {
    enabled_in_broker: terrars::PrimField<bool>,
}

impl MskCluster_OpenMonitoring_Prometheus_NodeExporter { }

impl terrars::ToListMappable for MskCluster_OpenMonitoring_Prometheus_NodeExporter {
    type O = terrars::BlockAssignable<MskCluster_OpenMonitoring_Prometheus_NodeExporter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_OpenMonitoring_Prometheus_NodeExporter {
    #[doc = ""]
    pub enabled_in_broker: terrars::PrimField<bool>,
}

impl BuildMskCluster_OpenMonitoring_Prometheus_NodeExporter {
    pub fn build(self) -> MskCluster_OpenMonitoring_Prometheus_NodeExporter {
        MskCluster_OpenMonitoring_Prometheus_NodeExporter { enabled_in_broker: self.enabled_in_broker }
    }
}

#[allow(dead_code)]
pub struct MskCluster_OpenMonitoring_Prometheus_NodeExporterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_OpenMonitoring_Prometheus_NodeExporterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskCluster_OpenMonitoring_Prometheus_NodeExporterRef {
        MskCluster_OpenMonitoring_Prometheus_NodeExporterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_OpenMonitoring_Prometheus_NodeExporterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled_in_broker` after provisioning.\n"]
    pub fn enabled_in_broker(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled_in_broker", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskCluster_OpenMonitoring_PrometheusDynamic {
    jmx_exporter: std::option::Option<terrars::DynamicBlock<MskCluster_OpenMonitoring_Prometheus_JmxExporter>>,
    node_exporter: std::option::Option<terrars::DynamicBlock<MskCluster_OpenMonitoring_Prometheus_NodeExporter>>,
}

#[derive(Serialize)]
pub struct MskCluster_OpenMonitoring_Prometheus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jmx_exporter: std::option::Option<std::vec::Vec<MskCluster_OpenMonitoring_Prometheus_JmxExporter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_exporter: std::option::Option<std::vec::Vec<MskCluster_OpenMonitoring_Prometheus_NodeExporter>>,
    dynamic: MskCluster_OpenMonitoring_PrometheusDynamic,
}

impl MskCluster_OpenMonitoring_Prometheus {
    #[doc = "Set the field `jmx_exporter`.\n"]
    pub fn set_jmx_exporter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskCluster_OpenMonitoring_Prometheus_JmxExporter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.jmx_exporter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.jmx_exporter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_exporter`.\n"]
    pub fn set_node_exporter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskCluster_OpenMonitoring_Prometheus_NodeExporter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_exporter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_exporter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskCluster_OpenMonitoring_Prometheus {
    type O = terrars::BlockAssignable<MskCluster_OpenMonitoring_Prometheus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_OpenMonitoring_Prometheus {}

impl BuildMskCluster_OpenMonitoring_Prometheus {
    pub fn build(self) -> MskCluster_OpenMonitoring_Prometheus {
        MskCluster_OpenMonitoring_Prometheus {
            jmx_exporter: core::default::Default::default(),
            node_exporter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskCluster_OpenMonitoring_PrometheusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_OpenMonitoring_PrometheusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskCluster_OpenMonitoring_PrometheusRef {
        MskCluster_OpenMonitoring_PrometheusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_OpenMonitoring_PrometheusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `jmx_exporter` after provisioning.\n"]
    pub fn jmx_exporter(&self) -> terrars::ListRef<MskCluster_OpenMonitoring_Prometheus_JmxExporterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jmx_exporter", self.base))
    }

    #[doc = "Get a reference to the value of field `node_exporter` after provisioning.\n"]
    pub fn node_exporter(&self) -> terrars::ListRef<MskCluster_OpenMonitoring_Prometheus_NodeExporterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_exporter", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskCluster_OpenMonitoringDynamic {
    prometheus: std::option::Option<terrars::DynamicBlock<MskCluster_OpenMonitoring_Prometheus>>,
}

#[derive(Serialize)]
pub struct MskCluster_OpenMonitoring {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prometheus: std::option::Option<std::vec::Vec<MskCluster_OpenMonitoring_Prometheus>>,
    dynamic: MskCluster_OpenMonitoringDynamic,
}

impl MskCluster_OpenMonitoring {
    #[doc = "Set the field `prometheus`.\n"]
    pub fn set_prometheus(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskCluster_OpenMonitoring_Prometheus>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.prometheus = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.prometheus = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskCluster_OpenMonitoring {
    type O = terrars::BlockAssignable<MskCluster_OpenMonitoring>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_OpenMonitoring {}

impl BuildMskCluster_OpenMonitoring {
    pub fn build(self) -> MskCluster_OpenMonitoring {
        MskCluster_OpenMonitoring {
            prometheus: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskCluster_OpenMonitoringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_OpenMonitoringRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskCluster_OpenMonitoringRef {
        MskCluster_OpenMonitoringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_OpenMonitoringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prometheus` after provisioning.\n"]
    pub fn prometheus(&self) -> terrars::ListRef<MskCluster_OpenMonitoring_PrometheusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prometheus", self.base))
    }
}

#[derive(Serialize)]
pub struct MskCluster_Rebalancing {
    status: terrars::PrimField<std::string::String>,
}

impl MskCluster_Rebalancing { }

impl terrars::ToListMappable for MskCluster_Rebalancing {
    type O = terrars::BlockAssignable<MskCluster_Rebalancing>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_Rebalancing {
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildMskCluster_Rebalancing {
    pub fn build(self) -> MskCluster_Rebalancing {
        MskCluster_Rebalancing { status: self.status }
    }
}

#[allow(dead_code)]
pub struct MskCluster_RebalancingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_RebalancingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskCluster_RebalancingRef {
        MskCluster_RebalancingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_RebalancingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct MskCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskCluster_Timeouts {
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

impl terrars::ToListMappable for MskCluster_Timeouts {
    type O = terrars::BlockAssignable<MskCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskCluster_Timeouts {}

impl BuildMskCluster_Timeouts {
    pub fn build(self) -> MskCluster_Timeouts {
        MskCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskCluster_TimeoutsRef {
        MskCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskCluster_TimeoutsRef {
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
