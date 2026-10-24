use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ElasticacheReplicationGroupData {
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
    apply_immediately: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    at_rest_encryption_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_minor_version_upgrade: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_failover_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zones: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_tiering_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    final_snapshot_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    global_replication_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multi_az_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_topic_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_cache_clusters: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_node_groups: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_cache_clusters: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_cache_cluster_azs: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replicas_per_node_group: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_group_description: std::option::Option<terrars::PrimField<std::string::String>>,
    replication_group_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_retention_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_window: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_encryption_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_mode: std::option::Option<std::vec::Vec<ElasticacheReplicationGroupClusterModeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_delivery_configuration: std::option::Option<std::vec::Vec<ElasticacheReplicationGroupLogDeliveryConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ElasticacheReplicationGroupTimeoutsEl>,
    dynamic: ElasticacheReplicationGroupDynamic,
}

struct ElasticacheReplicationGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ElasticacheReplicationGroupData>,
}

#[derive(Clone)]
pub struct ElasticacheReplicationGroup(std::rc::Rc<ElasticacheReplicationGroup_>);

impl ElasticacheReplicationGroup {
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

    #[doc = "Set the field `apply_immediately`.\n"]
    pub fn set_apply_immediately(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().apply_immediately = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `at_rest_encryption_enabled`.\n"]
    pub fn set_at_rest_encryption_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().at_rest_encryption_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_token`.\n"]
    pub fn set_auth_token(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().auth_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_minor_version_upgrade`.\n"]
    pub fn set_auto_minor_version_upgrade(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().auto_minor_version_upgrade = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `automatic_failover_enabled`.\n"]
    pub fn set_automatic_failover_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().automatic_failover_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zones`.\n"]
    pub fn set_availability_zones(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().availability_zones = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_tiering_enabled`.\n"]
    pub fn set_data_tiering_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().data_tiering_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `engine`.\n"]
    pub fn set_engine(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().engine = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `engine_version`.\n"]
    pub fn set_engine_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().engine_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `final_snapshot_identifier`.\n"]
    pub fn set_final_snapshot_identifier(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().final_snapshot_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `global_replication_group_id`.\n"]
    pub fn set_global_replication_group_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().global_replication_group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().maintenance_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `multi_az_enabled`.\n"]
    pub fn set_multi_az_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().multi_az_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_type`.\n"]
    pub fn set_node_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().node_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `notification_topic_arn`.\n"]
    pub fn set_notification_topic_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().notification_topic_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `num_cache_clusters`.\n"]
    pub fn set_num_cache_clusters(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().num_cache_clusters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `num_node_groups`.\n"]
    pub fn set_num_node_groups(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().num_node_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_cache_clusters`.\n"]
    pub fn set_number_cache_clusters(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().number_cache_clusters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameter_group_name`.\n"]
    pub fn set_parameter_group_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().parameter_group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_cache_cluster_azs`.\n"]
    pub fn set_preferred_cache_cluster_azs(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().preferred_cache_cluster_azs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replicas_per_node_group`.\n"]
    pub fn set_replicas_per_node_group(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().replicas_per_node_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replication_group_description`.\n"]
    pub fn set_replication_group_description(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().replication_group_description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_names`.\n"]
    pub fn set_security_group_names(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().security_group_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_arns`.\n"]
    pub fn set_snapshot_arns(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().snapshot_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_name`.\n"]
    pub fn set_snapshot_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().snapshot_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_retention_limit`.\n"]
    pub fn set_snapshot_retention_limit(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().snapshot_retention_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_window`.\n"]
    pub fn set_snapshot_window(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().snapshot_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_group_name`.\n"]
    pub fn set_subnet_group_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().subnet_group_name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `transit_encryption_enabled`.\n"]
    pub fn set_transit_encryption_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().transit_encryption_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_group_ids`.\n"]
    pub fn set_user_group_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().user_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_mode`.\n"]
    pub fn set_cluster_mode(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticacheReplicationGroupClusterModeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cluster_mode = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cluster_mode = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_delivery_configuration`.\n"]
    pub fn set_log_delivery_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticacheReplicationGroupLogDeliveryConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_delivery_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_delivery_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ElasticacheReplicationGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `apply_immediately` after provisioning.\n"]
    pub fn apply_immediately(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apply_immediately", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `at_rest_encryption_enabled` after provisioning.\n"]
    pub fn at_rest_encryption_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.at_rest_encryption_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_token` after provisioning.\n"]
    pub fn auth_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_minor_version_upgrade` after provisioning.\n"]
    pub fn auto_minor_version_upgrade(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_minor_version_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automatic_failover_enabled` after provisioning.\n"]
    pub fn automatic_failover_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_failover_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_enabled` after provisioning.\n"]
    pub fn cluster_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_endpoint_address` after provisioning.\n"]
    pub fn configuration_endpoint_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_endpoint_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_tiering_enabled` after provisioning.\n"]
    pub fn data_tiering_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_tiering_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version_actual` after provisioning.\n"]
    pub fn engine_version_actual(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_version_actual", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `final_snapshot_identifier` after provisioning.\n"]
    pub fn final_snapshot_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.final_snapshot_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_replication_group_id` after provisioning.\n"]
    pub fn global_replication_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_replication_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `notification_topic_arn` after provisioning.\n"]
    pub fn notification_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notification_topic_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `parameter_group_name` after provisioning.\n"]
    pub fn parameter_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameter_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_cache_cluster_azs` after provisioning.\n"]
    pub fn preferred_cache_cluster_azs(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferred_cache_cluster_azs", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_names` after provisioning.\n"]
    pub fn security_group_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_arns` after provisioning.\n"]
    pub fn snapshot_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.snapshot_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_name` after provisioning.\n"]
    pub fn snapshot_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_retention_limit` after provisioning.\n"]
    pub fn snapshot_retention_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_retention_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_window` after provisioning.\n"]
    pub fn snapshot_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_group_name` after provisioning.\n"]
    pub fn subnet_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_encryption_enabled` after provisioning.\n"]
    pub fn transit_encryption_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_encryption_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_group_ids` after provisioning.\n"]
    pub fn user_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.user_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_mode` after provisioning.\n"]
    pub fn cluster_mode(&self) -> terrars::ListRef<ElasticacheReplicationGroupClusterModeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ElasticacheReplicationGroupTimeoutsElRef {
        ElasticacheReplicationGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ElasticacheReplicationGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ElasticacheReplicationGroup { }

impl terrars::ToListMappable for ElasticacheReplicationGroup {
    type O = terrars::ListRef<ElasticacheReplicationGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ElasticacheReplicationGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_elasticache_replication_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildElasticacheReplicationGroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub replication_group_id: terrars::PrimField<std::string::String>,
}

impl BuildElasticacheReplicationGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> ElasticacheReplicationGroup {
        let out = ElasticacheReplicationGroup(std::rc::Rc::new(ElasticacheReplicationGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ElasticacheReplicationGroupData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                apply_immediately: core::default::Default::default(),
                at_rest_encryption_enabled: core::default::Default::default(),
                auth_token: core::default::Default::default(),
                auto_minor_version_upgrade: core::default::Default::default(),
                automatic_failover_enabled: core::default::Default::default(),
                availability_zones: core::default::Default::default(),
                data_tiering_enabled: core::default::Default::default(),
                description: core::default::Default::default(),
                engine: core::default::Default::default(),
                engine_version: core::default::Default::default(),
                final_snapshot_identifier: core::default::Default::default(),
                global_replication_group_id: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                maintenance_window: core::default::Default::default(),
                multi_az_enabled: core::default::Default::default(),
                node_type: core::default::Default::default(),
                notification_topic_arn: core::default::Default::default(),
                num_cache_clusters: core::default::Default::default(),
                num_node_groups: core::default::Default::default(),
                number_cache_clusters: core::default::Default::default(),
                parameter_group_name: core::default::Default::default(),
                port: core::default::Default::default(),
                preferred_cache_cluster_azs: core::default::Default::default(),
                replicas_per_node_group: core::default::Default::default(),
                replication_group_description: core::default::Default::default(),
                replication_group_id: self.replication_group_id,
                security_group_ids: core::default::Default::default(),
                security_group_names: core::default::Default::default(),
                snapshot_arns: core::default::Default::default(),
                snapshot_name: core::default::Default::default(),
                snapshot_retention_limit: core::default::Default::default(),
                snapshot_window: core::default::Default::default(),
                subnet_group_name: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                transit_encryption_enabled: core::default::Default::default(),
                user_group_ids: core::default::Default::default(),
                cluster_mode: core::default::Default::default(),
                log_delivery_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ElasticacheReplicationGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticacheReplicationGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ElasticacheReplicationGroupRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apply_immediately` after provisioning.\n"]
    pub fn apply_immediately(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apply_immediately", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `at_rest_encryption_enabled` after provisioning.\n"]
    pub fn at_rest_encryption_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.at_rest_encryption_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_token` after provisioning.\n"]
    pub fn auth_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_minor_version_upgrade` after provisioning.\n"]
    pub fn auto_minor_version_upgrade(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_minor_version_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automatic_failover_enabled` after provisioning.\n"]
    pub fn automatic_failover_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_failover_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_enabled` after provisioning.\n"]
    pub fn cluster_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_endpoint_address` after provisioning.\n"]
    pub fn configuration_endpoint_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_endpoint_address", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_tiering_enabled` after provisioning.\n"]
    pub fn data_tiering_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_tiering_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version_actual` after provisioning.\n"]
    pub fn engine_version_actual(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_version_actual", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `final_snapshot_identifier` after provisioning.\n"]
    pub fn final_snapshot_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.final_snapshot_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_replication_group_id` after provisioning.\n"]
    pub fn global_replication_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_replication_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `notification_topic_arn` after provisioning.\n"]
    pub fn notification_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notification_topic_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `parameter_group_name` after provisioning.\n"]
    pub fn parameter_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameter_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_cache_cluster_azs` after provisioning.\n"]
    pub fn preferred_cache_cluster_azs(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferred_cache_cluster_azs", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_names` after provisioning.\n"]
    pub fn security_group_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_arns` after provisioning.\n"]
    pub fn snapshot_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.snapshot_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_name` after provisioning.\n"]
    pub fn snapshot_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_retention_limit` after provisioning.\n"]
    pub fn snapshot_retention_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_retention_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_window` after provisioning.\n"]
    pub fn snapshot_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_group_name` after provisioning.\n"]
    pub fn subnet_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_encryption_enabled` after provisioning.\n"]
    pub fn transit_encryption_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_encryption_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_group_ids` after provisioning.\n"]
    pub fn user_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.user_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_mode` after provisioning.\n"]
    pub fn cluster_mode(&self) -> terrars::ListRef<ElasticacheReplicationGroupClusterModeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ElasticacheReplicationGroupTimeoutsElRef {
        ElasticacheReplicationGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ElasticacheReplicationGroupClusterModeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_node_groups: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replicas_per_node_group: std::option::Option<terrars::PrimField<f64>>,
}

impl ElasticacheReplicationGroupClusterModeEl {
    #[doc = "Set the field `num_node_groups`.\n"]
    pub fn set_num_node_groups(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_node_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replicas_per_node_group`.\n"]
    pub fn set_replicas_per_node_group(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.replicas_per_node_group = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElasticacheReplicationGroupClusterModeEl {
    type O = terrars::BlockAssignable<ElasticacheReplicationGroupClusterModeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticacheReplicationGroupClusterModeEl {}

impl BuildElasticacheReplicationGroupClusterModeEl {
    pub fn build(self) -> ElasticacheReplicationGroupClusterModeEl {
        ElasticacheReplicationGroupClusterModeEl {
            num_node_groups: core::default::Default::default(),
            replicas_per_node_group: core::default::Default::default(),
        }
    }
}

pub struct ElasticacheReplicationGroupClusterModeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticacheReplicationGroupClusterModeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticacheReplicationGroupClusterModeElRef {
        ElasticacheReplicationGroupClusterModeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticacheReplicationGroupClusterModeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `num_node_groups` after provisioning.\n"]
    pub fn num_node_groups(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_node_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `replicas_per_node_group` after provisioning.\n"]
    pub fn replicas_per_node_group(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replicas_per_node_group", self.base))
    }
}

#[derive(Serialize)]
pub struct ElasticacheReplicationGroupLogDeliveryConfigurationEl {
    destination: terrars::PrimField<std::string::String>,
    destination_type: terrars::PrimField<std::string::String>,
    log_format: terrars::PrimField<std::string::String>,
    log_type: terrars::PrimField<std::string::String>,
}

impl ElasticacheReplicationGroupLogDeliveryConfigurationEl { }

impl terrars::ToListMappable for ElasticacheReplicationGroupLogDeliveryConfigurationEl {
    type O = terrars::BlockAssignable<ElasticacheReplicationGroupLogDeliveryConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticacheReplicationGroupLogDeliveryConfigurationEl {
    #[doc = ""]
    pub destination: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub destination_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub log_format: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub log_type: terrars::PrimField<std::string::String>,
}

impl BuildElasticacheReplicationGroupLogDeliveryConfigurationEl {
    pub fn build(self) -> ElasticacheReplicationGroupLogDeliveryConfigurationEl {
        ElasticacheReplicationGroupLogDeliveryConfigurationEl {
            destination: self.destination,
            destination_type: self.destination_type,
            log_format: self.log_format,
            log_type: self.log_type,
        }
    }
}

pub struct ElasticacheReplicationGroupLogDeliveryConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticacheReplicationGroupLogDeliveryConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ElasticacheReplicationGroupLogDeliveryConfigurationElRef {
        ElasticacheReplicationGroupLogDeliveryConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticacheReplicationGroupLogDeliveryConfigurationElRef {
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

#[derive(Serialize)]
pub struct ElasticacheReplicationGroupTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ElasticacheReplicationGroupTimeoutsEl {
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

impl terrars::ToListMappable for ElasticacheReplicationGroupTimeoutsEl {
    type O = terrars::BlockAssignable<ElasticacheReplicationGroupTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticacheReplicationGroupTimeoutsEl {}

impl BuildElasticacheReplicationGroupTimeoutsEl {
    pub fn build(self) -> ElasticacheReplicationGroupTimeoutsEl {
        ElasticacheReplicationGroupTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ElasticacheReplicationGroupTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticacheReplicationGroupTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticacheReplicationGroupTimeoutsElRef {
        ElasticacheReplicationGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticacheReplicationGroupTimeoutsElRef {
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
struct ElasticacheReplicationGroupDynamic {
    cluster_mode: std::option::Option<terrars::DynamicBlock<ElasticacheReplicationGroupClusterModeEl>>,
    log_delivery_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<ElasticacheReplicationGroupLogDeliveryConfigurationEl>>,
}
