#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FsxOpenzfsFileSystemData_ {
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
    automatic_backup_retention_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_tags_to_backups: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_tags_to_volumes: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    daily_automatic_backup_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    deployment_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_ip_address_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    final_backup_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_table_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
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
    throughput_capacity: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_maintenance_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_iops_configuration: std::option::Option<std::vec::Vec<FsxOpenzfsFileSystem_DiskIopsConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_cache_configuration: std::option::Option<std::vec::Vec<FsxOpenzfsFileSystem_ReadCacheConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_volume_configuration: std::option::Option<std::vec::Vec<FsxOpenzfsFileSystem_RootVolumeConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FsxOpenzfsFileSystem_Timeouts>,
    dynamic: FsxOpenzfsFileSystemDynamic,
}

struct FsxOpenzfsFileSystem_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FsxOpenzfsFileSystemData_>,
}

#[derive(Clone)]
pub struct FsxOpenzfsFileSystem(std::rc::Rc<FsxOpenzfsFileSystem_>);

impl FsxOpenzfsFileSystem {
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

    #[doc = "Set the field `copy_tags_to_volumes`.\n"]
    pub fn set_copy_tags_to_volumes(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().copy_tags_to_volumes = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `delete_options`.\n"]
    pub fn set_delete_options(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().delete_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_ip_address_range`.\n"]
    pub fn set_endpoint_ip_address_range(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().endpoint_ip_address_range = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_type`.\n"]
    pub fn set_network_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_subnet_id`.\n"]
    pub fn set_preferred_subnet_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().preferred_subnet_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_table_ids`.\n"]
    pub fn set_route_table_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().route_table_ids = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `weekly_maintenance_start_time`.\n"]
    pub fn set_weekly_maintenance_start_time(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().weekly_maintenance_start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_iops_configuration`.\n"]
    pub fn set_disk_iops_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxOpenzfsFileSystem_DiskIopsConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().disk_iops_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.disk_iops_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `read_cache_configuration`.\n"]
    pub fn set_read_cache_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxOpenzfsFileSystem_ReadCacheConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().read_cache_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.read_cache_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `root_volume_configuration`.\n"]
    pub fn set_root_volume_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxOpenzfsFileSystem_RootVolumeConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().root_volume_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.root_volume_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<FsxOpenzfsFileSystem_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `copy_tags_to_volumes` after provisioning.\n"]
    pub fn copy_tags_to_volumes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_tags_to_volumes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `daily_automatic_backup_start_time` after provisioning.\n"]
    pub fn daily_automatic_backup_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.daily_automatic_backup_start_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delete_options` after provisioning.\n"]
    pub fn delete_options(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.delete_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\n"]
    pub fn deployment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_ip_address` after provisioning.\n"]
    pub fn endpoint_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_ip_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_ip_address_range` after provisioning.\n"]
    pub fn endpoint_ip_address_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_ip_address_range", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `final_backup_tags` after provisioning.\n"]
    pub fn final_backup_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.final_backup_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface_ids` after provisioning.\n"]
    pub fn network_interface_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_type` after provisioning.\n"]
    pub fn network_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_subnet_id` after provisioning.\n"]
    pub fn preferred_subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_subnet_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_volume_id` after provisioning.\n"]
    pub fn root_volume_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_volume_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_table_ids` after provisioning.\n"]
    pub fn route_table_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.route_table_ids", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `disk_iops_configuration` after provisioning.\n"]
    pub fn disk_iops_configuration(&self) -> terrars::ListRef<FsxOpenzfsFileSystem_DiskIopsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_iops_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_cache_configuration` after provisioning.\n"]
    pub fn read_cache_configuration(&self) -> terrars::ListRef<FsxOpenzfsFileSystem_ReadCacheConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.read_cache_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_volume_configuration` after provisioning.\n"]
    pub fn root_volume_configuration(&self) -> terrars::ListRef<FsxOpenzfsFileSystem_RootVolumeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_volume_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOpenzfsFileSystem_TimeoutsRef {
        FsxOpenzfsFileSystem_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for FsxOpenzfsFileSystem {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FsxOpenzfsFileSystem { }

impl terrars::ToListMappable for FsxOpenzfsFileSystem {
    type O = terrars::ListRef<FsxOpenzfsFileSystemRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FsxOpenzfsFileSystem_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_fsx_openzfs_file_system".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFsxOpenzfsFileSystem {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub deployment_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub subnet_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub throughput_capacity: terrars::PrimField<f64>,
}

impl BuildFsxOpenzfsFileSystem {
    pub fn build(self, stack: &mut terrars::Stack) -> FsxOpenzfsFileSystem {
        let out = FsxOpenzfsFileSystem(std::rc::Rc::new(FsxOpenzfsFileSystem_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FsxOpenzfsFileSystemData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                automatic_backup_retention_days: core::default::Default::default(),
                backup_id: core::default::Default::default(),
                copy_tags_to_backups: core::default::Default::default(),
                copy_tags_to_volumes: core::default::Default::default(),
                daily_automatic_backup_start_time: core::default::Default::default(),
                delete_options: core::default::Default::default(),
                deployment_type: self.deployment_type,
                endpoint_ip_address_range: core::default::Default::default(),
                final_backup_tags: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                network_type: core::default::Default::default(),
                preferred_subnet_id: core::default::Default::default(),
                region: core::default::Default::default(),
                route_table_ids: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                skip_final_backup: core::default::Default::default(),
                storage_capacity: core::default::Default::default(),
                storage_type: core::default::Default::default(),
                subnet_ids: self.subnet_ids,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                throughput_capacity: self.throughput_capacity,
                weekly_maintenance_start_time: core::default::Default::default(),
                disk_iops_configuration: core::default::Default::default(),
                read_cache_configuration: core::default::Default::default(),
                root_volume_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FsxOpenzfsFileSystemRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOpenzfsFileSystemRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FsxOpenzfsFileSystemRef {
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

    #[doc = "Get a reference to the value of field `copy_tags_to_volumes` after provisioning.\n"]
    pub fn copy_tags_to_volumes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_tags_to_volumes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `daily_automatic_backup_start_time` after provisioning.\n"]
    pub fn daily_automatic_backup_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.daily_automatic_backup_start_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delete_options` after provisioning.\n"]
    pub fn delete_options(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.delete_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\n"]
    pub fn deployment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_ip_address` after provisioning.\n"]
    pub fn endpoint_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_ip_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_ip_address_range` after provisioning.\n"]
    pub fn endpoint_ip_address_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_ip_address_range", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `final_backup_tags` after provisioning.\n"]
    pub fn final_backup_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.final_backup_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface_ids` after provisioning.\n"]
    pub fn network_interface_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_type` after provisioning.\n"]
    pub fn network_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_subnet_id` after provisioning.\n"]
    pub fn preferred_subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_subnet_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_volume_id` after provisioning.\n"]
    pub fn root_volume_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_volume_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_table_ids` after provisioning.\n"]
    pub fn route_table_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.route_table_ids", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `disk_iops_configuration` after provisioning.\n"]
    pub fn disk_iops_configuration(&self) -> terrars::ListRef<FsxOpenzfsFileSystem_DiskIopsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_iops_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_cache_configuration` after provisioning.\n"]
    pub fn read_cache_configuration(&self) -> terrars::ListRef<FsxOpenzfsFileSystem_ReadCacheConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.read_cache_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_volume_configuration` after provisioning.\n"]
    pub fn root_volume_configuration(&self) -> terrars::ListRef<FsxOpenzfsFileSystem_RootVolumeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_volume_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOpenzfsFileSystem_TimeoutsRef {
        FsxOpenzfsFileSystem_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct FsxOpenzfsFileSystemDynamic {
    disk_iops_configuration: std::option::Option<terrars::DynamicBlock<FsxOpenzfsFileSystem_DiskIopsConfiguration>>,
    read_cache_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<FsxOpenzfsFileSystem_ReadCacheConfiguration>>,
    root_volume_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<FsxOpenzfsFileSystem_RootVolumeConfiguration>>,
}

#[derive(Serialize)]
pub struct FsxOpenzfsFileSystem_DiskIopsConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FsxOpenzfsFileSystem_DiskIopsConfiguration {
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

impl terrars::ToListMappable for FsxOpenzfsFileSystem_DiskIopsConfiguration {
    type O = terrars::BlockAssignable<FsxOpenzfsFileSystem_DiskIopsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsFileSystem_DiskIopsConfiguration {}

impl BuildFsxOpenzfsFileSystem_DiskIopsConfiguration {
    pub fn build(self) -> FsxOpenzfsFileSystem_DiskIopsConfiguration {
        FsxOpenzfsFileSystem_DiskIopsConfiguration {
            iops: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOpenzfsFileSystem_DiskIopsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOpenzfsFileSystem_DiskIopsConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxOpenzfsFileSystem_DiskIopsConfigurationRef {
        FsxOpenzfsFileSystem_DiskIopsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsFileSystem_DiskIopsConfigurationRef {
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
pub struct FsxOpenzfsFileSystem_ReadCacheConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sizing_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FsxOpenzfsFileSystem_ReadCacheConfiguration {
    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sizing_mode`.\n"]
    pub fn set_sizing_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sizing_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxOpenzfsFileSystem_ReadCacheConfiguration {
    type O = terrars::BlockAssignable<FsxOpenzfsFileSystem_ReadCacheConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsFileSystem_ReadCacheConfiguration {}

impl BuildFsxOpenzfsFileSystem_ReadCacheConfiguration {
    pub fn build(self) -> FsxOpenzfsFileSystem_ReadCacheConfiguration {
        FsxOpenzfsFileSystem_ReadCacheConfiguration {
            size: core::default::Default::default(),
            sizing_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOpenzfsFileSystem_ReadCacheConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOpenzfsFileSystem_ReadCacheConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOpenzfsFileSystem_ReadCacheConfigurationRef {
        FsxOpenzfsFileSystem_ReadCacheConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsFileSystem_ReadCacheConfigurationRef {
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
pub struct FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports_ClientConfigurations {
    clients: terrars::PrimField<std::string::String>,
    options: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports_ClientConfigurations { }

impl terrars::ToListMappable for FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports_ClientConfigurations {
    type O = terrars::BlockAssignable<FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports_ClientConfigurations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports_ClientConfigurations {
    #[doc = ""]
    pub clients: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub options: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildFsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports_ClientConfigurations {
    pub fn build(self) -> FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports_ClientConfigurations {
        FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports_ClientConfigurations {
            clients: self.clients,
            options: self.options,
        }
    }
}

#[allow(dead_code)]
pub struct FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports_ClientConfigurationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports_ClientConfigurationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports_ClientConfigurationsRef {
        FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports_ClientConfigurationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports_ClientConfigurationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `clients` after provisioning.\n"]
    pub fn clients(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.clients", self.base))
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.options", self.base))
    }
}

#[derive(Serialize, Default)]
struct FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExportsDynamic {
    client_configurations: std
    ::option
    ::Option<terrars::DynamicBlock<FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports_ClientConfigurations>>,
}

#[derive(Serialize)]
pub struct FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_configurations: std
    ::option
    ::Option<std::vec::Vec<FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports_ClientConfigurations>>,
    dynamic: FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExportsDynamic,
}

impl FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports {
    #[doc = "Set the field `client_configurations`.\n"]
    pub fn set_client_configurations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports_ClientConfigurations,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_configurations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_configurations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports {
    type O = terrars::BlockAssignable<FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports {}

impl BuildFsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports {
    pub fn build(self) -> FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports {
        FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports {
            client_configurations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExportsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExportsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExportsRef {
        FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExportsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExportsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct FsxOpenzfsFileSystem_RootVolumeConfiguration_UserAndGroupQuotas {
    id: terrars::PrimField<f64>,
    storage_capacity_quota_gib: terrars::PrimField<f64>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl FsxOpenzfsFileSystem_RootVolumeConfiguration_UserAndGroupQuotas { }

impl terrars::ToListMappable for FsxOpenzfsFileSystem_RootVolumeConfiguration_UserAndGroupQuotas {
    type O = terrars::BlockAssignable<FsxOpenzfsFileSystem_RootVolumeConfiguration_UserAndGroupQuotas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsFileSystem_RootVolumeConfiguration_UserAndGroupQuotas {
    #[doc = ""]
    pub id: terrars::PrimField<f64>,
    #[doc = ""]
    pub storage_capacity_quota_gib: terrars::PrimField<f64>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildFsxOpenzfsFileSystem_RootVolumeConfiguration_UserAndGroupQuotas {
    pub fn build(self) -> FsxOpenzfsFileSystem_RootVolumeConfiguration_UserAndGroupQuotas {
        FsxOpenzfsFileSystem_RootVolumeConfiguration_UserAndGroupQuotas {
            id: self.id,
            storage_capacity_quota_gib: self.storage_capacity_quota_gib,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct FsxOpenzfsFileSystem_RootVolumeConfiguration_UserAndGroupQuotasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOpenzfsFileSystem_RootVolumeConfiguration_UserAndGroupQuotasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOpenzfsFileSystem_RootVolumeConfiguration_UserAndGroupQuotasRef {
        FsxOpenzfsFileSystem_RootVolumeConfiguration_UserAndGroupQuotasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsFileSystem_RootVolumeConfiguration_UserAndGroupQuotasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_capacity_quota_gib` after provisioning.\n"]
    pub fn storage_capacity_quota_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_capacity_quota_gib", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct FsxOpenzfsFileSystem_RootVolumeConfigurationDynamic {
    nfs_exports: std::option::Option<terrars::DynamicBlock<FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports>>,
    user_and_group_quotas: std
    ::option
    ::Option<terrars::DynamicBlock<FsxOpenzfsFileSystem_RootVolumeConfiguration_UserAndGroupQuotas>>,
}

#[derive(Serialize)]
pub struct FsxOpenzfsFileSystem_RootVolumeConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_tags_to_snapshots: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_compression_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_size_kib: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nfs_exports: std::option::Option<std::vec::Vec<FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_and_group_quotas: std
    ::option
    ::Option<std::vec::Vec<FsxOpenzfsFileSystem_RootVolumeConfiguration_UserAndGroupQuotas>>,
    dynamic: FsxOpenzfsFileSystem_RootVolumeConfigurationDynamic,
}

impl FsxOpenzfsFileSystem_RootVolumeConfiguration {
    #[doc = "Set the field `copy_tags_to_snapshots`.\n"]
    pub fn set_copy_tags_to_snapshots(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.copy_tags_to_snapshots = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_compression_type`.\n"]
    pub fn set_data_compression_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.data_compression_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_only`.\n"]
    pub fn set_read_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.read_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_size_kib`.\n"]
    pub fn set_record_size_kib(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.record_size_kib = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nfs_exports`.\n"]
    pub fn set_nfs_exports(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExports>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.nfs_exports = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.nfs_exports = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_and_group_quotas`.\n"]
    pub fn set_user_and_group_quotas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<FsxOpenzfsFileSystem_RootVolumeConfiguration_UserAndGroupQuotas>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user_and_group_quotas = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user_and_group_quotas = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FsxOpenzfsFileSystem_RootVolumeConfiguration {
    type O = terrars::BlockAssignable<FsxOpenzfsFileSystem_RootVolumeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsFileSystem_RootVolumeConfiguration {}

impl BuildFsxOpenzfsFileSystem_RootVolumeConfiguration {
    pub fn build(self) -> FsxOpenzfsFileSystem_RootVolumeConfiguration {
        FsxOpenzfsFileSystem_RootVolumeConfiguration {
            copy_tags_to_snapshots: core::default::Default::default(),
            data_compression_type: core::default::Default::default(),
            read_only: core::default::Default::default(),
            record_size_kib: core::default::Default::default(),
            nfs_exports: core::default::Default::default(),
            user_and_group_quotas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOpenzfsFileSystem_RootVolumeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOpenzfsFileSystem_RootVolumeConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOpenzfsFileSystem_RootVolumeConfigurationRef {
        FsxOpenzfsFileSystem_RootVolumeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsFileSystem_RootVolumeConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `copy_tags_to_snapshots` after provisioning.\n"]
    pub fn copy_tags_to_snapshots(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_tags_to_snapshots", self.base))
    }

    #[doc = "Get a reference to the value of field `data_compression_type` after provisioning.\n"]
    pub fn data_compression_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_compression_type", self.base))
    }

    #[doc = "Get a reference to the value of field `read_only` after provisioning.\n"]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }

    #[doc = "Get a reference to the value of field `record_size_kib` after provisioning.\n"]
    pub fn record_size_kib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_size_kib", self.base))
    }

    #[doc = "Get a reference to the value of field `nfs_exports` after provisioning.\n"]
    pub fn nfs_exports(&self) -> terrars::ListRef<FsxOpenzfsFileSystem_RootVolumeConfiguration_NfsExportsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs_exports", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOpenzfsFileSystem_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FsxOpenzfsFileSystem_Timeouts {
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

impl terrars::ToListMappable for FsxOpenzfsFileSystem_Timeouts {
    type O = terrars::BlockAssignable<FsxOpenzfsFileSystem_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsFileSystem_Timeouts {}

impl BuildFsxOpenzfsFileSystem_Timeouts {
    pub fn build(self) -> FsxOpenzfsFileSystem_Timeouts {
        FsxOpenzfsFileSystem_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOpenzfsFileSystem_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOpenzfsFileSystem_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxOpenzfsFileSystem_TimeoutsRef {
        FsxOpenzfsFileSystem_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsFileSystem_TimeoutsRef {
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
