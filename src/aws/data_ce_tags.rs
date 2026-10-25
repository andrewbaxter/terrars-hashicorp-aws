#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCeTagsData_ {
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
    search_string: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataCeTags_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort_by: std::option::Option<std::vec::Vec<DataCeTags_SortBy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_period: std::option::Option<std::vec::Vec<DataCeTags_TimePeriod>>,
    dynamic: DataCeTagsDynamic,
}

struct DataCeTags_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCeTagsData_>,
}

#[derive(Clone)]
pub struct DataCeTags(std::rc::Rc<DataCeTags_>);

impl DataCeTags {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search_string`.\n"]
    pub fn set_search_string(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().search_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_key`.\n"]
    pub fn set_tag_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tag_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::BlockAssignable<DataCeTags_Filter>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sort_by`.\n"]
    pub fn set_sort_by(self, v: impl std::convert::Into<terrars::BlockAssignable<DataCeTags_SortBy>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sort_by = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sort_by = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `time_period`.\n"]
    pub fn set_time_period(self, v: impl std::convert::Into<terrars::BlockAssignable<DataCeTags_TimePeriod>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().time_period = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.time_period = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `search_string` after provisioning.\n"]
    pub fn search_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search_string", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag_key` after provisioning.\n"]
    pub fn tag_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<DataCeTags_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort_by` after provisioning.\n"]
    pub fn sort_by(&self) -> terrars::ListRef<DataCeTags_SortByRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_period` after provisioning.\n"]
    pub fn time_period(&self) -> terrars::ListRef<DataCeTags_TimePeriodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_period", self.extract_ref()))
    }
}

