use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct KmsCustomKeyStoreData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cloud_hsm_cluster_id: PrimField<String>,
    custom_key_store_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    key_store_password: PrimField<String>,
    trust_anchor_certificate: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<KmsCustomKeyStoreTimeoutsEl>,
}

struct KmsCustomKeyStore_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<KmsCustomKeyStoreData>,
}

#[derive(Clone)]
pub struct KmsCustomKeyStore(Rc<KmsCustomKeyStore_>);

impl KmsCustomKeyStore {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Resource) -> Self {
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

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<KmsCustomKeyStoreTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `cloud_hsm_cluster_id` after provisioning.\n"]
    pub fn cloud_hsm_cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_hsm_cluster_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_key_store_name` after provisioning.\n"]
    pub fn custom_key_store_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_key_store_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_store_password` after provisioning.\n"]
    pub fn key_store_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_store_password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `trust_anchor_certificate` after provisioning.\n"]
    pub fn trust_anchor_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trust_anchor_certificate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsCustomKeyStoreTimeoutsElRef {
        KmsCustomKeyStoreTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Resource for KmsCustomKeyStore {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for KmsCustomKeyStore {
    type O = ListRef<KmsCustomKeyStoreRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for KmsCustomKeyStore_ {
    fn extract_resource_type(&self) -> String {
        "aws_kms_custom_key_store".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKmsCustomKeyStore {
    pub tf_id: String,
    #[doc= ""]
    pub cloud_hsm_cluster_id: PrimField<String>,
    #[doc= ""]
    pub custom_key_store_name: PrimField<String>,
    #[doc= ""]
    pub key_store_password: PrimField<String>,
    #[doc= ""]
    pub trust_anchor_certificate: PrimField<String>,
}

impl BuildKmsCustomKeyStore {
    pub fn build(self, stack: &mut Stack) -> KmsCustomKeyStore {
        let out = KmsCustomKeyStore(Rc::new(KmsCustomKeyStore_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(KmsCustomKeyStoreData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cloud_hsm_cluster_id: self.cloud_hsm_cluster_id,
                custom_key_store_name: self.custom_key_store_name,
                id: core::default::Default::default(),
                key_store_password: self.key_store_password,
                trust_anchor_certificate: self.trust_anchor_certificate,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct KmsCustomKeyStoreRef {
    shared: StackShared,
    base: String,
}

impl Ref for KmsCustomKeyStoreRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl KmsCustomKeyStoreRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cloud_hsm_cluster_id` after provisioning.\n"]
    pub fn cloud_hsm_cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_hsm_cluster_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_key_store_name` after provisioning.\n"]
    pub fn custom_key_store_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_key_store_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_store_password` after provisioning.\n"]
    pub fn key_store_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_store_password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `trust_anchor_certificate` after provisioning.\n"]
    pub fn trust_anchor_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trust_anchor_certificate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsCustomKeyStoreTimeoutsElRef {
        KmsCustomKeyStoreTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct KmsCustomKeyStoreTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl KmsCustomKeyStoreTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for KmsCustomKeyStoreTimeoutsEl {
    type O = BlockAssignable<KmsCustomKeyStoreTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKmsCustomKeyStoreTimeoutsEl {}

impl BuildKmsCustomKeyStoreTimeoutsEl {
    pub fn build(self) -> KmsCustomKeyStoreTimeoutsEl {
        KmsCustomKeyStoreTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct KmsCustomKeyStoreTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KmsCustomKeyStoreTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> KmsCustomKeyStoreTimeoutsElRef {
        KmsCustomKeyStoreTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KmsCustomKeyStoreTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
