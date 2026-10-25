#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FsxOpenzfsVolumeData_ {
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
    copy_tags_to_snapshots: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_compression_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_volume_options: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    parent_volume_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_size_kib: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_capacity_quota_gib: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_capacity_reservation_gib: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nfs_exports: std::option::Option<std::vec::Vec<FsxOpenzfsVolume_NfsExports>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_snapshot: std::option::Option<std::vec::Vec<FsxOpenzfsVolume_OriginSnapshot>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FsxOpenzfsVolume_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_and_group_quotas: std::option::Option<std::vec::Vec<FsxOpenzfsVolume_UserAndGroupQuotas>>,
    dynamic: FsxOpenzfsVolumeDynamic,
}

struct FsxOpenzfsVolume_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FsxOpenzfsVolumeData_>,
}

#[derive(Clone)]
pub struct FsxOpenzfsVolume(std::rc::Rc<FsxOpenzfsVolume_>);

impl FsxOpenzfsVolume {
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

    #[doc = "Set the field `copy_tags_to_snapshots`.\n"]
    pub fn set_copy_tags_to_snapshots(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().copy_tags_to_snapshots = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_compression_type`.\n"]
    pub fn set_data_compression_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().data_compression_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete_volume_options`.\n"]
    pub fn set_delete_volume_options(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().delete_volume_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_only`.\n"]
    pub fn set_read_only(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().read_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_size_kib`.\n"]
    pub fn set_record_size_kib(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().record_size_kib = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_capacity_quota_gib`.\n"]
    pub fn set_storage_capacity_quota_gib(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().storage_capacity_quota_gib = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_capacity_reservation_gib`.\n"]
    pub fn set_storage_capacity_reservation_gib(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().storage_capacity_reservation_gib = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().volume_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nfs_exports`.\n"]
    pub fn set_nfs_exports(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxOpenzfsVolume_NfsExports>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().nfs_exports = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.nfs_exports = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `origin_snapshot`.\n"]
    pub fn set_origin_snapshot(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxOpenzfsVolume_OriginSnapshot>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().origin_snapshot = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.origin_snapshot = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<FsxOpenzfsVolume_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_and_group_quotas`.\n"]
    pub fn set_user_and_group_quotas(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxOpenzfsVolume_UserAndGroupQuotas>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_and_group_quotas = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_and_group_quotas = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `copy_tags_to_snapshots` after provisioning.\n"]
    pub fn copy_tags_to_snapshots(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_tags_to_snapshots", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_compression_type` after provisioning.\n"]
    pub fn data_compression_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_compression_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_volume_options` after provisioning.\n"]
    pub fn delete_volume_options(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delete_volume_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_volume_id` after provisioning.\n"]
    pub fn parent_volume_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_volume_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_only` after provisioning.\n"]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `record_size_kib` after provisioning.\n"]
    pub fn record_size_kib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_size_kib", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_capacity_quota_gib` after provisioning.\n"]
    pub fn storage_capacity_quota_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_capacity_quota_gib", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_capacity_reservation_gib` after provisioning.\n"]
    pub fn storage_capacity_reservation_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_capacity_reservation_gib", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nfs_exports` after provisioning.\n"]
    pub fn nfs_exports(&self) -> terrars::ListRef<FsxOpenzfsVolume_NfsExportsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs_exports", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin_snapshot` after provisioning.\n"]
    pub fn origin_snapshot(&self) -> terrars::ListRef<FsxOpenzfsVolume_OriginSnapshotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOpenzfsVolume_TimeoutsRef {
        FsxOpenzfsVolume_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for FsxOpenzfsVolume {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FsxOpenzfsVolume { }

impl terrars::ToListMappable for FsxOpenzfsVolume {
    type O = terrars::ListRef<FsxOpenzfsVolumeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FsxOpenzfsVolume_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_fsx_openzfs_volume".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFsxOpenzfsVolume {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub parent_volume_id: terrars::PrimField<std::string::String>,
}

impl BuildFsxOpenzfsVolume {
    pub fn build(self, stack: &mut terrars::Stack) -> FsxOpenzfsVolume {
        let out = FsxOpenzfsVolume(std::rc::Rc::new(FsxOpenzfsVolume_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FsxOpenzfsVolumeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                copy_tags_to_snapshots: core::default::Default::default(),
                data_compression_type: core::default::Default::default(),
                delete_volume_options: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                parent_volume_id: self.parent_volume_id,
                read_only: core::default::Default::default(),
                record_size_kib: core::default::Default::default(),
                region: core::default::Default::default(),
                storage_capacity_quota_gib: core::default::Default::default(),
                storage_capacity_reservation_gib: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                volume_type: core::default::Default::default(),
                nfs_exports: core::default::Default::default(),
                origin_snapshot: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                user_and_group_quotas: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FsxOpenzfsVolumeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOpenzfsVolumeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FsxOpenzfsVolumeRef {
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

    #[doc = "Get a reference to the value of field `copy_tags_to_snapshots` after provisioning.\n"]
    pub fn copy_tags_to_snapshots(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_tags_to_snapshots", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_compression_type` after provisioning.\n"]
    pub fn data_compression_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_compression_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_volume_options` after provisioning.\n"]
    pub fn delete_volume_options(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delete_volume_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_volume_id` after provisioning.\n"]
    pub fn parent_volume_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_volume_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_only` after provisioning.\n"]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `record_size_kib` after provisioning.\n"]
    pub fn record_size_kib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_size_kib", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_capacity_quota_gib` after provisioning.\n"]
    pub fn storage_capacity_quota_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_capacity_quota_gib", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_capacity_reservation_gib` after provisioning.\n"]
    pub fn storage_capacity_reservation_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_capacity_reservation_gib", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nfs_exports` after provisioning.\n"]
    pub fn nfs_exports(&self) -> terrars::ListRef<FsxOpenzfsVolume_NfsExportsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs_exports", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin_snapshot` after provisioning.\n"]
    pub fn origin_snapshot(&self) -> terrars::ListRef<FsxOpenzfsVolume_OriginSnapshotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOpenzfsVolume_TimeoutsRef {
        FsxOpenzfsVolume_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct FsxOpenzfsVolumeDynamic {
    nfs_exports: std::option::Option<terrars::DynamicBlock<FsxOpenzfsVolume_NfsExports>>,
    origin_snapshot: std::option::Option<terrars::DynamicBlock<FsxOpenzfsVolume_OriginSnapshot>>,
    user_and_group_quotas: std::option::Option<terrars::DynamicBlock<FsxOpenzfsVolume_UserAndGroupQuotas>>,
}

#[derive(Serialize)]
pub struct FsxOpenzfsVolume_NfsExports_ClientConfigurations {
    clients: terrars::PrimField<std::string::String>,
    options: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl FsxOpenzfsVolume_NfsExports_ClientConfigurations { }

impl terrars::ToListMappable for FsxOpenzfsVolume_NfsExports_ClientConfigurations {
    type O = terrars::BlockAssignable<FsxOpenzfsVolume_NfsExports_ClientConfigurations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsVolume_NfsExports_ClientConfigurations {
    #[doc = ""]
    pub clients: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub options: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildFsxOpenzfsVolume_NfsExports_ClientConfigurations {
    pub fn build(self) -> FsxOpenzfsVolume_NfsExports_ClientConfigurations {
        FsxOpenzfsVolume_NfsExports_ClientConfigurations {
            clients: self.clients,
            options: self.options,
        }
    }
}

#[allow(dead_code)]
pub struct FsxOpenzfsVolume_NfsExports_ClientConfigurationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOpenzfsVolume_NfsExports_ClientConfigurationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOpenzfsVolume_NfsExports_ClientConfigurationsRef {
        FsxOpenzfsVolume_NfsExports_ClientConfigurationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsVolume_NfsExports_ClientConfigurationsRef {
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
struct FsxOpenzfsVolume_NfsExportsDynamic {
    client_configurations: std
    ::option
    ::Option<terrars::DynamicBlock<FsxOpenzfsVolume_NfsExports_ClientConfigurations>>,
}

#[derive(Serialize)]
pub struct FsxOpenzfsVolume_NfsExports {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_configurations: std::option::Option<std::vec::Vec<FsxOpenzfsVolume_NfsExports_ClientConfigurations>>,
    dynamic: FsxOpenzfsVolume_NfsExportsDynamic,
}

impl FsxOpenzfsVolume_NfsExports {
    #[doc = "Set the field `client_configurations`.\n"]
    pub fn set_client_configurations(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxOpenzfsVolume_NfsExports_ClientConfigurations>>,
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

impl terrars::ToListMappable for FsxOpenzfsVolume_NfsExports {
    type O = terrars::BlockAssignable<FsxOpenzfsVolume_NfsExports>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsVolume_NfsExports {}

impl BuildFsxOpenzfsVolume_NfsExports {
    pub fn build(self) -> FsxOpenzfsVolume_NfsExports {
        FsxOpenzfsVolume_NfsExports {
            client_configurations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOpenzfsVolume_NfsExportsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOpenzfsVolume_NfsExportsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxOpenzfsVolume_NfsExportsRef {
        FsxOpenzfsVolume_NfsExportsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsVolume_NfsExportsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct FsxOpenzfsVolume_OriginSnapshot {
    copy_strategy: terrars::PrimField<std::string::String>,
    snapshot_arn: terrars::PrimField<std::string::String>,
}

impl FsxOpenzfsVolume_OriginSnapshot { }

impl terrars::ToListMappable for FsxOpenzfsVolume_OriginSnapshot {
    type O = terrars::BlockAssignable<FsxOpenzfsVolume_OriginSnapshot>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsVolume_OriginSnapshot {
    #[doc = ""]
    pub copy_strategy: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub snapshot_arn: terrars::PrimField<std::string::String>,
}

impl BuildFsxOpenzfsVolume_OriginSnapshot {
    pub fn build(self) -> FsxOpenzfsVolume_OriginSnapshot {
        FsxOpenzfsVolume_OriginSnapshot {
            copy_strategy: self.copy_strategy,
            snapshot_arn: self.snapshot_arn,
        }
    }
}

#[allow(dead_code)]
pub struct FsxOpenzfsVolume_OriginSnapshotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOpenzfsVolume_OriginSnapshotRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxOpenzfsVolume_OriginSnapshotRef {
        FsxOpenzfsVolume_OriginSnapshotRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsVolume_OriginSnapshotRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `copy_strategy` after provisioning.\n"]
    pub fn copy_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_arn` after provisioning.\n"]
    pub fn snapshot_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOpenzfsVolume_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FsxOpenzfsVolume_Timeouts {
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

impl terrars::ToListMappable for FsxOpenzfsVolume_Timeouts {
    type O = terrars::BlockAssignable<FsxOpenzfsVolume_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsVolume_Timeouts {}

impl BuildFsxOpenzfsVolume_Timeouts {
    pub fn build(self) -> FsxOpenzfsVolume_Timeouts {
        FsxOpenzfsVolume_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOpenzfsVolume_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOpenzfsVolume_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxOpenzfsVolume_TimeoutsRef {
        FsxOpenzfsVolume_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsVolume_TimeoutsRef {
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

#[derive(Serialize)]
pub struct FsxOpenzfsVolume_UserAndGroupQuotas {
    id: terrars::PrimField<f64>,
    storage_capacity_quota_gib: terrars::PrimField<f64>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl FsxOpenzfsVolume_UserAndGroupQuotas { }

impl terrars::ToListMappable for FsxOpenzfsVolume_UserAndGroupQuotas {
    type O = terrars::BlockAssignable<FsxOpenzfsVolume_UserAndGroupQuotas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOpenzfsVolume_UserAndGroupQuotas {
    #[doc = ""]
    pub id: terrars::PrimField<f64>,
    #[doc = ""]
    pub storage_capacity_quota_gib: terrars::PrimField<f64>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildFsxOpenzfsVolume_UserAndGroupQuotas {
    pub fn build(self) -> FsxOpenzfsVolume_UserAndGroupQuotas {
        FsxOpenzfsVolume_UserAndGroupQuotas {
            id: self.id,
            storage_capacity_quota_gib: self.storage_capacity_quota_gib,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct FsxOpenzfsVolume_UserAndGroupQuotasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOpenzfsVolume_UserAndGroupQuotasRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxOpenzfsVolume_UserAndGroupQuotasRef {
        FsxOpenzfsVolume_UserAndGroupQuotasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOpenzfsVolume_UserAndGroupQuotasRef {
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
