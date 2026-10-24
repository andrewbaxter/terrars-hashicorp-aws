use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DbInstanceData {
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
    auto_minor_version_upgrade: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_retention_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_window: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_cert_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    character_set_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_tags_to_snapshot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_iam_instance_profile: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_owned_ip_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_subnet_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_automated_backups: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_iam_role_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled_cloudwatch_logs_exports: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    final_snapshot_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_database_authentication_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identifier_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_class: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    license_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_allocated_storage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multi_az: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nchar_character_set_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    option_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    performance_insights_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    performance_insights_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    performance_insights_retention_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publicly_accessible: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replicate_source_db: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_final_snapshot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_encrypted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timezone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blue_green_update: std::option::Option<std::vec::Vec<DbInstanceBlueGreenUpdateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_to_point_in_time: std::option::Option<std::vec::Vec<DbInstanceRestoreToPointInTimeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_import: std::option::Option<std::vec::Vec<DbInstanceS3ImportEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DbInstanceTimeoutsEl>,
    dynamic: DbInstanceDynamic,
}

struct DbInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DbInstanceData>,
}

#[derive(Clone)]
pub struct DbInstance(std::rc::Rc<DbInstance_>);

impl DbInstance {
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

    #[doc = "Set the field `auto_minor_version_upgrade`.\n"]
    pub fn set_auto_minor_version_upgrade(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_minor_version_upgrade = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().availability_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_retention_period`.\n"]
    pub fn set_backup_retention_period(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().backup_retention_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_window`.\n"]
    pub fn set_backup_window(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().backup_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ca_cert_identifier`.\n"]
    pub fn set_ca_cert_identifier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ca_cert_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `character_set_name`.\n"]
    pub fn set_character_set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().character_set_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `copy_tags_to_snapshot`.\n"]
    pub fn set_copy_tags_to_snapshot(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().copy_tags_to_snapshot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_iam_instance_profile`.\n"]
    pub fn set_custom_iam_instance_profile(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().custom_iam_instance_profile = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer_owned_ip_enabled`.\n"]
    pub fn set_customer_owned_ip_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().customer_owned_ip_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_name`.\n"]
    pub fn set_db_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().db_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_subnet_group_name`.\n"]
    pub fn set_db_subnet_group_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().db_subnet_group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete_automated_backups`.\n"]
    pub fn set_delete_automated_backups(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_automated_backups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_protection`.\n"]
    pub fn set_deletion_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain_iam_role_name`.\n"]
    pub fn set_domain_iam_role_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().domain_iam_role_name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `iam_database_authentication_enabled`.\n"]
    pub fn set_iam_database_authentication_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().iam_database_authentication_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identifier`.\n"]
    pub fn set_identifier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identifier_prefix`.\n"]
    pub fn set_identifier_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().identifier_prefix = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `license_model`.\n"]
    pub fn set_license_model(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().license_model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().maintenance_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_allocated_storage`.\n"]
    pub fn set_max_allocated_storage(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_allocated_storage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monitoring_interval`.\n"]
    pub fn set_monitoring_interval(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().monitoring_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monitoring_role_arn`.\n"]
    pub fn set_monitoring_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().monitoring_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `multi_az`.\n"]
    pub fn set_multi_az(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().multi_az = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nchar_character_set_name`.\n"]
    pub fn set_nchar_character_set_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().nchar_character_set_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_type`.\n"]
    pub fn set_network_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `option_group_name`.\n"]
    pub fn set_option_group_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().option_group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameter_group_name`.\n"]
    pub fn set_parameter_group_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().parameter_group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password`.\n"]
    pub fn set_password(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `performance_insights_enabled`.\n"]
    pub fn set_performance_insights_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().performance_insights_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `performance_insights_kms_key_id`.\n"]
    pub fn set_performance_insights_kms_key_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().performance_insights_kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `performance_insights_retention_period`.\n"]
    pub fn set_performance_insights_retention_period(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().performance_insights_retention_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `publicly_accessible`.\n"]
    pub fn set_publicly_accessible(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().publicly_accessible = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replica_mode`.\n"]
    pub fn set_replica_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().replica_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replicate_source_db`.\n"]
    pub fn set_replicate_source_db(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().replicate_source_db = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `storage_encrypted`.\n"]
    pub fn set_storage_encrypted(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().storage_encrypted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_throughput`.\n"]
    pub fn set_storage_throughput(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().storage_throughput = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `timezone`.\n"]
    pub fn set_timezone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().timezone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\n"]
    pub fn set_username(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().username = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `blue_green_update`.\n"]
    pub fn set_blue_green_update(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DbInstanceBlueGreenUpdateEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().blue_green_update = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.blue_green_update = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `restore_to_point_in_time`.\n"]
    pub fn set_restore_to_point_in_time(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DbInstanceRestoreToPointInTimeEl>>,
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
    pub fn set_s3_import(self, v: impl std::convert::Into<terrars::BlockAssignable<DbInstanceS3ImportEl>>) -> Self {
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DbInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `auto_minor_version_upgrade` after provisioning.\n"]
    pub fn auto_minor_version_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_minor_version_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_retention_period` after provisioning.\n"]
    pub fn backup_retention_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_window` after provisioning.\n"]
    pub fn backup_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ca_cert_identifier` after provisioning.\n"]
    pub fn ca_cert_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_cert_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `character_set_name` after provisioning.\n"]
    pub fn character_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.character_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `copy_tags_to_snapshot` after provisioning.\n"]
    pub fn copy_tags_to_snapshot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_tags_to_snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_iam_instance_profile` after provisioning.\n"]
    pub fn custom_iam_instance_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_iam_instance_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_owned_ip_enabled` after provisioning.\n"]
    pub fn customer_owned_ip_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_owned_ip_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_name` after provisioning.\n"]
    pub fn db_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_subnet_group_name` after provisioning.\n"]
    pub fn db_subnet_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_subnet_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_automated_backups` after provisioning.\n"]
    pub fn delete_automated_backups(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_automated_backups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_iam_role_name` after provisioning.\n"]
    pub fn domain_iam_role_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_iam_role_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identifier_prefix` after provisioning.\n"]
    pub fn identifier_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_class` after provisioning.\n"]
    pub fn instance_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iops", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `latest_restorable_time` after provisioning.\n"]
    pub fn latest_restorable_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_restorable_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `license_model` after provisioning.\n"]
    pub fn license_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_allocated_storage` after provisioning.\n"]
    pub fn max_allocated_storage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_allocated_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_interval` after provisioning.\n"]
    pub fn monitoring_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_role_arn` after provisioning.\n"]
    pub fn monitoring_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_az` after provisioning.\n"]
    pub fn multi_az(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multi_az", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nchar_character_set_name` after provisioning.\n"]
    pub fn nchar_character_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nchar_character_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_type` after provisioning.\n"]
    pub fn network_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `option_group_name` after provisioning.\n"]
    pub fn option_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.option_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameter_group_name` after provisioning.\n"]
    pub fn parameter_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameter_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `performance_insights_enabled` after provisioning.\n"]
    pub fn performance_insights_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.performance_insights_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `performance_insights_kms_key_id` after provisioning.\n"]
    pub fn performance_insights_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.performance_insights_kms_key_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `performance_insights_retention_period` after provisioning.\n"]
    pub fn performance_insights_retention_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.performance_insights_retention_period", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publicly_accessible", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replica_mode` after provisioning.\n"]
    pub fn replica_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replica_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replicas` after provisioning.\n"]
    pub fn replicas(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replicas", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replicate_source_db` after provisioning.\n"]
    pub fn replicate_source_db(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replicate_source_db", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_names` after provisioning.\n"]
    pub fn security_group_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_final_snapshot` after provisioning.\n"]
    pub fn skip_final_snapshot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_final_snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_identifier` after provisioning.\n"]
    pub fn snapshot_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_encrypted` after provisioning.\n"]
    pub fn storage_encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_throughput` after provisioning.\n"]
    pub fn storage_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_throughput", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blue_green_update` after provisioning.\n"]
    pub fn blue_green_update(&self) -> terrars::ListRef<DbInstanceBlueGreenUpdateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blue_green_update", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_to_point_in_time` after provisioning.\n"]
    pub fn restore_to_point_in_time(&self) -> terrars::ListRef<DbInstanceRestoreToPointInTimeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_to_point_in_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_import` after provisioning.\n"]
    pub fn s3_import(&self) -> terrars::ListRef<DbInstanceS3ImportElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_import", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DbInstanceTimeoutsElRef {
        DbInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DbInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DbInstance { }

impl terrars::ToListMappable for DbInstance {
    type O = terrars::ListRef<DbInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DbInstance_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_db_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDbInstance {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub instance_class: terrars::PrimField<std::string::String>,
}

impl BuildDbInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> DbInstance {
        let out = DbInstance(std::rc::Rc::new(DbInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DbInstanceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allocated_storage: core::default::Default::default(),
                allow_major_version_upgrade: core::default::Default::default(),
                apply_immediately: core::default::Default::default(),
                auto_minor_version_upgrade: core::default::Default::default(),
                availability_zone: core::default::Default::default(),
                backup_retention_period: core::default::Default::default(),
                backup_window: core::default::Default::default(),
                ca_cert_identifier: core::default::Default::default(),
                character_set_name: core::default::Default::default(),
                copy_tags_to_snapshot: core::default::Default::default(),
                custom_iam_instance_profile: core::default::Default::default(),
                customer_owned_ip_enabled: core::default::Default::default(),
                db_name: core::default::Default::default(),
                db_subnet_group_name: core::default::Default::default(),
                delete_automated_backups: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                domain: core::default::Default::default(),
                domain_iam_role_name: core::default::Default::default(),
                enabled_cloudwatch_logs_exports: core::default::Default::default(),
                engine: core::default::Default::default(),
                engine_version: core::default::Default::default(),
                final_snapshot_identifier: core::default::Default::default(),
                iam_database_authentication_enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                identifier: core::default::Default::default(),
                identifier_prefix: core::default::Default::default(),
                instance_class: self.instance_class,
                iops: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                license_model: core::default::Default::default(),
                maintenance_window: core::default::Default::default(),
                max_allocated_storage: core::default::Default::default(),
                monitoring_interval: core::default::Default::default(),
                monitoring_role_arn: core::default::Default::default(),
                multi_az: core::default::Default::default(),
                name: core::default::Default::default(),
                nchar_character_set_name: core::default::Default::default(),
                network_type: core::default::Default::default(),
                option_group_name: core::default::Default::default(),
                parameter_group_name: core::default::Default::default(),
                password: core::default::Default::default(),
                performance_insights_enabled: core::default::Default::default(),
                performance_insights_kms_key_id: core::default::Default::default(),
                performance_insights_retention_period: core::default::Default::default(),
                port: core::default::Default::default(),
                publicly_accessible: core::default::Default::default(),
                replica_mode: core::default::Default::default(),
                replicate_source_db: core::default::Default::default(),
                security_group_names: core::default::Default::default(),
                skip_final_snapshot: core::default::Default::default(),
                snapshot_identifier: core::default::Default::default(),
                storage_encrypted: core::default::Default::default(),
                storage_throughput: core::default::Default::default(),
                storage_type: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timezone: core::default::Default::default(),
                username: core::default::Default::default(),
                vpc_security_group_ids: core::default::Default::default(),
                blue_green_update: core::default::Default::default(),
                restore_to_point_in_time: core::default::Default::default(),
                s3_import: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DbInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DbInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DbInstanceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `auto_minor_version_upgrade` after provisioning.\n"]
    pub fn auto_minor_version_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_minor_version_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_retention_period` after provisioning.\n"]
    pub fn backup_retention_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_window` after provisioning.\n"]
    pub fn backup_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ca_cert_identifier` after provisioning.\n"]
    pub fn ca_cert_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_cert_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `character_set_name` after provisioning.\n"]
    pub fn character_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.character_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `copy_tags_to_snapshot` after provisioning.\n"]
    pub fn copy_tags_to_snapshot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_tags_to_snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_iam_instance_profile` after provisioning.\n"]
    pub fn custom_iam_instance_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_iam_instance_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_owned_ip_enabled` after provisioning.\n"]
    pub fn customer_owned_ip_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_owned_ip_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_name` after provisioning.\n"]
    pub fn db_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_subnet_group_name` after provisioning.\n"]
    pub fn db_subnet_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_subnet_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_automated_backups` after provisioning.\n"]
    pub fn delete_automated_backups(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_automated_backups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_iam_role_name` after provisioning.\n"]
    pub fn domain_iam_role_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_iam_role_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identifier_prefix` after provisioning.\n"]
    pub fn identifier_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_class` after provisioning.\n"]
    pub fn instance_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iops", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `latest_restorable_time` after provisioning.\n"]
    pub fn latest_restorable_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_restorable_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `license_model` after provisioning.\n"]
    pub fn license_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_allocated_storage` after provisioning.\n"]
    pub fn max_allocated_storage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_allocated_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_interval` after provisioning.\n"]
    pub fn monitoring_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_role_arn` after provisioning.\n"]
    pub fn monitoring_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_az` after provisioning.\n"]
    pub fn multi_az(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multi_az", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nchar_character_set_name` after provisioning.\n"]
    pub fn nchar_character_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nchar_character_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_type` after provisioning.\n"]
    pub fn network_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `option_group_name` after provisioning.\n"]
    pub fn option_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.option_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameter_group_name` after provisioning.\n"]
    pub fn parameter_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameter_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `performance_insights_enabled` after provisioning.\n"]
    pub fn performance_insights_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.performance_insights_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `performance_insights_kms_key_id` after provisioning.\n"]
    pub fn performance_insights_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.performance_insights_kms_key_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `performance_insights_retention_period` after provisioning.\n"]
    pub fn performance_insights_retention_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.performance_insights_retention_period", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publicly_accessible", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replica_mode` after provisioning.\n"]
    pub fn replica_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replica_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replicas` after provisioning.\n"]
    pub fn replicas(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replicas", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replicate_source_db` after provisioning.\n"]
    pub fn replicate_source_db(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replicate_source_db", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_names` after provisioning.\n"]
    pub fn security_group_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_final_snapshot` after provisioning.\n"]
    pub fn skip_final_snapshot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_final_snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_identifier` after provisioning.\n"]
    pub fn snapshot_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_encrypted` after provisioning.\n"]
    pub fn storage_encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_throughput` after provisioning.\n"]
    pub fn storage_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_throughput", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blue_green_update` after provisioning.\n"]
    pub fn blue_green_update(&self) -> terrars::ListRef<DbInstanceBlueGreenUpdateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blue_green_update", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_to_point_in_time` after provisioning.\n"]
    pub fn restore_to_point_in_time(&self) -> terrars::ListRef<DbInstanceRestoreToPointInTimeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_to_point_in_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_import` after provisioning.\n"]
    pub fn s3_import(&self) -> terrars::ListRef<DbInstanceS3ImportElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_import", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DbInstanceTimeoutsElRef {
        DbInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DbInstanceBlueGreenUpdateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DbInstanceBlueGreenUpdateEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DbInstanceBlueGreenUpdateEl {
    type O = terrars::BlockAssignable<DbInstanceBlueGreenUpdateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDbInstanceBlueGreenUpdateEl {}

impl BuildDbInstanceBlueGreenUpdateEl {
    pub fn build(self) -> DbInstanceBlueGreenUpdateEl {
        DbInstanceBlueGreenUpdateEl { enabled: core::default::Default::default() }
    }
}

pub struct DbInstanceBlueGreenUpdateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DbInstanceBlueGreenUpdateElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DbInstanceBlueGreenUpdateElRef {
        DbInstanceBlueGreenUpdateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DbInstanceBlueGreenUpdateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DbInstanceRestoreToPointInTimeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_db_instance_automated_backups_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_db_instance_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_dbi_resource_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_latest_restorable_time: std::option::Option<terrars::PrimField<bool>>,
}

impl DbInstanceRestoreToPointInTimeEl {
    #[doc = "Set the field `restore_time`.\n"]
    pub fn set_restore_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.restore_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_db_instance_automated_backups_arn`.\n"]
    pub fn set_source_db_instance_automated_backups_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_db_instance_automated_backups_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_db_instance_identifier`.\n"]
    pub fn set_source_db_instance_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_db_instance_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_dbi_resource_id`.\n"]
    pub fn set_source_dbi_resource_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_dbi_resource_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_latest_restorable_time`.\n"]
    pub fn set_use_latest_restorable_time(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_latest_restorable_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DbInstanceRestoreToPointInTimeEl {
    type O = terrars::BlockAssignable<DbInstanceRestoreToPointInTimeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDbInstanceRestoreToPointInTimeEl {}

impl BuildDbInstanceRestoreToPointInTimeEl {
    pub fn build(self) -> DbInstanceRestoreToPointInTimeEl {
        DbInstanceRestoreToPointInTimeEl {
            restore_time: core::default::Default::default(),
            source_db_instance_automated_backups_arn: core::default::Default::default(),
            source_db_instance_identifier: core::default::Default::default(),
            source_dbi_resource_id: core::default::Default::default(),
            use_latest_restorable_time: core::default::Default::default(),
        }
    }
}

pub struct DbInstanceRestoreToPointInTimeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DbInstanceRestoreToPointInTimeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DbInstanceRestoreToPointInTimeElRef {
        DbInstanceRestoreToPointInTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DbInstanceRestoreToPointInTimeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `restore_time` after provisioning.\n"]
    pub fn restore_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restore_time", self.base))
    }

    #[doc = "Get a reference to the value of field `source_db_instance_automated_backups_arn` after provisioning.\n"]
    pub fn source_db_instance_automated_backups_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_db_instance_automated_backups_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `source_db_instance_identifier` after provisioning.\n"]
    pub fn source_db_instance_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_db_instance_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `source_dbi_resource_id` after provisioning.\n"]
    pub fn source_dbi_resource_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_dbi_resource_id", self.base))
    }

    #[doc = "Get a reference to the value of field `use_latest_restorable_time` after provisioning.\n"]
    pub fn use_latest_restorable_time(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_latest_restorable_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DbInstanceS3ImportEl {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    ingestion_role: terrars::PrimField<std::string::String>,
    source_engine: terrars::PrimField<std::string::String>,
    source_engine_version: terrars::PrimField<std::string::String>,
}

impl DbInstanceS3ImportEl {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DbInstanceS3ImportEl {
    type O = terrars::BlockAssignable<DbInstanceS3ImportEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDbInstanceS3ImportEl {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub ingestion_role: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source_engine: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source_engine_version: terrars::PrimField<std::string::String>,
}

impl BuildDbInstanceS3ImportEl {
    pub fn build(self) -> DbInstanceS3ImportEl {
        DbInstanceS3ImportEl {
            bucket_name: self.bucket_name,
            bucket_prefix: core::default::Default::default(),
            ingestion_role: self.ingestion_role,
            source_engine: self.source_engine,
            source_engine_version: self.source_engine_version,
        }
    }
}

pub struct DbInstanceS3ImportElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DbInstanceS3ImportElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DbInstanceS3ImportElRef {
        DbInstanceS3ImportElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DbInstanceS3ImportElRef {
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
pub struct DbInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DbInstanceTimeoutsEl {
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

impl terrars::ToListMappable for DbInstanceTimeoutsEl {
    type O = terrars::BlockAssignable<DbInstanceTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDbInstanceTimeoutsEl {}

impl BuildDbInstanceTimeoutsEl {
    pub fn build(self) -> DbInstanceTimeoutsEl {
        DbInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DbInstanceTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DbInstanceTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DbInstanceTimeoutsElRef {
        DbInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DbInstanceTimeoutsElRef {
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
struct DbInstanceDynamic {
    blue_green_update: std::option::Option<terrars::DynamicBlock<DbInstanceBlueGreenUpdateEl>>,
    restore_to_point_in_time: std::option::Option<terrars::DynamicBlock<DbInstanceRestoreToPointInTimeEl>>,
    s3_import: std::option::Option<terrars::DynamicBlock<DbInstanceS3ImportEl>>,
}
