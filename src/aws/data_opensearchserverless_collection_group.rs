#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOpensearchserverlessCollectionGroupData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataOpensearchserverlessCollectionGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOpensearchserverlessCollectionGroupData_>,
}

#[derive(Clone)]
pub struct DataOpensearchserverlessCollectionGroup(std::rc::Rc<DataOpensearchserverlessCollectionGroup_>);

impl DataOpensearchserverlessCollectionGroup {
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

    #[doc = "Set the field `id`.\nID of the collection group."]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the collection group."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_limits` after provisioning.\n"]
    pub fn capacity_limits(&self) -> terrars::ListRef<DataOpensearchserverlessCollectionGroup_CapacityLimitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_limits", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_date` after provisioning.\nDate the collection group was created."]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the collection group."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generation` after provisioning.\nGeneration of the collection group."]
    pub fn generation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nID of the collection group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the collection group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `standby_replicas` after provisioning.\nIndicates whether standby replicas should be used for collections in this group."]
    pub fn standby_replicas(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standby_replicas", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataOpensearchserverlessCollectionGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOpensearchserverlessCollectionGroup { }

impl terrars::ToListMappable for DataOpensearchserverlessCollectionGroup {
    type O = terrars::ListRef<DataOpensearchserverlessCollectionGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOpensearchserverlessCollectionGroup_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_opensearchserverless_collection_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOpensearchserverlessCollectionGroup {
    pub tf_id: std::string::String,
}

impl BuildDataOpensearchserverlessCollectionGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOpensearchserverlessCollectionGroup {
        let out = DataOpensearchserverlessCollectionGroup(std::rc::Rc::new(DataOpensearchserverlessCollectionGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOpensearchserverlessCollectionGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOpensearchserverlessCollectionGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchserverlessCollectionGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOpensearchserverlessCollectionGroupRef {
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

    #[doc = "Get a reference to the value of field `capacity_limits` after provisioning.\n"]
    pub fn capacity_limits(&self) -> terrars::ListRef<DataOpensearchserverlessCollectionGroup_CapacityLimitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_limits", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_date` after provisioning.\nDate the collection group was created."]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the collection group."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generation` after provisioning.\nGeneration of the collection group."]
    pub fn generation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nID of the collection group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the collection group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `standby_replicas` after provisioning.\nIndicates whether standby replicas should be used for collections in this group."]
    pub fn standby_replicas(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standby_replicas", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchserverlessCollectionGroup_CapacityLimits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_indexing_capacity_in_ocu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_search_capacity_in_ocu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_indexing_capacity_in_ocu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_search_capacity_in_ocu: std::option::Option<terrars::PrimField<f64>>,
}

impl DataOpensearchserverlessCollectionGroup_CapacityLimits {
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

impl terrars::ToListMappable for DataOpensearchserverlessCollectionGroup_CapacityLimits {
    type O = terrars::BlockAssignable<DataOpensearchserverlessCollectionGroup_CapacityLimits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchserverlessCollectionGroup_CapacityLimits {}

impl BuildDataOpensearchserverlessCollectionGroup_CapacityLimits {
    pub fn build(self) -> DataOpensearchserverlessCollectionGroup_CapacityLimits {
        DataOpensearchserverlessCollectionGroup_CapacityLimits {
            max_indexing_capacity_in_ocu: core::default::Default::default(),
            max_search_capacity_in_ocu: core::default::Default::default(),
            min_indexing_capacity_in_ocu: core::default::Default::default(),
            min_search_capacity_in_ocu: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchserverlessCollectionGroup_CapacityLimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchserverlessCollectionGroup_CapacityLimitsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchserverlessCollectionGroup_CapacityLimitsRef {
        DataOpensearchserverlessCollectionGroup_CapacityLimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchserverlessCollectionGroup_CapacityLimitsRef {
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
