use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerFeatureGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    event_time_feature_name: PrimField<String>,
    feature_group_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    record_identifier_feature_name: PrimField<String>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    feature_definition: Option<Vec<SagemakerFeatureGroupFeatureDefinitionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offline_store_config: Option<Vec<SagemakerFeatureGroupOfflineStoreConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    online_store_config: Option<Vec<SagemakerFeatureGroupOnlineStoreConfigEl>>,
    dynamic: SagemakerFeatureGroupDynamic,
}

struct SagemakerFeatureGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerFeatureGroupData>,
}

#[derive(Clone)]
pub struct SagemakerFeatureGroup(Rc<SagemakerFeatureGroup_>);

impl SagemakerFeatureGroup {
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

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
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

    #[doc= "Set the field `feature_definition`.\n"]
    pub fn set_feature_definition(
        self,
        v: impl Into<BlockAssignable<SagemakerFeatureGroupFeatureDefinitionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().feature_definition = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.feature_definition = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `offline_store_config`.\n"]
    pub fn set_offline_store_config(
        self,
        v: impl Into<BlockAssignable<SagemakerFeatureGroupOfflineStoreConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().offline_store_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.offline_store_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `online_store_config`.\n"]
    pub fn set_online_store_config(
        self,
        v: impl Into<BlockAssignable<SagemakerFeatureGroupOnlineStoreConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().online_store_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.online_store_config = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `event_time_feature_name` after provisioning.\n"]
    pub fn event_time_feature_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_time_feature_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `feature_group_name` after provisioning.\n"]
    pub fn feature_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.feature_group_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `record_identifier_feature_name` after provisioning.\n"]
    pub fn record_identifier_feature_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.record_identifier_feature_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `feature_definition` after provisioning.\n"]
    pub fn feature_definition(&self) -> ListRef<SagemakerFeatureGroupFeatureDefinitionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.feature_definition", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `offline_store_config` after provisioning.\n"]
    pub fn offline_store_config(&self) -> ListRef<SagemakerFeatureGroupOfflineStoreConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.offline_store_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `online_store_config` after provisioning.\n"]
    pub fn online_store_config(&self) -> ListRef<SagemakerFeatureGroupOnlineStoreConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.online_store_config", self.extract_ref()))
    }
}

impl Referable for SagemakerFeatureGroup {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SagemakerFeatureGroup { }

impl ToListMappable for SagemakerFeatureGroup {
    type O = ListRef<SagemakerFeatureGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SagemakerFeatureGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_feature_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerFeatureGroup {
    pub tf_id: String,
    #[doc= ""]
    pub event_time_feature_name: PrimField<String>,
    #[doc= ""]
    pub feature_group_name: PrimField<String>,
    #[doc= ""]
    pub record_identifier_feature_name: PrimField<String>,
    #[doc= ""]
    pub role_arn: PrimField<String>,
}

impl BuildSagemakerFeatureGroup {
    pub fn build(self, stack: &mut Stack) -> SagemakerFeatureGroup {
        let out = SagemakerFeatureGroup(Rc::new(SagemakerFeatureGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerFeatureGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                event_time_feature_name: self.event_time_feature_name,
                feature_group_name: self.feature_group_name,
                id: core::default::Default::default(),
                record_identifier_feature_name: self.record_identifier_feature_name,
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                feature_definition: core::default::Default::default(),
                offline_store_config: core::default::Default::default(),
                online_store_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerFeatureGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerFeatureGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerFeatureGroupRef {
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

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `event_time_feature_name` after provisioning.\n"]
    pub fn event_time_feature_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_time_feature_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `feature_group_name` after provisioning.\n"]
    pub fn feature_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.feature_group_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `record_identifier_feature_name` after provisioning.\n"]
    pub fn record_identifier_feature_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.record_identifier_feature_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `feature_definition` after provisioning.\n"]
    pub fn feature_definition(&self) -> ListRef<SagemakerFeatureGroupFeatureDefinitionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.feature_definition", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `offline_store_config` after provisioning.\n"]
    pub fn offline_store_config(&self) -> ListRef<SagemakerFeatureGroupOfflineStoreConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.offline_store_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `online_store_config` after provisioning.\n"]
    pub fn online_store_config(&self) -> ListRef<SagemakerFeatureGroupOnlineStoreConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.online_store_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroupFeatureDefinitionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    feature_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    feature_type: Option<PrimField<String>>,
}

impl SagemakerFeatureGroupFeatureDefinitionEl {
    #[doc= "Set the field `feature_name`.\n"]
    pub fn set_feature_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.feature_name = Some(v.into());
        self
    }

    #[doc= "Set the field `feature_type`.\n"]
    pub fn set_feature_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.feature_type = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerFeatureGroupFeatureDefinitionEl {
    type O = BlockAssignable<SagemakerFeatureGroupFeatureDefinitionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFeatureGroupFeatureDefinitionEl {}

impl BuildSagemakerFeatureGroupFeatureDefinitionEl {
    pub fn build(self) -> SagemakerFeatureGroupFeatureDefinitionEl {
        SagemakerFeatureGroupFeatureDefinitionEl {
            feature_name: core::default::Default::default(),
            feature_type: core::default::Default::default(),
        }
    }
}

pub struct SagemakerFeatureGroupFeatureDefinitionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerFeatureGroupFeatureDefinitionElRef {
    fn new(shared: StackShared, base: String) -> SagemakerFeatureGroupFeatureDefinitionElRef {
        SagemakerFeatureGroupFeatureDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroupFeatureDefinitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `feature_name` after provisioning.\n"]
    pub fn feature_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.feature_name", self.base))
    }

    #[doc= "Get a reference to the value of field `feature_type` after provisioning.\n"]
    pub fn feature_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.feature_type", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_name: Option<PrimField<String>>,
}

impl SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigEl {
    #[doc= "Set the field `catalog`.\n"]
    pub fn set_catalog(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog = Some(v.into());
        self
    }

    #[doc= "Set the field `database`.\n"]
    pub fn set_database(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.database = Some(v.into());
        self
    }

    #[doc= "Set the field `table_name`.\n"]
    pub fn set_table_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.table_name = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigEl {
    type O = BlockAssignable<SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigEl {}

impl BuildSagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigEl {
    pub fn build(self) -> SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigEl {
        SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigEl {
            catalog: core::default::Default::default(),
            database: core::default::Default::default(),
            table_name: core::default::Default::default(),
        }
    }
}

pub struct SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigElRef {
        SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `catalog` after provisioning.\n"]
    pub fn catalog(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog", self.base))
    }

    #[doc= "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc= "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    s3_uri: PrimField<String>,
}

impl SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigEl {
    #[doc= "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigEl {
    type O = BlockAssignable<SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigEl {
    #[doc= ""]
    pub s3_uri: PrimField<String>,
}

impl BuildSagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigEl {
    pub fn build(self) -> SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigEl {
        SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigEl {
            kms_key_id: core::default::Default::default(),
            s3_uri: self.s3_uri,
        }
    }
}

pub struct SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigElRef {
        SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc= "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerFeatureGroupOfflineStoreConfigElDynamic {
    data_catalog_config: Option<DynamicBlock<SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigEl>>,
    s3_storage_config: Option<DynamicBlock<SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigEl>>,
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroupOfflineStoreConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_glue_table_creation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_catalog_config: Option<Vec<SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_storage_config: Option<Vec<SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigEl>>,
    dynamic: SagemakerFeatureGroupOfflineStoreConfigElDynamic,
}

impl SagemakerFeatureGroupOfflineStoreConfigEl {
    #[doc= "Set the field `disable_glue_table_creation`.\n"]
    pub fn set_disable_glue_table_creation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_glue_table_creation = Some(v.into());
        self
    }

    #[doc= "Set the field `data_catalog_config`.\n"]
    pub fn set_data_catalog_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_catalog_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_catalog_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `s3_storage_config`.\n"]
    pub fn set_s3_storage_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_storage_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_storage_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerFeatureGroupOfflineStoreConfigEl {
    type O = BlockAssignable<SagemakerFeatureGroupOfflineStoreConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFeatureGroupOfflineStoreConfigEl {}

impl BuildSagemakerFeatureGroupOfflineStoreConfigEl {
    pub fn build(self) -> SagemakerFeatureGroupOfflineStoreConfigEl {
        SagemakerFeatureGroupOfflineStoreConfigEl {
            disable_glue_table_creation: core::default::Default::default(),
            data_catalog_config: core::default::Default::default(),
            s3_storage_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerFeatureGroupOfflineStoreConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerFeatureGroupOfflineStoreConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerFeatureGroupOfflineStoreConfigElRef {
        SagemakerFeatureGroupOfflineStoreConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroupOfflineStoreConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disable_glue_table_creation` after provisioning.\n"]
    pub fn disable_glue_table_creation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_glue_table_creation", self.base))
    }

    #[doc= "Get a reference to the value of field `data_catalog_config` after provisioning.\n"]
    pub fn data_catalog_config(&self) -> ListRef<SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_catalog_config", self.base))
    }

    #[doc= "Get a reference to the value of field `s3_storage_config` after provisioning.\n"]
    pub fn s3_storage_config(&self) -> ListRef<SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_storage_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
}

impl SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigEl {
    #[doc= "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigEl {
    type O = BlockAssignable<SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFeatureGroupOnlineStoreConfigElSecurityConfigEl {}

impl BuildSagemakerFeatureGroupOnlineStoreConfigElSecurityConfigEl {
    pub fn build(self) -> SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigEl {
        SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigEl { kms_key_id: core::default::Default::default() }
    }
}

pub struct SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigElRef {
        SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerFeatureGroupOnlineStoreConfigElDynamic {
    security_config: Option<DynamicBlock<SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigEl>>,
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroupOnlineStoreConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_online_store: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_config: Option<Vec<SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigEl>>,
    dynamic: SagemakerFeatureGroupOnlineStoreConfigElDynamic,
}

impl SagemakerFeatureGroupOnlineStoreConfigEl {
    #[doc= "Set the field `enable_online_store`.\n"]
    pub fn set_enable_online_store(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_online_store = Some(v.into());
        self
    }

    #[doc= "Set the field `security_config`.\n"]
    pub fn set_security_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.security_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.security_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerFeatureGroupOnlineStoreConfigEl {
    type O = BlockAssignable<SagemakerFeatureGroupOnlineStoreConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFeatureGroupOnlineStoreConfigEl {}

impl BuildSagemakerFeatureGroupOnlineStoreConfigEl {
    pub fn build(self) -> SagemakerFeatureGroupOnlineStoreConfigEl {
        SagemakerFeatureGroupOnlineStoreConfigEl {
            enable_online_store: core::default::Default::default(),
            security_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerFeatureGroupOnlineStoreConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerFeatureGroupOnlineStoreConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerFeatureGroupOnlineStoreConfigElRef {
        SagemakerFeatureGroupOnlineStoreConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroupOnlineStoreConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enable_online_store` after provisioning.\n"]
    pub fn enable_online_store(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_online_store", self.base))
    }

    #[doc= "Get a reference to the value of field `security_config` after provisioning.\n"]
    pub fn security_config(&self) -> ListRef<SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.security_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerFeatureGroupDynamic {
    feature_definition: Option<DynamicBlock<SagemakerFeatureGroupFeatureDefinitionEl>>,
    offline_store_config: Option<DynamicBlock<SagemakerFeatureGroupOfflineStoreConfigEl>>,
    online_store_config: Option<DynamicBlock<SagemakerFeatureGroupOnlineStoreConfigEl>>,
}
