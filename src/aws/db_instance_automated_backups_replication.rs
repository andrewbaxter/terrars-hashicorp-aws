use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DbInstanceAutomatedBackupsReplicationData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pre_signed_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_period: Option<PrimField<f64>>,
    source_db_instance_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DbInstanceAutomatedBackupsReplicationTimeoutsEl>,
}

struct DbInstanceAutomatedBackupsReplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DbInstanceAutomatedBackupsReplicationData>,
}

#[derive(Clone)]
pub struct DbInstanceAutomatedBackupsReplication(Rc<DbInstanceAutomatedBackupsReplication_>);

impl DbInstanceAutomatedBackupsReplication {
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

    #[doc= "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = Some(v.into());
        self
    }

    #[doc= "Set the field `pre_signed_url`.\n"]
    pub fn set_pre_signed_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().pre_signed_url = Some(v.into());
        self
    }

    #[doc= "Set the field `retention_period`.\n"]
    pub fn set_retention_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retention_period = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DbInstanceAutomatedBackupsReplicationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pre_signed_url` after provisioning.\n"]
    pub fn pre_signed_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pre_signed_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retention_period` after provisioning.\n"]
    pub fn retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_db_instance_arn` after provisioning.\n"]
    pub fn source_db_instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_db_instance_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DbInstanceAutomatedBackupsReplicationTimeoutsElRef {
        DbInstanceAutomatedBackupsReplicationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Resource for DbInstanceAutomatedBackupsReplication {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DbInstanceAutomatedBackupsReplication {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for DbInstanceAutomatedBackupsReplication {
    type O = ListRef<DbInstanceAutomatedBackupsReplicationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for DbInstanceAutomatedBackupsReplication_ {
    fn extract_resource_type(&self) -> String {
        "aws_db_instance_automated_backups_replication".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDbInstanceAutomatedBackupsReplication {
    pub tf_id: String,
    #[doc= ""]
    pub source_db_instance_arn: PrimField<String>,
}

impl BuildDbInstanceAutomatedBackupsReplication {
    pub fn build(self, stack: &mut Stack) -> DbInstanceAutomatedBackupsReplication {
        let out = DbInstanceAutomatedBackupsReplication(Rc::new(DbInstanceAutomatedBackupsReplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DbInstanceAutomatedBackupsReplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                pre_signed_url: core::default::Default::default(),
                retention_period: core::default::Default::default(),
                source_db_instance_arn: self.source_db_instance_arn,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DbInstanceAutomatedBackupsReplicationRef {
    shared: StackShared,
    base: String,
}

impl Ref for DbInstanceAutomatedBackupsReplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DbInstanceAutomatedBackupsReplicationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pre_signed_url` after provisioning.\n"]
    pub fn pre_signed_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pre_signed_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retention_period` after provisioning.\n"]
    pub fn retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_db_instance_arn` after provisioning.\n"]
    pub fn source_db_instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_db_instance_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DbInstanceAutomatedBackupsReplicationTimeoutsElRef {
        DbInstanceAutomatedBackupsReplicationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DbInstanceAutomatedBackupsReplicationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl DbInstanceAutomatedBackupsReplicationTimeoutsEl {
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
}

impl ToListMappable for DbInstanceAutomatedBackupsReplicationTimeoutsEl {
    type O = BlockAssignable<DbInstanceAutomatedBackupsReplicationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDbInstanceAutomatedBackupsReplicationTimeoutsEl {}

impl BuildDbInstanceAutomatedBackupsReplicationTimeoutsEl {
    pub fn build(self) -> DbInstanceAutomatedBackupsReplicationTimeoutsEl {
        DbInstanceAutomatedBackupsReplicationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct DbInstanceAutomatedBackupsReplicationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DbInstanceAutomatedBackupsReplicationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DbInstanceAutomatedBackupsReplicationTimeoutsElRef {
        DbInstanceAutomatedBackupsReplicationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DbInstanceAutomatedBackupsReplicationTimeoutsElRef {
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
}
