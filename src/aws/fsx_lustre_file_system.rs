#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FsxLustreFileSystemData_ {
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
    auto_import_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_backup_retention_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_tags_to_backups: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    daily_automatic_backup_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_compression_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drive_cache_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    efa_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_type_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    final_backup_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    import_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    imported_file_chunk_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_unit_storage_throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_final_backup: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_type: std::option::Option<terrars::PrimField<std::string::String>>,
    subnet_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_maintenance_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_read_cache_configuration: std::option::Option<std::vec::Vec<FsxLustreFileSystem_DataReadCacheConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_configuration: std::option::Option<std::vec::Vec<FsxLustreFileSystem_LogConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_configuration: std::option::Option<std::vec::Vec<FsxLustreFileSystem_MetadataConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_squash_configuration: std::option::Option<std::vec::Vec<FsxLustreFileSystem_RootSquashConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FsxLustreFileSystem_Timeouts>,
    dynamic: FsxLustreFileSystemDynamic,
}

struct FsxLustreFileSystem_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FsxLustreFileSystemData_>,
}

#[derive(Clone)]
pub struct FsxLustreFileSystem(std::rc::Rc<FsxLustreFileSystem_>);

impl FsxLustreFileSystem {
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

    #[doc = "Set the field `auto_import_policy`.\n"]
    pub fn set_auto_import_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().auto_import_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `automatic_backup_retention_days`.\n"]
    pub fn set_automatic_backup_retention_days(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().automatic_backup_retention_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_id`.\n"]
    pub fn set_backup_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().backup_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `copy_tags_to_backups`.\n"]
    pub fn set_copy_tags_to_backups(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().copy_tags_to_backups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `daily_automatic_backup_start_time`.\n"]
    pub fn set_daily_automatic_backup_start_time(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().daily_automatic_backup_start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_compression_type`.\n"]
    pub fn set_data_compression_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().data_compression_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_type`.\n"]
    pub fn set_deployment_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deployment_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `drive_cache_type`.\n"]
    pub fn set_drive_cache_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().drive_cache_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `efa_enabled`.\n"]
    pub fn set_efa_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().efa_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `export_path`.\n"]
    pub fn set_export_path(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().export_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_system_type_version`.\n"]
    pub fn set_file_system_type_version(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().file_system_type_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `final_backup_tags`.\n"]
    pub fn set_final_backup_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().final_backup_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `import_path`.\n"]
    pub fn set_import_path(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().import_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `imported_file_chunk_size`.\n"]
    pub fn set_imported_file_chunk_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().imported_file_chunk_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `per_unit_storage_throughput`.\n"]
    pub fn set_per_unit_storage_throughput(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().per_unit_storage_throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `skip_final_backup`.\n"]
    pub fn set_skip_final_backup(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_final_backup = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_capacity`.\n"]
    pub fn set_storage_capacity(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().storage_capacity = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `throughput_capacity`.\n"]
    pub fn set_throughput_capacity(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().throughput_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weekly_maintenance_start_time`.\n"]
    pub fn set_weekly_maintenance_start_time(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().weekly_maintenance_start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_read_cache_configuration`.\n"]
    pub fn set_data_read_cache_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxLustreFileSystem_DataReadCacheConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_read_cache_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_read_cache_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_configuration`.\n"]
    pub fn set_log_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxLustreFileSystem_LogConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metadata_configuration`.\n"]
    pub fn set_metadata_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxLustreFileSystem_MetadataConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metadata_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metadata_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `root_squash_configuration`.\n"]
    pub fn set_root_squash_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxLustreFileSystem_RootSquashConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().root_squash_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.root_squash_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<FsxLustreFileSystem_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_import_policy` after provisioning.\n"]
    pub fn auto_import_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_import_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automatic_backup_retention_days` after provisioning.\n"]
    pub fn automatic_backup_retention_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.automatic_backup_retention_days", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `backup_id` after provisioning.\n"]
    pub fn backup_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `copy_tags_to_backups` after provisioning.\n"]
    pub fn copy_tags_to_backups(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_tags_to_backups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `daily_automatic_backup_start_time` after provisioning.\n"]
    pub fn daily_automatic_backup_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.daily_automatic_backup_start_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_compression_type` after provisioning.\n"]
    pub fn data_compression_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_compression_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\n"]
    pub fn deployment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `drive_cache_type` after provisioning.\n"]
    pub fn drive_cache_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drive_cache_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `efa_enabled` after provisioning.\n"]
    pub fn efa_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.efa_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `export_path` after provisioning.\n"]
    pub fn export_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_type_version` after provisioning.\n"]
    pub fn file_system_type_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_type_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `final_backup_tags` after provisioning.\n"]
    pub fn final_backup_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.final_backup_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `import_path` after provisioning.\n"]
    pub fn import_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.import_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `imported_file_chunk_size` after provisioning.\n"]
    pub fn imported_file_chunk_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.imported_file_chunk_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mount_name` after provisioning.\n"]
    pub fn mount_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface_ids` after provisioning.\n"]
    pub fn network_interface_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `per_unit_storage_throughput` after provisioning.\n"]
    pub fn per_unit_storage_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_unit_storage_throughput", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_final_backup` after provisioning.\n"]
    pub fn skip_final_backup(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_final_backup", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_capacity` after provisioning.\n"]
    pub fn storage_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `throughput_capacity` after provisioning.\n"]
    pub fn throughput_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `weekly_maintenance_start_time` after provisioning.\n"]
    pub fn weekly_maintenance_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.weekly_maintenance_start_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_read_cache_configuration` after provisioning.\n"]
    pub fn data_read_cache_configuration(&self) -> terrars::ListRef<FsxLustreFileSystem_DataReadCacheConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_read_cache_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(&self) -> terrars::ListRef<FsxLustreFileSystem_LogConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_configuration` after provisioning.\n"]
    pub fn metadata_configuration(&self) -> terrars::ListRef<FsxLustreFileSystem_MetadataConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_squash_configuration` after provisioning.\n"]
    pub fn root_squash_configuration(&self) -> terrars::ListRef<FsxLustreFileSystem_RootSquashConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_squash_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxLustreFileSystem_TimeoutsRef {
        FsxLustreFileSystem_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for FsxLustreFileSystem {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FsxLustreFileSystem { }

impl terrars::ToListMappable for FsxLustreFileSystem {
    type O = terrars::ListRef<FsxLustreFileSystemRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FsxLustreFileSystem_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_fsx_lustre_file_system".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFsxLustreFileSystem {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub subnet_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildFsxLustreFileSystem {
    pub fn build(self, stack: &mut terrars::Stack) -> FsxLustreFileSystem {
        let out = FsxLustreFileSystem(std::rc::Rc::new(FsxLustreFileSystem_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FsxLustreFileSystemData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                auto_import_policy: core::default::Default::default(),
                automatic_backup_retention_days: core::default::Default::default(),
                backup_id: core::default::Default::default(),
                copy_tags_to_backups: core::default::Default::default(),
                daily_automatic_backup_start_time: core::default::Default::default(),
                data_compression_type: core::default::Default::default(),
                deployment_type: core::default::Default::default(),
                drive_cache_type: core::default::Default::default(),
                efa_enabled: core::default::Default::default(),
                export_path: core::default::Default::default(),
                file_system_type_version: core::default::Default::default(),
                final_backup_tags: core::default::Default::default(),
                id: core::default::Default::default(),
                import_path: core::default::Default::default(),
                imported_file_chunk_size: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                per_unit_storage_throughput: core::default::Default::default(),
                region: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                skip_final_backup: core::default::Default::default(),
                storage_capacity: core::default::Default::default(),
                storage_type: core::default::Default::default(),
                subnet_ids: self.subnet_ids,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                throughput_capacity: core::default::Default::default(),
                weekly_maintenance_start_time: core::default::Default::default(),
                data_read_cache_configuration: core::default::Default::default(),
                log_configuration: core::default::Default::default(),
                metadata_configuration: core::default::Default::default(),
                root_squash_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FsxLustreFileSystemRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxLustreFileSystemRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FsxLustreFileSystemRef {
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

    #[doc = "Get a reference to the value of field `auto_import_policy` after provisioning.\n"]
    pub fn auto_import_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_import_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automatic_backup_retention_days` after provisioning.\n"]
    pub fn automatic_backup_retention_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.automatic_backup_retention_days", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `backup_id` after provisioning.\n"]
    pub fn backup_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `copy_tags_to_backups` after provisioning.\n"]
    pub fn copy_tags_to_backups(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_tags_to_backups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `daily_automatic_backup_start_time` after provisioning.\n"]
    pub fn daily_automatic_backup_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.daily_automatic_backup_start_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_compression_type` after provisioning.\n"]
    pub fn data_compression_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_compression_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\n"]
    pub fn deployment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `drive_cache_type` after provisioning.\n"]
    pub fn drive_cache_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drive_cache_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `efa_enabled` after provisioning.\n"]
    pub fn efa_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.efa_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `export_path` after provisioning.\n"]
    pub fn export_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_type_version` after provisioning.\n"]
    pub fn file_system_type_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_type_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `final_backup_tags` after provisioning.\n"]
    pub fn final_backup_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.final_backup_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `import_path` after provisioning.\n"]
    pub fn import_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.import_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `imported_file_chunk_size` after provisioning.\n"]
    pub fn imported_file_chunk_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.imported_file_chunk_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mount_name` after provisioning.\n"]
    pub fn mount_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface_ids` after provisioning.\n"]
    pub fn network_interface_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `per_unit_storage_throughput` after provisioning.\n"]
    pub fn per_unit_storage_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_unit_storage_throughput", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_final_backup` after provisioning.\n"]
    pub fn skip_final_backup(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_final_backup", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_capacity` after provisioning.\n"]
    pub fn storage_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `throughput_capacity` after provisioning.\n"]
    pub fn throughput_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `weekly_maintenance_start_time` after provisioning.\n"]
    pub fn weekly_maintenance_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.weekly_maintenance_start_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_read_cache_configuration` after provisioning.\n"]
    pub fn data_read_cache_configuration(&self) -> terrars::ListRef<FsxLustreFileSystem_DataReadCacheConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_read_cache_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(&self) -> terrars::ListRef<FsxLustreFileSystem_LogConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_configuration` after provisioning.\n"]
    pub fn metadata_configuration(&self) -> terrars::ListRef<FsxLustreFileSystem_MetadataConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_squash_configuration` after provisioning.\n"]
    pub fn root_squash_configuration(&self) -> terrars::ListRef<FsxLustreFileSystem_RootSquashConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_squash_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxLustreFileSystem_TimeoutsRef {
        FsxLustreFileSystem_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct FsxLustreFileSystemDynamic {
    data_read_cache_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<FsxLustreFileSystem_DataReadCacheConfiguration>>,
    log_configuration: std::option::Option<terrars::DynamicBlock<FsxLustreFileSystem_LogConfiguration>>,
    metadata_configuration: std::option::Option<terrars::DynamicBlock<FsxLustreFileSystem_MetadataConfiguration>>,
    root_squash_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<FsxLustreFileSystem_RootSquashConfiguration>>,
}

#[derive(Serialize)]
pub struct FsxLustreFileSystem_DataReadCacheConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<f64>>,
    sizing_mode: terrars::PrimField<std::string::String>,
}

impl FsxLustreFileSystem_DataReadCacheConfiguration {
    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxLustreFileSystem_DataReadCacheConfiguration {
    type O = terrars::BlockAssignable<FsxLustreFileSystem_DataReadCacheConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxLustreFileSystem_DataReadCacheConfiguration {
    #[doc = ""]
    pub sizing_mode: terrars::PrimField<std::string::String>,
}

impl BuildFsxLustreFileSystem_DataReadCacheConfiguration {
    pub fn build(self) -> FsxLustreFileSystem_DataReadCacheConfiguration {
        FsxLustreFileSystem_DataReadCacheConfiguration {
            size: core::default::Default::default(),
            sizing_mode: self.sizing_mode,
        }
    }
}

#[allow(dead_code)]
pub struct FsxLustreFileSystem_DataReadCacheConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxLustreFileSystem_DataReadCacheConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxLustreFileSystem_DataReadCacheConfigurationRef {
        FsxLustreFileSystem_DataReadCacheConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxLustreFileSystem_DataReadCacheConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `sizing_mode` after provisioning.\n"]
    pub fn sizing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sizing_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxLustreFileSystem_LogConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FsxLustreFileSystem_LogConfiguration {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `level`.\n"]
    pub fn set_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxLustreFileSystem_LogConfiguration {
    type O = terrars::BlockAssignable<FsxLustreFileSystem_LogConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxLustreFileSystem_LogConfiguration {}

impl BuildFsxLustreFileSystem_LogConfiguration {
    pub fn build(self) -> FsxLustreFileSystem_LogConfiguration {
        FsxLustreFileSystem_LogConfiguration {
            destination: core::default::Default::default(),
            level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxLustreFileSystem_LogConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxLustreFileSystem_LogConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxLustreFileSystem_LogConfigurationRef {
        FsxLustreFileSystem_LogConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxLustreFileSystem_LogConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `level` after provisioning.\n"]
    pub fn level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.level", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxLustreFileSystem_MetadataConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FsxLustreFileSystem_MetadataConfiguration {
    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxLustreFileSystem_MetadataConfiguration {
    type O = terrars::BlockAssignable<FsxLustreFileSystem_MetadataConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxLustreFileSystem_MetadataConfiguration {}

impl BuildFsxLustreFileSystem_MetadataConfiguration {
    pub fn build(self) -> FsxLustreFileSystem_MetadataConfiguration {
        FsxLustreFileSystem_MetadataConfiguration {
            iops: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxLustreFileSystem_MetadataConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxLustreFileSystem_MetadataConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxLustreFileSystem_MetadataConfigurationRef {
        FsxLustreFileSystem_MetadataConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxLustreFileSystem_MetadataConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxLustreFileSystem_RootSquashConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_squash_nids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_squash: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FsxLustreFileSystem_RootSquashConfiguration {
    #[doc = "Set the field `no_squash_nids`.\n"]
    pub fn set_no_squash_nids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.no_squash_nids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `root_squash`.\n"]
    pub fn set_root_squash(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.root_squash = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxLustreFileSystem_RootSquashConfiguration {
    type O = terrars::BlockAssignable<FsxLustreFileSystem_RootSquashConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxLustreFileSystem_RootSquashConfiguration {}

impl BuildFsxLustreFileSystem_RootSquashConfiguration {
    pub fn build(self) -> FsxLustreFileSystem_RootSquashConfiguration {
        FsxLustreFileSystem_RootSquashConfiguration {
            no_squash_nids: core::default::Default::default(),
            root_squash: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxLustreFileSystem_RootSquashConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxLustreFileSystem_RootSquashConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxLustreFileSystem_RootSquashConfigurationRef {
        FsxLustreFileSystem_RootSquashConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxLustreFileSystem_RootSquashConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `no_squash_nids` after provisioning.\n"]
    pub fn no_squash_nids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.no_squash_nids", self.base))
    }

    #[doc = "Get a reference to the value of field `root_squash` after provisioning.\n"]
    pub fn root_squash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_squash", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxLustreFileSystem_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FsxLustreFileSystem_Timeouts {
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

impl terrars::ToListMappable for FsxLustreFileSystem_Timeouts {
    type O = terrars::BlockAssignable<FsxLustreFileSystem_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxLustreFileSystem_Timeouts {}

impl BuildFsxLustreFileSystem_Timeouts {
    pub fn build(self) -> FsxLustreFileSystem_Timeouts {
        FsxLustreFileSystem_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxLustreFileSystem_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxLustreFileSystem_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxLustreFileSystem_TimeoutsRef {
        FsxLustreFileSystem_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxLustreFileSystem_TimeoutsRef {
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
