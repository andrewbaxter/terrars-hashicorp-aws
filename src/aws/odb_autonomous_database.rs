#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OdbAutonomousDatabaseData_ {
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
    admin_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_password_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_password_wo_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowlisted_ips: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_refresh_frequency_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_refresh_point_lag_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autonomous_maintenance_schedule_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_retention_period_in_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    byol_compute_count_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    character_set: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_storage_size_in_gbs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_storage_size_in_tbs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_edition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_workload: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key_provider: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_auto_scaling_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_auto_scaling_for_storage_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_backup_retention_locked: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_local_data_guard_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_mtls_connection_required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_refreshable_clone: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    license_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_adg_auto_failover_max_data_loss_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ncharacter_set: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    odb_network_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permission_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint_label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refreshable_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_pool_leader_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standby_allowlisted_ips: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standby_allowlisted_ips_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_of_auto_refresh_start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_password_source: std::option::Option<std::vec::Vec<OdbAutonomousDatabase_AdminPasswordSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_contacts_to_send_to_oci: std
    ::option
    ::Option<std::vec::Vec<OdbAutonomousDatabase_CustomerContactsToSendToOci>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_tools_details: std::option::Option<std::vec::Vec<OdbAutonomousDatabase_DbToolsDetails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    long_term_backup_schedule: std::option::Option<std::vec::Vec<OdbAutonomousDatabase_LongTermBackupSchedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_pool_summary: std::option::Option<std::vec::Vec<OdbAutonomousDatabase_ResourcePoolSummary>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduled_operations: std::option::Option<std::vec::Vec<OdbAutonomousDatabase_ScheduledOperations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_configuration: std::option::Option<std::vec::Vec<OdbAutonomousDatabase_SourceConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OdbAutonomousDatabase_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transportable_tablespace: std::option::Option<std::vec::Vec<OdbAutonomousDatabase_TransportableTablespace>>,
    dynamic: OdbAutonomousDatabaseDynamic,
}

struct OdbAutonomousDatabase_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OdbAutonomousDatabaseData_>,
}

#[derive(Clone)]
pub struct OdbAutonomousDatabase(std::rc::Rc<OdbAutonomousDatabase_>);

impl OdbAutonomousDatabase {
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

