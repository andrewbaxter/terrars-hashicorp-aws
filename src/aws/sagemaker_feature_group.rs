use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerFeatureGroupData {
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
    event_time_feature_name: terrars::PrimField<std::string::String>,
    feature_group_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    record_identifier_feature_name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feature_definition: std::option::Option<std::vec::Vec<SagemakerFeatureGroupFeatureDefinitionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    offline_store_config: std::option::Option<std::vec::Vec<SagemakerFeatureGroupOfflineStoreConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    online_store_config: std::option::Option<std::vec::Vec<SagemakerFeatureGroupOnlineStoreConfigEl>>,
    dynamic: SagemakerFeatureGroupDynamic,
}

struct SagemakerFeatureGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerFeatureGroupData>,
}

#[derive(Clone)]
pub struct SagemakerFeatureGroup(std::rc::Rc<SagemakerFeatureGroup_>);

impl SagemakerFeatureGroup {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `feature_definition`.\n"]
    pub fn set_feature_definition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerFeatureGroupFeatureDefinitionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().feature_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.feature_definition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `offline_store_config`.\n"]
    pub fn set_offline_store_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerFeatureGroupOfflineStoreConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().offline_store_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.offline_store_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `online_store_config`.\n"]
    pub fn set_online_store_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerFeatureGroupOnlineStoreConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().online_store_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.online_store_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_time_feature_name` after provisioning.\n"]
    pub fn event_time_feature_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_time_feature_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feature_group_name` after provisioning.\n"]
    pub fn feature_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `record_identifier_feature_name` after provisioning.\n"]
    pub fn record_identifier_feature_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.record_identifier_feature_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feature_definition` after provisioning.\n"]
    pub fn feature_definition(&self) -> terrars::ListRef<SagemakerFeatureGroupFeatureDefinitionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.feature_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `offline_store_config` after provisioning.\n"]
    pub fn offline_store_config(&self) -> terrars::ListRef<SagemakerFeatureGroupOfflineStoreConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.offline_store_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `online_store_config` after provisioning.\n"]
    pub fn online_store_config(&self) -> terrars::ListRef<SagemakerFeatureGroupOnlineStoreConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.online_store_config", self.extract_ref()))
    }
}

impl terrars::Referable for SagemakerFeatureGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerFeatureGroup { }

impl terrars::ToListMappable for SagemakerFeatureGroup {
    type O = terrars::ListRef<SagemakerFeatureGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerFeatureGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_feature_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerFeatureGroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub event_time_feature_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub feature_group_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub record_identifier_feature_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerFeatureGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerFeatureGroup {
        let out = SagemakerFeatureGroup(std::rc::Rc::new(SagemakerFeatureGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerFeatureGroupData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
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
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerFeatureGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFeatureGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerFeatureGroupRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_time_feature_name` after provisioning.\n"]
    pub fn event_time_feature_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_time_feature_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feature_group_name` after provisioning.\n"]
    pub fn feature_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `record_identifier_feature_name` after provisioning.\n"]
    pub fn record_identifier_feature_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.record_identifier_feature_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feature_definition` after provisioning.\n"]
    pub fn feature_definition(&self) -> terrars::ListRef<SagemakerFeatureGroupFeatureDefinitionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.feature_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `offline_store_config` after provisioning.\n"]
    pub fn offline_store_config(&self) -> terrars::ListRef<SagemakerFeatureGroupOfflineStoreConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.offline_store_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `online_store_config` after provisioning.\n"]
    pub fn online_store_config(&self) -> terrars::ListRef<SagemakerFeatureGroupOnlineStoreConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.online_store_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroupFeatureDefinitionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feature_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feature_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerFeatureGroupFeatureDefinitionEl {
    #[doc = "Set the field `feature_name`.\n"]
    pub fn set_feature_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.feature_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `feature_type`.\n"]
    pub fn set_feature_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.feature_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerFeatureGroupFeatureDefinitionEl {
    type O = terrars::BlockAssignable<SagemakerFeatureGroupFeatureDefinitionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFeatureGroupFeatureDefinitionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerFeatureGroupFeatureDefinitionElRef {
        SagemakerFeatureGroupFeatureDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroupFeatureDefinitionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `feature_name` after provisioning.\n"]
    pub fn feature_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature_name", self.base))
    }

    #[doc = "Get a reference to the value of field `feature_type` after provisioning.\n"]
    pub fn feature_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature_type", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigEl {
    #[doc = "Set the field `catalog`.\n"]
    pub fn set_catalog(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database`.\n"]
    pub fn set_database(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_name`.\n"]
    pub fn set_table_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigEl {
    type O = terrars::BlockAssignable<SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigElRef {
        SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog` after provisioning.\n"]
    pub fn catalog(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog", self.base))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_uri: terrars::PrimField<std::string::String>,
}

impl SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigEl {
    type O = terrars::BlockAssignable<SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigEl {
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigElRef {
        SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerFeatureGroupOfflineStoreConfigElDynamic {
    data_catalog_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigEl>>,
    s3_storage_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigEl>>,
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroupOfflineStoreConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_glue_table_creation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_catalog_config: std::option::Option<std::vec::Vec<SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_storage_config: std::option::Option<std::vec::Vec<SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigEl>>,
    dynamic: SagemakerFeatureGroupOfflineStoreConfigElDynamic,
}

impl SagemakerFeatureGroupOfflineStoreConfigEl {
    #[doc = "Set the field `disable_glue_table_creation`.\n"]
    pub fn set_disable_glue_table_creation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_glue_table_creation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_catalog_config`.\n"]
    pub fn set_data_catalog_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_catalog_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_catalog_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_storage_config`.\n"]
    pub fn set_s3_storage_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_storage_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_storage_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerFeatureGroupOfflineStoreConfigEl {
    type O = terrars::BlockAssignable<SagemakerFeatureGroupOfflineStoreConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerFeatureGroupOfflineStoreConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFeatureGroupOfflineStoreConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerFeatureGroupOfflineStoreConfigElRef {
        SagemakerFeatureGroupOfflineStoreConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroupOfflineStoreConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disable_glue_table_creation` after provisioning.\n"]
    pub fn disable_glue_table_creation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_glue_table_creation", self.base))
    }

    #[doc = "Get a reference to the value of field `data_catalog_config` after provisioning.\n"]
    pub fn data_catalog_config(
        &self,
    ) -> terrars::ListRef<SagemakerFeatureGroupOfflineStoreConfigElDataCatalogConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_catalog_config", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_storage_config` after provisioning.\n"]
    pub fn s3_storage_config(&self) -> terrars::ListRef<SagemakerFeatureGroupOfflineStoreConfigElS3StorageConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_storage_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigEl {
    type O = terrars::BlockAssignable<SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigElRef {
        SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerFeatureGroupOnlineStoreConfigElDynamic {
    security_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigEl>>,
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroupOnlineStoreConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_online_store: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_config: std::option::Option<std::vec::Vec<SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigEl>>,
    dynamic: SagemakerFeatureGroupOnlineStoreConfigElDynamic,
}

impl SagemakerFeatureGroupOnlineStoreConfigEl {
    #[doc = "Set the field `enable_online_store`.\n"]
    pub fn set_enable_online_store(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_online_store = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_config`.\n"]
    pub fn set_security_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.security_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.security_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerFeatureGroupOnlineStoreConfigEl {
    type O = terrars::BlockAssignable<SagemakerFeatureGroupOnlineStoreConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerFeatureGroupOnlineStoreConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFeatureGroupOnlineStoreConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerFeatureGroupOnlineStoreConfigElRef {
        SagemakerFeatureGroupOnlineStoreConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroupOnlineStoreConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_online_store` after provisioning.\n"]
    pub fn enable_online_store(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_online_store", self.base))
    }

    #[doc = "Get a reference to the value of field `security_config` after provisioning.\n"]
    pub fn security_config(&self) -> terrars::ListRef<SagemakerFeatureGroupOnlineStoreConfigElSecurityConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerFeatureGroupDynamic {
    feature_definition: std::option::Option<terrars::DynamicBlock<SagemakerFeatureGroupFeatureDefinitionEl>>,
    offline_store_config: std::option::Option<terrars::DynamicBlock<SagemakerFeatureGroupOfflineStoreConfigEl>>,
    online_store_config: std::option::Option<terrars::DynamicBlock<SagemakerFeatureGroupOnlineStoreConfigEl>>,
}
