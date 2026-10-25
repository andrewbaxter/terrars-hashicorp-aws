#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOdbAutonomousDatabaseData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataOdbAutonomousDatabase_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOdbAutonomousDatabaseData_>,
}

#[derive(Clone)]
pub struct DataOdbAutonomousDatabase(std::rc::Rc<DataOdbAutonomousDatabase_>);

impl DataOdbAutonomousDatabase {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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
        "Get a reference to the value of field `admin_password_source` after provisioning.\nSource of the ADMIN password."]
    pub fn admin_password_source(&self) -> terrars::ListRef<DataOdbAutonomousDatabase_AdminPasswordSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_password_source", self.extract_ref()))
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
        "Get a reference to the value of field `auto_refresh_frequency_in_seconds` after provisioning.\nAutomatic refresh frequency, in seconds."]
    pub fn auto_refresh_frequency_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_refresh_frequency_in_seconds", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `auto_refresh_point_lag_in_seconds` after provisioning.\nRefresh lag from the source, in seconds."]
    pub fn auto_refresh_point_lag_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_refresh_point_lag_in_seconds", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `autonomous_maintenance_schedule_type` after provisioning.\nMaintenance schedule type."]
    pub fn autonomous_maintenance_schedule_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.autonomous_maintenance_schedule_type", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `availability_zone` after provisioning.\nAvailability Zone of the Autonomous Database."]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `availability_zone_id` after provisioning.\nAvailability Zone ID of the Autonomous Database."]
    pub fn availability_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `available_upgrade_versions` after provisioning.\nOracle Database versions available for upgrade."]
    pub fn available_upgrade_versions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.available_upgrade_versions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_retention_period_in_days` after provisioning.\nAutomatic backup retention period, in days."]
    pub fn backup_retention_period_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_retention_period_in_days", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `byol_compute_count_limit` after provisioning.\nMaximum BYOL compute capacity."]
    pub fn byol_compute_count_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.byol_compute_count_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `character_set` after provisioning.\nDatabase character set."]
    pub fn character_set(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.character_set", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compute_count` after provisioning.\nCompute capacity in ECPUs or OCPUs."]
    pub fn compute_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_model` after provisioning.\nCompute model."]
    pub fn compute_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu_core_count` after provisioning.\nAllocated CPU core count."]
    pub fn cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nCreation date and time."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_contacts_to_send_to_oci` after provisioning.\nCustomer contacts that receive operational notifications from OCI."]
    pub fn customer_contacts_to_send_to_oci(
        &self,
    ) -> terrars::ListRef<DataOdbAutonomousDatabase_CustomerContactsToSendToOciRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.customer_contacts_to_send_to_oci", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_gbs` after provisioning.\nData volume size in GB."]
    pub fn data_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_tbs` after provisioning.\nData volume size in TB."]
    pub fn data_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_tbs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_edition` after provisioning.\nOracle Database edition."]
    pub fn database_edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_edition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_type` after provisioning.\nAutonomous Database type."]
    pub fn database_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_name` after provisioning.\nDatabase name."]
    pub fn db_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_tools_details` after provisioning.\nDatabase management tools enabled for the Autonomous Database."]
    pub fn db_tools_details(&self) -> terrars::ListRef<DataOdbAutonomousDatabase_DbToolsDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db_tools_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_version` after provisioning.\nOracle Database software version."]
    pub fn db_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_workload` after provisioning.\nDatabase workload."]
    pub fn db_workload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_workload", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-friendly database name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption_key_provider` after provisioning.\nEncryption key provider."]
    pub fn encryption_key_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_provider", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nUnique identifier of the Autonomous Database."]
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
        "Get a reference to the value of field `is_backup_retention_locked` after provisioning.\nWhether backup retention is locked."]
    pub fn is_backup_retention_locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_backup_retention_locked", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_local_data_guard_enabled` after provisioning.\nWhether local Data Guard is enabled."]
    pub fn is_local_data_guard_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_local_data_guard_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_mtls_connection_required` after provisioning.\nWhether mTLS is required."]
    pub fn is_mtls_connection_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_mtls_connection_required", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_refreshable_clone` after provisioning.\nWhether the database is a refreshable clone."]
    pub fn is_refreshable_clone(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_refreshable_clone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_id` after provisioning.\nARN of the AWS KMS encryption key."]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `license_model` after provisioning.\nOracle license model."]
    pub fn license_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `local_adg_auto_failover_max_data_loss_limit` after provisioning.\nMaximum automatic-failover data-loss limit, in seconds."]
    pub fn local_adg_auto_failover_max_data_loss_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_adg_auto_failover_max_data_loss_limit", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `long_term_backup_schedule` after provisioning.\nLong-term backup schedule."]
    pub fn long_term_backup_schedule(&self) -> terrars::ListRef<DataOdbAutonomousDatabase_LongTermBackupScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.long_term_backup_schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ncharacter_set` after provisioning.\nNational character set."]
    pub fn ncharacter_set(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ncharacter_set", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nOCI resource anchor name."]
    pub fn oci_resource_anchor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oci_url` after provisioning.\nOCI console URL."]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\nOracle Cloud Identifier."]
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

    #[doc = "Get a reference to the value of field `open_mode` after provisioning.\nDatabase open mode."]
    pub fn open_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.open_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_progress` after provisioning.\nProgress of the current operation."]
    pub fn percent_progress(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission_level` after provisioning.\nDatabase permission level."]
    pub fn permission_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission_level", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_endpoint` after provisioning.\nPrivate endpoint."]
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

    #[doc = "Get a reference to the value of field `refreshable_mode` after provisioning.\nRefreshable clone mode."]
    pub fn refreshable_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refreshable_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_pool_leader_id` after provisioning.\nResource-pool leader database ID."]
    pub fn resource_pool_leader_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_pool_leader_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_pool_summary` after provisioning.\nResource pool configuration."]
    pub fn resource_pool_summary(&self) -> terrars::ListRef<DataOdbAutonomousDatabase_ResourcePoolSummaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_pool_summary", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scheduled_operations` after provisioning.\nScheduled database start and stop times."]
    pub fn scheduled_operations(&self) -> terrars::ListRef<DataOdbAutonomousDatabase_ScheduledOperationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduled_operations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_console_url` after provisioning.\nOracle service console URL."]
    pub fn service_console_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_console_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_id` after provisioning.\nSource database or backup ID."]
    pub fn source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sql_web_developer_url` after provisioning.\nOracle SQL Developer Web URL."]
    pub fn sql_web_developer_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_web_developer_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `standby_allowlisted_ips` after provisioning.\nIP addresses allowed to access the standby database."]
    pub fn standby_allowlisted_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standby_allowlisted_ips", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `standby_allowlisted_ips_source` after provisioning.\nSource of standby allowlisted IPs."]
    pub fn standby_allowlisted_ips_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.standby_allowlisted_ips_source", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nCurrent database status."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\nAdditional status information."]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `time_of_auto_refresh_start` after provisioning.\nAutomatic refresh start time."]
    pub fn time_of_auto_refresh_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_of_auto_refresh_start", self.extract_ref()))
    }
}

impl terrars::Referable for DataOdbAutonomousDatabase {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOdbAutonomousDatabase { }

impl terrars::ToListMappable for DataOdbAutonomousDatabase {
    type O = terrars::ListRef<DataOdbAutonomousDatabaseRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOdbAutonomousDatabase_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_odb_autonomous_database".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOdbAutonomousDatabase {
    pub tf_id: std::string::String,
    #[doc = "Unique identifier of the Autonomous Database."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildDataOdbAutonomousDatabase {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOdbAutonomousDatabase {
        let out = DataOdbAutonomousDatabase(std::rc::Rc::new(DataOdbAutonomousDatabase_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOdbAutonomousDatabaseData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: self.id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOdbAutonomousDatabaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbAutonomousDatabaseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOdbAutonomousDatabaseRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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
        "Get a reference to the value of field `admin_password_source` after provisioning.\nSource of the ADMIN password."]
    pub fn admin_password_source(&self) -> terrars::ListRef<DataOdbAutonomousDatabase_AdminPasswordSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_password_source", self.extract_ref()))
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
        "Get a reference to the value of field `auto_refresh_frequency_in_seconds` after provisioning.\nAutomatic refresh frequency, in seconds."]
    pub fn auto_refresh_frequency_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_refresh_frequency_in_seconds", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `auto_refresh_point_lag_in_seconds` after provisioning.\nRefresh lag from the source, in seconds."]
    pub fn auto_refresh_point_lag_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_refresh_point_lag_in_seconds", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `autonomous_maintenance_schedule_type` after provisioning.\nMaintenance schedule type."]
    pub fn autonomous_maintenance_schedule_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.autonomous_maintenance_schedule_type", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `availability_zone` after provisioning.\nAvailability Zone of the Autonomous Database."]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `availability_zone_id` after provisioning.\nAvailability Zone ID of the Autonomous Database."]
    pub fn availability_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `available_upgrade_versions` after provisioning.\nOracle Database versions available for upgrade."]
    pub fn available_upgrade_versions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.available_upgrade_versions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_retention_period_in_days` after provisioning.\nAutomatic backup retention period, in days."]
    pub fn backup_retention_period_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_retention_period_in_days", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `byol_compute_count_limit` after provisioning.\nMaximum BYOL compute capacity."]
    pub fn byol_compute_count_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.byol_compute_count_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `character_set` after provisioning.\nDatabase character set."]
    pub fn character_set(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.character_set", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compute_count` after provisioning.\nCompute capacity in ECPUs or OCPUs."]
    pub fn compute_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_model` after provisioning.\nCompute model."]
    pub fn compute_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu_core_count` after provisioning.\nAllocated CPU core count."]
    pub fn cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nCreation date and time."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_contacts_to_send_to_oci` after provisioning.\nCustomer contacts that receive operational notifications from OCI."]
    pub fn customer_contacts_to_send_to_oci(
        &self,
    ) -> terrars::ListRef<DataOdbAutonomousDatabase_CustomerContactsToSendToOciRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.customer_contacts_to_send_to_oci", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_gbs` after provisioning.\nData volume size in GB."]
    pub fn data_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_tbs` after provisioning.\nData volume size in TB."]
    pub fn data_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_tbs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_edition` after provisioning.\nOracle Database edition."]
    pub fn database_edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_edition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_type` after provisioning.\nAutonomous Database type."]
    pub fn database_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_name` after provisioning.\nDatabase name."]
    pub fn db_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_tools_details` after provisioning.\nDatabase management tools enabled for the Autonomous Database."]
    pub fn db_tools_details(&self) -> terrars::ListRef<DataOdbAutonomousDatabase_DbToolsDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db_tools_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_version` after provisioning.\nOracle Database software version."]
    pub fn db_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_workload` after provisioning.\nDatabase workload."]
    pub fn db_workload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_workload", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-friendly database name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption_key_provider` after provisioning.\nEncryption key provider."]
    pub fn encryption_key_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_provider", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nUnique identifier of the Autonomous Database."]
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
        "Get a reference to the value of field `is_backup_retention_locked` after provisioning.\nWhether backup retention is locked."]
    pub fn is_backup_retention_locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_backup_retention_locked", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_local_data_guard_enabled` after provisioning.\nWhether local Data Guard is enabled."]
    pub fn is_local_data_guard_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_local_data_guard_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_mtls_connection_required` after provisioning.\nWhether mTLS is required."]
    pub fn is_mtls_connection_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_mtls_connection_required", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_refreshable_clone` after provisioning.\nWhether the database is a refreshable clone."]
    pub fn is_refreshable_clone(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_refreshable_clone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_id` after provisioning.\nARN of the AWS KMS encryption key."]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `license_model` after provisioning.\nOracle license model."]
    pub fn license_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `local_adg_auto_failover_max_data_loss_limit` after provisioning.\nMaximum automatic-failover data-loss limit, in seconds."]
    pub fn local_adg_auto_failover_max_data_loss_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_adg_auto_failover_max_data_loss_limit", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `long_term_backup_schedule` after provisioning.\nLong-term backup schedule."]
    pub fn long_term_backup_schedule(&self) -> terrars::ListRef<DataOdbAutonomousDatabase_LongTermBackupScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.long_term_backup_schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ncharacter_set` after provisioning.\nNational character set."]
    pub fn ncharacter_set(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ncharacter_set", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nOCI resource anchor name."]
    pub fn oci_resource_anchor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oci_url` after provisioning.\nOCI console URL."]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\nOracle Cloud Identifier."]
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

    #[doc = "Get a reference to the value of field `open_mode` after provisioning.\nDatabase open mode."]
    pub fn open_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.open_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_progress` after provisioning.\nProgress of the current operation."]
    pub fn percent_progress(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission_level` after provisioning.\nDatabase permission level."]
    pub fn permission_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission_level", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_endpoint` after provisioning.\nPrivate endpoint."]
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

    #[doc = "Get a reference to the value of field `refreshable_mode` after provisioning.\nRefreshable clone mode."]
    pub fn refreshable_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refreshable_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_pool_leader_id` after provisioning.\nResource-pool leader database ID."]
    pub fn resource_pool_leader_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_pool_leader_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_pool_summary` after provisioning.\nResource pool configuration."]
    pub fn resource_pool_summary(&self) -> terrars::ListRef<DataOdbAutonomousDatabase_ResourcePoolSummaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_pool_summary", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scheduled_operations` after provisioning.\nScheduled database start and stop times."]
    pub fn scheduled_operations(&self) -> terrars::ListRef<DataOdbAutonomousDatabase_ScheduledOperationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduled_operations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_console_url` after provisioning.\nOracle service console URL."]
    pub fn service_console_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_console_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_id` after provisioning.\nSource database or backup ID."]
    pub fn source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sql_web_developer_url` after provisioning.\nOracle SQL Developer Web URL."]
    pub fn sql_web_developer_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_web_developer_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `standby_allowlisted_ips` after provisioning.\nIP addresses allowed to access the standby database."]
    pub fn standby_allowlisted_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standby_allowlisted_ips", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `standby_allowlisted_ips_source` after provisioning.\nSource of standby allowlisted IPs."]
    pub fn standby_allowlisted_ips_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.standby_allowlisted_ips_source", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nCurrent database status."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\nAdditional status information."]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `time_of_auto_refresh_start` after provisioning.\nAutomatic refresh start time."]
    pub fn time_of_auto_refresh_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_of_auto_refresh_start", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecret {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecret {
    #[doc = "Set the field `external_id_type`.\n"]
    pub fn set_external_id_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_id_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iam_role_arn`.\n"]
    pub fn set_iam_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.iam_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_arn`.\n"]
    pub fn set_secret_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecret {
    type O = terrars::BlockAssignable<DataOdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecret>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecret {}

impl BuildDataOdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecret {
    pub fn build(self) -> DataOdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecret {
        DataOdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecret {
            external_id_type: core::default::Default::default(),
            iam_role_arn: core::default::Default::default(),
            secret_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecretRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecretRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecretRef {
        DataOdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecretRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecretRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `external_id_type` after provisioning.\n"]
    pub fn external_id_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id_type", self.base))
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbAutonomousDatabase_AdminPasswordSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_managed_aws_secret: std
    ::option
    ::Option<terrars::ListField<DataOdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecret>>,
}

impl DataOdbAutonomousDatabase_AdminPasswordSource {
    #[doc = "Set the field `customer_managed_aws_secret`.\n"]
    pub fn set_customer_managed_aws_secret(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataOdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecret>,
                    >,
    ) -> Self {
        self.customer_managed_aws_secret = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbAutonomousDatabase_AdminPasswordSource {
    type O = terrars::BlockAssignable<DataOdbAutonomousDatabase_AdminPasswordSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbAutonomousDatabase_AdminPasswordSource {}

impl BuildDataOdbAutonomousDatabase_AdminPasswordSource {
    pub fn build(self) -> DataOdbAutonomousDatabase_AdminPasswordSource {
        DataOdbAutonomousDatabase_AdminPasswordSource {
            customer_managed_aws_secret: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbAutonomousDatabase_AdminPasswordSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbAutonomousDatabase_AdminPasswordSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOdbAutonomousDatabase_AdminPasswordSourceRef {
        DataOdbAutonomousDatabase_AdminPasswordSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbAutonomousDatabase_AdminPasswordSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `customer_managed_aws_secret` after provisioning.\n"]
    pub fn customer_managed_aws_secret(
        &self,
    ) -> terrars::ListRef<DataOdbAutonomousDatabase_AdminPasswordSource_CustomerManagedAwsSecretRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_managed_aws_secret", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbAutonomousDatabase_CustomerContactsToSendToOci {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOdbAutonomousDatabase_CustomerContactsToSendToOci {
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbAutonomousDatabase_CustomerContactsToSendToOci {
    type O = terrars::BlockAssignable<DataOdbAutonomousDatabase_CustomerContactsToSendToOci>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbAutonomousDatabase_CustomerContactsToSendToOci {}

impl BuildDataOdbAutonomousDatabase_CustomerContactsToSendToOci {
    pub fn build(self) -> DataOdbAutonomousDatabase_CustomerContactsToSendToOci {
        DataOdbAutonomousDatabase_CustomerContactsToSendToOci { email: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataOdbAutonomousDatabase_CustomerContactsToSendToOciRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbAutonomousDatabase_CustomerContactsToSendToOciRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOdbAutonomousDatabase_CustomerContactsToSendToOciRef {
        DataOdbAutonomousDatabase_CustomerContactsToSendToOciRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbAutonomousDatabase_CustomerContactsToSendToOciRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbAutonomousDatabase_DbToolsDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_idle_time_in_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOdbAutonomousDatabase_DbToolsDetails {
    #[doc = "Set the field `compute_count`.\n"]
    pub fn set_compute_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.compute_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_enabled`.\n"]
    pub fn set_is_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_idle_time_in_minutes`.\n"]
    pub fn set_max_idle_time_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_idle_time_in_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbAutonomousDatabase_DbToolsDetails {
    type O = terrars::BlockAssignable<DataOdbAutonomousDatabase_DbToolsDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbAutonomousDatabase_DbToolsDetails {}

impl BuildDataOdbAutonomousDatabase_DbToolsDetails {
    pub fn build(self) -> DataOdbAutonomousDatabase_DbToolsDetails {
        DataOdbAutonomousDatabase_DbToolsDetails {
            compute_count: core::default::Default::default(),
            is_enabled: core::default::Default::default(),
            max_idle_time_in_minutes: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbAutonomousDatabase_DbToolsDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbAutonomousDatabase_DbToolsDetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOdbAutonomousDatabase_DbToolsDetailsRef {
        DataOdbAutonomousDatabase_DbToolsDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbAutonomousDatabase_DbToolsDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compute_count` after provisioning.\n"]
    pub fn compute_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_count", self.base))
    }

    #[doc = "Get a reference to the value of field `is_enabled` after provisioning.\n"]
    pub fn is_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `max_idle_time_in_minutes` after provisioning.\n"]
    pub fn max_idle_time_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_idle_time_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbAutonomousDatabase_LongTermBackupSchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repeat_cadence: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_period_in_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_of_backup: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOdbAutonomousDatabase_LongTermBackupSchedule {
    #[doc = "Set the field `is_disabled`.\n"]
    pub fn set_is_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repeat_cadence`.\n"]
    pub fn set_repeat_cadence(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repeat_cadence = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention_period_in_days`.\n"]
    pub fn set_retention_period_in_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retention_period_in_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_of_backup`.\n"]
    pub fn set_time_of_backup(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_of_backup = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbAutonomousDatabase_LongTermBackupSchedule {
    type O = terrars::BlockAssignable<DataOdbAutonomousDatabase_LongTermBackupSchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbAutonomousDatabase_LongTermBackupSchedule {}

impl BuildDataOdbAutonomousDatabase_LongTermBackupSchedule {
    pub fn build(self) -> DataOdbAutonomousDatabase_LongTermBackupSchedule {
        DataOdbAutonomousDatabase_LongTermBackupSchedule {
            is_disabled: core::default::Default::default(),
            repeat_cadence: core::default::Default::default(),
            retention_period_in_days: core::default::Default::default(),
            time_of_backup: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbAutonomousDatabase_LongTermBackupScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbAutonomousDatabase_LongTermBackupScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOdbAutonomousDatabase_LongTermBackupScheduleRef {
        DataOdbAutonomousDatabase_LongTermBackupScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbAutonomousDatabase_LongTermBackupScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_disabled` after provisioning.\n"]
    pub fn is_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `repeat_cadence` after provisioning.\n"]
    pub fn repeat_cadence(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repeat_cadence", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_period_in_days` after provisioning.\n"]
    pub fn retention_period_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_period_in_days", self.base))
    }

    #[doc = "Get a reference to the value of field `time_of_backup` after provisioning.\n"]
    pub fn time_of_backup(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_of_backup", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbAutonomousDatabase_ResourcePoolSummary {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_compute_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_storage_capacity_in_tbs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_storage_size_in_tbs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_compute_capacity: std::option::Option<terrars::PrimField<f64>>,
}

