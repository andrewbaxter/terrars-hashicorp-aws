use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EbsSnapshotImportData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permanent_restore: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_tier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temporary_restore_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_data: std::option::Option<std::vec::Vec<EbsSnapshotImportClientDataEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_container: std::option::Option<std::vec::Vec<EbsSnapshotImportDiskContainerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EbsSnapshotImportTimeoutsEl>,
    dynamic: EbsSnapshotImportDynamic,
}

struct EbsSnapshotImport_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EbsSnapshotImportData>,
}

#[derive(Clone)]
pub struct EbsSnapshotImport(std::rc::Rc<EbsSnapshotImport_>);

impl EbsSnapshotImport {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().encrypted = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `permanent_restore`.\n"]
    pub fn set_permanent_restore(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().permanent_restore = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_name`.\n"]
    pub fn set_role_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().role_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_tier`.\n"]
    pub fn set_storage_tier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().storage_tier = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `temporary_restore_days`.\n"]
    pub fn set_temporary_restore_days(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().temporary_restore_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_data`.\n"]
    pub fn set_client_data(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EbsSnapshotImportClientDataEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `disk_container`.\n"]
    pub fn set_disk_container(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EbsSnapshotImportDiskContainerEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().disk_container = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.disk_container = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<EbsSnapshotImportTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_encryption_key_id` after provisioning.\n"]
    pub fn data_encryption_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_encryption_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outpost_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_alias` after provisioning.\n"]
    pub fn owner_alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permanent_restore` after provisioning.\n"]
    pub fn permanent_restore(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permanent_restore", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_name` after provisioning.\n"]
    pub fn role_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_tier` after provisioning.\n"]
    pub fn storage_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_tier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `temporary_restore_days` after provisioning.\n"]
    pub fn temporary_restore_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temporary_restore_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_id` after provisioning.\n"]
    pub fn volume_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_data` after provisioning.\n"]
    pub fn client_data(&self) -> terrars::ListRef<EbsSnapshotImportClientDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_container` after provisioning.\n"]
    pub fn disk_container(&self) -> terrars::ListRef<EbsSnapshotImportDiskContainerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_container", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EbsSnapshotImportTimeoutsElRef {
        EbsSnapshotImportTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for EbsSnapshotImport {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EbsSnapshotImport { }

impl terrars::ToListMappable for EbsSnapshotImport {
    type O = terrars::ListRef<EbsSnapshotImportRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EbsSnapshotImport_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ebs_snapshot_import".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEbsSnapshotImport {
    pub tf_id: std::string::String,
}

impl BuildEbsSnapshotImport {
    pub fn build(self, stack: &mut terrars::Stack) -> EbsSnapshotImport {
        let out = EbsSnapshotImport(std::rc::Rc::new(EbsSnapshotImport_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EbsSnapshotImportData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                encrypted: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                permanent_restore: core::default::Default::default(),
                role_name: core::default::Default::default(),
                storage_tier: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                temporary_restore_days: core::default::Default::default(),
                client_data: core::default::Default::default(),
                disk_container: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EbsSnapshotImportRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EbsSnapshotImportRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EbsSnapshotImportRef {
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

    #[doc = "Get a reference to the value of field `data_encryption_key_id` after provisioning.\n"]
    pub fn data_encryption_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_encryption_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outpost_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_alias` after provisioning.\n"]
    pub fn owner_alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permanent_restore` after provisioning.\n"]
    pub fn permanent_restore(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permanent_restore", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_name` after provisioning.\n"]
    pub fn role_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_tier` after provisioning.\n"]
    pub fn storage_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_tier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `temporary_restore_days` after provisioning.\n"]
    pub fn temporary_restore_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temporary_restore_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_id` after provisioning.\n"]
    pub fn volume_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_data` after provisioning.\n"]
    pub fn client_data(&self) -> terrars::ListRef<EbsSnapshotImportClientDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_container` after provisioning.\n"]
    pub fn disk_container(&self) -> terrars::ListRef<EbsSnapshotImportDiskContainerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_container", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EbsSnapshotImportTimeoutsElRef {
        EbsSnapshotImportTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EbsSnapshotImportClientDataEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upload_end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upload_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upload_start: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EbsSnapshotImportClientDataEl {
    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upload_end`.\n"]
    pub fn set_upload_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.upload_end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upload_size`.\n"]
    pub fn set_upload_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.upload_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upload_start`.\n"]
    pub fn set_upload_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.upload_start = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EbsSnapshotImportClientDataEl {
    type O = terrars::BlockAssignable<EbsSnapshotImportClientDataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEbsSnapshotImportClientDataEl {}

impl BuildEbsSnapshotImportClientDataEl {
    pub fn build(self) -> EbsSnapshotImportClientDataEl {
        EbsSnapshotImportClientDataEl {
            comment: core::default::Default::default(),
            upload_end: core::default::Default::default(),
            upload_size: core::default::Default::default(),
            upload_start: core::default::Default::default(),
        }
    }
}

pub struct EbsSnapshotImportClientDataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EbsSnapshotImportClientDataElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EbsSnapshotImportClientDataElRef {
        EbsSnapshotImportClientDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EbsSnapshotImportClientDataElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.base))
    }

    #[doc = "Get a reference to the value of field `upload_end` after provisioning.\n"]
    pub fn upload_end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upload_end", self.base))
    }

    #[doc = "Get a reference to the value of field `upload_size` after provisioning.\n"]
    pub fn upload_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upload_size", self.base))
    }

    #[doc = "Get a reference to the value of field `upload_start` after provisioning.\n"]
    pub fn upload_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upload_start", self.base))
    }
}

#[derive(Serialize)]
pub struct EbsSnapshotImportDiskContainerElUserBucketEl {
    s3_bucket: terrars::PrimField<std::string::String>,
    s3_key: terrars::PrimField<std::string::String>,
}

impl EbsSnapshotImportDiskContainerElUserBucketEl { }

impl terrars::ToListMappable for EbsSnapshotImportDiskContainerElUserBucketEl {
    type O = terrars::BlockAssignable<EbsSnapshotImportDiskContainerElUserBucketEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEbsSnapshotImportDiskContainerElUserBucketEl {
    #[doc = ""]
    pub s3_bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_key: terrars::PrimField<std::string::String>,
}

impl BuildEbsSnapshotImportDiskContainerElUserBucketEl {
    pub fn build(self) -> EbsSnapshotImportDiskContainerElUserBucketEl {
        EbsSnapshotImportDiskContainerElUserBucketEl {
            s3_bucket: self.s3_bucket,
            s3_key: self.s3_key,
        }
    }
}

pub struct EbsSnapshotImportDiskContainerElUserBucketElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EbsSnapshotImportDiskContainerElUserBucketElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EbsSnapshotImportDiskContainerElUserBucketElRef {
        EbsSnapshotImportDiskContainerElUserBucketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EbsSnapshotImportDiskContainerElUserBucketElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_key` after provisioning.\n"]
    pub fn s3_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct EbsSnapshotImportDiskContainerElDynamic {
    user_bucket: std::option::Option<terrars::DynamicBlock<EbsSnapshotImportDiskContainerElUserBucketEl>>,
}

#[derive(Serialize)]
pub struct EbsSnapshotImportDiskContainerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    format: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_bucket: std::option::Option<std::vec::Vec<EbsSnapshotImportDiskContainerElUserBucketEl>>,
    dynamic: EbsSnapshotImportDiskContainerElDynamic,
}

impl EbsSnapshotImportDiskContainerEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_bucket`.\n"]
    pub fn set_user_bucket(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EbsSnapshotImportDiskContainerElUserBucketEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user_bucket = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user_bucket = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EbsSnapshotImportDiskContainerEl {
    type O = terrars::BlockAssignable<EbsSnapshotImportDiskContainerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEbsSnapshotImportDiskContainerEl {
    #[doc = ""]
    pub format: terrars::PrimField<std::string::String>,
}

impl BuildEbsSnapshotImportDiskContainerEl {
    pub fn build(self) -> EbsSnapshotImportDiskContainerEl {
        EbsSnapshotImportDiskContainerEl {
            description: core::default::Default::default(),
            format: self.format,
            url: core::default::Default::default(),
            user_bucket: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EbsSnapshotImportDiskContainerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EbsSnapshotImportDiskContainerElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EbsSnapshotImportDiskContainerElRef {
        EbsSnapshotImportDiskContainerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EbsSnapshotImportDiskContainerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `user_bucket` after provisioning.\n"]
    pub fn user_bucket(&self) -> terrars::ListRef<EbsSnapshotImportDiskContainerElUserBucketElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_bucket", self.base))
    }
}

#[derive(Serialize)]
pub struct EbsSnapshotImportTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EbsSnapshotImportTimeoutsEl {
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
}

impl terrars::ToListMappable for EbsSnapshotImportTimeoutsEl {
    type O = terrars::BlockAssignable<EbsSnapshotImportTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEbsSnapshotImportTimeoutsEl {}

impl BuildEbsSnapshotImportTimeoutsEl {
    pub fn build(self) -> EbsSnapshotImportTimeoutsEl {
        EbsSnapshotImportTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct EbsSnapshotImportTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EbsSnapshotImportTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EbsSnapshotImportTimeoutsElRef {
        EbsSnapshotImportTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EbsSnapshotImportTimeoutsElRef {
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
}

#[derive(Serialize, Default)]
struct EbsSnapshotImportDynamic {
    client_data: std::option::Option<terrars::DynamicBlock<EbsSnapshotImportClientDataEl>>,
    disk_container: std::option::Option<terrars::DynamicBlock<EbsSnapshotImportDiskContainerEl>>,
}
