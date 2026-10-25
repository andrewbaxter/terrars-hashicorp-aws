#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FsxOntapVolumeData_ {
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
    bypass_snaplock_enterprise_retention: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_tags_to_backups: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    final_backup_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    junction_path: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ontap_volume_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_style: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_in_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_in_megabytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_final_backup: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_efficiency_enabled: std::option::Option<terrars::PrimField<bool>>,
    storage_virtual_machine_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_style: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aggregate_configuration: std::option::Option<std::vec::Vec<FsxOntapVolume_AggregateConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snaplock_configuration: std::option::Option<std::vec::Vec<FsxOntapVolume_SnaplockConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tiering_policy: std::option::Option<std::vec::Vec<FsxOntapVolume_TieringPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FsxOntapVolume_Timeouts>,
    dynamic: FsxOntapVolumeDynamic,
}

struct FsxOntapVolume_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FsxOntapVolumeData_>,
}

#[derive(Clone)]
pub struct FsxOntapVolume(std::rc::Rc<FsxOntapVolume_>);

impl FsxOntapVolume {
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

    #[doc = "Set the field `bypass_snaplock_enterprise_retention`.\n"]
    pub fn set_bypass_snaplock_enterprise_retention(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().bypass_snaplock_enterprise_retention = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `copy_tags_to_backups`.\n"]
    pub fn set_copy_tags_to_backups(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().copy_tags_to_backups = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `junction_path`.\n"]
    pub fn set_junction_path(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().junction_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ontap_volume_type`.\n"]
    pub fn set_ontap_volume_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ontap_volume_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_style`.\n"]
    pub fn set_security_style(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().security_style = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size_in_bytes`.\n"]
    pub fn set_size_in_bytes(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().size_in_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size_in_megabytes`.\n"]
    pub fn set_size_in_megabytes(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().size_in_megabytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `skip_final_backup`.\n"]
    pub fn set_skip_final_backup(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_final_backup = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_policy`.\n"]
    pub fn set_snapshot_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().snapshot_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_efficiency_enabled`.\n"]
    pub fn set_storage_efficiency_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().storage_efficiency_enabled = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `volume_style`.\n"]
    pub fn set_volume_style(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().volume_style = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().volume_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aggregate_configuration`.\n"]
    pub fn set_aggregate_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxOntapVolume_AggregateConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().aggregate_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.aggregate_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `snaplock_configuration`.\n"]
    pub fn set_snaplock_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxOntapVolume_SnaplockConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().snaplock_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.snaplock_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tiering_policy`.\n"]
    pub fn set_tiering_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxOntapVolume_TieringPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tiering_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tiering_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<FsxOntapVolume_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bypass_snaplock_enterprise_retention` after provisioning.\n"]
    pub fn bypass_snaplock_enterprise_retention(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bypass_snaplock_enterprise_retention", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `copy_tags_to_backups` after provisioning.\n"]
    pub fn copy_tags_to_backups(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_tags_to_backups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `final_backup_tags` after provisioning.\n"]
    pub fn final_backup_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.final_backup_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `flexcache_endpoint_type` after provisioning.\n"]
    pub fn flexcache_endpoint_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flexcache_endpoint_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `junction_path` after provisioning.\n"]
    pub fn junction_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.junction_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ontap_volume_type` after provisioning.\n"]
    pub fn ontap_volume_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ontap_volume_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_style` after provisioning.\n"]
    pub fn security_style(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_style", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size_in_bytes` after provisioning.\n"]
    pub fn size_in_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_in_bytes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size_in_megabytes` after provisioning.\n"]
    pub fn size_in_megabytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_in_megabytes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_final_backup` after provisioning.\n"]
    pub fn skip_final_backup(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_final_backup", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_policy` after provisioning.\n"]
    pub fn snapshot_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_efficiency_enabled` after provisioning.\n"]
    pub fn storage_efficiency_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_efficiency_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_virtual_machine_id` after provisioning.\n"]
    pub fn storage_virtual_machine_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_virtual_machine_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_style` after provisioning.\n"]
    pub fn volume_style(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_style", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aggregate_configuration` after provisioning.\n"]
    pub fn aggregate_configuration(&self) -> terrars::ListRef<FsxOntapVolume_AggregateConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aggregate_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snaplock_configuration` after provisioning.\n"]
    pub fn snaplock_configuration(&self) -> terrars::ListRef<FsxOntapVolume_SnaplockConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snaplock_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tiering_policy` after provisioning.\n"]
    pub fn tiering_policy(&self) -> terrars::ListRef<FsxOntapVolume_TieringPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tiering_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOntapVolume_TimeoutsRef {
        FsxOntapVolume_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for FsxOntapVolume {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FsxOntapVolume { }

impl terrars::ToListMappable for FsxOntapVolume {
    type O = terrars::ListRef<FsxOntapVolumeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FsxOntapVolume_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_fsx_ontap_volume".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFsxOntapVolume {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub storage_virtual_machine_id: terrars::PrimField<std::string::String>,
}

impl BuildFsxOntapVolume {
    pub fn build(self, stack: &mut terrars::Stack) -> FsxOntapVolume {
        let out = FsxOntapVolume(std::rc::Rc::new(FsxOntapVolume_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FsxOntapVolumeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bypass_snaplock_enterprise_retention: core::default::Default::default(),
                copy_tags_to_backups: core::default::Default::default(),
                final_backup_tags: core::default::Default::default(),
                id: core::default::Default::default(),
                junction_path: core::default::Default::default(),
                name: self.name,
                ontap_volume_type: core::default::Default::default(),
                region: core::default::Default::default(),
                security_style: core::default::Default::default(),
                size_in_bytes: core::default::Default::default(),
                size_in_megabytes: core::default::Default::default(),
                skip_final_backup: core::default::Default::default(),
                snapshot_policy: core::default::Default::default(),
                storage_efficiency_enabled: core::default::Default::default(),
                storage_virtual_machine_id: self.storage_virtual_machine_id,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                volume_style: core::default::Default::default(),
                volume_type: core::default::Default::default(),
                aggregate_configuration: core::default::Default::default(),
                snaplock_configuration: core::default::Default::default(),
                tiering_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FsxOntapVolumeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapVolumeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FsxOntapVolumeRef {
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

    #[doc = "Get a reference to the value of field `bypass_snaplock_enterprise_retention` after provisioning.\n"]
    pub fn bypass_snaplock_enterprise_retention(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bypass_snaplock_enterprise_retention", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `copy_tags_to_backups` after provisioning.\n"]
    pub fn copy_tags_to_backups(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_tags_to_backups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `final_backup_tags` after provisioning.\n"]
    pub fn final_backup_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.final_backup_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `flexcache_endpoint_type` after provisioning.\n"]
    pub fn flexcache_endpoint_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flexcache_endpoint_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `junction_path` after provisioning.\n"]
    pub fn junction_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.junction_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ontap_volume_type` after provisioning.\n"]
    pub fn ontap_volume_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ontap_volume_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_style` after provisioning.\n"]
    pub fn security_style(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_style", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size_in_bytes` after provisioning.\n"]
    pub fn size_in_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_in_bytes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size_in_megabytes` after provisioning.\n"]
    pub fn size_in_megabytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_in_megabytes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_final_backup` after provisioning.\n"]
    pub fn skip_final_backup(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_final_backup", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_policy` after provisioning.\n"]
    pub fn snapshot_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_efficiency_enabled` after provisioning.\n"]
    pub fn storage_efficiency_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_efficiency_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_virtual_machine_id` after provisioning.\n"]
    pub fn storage_virtual_machine_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_virtual_machine_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_style` after provisioning.\n"]
    pub fn volume_style(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_style", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aggregate_configuration` after provisioning.\n"]
    pub fn aggregate_configuration(&self) -> terrars::ListRef<FsxOntapVolume_AggregateConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aggregate_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snaplock_configuration` after provisioning.\n"]
    pub fn snaplock_configuration(&self) -> terrars::ListRef<FsxOntapVolume_SnaplockConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snaplock_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tiering_policy` after provisioning.\n"]
    pub fn tiering_policy(&self) -> terrars::ListRef<FsxOntapVolume_TieringPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tiering_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOntapVolume_TimeoutsRef {
        FsxOntapVolume_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct FsxOntapVolumeDynamic {
    aggregate_configuration: std::option::Option<terrars::DynamicBlock<FsxOntapVolume_AggregateConfiguration>>,
    snaplock_configuration: std::option::Option<terrars::DynamicBlock<FsxOntapVolume_SnaplockConfiguration>>,
    tiering_policy: std::option::Option<terrars::DynamicBlock<FsxOntapVolume_TieringPolicy>>,
}

#[derive(Serialize)]
pub struct FsxOntapVolume_AggregateConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aggregates: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    constituents_per_aggregate: std::option::Option<terrars::PrimField<f64>>,
}

impl FsxOntapVolume_AggregateConfiguration {
    #[doc = "Set the field `aggregates`.\n"]
    pub fn set_aggregates(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.aggregates = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `constituents_per_aggregate`.\n"]
    pub fn set_constituents_per_aggregate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.constituents_per_aggregate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxOntapVolume_AggregateConfiguration {
    type O = terrars::BlockAssignable<FsxOntapVolume_AggregateConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapVolume_AggregateConfiguration {}

impl BuildFsxOntapVolume_AggregateConfiguration {
    pub fn build(self) -> FsxOntapVolume_AggregateConfiguration {
        FsxOntapVolume_AggregateConfiguration {
            aggregates: core::default::Default::default(),
            constituents_per_aggregate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOntapVolume_AggregateConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapVolume_AggregateConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxOntapVolume_AggregateConfigurationRef {
        FsxOntapVolume_AggregateConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapVolume_AggregateConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aggregates` after provisioning.\n"]
    pub fn aggregates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aggregates", self.base))
    }

    #[doc = "Get a reference to the value of field `constituents_per_aggregate` after provisioning.\n"]
    pub fn constituents_per_aggregate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.constituents_per_aggregate", self.base))
    }

    #[doc = "Get a reference to the value of field `total_constituents` after provisioning.\n"]
    pub fn total_constituents(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_constituents", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapVolume_SnaplockConfiguration_AutocommitPeriod {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl FsxOntapVolume_SnaplockConfiguration_AutocommitPeriod {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxOntapVolume_SnaplockConfiguration_AutocommitPeriod {
    type O = terrars::BlockAssignable<FsxOntapVolume_SnaplockConfiguration_AutocommitPeriod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapVolume_SnaplockConfiguration_AutocommitPeriod {}

impl BuildFsxOntapVolume_SnaplockConfiguration_AutocommitPeriod {
    pub fn build(self) -> FsxOntapVolume_SnaplockConfiguration_AutocommitPeriod {
        FsxOntapVolume_SnaplockConfiguration_AutocommitPeriod {
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOntapVolume_SnaplockConfiguration_AutocommitPeriodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapVolume_SnaplockConfiguration_AutocommitPeriodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOntapVolume_SnaplockConfiguration_AutocommitPeriodRef {
        FsxOntapVolume_SnaplockConfiguration_AutocommitPeriodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapVolume_SnaplockConfiguration_AutocommitPeriodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_DefaultRetention {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_DefaultRetention {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_DefaultRetention {
    type O = terrars::BlockAssignable<FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_DefaultRetention>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapVolume_SnaplockConfiguration_RetentionPeriod_DefaultRetention {}

impl BuildFsxOntapVolume_SnaplockConfiguration_RetentionPeriod_DefaultRetention {
    pub fn build(self) -> FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_DefaultRetention {
        FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_DefaultRetention {
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_DefaultRetentionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_DefaultRetentionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_DefaultRetentionRef {
        FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_DefaultRetentionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_DefaultRetentionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MaximumRetention {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MaximumRetention {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MaximumRetention {
    type O = terrars::BlockAssignable<FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MaximumRetention>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MaximumRetention {}

impl BuildFsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MaximumRetention {
    pub fn build(self) -> FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MaximumRetention {
        FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MaximumRetention {
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MaximumRetentionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MaximumRetentionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MaximumRetentionRef {
        FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MaximumRetentionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MaximumRetentionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MinimumRetention {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MinimumRetention {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MinimumRetention {
    type O = terrars::BlockAssignable<FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MinimumRetention>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MinimumRetention {}

impl BuildFsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MinimumRetention {
    pub fn build(self) -> FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MinimumRetention {
        FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MinimumRetention {
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MinimumRetentionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MinimumRetentionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MinimumRetentionRef {
        FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MinimumRetentionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MinimumRetentionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct FsxOntapVolume_SnaplockConfiguration_RetentionPeriodDynamic {
    default_retention: std
    ::option
    ::Option<terrars::DynamicBlock<FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_DefaultRetention>>,
    maximum_retention: std
    ::option
    ::Option<terrars::DynamicBlock<FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MaximumRetention>>,
    minimum_retention: std
    ::option
    ::Option<terrars::DynamicBlock<FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MinimumRetention>>,
}

#[derive(Serialize)]
pub struct FsxOntapVolume_SnaplockConfiguration_RetentionPeriod {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_retention: std
    ::option
    ::Option<std::vec::Vec<FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_DefaultRetention>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_retention: std
    ::option
    ::Option<std::vec::Vec<FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MaximumRetention>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_retention: std
    ::option
    ::Option<std::vec::Vec<FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MinimumRetention>>,
    dynamic: FsxOntapVolume_SnaplockConfiguration_RetentionPeriodDynamic,
}

impl FsxOntapVolume_SnaplockConfiguration_RetentionPeriod {
    #[doc = "Set the field `default_retention`.\n"]
    pub fn set_default_retention(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_DefaultRetention>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_retention = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_retention = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maximum_retention`.\n"]
    pub fn set_maximum_retention(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MaximumRetention>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.maximum_retention = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.maximum_retention = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `minimum_retention`.\n"]
    pub fn set_minimum_retention(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MinimumRetention>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.minimum_retention = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.minimum_retention = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FsxOntapVolume_SnaplockConfiguration_RetentionPeriod {
    type O = terrars::BlockAssignable<FsxOntapVolume_SnaplockConfiguration_RetentionPeriod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapVolume_SnaplockConfiguration_RetentionPeriod {}

impl BuildFsxOntapVolume_SnaplockConfiguration_RetentionPeriod {
    pub fn build(self) -> FsxOntapVolume_SnaplockConfiguration_RetentionPeriod {
        FsxOntapVolume_SnaplockConfiguration_RetentionPeriod {
            default_retention: core::default::Default::default(),
            maximum_retention: core::default::Default::default(),
            minimum_retention: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOntapVolume_SnaplockConfiguration_RetentionPeriodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapVolume_SnaplockConfiguration_RetentionPeriodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOntapVolume_SnaplockConfiguration_RetentionPeriodRef {
        FsxOntapVolume_SnaplockConfiguration_RetentionPeriodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapVolume_SnaplockConfiguration_RetentionPeriodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_retention` after provisioning.\n"]
    pub fn default_retention(
        &self,
    ) -> terrars::ListRef<FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_DefaultRetentionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_retention", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_retention` after provisioning.\n"]
    pub fn maximum_retention(
        &self,
    ) -> terrars::ListRef<FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MaximumRetentionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maximum_retention", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum_retention` after provisioning.\n"]
    pub fn minimum_retention(
        &self,
    ) -> terrars::ListRef<FsxOntapVolume_SnaplockConfiguration_RetentionPeriod_MinimumRetentionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.minimum_retention", self.base))
    }
}

#[derive(Serialize, Default)]
struct FsxOntapVolume_SnaplockConfigurationDynamic {
    autocommit_period: std
    ::option
    ::Option<terrars::DynamicBlock<FsxOntapVolume_SnaplockConfiguration_AutocommitPeriod>>,
    retention_period: std
    ::option
    ::Option<terrars::DynamicBlock<FsxOntapVolume_SnaplockConfiguration_RetentionPeriod>>,
}

#[derive(Serialize)]
pub struct FsxOntapVolume_SnaplockConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audit_log_volume: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    privileged_delete: std::option::Option<terrars::PrimField<std::string::String>>,
    snaplock_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_append_mode_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autocommit_period: std::option::Option<std::vec::Vec<FsxOntapVolume_SnaplockConfiguration_AutocommitPeriod>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_period: std::option::Option<std::vec::Vec<FsxOntapVolume_SnaplockConfiguration_RetentionPeriod>>,
    dynamic: FsxOntapVolume_SnaplockConfigurationDynamic,
}

impl FsxOntapVolume_SnaplockConfiguration {
    #[doc = "Set the field `audit_log_volume`.\n"]
    pub fn set_audit_log_volume(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.audit_log_volume = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `privileged_delete`.\n"]
    pub fn set_privileged_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.privileged_delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_append_mode_enabled`.\n"]
    pub fn set_volume_append_mode_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.volume_append_mode_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autocommit_period`.\n"]
    pub fn set_autocommit_period(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxOntapVolume_SnaplockConfiguration_AutocommitPeriod>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autocommit_period = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autocommit_period = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retention_period`.\n"]
    pub fn set_retention_period(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxOntapVolume_SnaplockConfiguration_RetentionPeriod>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retention_period = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retention_period = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FsxOntapVolume_SnaplockConfiguration {
    type O = terrars::BlockAssignable<FsxOntapVolume_SnaplockConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapVolume_SnaplockConfiguration {
    #[doc = ""]
    pub snaplock_type: terrars::PrimField<std::string::String>,
}

impl BuildFsxOntapVolume_SnaplockConfiguration {
    pub fn build(self) -> FsxOntapVolume_SnaplockConfiguration {
        FsxOntapVolume_SnaplockConfiguration {
            audit_log_volume: core::default::Default::default(),
            privileged_delete: core::default::Default::default(),
            snaplock_type: self.snaplock_type,
            volume_append_mode_enabled: core::default::Default::default(),
            autocommit_period: core::default::Default::default(),
            retention_period: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOntapVolume_SnaplockConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapVolume_SnaplockConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxOntapVolume_SnaplockConfigurationRef {
        FsxOntapVolume_SnaplockConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapVolume_SnaplockConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audit_log_volume` after provisioning.\n"]
    pub fn audit_log_volume(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audit_log_volume", self.base))
    }

    #[doc = "Get a reference to the value of field `privileged_delete` after provisioning.\n"]
    pub fn privileged_delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.privileged_delete", self.base))
    }

    #[doc = "Get a reference to the value of field `snaplock_type` after provisioning.\n"]
    pub fn snaplock_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snaplock_type", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_append_mode_enabled` after provisioning.\n"]
    pub fn volume_append_mode_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_append_mode_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `autocommit_period` after provisioning.\n"]
    pub fn autocommit_period(&self) -> terrars::ListRef<FsxOntapVolume_SnaplockConfiguration_AutocommitPeriodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autocommit_period", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_period` after provisioning.\n"]
    pub fn retention_period(&self) -> terrars::ListRef<FsxOntapVolume_SnaplockConfiguration_RetentionPeriodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_period", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapVolume_TieringPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cooling_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FsxOntapVolume_TieringPolicy {
    #[doc = "Set the field `cooling_period`.\n"]
    pub fn set_cooling_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cooling_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxOntapVolume_TieringPolicy {
    type O = terrars::BlockAssignable<FsxOntapVolume_TieringPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapVolume_TieringPolicy {}

impl BuildFsxOntapVolume_TieringPolicy {
    pub fn build(self) -> FsxOntapVolume_TieringPolicy {
        FsxOntapVolume_TieringPolicy {
            cooling_period: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOntapVolume_TieringPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapVolume_TieringPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxOntapVolume_TieringPolicyRef {
        FsxOntapVolume_TieringPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapVolume_TieringPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cooling_period` after provisioning.\n"]
    pub fn cooling_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cooling_period", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapVolume_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FsxOntapVolume_Timeouts {
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

impl terrars::ToListMappable for FsxOntapVolume_Timeouts {
    type O = terrars::BlockAssignable<FsxOntapVolume_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapVolume_Timeouts {}

impl BuildFsxOntapVolume_Timeouts {
    pub fn build(self) -> FsxOntapVolume_Timeouts {
        FsxOntapVolume_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOntapVolume_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapVolume_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxOntapVolume_TimeoutsRef {
        FsxOntapVolume_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapVolume_TimeoutsRef {
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
