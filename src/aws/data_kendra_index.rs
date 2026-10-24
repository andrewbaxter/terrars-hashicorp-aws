use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataKendraIndexData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataKendraIndex_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataKendraIndexData>,
}

#[derive(Clone)]
pub struct DataKendraIndex(std::rc::Rc<DataKendraIndex_>);

impl DataKendraIndex {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_units` after provisioning.\n"]
    pub fn capacity_units(&self) -> terrars::ListRef<DataKendraIndexCapacityUnitsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `document_metadata_configuration_updates` after provisioning.\n"]
    pub fn document_metadata_configuration_updates(
        &self,
    ) -> terrars::SetRef<DataKendraIndexDocumentMetadataConfigurationUpdatesElRef> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.document_metadata_configuration_updates", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `index_statistics` after provisioning.\n"]
    pub fn index_statistics(&self) -> terrars::ListRef<DataKendraIndexIndexStatisticsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.index_statistics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_configuration` after provisioning.\n"]
    pub fn server_side_encryption_configuration(
        &self,
    ) -> terrars::ListRef<DataKendraIndexServerSideEncryptionConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.server_side_encryption_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_context_policy` after provisioning.\n"]
    pub fn user_context_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_context_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_group_resolution_configuration` after provisioning.\n"]
    pub fn user_group_resolution_configuration(
        &self,
    ) -> terrars::ListRef<DataKendraIndexUserGroupResolutionConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.user_group_resolution_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_token_configurations` after provisioning.\n"]
    pub fn user_token_configurations(&self) -> terrars::ListRef<DataKendraIndexUserTokenConfigurationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_token_configurations", self.extract_ref()))
    }
}

impl terrars::Referable for DataKendraIndex {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataKendraIndex { }

impl terrars::ToListMappable for DataKendraIndex {
    type O = terrars::ListRef<DataKendraIndexRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataKendraIndex_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_kendra_index".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataKendraIndex {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildDataKendraIndex {
    pub fn build(self, stack: &mut terrars::Stack) -> DataKendraIndex {
        let out = DataKendraIndex(std::rc::Rc::new(DataKendraIndex_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataKendraIndexData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: self.id,
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataKendraIndexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndexRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataKendraIndexRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_units` after provisioning.\n"]
    pub fn capacity_units(&self) -> terrars::ListRef<DataKendraIndexCapacityUnitsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `document_metadata_configuration_updates` after provisioning.\n"]
    pub fn document_metadata_configuration_updates(
        &self,
    ) -> terrars::SetRef<DataKendraIndexDocumentMetadataConfigurationUpdatesElRef> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.document_metadata_configuration_updates", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `index_statistics` after provisioning.\n"]
    pub fn index_statistics(&self) -> terrars::ListRef<DataKendraIndexIndexStatisticsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.index_statistics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_configuration` after provisioning.\n"]
    pub fn server_side_encryption_configuration(
        &self,
    ) -> terrars::ListRef<DataKendraIndexServerSideEncryptionConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.server_side_encryption_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_context_policy` after provisioning.\n"]
    pub fn user_context_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_context_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_group_resolution_configuration` after provisioning.\n"]
    pub fn user_group_resolution_configuration(
        &self,
    ) -> terrars::ListRef<DataKendraIndexUserGroupResolutionConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.user_group_resolution_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_token_configurations` after provisioning.\n"]
    pub fn user_token_configurations(&self) -> terrars::ListRef<DataKendraIndexUserTokenConfigurationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_token_configurations", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexCapacityUnitsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_capacity_units: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_capacity_units: std::option::Option<terrars::PrimField<f64>>,
}

impl DataKendraIndexCapacityUnitsEl {
    #[doc = "Set the field `query_capacity_units`.\n"]
    pub fn set_query_capacity_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.query_capacity_units = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_capacity_units`.\n"]
    pub fn set_storage_capacity_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_capacity_units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKendraIndexCapacityUnitsEl {
    type O = terrars::BlockAssignable<DataKendraIndexCapacityUnitsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexCapacityUnitsEl {}

impl BuildDataKendraIndexCapacityUnitsEl {
    pub fn build(self) -> DataKendraIndexCapacityUnitsEl {
        DataKendraIndexCapacityUnitsEl {
            query_capacity_units: core::default::Default::default(),
            storage_capacity_units: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexCapacityUnitsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndexCapacityUnitsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKendraIndexCapacityUnitsElRef {
        DataKendraIndexCapacityUnitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexCapacityUnitsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `query_capacity_units` after provisioning.\n"]
    pub fn query_capacity_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_capacity_units", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_capacity_units` after provisioning.\n"]
    pub fn storage_capacity_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_capacity_units", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    freshness: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    importance: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rank_order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values_importance_map: std::option::Option<terrars::RecField<terrars::PrimField<f64>>>,
}

