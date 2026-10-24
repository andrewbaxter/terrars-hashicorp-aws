use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RdsClusterData {
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
    allocated_storage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_major_version_upgrade: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    apply_immediately: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zones: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backtrack_window: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_retention_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_identifier_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_members: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_tags_to_snapshot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_cluster_instance_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_cluster_parameter_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_instance_parameter_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_subnet_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_global_write_forwarding: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_http_endpoint: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled_cloudwatch_logs_exports: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    final_snapshot_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    global_cluster_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_database_authentication_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_roles: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_username: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_backup_window: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_maintenance_window: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_source_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_final_snapshot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_encrypted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_to_point_in_time: std::option::Option<std::vec::Vec<RdsClusterRestoreToPointInTimeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_import: std::option::Option<std::vec::Vec<RdsClusterS3ImportEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_configuration: std::option::Option<std::vec::Vec<RdsClusterScalingConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serverlessv2_scaling_configuration: std::option::Option<std::vec::Vec<RdsClusterServerlessv2ScalingConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<RdsClusterTimeoutsEl>,
    dynamic: RdsClusterDynamic,
}

struct RdsCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RdsClusterData>,
}

#[derive(Clone)]
pub struct RdsCluster(std::rc::Rc<RdsCluster_>);

impl RdsCluster {
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

