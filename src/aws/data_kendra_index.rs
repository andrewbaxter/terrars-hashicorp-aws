#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataKendraIndexData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataKendraIndex_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataKendraIndexData_>,
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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
    pub fn capacity_units(&self) -> terrars::ListRef<DataKendraIndex_CapacityUnitsRef> {
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
    ) -> terrars::SetRef<DataKendraIndex_DocumentMetadataConfigurationUpdatesRef> {
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
    pub fn index_statistics(&self) -> terrars::ListRef<DataKendraIndex_IndexStatisticsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.index_statistics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_configuration` after provisioning.\n"]
    pub fn server_side_encryption_configuration(
        &self,
    ) -> terrars::ListRef<DataKendraIndex_ServerSideEncryptionConfigurationRef> {
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
    ) -> terrars::ListRef<DataKendraIndex_UserGroupResolutionConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.user_group_resolution_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_token_configurations` after provisioning.\n"]
    pub fn user_token_configurations(&self) -> terrars::ListRef<DataKendraIndex_UserTokenConfigurationsRef> {
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
            data: std::cell::RefCell::new(DataKendraIndexData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: self.id,
                region: core::default::Default::default(),
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
    pub fn capacity_units(&self) -> terrars::ListRef<DataKendraIndex_CapacityUnitsRef> {
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
    ) -> terrars::SetRef<DataKendraIndex_DocumentMetadataConfigurationUpdatesRef> {
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
    pub fn index_statistics(&self) -> terrars::ListRef<DataKendraIndex_IndexStatisticsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.index_statistics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_configuration` after provisioning.\n"]
    pub fn server_side_encryption_configuration(
        &self,
    ) -> terrars::ListRef<DataKendraIndex_ServerSideEncryptionConfigurationRef> {
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
    ) -> terrars::ListRef<DataKendraIndex_UserGroupResolutionConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.user_group_resolution_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_token_configurations` after provisioning.\n"]
    pub fn user_token_configurations(&self) -> terrars::ListRef<DataKendraIndex_UserTokenConfigurationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_token_configurations", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndex_CapacityUnits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_capacity_units: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_capacity_units: std::option::Option<terrars::PrimField<f64>>,
}

impl DataKendraIndex_CapacityUnits {
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

impl terrars::ToListMappable for DataKendraIndex_CapacityUnits {
    type O = terrars::BlockAssignable<DataKendraIndex_CapacityUnits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndex_CapacityUnits {}

impl BuildDataKendraIndex_CapacityUnits {
    pub fn build(self) -> DataKendraIndex_CapacityUnits {
        DataKendraIndex_CapacityUnits {
            query_capacity_units: core::default::Default::default(),
            storage_capacity_units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataKendraIndex_CapacityUnitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndex_CapacityUnitsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKendraIndex_CapacityUnitsRef {
        DataKendraIndex_CapacityUnitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndex_CapacityUnitsRef {
    #[allow(dead_code)]
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
pub struct DataKendraIndex_DocumentMetadataConfigurationUpdates_Relevance {
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

impl DataKendraIndex_DocumentMetadataConfigurationUpdates_Relevance {
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

impl terrars::ToListMappable for DataKendraIndex_DocumentMetadataConfigurationUpdates_Relevance {
    type O = terrars::BlockAssignable<DataKendraIndex_DocumentMetadataConfigurationUpdates_Relevance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndex_DocumentMetadataConfigurationUpdates_Relevance {}

impl BuildDataKendraIndex_DocumentMetadataConfigurationUpdates_Relevance {
    pub fn build(self) -> DataKendraIndex_DocumentMetadataConfigurationUpdates_Relevance {
        DataKendraIndex_DocumentMetadataConfigurationUpdates_Relevance {
            duration: core::default::Default::default(),
            freshness: core::default::Default::default(),
            importance: core::default::Default::default(),
            rank_order: core::default::Default::default(),
            values_importance_map: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataKendraIndex_DocumentMetadataConfigurationUpdates_RelevanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndex_DocumentMetadataConfigurationUpdates_RelevanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKendraIndex_DocumentMetadataConfigurationUpdates_RelevanceRef {
        DataKendraIndex_DocumentMetadataConfigurationUpdates_RelevanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndex_DocumentMetadataConfigurationUpdates_RelevanceRef {
    #[allow(dead_code)]
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
pub struct DataKendraIndex_DocumentMetadataConfigurationUpdates_Search {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    displayable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    facetable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    searchable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sortable: std::option::Option<terrars::PrimField<bool>>,
}

impl DataKendraIndex_DocumentMetadataConfigurationUpdates_Search {
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

impl terrars::ToListMappable for DataKendraIndex_DocumentMetadataConfigurationUpdates_Search {
    type O = terrars::BlockAssignable<DataKendraIndex_DocumentMetadataConfigurationUpdates_Search>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndex_DocumentMetadataConfigurationUpdates_Search {}

impl BuildDataKendraIndex_DocumentMetadataConfigurationUpdates_Search {
    pub fn build(self) -> DataKendraIndex_DocumentMetadataConfigurationUpdates_Search {
        DataKendraIndex_DocumentMetadataConfigurationUpdates_Search {
            displayable: core::default::Default::default(),
            facetable: core::default::Default::default(),
            searchable: core::default::Default::default(),
            sortable: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataKendraIndex_DocumentMetadataConfigurationUpdates_SearchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndex_DocumentMetadataConfigurationUpdates_SearchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKendraIndex_DocumentMetadataConfigurationUpdates_SearchRef {
        DataKendraIndex_DocumentMetadataConfigurationUpdates_SearchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndex_DocumentMetadataConfigurationUpdates_SearchRef {
    #[allow(dead_code)]
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
pub struct DataKendraIndex_DocumentMetadataConfigurationUpdates {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relevance: std::option::Option<terrars::ListField<DataKendraIndex_DocumentMetadataConfigurationUpdates_Relevance>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::ListField<DataKendraIndex_DocumentMetadataConfigurationUpdates_Search>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataKendraIndex_DocumentMetadataConfigurationUpdates {
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
                    ::Into<terrars::ListField<DataKendraIndex_DocumentMetadataConfigurationUpdates_Relevance>>,
    ) -> Self {
        self.relevance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search`.\n"]
    pub fn set_search(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataKendraIndex_DocumentMetadataConfigurationUpdates_Search>>,
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

impl terrars::ToListMappable for DataKendraIndex_DocumentMetadataConfigurationUpdates {
    type O = terrars::BlockAssignable<DataKendraIndex_DocumentMetadataConfigurationUpdates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndex_DocumentMetadataConfigurationUpdates {}

impl BuildDataKendraIndex_DocumentMetadataConfigurationUpdates {
    pub fn build(self) -> DataKendraIndex_DocumentMetadataConfigurationUpdates {
        DataKendraIndex_DocumentMetadataConfigurationUpdates {
            name: core::default::Default::default(),
            relevance: core::default::Default::default(),
            search: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataKendraIndex_DocumentMetadataConfigurationUpdatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndex_DocumentMetadataConfigurationUpdatesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKendraIndex_DocumentMetadataConfigurationUpdatesRef {
        DataKendraIndex_DocumentMetadataConfigurationUpdatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndex_DocumentMetadataConfigurationUpdatesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `relevance` after provisioning.\n"]
    pub fn relevance(&self) -> terrars::ListRef<DataKendraIndex_DocumentMetadataConfigurationUpdates_RelevanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.relevance", self.base))
    }

    #[doc = "Get a reference to the value of field `search` after provisioning.\n"]
    pub fn search(&self) -> terrars::ListRef<DataKendraIndex_DocumentMetadataConfigurationUpdates_SearchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.search", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndex_IndexStatistics_FaqStatistics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indexed_question_answers_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataKendraIndex_IndexStatistics_FaqStatistics {
    #[doc = "Set the field `indexed_question_answers_count`.\n"]
    pub fn set_indexed_question_answers_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.indexed_question_answers_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKendraIndex_IndexStatistics_FaqStatistics {
    type O = terrars::BlockAssignable<DataKendraIndex_IndexStatistics_FaqStatistics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndex_IndexStatistics_FaqStatistics {}

impl BuildDataKendraIndex_IndexStatistics_FaqStatistics {
    pub fn build(self) -> DataKendraIndex_IndexStatistics_FaqStatistics {
        DataKendraIndex_IndexStatistics_FaqStatistics {
            indexed_question_answers_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataKendraIndex_IndexStatistics_FaqStatisticsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndex_IndexStatistics_FaqStatisticsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKendraIndex_IndexStatistics_FaqStatisticsRef {
        DataKendraIndex_IndexStatistics_FaqStatisticsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndex_IndexStatistics_FaqStatisticsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `indexed_question_answers_count` after provisioning.\n"]
    pub fn indexed_question_answers_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.indexed_question_answers_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndex_IndexStatistics_TextDocumentStatistics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indexed_text_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indexed_text_documents_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataKendraIndex_IndexStatistics_TextDocumentStatistics {
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

impl terrars::ToListMappable for DataKendraIndex_IndexStatistics_TextDocumentStatistics {
    type O = terrars::BlockAssignable<DataKendraIndex_IndexStatistics_TextDocumentStatistics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndex_IndexStatistics_TextDocumentStatistics {}

impl BuildDataKendraIndex_IndexStatistics_TextDocumentStatistics {
    pub fn build(self) -> DataKendraIndex_IndexStatistics_TextDocumentStatistics {
        DataKendraIndex_IndexStatistics_TextDocumentStatistics {
            indexed_text_bytes: core::default::Default::default(),
            indexed_text_documents_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataKendraIndex_IndexStatistics_TextDocumentStatisticsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndex_IndexStatistics_TextDocumentStatisticsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKendraIndex_IndexStatistics_TextDocumentStatisticsRef {
        DataKendraIndex_IndexStatistics_TextDocumentStatisticsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndex_IndexStatistics_TextDocumentStatisticsRef {
    #[allow(dead_code)]
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
pub struct DataKendraIndex_IndexStatistics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    faq_statistics: std::option::Option<terrars::ListField<DataKendraIndex_IndexStatistics_FaqStatistics>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_document_statistics: std
    ::option
    ::Option<terrars::ListField<DataKendraIndex_IndexStatistics_TextDocumentStatistics>>,
}

impl DataKendraIndex_IndexStatistics {
    #[doc = "Set the field `faq_statistics`.\n"]
    pub fn set_faq_statistics(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataKendraIndex_IndexStatistics_FaqStatistics>>,
    ) -> Self {
        self.faq_statistics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text_document_statistics`.\n"]
    pub fn set_text_document_statistics(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataKendraIndex_IndexStatistics_TextDocumentStatistics>>,
    ) -> Self {
        self.text_document_statistics = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKendraIndex_IndexStatistics {
    type O = terrars::BlockAssignable<DataKendraIndex_IndexStatistics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndex_IndexStatistics {}

impl BuildDataKendraIndex_IndexStatistics {
    pub fn build(self) -> DataKendraIndex_IndexStatistics {
        DataKendraIndex_IndexStatistics {
            faq_statistics: core::default::Default::default(),
            text_document_statistics: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataKendraIndex_IndexStatisticsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndex_IndexStatisticsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKendraIndex_IndexStatisticsRef {
        DataKendraIndex_IndexStatisticsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndex_IndexStatisticsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `faq_statistics` after provisioning.\n"]
    pub fn faq_statistics(&self) -> terrars::ListRef<DataKendraIndex_IndexStatistics_FaqStatisticsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.faq_statistics", self.base))
    }

    #[doc = "Get a reference to the value of field `text_document_statistics` after provisioning.\n"]
    pub fn text_document_statistics(
        &self,
    ) -> terrars::ListRef<DataKendraIndex_IndexStatistics_TextDocumentStatisticsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_document_statistics", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndex_ServerSideEncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataKendraIndex_ServerSideEncryptionConfiguration {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKendraIndex_ServerSideEncryptionConfiguration {
    type O = terrars::BlockAssignable<DataKendraIndex_ServerSideEncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndex_ServerSideEncryptionConfiguration {}

impl BuildDataKendraIndex_ServerSideEncryptionConfiguration {
    pub fn build(self) -> DataKendraIndex_ServerSideEncryptionConfiguration {
        DataKendraIndex_ServerSideEncryptionConfiguration { kms_key_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataKendraIndex_ServerSideEncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndex_ServerSideEncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKendraIndex_ServerSideEncryptionConfigurationRef {
        DataKendraIndex_ServerSideEncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndex_ServerSideEncryptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndex_UserGroupResolutionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_group_resolution_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataKendraIndex_UserGroupResolutionConfiguration {
    #[doc = "Set the field `user_group_resolution_mode`.\n"]
    pub fn set_user_group_resolution_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.user_group_resolution_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKendraIndex_UserGroupResolutionConfiguration {
    type O = terrars::BlockAssignable<DataKendraIndex_UserGroupResolutionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndex_UserGroupResolutionConfiguration {}

impl BuildDataKendraIndex_UserGroupResolutionConfiguration {
    pub fn build(self) -> DataKendraIndex_UserGroupResolutionConfiguration {
        DataKendraIndex_UserGroupResolutionConfiguration {
            user_group_resolution_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataKendraIndex_UserGroupResolutionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndex_UserGroupResolutionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKendraIndex_UserGroupResolutionConfigurationRef {
        DataKendraIndex_UserGroupResolutionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndex_UserGroupResolutionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `user_group_resolution_mode` after provisioning.\n"]
    pub fn user_group_resolution_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_group_resolution_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndex_UserTokenConfigurations_JsonTokenTypeConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_attribute_field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_name_attribute_field: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataKendraIndex_UserTokenConfigurations_JsonTokenTypeConfiguration {
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

impl terrars::ToListMappable for DataKendraIndex_UserTokenConfigurations_JsonTokenTypeConfiguration {
    type O = terrars::BlockAssignable<DataKendraIndex_UserTokenConfigurations_JsonTokenTypeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndex_UserTokenConfigurations_JsonTokenTypeConfiguration {}

impl BuildDataKendraIndex_UserTokenConfigurations_JsonTokenTypeConfiguration {
    pub fn build(self) -> DataKendraIndex_UserTokenConfigurations_JsonTokenTypeConfiguration {
        DataKendraIndex_UserTokenConfigurations_JsonTokenTypeConfiguration {
            group_attribute_field: core::default::Default::default(),
            user_name_attribute_field: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataKendraIndex_UserTokenConfigurations_JsonTokenTypeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndex_UserTokenConfigurations_JsonTokenTypeConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKendraIndex_UserTokenConfigurations_JsonTokenTypeConfigurationRef {
        DataKendraIndex_UserTokenConfigurations_JsonTokenTypeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndex_UserTokenConfigurations_JsonTokenTypeConfigurationRef {
    #[allow(dead_code)]
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
pub struct DataKendraIndex_UserTokenConfigurations_JwtTokenTypeConfiguration {
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

impl DataKendraIndex_UserTokenConfigurations_JwtTokenTypeConfiguration {
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

impl terrars::ToListMappable for DataKendraIndex_UserTokenConfigurations_JwtTokenTypeConfiguration {
    type O = terrars::BlockAssignable<DataKendraIndex_UserTokenConfigurations_JwtTokenTypeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndex_UserTokenConfigurations_JwtTokenTypeConfiguration {}

impl BuildDataKendraIndex_UserTokenConfigurations_JwtTokenTypeConfiguration {
    pub fn build(self) -> DataKendraIndex_UserTokenConfigurations_JwtTokenTypeConfiguration {
        DataKendraIndex_UserTokenConfigurations_JwtTokenTypeConfiguration {
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

#[allow(dead_code)]
pub struct DataKendraIndex_UserTokenConfigurations_JwtTokenTypeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndex_UserTokenConfigurations_JwtTokenTypeConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKendraIndex_UserTokenConfigurations_JwtTokenTypeConfigurationRef {
        DataKendraIndex_UserTokenConfigurations_JwtTokenTypeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndex_UserTokenConfigurations_JwtTokenTypeConfigurationRef {
    #[allow(dead_code)]
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
pub struct DataKendraIndex_UserTokenConfigurations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_token_type_configuration: std
    ::option
    ::Option<terrars::ListField<DataKendraIndex_UserTokenConfigurations_JsonTokenTypeConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwt_token_type_configuration: std
    ::option
    ::Option<terrars::ListField<DataKendraIndex_UserTokenConfigurations_JwtTokenTypeConfiguration>>,
}

impl DataKendraIndex_UserTokenConfigurations {
    #[doc = "Set the field `json_token_type_configuration`.\n"]
    pub fn set_json_token_type_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataKendraIndex_UserTokenConfigurations_JsonTokenTypeConfiguration>>,
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
                    ::Into<terrars::ListField<DataKendraIndex_UserTokenConfigurations_JwtTokenTypeConfiguration>>,
    ) -> Self {
        self.jwt_token_type_configuration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKendraIndex_UserTokenConfigurations {
    type O = terrars::BlockAssignable<DataKendraIndex_UserTokenConfigurations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndex_UserTokenConfigurations {}

impl BuildDataKendraIndex_UserTokenConfigurations {
    pub fn build(self) -> DataKendraIndex_UserTokenConfigurations {
        DataKendraIndex_UserTokenConfigurations {
            json_token_type_configuration: core::default::Default::default(),
            jwt_token_type_configuration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataKendraIndex_UserTokenConfigurationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraIndex_UserTokenConfigurationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKendraIndex_UserTokenConfigurationsRef {
        DataKendraIndex_UserTokenConfigurationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndex_UserTokenConfigurationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `json_token_type_configuration` after provisioning.\n"]
    pub fn json_token_type_configuration(
        &self,
    ) -> terrars::ListRef<DataKendraIndex_UserTokenConfigurations_JsonTokenTypeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_token_type_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `jwt_token_type_configuration` after provisioning.\n"]
    pub fn jwt_token_type_configuration(
        &self,
    ) -> terrars::ListRef<DataKendraIndex_UserTokenConfigurations_JwtTokenTypeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jwt_token_type_configuration", self.base))
    }
}
