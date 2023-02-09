use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct StoragegatewayCachedIscsiVolumeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    gateway_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_encrypted: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
    network_interface_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_volume_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    target_name: PrimField<String>,
    volume_size_in_bytes: PrimField<f64>,
}

struct StoragegatewayCachedIscsiVolume_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<StoragegatewayCachedIscsiVolumeData>,
}

#[derive(Clone)]
pub struct StoragegatewayCachedIscsiVolume(Rc<StoragegatewayCachedIscsiVolume_>);

impl StoragegatewayCachedIscsiVolume {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `kms_encrypted`.\n"]
    pub fn set_kms_encrypted(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().kms_encrypted = Some(v.into());
        self
    }

    #[doc= "Set the field `kms_key`.\n"]
    pub fn set_kms_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key = Some(v.into());
        self
    }

    #[doc= "Set the field `snapshot_id`.\n"]
    pub fn set_snapshot_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().snapshot_id = Some(v.into());
        self
    }

    #[doc= "Set the field `source_volume_arn`.\n"]
    pub fn set_source_volume_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_volume_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `chap_enabled` after provisioning.\n"]
    pub fn chap_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.chap_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gateway_arn` after provisioning.\n"]
    pub fn gateway_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kms_encrypted` after provisioning.\n"]
    pub fn kms_encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_encrypted", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lun_number` after provisioning.\n"]
    pub fn lun_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.lun_number", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_interface_port` after provisioning.\n"]
    pub fn network_interface_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_interface_port", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `snapshot_id` after provisioning.\n"]
    pub fn snapshot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_volume_arn` after provisioning.\n"]
    pub fn source_volume_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_volume_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_arn` after provisioning.\n"]
    pub fn target_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_name` after provisioning.\n"]
    pub fn target_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `volume_arn` after provisioning.\n"]
    pub fn volume_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `volume_id` after provisioning.\n"]
    pub fn volume_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `volume_size_in_bytes` after provisioning.\n"]
    pub fn volume_size_in_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_size_in_bytes", self.extract_ref()))
    }
}

impl Resource for StoragegatewayCachedIscsiVolume {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for StoragegatewayCachedIscsiVolume {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for StoragegatewayCachedIscsiVolume {
    type O = ListRef<StoragegatewayCachedIscsiVolumeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for StoragegatewayCachedIscsiVolume_ {
    fn extract_resource_type(&self) -> String {
        "aws_storagegateway_cached_iscsi_volume".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildStoragegatewayCachedIscsiVolume {
    pub tf_id: String,
    #[doc= ""]
    pub gateway_arn: PrimField<String>,
    #[doc= ""]
    pub network_interface_id: PrimField<String>,
    #[doc= ""]
    pub target_name: PrimField<String>,
    #[doc= ""]
    pub volume_size_in_bytes: PrimField<f64>,
}

impl BuildStoragegatewayCachedIscsiVolume {
    pub fn build(self, stack: &mut Stack) -> StoragegatewayCachedIscsiVolume {
        let out = StoragegatewayCachedIscsiVolume(Rc::new(StoragegatewayCachedIscsiVolume_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(StoragegatewayCachedIscsiVolumeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                gateway_arn: self.gateway_arn,
                id: core::default::Default::default(),
                kms_encrypted: core::default::Default::default(),
                kms_key: core::default::Default::default(),
                network_interface_id: self.network_interface_id,
                snapshot_id: core::default::Default::default(),
                source_volume_arn: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                target_name: self.target_name,
                volume_size_in_bytes: self.volume_size_in_bytes,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct StoragegatewayCachedIscsiVolumeRef {
    shared: StackShared,
    base: String,
}

impl Ref for StoragegatewayCachedIscsiVolumeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl StoragegatewayCachedIscsiVolumeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `chap_enabled` after provisioning.\n"]
    pub fn chap_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.chap_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gateway_arn` after provisioning.\n"]
    pub fn gateway_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kms_encrypted` after provisioning.\n"]
    pub fn kms_encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_encrypted", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lun_number` after provisioning.\n"]
    pub fn lun_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.lun_number", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_interface_port` after provisioning.\n"]
    pub fn network_interface_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_interface_port", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `snapshot_id` after provisioning.\n"]
    pub fn snapshot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_volume_arn` after provisioning.\n"]
    pub fn source_volume_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_volume_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_arn` after provisioning.\n"]
    pub fn target_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_name` after provisioning.\n"]
    pub fn target_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `volume_arn` after provisioning.\n"]
    pub fn volume_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `volume_id` after provisioning.\n"]
    pub fn volume_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `volume_size_in_bytes` after provisioning.\n"]
    pub fn volume_size_in_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_size_in_bytes", self.extract_ref()))
    }
}