    #[doc = "Set the field `allocated_storage`.\n"]
    pub fn set_allocated_storage(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().allocated_storage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_major_version_upgrade`.\n"]
    pub fn set_allow_major_version_upgrade(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_major_version_upgrade = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `apply_immediately`.\n"]
    pub fn set_apply_immediately(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().apply_immediately = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `backtrack_window`.\n"]
    pub fn set_backtrack_window(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().backtrack_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_retention_period`.\n"]
    pub fn set_backup_retention_period(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().backup_retention_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_identifier`.\n"]
    pub fn set_cluster_identifier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cluster_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_identifier_prefix`.\n"]
    pub fn set_cluster_identifier_prefix(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().cluster_identifier_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_members`.\n"]
    pub fn set_cluster_members(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().cluster_members = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `copy_tags_to_snapshot`.\n"]
    pub fn set_copy_tags_to_snapshot(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().copy_tags_to_snapshot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().database_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_cluster_instance_class`.\n"]
    pub fn set_db_cluster_instance_class(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().db_cluster_instance_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_cluster_parameter_group_name`.\n"]
    pub fn set_db_cluster_parameter_group_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().db_cluster_parameter_group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_instance_parameter_group_name`.\n"]
    pub fn set_db_instance_parameter_group_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().db_instance_parameter_group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_subnet_group_name`.\n"]
    pub fn set_db_subnet_group_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().db_subnet_group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_protection`.\n"]
    pub fn set_deletion_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_global_write_forwarding`.\n"]
    pub fn set_enable_global_write_forwarding(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_global_write_forwarding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_http_endpoint`.\n"]
    pub fn set_enable_http_endpoint(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_http_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled_cloudwatch_logs_exports`.\n"]
    pub fn set_enabled_cloudwatch_logs_exports(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().enabled_cloudwatch_logs_exports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `engine`.\n"]
    pub fn set_engine(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().engine = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `engine_mode`.\n"]
    pub fn set_engine_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().engine_mode = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `global_cluster_identifier`.\n"]
    pub fn set_global_cluster_identifier(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().global_cluster_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iam_database_authentication_enabled`.\n"]
    pub fn set_iam_database_authentication_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().iam_database_authentication_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iam_roles`.\n"]
    pub fn set_iam_roles(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().iam_roles = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `master_password`.\n"]
    pub fn set_master_password(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().master_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `master_username`.\n"]
    pub fn set_master_username(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().master_username = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_type`.\n"]
    pub fn set_network_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_backup_window`.\n"]
    pub fn set_preferred_backup_window(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().preferred_backup_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_maintenance_window`.\n"]
    pub fn set_preferred_maintenance_window(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().preferred_maintenance_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replication_source_identifier`.\n"]
    pub fn set_replication_source_identifier(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().replication_source_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `skip_final_snapshot`.\n"]
    pub fn set_skip_final_snapshot(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_final_snapshot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_identifier`.\n"]
    pub fn set_snapshot_identifier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().snapshot_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_region`.\n"]
    pub fn set_source_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source_region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_encrypted`.\n"]
    pub fn set_storage_encrypted(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().storage_encrypted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_type`.\n"]
    pub fn set_storage_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().storage_type = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `vpc_security_group_ids`.\n"]
    pub fn set_vpc_security_group_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().vpc_security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restore_to_point_in_time`.\n"]
    pub fn set_restore_to_point_in_time(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RdsClusterRestoreToPointInTimeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restore_to_point_in_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restore_to_point_in_time = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_import`.\n"]
    pub fn set_s3_import(self, v: impl std::convert::Into<terrars::BlockAssignable<RdsClusterS3ImportEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3_import = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3_import = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scaling_configuration`.\n"]
    pub fn set_scaling_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RdsClusterScalingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scaling_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scaling_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `serverlessv2_scaling_configuration`.\n"]
    pub fn set_serverlessv2_scaling_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RdsClusterServerlessv2ScalingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().serverlessv2_scaling_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.serverlessv2_scaling_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<RdsClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `allocated_storage` after provisioning.\n"]
    pub fn allocated_storage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allow_major_version_upgrade` after provisioning.\n"]
    pub fn allow_major_version_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_major_version_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `apply_immediately` after provisioning.\n"]
    pub fn apply_immediately(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apply_immediately", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backtrack_window` after provisioning.\n"]
    pub fn backtrack_window(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backtrack_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_retention_period` after provisioning.\n"]
    pub fn backup_retention_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_identifier_prefix` after provisioning.\n"]
    pub fn cluster_identifier_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_members` after provisioning.\n"]
    pub fn cluster_members(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cluster_members", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_resource_id` after provisioning.\n"]
    pub fn cluster_resource_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `copy_tags_to_snapshot` after provisioning.\n"]
    pub fn copy_tags_to_snapshot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_tags_to_snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_cluster_instance_class` after provisioning.\n"]
    pub fn db_cluster_instance_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_cluster_instance_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_cluster_parameter_group_name` after provisioning.\n"]
    pub fn db_cluster_parameter_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_cluster_parameter_group_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_instance_parameter_group_name` after provisioning.\n"]
    pub fn db_instance_parameter_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_instance_parameter_group_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_subnet_group_name` after provisioning.\n"]
    pub fn db_subnet_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_subnet_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_global_write_forwarding` after provisioning.\n"]
    pub fn enable_global_write_forwarding(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_global_write_forwarding", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_http_endpoint` after provisioning.\n"]
    pub fn enable_http_endpoint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_http_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled_cloudwatch_logs_exports` after provisioning.\n"]
    pub fn enabled_cloudwatch_logs_exports(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.enabled_cloudwatch_logs_exports", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_mode` after provisioning.\n"]
    pub fn engine_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_mode", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `global_cluster_identifier` after provisioning.\n"]
    pub fn global_cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_database_authentication_enabled` after provisioning.\n"]
    pub fn iam_database_authentication_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_database_authentication_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `iam_roles` after provisioning.\n"]
    pub fn iam_roles(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.iam_roles", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iops", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_password` after provisioning.\n"]
    pub fn master_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_username` after provisioning.\n"]
    pub fn master_username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_username", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_type` after provisioning.\n"]
    pub fn network_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_backup_window` after provisioning.\n"]
    pub fn preferred_backup_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_backup_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_maintenance_window` after provisioning.\n"]
    pub fn preferred_maintenance_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reader_endpoint` after provisioning.\n"]
    pub fn reader_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reader_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_source_identifier` after provisioning.\n"]
    pub fn replication_source_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_source_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `skip_final_snapshot` after provisioning.\n"]
    pub fn skip_final_snapshot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_final_snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_identifier` after provisioning.\n"]
    pub fn snapshot_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_region` after provisioning.\n"]
    pub fn source_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_encrypted` after provisioning.\n"]
    pub fn storage_encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_to_point_in_time` after provisioning.\n"]
    pub fn restore_to_point_in_time(&self) -> terrars::ListRef<RdsClusterRestoreToPointInTimeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_to_point_in_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_import` after provisioning.\n"]
    pub fn s3_import(&self) -> terrars::ListRef<RdsClusterS3ImportElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_import", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_configuration` after provisioning.\n"]
    pub fn scaling_configuration(&self) -> terrars::ListRef<RdsClusterScalingConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `serverlessv2_scaling_configuration` after provisioning.\n"]
    pub fn serverlessv2_scaling_configuration(
        &self,
    ) -> terrars::ListRef<RdsClusterServerlessv2ScalingConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.serverlessv2_scaling_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RdsClusterTimeoutsElRef {
        RdsClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for RdsCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for RdsCluster { }

impl terrars::ToListMappable for RdsCluster {
    type O = terrars::ListRef<RdsClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for RdsCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_rds_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRdsCluster {
    pub tf_id: std::string::String,
}

impl BuildRdsCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> RdsCluster {
        let out = RdsCluster(std::rc::Rc::new(RdsCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(RdsClusterData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allocated_storage: core::default::Default::default(),
                allow_major_version_upgrade: core::default::Default::default(),
                apply_immediately: core::default::Default::default(),
                availability_zones: core::default::Default::default(),
                backtrack_window: core::default::Default::default(),
                backup_retention_period: core::default::Default::default(),
                cluster_identifier: core::default::Default::default(),
                cluster_identifier_prefix: core::default::Default::default(),
                cluster_members: core::default::Default::default(),
                copy_tags_to_snapshot: core::default::Default::default(),
                database_name: core::default::Default::default(),
                db_cluster_instance_class: core::default::Default::default(),
                db_cluster_parameter_group_name: core::default::Default::default(),
                db_instance_parameter_group_name: core::default::Default::default(),
                db_subnet_group_name: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                enable_global_write_forwarding: core::default::Default::default(),
                enable_http_endpoint: core::default::Default::default(),
                enabled_cloudwatch_logs_exports: core::default::Default::default(),
                engine: core::default::Default::default(),
                engine_mode: core::default::Default::default(),
                engine_version: core::default::Default::default(),
                final_snapshot_identifier: core::default::Default::default(),
                global_cluster_identifier: core::default::Default::default(),
                iam_database_authentication_enabled: core::default::Default::default(),
                iam_roles: core::default::Default::default(),
                id: core::default::Default::default(),
                iops: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                master_password: core::default::Default::default(),
                master_username: core::default::Default::default(),
                network_type: core::default::Default::default(),
                port: core::default::Default::default(),
                preferred_backup_window: core::default::Default::default(),
                preferred_maintenance_window: core::default::Default::default(),
                replication_source_identifier: core::default::Default::default(),
                skip_final_snapshot: core::default::Default::default(),
                snapshot_identifier: core::default::Default::default(),
                source_region: core::default::Default::default(),
                storage_encrypted: core::default::Default::default(),
                storage_type: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                vpc_security_group_ids: core::default::Default::default(),
                restore_to_point_in_time: core::default::Default::default(),
                s3_import: core::default::Default::default(),
                scaling_configuration: core::default::Default::default(),
                serverlessv2_scaling_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RdsClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RdsClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RdsClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocated_storage` after provisioning.\n"]
    pub fn allocated_storage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allow_major_version_upgrade` after provisioning.\n"]
    pub fn allow_major_version_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_major_version_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `apply_immediately` after provisioning.\n"]
    pub fn apply_immediately(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apply_immediately", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backtrack_window` after provisioning.\n"]
    pub fn backtrack_window(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backtrack_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_retention_period` after provisioning.\n"]
    pub fn backup_retention_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_identifier_prefix` after provisioning.\n"]
    pub fn cluster_identifier_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_members` after provisioning.\n"]
    pub fn cluster_members(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cluster_members", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_resource_id` after provisioning.\n"]
    pub fn cluster_resource_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `copy_tags_to_snapshot` after provisioning.\n"]
    pub fn copy_tags_to_snapshot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_tags_to_snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_cluster_instance_class` after provisioning.\n"]
    pub fn db_cluster_instance_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_cluster_instance_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_cluster_parameter_group_name` after provisioning.\n"]
    pub fn db_cluster_parameter_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_cluster_parameter_group_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_instance_parameter_group_name` after provisioning.\n"]
    pub fn db_instance_parameter_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_instance_parameter_group_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_subnet_group_name` after provisioning.\n"]
    pub fn db_subnet_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_subnet_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_global_write_forwarding` after provisioning.\n"]
    pub fn enable_global_write_forwarding(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_global_write_forwarding", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_http_endpoint` after provisioning.\n"]
    pub fn enable_http_endpoint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_http_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled_cloudwatch_logs_exports` after provisioning.\n"]
    pub fn enabled_cloudwatch_logs_exports(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.enabled_cloudwatch_logs_exports", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_mode` after provisioning.\n"]
    pub fn engine_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_mode", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `global_cluster_identifier` after provisioning.\n"]
    pub fn global_cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_database_authentication_enabled` after provisioning.\n"]
    pub fn iam_database_authentication_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_database_authentication_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `iam_roles` after provisioning.\n"]
    pub fn iam_roles(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.iam_roles", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iops", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_password` after provisioning.\n"]
    pub fn master_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_username` after provisioning.\n"]
    pub fn master_username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_username", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_type` after provisioning.\n"]
    pub fn network_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_backup_window` after provisioning.\n"]
    pub fn preferred_backup_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_backup_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_maintenance_window` after provisioning.\n"]
    pub fn preferred_maintenance_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reader_endpoint` after provisioning.\n"]
    pub fn reader_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reader_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_source_identifier` after provisioning.\n"]
    pub fn replication_source_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_source_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `skip_final_snapshot` after provisioning.\n"]
    pub fn skip_final_snapshot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_final_snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_identifier` after provisioning.\n"]
    pub fn snapshot_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_region` after provisioning.\n"]
    pub fn source_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_encrypted` after provisioning.\n"]
    pub fn storage_encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_to_point_in_time` after provisioning.\n"]
    pub fn restore_to_point_in_time(&self) -> terrars::ListRef<RdsClusterRestoreToPointInTimeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_to_point_in_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_import` after provisioning.\n"]
    pub fn s3_import(&self) -> terrars::ListRef<RdsClusterS3ImportElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_import", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_configuration` after provisioning.\n"]
    pub fn scaling_configuration(&self) -> terrars::ListRef<RdsClusterScalingConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `serverlessv2_scaling_configuration` after provisioning.\n"]
    pub fn serverlessv2_scaling_configuration(
        &self,
    ) -> terrars::ListRef<RdsClusterServerlessv2ScalingConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.serverlessv2_scaling_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RdsClusterTimeoutsElRef {
        RdsClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RdsClusterRestoreToPointInTimeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_to_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_type: std::option::Option<terrars::PrimField<std::string::String>>,
    source_cluster_identifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_latest_restorable_time: std::option::Option<terrars::PrimField<bool>>,
}

impl RdsClusterRestoreToPointInTimeEl {
    #[doc = "Set the field `restore_to_time`.\n"]
    pub fn set_restore_to_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.restore_to_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restore_type`.\n"]
    pub fn set_restore_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.restore_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_latest_restorable_time`.\n"]
    pub fn set_use_latest_restorable_time(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_latest_restorable_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RdsClusterRestoreToPointInTimeEl {
    type O = terrars::BlockAssignable<RdsClusterRestoreToPointInTimeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRdsClusterRestoreToPointInTimeEl {
    #[doc = ""]
    pub source_cluster_identifier: terrars::PrimField<std::string::String>,
}

impl BuildRdsClusterRestoreToPointInTimeEl {
    pub fn build(self) -> RdsClusterRestoreToPointInTimeEl {
        RdsClusterRestoreToPointInTimeEl {
            restore_to_time: core::default::Default::default(),
            restore_type: core::default::Default::default(),
            source_cluster_identifier: self.source_cluster_identifier,
            use_latest_restorable_time: core::default::Default::default(),
        }
    }
}

pub struct RdsClusterRestoreToPointInTimeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RdsClusterRestoreToPointInTimeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RdsClusterRestoreToPointInTimeElRef {
        RdsClusterRestoreToPointInTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RdsClusterRestoreToPointInTimeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `restore_to_time` after provisioning.\n"]
    pub fn restore_to_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restore_to_time", self.base))
    }

    #[doc = "Get a reference to the value of field `restore_type` after provisioning.\n"]
    pub fn restore_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restore_type", self.base))
    }

    #[doc = "Get a reference to the value of field `source_cluster_identifier` after provisioning.\n"]
    pub fn source_cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_cluster_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `use_latest_restorable_time` after provisioning.\n"]
    pub fn use_latest_restorable_time(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_latest_restorable_time", self.base))
    }
}

#[derive(Serialize)]
pub struct RdsClusterS3ImportEl {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    ingestion_role: terrars::PrimField<std::string::String>,
    source_engine: terrars::PrimField<std::string::String>,
    source_engine_version: terrars::PrimField<std::string::String>,
}

impl RdsClusterS3ImportEl {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RdsClusterS3ImportEl {
    type O = terrars::BlockAssignable<RdsClusterS3ImportEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRdsClusterS3ImportEl {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub ingestion_role: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source_engine: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source_engine_version: terrars::PrimField<std::string::String>,
}

impl BuildRdsClusterS3ImportEl {
    pub fn build(self) -> RdsClusterS3ImportEl {
        RdsClusterS3ImportEl {
            bucket_name: self.bucket_name,
            bucket_prefix: core::default::Default::default(),
            ingestion_role: self.ingestion_role,
            source_engine: self.source_engine,
            source_engine_version: self.source_engine_version,
        }
    }
}

pub struct RdsClusterS3ImportElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RdsClusterS3ImportElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RdsClusterS3ImportElRef {
        RdsClusterS3ImportElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RdsClusterS3ImportElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `ingestion_role` after provisioning.\n"]
    pub fn ingestion_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingestion_role", self.base))
    }

    #[doc = "Get a reference to the value of field `source_engine` after provisioning.\n"]
    pub fn source_engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_engine", self.base))
    }

    #[doc = "Get a reference to the value of field `source_engine_version` after provisioning.\n"]
    pub fn source_engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_engine_version", self.base))
    }
}

#[derive(Serialize)]
pub struct RdsClusterScalingConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_pause: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds_until_auto_pause: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_action: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RdsClusterScalingConfigurationEl {
    #[doc = "Set the field `auto_pause`.\n"]
    pub fn set_auto_pause(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_pause = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_capacity`.\n"]
    pub fn set_max_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_capacity`.\n"]
    pub fn set_min_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seconds_until_auto_pause`.\n"]
    pub fn set_seconds_until_auto_pause(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds_until_auto_pause = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_action`.\n"]
    pub fn set_timeout_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timeout_action = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RdsClusterScalingConfigurationEl {
    type O = terrars::BlockAssignable<RdsClusterScalingConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRdsClusterScalingConfigurationEl {}

impl BuildRdsClusterScalingConfigurationEl {
    pub fn build(self) -> RdsClusterScalingConfigurationEl {
        RdsClusterScalingConfigurationEl {
            auto_pause: core::default::Default::default(),
            max_capacity: core::default::Default::default(),
            min_capacity: core::default::Default::default(),
            seconds_until_auto_pause: core::default::Default::default(),
            timeout_action: core::default::Default::default(),
        }
    }
}

pub struct RdsClusterScalingConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RdsClusterScalingConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RdsClusterScalingConfigurationElRef {
        RdsClusterScalingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RdsClusterScalingConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_pause` after provisioning.\n"]
    pub fn auto_pause(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_pause", self.base))
    }

    #[doc = "Get a reference to the value of field `max_capacity` after provisioning.\n"]
    pub fn max_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `min_capacity` after provisioning.\n"]
    pub fn min_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `seconds_until_auto_pause` after provisioning.\n"]
    pub fn seconds_until_auto_pause(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds_until_auto_pause", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_action` after provisioning.\n"]
    pub fn timeout_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_action", self.base))
    }
}

#[derive(Serialize)]
pub struct RdsClusterServerlessv2ScalingConfigurationEl {
    max_capacity: terrars::PrimField<f64>,
    min_capacity: terrars::PrimField<f64>,
}

impl RdsClusterServerlessv2ScalingConfigurationEl { }

impl terrars::ToListMappable for RdsClusterServerlessv2ScalingConfigurationEl {
    type O = terrars::BlockAssignable<RdsClusterServerlessv2ScalingConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRdsClusterServerlessv2ScalingConfigurationEl {
    #[doc = ""]
    pub max_capacity: terrars::PrimField<f64>,
    #[doc = ""]
    pub min_capacity: terrars::PrimField<f64>,
}

impl BuildRdsClusterServerlessv2ScalingConfigurationEl {
    pub fn build(self) -> RdsClusterServerlessv2ScalingConfigurationEl {
        RdsClusterServerlessv2ScalingConfigurationEl {
            max_capacity: self.max_capacity,
            min_capacity: self.min_capacity,
        }
    }
}

pub struct RdsClusterServerlessv2ScalingConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RdsClusterServerlessv2ScalingConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RdsClusterServerlessv2ScalingConfigurationElRef {
        RdsClusterServerlessv2ScalingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RdsClusterServerlessv2ScalingConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_capacity` after provisioning.\n"]
    pub fn max_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `min_capacity` after provisioning.\n"]
    pub fn min_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_capacity", self.base))
    }
}

#[derive(Serialize)]
pub struct RdsClusterTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RdsClusterTimeoutsEl {
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

impl terrars::ToListMappable for RdsClusterTimeoutsEl {
    type O = terrars::BlockAssignable<RdsClusterTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRdsClusterTimeoutsEl {}

impl BuildRdsClusterTimeoutsEl {
    pub fn build(self) -> RdsClusterTimeoutsEl {
        RdsClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct RdsClusterTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RdsClusterTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RdsClusterTimeoutsElRef {
        RdsClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RdsClusterTimeoutsElRef {
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
struct RdsClusterDynamic {
    restore_to_point_in_time: std::option::Option<terrars::DynamicBlock<RdsClusterRestoreToPointInTimeEl>>,
    s3_import: std::option::Option<terrars::DynamicBlock<RdsClusterS3ImportEl>>,
    scaling_configuration: std::option::Option<terrars::DynamicBlock<RdsClusterScalingConfigurationEl>>,
    serverlessv2_scaling_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<RdsClusterServerlessv2ScalingConfigurationEl>>,
}
