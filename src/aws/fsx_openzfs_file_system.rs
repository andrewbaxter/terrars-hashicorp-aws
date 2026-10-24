use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FsxOpenzfsFileSystemData {
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
    deployment_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
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
    disk_iops_configuration: std::option::Option<std::vec::Vec<FsxOpenzfsFileSystemDiskIopsConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_volume_configuration: std::option::Option<std::vec::Vec<FsxOpenzfsFileSystemRootVolumeConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FsxOpenzfsFileSystemTimeoutsEl>,
    dynamic: FsxOpenzfsFileSystemDynamic,
}

struct FsxOpenzfsFileSystem_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FsxOpenzfsFileSystemData>,
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

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().security_group_ids = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<FsxOpenzfsFileSystemDiskIopsConfigurationEl>>,
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

    #[doc = "Set the field `root_volume_configuration`.\n"]
    pub fn set_root_volume_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxOpenzfsFileSystemRootVolumeConfigurationEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<FsxOpenzfsFileSystemTimeoutsEl>) -> Self {
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

    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\n"]
    pub fn deployment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_volume_id` after provisioning.\n"]
    pub fn root_volume_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_volume_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
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
    pub fn disk_iops_configuration(&self) -> terrars::ListRef<FsxOpenzfsFileSystemDiskIopsConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_iops_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_volume_configuration` after provisioning.\n"]
    pub fn root_volume_configuration(&self) -> terrars::ListRef<FsxOpenzfsFileSystemRootVolumeConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_volume_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOpenzfsFileSystemTimeoutsElRef {
        FsxOpenzfsFileSystemTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(FsxOpenzfsFileSystemData {
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
                deployment_type: self.deployment_type,
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                storage_capacity: core::default::Default::default(),
                storage_type: core::default::Default::default(),
                subnet_ids: self.subnet_ids,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                throughput_capacity: self.throughput_capacity,
                weekly_maintenance_start_time: core::default::Default::default(),
                disk_iops_configuration: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\n"]
    pub fn deployment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_volume_id` after provisioning.\n"]
    pub fn root_volume_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_volume_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
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
    pub fn disk_iops_configuration(&self) -> terrars::ListRef<FsxOpenzfsFileSystemDiskIopsConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_iops_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_volume_configuration` after provisioning.\n"]
    pub fn root_volume_configuration(&self) -> terrars::ListRef<FsxOpenzfsFileSystemRootVolumeConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_volume_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOpenzfsFileSystemTimeoutsElRef {
        FsxOpenzfsFileSystemTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FsxOpenzfsFileSystemDiskIopsConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FsxOpenzfsFileSystemDiskIopsConfigurationEl {
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

impl terrars::ToListMappable for FsxOpenzfsFileSystemDiskIopsConfigurationEl {
    type O = terrars::BlockAssignable<FsxOpenzfsFileSystemDiskIopsConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsFileSystemDiskIopsConfigurationEl {}

impl BuildFsxOpenzfsFileSystemDiskIopsConfigurationEl {
    pub fn build(self) -> FsxOpenzfsFileSystemDiskIopsConfigurationEl {
        FsxOpenzfsFileSystemDiskIopsConfigurationEl {
            iops: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}

pub struct FsxOpenzfsFileSystemDiskIopsConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOpenzfsFileSystemDiskIopsConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOpenzfsFileSystemDiskIopsConfigurationElRef {
        FsxOpenzfsFileSystemDiskIopsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsFileSystemDiskIopsConfigurationElRef {
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
pub struct FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl {
    clients: terrars::PrimField<std::string::String>,
    options: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl { }

impl terrars::ToListMappable for FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl {
    type O =
        terrars::BlockAssignable<FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl {
    #[doc = ""]
    pub clients: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub options: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildFsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl {
    pub fn build(self) -> FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl {
        FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl {
            clients: self.clients,
            options: self.options,
        }
    }
}

pub struct FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsElRef {
        FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsElRef {
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
struct FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElDynamic {
    client_configurations: std
    ::option
    ::Option<terrars::DynamicBlock<FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl>>,
}

#[derive(Serialize)]
pub struct FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_configurations: std
    ::option
    ::Option<std::vec::Vec<FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl>>,
    dynamic: FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElDynamic,
}

impl FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl {
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
                            FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElClientConfigurationsEl,
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

impl terrars::ToListMappable for FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl {
    type O = terrars::BlockAssignable<FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl {}

impl BuildFsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl {
    pub fn build(self) -> FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl {
        FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl {
            client_configurations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElRef {
        FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl {
    id: terrars::PrimField<f64>,
    storage_capacity_quota_gib: terrars::PrimField<f64>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl { }

impl terrars::ToListMappable for FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl {
    type O = terrars::BlockAssignable<FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl {
    #[doc = ""]
    pub id: terrars::PrimField<f64>,
    #[doc = ""]
    pub storage_capacity_quota_gib: terrars::PrimField<f64>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildFsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl {
    pub fn build(self) -> FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl {
        FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl {
            id: self.id,
            storage_capacity_quota_gib: self.storage_capacity_quota_gib,
            type_: self.type_,
        }
    }
}

pub struct FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasElRef {
        FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasElRef {
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
struct FsxOpenzfsFileSystemRootVolumeConfigurationElDynamic {
    nfs_exports: std
    ::option
    ::Option<terrars::DynamicBlock<FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl>>,
    user_and_group_quotas: std
    ::option
    ::Option<terrars::DynamicBlock<FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl>>,
}

#[derive(Serialize)]
pub struct FsxOpenzfsFileSystemRootVolumeConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_tags_to_snapshots: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_compression_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_size_kib: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nfs_exports: std::option::Option<std::vec::Vec<FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_and_group_quotas: std
    ::option
    ::Option<std::vec::Vec<FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl>>,
    dynamic: FsxOpenzfsFileSystemRootVolumeConfigurationElDynamic,
}

impl FsxOpenzfsFileSystemRootVolumeConfigurationEl {
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
                    ::Into<terrars::BlockAssignable<FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsEl>>,
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
                    ::Into<
                        terrars::BlockAssignable<FsxOpenzfsFileSystemRootVolumeConfigurationElUserAndGroupQuotasEl>,
                    >,
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

impl terrars::ToListMappable for FsxOpenzfsFileSystemRootVolumeConfigurationEl {
    type O = terrars::BlockAssignable<FsxOpenzfsFileSystemRootVolumeConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsFileSystemRootVolumeConfigurationEl {}

impl BuildFsxOpenzfsFileSystemRootVolumeConfigurationEl {
    pub fn build(self) -> FsxOpenzfsFileSystemRootVolumeConfigurationEl {
        FsxOpenzfsFileSystemRootVolumeConfigurationEl {
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

pub struct FsxOpenzfsFileSystemRootVolumeConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOpenzfsFileSystemRootVolumeConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOpenzfsFileSystemRootVolumeConfigurationElRef {
        FsxOpenzfsFileSystemRootVolumeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsFileSystemRootVolumeConfigurationElRef {
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
    pub fn nfs_exports(&self) -> terrars::ListRef<FsxOpenzfsFileSystemRootVolumeConfigurationElNfsExportsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs_exports", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOpenzfsFileSystemTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FsxOpenzfsFileSystemTimeoutsEl {
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

impl terrars::ToListMappable for FsxOpenzfsFileSystemTimeoutsEl {
    type O = terrars::BlockAssignable<FsxOpenzfsFileSystemTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsFileSystemTimeoutsEl {}

impl BuildFsxOpenzfsFileSystemTimeoutsEl {
    pub fn build(self) -> FsxOpenzfsFileSystemTimeoutsEl {
        FsxOpenzfsFileSystemTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct FsxOpenzfsFileSystemTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOpenzfsFileSystemTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxOpenzfsFileSystemTimeoutsElRef {
        FsxOpenzfsFileSystemTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsFileSystemTimeoutsElRef {
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
struct FsxOpenzfsFileSystemDynamic {
    disk_iops_configuration: std::option::Option<terrars::DynamicBlock<FsxOpenzfsFileSystemDiskIopsConfigurationEl>>,
    root_volume_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<FsxOpenzfsFileSystemRootVolumeConfigurationEl>>,
}