impl terrars::Referable for DataCeTags {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCeTags { }

impl terrars::ToListMappable for DataCeTags {
    type O = terrars::ListRef<DataCeTagsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCeTags_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ce_tags".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCeTags {
    pub tf_id: std::string::String,
}

impl BuildDataCeTags {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCeTags {
        let out = DataCeTags(std::rc::Rc::new(DataCeTags_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCeTagsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                search_string: core::default::Default::default(),
                tag_key: core::default::Default::default(),
                filter: core::default::Default::default(),
                sort_by: core::default::Default::default(),
                time_period: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCeTagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTagsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCeTagsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `search_string` after provisioning.\n"]
    pub fn search_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search_string", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag_key` after provisioning.\n"]
    pub fn tag_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<DataCeTags_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort_by` after provisioning.\n"]
    pub fn sort_by(&self) -> terrars::ListRef<DataCeTags_SortByRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_period` after provisioning.\n"]
    pub fn time_period(&self) -> terrars::ListRef<DataCeTags_TimePeriodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_period", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataCeTagsDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataCeTags_Filter>>,
    sort_by: std::option::Option<terrars::DynamicBlock<DataCeTags_SortBy>>,
    time_period: std::option::Option<terrars::DynamicBlock<DataCeTags_TimePeriod>>,
}

#[derive(Serialize)]
pub struct DataCeTags_Filter_CostCategory {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeTags_Filter_CostCategory {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.match_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCeTags_Filter_CostCategory {
    type O = terrars::BlockAssignable<DataCeTags_Filter_CostCategory>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTags_Filter_CostCategory {}

impl BuildDataCeTags_Filter_CostCategory {
    pub fn build(self) -> DataCeTags_Filter_CostCategory {
        DataCeTags_Filter_CostCategory {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeTags_Filter_CostCategoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTags_Filter_CostCategoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTags_Filter_CostCategoryRef {
        DataCeTags_Filter_CostCategoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTags_Filter_CostCategoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeTags_Filter_Dimension {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeTags_Filter_Dimension {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.match_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCeTags_Filter_Dimension {
    type O = terrars::BlockAssignable<DataCeTags_Filter_Dimension>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTags_Filter_Dimension {}

impl BuildDataCeTags_Filter_Dimension {
    pub fn build(self) -> DataCeTags_Filter_Dimension {
        DataCeTags_Filter_Dimension {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeTags_Filter_DimensionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTags_Filter_DimensionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTags_Filter_DimensionRef {
        DataCeTags_Filter_DimensionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTags_Filter_DimensionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeTags_Filter_Tags {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeTags_Filter_Tags {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.match_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCeTags_Filter_Tags {
    type O = terrars::BlockAssignable<DataCeTags_Filter_Tags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTags_Filter_Tags {}

impl BuildDataCeTags_Filter_Tags {
    pub fn build(self) -> DataCeTags_Filter_Tags {
        DataCeTags_Filter_Tags {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeTags_Filter_TagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTags_Filter_TagsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTags_Filter_TagsRef {
        DataCeTags_Filter_TagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTags_Filter_TagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataCeTags_Filter_AndDynamic {
    cost_category: std::option::Option<terrars::DynamicBlock<DataCeTags_Filter_CostCategory>>,
    dimension: std::option::Option<terrars::DynamicBlock<DataCeTags_Filter_Dimension>>,
    tags: std::option::Option<terrars::DynamicBlock<DataCeTags_Filter_Tags>>,
}

#[derive(Serialize)]
pub struct DataCeTags_Filter_And {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<DataCeTags_Filter_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<DataCeTags_Filter_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<DataCeTags_Filter_Tags>>,
    dynamic: DataCeTags_Filter_AndDynamic,
}

impl DataCeTags_Filter_And {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataCeTags_Filter_CostCategory>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cost_category = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_category = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataCeTags_Filter_Dimension>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dimension = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl std::convert::Into<terrars::BlockAssignable<DataCeTags_Filter_Tags>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataCeTags_Filter_And {
    type O = terrars::BlockAssignable<DataCeTags_Filter_And>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTags_Filter_And {}

impl BuildDataCeTags_Filter_And {
    pub fn build(self) -> DataCeTags_Filter_And {
        DataCeTags_Filter_And {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeTags_Filter_AndRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTags_Filter_AndRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTags_Filter_AndRef {
        DataCeTags_Filter_AndRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTags_Filter_AndRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<DataCeTags_Filter_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<DataCeTags_Filter_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DataCeTags_Filter_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataCeTags_Filter_NotDynamic {
    cost_category: std::option::Option<terrars::DynamicBlock<DataCeTags_Filter_CostCategory>>,
    dimension: std::option::Option<terrars::DynamicBlock<DataCeTags_Filter_Dimension>>,
    tags: std::option::Option<terrars::DynamicBlock<DataCeTags_Filter_Tags>>,
}

#[derive(Serialize)]
pub struct DataCeTags_Filter_Not {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<DataCeTags_Filter_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<DataCeTags_Filter_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<DataCeTags_Filter_Tags>>,
    dynamic: DataCeTags_Filter_NotDynamic,
}

impl DataCeTags_Filter_Not {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataCeTags_Filter_CostCategory>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cost_category = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_category = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataCeTags_Filter_Dimension>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dimension = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl std::convert::Into<terrars::BlockAssignable<DataCeTags_Filter_Tags>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataCeTags_Filter_Not {
    type O = terrars::BlockAssignable<DataCeTags_Filter_Not>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTags_Filter_Not {}

impl BuildDataCeTags_Filter_Not {
    pub fn build(self) -> DataCeTags_Filter_Not {
        DataCeTags_Filter_Not {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeTags_Filter_NotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTags_Filter_NotRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTags_Filter_NotRef {
        DataCeTags_Filter_NotRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTags_Filter_NotRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<DataCeTags_Filter_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<DataCeTags_Filter_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DataCeTags_Filter_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataCeTags_Filter_OrDynamic {
    cost_category: std::option::Option<terrars::DynamicBlock<DataCeTags_Filter_CostCategory>>,
    dimension: std::option::Option<terrars::DynamicBlock<DataCeTags_Filter_Dimension>>,
    tags: std::option::Option<terrars::DynamicBlock<DataCeTags_Filter_Tags>>,
}

#[derive(Serialize)]
pub struct DataCeTags_Filter_Or {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<DataCeTags_Filter_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<DataCeTags_Filter_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<DataCeTags_Filter_Tags>>,
    dynamic: DataCeTags_Filter_OrDynamic,
}

impl DataCeTags_Filter_Or {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataCeTags_Filter_CostCategory>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cost_category = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_category = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataCeTags_Filter_Dimension>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dimension = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl std::convert::Into<terrars::BlockAssignable<DataCeTags_Filter_Tags>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataCeTags_Filter_Or {
    type O = terrars::BlockAssignable<DataCeTags_Filter_Or>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTags_Filter_Or {}

impl BuildDataCeTags_Filter_Or {
    pub fn build(self) -> DataCeTags_Filter_Or {
        DataCeTags_Filter_Or {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeTags_Filter_OrRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTags_Filter_OrRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTags_Filter_OrRef {
        DataCeTags_Filter_OrRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTags_Filter_OrRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<DataCeTags_Filter_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<DataCeTags_Filter_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DataCeTags_Filter_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataCeTags_FilterDynamic {
    and: std::option::Option<terrars::DynamicBlock<DataCeTags_Filter_And>>,
    cost_category: std::option::Option<terrars::DynamicBlock<DataCeTags_Filter_CostCategory>>,
    dimension: std::option::Option<terrars::DynamicBlock<DataCeTags_Filter_Dimension>>,
    not: std::option::Option<terrars::DynamicBlock<DataCeTags_Filter_Not>>,
    or: std::option::Option<terrars::DynamicBlock<DataCeTags_Filter_Or>>,
    tags: std::option::Option<terrars::DynamicBlock<DataCeTags_Filter_Tags>>,
}

#[derive(Serialize)]
pub struct DataCeTags_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<DataCeTags_Filter_And>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<DataCeTags_Filter_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<DataCeTags_Filter_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not: std::option::Option<std::vec::Vec<DataCeTags_Filter_Not>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or: std::option::Option<std::vec::Vec<DataCeTags_Filter_Or>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<DataCeTags_Filter_Tags>>,
    dynamic: DataCeTags_FilterDynamic,
}

impl DataCeTags_Filter {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(mut self, v: impl std::convert::Into<terrars::BlockAssignable<DataCeTags_Filter_And>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.and = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.and = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataCeTags_Filter_CostCategory>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cost_category = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_category = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataCeTags_Filter_Dimension>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dimension = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `not`.\n"]
    pub fn set_not(mut self, v: impl std::convert::Into<terrars::BlockAssignable<DataCeTags_Filter_Not>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.not = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.not = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `or`.\n"]
    pub fn set_or(mut self, v: impl std::convert::Into<terrars::BlockAssignable<DataCeTags_Filter_Or>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.or = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.or = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl std::convert::Into<terrars::BlockAssignable<DataCeTags_Filter_Tags>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataCeTags_Filter {
    type O = terrars::BlockAssignable<DataCeTags_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTags_Filter {}

impl BuildDataCeTags_Filter {
    pub fn build(self) -> DataCeTags_Filter {
        DataCeTags_Filter {
            and: core::default::Default::default(),
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            not: core::default::Default::default(),
            or: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeTags_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTags_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTags_FilterRef {
        DataCeTags_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTags_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<DataCeTags_Filter_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<DataCeTags_Filter_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> terrars::ListRef<DataCeTags_Filter_NotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DataCeTags_Filter_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeTags_SortBy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort_order: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCeTags_SortBy {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sort_order`.\n"]
    pub fn set_sort_order(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sort_order = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCeTags_SortBy {
    type O = terrars::BlockAssignable<DataCeTags_SortBy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTags_SortBy {}

impl BuildDataCeTags_SortBy {
    pub fn build(self) -> DataCeTags_SortBy {
        DataCeTags_SortBy {
            key: core::default::Default::default(),
            sort_order: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeTags_SortByRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTags_SortByRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTags_SortByRef {
        DataCeTags_SortByRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTags_SortByRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `sort_order` after provisioning.\n"]
    pub fn sort_order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sort_order", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeTags_TimePeriod {
    end: terrars::PrimField<std::string::String>,
    start: terrars::PrimField<std::string::String>,
}

impl DataCeTags_TimePeriod { }

impl terrars::ToListMappable for DataCeTags_TimePeriod {
    type O = terrars::BlockAssignable<DataCeTags_TimePeriod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTags_TimePeriod {
    #[doc = ""]
    pub end: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub start: terrars::PrimField<std::string::String>,
}

impl BuildDataCeTags_TimePeriod {
    pub fn build(self) -> DataCeTags_TimePeriod {
        DataCeTags_TimePeriod {
            end: self.end,
            start: self.start,
        }
    }
}

#[allow(dead_code)]
pub struct DataCeTags_TimePeriodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTags_TimePeriodRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTags_TimePeriodRef {
        DataCeTags_TimePeriodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTags_TimePeriodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
}