impl DataOdbAutonomousDatabase_ResourcePoolSummary {
    #[doc = "Set the field `available_compute_capacity`.\n"]
    pub fn set_available_compute_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.available_compute_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `available_storage_capacity_in_tbs`.\n"]
    pub fn set_available_storage_capacity_in_tbs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.available_storage_capacity_in_tbs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_disabled`.\n"]
    pub fn set_is_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pool_size`.\n"]
    pub fn set_pool_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pool_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pool_storage_size_in_tbs`.\n"]
    pub fn set_pool_storage_size_in_tbs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pool_storage_size_in_tbs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_compute_capacity`.\n"]
    pub fn set_total_compute_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_compute_capacity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbAutonomousDatabase_ResourcePoolSummary {
    type O = terrars::BlockAssignable<DataOdbAutonomousDatabase_ResourcePoolSummary>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbAutonomousDatabase_ResourcePoolSummary {}

impl BuildDataOdbAutonomousDatabase_ResourcePoolSummary {
    pub fn build(self) -> DataOdbAutonomousDatabase_ResourcePoolSummary {
        DataOdbAutonomousDatabase_ResourcePoolSummary {
            available_compute_capacity: core::default::Default::default(),
            available_storage_capacity_in_tbs: core::default::Default::default(),
            is_disabled: core::default::Default::default(),
            pool_size: core::default::Default::default(),
            pool_storage_size_in_tbs: core::default::Default::default(),
            total_compute_capacity: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbAutonomousDatabase_ResourcePoolSummaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbAutonomousDatabase_ResourcePoolSummaryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOdbAutonomousDatabase_ResourcePoolSummaryRef {
        DataOdbAutonomousDatabase_ResourcePoolSummaryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbAutonomousDatabase_ResourcePoolSummaryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `available_compute_capacity` after provisioning.\n"]
    pub fn available_compute_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_compute_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `available_storage_capacity_in_tbs` after provisioning.\n"]
    pub fn available_storage_capacity_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_storage_capacity_in_tbs", self.base))
    }

    #[doc = "Get a reference to the value of field `is_disabled` after provisioning.\n"]
    pub fn is_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `pool_size` after provisioning.\n"]
    pub fn pool_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_size", self.base))
    }

    #[doc = "Get a reference to the value of field `pool_storage_size_in_tbs` after provisioning.\n"]
    pub fn pool_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_storage_size_in_tbs", self.base))
    }

    #[doc = "Get a reference to the value of field `total_compute_capacity` after provisioning.\n"]
    pub fn total_compute_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_compute_capacity", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbAutonomousDatabase_ScheduledOperations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day_of_week: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduled_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduled_stop_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOdbAutonomousDatabase_ScheduledOperations {
    #[doc = "Set the field `day_of_week`.\n"]
    pub fn set_day_of_week(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.day_of_week = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scheduled_start_time`.\n"]
    pub fn set_scheduled_start_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.scheduled_start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scheduled_stop_time`.\n"]
    pub fn set_scheduled_stop_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.scheduled_stop_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbAutonomousDatabase_ScheduledOperations {
    type O = terrars::BlockAssignable<DataOdbAutonomousDatabase_ScheduledOperations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbAutonomousDatabase_ScheduledOperations {}

impl BuildDataOdbAutonomousDatabase_ScheduledOperations {
    pub fn build(self) -> DataOdbAutonomousDatabase_ScheduledOperations {
        DataOdbAutonomousDatabase_ScheduledOperations {
            day_of_week: core::default::Default::default(),
            scheduled_start_time: core::default::Default::default(),
            scheduled_stop_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbAutonomousDatabase_ScheduledOperationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbAutonomousDatabase_ScheduledOperationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOdbAutonomousDatabase_ScheduledOperationsRef {
        DataOdbAutonomousDatabase_ScheduledOperationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbAutonomousDatabase_ScheduledOperationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\n"]
    pub fn day_of_week(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduled_start_time` after provisioning.\n"]
    pub fn scheduled_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduled_start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduled_stop_time` after provisioning.\n"]
    pub fn scheduled_stop_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduled_stop_time", self.base))
    }
}
