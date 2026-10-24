use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FsxWindowsFileSystemData {
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
    active_directory_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aliases: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_backup_retention_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_tags_to_backups: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    daily_automatic_backup_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
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
    audit_log_configuration: std::option::Option<std::vec::Vec<FsxWindowsFileSystemAuditLogConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_managed_active_directory: std::option::Option<std::vec::Vec<FsxWindowsFileSystemSelfManagedActiveDirectoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FsxWindowsFileSystemTimeoutsEl>,
    dynamic: FsxWindowsFileSystemDynamic,
}

struct FsxWindowsFileSystem_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FsxWindowsFileSystemData>,
}

#[derive(Clone)]
pub struct FsxWindowsFileSystem(std::rc::Rc<FsxWindowsFileSystem_>);

impl FsxWindowsFileSystem {
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

    #[doc = "Set the field `active_directory_id`.\n"]
    pub fn set_active_directory_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().active_directory_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aliases`.\n"]
    pub fn set_aliases(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().aliases = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `deployment_type`.\n"]
    pub fn set_deployment_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deployment_type = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `preferred_subnet_id`.\n"]
    pub fn set_preferred_subnet_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().preferred_subnet_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `audit_log_configuration`.\n"]
    pub fn set_audit_log_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxWindowsFileSystemAuditLogConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().audit_log_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.audit_log_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `self_managed_active_directory`.\n"]
    pub fn set_self_managed_active_directory(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxWindowsFileSystemSelfManagedActiveDirectoryEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().self_managed_active_directory = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.self_managed_active_directory = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<FsxWindowsFileSystemTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `active_directory_id` after provisioning.\n"]
    pub fn active_directory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aliases` after provisioning.\n"]
    pub fn aliases(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.aliases", self.extract_ref()))
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
    pub fn network_interface_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.network_interface_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_file_server_ip` after provisioning.\n"]
    pub fn preferred_file_server_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_file_server_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_subnet_id` after provisioning.\n"]
    pub fn preferred_subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_administration_endpoint` after provisioning.\n"]
    pub fn remote_administration_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.remote_administration_endpoint", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `audit_log_configuration` after provisioning.\n"]
    pub fn audit_log_configuration(&self) -> terrars::ListRef<FsxWindowsFileSystemAuditLogConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audit_log_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_managed_active_directory` after provisioning.\n"]
    pub fn self_managed_active_directory(&self) -> terrars::ListRef<FsxWindowsFileSystemSelfManagedActiveDirectoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_managed_active_directory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxWindowsFileSystemTimeoutsElRef {
        FsxWindowsFileSystemTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for FsxWindowsFileSystem {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FsxWindowsFileSystem { }

impl terrars::ToListMappable for FsxWindowsFileSystem {
    type O = terrars::ListRef<FsxWindowsFileSystemRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FsxWindowsFileSystem_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_fsx_windows_file_system".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFsxWindowsFileSystem {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub subnet_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub throughput_capacity: terrars::PrimField<f64>,
}

impl BuildFsxWindowsFileSystem {
    pub fn build(self, stack: &mut terrars::Stack) -> FsxWindowsFileSystem {
        let out = FsxWindowsFileSystem(std::rc::Rc::new(FsxWindowsFileSystem_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FsxWindowsFileSystemData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                active_directory_id: core::default::Default::default(),
                aliases: core::default::Default::default(),
                automatic_backup_retention_days: core::default::Default::default(),
                backup_id: core::default::Default::default(),
                copy_tags_to_backups: core::default::Default::default(),
                daily_automatic_backup_start_time: core::default::Default::default(),
                deployment_type: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                preferred_subnet_id: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                skip_final_backup: core::default::Default::default(),
                storage_capacity: core::default::Default::default(),
                storage_type: core::default::Default::default(),
                subnet_ids: self.subnet_ids,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                throughput_capacity: self.throughput_capacity,
                weekly_maintenance_start_time: core::default::Default::default(),
                audit_log_configuration: core::default::Default::default(),
                self_managed_active_directory: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FsxWindowsFileSystemRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxWindowsFileSystemRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FsxWindowsFileSystemRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active_directory_id` after provisioning.\n"]
    pub fn active_directory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aliases` after provisioning.\n"]
    pub fn aliases(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.aliases", self.extract_ref()))
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
    pub fn network_interface_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.network_interface_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_file_server_ip` after provisioning.\n"]
    pub fn preferred_file_server_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_file_server_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_subnet_id` after provisioning.\n"]
    pub fn preferred_subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_administration_endpoint` after provisioning.\n"]
    pub fn remote_administration_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.remote_administration_endpoint", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `audit_log_configuration` after provisioning.\n"]
    pub fn audit_log_configuration(&self) -> terrars::ListRef<FsxWindowsFileSystemAuditLogConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audit_log_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_managed_active_directory` after provisioning.\n"]
    pub fn self_managed_active_directory(&self) -> terrars::ListRef<FsxWindowsFileSystemSelfManagedActiveDirectoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_managed_active_directory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxWindowsFileSystemTimeoutsElRef {
        FsxWindowsFileSystemTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FsxWindowsFileSystemAuditLogConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audit_log_destination: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_access_audit_log_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_share_access_audit_log_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FsxWindowsFileSystemAuditLogConfigurationEl {
    #[doc = "Set the field `audit_log_destination`.\n"]
    pub fn set_audit_log_destination(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.audit_log_destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_access_audit_log_level`.\n"]
    pub fn set_file_access_audit_log_level(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.file_access_audit_log_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_share_access_audit_log_level`.\n"]
    pub fn set_file_share_access_audit_log_level(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.file_share_access_audit_log_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxWindowsFileSystemAuditLogConfigurationEl {
    type O = terrars::BlockAssignable<FsxWindowsFileSystemAuditLogConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxWindowsFileSystemAuditLogConfigurationEl {}

impl BuildFsxWindowsFileSystemAuditLogConfigurationEl {
    pub fn build(self) -> FsxWindowsFileSystemAuditLogConfigurationEl {
        FsxWindowsFileSystemAuditLogConfigurationEl {
            audit_log_destination: core::default::Default::default(),
            file_access_audit_log_level: core::default::Default::default(),
            file_share_access_audit_log_level: core::default::Default::default(),
        }
    }
}

pub struct FsxWindowsFileSystemAuditLogConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxWindowsFileSystemAuditLogConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxWindowsFileSystemAuditLogConfigurationElRef {
        FsxWindowsFileSystemAuditLogConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxWindowsFileSystemAuditLogConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audit_log_destination` after provisioning.\n"]
    pub fn audit_log_destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audit_log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `file_access_audit_log_level` after provisioning.\n"]
    pub fn file_access_audit_log_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_access_audit_log_level", self.base))
    }

    #[doc = "Get a reference to the value of field `file_share_access_audit_log_level` after provisioning.\n"]
    pub fn file_share_access_audit_log_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_share_access_audit_log_level", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxWindowsFileSystemSelfManagedActiveDirectoryEl {
    dns_ips: terrars::SetField<terrars::PrimField<std::string::String>>,
    domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_administrators_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organizational_unit_distinguished_name: std::option::Option<terrars::PrimField<std::string::String>>,
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl FsxWindowsFileSystemSelfManagedActiveDirectoryEl {
    #[doc = "Set the field `file_system_administrators_group`.\n"]
    pub fn set_file_system_administrators_group(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.file_system_administrators_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `organizational_unit_distinguished_name`.\n"]
    pub fn set_organizational_unit_distinguished_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.organizational_unit_distinguished_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxWindowsFileSystemSelfManagedActiveDirectoryEl {
    type O = terrars::BlockAssignable<FsxWindowsFileSystemSelfManagedActiveDirectoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxWindowsFileSystemSelfManagedActiveDirectoryEl {
    #[doc = ""]
    pub dns_ips: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildFsxWindowsFileSystemSelfManagedActiveDirectoryEl {
    pub fn build(self) -> FsxWindowsFileSystemSelfManagedActiveDirectoryEl {
        FsxWindowsFileSystemSelfManagedActiveDirectoryEl {
            dns_ips: self.dns_ips,
            domain_name: self.domain_name,
            file_system_administrators_group: core::default::Default::default(),
            organizational_unit_distinguished_name: core::default::Default::default(),
            password: self.password,
            username: self.username,
        }
    }
}

pub struct FsxWindowsFileSystemSelfManagedActiveDirectoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxWindowsFileSystemSelfManagedActiveDirectoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxWindowsFileSystemSelfManagedActiveDirectoryElRef {
        FsxWindowsFileSystemSelfManagedActiveDirectoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxWindowsFileSystemSelfManagedActiveDirectoryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_ips` after provisioning.\n"]
    pub fn dns_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dns_ips", self.base))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_administrators_group` after provisioning.\n"]
    pub fn file_system_administrators_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_administrators_group", self.base))
    }

    #[doc = "Get a reference to the value of field `organizational_unit_distinguished_name` after provisioning.\n"]
    pub fn organizational_unit_distinguished_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.organizational_unit_distinguished_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxWindowsFileSystemTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FsxWindowsFileSystemTimeoutsEl {
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

impl terrars::ToListMappable for FsxWindowsFileSystemTimeoutsEl {
    type O = terrars::BlockAssignable<FsxWindowsFileSystemTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxWindowsFileSystemTimeoutsEl {}

impl BuildFsxWindowsFileSystemTimeoutsEl {
    pub fn build(self) -> FsxWindowsFileSystemTimeoutsEl {
        FsxWindowsFileSystemTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct FsxWindowsFileSystemTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxWindowsFileSystemTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxWindowsFileSystemTimeoutsElRef {
        FsxWindowsFileSystemTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxWindowsFileSystemTimeoutsElRef {
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
struct FsxWindowsFileSystemDynamic {
    audit_log_configuration: std::option::Option<terrars::DynamicBlock<FsxWindowsFileSystemAuditLogConfigurationEl>>,
    self_managed_active_directory: std
    ::option
    ::Option<terrars::DynamicBlock<FsxWindowsFileSystemSelfManagedActiveDirectoryEl>>,
}
