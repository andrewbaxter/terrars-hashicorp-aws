#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOpensearchserverlessCollectionGroupsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataOpensearchserverlessCollectionGroups_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOpensearchserverlessCollectionGroupsData_>,
}

#[derive(Clone)]
pub struct DataOpensearchserverlessCollectionGroups(std::rc::Rc<DataOpensearchserverlessCollectionGroups_>);

impl DataOpensearchserverlessCollectionGroups {
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

    #[doc =
        "Get a reference to the value of field `collection_group_summaries` after provisioning.\nList of collection group summaries."]
    pub fn collection_group_summaries(
        &self,
    ) -> terrars::ListRef<DataOpensearchserverlessCollectionGroups_CollectionGroupSummariesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.collection_group_summaries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DataOpensearchserverlessCollectionGroups {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOpensearchserverlessCollectionGroups { }

impl terrars::ToListMappable for DataOpensearchserverlessCollectionGroups {
    type O = terrars::ListRef<DataOpensearchserverlessCollectionGroupsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOpensearchserverlessCollectionGroups_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_opensearchserverless_collection_groups".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOpensearchserverlessCollectionGroups {
    pub tf_id: std::string::String,
}

impl BuildDataOpensearchserverlessCollectionGroups {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOpensearchserverlessCollectionGroups {
        let out =
            DataOpensearchserverlessCollectionGroups(std::rc::Rc::new(DataOpensearchserverlessCollectionGroups_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(DataOpensearchserverlessCollectionGroupsData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    region: core::default::Default::default(),
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOpensearchserverlessCollectionGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchserverlessCollectionGroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOpensearchserverlessCollectionGroupsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `collection_group_summaries` after provisioning.\nList of collection group summaries."]
    pub fn collection_group_summaries(
        &self,
    ) -> terrars::ListRef<DataOpensearchserverlessCollectionGroups_CollectionGroupSummariesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.collection_group_summaries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchserverlessCollectionGroups_CollectionGroupSummaries_CapacityLimits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_indexing_capacity_in_ocu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_search_capacity_in_ocu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_indexing_capacity_in_ocu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_search_capacity_in_ocu: std::option::Option<terrars::PrimField<f64>>,
}

impl DataOpensearchserverlessCollectionGroups_CollectionGroupSummaries_CapacityLimits {
    #[doc = "Set the field `max_indexing_capacity_in_ocu`.\n"]
    pub fn set_max_indexing_capacity_in_ocu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_indexing_capacity_in_ocu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_search_capacity_in_ocu`.\n"]
    pub fn set_max_search_capacity_in_ocu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_search_capacity_in_ocu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_indexing_capacity_in_ocu`.\n"]
    pub fn set_min_indexing_capacity_in_ocu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_indexing_capacity_in_ocu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_search_capacity_in_ocu`.\n"]
    pub fn set_min_search_capacity_in_ocu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_search_capacity_in_ocu = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchserverlessCollectionGroups_CollectionGroupSummaries_CapacityLimits {
    type O =
        terrars::BlockAssignable<DataOpensearchserverlessCollectionGroups_CollectionGroupSummaries_CapacityLimits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchserverlessCollectionGroups_CollectionGroupSummaries_CapacityLimits {}

impl BuildDataOpensearchserverlessCollectionGroups_CollectionGroupSummaries_CapacityLimits {
    pub fn build(self) -> DataOpensearchserverlessCollectionGroups_CollectionGroupSummaries_CapacityLimits {
        DataOpensearchserverlessCollectionGroups_CollectionGroupSummaries_CapacityLimits {
            max_indexing_capacity_in_ocu: core::default::Default::default(),
            max_search_capacity_in_ocu: core::default::Default::default(),
            min_indexing_capacity_in_ocu: core::default::Default::default(),
            min_search_capacity_in_ocu: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchserverlessCollectionGroups_CollectionGroupSummaries_CapacityLimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchserverlessCollectionGroups_CollectionGroupSummaries_CapacityLimitsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchserverlessCollectionGroups_CollectionGroupSummaries_CapacityLimitsRef {
        DataOpensearchserverlessCollectionGroups_CollectionGroupSummaries_CapacityLimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchserverlessCollectionGroups_CollectionGroupSummaries_CapacityLimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_indexing_capacity_in_ocu` after provisioning.\n"]
    pub fn max_indexing_capacity_in_ocu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_indexing_capacity_in_ocu", self.base))
    }

    #[doc = "Get a reference to the value of field `max_search_capacity_in_ocu` after provisioning.\n"]
    pub fn max_search_capacity_in_ocu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_search_capacity_in_ocu", self.base))
    }

    #[doc = "Get a reference to the value of field `min_indexing_capacity_in_ocu` after provisioning.\n"]
    pub fn min_indexing_capacity_in_ocu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_indexing_capacity_in_ocu", self.base))
    }

    #[doc = "Get a reference to the value of field `min_search_capacity_in_ocu` after provisioning.\n"]
    pub fn min_search_capacity_in_ocu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_search_capacity_in_ocu", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchserverlessCollectionGroups_CollectionGroupSummaries {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_limits: std
    ::option
    ::Option<terrars::ListField<DataOpensearchserverlessCollectionGroups_CollectionGroupSummaries_CapacityLimits>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_of_collections: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standby_replicas: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOpensearchserverlessCollectionGroups_CollectionGroupSummaries {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_limits`.\n"]
    pub fn set_capacity_limits(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataOpensearchserverlessCollectionGroups_CollectionGroupSummaries_CapacityLimits>,
                    >,
    ) -> Self {
        self.capacity_limits = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_date`.\n"]
    pub fn set_created_date(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_of_collections`.\n"]
    pub fn set_number_of_collections(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.number_of_collections = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `standby_replicas`.\n"]
    pub fn set_standby_replicas(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.standby_replicas = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchserverlessCollectionGroups_CollectionGroupSummaries {
    type O = terrars::BlockAssignable<DataOpensearchserverlessCollectionGroups_CollectionGroupSummaries>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchserverlessCollectionGroups_CollectionGroupSummaries {}

impl BuildDataOpensearchserverlessCollectionGroups_CollectionGroupSummaries {
    pub fn build(self) -> DataOpensearchserverlessCollectionGroups_CollectionGroupSummaries {
        DataOpensearchserverlessCollectionGroups_CollectionGroupSummaries {
            arn: core::default::Default::default(),
            capacity_limits: core::default::Default::default(),
            created_date: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            number_of_collections: core::default::Default::default(),
            standby_replicas: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchserverlessCollectionGroups_CollectionGroupSummariesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchserverlessCollectionGroups_CollectionGroupSummariesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchserverlessCollectionGroups_CollectionGroupSummariesRef {
        DataOpensearchserverlessCollectionGroups_CollectionGroupSummariesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchserverlessCollectionGroups_CollectionGroupSummariesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_limits` after provisioning.\n"]
    pub fn capacity_limits(
        &self,
    ) -> terrars::ListRef<DataOpensearchserverlessCollectionGroups_CollectionGroupSummaries_CapacityLimitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_limits", self.base))
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `number_of_collections` after provisioning.\n"]
    pub fn number_of_collections(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_collections", self.base))
    }

    #[doc = "Get a reference to the value of field `standby_replicas` after provisioning.\n"]
    pub fn standby_replicas(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standby_replicas", self.base))
    }
}
