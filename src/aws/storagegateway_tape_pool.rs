use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct StoragegatewayTapePoolData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    pool_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_lock_time_in_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_lock_type: Option<PrimField<String>>,
    storage_class: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
}

struct StoragegatewayTapePool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<StoragegatewayTapePoolData>,
}

#[derive(Clone)]
pub struct StoragegatewayTapePool(Rc<StoragegatewayTapePool_>);

impl StoragegatewayTapePool {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
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

    #[doc= "Set the field `retention_lock_time_in_days`.\n"]
    pub fn set_retention_lock_time_in_days(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retention_lock_time_in_days = Some(v.into());
        self
    }

    #[doc= "Set the field `retention_lock_type`.\n"]
    pub fn set_retention_lock_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().retention_lock_type = Some(v.into());
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

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pool_name` after provisioning.\n"]
    pub fn pool_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pool_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retention_lock_time_in_days` after provisioning.\n"]
    pub fn retention_lock_time_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention_lock_time_in_days", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retention_lock_type` after provisioning.\n"]
    pub fn retention_lock_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention_lock_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

impl Referable for StoragegatewayTapePool {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for StoragegatewayTapePool { }

impl ToListMappable for StoragegatewayTapePool {
    type O = ListRef<StoragegatewayTapePoolRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for StoragegatewayTapePool_ {
    fn extract_resource_type(&self) -> String {
        "aws_storagegateway_tape_pool".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildStoragegatewayTapePool {
    pub tf_id: String,
    #[doc= ""]
    pub pool_name: PrimField<String>,
    #[doc= ""]
    pub storage_class: PrimField<String>,
}

impl BuildStoragegatewayTapePool {
    pub fn build(self, stack: &mut Stack) -> StoragegatewayTapePool {
        let out = StoragegatewayTapePool(Rc::new(StoragegatewayTapePool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(StoragegatewayTapePoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                pool_name: self.pool_name,
                retention_lock_time_in_days: core::default::Default::default(),
                retention_lock_type: core::default::Default::default(),
                storage_class: self.storage_class,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct StoragegatewayTapePoolRef {
    shared: StackShared,
    base: String,
}

impl Ref for StoragegatewayTapePoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl StoragegatewayTapePoolRef {
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

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pool_name` after provisioning.\n"]
    pub fn pool_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pool_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retention_lock_time_in_days` after provisioning.\n"]
    pub fn retention_lock_time_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention_lock_time_in_days", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retention_lock_type` after provisioning.\n"]
    pub fn retention_lock_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention_lock_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}
