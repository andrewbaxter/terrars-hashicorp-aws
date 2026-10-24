use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MskClusterData {
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
    storage_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    broker_node_group_info: std::option::Option<std::vec::Vec<MskClusterBrokerNodeGroupInfoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_authentication: std::option::Option<std::vec::Vec<MskClusterClientAuthenticationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration_info: std::option::Option<std::vec::Vec<MskClusterConfigurationInfoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_info: std::option::Option<std::vec::Vec<MskClusterEncryptionInfoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_info: std::option::Option<std::vec::Vec<MskClusterLoggingInfoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_monitoring: std::option::Option<std::vec::Vec<MskClusterOpenMonitoringEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MskClusterTimeoutsEl>,
    dynamic: MskClusterDynamic,
}

struct MskCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MskClusterData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MskClusterBrokerNodeGroupInfoEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MskClusterClientAuthenticationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MskClusterConfigurationInfoEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MskClusterEncryptionInfoEl>>,
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
    pub fn set_logging_info(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskClusterLoggingInfoEl>>,
    ) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<MskClusterOpenMonitoringEl>>,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<MskClusterTimeoutsEl>) -> Self {
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

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_scram(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_sasl_scram", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `current_version` after provisioning.\n"]
    pub fn current_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_version", self.extract_ref()))
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
    pub fn broker_node_group_info(&self) -> terrars::ListRef<MskClusterBrokerNodeGroupInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.broker_node_group_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_authentication` after provisioning.\n"]
    pub fn client_authentication(&self) -> terrars::ListRef<MskClusterClientAuthenticationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_info` after provisioning.\n"]
    pub fn configuration_info(&self) -> terrars::ListRef<MskClusterConfigurationInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_info` after provisioning.\n"]
    pub fn encryption_info(&self) -> terrars::ListRef<MskClusterEncryptionInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_info` after provisioning.\n"]
    pub fn logging_info(&self) -> terrars::ListRef<MskClusterLoggingInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `open_monitoring` after provisioning.\n"]
    pub fn open_monitoring(&self) -> terrars::ListRef<MskClusterOpenMonitoringElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskClusterTimeoutsElRef {
        MskClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(MskClusterData {
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
                storage_mode: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                broker_node_group_info: core::default::Default::default(),
                client_authentication: core::default::Default::default(),
                configuration_info: core::default::Default::default(),
                encryption_info: core::default::Default::default(),
                logging_info: core::default::Default::default(),
                open_monitoring: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_scram(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_sasl_scram", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `current_version` after provisioning.\n"]
    pub fn current_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_version", self.extract_ref()))
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
    pub fn broker_node_group_info(&self) -> terrars::ListRef<MskClusterBrokerNodeGroupInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.broker_node_group_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_authentication` after provisioning.\n"]
    pub fn client_authentication(&self) -> terrars::ListRef<MskClusterClientAuthenticationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_info` after provisioning.\n"]
    pub fn configuration_info(&self) -> terrars::ListRef<MskClusterConfigurationInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_info` after provisioning.\n"]
    pub fn encryption_info(&self) -> terrars::ListRef<MskClusterEncryptionInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_info` after provisioning.\n"]
    pub fn logging_info(&self) -> terrars::ListRef<MskClusterLoggingInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `open_monitoring` after provisioning.\n"]
    pub fn open_monitoring(&self) -> terrars::ListRef<MskClusterOpenMonitoringElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskClusterTimeoutsElRef {
        MskClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl {
    type O = terrars::BlockAssignable<MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl {}

impl BuildMskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl {
    pub fn build(self) -> MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl {
        MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl { type_: core::default::Default::default() }
    }
}

pub struct MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessElRef {
        MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskClusterBrokerNodeGroupInfoElConnectivityInfoElDynamic {
    public_access: std
    ::option
    ::Option<terrars::DynamicBlock<MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl>>,
}

#[derive(Serialize)]
pub struct MskClusterBrokerNodeGroupInfoElConnectivityInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_access: std::option::Option<std::vec::Vec<MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl>>,
    dynamic: MskClusterBrokerNodeGroupInfoElConnectivityInfoElDynamic,
}

impl MskClusterBrokerNodeGroupInfoElConnectivityInfoEl {
    #[doc = "Set the field `public_access`.\n"]
    pub fn set_public_access(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessEl>>,
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
}

impl terrars::ToListMappable for MskClusterBrokerNodeGroupInfoElConnectivityInfoEl {
    type O = terrars::BlockAssignable<MskClusterBrokerNodeGroupInfoElConnectivityInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterBrokerNodeGroupInfoElConnectivityInfoEl {}

impl BuildMskClusterBrokerNodeGroupInfoElConnectivityInfoEl {
    pub fn build(self) -> MskClusterBrokerNodeGroupInfoElConnectivityInfoEl {
        MskClusterBrokerNodeGroupInfoElConnectivityInfoEl {
            public_access: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MskClusterBrokerNodeGroupInfoElConnectivityInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterBrokerNodeGroupInfoElConnectivityInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskClusterBrokerNodeGroupInfoElConnectivityInfoElRef {
        MskClusterBrokerNodeGroupInfoElConnectivityInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterBrokerNodeGroupInfoElConnectivityInfoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `public_access` after provisioning.\n"]
    pub fn public_access(&self) -> terrars::ListRef<MskClusterBrokerNodeGroupInfoElConnectivityInfoElPublicAccessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_access", self.base))
    }
}

#[derive(Serialize)]
pub struct MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_throughput: std::option::Option<terrars::PrimField<f64>>,
}

impl MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl {
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

impl terrars::ToListMappable for MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl {
    type O =
        terrars
        ::BlockAssignable<MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl {}

impl BuildMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl {
    pub fn build(self) -> MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl {
        MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl {
            enabled: core::default::Default::default(),
            volume_throughput: core::default::Default::default(),
        }
    }
}

pub struct MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputElRef {
        MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputElRef {
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
struct MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElDynamic {
    provisioned_throughput: std
    ::option
    ::Option<
        terrars::DynamicBlock<MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl>,
    >,
}

#[derive(Serialize)]
pub struct MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_throughput: std
    ::option
    ::Option<std::vec::Vec<MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl>>,
    dynamic: MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElDynamic,
}

impl MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl {
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
                            MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputEl,
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

impl terrars::ToListMappable for MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl {
    type O = terrars::BlockAssignable<MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl {}

impl BuildMskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl {
    pub fn build(self) -> MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl {
        MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl {
            volume_size: core::default::Default::default(),
            provisioned_throughput: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElRef {
        MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElRef {
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
    ) -> terrars::ListRef<MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElProvisionedThroughputElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provisioned_throughput", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskClusterBrokerNodeGroupInfoElStorageInfoElDynamic {
    ebs_storage_info: std
    ::option
    ::Option<terrars::DynamicBlock<MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl>>,
}

#[derive(Serialize)]
pub struct MskClusterBrokerNodeGroupInfoElStorageInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_storage_info: std::option::Option<std::vec::Vec<MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl>>,
    dynamic: MskClusterBrokerNodeGroupInfoElStorageInfoElDynamic,
}

impl MskClusterBrokerNodeGroupInfoElStorageInfoEl {
    #[doc = "Set the field `ebs_storage_info`.\n"]
    pub fn set_ebs_storage_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoEl>>,
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

impl terrars::ToListMappable for MskClusterBrokerNodeGroupInfoElStorageInfoEl {
    type O = terrars::BlockAssignable<MskClusterBrokerNodeGroupInfoElStorageInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterBrokerNodeGroupInfoElStorageInfoEl {}

impl BuildMskClusterBrokerNodeGroupInfoElStorageInfoEl {
    pub fn build(self) -> MskClusterBrokerNodeGroupInfoElStorageInfoEl {
        MskClusterBrokerNodeGroupInfoElStorageInfoEl {
            ebs_storage_info: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MskClusterBrokerNodeGroupInfoElStorageInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterBrokerNodeGroupInfoElStorageInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskClusterBrokerNodeGroupInfoElStorageInfoElRef {
        MskClusterBrokerNodeGroupInfoElStorageInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterBrokerNodeGroupInfoElStorageInfoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ebs_storage_info` after provisioning.\n"]
    pub fn ebs_storage_info(&self) -> terrars::ListRef<MskClusterBrokerNodeGroupInfoElStorageInfoElEbsStorageInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs_storage_info", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskClusterBrokerNodeGroupInfoElDynamic {
    connectivity_info: std::option::Option<terrars::DynamicBlock<MskClusterBrokerNodeGroupInfoElConnectivityInfoEl>>,
    storage_info: std::option::Option<terrars::DynamicBlock<MskClusterBrokerNodeGroupInfoElStorageInfoEl>>,
}

#[derive(Serialize)]
pub struct MskClusterBrokerNodeGroupInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    az_distribution: std::option::Option<terrars::PrimField<std::string::String>>,
    client_subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_volume_size: std::option::Option<terrars::PrimField<f64>>,
    instance_type: terrars::PrimField<std::string::String>,
    security_groups: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connectivity_info: std::option::Option<std::vec::Vec<MskClusterBrokerNodeGroupInfoElConnectivityInfoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_info: std::option::Option<std::vec::Vec<MskClusterBrokerNodeGroupInfoElStorageInfoEl>>,
    dynamic: MskClusterBrokerNodeGroupInfoElDynamic,
}

impl MskClusterBrokerNodeGroupInfoEl {
    #[doc = "Set the field `az_distribution`.\n"]
    pub fn set_az_distribution(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.az_distribution = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_volume_size`.\n"]
    pub fn set_ebs_volume_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ebs_volume_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connectivity_info`.\n"]
    pub fn set_connectivity_info(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskClusterBrokerNodeGroupInfoElConnectivityInfoEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MskClusterBrokerNodeGroupInfoElStorageInfoEl>>,
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

impl terrars::ToListMappable for MskClusterBrokerNodeGroupInfoEl {
    type O = terrars::BlockAssignable<MskClusterBrokerNodeGroupInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterBrokerNodeGroupInfoEl {
    #[doc = ""]
    pub client_subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub instance_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub security_groups: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildMskClusterBrokerNodeGroupInfoEl {
    pub fn build(self) -> MskClusterBrokerNodeGroupInfoEl {
        MskClusterBrokerNodeGroupInfoEl {
            az_distribution: core::default::Default::default(),
            client_subnets: self.client_subnets,
            ebs_volume_size: core::default::Default::default(),
            instance_type: self.instance_type,
            security_groups: self.security_groups,
            connectivity_info: core::default::Default::default(),
            storage_info: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MskClusterBrokerNodeGroupInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterBrokerNodeGroupInfoElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskClusterBrokerNodeGroupInfoElRef {
        MskClusterBrokerNodeGroupInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterBrokerNodeGroupInfoElRef {
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

    #[doc = "Get a reference to the value of field `ebs_volume_size` after provisioning.\n"]
    pub fn ebs_volume_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_volume_size", self.base))
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
    pub fn connectivity_info(&self) -> terrars::ListRef<MskClusterBrokerNodeGroupInfoElConnectivityInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connectivity_info", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_info` after provisioning.\n"]
    pub fn storage_info(&self) -> terrars::ListRef<MskClusterBrokerNodeGroupInfoElStorageInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_info", self.base))
    }
}

#[derive(Serialize)]
pub struct MskClusterClientAuthenticationElSaslEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scram: std::option::Option<terrars::PrimField<bool>>,
}

impl MskClusterClientAuthenticationElSaslEl {
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

impl terrars::ToListMappable for MskClusterClientAuthenticationElSaslEl {
    type O = terrars::BlockAssignable<MskClusterClientAuthenticationElSaslEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterClientAuthenticationElSaslEl {}

impl BuildMskClusterClientAuthenticationElSaslEl {
    pub fn build(self) -> MskClusterClientAuthenticationElSaslEl {
        MskClusterClientAuthenticationElSaslEl {
            iam: core::default::Default::default(),
            scram: core::default::Default::default(),
        }
    }
}

pub struct MskClusterClientAuthenticationElSaslElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterClientAuthenticationElSaslElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskClusterClientAuthenticationElSaslElRef {
        MskClusterClientAuthenticationElSaslElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterClientAuthenticationElSaslElRef {
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

#[derive(Serialize)]
pub struct MskClusterClientAuthenticationElTlsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_authority_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl MskClusterClientAuthenticationElTlsEl {
    #[doc = "Set the field `certificate_authority_arns`.\n"]
    pub fn set_certificate_authority_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.certificate_authority_arns = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskClusterClientAuthenticationElTlsEl {
    type O = terrars::BlockAssignable<MskClusterClientAuthenticationElTlsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterClientAuthenticationElTlsEl {}

impl BuildMskClusterClientAuthenticationElTlsEl {
    pub fn build(self) -> MskClusterClientAuthenticationElTlsEl {
        MskClusterClientAuthenticationElTlsEl { certificate_authority_arns: core::default::Default::default() }
    }
}

pub struct MskClusterClientAuthenticationElTlsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterClientAuthenticationElTlsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskClusterClientAuthenticationElTlsElRef {
        MskClusterClientAuthenticationElTlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterClientAuthenticationElTlsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority_arns` after provisioning.\n"]
    pub fn certificate_authority_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.certificate_authority_arns", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskClusterClientAuthenticationElDynamic {
    sasl: std::option::Option<terrars::DynamicBlock<MskClusterClientAuthenticationElSaslEl>>,
    tls: std::option::Option<terrars::DynamicBlock<MskClusterClientAuthenticationElTlsEl>>,
}

#[derive(Serialize)]
pub struct MskClusterClientAuthenticationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unauthenticated: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sasl: std::option::Option<std::vec::Vec<MskClusterClientAuthenticationElSaslEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls: std::option::Option<std::vec::Vec<MskClusterClientAuthenticationElTlsEl>>,
    dynamic: MskClusterClientAuthenticationElDynamic,
}

impl MskClusterClientAuthenticationEl {
    #[doc = "Set the field `unauthenticated`.\n"]
    pub fn set_unauthenticated(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.unauthenticated = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sasl`.\n"]
    pub fn set_sasl(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskClusterClientAuthenticationElSaslEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MskClusterClientAuthenticationElTlsEl>>,
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

impl terrars::ToListMappable for MskClusterClientAuthenticationEl {
    type O = terrars::BlockAssignable<MskClusterClientAuthenticationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterClientAuthenticationEl {}

impl BuildMskClusterClientAuthenticationEl {
    pub fn build(self) -> MskClusterClientAuthenticationEl {
        MskClusterClientAuthenticationEl {
            unauthenticated: core::default::Default::default(),
            sasl: core::default::Default::default(),
            tls: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MskClusterClientAuthenticationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterClientAuthenticationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskClusterClientAuthenticationElRef {
        MskClusterClientAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterClientAuthenticationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unauthenticated` after provisioning.\n"]
    pub fn unauthenticated(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unauthenticated", self.base))
    }

    #[doc = "Get a reference to the value of field `sasl` after provisioning.\n"]
    pub fn sasl(&self) -> terrars::ListRef<MskClusterClientAuthenticationElSaslElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sasl", self.base))
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> terrars::ListRef<MskClusterClientAuthenticationElTlsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize)]
pub struct MskClusterConfigurationInfoEl {
    arn: terrars::PrimField<std::string::String>,
    revision: terrars::PrimField<f64>,
}

impl MskClusterConfigurationInfoEl { }

impl terrars::ToListMappable for MskClusterConfigurationInfoEl {
    type O = terrars::BlockAssignable<MskClusterConfigurationInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterConfigurationInfoEl {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub revision: terrars::PrimField<f64>,
}

impl BuildMskClusterConfigurationInfoEl {
    pub fn build(self) -> MskClusterConfigurationInfoEl {
        MskClusterConfigurationInfoEl {
            arn: self.arn,
            revision: self.revision,
        }
    }
}

pub struct MskClusterConfigurationInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterConfigurationInfoElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskClusterConfigurationInfoElRef {
        MskClusterConfigurationInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterConfigurationInfoElRef {
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
pub struct MskClusterEncryptionInfoElEncryptionInTransitEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_broker: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    in_cluster: std::option::Option<terrars::PrimField<bool>>,
}

impl MskClusterEncryptionInfoElEncryptionInTransitEl {
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

impl terrars::ToListMappable for MskClusterEncryptionInfoElEncryptionInTransitEl {
    type O = terrars::BlockAssignable<MskClusterEncryptionInfoElEncryptionInTransitEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterEncryptionInfoElEncryptionInTransitEl {}

impl BuildMskClusterEncryptionInfoElEncryptionInTransitEl {
    pub fn build(self) -> MskClusterEncryptionInfoElEncryptionInTransitEl {
        MskClusterEncryptionInfoElEncryptionInTransitEl {
            client_broker: core::default::Default::default(),
            in_cluster: core::default::Default::default(),
        }
    }
}

pub struct MskClusterEncryptionInfoElEncryptionInTransitElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterEncryptionInfoElEncryptionInTransitElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskClusterEncryptionInfoElEncryptionInTransitElRef {
        MskClusterEncryptionInfoElEncryptionInTransitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterEncryptionInfoElEncryptionInTransitElRef {
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
struct MskClusterEncryptionInfoElDynamic {
    encryption_in_transit: std
    ::option
    ::Option<terrars::DynamicBlock<MskClusterEncryptionInfoElEncryptionInTransitEl>>,
}

#[derive(Serialize)]
pub struct MskClusterEncryptionInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_at_rest_kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_in_transit: std::option::Option<std::vec::Vec<MskClusterEncryptionInfoElEncryptionInTransitEl>>,
    dynamic: MskClusterEncryptionInfoElDynamic,
}

impl MskClusterEncryptionInfoEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<MskClusterEncryptionInfoElEncryptionInTransitEl>>,
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

impl terrars::ToListMappable for MskClusterEncryptionInfoEl {
    type O = terrars::BlockAssignable<MskClusterEncryptionInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterEncryptionInfoEl {}

impl BuildMskClusterEncryptionInfoEl {
    pub fn build(self) -> MskClusterEncryptionInfoEl {
        MskClusterEncryptionInfoEl {
            encryption_at_rest_kms_key_arn: core::default::Default::default(),
            encryption_in_transit: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MskClusterEncryptionInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterEncryptionInfoElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskClusterEncryptionInfoElRef {
        MskClusterEncryptionInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterEncryptionInfoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_at_rest_kms_key_arn` after provisioning.\n"]
    pub fn encryption_at_rest_kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_at_rest_kms_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_in_transit` after provisioning.\n"]
    pub fn encryption_in_transit(&self) -> terrars::ListRef<MskClusterEncryptionInfoElEncryptionInTransitElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_in_transit", self.base))
    }
}

#[derive(Serialize)]
pub struct MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl {
    #[doc = "Set the field `log_group`.\n"]
    pub fn set_log_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_group = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl {
    type O = terrars::BlockAssignable<MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildMskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl {
    pub fn build(self) -> MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl {
        MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl {
            enabled: self.enabled,
            log_group: core::default::Default::default(),
        }
    }
}

pub struct MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsElRef {
        MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsElRef {
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
pub struct MskClusterLoggingInfoElBrokerLogsElFirehoseEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delivery_stream: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
}

impl MskClusterLoggingInfoElBrokerLogsElFirehoseEl {
    #[doc = "Set the field `delivery_stream`.\n"]
    pub fn set_delivery_stream(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delivery_stream = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskClusterLoggingInfoElBrokerLogsElFirehoseEl {
    type O = terrars::BlockAssignable<MskClusterLoggingInfoElBrokerLogsElFirehoseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterLoggingInfoElBrokerLogsElFirehoseEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildMskClusterLoggingInfoElBrokerLogsElFirehoseEl {
    pub fn build(self) -> MskClusterLoggingInfoElBrokerLogsElFirehoseEl {
        MskClusterLoggingInfoElBrokerLogsElFirehoseEl {
            delivery_stream: core::default::Default::default(),
            enabled: self.enabled,
        }
    }
}

pub struct MskClusterLoggingInfoElBrokerLogsElFirehoseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterLoggingInfoElBrokerLogsElFirehoseElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskClusterLoggingInfoElBrokerLogsElFirehoseElRef {
        MskClusterLoggingInfoElBrokerLogsElFirehoseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterLoggingInfoElBrokerLogsElFirehoseElRef {
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
pub struct MskClusterLoggingInfoElBrokerLogsElS3El {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskClusterLoggingInfoElBrokerLogsElS3El {
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

impl terrars::ToListMappable for MskClusterLoggingInfoElBrokerLogsElS3El {
    type O = terrars::BlockAssignable<MskClusterLoggingInfoElBrokerLogsElS3El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterLoggingInfoElBrokerLogsElS3El {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildMskClusterLoggingInfoElBrokerLogsElS3El {
    pub fn build(self) -> MskClusterLoggingInfoElBrokerLogsElS3El {
        MskClusterLoggingInfoElBrokerLogsElS3El {
            bucket: core::default::Default::default(),
            enabled: self.enabled,
            prefix: core::default::Default::default(),
        }
    }
}

pub struct MskClusterLoggingInfoElBrokerLogsElS3ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterLoggingInfoElBrokerLogsElS3ElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskClusterLoggingInfoElBrokerLogsElS3ElRef {
        MskClusterLoggingInfoElBrokerLogsElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterLoggingInfoElBrokerLogsElS3ElRef {
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
struct MskClusterLoggingInfoElBrokerLogsElDynamic {
    cloudwatch_logs: std::option::Option<terrars::DynamicBlock<MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl>>,
    firehose: std::option::Option<terrars::DynamicBlock<MskClusterLoggingInfoElBrokerLogsElFirehoseEl>>,
    s3: std::option::Option<terrars::DynamicBlock<MskClusterLoggingInfoElBrokerLogsElS3El>>,
}

#[derive(Serialize)]
pub struct MskClusterLoggingInfoElBrokerLogsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs: std::option::Option<std::vec::Vec<MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firehose: std::option::Option<std::vec::Vec<MskClusterLoggingInfoElBrokerLogsElFirehoseEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<MskClusterLoggingInfoElBrokerLogsElS3El>>,
    dynamic: MskClusterLoggingInfoElBrokerLogsElDynamic,
}

impl MskClusterLoggingInfoElBrokerLogsEl {
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MskClusterLoggingInfoElBrokerLogsElFirehoseEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MskClusterLoggingInfoElBrokerLogsElS3El>>,
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

impl terrars::ToListMappable for MskClusterLoggingInfoElBrokerLogsEl {
    type O = terrars::BlockAssignable<MskClusterLoggingInfoElBrokerLogsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterLoggingInfoElBrokerLogsEl {}

impl BuildMskClusterLoggingInfoElBrokerLogsEl {
    pub fn build(self) -> MskClusterLoggingInfoElBrokerLogsEl {
        MskClusterLoggingInfoElBrokerLogsEl {
            cloudwatch_logs: core::default::Default::default(),
            firehose: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MskClusterLoggingInfoElBrokerLogsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterLoggingInfoElBrokerLogsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskClusterLoggingInfoElBrokerLogsElRef {
        MskClusterLoggingInfoElBrokerLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterLoggingInfoElBrokerLogsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(&self) -> terrars::ListRef<MskClusterLoggingInfoElBrokerLogsElCloudwatchLogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `firehose` after provisioning.\n"]
    pub fn firehose(&self) -> terrars::ListRef<MskClusterLoggingInfoElBrokerLogsElFirehoseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firehose", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<MskClusterLoggingInfoElBrokerLogsElS3ElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskClusterLoggingInfoElDynamic {
    broker_logs: std::option::Option<terrars::DynamicBlock<MskClusterLoggingInfoElBrokerLogsEl>>,
}

#[derive(Serialize)]
pub struct MskClusterLoggingInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    broker_logs: std::option::Option<std::vec::Vec<MskClusterLoggingInfoElBrokerLogsEl>>,
    dynamic: MskClusterLoggingInfoElDynamic,
}

impl MskClusterLoggingInfoEl {
    #[doc = "Set the field `broker_logs`.\n"]
    pub fn set_broker_logs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskClusterLoggingInfoElBrokerLogsEl>>,
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

impl terrars::ToListMappable for MskClusterLoggingInfoEl {
    type O = terrars::BlockAssignable<MskClusterLoggingInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterLoggingInfoEl {}

impl BuildMskClusterLoggingInfoEl {
    pub fn build(self) -> MskClusterLoggingInfoEl {
        MskClusterLoggingInfoEl {
            broker_logs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MskClusterLoggingInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterLoggingInfoElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskClusterLoggingInfoElRef {
        MskClusterLoggingInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterLoggingInfoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `broker_logs` after provisioning.\n"]
    pub fn broker_logs(&self) -> terrars::ListRef<MskClusterLoggingInfoElBrokerLogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.broker_logs", self.base))
    }
}

#[derive(Serialize)]
pub struct MskClusterOpenMonitoringElPrometheusElJmxExporterEl {
    enabled_in_broker: terrars::PrimField<bool>,
}

impl MskClusterOpenMonitoringElPrometheusElJmxExporterEl { }

impl terrars::ToListMappable for MskClusterOpenMonitoringElPrometheusElJmxExporterEl {
    type O = terrars::BlockAssignable<MskClusterOpenMonitoringElPrometheusElJmxExporterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterOpenMonitoringElPrometheusElJmxExporterEl {
    #[doc = ""]
    pub enabled_in_broker: terrars::PrimField<bool>,
}

impl BuildMskClusterOpenMonitoringElPrometheusElJmxExporterEl {
    pub fn build(self) -> MskClusterOpenMonitoringElPrometheusElJmxExporterEl {
        MskClusterOpenMonitoringElPrometheusElJmxExporterEl { enabled_in_broker: self.enabled_in_broker }
    }
}

pub struct MskClusterOpenMonitoringElPrometheusElJmxExporterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterOpenMonitoringElPrometheusElJmxExporterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskClusterOpenMonitoringElPrometheusElJmxExporterElRef {
        MskClusterOpenMonitoringElPrometheusElJmxExporterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterOpenMonitoringElPrometheusElJmxExporterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled_in_broker` after provisioning.\n"]
    pub fn enabled_in_broker(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled_in_broker", self.base))
    }
}

#[derive(Serialize)]
pub struct MskClusterOpenMonitoringElPrometheusElNodeExporterEl {
    enabled_in_broker: terrars::PrimField<bool>,
}

impl MskClusterOpenMonitoringElPrometheusElNodeExporterEl { }

impl terrars::ToListMappable for MskClusterOpenMonitoringElPrometheusElNodeExporterEl {
    type O = terrars::BlockAssignable<MskClusterOpenMonitoringElPrometheusElNodeExporterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterOpenMonitoringElPrometheusElNodeExporterEl {
    #[doc = ""]
    pub enabled_in_broker: terrars::PrimField<bool>,
}

impl BuildMskClusterOpenMonitoringElPrometheusElNodeExporterEl {
    pub fn build(self) -> MskClusterOpenMonitoringElPrometheusElNodeExporterEl {
        MskClusterOpenMonitoringElPrometheusElNodeExporterEl { enabled_in_broker: self.enabled_in_broker }
    }
}

pub struct MskClusterOpenMonitoringElPrometheusElNodeExporterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterOpenMonitoringElPrometheusElNodeExporterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskClusterOpenMonitoringElPrometheusElNodeExporterElRef {
        MskClusterOpenMonitoringElPrometheusElNodeExporterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterOpenMonitoringElPrometheusElNodeExporterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled_in_broker` after provisioning.\n"]
    pub fn enabled_in_broker(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled_in_broker", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskClusterOpenMonitoringElPrometheusElDynamic {
    jmx_exporter: std::option::Option<terrars::DynamicBlock<MskClusterOpenMonitoringElPrometheusElJmxExporterEl>>,
    node_exporter: std::option::Option<terrars::DynamicBlock<MskClusterOpenMonitoringElPrometheusElNodeExporterEl>>,
}

#[derive(Serialize)]
pub struct MskClusterOpenMonitoringElPrometheusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jmx_exporter: std::option::Option<std::vec::Vec<MskClusterOpenMonitoringElPrometheusElJmxExporterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_exporter: std::option::Option<std::vec::Vec<MskClusterOpenMonitoringElPrometheusElNodeExporterEl>>,
    dynamic: MskClusterOpenMonitoringElPrometheusElDynamic,
}

impl MskClusterOpenMonitoringElPrometheusEl {
    #[doc = "Set the field `jmx_exporter`.\n"]
    pub fn set_jmx_exporter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskClusterOpenMonitoringElPrometheusElJmxExporterEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MskClusterOpenMonitoringElPrometheusElNodeExporterEl>>,
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

impl terrars::ToListMappable for MskClusterOpenMonitoringElPrometheusEl {
    type O = terrars::BlockAssignable<MskClusterOpenMonitoringElPrometheusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterOpenMonitoringElPrometheusEl {}

impl BuildMskClusterOpenMonitoringElPrometheusEl {
    pub fn build(self) -> MskClusterOpenMonitoringElPrometheusEl {
        MskClusterOpenMonitoringElPrometheusEl {
            jmx_exporter: core::default::Default::default(),
            node_exporter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MskClusterOpenMonitoringElPrometheusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterOpenMonitoringElPrometheusElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskClusterOpenMonitoringElPrometheusElRef {
        MskClusterOpenMonitoringElPrometheusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterOpenMonitoringElPrometheusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `jmx_exporter` after provisioning.\n"]
    pub fn jmx_exporter(&self) -> terrars::ListRef<MskClusterOpenMonitoringElPrometheusElJmxExporterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jmx_exporter", self.base))
    }

    #[doc = "Get a reference to the value of field `node_exporter` after provisioning.\n"]
    pub fn node_exporter(&self) -> terrars::ListRef<MskClusterOpenMonitoringElPrometheusElNodeExporterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_exporter", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskClusterOpenMonitoringElDynamic {
    prometheus: std::option::Option<terrars::DynamicBlock<MskClusterOpenMonitoringElPrometheusEl>>,
}

#[derive(Serialize)]
pub struct MskClusterOpenMonitoringEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prometheus: std::option::Option<std::vec::Vec<MskClusterOpenMonitoringElPrometheusEl>>,
    dynamic: MskClusterOpenMonitoringElDynamic,
}

impl MskClusterOpenMonitoringEl {
    #[doc = "Set the field `prometheus`.\n"]
    pub fn set_prometheus(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskClusterOpenMonitoringElPrometheusEl>>,
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

impl terrars::ToListMappable for MskClusterOpenMonitoringEl {
    type O = terrars::BlockAssignable<MskClusterOpenMonitoringEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterOpenMonitoringEl {}

impl BuildMskClusterOpenMonitoringEl {
    pub fn build(self) -> MskClusterOpenMonitoringEl {
        MskClusterOpenMonitoringEl {
            prometheus: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MskClusterOpenMonitoringElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterOpenMonitoringElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskClusterOpenMonitoringElRef {
        MskClusterOpenMonitoringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterOpenMonitoringElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prometheus` after provisioning.\n"]
    pub fn prometheus(&self) -> terrars::ListRef<MskClusterOpenMonitoringElPrometheusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prometheus", self.base))
    }
}

#[derive(Serialize)]
pub struct MskClusterTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskClusterTimeoutsEl {
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

impl terrars::ToListMappable for MskClusterTimeoutsEl {
    type O = terrars::BlockAssignable<MskClusterTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskClusterTimeoutsEl {}

impl BuildMskClusterTimeoutsEl {
    pub fn build(self) -> MskClusterTimeoutsEl {
        MskClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct MskClusterTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskClusterTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskClusterTimeoutsElRef {
        MskClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskClusterTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct MskClusterDynamic {
    broker_node_group_info: std::option::Option<terrars::DynamicBlock<MskClusterBrokerNodeGroupInfoEl>>,
    client_authentication: std::option::Option<terrars::DynamicBlock<MskClusterClientAuthenticationEl>>,
    configuration_info: std::option::Option<terrars::DynamicBlock<MskClusterConfigurationInfoEl>>,
    encryption_info: std::option::Option<terrars::DynamicBlock<MskClusterEncryptionInfoEl>>,
    logging_info: std::option::Option<terrars::DynamicBlock<MskClusterLoggingInfoEl>>,
    open_monitoring: std::option::Option<terrars::DynamicBlock<MskClusterOpenMonitoringEl>>,
}
