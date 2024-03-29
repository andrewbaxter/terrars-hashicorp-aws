use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DocdbGlobalClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_version: Option<PrimField<String>>,
    global_cluster_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_db_cluster_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_encrypted: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DocdbGlobalClusterTimeoutsEl>,
}

struct DocdbGlobalCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DocdbGlobalClusterData>,
}

#[derive(Clone)]
pub struct DocdbGlobalCluster(Rc<DocdbGlobalCluster_>);

impl DocdbGlobalCluster {
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

    #[doc= "Set the field `database_name`.\n"]
    pub fn set_database_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().database_name = Some(v.into());
        self
    }

    #[doc= "Set the field `deletion_protection`.\n"]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }

    #[doc= "Set the field `engine`.\n"]
    pub fn set_engine(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().engine = Some(v.into());
        self
    }

    #[doc= "Set the field `engine_version`.\n"]
    pub fn set_engine_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().engine_version = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `source_db_cluster_identifier`.\n"]
    pub fn set_source_db_cluster_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_db_cluster_identifier = Some(v.into());
        self
    }

    #[doc= "Set the field `storage_encrypted`.\n"]
    pub fn set_storage_encrypted(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().storage_encrypted = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DocdbGlobalClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `global_cluster_identifier` after provisioning.\n"]
    pub fn global_cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.global_cluster_identifier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `global_cluster_members` after provisioning.\n"]
    pub fn global_cluster_members(&self) -> SetRef<DocdbGlobalClusterGlobalClusterMembersElRef> {
        SetRef::new(self.shared().clone(), format!("{}.global_cluster_members", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `global_cluster_resource_id` after provisioning.\n"]
    pub fn global_cluster_resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.global_cluster_resource_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_db_cluster_identifier` after provisioning.\n"]
    pub fn source_db_cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_db_cluster_identifier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_encrypted` after provisioning.\n"]
    pub fn storage_encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_encrypted", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DocdbGlobalClusterTimeoutsElRef {
        DocdbGlobalClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for DocdbGlobalCluster {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DocdbGlobalCluster { }

impl ToListMappable for DocdbGlobalCluster {
    type O = ListRef<DocdbGlobalClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DocdbGlobalCluster_ {
    fn extract_resource_type(&self) -> String {
        "aws_docdb_global_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDocdbGlobalCluster {
    pub tf_id: String,
    #[doc= ""]
    pub global_cluster_identifier: PrimField<String>,
}

impl BuildDocdbGlobalCluster {
    pub fn build(self, stack: &mut Stack) -> DocdbGlobalCluster {
        let out = DocdbGlobalCluster(Rc::new(DocdbGlobalCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DocdbGlobalClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                database_name: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                engine: core::default::Default::default(),
                engine_version: core::default::Default::default(),
                global_cluster_identifier: self.global_cluster_identifier,
                id: core::default::Default::default(),
                source_db_cluster_identifier: core::default::Default::default(),
                storage_encrypted: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DocdbGlobalClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for DocdbGlobalClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DocdbGlobalClusterRef {
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

    #[doc= "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `global_cluster_identifier` after provisioning.\n"]
    pub fn global_cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.global_cluster_identifier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `global_cluster_members` after provisioning.\n"]
    pub fn global_cluster_members(&self) -> SetRef<DocdbGlobalClusterGlobalClusterMembersElRef> {
        SetRef::new(self.shared().clone(), format!("{}.global_cluster_members", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `global_cluster_resource_id` after provisioning.\n"]
    pub fn global_cluster_resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.global_cluster_resource_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_db_cluster_identifier` after provisioning.\n"]
    pub fn source_db_cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_db_cluster_identifier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_encrypted` after provisioning.\n"]
    pub fn storage_encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_encrypted", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DocdbGlobalClusterTimeoutsElRef {
        DocdbGlobalClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DocdbGlobalClusterGlobalClusterMembersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    db_cluster_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_writer: Option<PrimField<bool>>,
}

impl DocdbGlobalClusterGlobalClusterMembersEl {
    #[doc= "Set the field `db_cluster_arn`.\n"]
    pub fn set_db_cluster_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.db_cluster_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `is_writer`.\n"]
    pub fn set_is_writer(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_writer = Some(v.into());
        self
    }
}

impl ToListMappable for DocdbGlobalClusterGlobalClusterMembersEl {
    type O = BlockAssignable<DocdbGlobalClusterGlobalClusterMembersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocdbGlobalClusterGlobalClusterMembersEl {}

impl BuildDocdbGlobalClusterGlobalClusterMembersEl {
    pub fn build(self) -> DocdbGlobalClusterGlobalClusterMembersEl {
        DocdbGlobalClusterGlobalClusterMembersEl {
            db_cluster_arn: core::default::Default::default(),
            is_writer: core::default::Default::default(),
        }
    }
}

pub struct DocdbGlobalClusterGlobalClusterMembersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DocdbGlobalClusterGlobalClusterMembersElRef {
    fn new(shared: StackShared, base: String) -> DocdbGlobalClusterGlobalClusterMembersElRef {
        DocdbGlobalClusterGlobalClusterMembersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocdbGlobalClusterGlobalClusterMembersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `db_cluster_arn` after provisioning.\n"]
    pub fn db_cluster_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_cluster_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `is_writer` after provisioning.\n"]
    pub fn is_writer(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_writer", self.base))
    }
}

#[derive(Serialize)]
pub struct DocdbGlobalClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl DocdbGlobalClusterTimeoutsEl {
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

impl ToListMappable for DocdbGlobalClusterTimeoutsEl {
    type O = BlockAssignable<DocdbGlobalClusterTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocdbGlobalClusterTimeoutsEl {}

impl BuildDocdbGlobalClusterTimeoutsEl {
    pub fn build(self) -> DocdbGlobalClusterTimeoutsEl {
        DocdbGlobalClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DocdbGlobalClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DocdbGlobalClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DocdbGlobalClusterTimeoutsElRef {
        DocdbGlobalClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocdbGlobalClusterTimeoutsElRef {
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