impl DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `freshness`.\n"]
    pub fn set_freshness(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.freshness = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `importance`.\n"]
    pub fn set_importance(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.importance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rank_order`.\n"]
    pub fn set_rank_order(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rank_order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values_importance_map`.\n"]
    pub fn set_values_importance_map(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.values_importance_map = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
    type O = terrars::BlockAssignable<DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {}

impl BuildDataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
    pub fn build(self) -> DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
        DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
            duration: core::default::Default::default(),
            freshness: core::default::Default::default(),
            importance: core::default::Default::default(),
            rank_order: core::default::Default::default(),
            values_importance_map: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef {
        DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `freshness` after provisioning.\n"]
    pub fn freshness(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.freshness", self.base))
    }

    #[doc = "Get a reference to the value of field `importance` after provisioning.\n"]
    pub fn importance(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.importance", self.base))
    }

    #[doc = "Get a reference to the value of field `rank_order` after provisioning.\n"]
    pub fn rank_order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rank_order", self.base))
    }

    #[doc = "Get a reference to the value of field `values_importance_map` after provisioning.\n"]
    pub fn values_importance_map(&self) -> terrars::RecRef<terrars::PrimExpr<f64>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.values_importance_map", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    displayable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    facetable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    searchable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sortable: std::option::Option<terrars::PrimField<bool>>,
}

impl DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
    #[doc = "Set the field `displayable`.\n"]
    pub fn set_displayable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.displayable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `facetable`.\n"]
    pub fn set_facetable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.facetable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `searchable`.\n"]
    pub fn set_searchable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.searchable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sortable`.\n"]
    pub fn set_sortable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.sortable = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
    type O = terrars::BlockAssignable<DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {}

impl BuildDataKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
    pub fn build(self) -> DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
        DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
            displayable: core::default::Default::default(),
            facetable: core::default::Default::default(),
            searchable: core::default::Default::default(),
            sortable: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef {
        DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `displayable` after provisioning.\n"]
    pub fn displayable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.displayable", self.base))
    }

    #[doc = "Get a reference to the value of field `facetable` after provisioning.\n"]
    pub fn facetable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.facetable", self.base))
    }

    #[doc = "Get a reference to the value of field `searchable` after provisioning.\n"]
    pub fn searchable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.searchable", self.base))
    }

    #[doc = "Get a reference to the value of field `sortable` after provisioning.\n"]
    pub fn sortable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sortable", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexDocumentMetadataConfigurationUpdatesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relevance: std::option::Option<terrars::ListField<DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::ListField<DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataKendraIndexDocumentMetadataConfigurationUpdatesEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `relevance`.\n"]
    pub fn set_relevance(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl>>,
    ) -> Self {
        self.relevance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search`.\n"]
    pub fn set_search(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl>>,
    ) -> Self {
        self.search = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKendraIndexDocumentMetadataConfigurationUpdatesEl {
    type O = terrars::BlockAssignable<DataKendraIndexDocumentMetadataConfigurationUpdatesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexDocumentMetadataConfigurationUpdatesEl {}

impl BuildDataKendraIndexDocumentMetadataConfigurationUpdatesEl {
    pub fn build(self) -> DataKendraIndexDocumentMetadataConfigurationUpdatesEl {
        DataKendraIndexDocumentMetadataConfigurationUpdatesEl {
            name: core::default::Default::default(),
            relevance: core::default::Default::default(),
            search: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexDocumentMetadataConfigurationUpdatesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndexDocumentMetadataConfigurationUpdatesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKendraIndexDocumentMetadataConfigurationUpdatesElRef {
        DataKendraIndexDocumentMetadataConfigurationUpdatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexDocumentMetadataConfigurationUpdatesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `relevance` after provisioning.\n"]
    pub fn relevance(&self) -> terrars::ListRef<DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.relevance", self.base))
    }

    #[doc = "Get a reference to the value of field `search` after provisioning.\n"]
    pub fn search(&self) -> terrars::ListRef<DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.search", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexIndexStatisticsElFaqStatisticsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indexed_question_answers_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataKendraIndexIndexStatisticsElFaqStatisticsEl {
    #[doc = "Set the field `indexed_question_answers_count`.\n"]
    pub fn set_indexed_question_answers_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.indexed_question_answers_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKendraIndexIndexStatisticsElFaqStatisticsEl {
    type O = terrars::BlockAssignable<DataKendraIndexIndexStatisticsElFaqStatisticsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexIndexStatisticsElFaqStatisticsEl {}

impl BuildDataKendraIndexIndexStatisticsElFaqStatisticsEl {
    pub fn build(self) -> DataKendraIndexIndexStatisticsElFaqStatisticsEl {
        DataKendraIndexIndexStatisticsElFaqStatisticsEl {
            indexed_question_answers_count: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexIndexStatisticsElFaqStatisticsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndexIndexStatisticsElFaqStatisticsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKendraIndexIndexStatisticsElFaqStatisticsElRef {
        DataKendraIndexIndexStatisticsElFaqStatisticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexIndexStatisticsElFaqStatisticsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `indexed_question_answers_count` after provisioning.\n"]
    pub fn indexed_question_answers_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.indexed_question_answers_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexIndexStatisticsElTextDocumentStatisticsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indexed_text_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indexed_text_documents_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataKendraIndexIndexStatisticsElTextDocumentStatisticsEl {
    #[doc = "Set the field `indexed_text_bytes`.\n"]
    pub fn set_indexed_text_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.indexed_text_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `indexed_text_documents_count`.\n"]
    pub fn set_indexed_text_documents_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.indexed_text_documents_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKendraIndexIndexStatisticsElTextDocumentStatisticsEl {
    type O = terrars::BlockAssignable<DataKendraIndexIndexStatisticsElTextDocumentStatisticsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexIndexStatisticsElTextDocumentStatisticsEl {}

impl BuildDataKendraIndexIndexStatisticsElTextDocumentStatisticsEl {
    pub fn build(self) -> DataKendraIndexIndexStatisticsElTextDocumentStatisticsEl {
        DataKendraIndexIndexStatisticsElTextDocumentStatisticsEl {
            indexed_text_bytes: core::default::Default::default(),
            indexed_text_documents_count: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexIndexStatisticsElTextDocumentStatisticsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndexIndexStatisticsElTextDocumentStatisticsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKendraIndexIndexStatisticsElTextDocumentStatisticsElRef {
        DataKendraIndexIndexStatisticsElTextDocumentStatisticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexIndexStatisticsElTextDocumentStatisticsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `indexed_text_bytes` after provisioning.\n"]
    pub fn indexed_text_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.indexed_text_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `indexed_text_documents_count` after provisioning.\n"]
    pub fn indexed_text_documents_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.indexed_text_documents_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexIndexStatisticsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    faq_statistics: std::option::Option<terrars::ListField<DataKendraIndexIndexStatisticsElFaqStatisticsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_document_statistics: std
    ::option
    ::Option<terrars::ListField<DataKendraIndexIndexStatisticsElTextDocumentStatisticsEl>>,
}

impl DataKendraIndexIndexStatisticsEl {
    #[doc = "Set the field `faq_statistics`.\n"]
    pub fn set_faq_statistics(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataKendraIndexIndexStatisticsElFaqStatisticsEl>>,
    ) -> Self {
        self.faq_statistics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text_document_statistics`.\n"]
    pub fn set_text_document_statistics(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataKendraIndexIndexStatisticsElTextDocumentStatisticsEl>>,
    ) -> Self {
        self.text_document_statistics = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKendraIndexIndexStatisticsEl {
    type O = terrars::BlockAssignable<DataKendraIndexIndexStatisticsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexIndexStatisticsEl {}

impl BuildDataKendraIndexIndexStatisticsEl {
    pub fn build(self) -> DataKendraIndexIndexStatisticsEl {
        DataKendraIndexIndexStatisticsEl {
            faq_statistics: core::default::Default::default(),
            text_document_statistics: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexIndexStatisticsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndexIndexStatisticsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKendraIndexIndexStatisticsElRef {
        DataKendraIndexIndexStatisticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexIndexStatisticsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `faq_statistics` after provisioning.\n"]
    pub fn faq_statistics(&self) -> terrars::ListRef<DataKendraIndexIndexStatisticsElFaqStatisticsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.faq_statistics", self.base))
    }

    #[doc = "Get a reference to the value of field `text_document_statistics` after provisioning.\n"]
    pub fn text_document_statistics(
        &self,
    ) -> terrars::ListRef<DataKendraIndexIndexStatisticsElTextDocumentStatisticsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_document_statistics", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexServerSideEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataKendraIndexServerSideEncryptionConfigurationEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKendraIndexServerSideEncryptionConfigurationEl {
    type O = terrars::BlockAssignable<DataKendraIndexServerSideEncryptionConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexServerSideEncryptionConfigurationEl {}

impl BuildDataKendraIndexServerSideEncryptionConfigurationEl {
    pub fn build(self) -> DataKendraIndexServerSideEncryptionConfigurationEl {
        DataKendraIndexServerSideEncryptionConfigurationEl { kms_key_id: core::default::Default::default() }
    }
}

pub struct DataKendraIndexServerSideEncryptionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndexServerSideEncryptionConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKendraIndexServerSideEncryptionConfigurationElRef {
        DataKendraIndexServerSideEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexServerSideEncryptionConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexUserGroupResolutionConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_group_resolution_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataKendraIndexUserGroupResolutionConfigurationEl {
    #[doc = "Set the field `user_group_resolution_mode`.\n"]
    pub fn set_user_group_resolution_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.user_group_resolution_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKendraIndexUserGroupResolutionConfigurationEl {
    type O = terrars::BlockAssignable<DataKendraIndexUserGroupResolutionConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexUserGroupResolutionConfigurationEl {}

impl BuildDataKendraIndexUserGroupResolutionConfigurationEl {
    pub fn build(self) -> DataKendraIndexUserGroupResolutionConfigurationEl {
        DataKendraIndexUserGroupResolutionConfigurationEl {
            user_group_resolution_mode: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexUserGroupResolutionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndexUserGroupResolutionConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKendraIndexUserGroupResolutionConfigurationElRef {
        DataKendraIndexUserGroupResolutionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexUserGroupResolutionConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `user_group_resolution_mode` after provisioning.\n"]
    pub fn user_group_resolution_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_group_resolution_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_attribute_field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_name_attribute_field: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
    #[doc = "Set the field `group_attribute_field`.\n"]
    pub fn set_group_attribute_field(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.group_attribute_field = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_name_attribute_field`.\n"]
    pub fn set_user_name_attribute_field(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.user_name_attribute_field = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
    type O = terrars::BlockAssignable<DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {}

impl BuildDataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
    pub fn build(self) -> DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
        DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
            group_attribute_field: core::default::Default::default(),
            user_name_attribute_field: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef {
        DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_attribute_field` after provisioning.\n"]
    pub fn group_attribute_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_attribute_field", self.base))
    }

    #[doc = "Get a reference to the value of field `user_name_attribute_field` after provisioning.\n"]
    pub fn user_name_attribute_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_name_attribute_field", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    claim_regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_attribute_field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_manager_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_name_attribute_field: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
    #[doc = "Set the field `claim_regex`.\n"]
    pub fn set_claim_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.claim_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_attribute_field`.\n"]
    pub fn set_group_attribute_field(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.group_attribute_field = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_location`.\n"]
    pub fn set_key_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secrets_manager_arn`.\n"]
    pub fn set_secrets_manager_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secrets_manager_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_name_attribute_field`.\n"]
    pub fn set_user_name_attribute_field(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.user_name_attribute_field = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
    type O = terrars::BlockAssignable<DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {}

impl BuildDataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
    pub fn build(self) -> DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
        DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
            claim_regex: core::default::Default::default(),
            group_attribute_field: core::default::Default::default(),
            issuer: core::default::Default::default(),
            key_location: core::default::Default::default(),
            secrets_manager_arn: core::default::Default::default(),
            url: core::default::Default::default(),
            user_name_attribute_field: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef {
        DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `claim_regex` after provisioning.\n"]
    pub fn claim_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.claim_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `group_attribute_field` after provisioning.\n"]
    pub fn group_attribute_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_attribute_field", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `key_location` after provisioning.\n"]
    pub fn key_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_location", self.base))
    }

    #[doc = "Get a reference to the value of field `secrets_manager_arn` after provisioning.\n"]
    pub fn secrets_manager_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secrets_manager_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `user_name_attribute_field` after provisioning.\n"]
    pub fn user_name_attribute_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_name_attribute_field", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexUserTokenConfigurationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_token_type_configuration: std
    ::option
    ::Option<terrars::ListField<DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwt_token_type_configuration: std
    ::option
    ::Option<terrars::ListField<DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl>>,
}

impl DataKendraIndexUserTokenConfigurationsEl {
    #[doc = "Set the field `json_token_type_configuration`.\n"]
    pub fn set_json_token_type_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl>>,
    ) -> Self {
        self.json_token_type_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `jwt_token_type_configuration`.\n"]
    pub fn set_jwt_token_type_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl>>,
    ) -> Self {
        self.jwt_token_type_configuration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKendraIndexUserTokenConfigurationsEl {
    type O = terrars::BlockAssignable<DataKendraIndexUserTokenConfigurationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexUserTokenConfigurationsEl {}

impl BuildDataKendraIndexUserTokenConfigurationsEl {
    pub fn build(self) -> DataKendraIndexUserTokenConfigurationsEl {
        DataKendraIndexUserTokenConfigurationsEl {
            json_token_type_configuration: core::default::Default::default(),
            jwt_token_type_configuration: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexUserTokenConfigurationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndexUserTokenConfigurationsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKendraIndexUserTokenConfigurationsElRef {
        DataKendraIndexUserTokenConfigurationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexUserTokenConfigurationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `json_token_type_configuration` after provisioning.\n"]
    pub fn json_token_type_configuration(
        &self,
    ) -> terrars::ListRef<DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_token_type_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `jwt_token_type_configuration` after provisioning.\n"]
    pub fn jwt_token_type_configuration(
        &self,
    ) -> terrars::ListRef<DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jwt_token_type_configuration", self.base))
    }
}
