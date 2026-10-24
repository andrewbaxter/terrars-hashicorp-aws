use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataElasticacheReplicationGroupData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    replication_group_id: terrars::PrimField<std::string::String>,
}

struct DataElasticacheReplicationGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataElasticacheReplicationGroupData>,
}

#[derive(Clone)]
pub struct DataElasticacheReplicationGroup(std::rc::Rc<DataElasticacheReplicationGroup_>);

impl DataElasticacheReplicationGroup {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_token_enabled` after provisioning.\n"]
    pub fn auth_token_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_token_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automatic_failover_enabled` after provisioning.\n"]
    pub fn automatic_failover_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_failover_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_endpoint_address` after provisioning.\n"]
    pub fn configuration_endpoint_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_endpoint_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_delivery_configuration` after provisioning.\n"]
    pub fn log_delivery_configuration(
        &self,
    ) -> terrars::SetRef<DataElasticacheReplicationGroupLogDeliveryConfigurationElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.log_delivery_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `member_clusters` after provisioning.\n"]
    pub fn member_clusters(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.member_clusters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_az_enabled` after provisioning.\n"]
    pub fn multi_az_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multi_az_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `num_cache_clusters` after provisioning.\n"]
    pub fn num_cache_clusters(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_cache_clusters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `num_node_groups` after provisioning.\n"]
    pub fn num_node_groups(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_node_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_cache_clusters` after provisioning.\n"]
    pub fn number_cache_clusters(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_cache_clusters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `primary_endpoint_address` after provisioning.\n"]
    pub fn primary_endpoint_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_endpoint_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reader_endpoint_address` after provisioning.\n"]
    pub fn reader_endpoint_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reader_endpoint_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replicas_per_node_group` after provisioning.\n"]
    pub fn replicas_per_node_group(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replicas_per_node_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_group_description` after provisioning.\n"]
    pub fn replication_group_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_group_description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replication_group_id` after provisioning.\n"]
    pub fn replication_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_retention_limit` after provisioning.\n"]
    pub fn snapshot_retention_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_retention_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_window` after provisioning.\n"]
    pub fn snapshot_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_window", self.extract_ref()))
    }
}

impl terrars::Referable for DataElasticacheReplicationGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataElasticacheReplicationGroup { }

impl terrars::ToListMappable for DataElasticacheReplicationGroup {
    type O = terrars::ListRef<DataElasticacheReplicationGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataElasticacheReplicationGroup_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_elasticache_replication_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataElasticacheReplicationGroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub replication_group_id: terrars::PrimField<std::string::String>,
}

impl BuildDataElasticacheReplicationGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> DataElasticacheReplicationGroup {
        let out = DataElasticacheReplicationGroup(std::rc::Rc::new(DataElasticacheReplicationGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataElasticacheReplicationGroupData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                replication_group_id: self.replication_group_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataElasticacheReplicationGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataElasticacheReplicationGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataElasticacheReplicationGroupRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_token_enabled` after provisioning.\n"]
    pub fn auth_token_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_token_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automatic_failover_enabled` after provisioning.\n"]
    pub fn automatic_failover_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_failover_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_endpoint_address` after provisioning.\n"]
    pub fn configuration_endpoint_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_endpoint_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_delivery_configuration` after provisioning.\n"]
    pub fn log_delivery_configuration(
        &self,
    ) -> terrars::SetRef<DataElasticacheReplicationGroupLogDeliveryConfigurationElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.log_delivery_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `member_clusters` after provisioning.\n"]
    pub fn member_clusters(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.member_clusters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_az_enabled` after provisioning.\n"]
    pub fn multi_az_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multi_az_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `num_cache_clusters` after provisioning.\n"]
    pub fn num_cache_clusters(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_cache_clusters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `num_node_groups` after provisioning.\n"]
    pub fn num_node_groups(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_node_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_cache_clusters` after provisioning.\n"]
    pub fn number_cache_clusters(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_cache_clusters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `primary_endpoint_address` after provisioning.\n"]
    pub fn primary_endpoint_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_endpoint_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reader_endpoint_address` after provisioning.\n"]
    pub fn reader_endpoint_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reader_endpoint_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replicas_per_node_group` after provisioning.\n"]
    pub fn replicas_per_node_group(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replicas_per_node_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_group_description` after provisioning.\n"]
    pub fn replication_group_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_group_description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replication_group_id` after provisioning.\n"]
    pub fn replication_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_retention_limit` after provisioning.\n"]
    pub fn snapshot_retention_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_retention_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_window` after provisioning.\n"]
    pub fn snapshot_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_window", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataElasticacheReplicationGroupLogDeliveryConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataElasticacheReplicationGroupLogDeliveryConfigurationEl {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_type`.\n"]
    pub fn set_destination_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.destination_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_format`.\n"]
    pub fn set_log_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_type`.\n"]
    pub fn set_log_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataElasticacheReplicationGroupLogDeliveryConfigurationEl {
    type O = terrars::BlockAssignable<DataElasticacheReplicationGroupLogDeliveryConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataElasticacheReplicationGroupLogDeliveryConfigurationEl {}

impl BuildDataElasticacheReplicationGroupLogDeliveryConfigurationEl {
    pub fn build(self) -> DataElasticacheReplicationGroupLogDeliveryConfigurationEl {
        DataElasticacheReplicationGroupLogDeliveryConfigurationEl {
            destination: core::default::Default::default(),
            destination_type: core::default::Default::default(),
            log_format: core::default::Default::default(),
            log_type: core::default::Default::default(),
        }
    }
}

pub struct DataElasticacheReplicationGroupLogDeliveryConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataElasticacheReplicationGroupLogDeliveryConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataElasticacheReplicationGroupLogDeliveryConfigurationElRef {
        DataElasticacheReplicationGroupLogDeliveryConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataElasticacheReplicationGroupLogDeliveryConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_type` after provisioning.\n"]
    pub fn destination_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_type", self.base))
    }

    #[doc = "Get a reference to the value of field `log_format` after provisioning.\n"]
    pub fn log_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_format", self.base))
    }

    #[doc = "Get a reference to the value of field `log_type` after provisioning.\n"]
    pub fn log_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_type", self.base))
    }
}