    #[doc =
        "Set the field `admin_password`.\nPassword for the ADMIN user. This value is stored in Terraform state. Use admin_password_wo with Terraform 1.11 or later to avoid storing the password in state."]
    pub fn set_admin_password(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().admin_password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `admin_password_wo`.\nPassword for the ADMIN user. This write-only value is never stored in Terraform state."]
    pub fn set_admin_password_wo(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().admin_password_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `admin_password_wo_version`.\nArbitrary version used to trigger an ADMIN password update."]
    pub fn set_admin_password_wo_version(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().admin_password_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowlisted_ips`.\nIP addresses allowed to access the Autonomous Database."]
    pub fn set_allowlisted_ips(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().allowlisted_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `auto_refresh_frequency_in_seconds`.\nFrequency at which a refreshable clone is automatically refreshed, in seconds."]
    pub fn set_auto_refresh_frequency_in_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().auto_refresh_frequency_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `auto_refresh_point_lag_in_seconds`.\nTime lag between a refreshable clone and its source, in seconds."]
    pub fn set_auto_refresh_point_lag_in_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().auto_refresh_point_lag_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `autonomous_maintenance_schedule_type`.\nMaintenance schedule type for the Autonomous Database."]
    pub fn set_autonomous_maintenance_schedule_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().autonomous_maintenance_schedule_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_retention_period_in_days`.\nRetention period for automatic backups, in days."]
    pub fn set_backup_retention_period_in_days(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().backup_retention_period_in_days = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `byol_compute_count_limit`.\nMaximum compute capacity under the bring-your-own-license model."]
    pub fn set_byol_compute_count_limit(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().byol_compute_count_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `character_set`.\nCharacter set of the Autonomous Database."]
    pub fn set_character_set(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().character_set = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compute_count`.\nCompute capacity in ECPUs or OCPUs."]
    pub fn set_compute_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().compute_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_core_count`.\nNumber of CPU cores allocated to the Autonomous Database."]
    pub fn set_cpu_core_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().cpu_core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_storage_size_in_gbs`.\nData volume size in GB."]
    pub fn set_data_storage_size_in_gbs(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().data_storage_size_in_gbs = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `data_storage_size_in_tbs`.\nData volume size in TB. Configured values must be whole numbers; computed values may be fractional when storage is configured in GB."]
    pub fn set_data_storage_size_in_tbs(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().data_storage_size_in_tbs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_edition`.\nOracle Database edition."]
    pub fn set_database_edition(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().database_edition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_name`.\nName of the Autonomous Database."]
    pub fn set_db_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().db_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_version`.\nOracle Database software version."]
    pub fn set_db_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().db_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_workload`.\nIntended database workload."]
    pub fn set_db_workload(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().db_workload = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nUser-friendly name for the Autonomous Database."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `encryption_key_provider`.\nEncryption key provider. Configurable values are ORACLE_MANAGED and AWS_KMS."]
    pub fn set_encryption_key_provider(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().encryption_key_provider = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_auto_scaling_enabled`.\nWhether automatic compute scaling is enabled."]
    pub fn set_is_auto_scaling_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_auto_scaling_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_auto_scaling_for_storage_enabled`.\nWhether automatic storage scaling is enabled."]
    pub fn set_is_auto_scaling_for_storage_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_auto_scaling_for_storage_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_backup_retention_locked`.\nWhether the backup retention period is locked."]
    pub fn set_is_backup_retention_locked(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_backup_retention_locked = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_local_data_guard_enabled`.\nWhether local Oracle Data Guard is enabled."]
    pub fn set_is_local_data_guard_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_local_data_guard_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_mtls_connection_required`.\nWhether mutual TLS authentication is required."]
    pub fn set_is_mtls_connection_required(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_mtls_connection_required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_refreshable_clone`.\nWhether the Autonomous Database is a refreshable clone."]
    pub fn set_is_refreshable_clone(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_refreshable_clone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\nARN of the AWS KMS key used to encrypt the Autonomous Database."]
    pub fn set_kms_key_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `license_model`.\nOracle license model."]
    pub fn set_license_model(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().license_model = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `local_adg_auto_failover_max_data_loss_limit`.\nMaximum data-loss limit for automatic local Data Guard failover, in seconds."]
    pub fn set_local_adg_auto_failover_max_data_loss_limit(
        self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.0.data.borrow_mut().local_adg_auto_failover_max_data_loss_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ncharacter_set`.\nNational character set of the Autonomous Database."]
    pub fn set_ncharacter_set(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ncharacter_set = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `odb_network_id`.\nID of the associated ODB network."]
    pub fn set_odb_network_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().odb_network_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `open_mode`.\nOpen mode of the Autonomous Database."]
    pub fn set_open_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().open_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `permission_level`.\nPermission level of the Autonomous Database."]
    pub fn set_permission_level(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().permission_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_endpoint_ip`.\nPrivate endpoint IP address."]
    pub fn set_private_endpoint_ip(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().private_endpoint_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_endpoint_label`.\nPrivate endpoint label."]
    pub fn set_private_endpoint_label(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().private_endpoint_label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `refreshable_mode`.\nRefresh mode of a refreshable clone."]
    pub fn set_refreshable_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().refreshable_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_pool_leader_id`.\nID of the resource-pool leader Autonomous Database."]
    pub fn set_resource_pool_leader_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().resource_pool_leader_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\nSource from which to create the Autonomous Database."]
    pub fn set_source(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `standby_allowlisted_ips`.\nIP addresses allowed to access the standby Autonomous Database."]
    pub fn set_standby_allowlisted_ips(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().standby_allowlisted_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `standby_allowlisted_ips_source`.\nSource of the standby allowlisted IP addresses."]
    pub fn set_standby_allowlisted_ips_source(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().standby_allowlisted_ips_source = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `time_of_auto_refresh_start`.\nDate and time when automatic refresh begins."]
    pub fn set_time_of_auto_refresh_start(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().time_of_auto_refresh_start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `admin_password_source`.\n"]
    pub fn set_admin_password_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OdbAutonomousDatabase_AdminPasswordSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().admin_password_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.admin_password_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `customer_contacts_to_send_to_oci`.\n"]
    pub fn set_customer_contacts_to_send_to_oci(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OdbAutonomousDatabase_CustomerContactsToSendToOci>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().customer_contacts_to_send_to_oci = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.customer_contacts_to_send_to_oci = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `db_tools_details`.\n"]
    pub fn set_db_tools_details(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OdbAutonomousDatabase_DbToolsDetails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().db_tools_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.db_tools_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `long_term_backup_schedule`.\n"]
    pub fn set_long_term_backup_schedule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OdbAutonomousDatabase_LongTermBackupSchedule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().long_term_backup_schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.long_term_backup_schedule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_pool_summary`.\n"]
    pub fn set_resource_pool_summary(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OdbAutonomousDatabase_ResourcePoolSummary>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_pool_summary = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_pool_summary = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scheduled_operations`.\n"]
    pub fn set_scheduled_operations(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OdbAutonomousDatabase_ScheduledOperations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scheduled_operations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scheduled_operations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_configuration`.\n"]
    pub fn set_source_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OdbAutonomousDatabase_SourceConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<OdbAutonomousDatabase_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transportable_tablespace`.\n"]
    pub fn set_transportable_tablespace(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OdbAutonomousDatabase_TransportableTablespace>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().transportable_tablespace = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.transportable_tablespace = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `actual_used_data_storage_size_in_tbs` after provisioning.\nActual amount of data storage currently in use, in TB."]
    pub fn actual_used_data_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.actual_used_data_storage_size_in_tbs", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `admin_password` after provisioning.\nPassword for the ADMIN user. This value is stored in Terraform state. Use admin_password_wo with Terraform 1.11 or later to avoid storing the password in state."]
    pub fn admin_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `admin_password_wo` after provisioning.\nPassword for the ADMIN user. This write-only value is never stored in Terraform state."]
    pub fn admin_password_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_password_wo", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `admin_password_wo_version` after provisioning.\nArbitrary version used to trigger an ADMIN password update."]
    pub fn admin_password_wo_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_password_wo_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allocated_storage_size_in_tbs` after provisioning.\nAmount of storage currently allocated, in TB."]
    pub fn allocated_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocated_storage_size_in_tbs", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `allowlisted_ips` after provisioning.\nIP addresses allowed to access the Autonomous Database."]
    pub fn allowlisted_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowlisted_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_refresh_frequency_in_seconds` after provisioning.\nFrequency at which a refreshable clone is automatically refreshed, in seconds."]
    pub fn auto_refresh_frequency_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_refresh_frequency_in_seconds", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `auto_refresh_point_lag_in_seconds` after provisioning.\nTime lag between a refreshable clone and its source, in seconds."]
    pub fn auto_refresh_point_lag_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_refresh_point_lag_in_seconds", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `autonomous_maintenance_schedule_type` after provisioning.\nMaintenance schedule type for the Autonomous Database."]
    pub fn autonomous_maintenance_schedule_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.autonomous_maintenance_schedule_type", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `availability_zone` after provisioning.\nAvailability Zone where the Autonomous Database is located."]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `availability_zone_id` after provisioning.\nAvailability Zone ID where the Autonomous Database is located."]
    pub fn availability_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `available_upgrade_versions` after provisioning.\nOracle Database versions to which the Autonomous Database can be upgraded."]
    pub fn available_upgrade_versions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.available_upgrade_versions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_retention_period_in_days` after provisioning.\nRetention period for automatic backups, in days."]
    pub fn backup_retention_period_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_retention_period_in_days", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `byol_compute_count_limit` after provisioning.\nMaximum compute capacity under the bring-your-own-license model."]
    pub fn byol_compute_count_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.byol_compute_count_limit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `character_set` after provisioning.\nCharacter set of the Autonomous Database."]
    pub fn character_set(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.character_set", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compute_count` after provisioning.\nCompute capacity in ECPUs or OCPUs."]
    pub fn compute_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compute_model` after provisioning.\nCompute model of the Autonomous Database."]
    pub fn compute_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count` after provisioning.\nNumber of CPU cores allocated to the Autonomous Database."]
    pub fn cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nDate and time when the Autonomous Database was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_gbs` after provisioning.\nData volume size in GB."]
    pub fn data_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_tbs` after provisioning.\nData volume size in TB. Configured values must be whole numbers; computed values may be fractional when storage is configured in GB."]
    pub fn data_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_tbs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_edition` after provisioning.\nOracle Database edition."]
    pub fn database_edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_edition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_type` after provisioning.\nType of Autonomous Database."]
    pub fn database_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_name` after provisioning.\nName of the Autonomous Database."]
    pub fn db_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_version` after provisioning.\nOracle Database software version."]
    pub fn db_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_workload` after provisioning.\nIntended database workload."]
    pub fn db_workload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_workload", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-friendly name for the Autonomous Database."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption_key_provider` after provisioning.\nEncryption key provider. Configurable values are ORACLE_MANAGED and AWS_KMS."]
    pub fn encryption_key_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_auto_scaling_enabled` after provisioning.\nWhether automatic compute scaling is enabled."]
    pub fn is_auto_scaling_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_auto_scaling_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_auto_scaling_for_storage_enabled` after provisioning.\nWhether automatic storage scaling is enabled."]
    pub fn is_auto_scaling_for_storage_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_auto_scaling_for_storage_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `is_backup_retention_locked` after provisioning.\nWhether the backup retention period is locked."]
    pub fn is_backup_retention_locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_backup_retention_locked", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_local_data_guard_enabled` after provisioning.\nWhether local Oracle Data Guard is enabled."]
    pub fn is_local_data_guard_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_local_data_guard_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_mtls_connection_required` after provisioning.\nWhether mutual TLS authentication is required."]
    pub fn is_mtls_connection_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_mtls_connection_required", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_refreshable_clone` after provisioning.\nWhether the Autonomous Database is a refreshable clone."]
    pub fn is_refreshable_clone(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_refreshable_clone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_id` after provisioning.\nARN of the AWS KMS key used to encrypt the Autonomous Database."]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `license_model` after provisioning.\nOracle license model."]
    pub fn license_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `local_adg_auto_failover_max_data_loss_limit` after provisioning.\nMaximum data-loss limit for automatic local Data Guard failover, in seconds."]
    pub fn local_adg_auto_failover_max_data_loss_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_adg_auto_failover_max_data_loss_limit", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `ncharacter_set` after provisioning.\nNational character set of the Autonomous Database."]
    pub fn ncharacter_set(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ncharacter_set", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nName of the OCI resource anchor."]
    pub fn oci_resource_anchor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_url` after provisioning.\nURL for the Autonomous Database in the OCI console."]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ocid` after provisioning.\nOracle Cloud Identifier of the Autonomous Database."]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network_arn` after provisioning.\nARN of the associated ODB network."]
    pub fn odb_network_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network_id` after provisioning.\nID of the associated ODB network."]
    pub fn odb_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `open_mode` after provisioning.\nOpen mode of the Autonomous Database."]
    pub fn open_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.open_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_progress` after provisioning.\nProgress of the current operation, as a percentage."]
    pub fn percent_progress(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `permission_level` after provisioning.\nPermission level of the Autonomous Database."]
    pub fn permission_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission_level", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_endpoint` after provisioning.\nPrivate endpoint of the Autonomous Database."]
    pub fn private_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_endpoint_ip` after provisioning.\nPrivate endpoint IP address."]
    pub fn private_endpoint_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_endpoint_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_endpoint_label` after provisioning.\nPrivate endpoint label."]
    pub fn private_endpoint_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_endpoint_label", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `refreshable_mode` after provisioning.\nRefresh mode of a refreshable clone."]
    pub fn refreshable_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refreshable_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_pool_leader_id` after provisioning.\nID of the resource-pool leader Autonomous Database."]
    pub fn resource_pool_leader_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_pool_leader_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_console_url` after provisioning.\nURL for the Oracle service console."]
    pub fn service_console_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_console_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nSource from which to create the Autonomous Database."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_id` after provisioning.\nID of the source used to create the Autonomous Database."]
    pub fn source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sql_web_developer_url` after provisioning.\nURL for Oracle SQL Developer Web."]
    pub fn sql_web_developer_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_web_developer_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `standby_allowlisted_ips` after provisioning.\nIP addresses allowed to access the standby Autonomous Database."]
    pub fn standby_allowlisted_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standby_allowlisted_ips", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `standby_allowlisted_ips_source` after provisioning.\nSource of the standby allowlisted IP addresses."]
    pub fn standby_allowlisted_ips_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.standby_allowlisted_ips_source", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nCurrent status of the Autonomous Database."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the current status."]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `time_of_auto_refresh_start` after provisioning.\nDate and time when automatic refresh begins."]
    pub fn time_of_auto_refresh_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_of_auto_refresh_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_password_source` after provisioning.\n"]
    pub fn admin_password_source(&self) -> terrars::ListRef<OdbAutonomousDatabase_AdminPasswordSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_password_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_contacts_to_send_to_oci` after provisioning.\n"]
    pub fn customer_contacts_to_send_to_oci(
        &self,
    ) -> terrars::ListRef<OdbAutonomousDatabase_CustomerContactsToSendToOciRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.customer_contacts_to_send_to_oci", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_tools_details` after provisioning.\n"]
    pub fn db_tools_details(&self) -> terrars::ListRef<OdbAutonomousDatabase_DbToolsDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db_tools_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `long_term_backup_schedule` after provisioning.\n"]
    pub fn long_term_backup_schedule(&self) -> terrars::ListRef<OdbAutonomousDatabase_LongTermBackupScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.long_term_backup_schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_pool_summary` after provisioning.\n"]
    pub fn resource_pool_summary(&self) -> terrars::ListRef<OdbAutonomousDatabase_ResourcePoolSummaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_pool_summary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduled_operations` after provisioning.\n"]
    pub fn scheduled_operations(&self) -> terrars::ListRef<OdbAutonomousDatabase_ScheduledOperationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduled_operations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_configuration` after provisioning.\n"]
    pub fn source_configuration(&self) -> terrars::ListRef<OdbAutonomousDatabase_SourceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OdbAutonomousDatabase_TimeoutsRef {
        OdbAutonomousDatabase_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transportable_tablespace` after provisioning.\n"]
    pub fn transportable_tablespace(&self) -> terrars::ListRef<OdbAutonomousDatabase_TransportableTablespaceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transportable_tablespace", self.extract_ref()))
    }
}

impl terrars::Referable for OdbAutonomousDatabase {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OdbAutonomousDatabase { }

impl terrars::ToListMappable for OdbAutonomousDatabase {
    type O = terrars::ListRef<OdbAutonomousDatabaseRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OdbAutonomousDatabase_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_odb_autonomous_database".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOdbAutonomousDatabase {
    pub tf_id: std::string::String,
}

impl BuildOdbAutonomousDatabase {
    pub fn build(self, stack: &mut terrars::Stack) -> OdbAutonomousDatabase {
        let out = OdbAutonomousDatabase(std::rc::Rc::new(OdbAutonomousDatabase_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OdbAutonomousDatabaseData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                admin_password: core::default::Default::default(),
                admin_password_wo: core::default::Default::default(),
                admin_password_wo_version: core::default::Default::default(),
                allowlisted_ips: core::default::Default::default(),
                auto_refresh_frequency_in_seconds: core::default::Default::default(),
                auto_refresh_point_lag_in_seconds: core::default::Default::default(),
                autonomous_maintenance_schedule_type: core::default::Default::default(),
                backup_retention_period_in_days: core::default::Default::default(),
                byol_compute_count_limit: core::default::Default::default(),
                character_set: core::default::Default::default(),
                compute_count: core::default::Default::default(),
                cpu_core_count: core::default::Default::default(),
                data_storage_size_in_gbs: core::default::Default::default(),
                data_storage_size_in_tbs: core::default::Default::default(),
                database_edition: core::default::Default::default(),
                db_name: core::default::Default::default(),
                db_version: core::default::Default::default(),
                db_workload: core::default::Default::default(),
                display_name: core::default::Default::default(),
                encryption_key_provider: core::default::Default::default(),
                is_auto_scaling_enabled: core::default::Default::default(),
                is_auto_scaling_for_storage_enabled: core::default::Default::default(),
                is_backup_retention_locked: core::default::Default::default(),
                is_local_data_guard_enabled: core::default::Default::default(),
                is_mtls_connection_required: core::default::Default::default(),
                is_refreshable_clone: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                license_model: core::default::Default::default(),
                local_adg_auto_failover_max_data_loss_limit: core::default::Default::default(),
                ncharacter_set: core::default::Default::default(),
                odb_network_id: core::default::Default::default(),
                open_mode: core::default::Default::default(),
                permission_level: core::default::Default::default(),
                private_endpoint_ip: core::default::Default::default(),
                private_endpoint_label: core::default::Default::default(),
                refreshable_mode: core::default::Default::default(),
                region: core::default::Default::default(),
                resource_pool_leader_id: core::default::Default::default(),
                source: core::default::Default::default(),
                standby_allowlisted_ips: core::default::Default::default(),
                standby_allowlisted_ips_source: core::default::Default::default(),
                tags: core::default::Default::default(),
                time_of_auto_refresh_start: core::default::Default::default(),
                admin_password_source: core::default::Default::default(),
                customer_contacts_to_send_to_oci: core::default::Default::default(),
                db_tools_details: core::default::Default::default(),
                long_term_backup_schedule: core::default::Default::default(),
                resource_pool_summary: core::default::Default::default(),
                scheduled_operations: core::default::Default::default(),
                source_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                transportable_tablespace: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OdbAutonomousDatabaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbAutonomousDatabaseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OdbAutonomousDatabaseRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `actual_used_data_storage_size_in_tbs` after provisioning.\nActual amount of data storage currently in use, in TB."]
    pub fn actual_used_data_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.actual_used_data_storage_size_in_tbs", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `admin_password` after provisioning.\nPassword for the ADMIN user. This value is stored in Terraform state. Use admin_password_wo with Terraform 1.11 or later to avoid storing the password in state."]
    pub fn admin_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `admin_password_wo` after provisioning.\nPassword for the ADMIN user. This write-only value is never stored in Terraform state."]
    pub fn admin_password_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_password_wo", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `admin_password_wo_version` after provisioning.\nArbitrary version used to trigger an ADMIN password update."]
    pub fn admin_password_wo_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_password_wo_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allocated_storage_size_in_tbs` after provisioning.\nAmount of storage currently allocated, in TB."]
    pub fn allocated_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocated_storage_size_in_tbs", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `allowlisted_ips` after provisioning.\nIP addresses allowed to access the Autonomous Database."]
    pub fn allowlisted_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowlisted_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_refresh_frequency_in_seconds` after provisioning.\nFrequency at which a refreshable clone is automatically refreshed, in seconds."]
    pub fn auto_refresh_frequency_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_refresh_frequency_in_seconds", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `auto_refresh_point_lag_in_seconds` after provisioning.\nTime lag between a refreshable clone and its source, in seconds."]
    pub fn auto_refresh_point_lag_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_refresh_point_lag_in_seconds", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `autonomous_maintenance_schedule_type` after provisioning.\nMaintenance schedule type for the Autonomous Database."]
    pub fn autonomous_maintenance_schedule_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.autonomous_maintenance_schedule_type", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `availability_zone` after provisioning.\nAvailability Zone where the Autonomous Database is located."]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `availability_zone_id` after provisioning.\nAvailability Zone ID where the Autonomous Database is located."]
    pub fn availability_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `available_upgrade_versions` after provisioning.\nOracle Database versions to which the Autonomous Database can be upgraded."]
    pub fn available_upgrade_versions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.available_upgrade_versions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_retention_period_in_days` after provisioning.\nRetention period for automatic backups, in days."]
    pub fn backup_retention_period_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_retention_period_in_days", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `byol_compute_count_limit` after provisioning.\nMaximum compute capacity under the bring-your-own-license model."]
    pub fn byol_compute_count_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.byol_compute_count_limit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `character_set` after provisioning.\nCharacter set of the Autonomous Database."]
    pub fn character_set(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.character_set", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compute_count` after provisioning.\nCompute capacity in ECPUs or OCPUs."]
    pub fn compute_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compute_model` after provisioning.\nCompute model of the Autonomous Database."]
    pub fn compute_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count` after provisioning.\nNumber of CPU cores allocated to the Autonomous Database."]
    pub fn cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nDate and time when the Autonomous Database was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_gbs` after provisioning.\nData volume size in GB."]
    pub fn data_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_tbs` after provisioning.\nData volume size in TB. Configured values must be whole numbers; computed values may be fractional when storage is configured in GB."]
    pub fn data_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_tbs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_edition` after provisioning.\nOracle Database edition."]
    pub fn database_edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_edition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_type` after provisioning.\nType of Autonomous Database."]
    pub fn database_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_name` after provisioning.\nName of the Autonomous Database."]
    pub fn db_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_version` after provisioning.\nOracle Database software version."]
    pub fn db_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_workload` after provisioning.\nIntended database workload."]
    pub fn db_workload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_workload", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-friendly name for the Autonomous Database."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption_key_provider` after provisioning.\nEncryption key provider. Configurable values are ORACLE_MANAGED and AWS_KMS."]
    pub fn encryption_key_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_auto_scaling_enabled` after provisioning.\nWhether automatic compute scaling is enabled."]
    pub fn is_auto_scaling_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_auto_scaling_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_auto_scaling_for_storage_enabled` after provisioning.\nWhether automatic storage scaling is enabled."]
    pub fn is_auto_scaling_for_storage_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_auto_scaling_for_storage_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `is_backup_retention_locked` after provisioning.\nWhether the backup retention period is locked."]
    pub fn is_backup_retention_locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_backup_retention_locked", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_local_data_guard_enabled` after provisioning.\nWhether local Oracle Data Guard is enabled."]
    pub fn is_local_data_guard_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_local_data_guard_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_mtls_connection_required` after provisioning.\nWhether mutual TLS authentication is required."]
    pub fn is_mtls_connection_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_mtls_connection_required", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_refreshable_clone` after provisioning.\nWhether the Autonomous Database is a refreshable clone."]
    pub fn is_refreshable_clone(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_refreshable_clone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_id` after provisioning.\nARN of the AWS KMS key used to encrypt the Autonomous Database."]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `license_model` after provisioning.\nOracle license model."]
    pub fn license_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `local_adg_auto_failover_max_data_loss_limit` after provisioning.\nMaximum data-loss limit for automatic local Data Guard failover, in seconds."]
    pub fn local_adg_auto_failover_max_data_loss_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_adg_auto_failover_max_data_loss_limit", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `ncharacter_set` after provisioning.\nNational character set of the Autonomous Database."]
    pub fn ncharacter_set(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ncharacter_set", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nName of the OCI resource anchor."]
    pub fn oci_resource_anchor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_url` after provisioning.\nURL for the Autonomous Database in the OCI console."]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ocid` after provisioning.\nOracle Cloud Identifier of the Autonomous Database."]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network_arn` after provisioning.\nARN of the associated ODB network."]
    pub fn odb_network_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network_id` after provisioning.\nID of the associated ODB network."]
    pub fn odb_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `open_mode` after provisioning.\nOpen mode of the Autonomous Database."]
    pub fn open_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.open_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_progress` after provisioning.\nProgress of the current operation, as a percentage."]
    pub fn percent_progress(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `permission_level` after provisioning.\nPermission level of the Autonomous Database."]
    pub fn permission_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission_level", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_endpoint` after provisioning.\nPrivate endpoint of the Autonomous Database."]
    pub fn private_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_endpoint_ip` after provisioning.\nPrivate endpoint IP address."]
    pub fn private_endpoint_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_endpoint_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_endpoint_label` after provisioning.\nPrivate endpoint label."]
    pub fn private_endpoint_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_endpoint_label", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `refreshable_mode` after provisioning.\nRefresh mode of a refreshable clone."]
    pub fn refreshable_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refreshable_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_pool_leader_id` after provisioning.\nID of the resource-pool leader Autonomous Database."]
    pub fn resource_pool_leader_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_pool_leader_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_console_url` after provisioning.\nURL for the Oracle service console."]
    pub fn service_console_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_console_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nSource from which to create the Autonomous Database."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_id` after provisioning.\nID of the source used to create the Autonomous Database."]
    pub fn source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sql_web_developer_url` after provisioning.\nURL for Oracle SQL Developer Web."]
    pub fn sql_web_developer_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_web_developer_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `standby_allowlisted_ips` after provisioning.\nIP addresses allowed to access the standby Autonomous Database."]
    pub fn standby_allowlisted_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standby_allowlisted_ips", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `standby_allowlisted_ips_source` after provisioning.\nSource of the standby allowlisted IP addresses."]
    pub fn standby_allowlisted_ips_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.standby_allowlisted_ips_source", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nCurrent status of the Autonomous Database."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the current status."]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `time_of_auto_refresh_start` after provisioning.\nDate and time when automatic refresh begins."]
    pub fn time_of_auto_refresh_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_of_auto_refresh_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_password_source` after provisioning.\n"]
    pub fn admin_password_source(&self) -> terrars::ListRef<OdbAutonomousDatabase_AdminPasswordSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_password_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_contacts_to_send_to_oci` after provisioning.\n"]
    pub fn customer_contacts_to_send_to_oci(
        &self,
    ) -> terrars::ListRef<OdbAutonomousDatabase_CustomerContactsToSendToOciRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.customer_contacts_to_send_to_oci", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_tools_details` after provisioning.\n"]
    pub fn db_tools_details(&self) -> terrars::ListRef<OdbAutonomousDatabase_DbToolsDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db_tools_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `long_term_backup_schedule` after provisioning.\n"]
    pub fn long_term_backup_schedule(&self) -> terrars::ListRef<OdbAutonomousDatabase_LongTermBackupScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.long_term_backup_schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_pool_summary` after provisioning.\n"]
    pub fn resource_pool_summary(&self) -> terrars::ListRef<OdbAutonomousDatabase_ResourcePoolSummaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_pool_summary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduled_operations` after provisioning.\n"]
    pub fn scheduled_operations(&self) -> terrars::ListRef<OdbAutonomousDatabase_ScheduledOperationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduled_operations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_configuration` after provisioning.\n"]
    pub fn source_configuration(&self) -> terrars::ListRef<OdbAutonomousDatabase_SourceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OdbAutonomousDatabase_TimeoutsRef {
        OdbAutonomousDatabase_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transportable_tablespace` after provisioning.\n"]
    pub fn transportable_tablespace(&self) -> terrars::ListRef<OdbAutonomousDatabase_TransportableTablespaceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transportable_tablespace", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct OdbAutonomousDatabaseDynamic {
    admin_password_source: std::option::Option<terrars::DynamicBlock<OdbAutonomousDatabase_AdminPasswordSource>>,
    customer_contacts_to_send_to_oci: std
    ::option
    ::Option<terrars::DynamicBlock<OdbAutonomousDatabase_CustomerContactsToSendToOci>>,
    db_tools_details: std::option::Option<terrars::DynamicBlock<OdbAutonomousDatabase_DbToolsDetails>>,
    long_term_backup_schedule: std
    ::option
    ::Option<terrars::DynamicBlock<OdbAutonomousDatabase_LongTermBackupSchedule>>,
    resource_pool_summary: std::option::Option<terrars::DynamicBlock<OdbAutonomousDatabase_ResourcePoolSummary>>,
    scheduled_operations: std::option::Option<terrars::DynamicBlock<OdbAutonomousDatabase_ScheduledOperations>>,
    source_configuration: std::option::Option<terrars::DynamicBlock<OdbAutonomousDatabase_SourceConfiguration>>,
    transportable_tablespace: std
    ::option
    ::Option<terrars::DynamicBlock<OdbAutonomousDatabase_TransportableTablespace>>,
}

#[derive(Serialize)]
pub struct OdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecret {
    external_id_type: terrars::PrimField<std::string::String>,
    iam_role_arn: terrars::PrimField<std::string::String>,
    secret_arn: terrars::PrimField<std::string::String>,
}

impl OdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecret { }

impl terrars::ToListMappable for OdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecret {
    type O = terrars::BlockAssignable<OdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecret>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecret {
    #[doc = "Type of OCI identifier supplied as the external ID when OCI assumes the IAM role."]
    pub external_id_type: terrars::PrimField<std::string::String>,
    #[doc = "ARN of the customer-managed IAM role OCI assumes to retrieve the secret."]
    pub iam_role_arn: terrars::PrimField<std::string::String>,
    #[doc = "ARN of the AWS Secrets Manager secret that contains the ADMIN password."]
    pub secret_arn: terrars::PrimField<std::string::String>,
}

impl BuildOdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecret {
    pub fn build(self) -> OdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecret {
        OdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecret {
            external_id_type: self.external_id_type,
            iam_role_arn: self.iam_role_arn,
            secret_arn: self.secret_arn,
        }
    }
}

#[allow(dead_code)]
pub struct OdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecretRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecretRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecretRef {
        OdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecretRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecretRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `external_id_type` after provisioning.\nType of OCI identifier supplied as the external ID when OCI assumes the IAM role."]
    pub fn external_id_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `iam_role_arn` after provisioning.\nARN of the customer-managed IAM role OCI assumes to retrieve the secret."]
    pub fn iam_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_arn` after provisioning.\nARN of the AWS Secrets Manager secret that contains the ADMIN password."]
    pub fn secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct OdbAutonomousDatabase_AdminPasswordSourceDynamic {
    customer_managed_aws_secret: std
    ::option
    ::Option<terrars::DynamicBlock<OdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecret>>,
}

#[derive(Serialize)]
pub struct OdbAutonomousDatabase_AdminPasswordSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_managed_aws_secret: std
    ::option
    ::Option<std::vec::Vec<OdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecret>>,
    dynamic: OdbAutonomousDatabase_AdminPasswordSourceDynamic,
}

impl OdbAutonomousDatabase_AdminPasswordSource {
    #[doc = "Set the field `customer_managed_aws_secret`.\n"]
    pub fn set_customer_managed_aws_secret(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<OdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecret>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.customer_managed_aws_secret = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.customer_managed_aws_secret = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OdbAutonomousDatabase_AdminPasswordSource {
    type O = terrars::BlockAssignable<OdbAutonomousDatabase_AdminPasswordSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbAutonomousDatabase_AdminPasswordSource {}

impl BuildOdbAutonomousDatabase_AdminPasswordSource {
    pub fn build(self) -> OdbAutonomousDatabase_AdminPasswordSource {
        OdbAutonomousDatabase_AdminPasswordSource {
            customer_managed_aws_secret: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OdbAutonomousDatabase_AdminPasswordSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbAutonomousDatabase_AdminPasswordSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OdbAutonomousDatabase_AdminPasswordSourceRef {
        OdbAutonomousDatabase_AdminPasswordSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbAutonomousDatabase_AdminPasswordSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `customer_managed_aws_secret` after provisioning.\n"]
    pub fn customer_managed_aws_secret(
        &self,
    ) -> terrars::ListRef<OdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecretRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_managed_aws_secret", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbAutonomousDatabase_CustomerContactsToSendToOci {
    email: terrars::PrimField<std::string::String>,
}

impl OdbAutonomousDatabase_CustomerContactsToSendToOci { }

impl terrars::ToListMappable for OdbAutonomousDatabase_CustomerContactsToSendToOci {
    type O = terrars::BlockAssignable<OdbAutonomousDatabase_CustomerContactsToSendToOci>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbAutonomousDatabase_CustomerContactsToSendToOci {
    #[doc = "Email address of the customer contact."]
    pub email: terrars::PrimField<std::string::String>,
}

impl BuildOdbAutonomousDatabase_CustomerContactsToSendToOci {
    pub fn build(self) -> OdbAutonomousDatabase_CustomerContactsToSendToOci {
        OdbAutonomousDatabase_CustomerContactsToSendToOci { email: self.email }
    }
}

#[allow(dead_code)]
pub struct OdbAutonomousDatabase_CustomerContactsToSendToOciRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbAutonomousDatabase_CustomerContactsToSendToOciRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OdbAutonomousDatabase_CustomerContactsToSendToOciRef {
        OdbAutonomousDatabase_CustomerContactsToSendToOciRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbAutonomousDatabase_CustomerContactsToSendToOciRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\nEmail address of the customer contact."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbAutonomousDatabase_DbToolsDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_idle_time_in_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OdbAutonomousDatabase_DbToolsDetails {
    #[doc = "Set the field `compute_count`.\nCompute capacity allocated to the database tool."]
    pub fn set_compute_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.compute_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_enabled`.\nWhether the database tool is enabled."]
    pub fn set_is_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_idle_time_in_minutes`.\nMaximum idle time before the database tool is shut down, in minutes."]
    pub fn set_max_idle_time_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_idle_time_in_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the database tool."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OdbAutonomousDatabase_DbToolsDetails {
    type O = terrars::BlockAssignable<OdbAutonomousDatabase_DbToolsDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbAutonomousDatabase_DbToolsDetails {}

impl BuildOdbAutonomousDatabase_DbToolsDetails {
    pub fn build(self) -> OdbAutonomousDatabase_DbToolsDetails {
        OdbAutonomousDatabase_DbToolsDetails {
            compute_count: core::default::Default::default(),
            is_enabled: core::default::Default::default(),
            max_idle_time_in_minutes: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OdbAutonomousDatabase_DbToolsDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbAutonomousDatabase_DbToolsDetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OdbAutonomousDatabase_DbToolsDetailsRef {
        OdbAutonomousDatabase_DbToolsDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbAutonomousDatabase_DbToolsDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `compute_count` after provisioning.\nCompute capacity allocated to the database tool."]
    pub fn compute_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_enabled` after provisioning.\nWhether the database tool is enabled."]
    pub fn is_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_idle_time_in_minutes` after provisioning.\nMaximum idle time before the database tool is shut down, in minutes."]
    pub fn max_idle_time_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_idle_time_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the database tool."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbAutonomousDatabase_LongTermBackupSchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repeat_cadence: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_period_in_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_of_backup: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OdbAutonomousDatabase_LongTermBackupSchedule {
    #[doc = "Set the field `is_disabled`.\nWhether the long-term backup schedule is disabled."]
    pub fn set_is_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repeat_cadence`.\nCadence at which long-term backups are taken."]
    pub fn set_repeat_cadence(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repeat_cadence = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention_period_in_days`.\nRetention period for long-term backups, in days."]
    pub fn set_retention_period_in_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retention_period_in_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_of_backup`.\nDate and time at which the long-term backup is taken."]
    pub fn set_time_of_backup(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_of_backup = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OdbAutonomousDatabase_LongTermBackupSchedule {
    type O = terrars::BlockAssignable<OdbAutonomousDatabase_LongTermBackupSchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbAutonomousDatabase_LongTermBackupSchedule {}

impl BuildOdbAutonomousDatabase_LongTermBackupSchedule {
    pub fn build(self) -> OdbAutonomousDatabase_LongTermBackupSchedule {
        OdbAutonomousDatabase_LongTermBackupSchedule {
            is_disabled: core::default::Default::default(),
            repeat_cadence: core::default::Default::default(),
            retention_period_in_days: core::default::Default::default(),
            time_of_backup: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OdbAutonomousDatabase_LongTermBackupScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbAutonomousDatabase_LongTermBackupScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OdbAutonomousDatabase_LongTermBackupScheduleRef {
        OdbAutonomousDatabase_LongTermBackupScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbAutonomousDatabase_LongTermBackupScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `is_disabled` after provisioning.\nWhether the long-term backup schedule is disabled."]
    pub fn is_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repeat_cadence` after provisioning.\nCadence at which long-term backups are taken."]
    pub fn repeat_cadence(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repeat_cadence", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retention_period_in_days` after provisioning.\nRetention period for long-term backups, in days."]
    pub fn retention_period_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_period_in_days", self.base))
    }

    #[doc =
        "Get a reference to the value of field `time_of_backup` after provisioning.\nDate and time at which the long-term backup is taken."]
    pub fn time_of_backup(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_of_backup", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbAutonomousDatabase_ResourcePoolSummary {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_storage_size_in_tbs: std::option::Option<terrars::PrimField<f64>>,
}

impl OdbAutonomousDatabase_ResourcePoolSummary {
    #[doc = "Set the field `is_disabled`.\nWhether the resource pool is disabled."]
    pub fn set_is_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pool_size`.\nNumber of Autonomous Databases the resource pool can contain."]
    pub fn set_pool_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pool_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pool_storage_size_in_tbs`.\nTotal storage size of the resource pool, in TB."]
    pub fn set_pool_storage_size_in_tbs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pool_storage_size_in_tbs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OdbAutonomousDatabase_ResourcePoolSummary {
    type O = terrars::BlockAssignable<OdbAutonomousDatabase_ResourcePoolSummary>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbAutonomousDatabase_ResourcePoolSummary {}

impl BuildOdbAutonomousDatabase_ResourcePoolSummary {
    pub fn build(self) -> OdbAutonomousDatabase_ResourcePoolSummary {
        OdbAutonomousDatabase_ResourcePoolSummary {
            is_disabled: core::default::Default::default(),
            pool_size: core::default::Default::default(),
            pool_storage_size_in_tbs: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OdbAutonomousDatabase_ResourcePoolSummaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbAutonomousDatabase_ResourcePoolSummaryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OdbAutonomousDatabase_ResourcePoolSummaryRef {
        OdbAutonomousDatabase_ResourcePoolSummaryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbAutonomousDatabase_ResourcePoolSummaryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `available_compute_capacity` after provisioning.\nAvailable compute capacity in the resource pool."]
    pub fn available_compute_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_compute_capacity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `available_storage_capacity_in_tbs` after provisioning.\nAvailable storage capacity in the resource pool, in TB."]
    pub fn available_storage_capacity_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_storage_capacity_in_tbs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_disabled` after provisioning.\nWhether the resource pool is disabled."]
    pub fn is_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pool_size` after provisioning.\nNumber of Autonomous Databases the resource pool can contain."]
    pub fn pool_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pool_storage_size_in_tbs` after provisioning.\nTotal storage size of the resource pool, in TB."]
    pub fn pool_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_storage_size_in_tbs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `total_compute_capacity` after provisioning.\nTotal compute capacity of the resource pool."]
    pub fn total_compute_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_compute_capacity", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbAutonomousDatabase_ScheduledOperations {
    day_of_week: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduled_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduled_stop_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OdbAutonomousDatabase_ScheduledOperations {
    #[doc = "Set the field `scheduled_start_time`.\nScheduled start time in UTC."]
    pub fn set_scheduled_start_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.scheduled_start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scheduled_stop_time`.\nScheduled stop time in UTC."]
    pub fn set_scheduled_stop_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.scheduled_stop_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OdbAutonomousDatabase_ScheduledOperations {
    type O = terrars::BlockAssignable<OdbAutonomousDatabase_ScheduledOperations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbAutonomousDatabase_ScheduledOperations {
    #[doc = "Day of the week."]
    pub day_of_week: terrars::PrimField<std::string::String>,
}

impl BuildOdbAutonomousDatabase_ScheduledOperations {
    pub fn build(self) -> OdbAutonomousDatabase_ScheduledOperations {
        OdbAutonomousDatabase_ScheduledOperations {
            day_of_week: self.day_of_week,
            scheduled_start_time: core::default::Default::default(),
            scheduled_stop_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OdbAutonomousDatabase_ScheduledOperationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbAutonomousDatabase_ScheduledOperationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OdbAutonomousDatabase_ScheduledOperationsRef {
        OdbAutonomousDatabase_ScheduledOperationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbAutonomousDatabase_ScheduledOperationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\nDay of the week."]
    pub fn day_of_week(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scheduled_start_time` after provisioning.\nScheduled start time in UTC."]
    pub fn scheduled_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduled_start_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scheduled_stop_time` after provisioning.\nScheduled stop time in UTC."]
    pub fn scheduled_stop_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduled_stop_time", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbAutonomousDatabase_SourceConfiguration_CloneToRefreshable {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_refresh_frequency_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_refresh_point_lag_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clone_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refreshable_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    source_autonomous_database_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_of_auto_refresh_start: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OdbAutonomousDatabase_SourceConfiguration_CloneToRefreshable {
    #[doc =
        "Set the field `auto_refresh_frequency_in_seconds`.\nFrequency at which the refreshable clone is automatically refreshed, in seconds."]
    pub fn set_auto_refresh_frequency_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.auto_refresh_frequency_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `auto_refresh_point_lag_in_seconds`.\nTime lag between the refreshable clone and its source, in seconds."]
    pub fn set_auto_refresh_point_lag_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.auto_refresh_point_lag_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `clone_type`.\nType of clone to create."]
    pub fn set_clone_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.clone_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `open_mode`.\nOpen mode of the refreshable clone."]
    pub fn set_open_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.open_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `refreshable_mode`.\nRefresh mode of the clone."]
    pub fn set_refreshable_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.refreshable_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_of_auto_refresh_start`.\nDate and time when automatic refresh starts."]
    pub fn set_time_of_auto_refresh_start(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.time_of_auto_refresh_start = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OdbAutonomousDatabase_SourceConfiguration_CloneToRefreshable {
    type O = terrars::BlockAssignable<OdbAutonomousDatabase_SourceConfiguration_CloneToRefreshable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbAutonomousDatabase_SourceConfiguration_CloneToRefreshable {
    #[doc = "ID of the source Autonomous Database."]
    pub source_autonomous_database_id: terrars::PrimField<std::string::String>,
}

impl BuildOdbAutonomousDatabase_SourceConfiguration_CloneToRefreshable {
    pub fn build(self) -> OdbAutonomousDatabase_SourceConfiguration_CloneToRefreshable {
        OdbAutonomousDatabase_SourceConfiguration_CloneToRefreshable {
            auto_refresh_frequency_in_seconds: core::default::Default::default(),
            auto_refresh_point_lag_in_seconds: core::default::Default::default(),
            clone_type: core::default::Default::default(),
            open_mode: core::default::Default::default(),
            refreshable_mode: core::default::Default::default(),
            source_autonomous_database_id: self.source_autonomous_database_id,
            time_of_auto_refresh_start: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OdbAutonomousDatabase_SourceConfiguration_CloneToRefreshableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbAutonomousDatabase_SourceConfiguration_CloneToRefreshableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OdbAutonomousDatabase_SourceConfiguration_CloneToRefreshableRef {
        OdbAutonomousDatabase_SourceConfiguration_CloneToRefreshableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbAutonomousDatabase_SourceConfiguration_CloneToRefreshableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auto_refresh_frequency_in_seconds` after provisioning.\nFrequency at which the refreshable clone is automatically refreshed, in seconds."]
    pub fn auto_refresh_frequency_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_refresh_frequency_in_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `auto_refresh_point_lag_in_seconds` after provisioning.\nTime lag between the refreshable clone and its source, in seconds."]
    pub fn auto_refresh_point_lag_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_refresh_point_lag_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `clone_type` after provisioning.\nType of clone to create."]
    pub fn clone_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.clone_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `open_mode` after provisioning.\nOpen mode of the refreshable clone."]
    pub fn open_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.open_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `refreshable_mode` after provisioning.\nRefresh mode of the clone."]
    pub fn refreshable_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refreshable_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_autonomous_database_id` after provisioning.\nID of the source Autonomous Database."]
    pub fn source_autonomous_database_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_autonomous_database_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `time_of_auto_refresh_start` after provisioning.\nDate and time when automatic refresh starts."]
    pub fn time_of_auto_refresh_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_of_auto_refresh_start", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbAutonomousDatabase_SourceConfiguration_CrossRegionDataGuard {
    source_autonomous_database_arn: terrars::PrimField<std::string::String>,
}

impl OdbAutonomousDatabase_SourceConfiguration_CrossRegionDataGuard { }

impl terrars::ToListMappable for OdbAutonomousDatabase_SourceConfiguration_CrossRegionDataGuard {
    type O = terrars::BlockAssignable<OdbAutonomousDatabase_SourceConfiguration_CrossRegionDataGuard>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbAutonomousDatabase_SourceConfiguration_CrossRegionDataGuard {
    #[doc = "ARN of the source Autonomous Database."]
    pub source_autonomous_database_arn: terrars::PrimField<std::string::String>,
}

impl BuildOdbAutonomousDatabase_SourceConfiguration_CrossRegionDataGuard {
    pub fn build(self) -> OdbAutonomousDatabase_SourceConfiguration_CrossRegionDataGuard {
        OdbAutonomousDatabase_SourceConfiguration_CrossRegionDataGuard {
            source_autonomous_database_arn: self.source_autonomous_database_arn,
        }
    }
}

#[allow(dead_code)]
pub struct OdbAutonomousDatabase_SourceConfiguration_CrossRegionDataGuardRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbAutonomousDatabase_SourceConfiguration_CrossRegionDataGuardRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OdbAutonomousDatabase_SourceConfiguration_CrossRegionDataGuardRef {
        OdbAutonomousDatabase_SourceConfiguration_CrossRegionDataGuardRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbAutonomousDatabase_SourceConfiguration_CrossRegionDataGuardRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `source_autonomous_database_arn` after provisioning.\nARN of the source Autonomous Database."]
    pub fn source_autonomous_database_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_autonomous_database_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbAutonomousDatabase_SourceConfiguration_CrossRegionDisasterRecovery {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_replicate_automatic_backups: std::option::Option<terrars::PrimField<bool>>,
    remote_disaster_recovery_type: terrars::PrimField<std::string::String>,
    source_autonomous_database_arn: terrars::PrimField<std::string::String>,
}

impl OdbAutonomousDatabase_SourceConfiguration_CrossRegionDisasterRecovery {
    #[doc =
        "Set the field `is_replicate_automatic_backups`.\nWhether automatic backups are replicated to the disaster recovery database."]
    pub fn set_is_replicate_automatic_backups(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_replicate_automatic_backups = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OdbAutonomousDatabase_SourceConfiguration_CrossRegionDisasterRecovery {
    type O = terrars::BlockAssignable<OdbAutonomousDatabase_SourceConfiguration_CrossRegionDisasterRecovery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbAutonomousDatabase_SourceConfiguration_CrossRegionDisasterRecovery {
    #[doc = "Type of remote disaster recovery."]
    pub remote_disaster_recovery_type: terrars::PrimField<std::string::String>,
    #[doc = "ARN of the source Autonomous Database."]
    pub source_autonomous_database_arn: terrars::PrimField<std::string::String>,
}

impl BuildOdbAutonomousDatabase_SourceConfiguration_CrossRegionDisasterRecovery {
    pub fn build(self) -> OdbAutonomousDatabase_SourceConfiguration_CrossRegionDisasterRecovery {
        OdbAutonomousDatabase_SourceConfiguration_CrossRegionDisasterRecovery {
            is_replicate_automatic_backups: core::default::Default::default(),
            remote_disaster_recovery_type: self.remote_disaster_recovery_type,
            source_autonomous_database_arn: self.source_autonomous_database_arn,
        }
    }
}

#[allow(dead_code)]
pub struct OdbAutonomousDatabase_SourceConfiguration_CrossRegionDisasterRecoveryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbAutonomousDatabase_SourceConfiguration_CrossRegionDisasterRecoveryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OdbAutonomousDatabase_SourceConfiguration_CrossRegionDisasterRecoveryRef {
        OdbAutonomousDatabase_SourceConfiguration_CrossRegionDisasterRecoveryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbAutonomousDatabase_SourceConfiguration_CrossRegionDisasterRecoveryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `is_replicate_automatic_backups` after provisioning.\nWhether automatic backups are replicated to the disaster recovery database."]
    pub fn is_replicate_automatic_backups(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_replicate_automatic_backups", self.base))
    }

    #[doc =
        "Get a reference to the value of field `remote_disaster_recovery_type` after provisioning.\nType of remote disaster recovery."]
    pub fn remote_disaster_recovery_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remote_disaster_recovery_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_autonomous_database_arn` after provisioning.\nARN of the source Autonomous Database."]
    pub fn source_autonomous_database_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_autonomous_database_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbAutonomousDatabase_SourceConfiguration_DatabaseClone {
    clone_type: terrars::PrimField<std::string::String>,
    source_autonomous_database_id: terrars::PrimField<std::string::String>,
}

impl OdbAutonomousDatabase_SourceConfiguration_DatabaseClone { }

impl terrars::ToListMappable for OdbAutonomousDatabase_SourceConfiguration_DatabaseClone {
    type O = terrars::BlockAssignable<OdbAutonomousDatabase_SourceConfiguration_DatabaseClone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbAutonomousDatabase_SourceConfiguration_DatabaseClone {
    #[doc = "Type of clone to create."]
    pub clone_type: terrars::PrimField<std::string::String>,
    #[doc = "ID of the source Autonomous Database."]
    pub source_autonomous_database_id: terrars::PrimField<std::string::String>,
}

impl BuildOdbAutonomousDatabase_SourceConfiguration_DatabaseClone {
    pub fn build(self) -> OdbAutonomousDatabase_SourceConfiguration_DatabaseClone {
        OdbAutonomousDatabase_SourceConfiguration_DatabaseClone {
            clone_type: self.clone_type,
            source_autonomous_database_id: self.source_autonomous_database_id,
        }
    }
}

#[allow(dead_code)]
pub struct OdbAutonomousDatabase_SourceConfiguration_DatabaseCloneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbAutonomousDatabase_SourceConfiguration_DatabaseCloneRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OdbAutonomousDatabase_SourceConfiguration_DatabaseCloneRef {
        OdbAutonomousDatabase_SourceConfiguration_DatabaseCloneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbAutonomousDatabase_SourceConfiguration_DatabaseCloneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `clone_type` after provisioning.\nType of clone to create."]
    pub fn clone_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.clone_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_autonomous_database_id` after provisioning.\nID of the source Autonomous Database."]
    pub fn source_autonomous_database_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_autonomous_database_id", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbAutonomousDatabase_SourceConfiguration_PointInTimeRestore {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clone_table_space_list: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    clone_type: terrars::PrimField<std::string::String>,
    source_autonomous_database_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamp: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_latest_available_backup_timestamp: std::option::Option<terrars::PrimField<bool>>,
}

impl OdbAutonomousDatabase_SourceConfiguration_PointInTimeRestore {
    #[doc = "Set the field `clone_table_space_list`.\nTablespace IDs to clone."]
    pub fn set_clone_table_space_list(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.clone_table_space_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timestamp`.\nDate and time to which the database is restored."]
    pub fn set_timestamp(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timestamp = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_latest_available_backup_timestamp`.\nWhether to use the latest available backup timestamp."]
    pub fn set_use_latest_available_backup_timestamp(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.use_latest_available_backup_timestamp = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OdbAutonomousDatabase_SourceConfiguration_PointInTimeRestore {
    type O = terrars::BlockAssignable<OdbAutonomousDatabase_SourceConfiguration_PointInTimeRestore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbAutonomousDatabase_SourceConfiguration_PointInTimeRestore {
    #[doc = "Type of clone to create."]
    pub clone_type: terrars::PrimField<std::string::String>,
    #[doc = "ID of the source Autonomous Database."]
    pub source_autonomous_database_id: terrars::PrimField<std::string::String>,
}

impl BuildOdbAutonomousDatabase_SourceConfiguration_PointInTimeRestore {
    pub fn build(self) -> OdbAutonomousDatabase_SourceConfiguration_PointInTimeRestore {
        OdbAutonomousDatabase_SourceConfiguration_PointInTimeRestore {
            clone_table_space_list: core::default::Default::default(),
            clone_type: self.clone_type,
            source_autonomous_database_id: self.source_autonomous_database_id,
            timestamp: core::default::Default::default(),
            use_latest_available_backup_timestamp: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OdbAutonomousDatabase_SourceConfiguration_PointInTimeRestoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbAutonomousDatabase_SourceConfiguration_PointInTimeRestoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OdbAutonomousDatabase_SourceConfiguration_PointInTimeRestoreRef {
        OdbAutonomousDatabase_SourceConfiguration_PointInTimeRestoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbAutonomousDatabase_SourceConfiguration_PointInTimeRestoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `clone_table_space_list` after provisioning.\nTablespace IDs to clone."]
    pub fn clone_table_space_list(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clone_table_space_list", self.base))
    }

    #[doc = "Get a reference to the value of field `clone_type` after provisioning.\nType of clone to create."]
    pub fn clone_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.clone_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_autonomous_database_id` after provisioning.\nID of the source Autonomous Database."]
    pub fn source_autonomous_database_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_autonomous_database_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timestamp` after provisioning.\nDate and time to which the database is restored."]
    pub fn timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timestamp", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_latest_available_backup_timestamp` after provisioning.\nWhether to use the latest available backup timestamp."]
    pub fn use_latest_available_backup_timestamp(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_latest_available_backup_timestamp", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbAutonomousDatabase_SourceConfiguration_RestoreFromBackup {
    autonomous_database_backup_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clone_table_space_list: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    clone_type: terrars::PrimField<std::string::String>,
}

impl OdbAutonomousDatabase_SourceConfiguration_RestoreFromBackup {
    #[doc = "Set the field `clone_table_space_list`.\nTablespace IDs to clone."]
    pub fn set_clone_table_space_list(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.clone_table_space_list = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OdbAutonomousDatabase_SourceConfiguration_RestoreFromBackup {
    type O = terrars::BlockAssignable<OdbAutonomousDatabase_SourceConfiguration_RestoreFromBackup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbAutonomousDatabase_SourceConfiguration_RestoreFromBackup {
    #[doc = "ID of the Autonomous Database backup."]
    pub autonomous_database_backup_id: terrars::PrimField<std::string::String>,
    #[doc = "Type of clone to create from the backup."]
    pub clone_type: terrars::PrimField<std::string::String>,
}

impl BuildOdbAutonomousDatabase_SourceConfiguration_RestoreFromBackup {
    pub fn build(self) -> OdbAutonomousDatabase_SourceConfiguration_RestoreFromBackup {
        OdbAutonomousDatabase_SourceConfiguration_RestoreFromBackup {
            autonomous_database_backup_id: self.autonomous_database_backup_id,
            clone_table_space_list: core::default::Default::default(),
            clone_type: self.clone_type,
        }
    }
}

#[allow(dead_code)]
pub struct OdbAutonomousDatabase_SourceConfiguration_RestoreFromBackupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbAutonomousDatabase_SourceConfiguration_RestoreFromBackupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OdbAutonomousDatabase_SourceConfiguration_RestoreFromBackupRef {
        OdbAutonomousDatabase_SourceConfiguration_RestoreFromBackupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbAutonomousDatabase_SourceConfiguration_RestoreFromBackupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `autonomous_database_backup_id` after provisioning.\nID of the Autonomous Database backup."]
    pub fn autonomous_database_backup_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autonomous_database_backup_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `clone_table_space_list` after provisioning.\nTablespace IDs to clone."]
    pub fn clone_table_space_list(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clone_table_space_list", self.base))
    }

    #[doc =
        "Get a reference to the value of field `clone_type` after provisioning.\nType of clone to create from the backup."]
    pub fn clone_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.clone_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct OdbAutonomousDatabase_SourceConfigurationDynamic {
    clone_to_refreshable: std
    ::option
    ::Option<terrars::DynamicBlock<OdbAutonomousDatabase_SourceConfiguration_CloneToRefreshable>>,
    cross_region_data_guard: std
    ::option
    ::Option<terrars::DynamicBlock<OdbAutonomousDatabase_SourceConfiguration_CrossRegionDataGuard>>,
    cross_region_disaster_recovery: std
    ::option
    ::Option<terrars::DynamicBlock<OdbAutonomousDatabase_SourceConfiguration_CrossRegionDisasterRecovery>>,
    database_clone: std
    ::option
    ::Option<terrars::DynamicBlock<OdbAutonomousDatabase_SourceConfiguration_DatabaseClone>>,
    point_in_time_restore: std
    ::option
    ::Option<terrars::DynamicBlock<OdbAutonomousDatabase_SourceConfiguration_PointInTimeRestore>>,
    restore_from_backup: std
    ::option
    ::Option<terrars::DynamicBlock<OdbAutonomousDatabase_SourceConfiguration_RestoreFromBackup>>,
}

#[derive(Serialize)]
pub struct OdbAutonomousDatabase_SourceConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clone_to_refreshable: std
    ::option
    ::Option<std::vec::Vec<OdbAutonomousDatabase_SourceConfiguration_CloneToRefreshable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_region_data_guard: std
    ::option
    ::Option<std::vec::Vec<OdbAutonomousDatabase_SourceConfiguration_CrossRegionDataGuard>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_region_disaster_recovery: std
    ::option
    ::Option<std::vec::Vec<OdbAutonomousDatabase_SourceConfiguration_CrossRegionDisasterRecovery>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_clone: std::option::Option<std::vec::Vec<OdbAutonomousDatabase_SourceConfiguration_DatabaseClone>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    point_in_time_restore: std
    ::option
    ::Option<std::vec::Vec<OdbAutonomousDatabase_SourceConfiguration_PointInTimeRestore>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_from_backup: std::option::Option<std::vec::Vec<OdbAutonomousDatabase_SourceConfiguration_RestoreFromBackup>>,
    dynamic: OdbAutonomousDatabase_SourceConfigurationDynamic,
}

impl OdbAutonomousDatabase_SourceConfiguration {
    #[doc = "Set the field `clone_to_refreshable`.\n"]
    pub fn set_clone_to_refreshable(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OdbAutonomousDatabase_SourceConfiguration_CloneToRefreshable>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.clone_to_refreshable = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.clone_to_refreshable = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cross_region_data_guard`.\n"]
    pub fn set_cross_region_data_guard(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OdbAutonomousDatabase_SourceConfiguration_CrossRegionDataGuard>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cross_region_data_guard = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cross_region_data_guard = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cross_region_disaster_recovery`.\n"]
    pub fn set_cross_region_disaster_recovery(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OdbAutonomousDatabase_SourceConfiguration_CrossRegionDisasterRecovery>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cross_region_disaster_recovery = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cross_region_disaster_recovery = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `database_clone`.\n"]
    pub fn set_database_clone(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OdbAutonomousDatabase_SourceConfiguration_DatabaseClone>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.database_clone = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.database_clone = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `point_in_time_restore`.\n"]
    pub fn set_point_in_time_restore(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OdbAutonomousDatabase_SourceConfiguration_PointInTimeRestore>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.point_in_time_restore = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.point_in_time_restore = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `restore_from_backup`.\n"]
    pub fn set_restore_from_backup(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OdbAutonomousDatabase_SourceConfiguration_RestoreFromBackup>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.restore_from_backup = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.restore_from_backup = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OdbAutonomousDatabase_SourceConfiguration {
    type O = terrars::BlockAssignable<OdbAutonomousDatabase_SourceConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbAutonomousDatabase_SourceConfiguration {}

impl BuildOdbAutonomousDatabase_SourceConfiguration {
    pub fn build(self) -> OdbAutonomousDatabase_SourceConfiguration {
        OdbAutonomousDatabase_SourceConfiguration {
            clone_to_refreshable: core::default::Default::default(),
            cross_region_data_guard: core::default::Default::default(),
            cross_region_disaster_recovery: core::default::Default::default(),
            database_clone: core::default::Default::default(),
            point_in_time_restore: core::default::Default::default(),
            restore_from_backup: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OdbAutonomousDatabase_SourceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbAutonomousDatabase_SourceConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OdbAutonomousDatabase_SourceConfigurationRef {
        OdbAutonomousDatabase_SourceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbAutonomousDatabase_SourceConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `clone_to_refreshable` after provisioning.\n"]
    pub fn clone_to_refreshable(
        &self,
    ) -> terrars::ListRef<OdbAutonomousDatabase_SourceConfiguration_CloneToRefreshableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clone_to_refreshable", self.base))
    }

    #[doc = "Get a reference to the value of field `cross_region_data_guard` after provisioning.\n"]
    pub fn cross_region_data_guard(
        &self,
    ) -> terrars::ListRef<OdbAutonomousDatabase_SourceConfiguration_CrossRegionDataGuardRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cross_region_data_guard", self.base))
    }

    #[doc = "Get a reference to the value of field `cross_region_disaster_recovery` after provisioning.\n"]
    pub fn cross_region_disaster_recovery(
        &self,
    ) -> terrars::ListRef<OdbAutonomousDatabase_SourceConfiguration_CrossRegionDisasterRecoveryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cross_region_disaster_recovery", self.base))
    }

    #[doc = "Get a reference to the value of field `database_clone` after provisioning.\n"]
    pub fn database_clone(&self) -> terrars::ListRef<OdbAutonomousDatabase_SourceConfiguration_DatabaseCloneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database_clone", self.base))
    }

    #[doc = "Get a reference to the value of field `point_in_time_restore` after provisioning.\n"]
    pub fn point_in_time_restore(
        &self,
    ) -> terrars::ListRef<OdbAutonomousDatabase_SourceConfiguration_PointInTimeRestoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.point_in_time_restore", self.base))
    }

    #[doc = "Get a reference to the value of field `restore_from_backup` after provisioning.\n"]
    pub fn restore_from_backup(
        &self,
    ) -> terrars::ListRef<OdbAutonomousDatabase_SourceConfiguration_RestoreFromBackupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_from_backup", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbAutonomousDatabase_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OdbAutonomousDatabase_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OdbAutonomousDatabase_Timeouts {
    type O = terrars::BlockAssignable<OdbAutonomousDatabase_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbAutonomousDatabase_Timeouts {}

impl BuildOdbAutonomousDatabase_Timeouts {
    pub fn build(self) -> OdbAutonomousDatabase_Timeouts {
        OdbAutonomousDatabase_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OdbAutonomousDatabase_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbAutonomousDatabase_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OdbAutonomousDatabase_TimeoutsRef {
        OdbAutonomousDatabase_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbAutonomousDatabase_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbAutonomousDatabase_TransportableTablespace {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tts_bundle_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OdbAutonomousDatabase_TransportableTablespace {
    #[doc = "Set the field `tts_bundle_url`.\nURL of the transportable tablespace bundle."]
    pub fn set_tts_bundle_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tts_bundle_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OdbAutonomousDatabase_TransportableTablespace {
    type O = terrars::BlockAssignable<OdbAutonomousDatabase_TransportableTablespace>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbAutonomousDatabase_TransportableTablespace {}

impl BuildOdbAutonomousDatabase_TransportableTablespace {
    pub fn build(self) -> OdbAutonomousDatabase_TransportableTablespace {
        OdbAutonomousDatabase_TransportableTablespace { tts_bundle_url: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct OdbAutonomousDatabase_TransportableTablespaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbAutonomousDatabase_TransportableTablespaceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OdbAutonomousDatabase_TransportableTablespaceRef {
        OdbAutonomousDatabase_TransportableTablespaceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbAutonomousDatabase_TransportableTablespaceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `tts_bundle_url` after provisioning.\nURL of the transportable tablespace bundle."]
    pub fn tts_bundle_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tts_bundle_url", self.base))
    }
}
