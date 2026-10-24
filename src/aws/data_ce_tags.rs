use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCeTagsData {
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
    filter: std::option::Option<std::vec::Vec<DataCeTagsFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort_by: std::option::Option<std::vec::Vec<DataCeTagsSortByEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_period: std::option::Option<std::vec::Vec<DataCeTagsTimePeriodEl>>,
    dynamic: DataCeTagsDynamic,
}

struct DataCeTags_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCeTagsData>,
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
    pub fn set_filter(self, v: impl std::convert::Into<terrars::BlockAssignable<DataCeTagsFilterEl>>) -> Self {
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
    pub fn set_sort_by(self, v: impl std::convert::Into<terrars::BlockAssignable<DataCeTagsSortByEl>>) -> Self {
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
    pub fn set_time_period(self, v: impl std::convert::Into<terrars::BlockAssignable<DataCeTagsTimePeriodEl>>) -> Self {
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
    pub fn filter(&self) -> terrars::ListRef<DataCeTagsFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort_by` after provisioning.\n"]
    pub fn sort_by(&self) -> terrars::ListRef<DataCeTagsSortByElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_period` after provisioning.\n"]
    pub fn time_period(&self) -> terrars::ListRef<DataCeTagsTimePeriodElRef> {
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
            data: std::cell::RefCell::new(DataCeTagsData {
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
    pub fn filter(&self) -> terrars::ListRef<DataCeTagsFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort_by` after provisioning.\n"]
    pub fn sort_by(&self) -> terrars::ListRef<DataCeTagsSortByElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_period` after provisioning.\n"]
    pub fn time_period(&self) -> terrars::ListRef<DataCeTagsTimePeriodElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_period", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElAndElCostCategoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeTagsFilterElAndElCostCategoryEl {
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

impl terrars::ToListMappable for DataCeTagsFilterElAndElCostCategoryEl {
    type O = terrars::BlockAssignable<DataCeTagsFilterElAndElCostCategoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElAndElCostCategoryEl {}

impl BuildDataCeTagsFilterElAndElCostCategoryEl {
    pub fn build(self) -> DataCeTagsFilterElAndElCostCategoryEl {
        DataCeTagsFilterElAndElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElAndElCostCategoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTagsFilterElAndElCostCategoryElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTagsFilterElAndElCostCategoryElRef {
        DataCeTagsFilterElAndElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElAndElCostCategoryElRef {
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
pub struct DataCeTagsFilterElAndElDimensionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeTagsFilterElAndElDimensionEl {
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

impl terrars::ToListMappable for DataCeTagsFilterElAndElDimensionEl {
    type O = terrars::BlockAssignable<DataCeTagsFilterElAndElDimensionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElAndElDimensionEl {}

impl BuildDataCeTagsFilterElAndElDimensionEl {
    pub fn build(self) -> DataCeTagsFilterElAndElDimensionEl {
        DataCeTagsFilterElAndElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElAndElDimensionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTagsFilterElAndElDimensionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTagsFilterElAndElDimensionElRef {
        DataCeTagsFilterElAndElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElAndElDimensionElRef {
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
pub struct DataCeTagsFilterElAndElTagsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeTagsFilterElAndElTagsEl {
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

impl terrars::ToListMappable for DataCeTagsFilterElAndElTagsEl {
    type O = terrars::BlockAssignable<DataCeTagsFilterElAndElTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElAndElTagsEl {}

impl BuildDataCeTagsFilterElAndElTagsEl {
    pub fn build(self) -> DataCeTagsFilterElAndElTagsEl {
        DataCeTagsFilterElAndElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElAndElTagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTagsFilterElAndElTagsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTagsFilterElAndElTagsElRef {
        DataCeTagsFilterElAndElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElAndElTagsElRef {
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
struct DataCeTagsFilterElAndElDynamic {
    cost_category: std::option::Option<terrars::DynamicBlock<DataCeTagsFilterElAndElCostCategoryEl>>,
    dimension: std::option::Option<terrars::DynamicBlock<DataCeTagsFilterElAndElDimensionEl>>,
    tags: std::option::Option<terrars::DynamicBlock<DataCeTagsFilterElAndElTagsEl>>,
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElAndEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<DataCeTagsFilterElAndElCostCategoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<DataCeTagsFilterElAndElDimensionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<DataCeTagsFilterElAndElTagsEl>>,
    dynamic: DataCeTagsFilterElAndElDynamic,
}

impl DataCeTagsFilterElAndEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataCeTagsFilterElAndElCostCategoryEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataCeTagsFilterElAndElDimensionEl>>,
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
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataCeTagsFilterElAndElTagsEl>>,
    ) -> Self {
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

impl terrars::ToListMappable for DataCeTagsFilterElAndEl {
    type O = terrars::BlockAssignable<DataCeTagsFilterElAndEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElAndEl {}

impl BuildDataCeTagsFilterElAndEl {
    pub fn build(self) -> DataCeTagsFilterElAndEl {
        DataCeTagsFilterElAndEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElAndElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTagsFilterElAndElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTagsFilterElAndElRef {
        DataCeTagsFilterElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElAndElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<DataCeTagsFilterElAndElCostCategoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<DataCeTagsFilterElAndElDimensionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DataCeTagsFilterElAndElTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElCostCategoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeTagsFilterElCostCategoryEl {
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

impl terrars::ToListMappable for DataCeTagsFilterElCostCategoryEl {
    type O = terrars::BlockAssignable<DataCeTagsFilterElCostCategoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElCostCategoryEl {}

impl BuildDataCeTagsFilterElCostCategoryEl {
    pub fn build(self) -> DataCeTagsFilterElCostCategoryEl {
        DataCeTagsFilterElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElCostCategoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTagsFilterElCostCategoryElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTagsFilterElCostCategoryElRef {
        DataCeTagsFilterElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElCostCategoryElRef {
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
pub struct DataCeTagsFilterElDimensionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeTagsFilterElDimensionEl {
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

impl terrars::ToListMappable for DataCeTagsFilterElDimensionEl {
    type O = terrars::BlockAssignable<DataCeTagsFilterElDimensionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElDimensionEl {}

impl BuildDataCeTagsFilterElDimensionEl {
    pub fn build(self) -> DataCeTagsFilterElDimensionEl {
        DataCeTagsFilterElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElDimensionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTagsFilterElDimensionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTagsFilterElDimensionElRef {
        DataCeTagsFilterElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElDimensionElRef {
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
pub struct DataCeTagsFilterElNotElCostCategoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeTagsFilterElNotElCostCategoryEl {
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

impl terrars::ToListMappable for DataCeTagsFilterElNotElCostCategoryEl {
    type O = terrars::BlockAssignable<DataCeTagsFilterElNotElCostCategoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElNotElCostCategoryEl {}

impl BuildDataCeTagsFilterElNotElCostCategoryEl {
    pub fn build(self) -> DataCeTagsFilterElNotElCostCategoryEl {
        DataCeTagsFilterElNotElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElNotElCostCategoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTagsFilterElNotElCostCategoryElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTagsFilterElNotElCostCategoryElRef {
        DataCeTagsFilterElNotElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElNotElCostCategoryElRef {
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
pub struct DataCeTagsFilterElNotElDimensionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeTagsFilterElNotElDimensionEl {
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

impl terrars::ToListMappable for DataCeTagsFilterElNotElDimensionEl {
    type O = terrars::BlockAssignable<DataCeTagsFilterElNotElDimensionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElNotElDimensionEl {}

impl BuildDataCeTagsFilterElNotElDimensionEl {
    pub fn build(self) -> DataCeTagsFilterElNotElDimensionEl {
        DataCeTagsFilterElNotElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElNotElDimensionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTagsFilterElNotElDimensionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTagsFilterElNotElDimensionElRef {
        DataCeTagsFilterElNotElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElNotElDimensionElRef {
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
pub struct DataCeTagsFilterElNotElTagsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeTagsFilterElNotElTagsEl {
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

impl terrars::ToListMappable for DataCeTagsFilterElNotElTagsEl {
    type O = terrars::BlockAssignable<DataCeTagsFilterElNotElTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElNotElTagsEl {}

impl BuildDataCeTagsFilterElNotElTagsEl {
    pub fn build(self) -> DataCeTagsFilterElNotElTagsEl {
        DataCeTagsFilterElNotElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElNotElTagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTagsFilterElNotElTagsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTagsFilterElNotElTagsElRef {
        DataCeTagsFilterElNotElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElNotElTagsElRef {
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
struct DataCeTagsFilterElNotElDynamic {
    cost_category: std::option::Option<terrars::DynamicBlock<DataCeTagsFilterElNotElCostCategoryEl>>,
    dimension: std::option::Option<terrars::DynamicBlock<DataCeTagsFilterElNotElDimensionEl>>,
    tags: std::option::Option<terrars::DynamicBlock<DataCeTagsFilterElNotElTagsEl>>,
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElNotEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<DataCeTagsFilterElNotElCostCategoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<DataCeTagsFilterElNotElDimensionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<DataCeTagsFilterElNotElTagsEl>>,
    dynamic: DataCeTagsFilterElNotElDynamic,
}

impl DataCeTagsFilterElNotEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataCeTagsFilterElNotElCostCategoryEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataCeTagsFilterElNotElDimensionEl>>,
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
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataCeTagsFilterElNotElTagsEl>>,
    ) -> Self {
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

impl terrars::ToListMappable for DataCeTagsFilterElNotEl {
    type O = terrars::BlockAssignable<DataCeTagsFilterElNotEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElNotEl {}

impl BuildDataCeTagsFilterElNotEl {
    pub fn build(self) -> DataCeTagsFilterElNotEl {
        DataCeTagsFilterElNotEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElNotElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTagsFilterElNotElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTagsFilterElNotElRef {
        DataCeTagsFilterElNotElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElNotElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<DataCeTagsFilterElNotElCostCategoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<DataCeTagsFilterElNotElDimensionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DataCeTagsFilterElNotElTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElOrElCostCategoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeTagsFilterElOrElCostCategoryEl {
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

impl terrars::ToListMappable for DataCeTagsFilterElOrElCostCategoryEl {
    type O = terrars::BlockAssignable<DataCeTagsFilterElOrElCostCategoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElOrElCostCategoryEl {}

impl BuildDataCeTagsFilterElOrElCostCategoryEl {
    pub fn build(self) -> DataCeTagsFilterElOrElCostCategoryEl {
        DataCeTagsFilterElOrElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElOrElCostCategoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTagsFilterElOrElCostCategoryElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTagsFilterElOrElCostCategoryElRef {
        DataCeTagsFilterElOrElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElOrElCostCategoryElRef {
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
pub struct DataCeTagsFilterElOrElDimensionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeTagsFilterElOrElDimensionEl {
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

impl terrars::ToListMappable for DataCeTagsFilterElOrElDimensionEl {
    type O = terrars::BlockAssignable<DataCeTagsFilterElOrElDimensionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElOrElDimensionEl {}

impl BuildDataCeTagsFilterElOrElDimensionEl {
    pub fn build(self) -> DataCeTagsFilterElOrElDimensionEl {
        DataCeTagsFilterElOrElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElOrElDimensionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTagsFilterElOrElDimensionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTagsFilterElOrElDimensionElRef {
        DataCeTagsFilterElOrElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElOrElDimensionElRef {
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
pub struct DataCeTagsFilterElOrElTagsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeTagsFilterElOrElTagsEl {
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

impl terrars::ToListMappable for DataCeTagsFilterElOrElTagsEl {
    type O = terrars::BlockAssignable<DataCeTagsFilterElOrElTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElOrElTagsEl {}

impl BuildDataCeTagsFilterElOrElTagsEl {
    pub fn build(self) -> DataCeTagsFilterElOrElTagsEl {
        DataCeTagsFilterElOrElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElOrElTagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTagsFilterElOrElTagsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTagsFilterElOrElTagsElRef {
        DataCeTagsFilterElOrElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElOrElTagsElRef {
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
struct DataCeTagsFilterElOrElDynamic {
    cost_category: std::option::Option<terrars::DynamicBlock<DataCeTagsFilterElOrElCostCategoryEl>>,
    dimension: std::option::Option<terrars::DynamicBlock<DataCeTagsFilterElOrElDimensionEl>>,
    tags: std::option::Option<terrars::DynamicBlock<DataCeTagsFilterElOrElTagsEl>>,
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElOrEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<DataCeTagsFilterElOrElCostCategoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<DataCeTagsFilterElOrElDimensionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<DataCeTagsFilterElOrElTagsEl>>,
    dynamic: DataCeTagsFilterElOrElDynamic,
}

impl DataCeTagsFilterElOrEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataCeTagsFilterElOrElCostCategoryEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataCeTagsFilterElOrElDimensionEl>>,
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
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataCeTagsFilterElOrElTagsEl>>,
    ) -> Self {
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

impl terrars::ToListMappable for DataCeTagsFilterElOrEl {
    type O = terrars::BlockAssignable<DataCeTagsFilterElOrEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElOrEl {}

impl BuildDataCeTagsFilterElOrEl {
    pub fn build(self) -> DataCeTagsFilterElOrEl {
        DataCeTagsFilterElOrEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElOrElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTagsFilterElOrElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTagsFilterElOrElRef {
        DataCeTagsFilterElOrElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElOrElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<DataCeTagsFilterElOrElCostCategoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<DataCeTagsFilterElOrElDimensionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DataCeTagsFilterElOrElTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElTagsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeTagsFilterElTagsEl {
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

impl terrars::ToListMappable for DataCeTagsFilterElTagsEl {
    type O = terrars::BlockAssignable<DataCeTagsFilterElTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElTagsEl {}

impl BuildDataCeTagsFilterElTagsEl {
    pub fn build(self) -> DataCeTagsFilterElTagsEl {
        DataCeTagsFilterElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElTagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTagsFilterElTagsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTagsFilterElTagsElRef {
        DataCeTagsFilterElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElTagsElRef {
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
struct DataCeTagsFilterElDynamic {
    and: std::option::Option<terrars::DynamicBlock<DataCeTagsFilterElAndEl>>,
    cost_category: std::option::Option<terrars::DynamicBlock<DataCeTagsFilterElCostCategoryEl>>,
    dimension: std::option::Option<terrars::DynamicBlock<DataCeTagsFilterElDimensionEl>>,
    not: std::option::Option<terrars::DynamicBlock<DataCeTagsFilterElNotEl>>,
    or: std::option::Option<terrars::DynamicBlock<DataCeTagsFilterElOrEl>>,
    tags: std::option::Option<terrars::DynamicBlock<DataCeTagsFilterElTagsEl>>,
}

#[derive(Serialize)]
pub struct DataCeTagsFilterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<DataCeTagsFilterElAndEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<DataCeTagsFilterElCostCategoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<DataCeTagsFilterElDimensionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not: std::option::Option<std::vec::Vec<DataCeTagsFilterElNotEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or: std::option::Option<std::vec::Vec<DataCeTagsFilterElOrEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<DataCeTagsFilterElTagsEl>>,
    dynamic: DataCeTagsFilterElDynamic,
}

impl DataCeTagsFilterEl {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(mut self, v: impl std::convert::Into<terrars::BlockAssignable<DataCeTagsFilterElAndEl>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataCeTagsFilterElCostCategoryEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataCeTagsFilterElDimensionEl>>,
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
    pub fn set_not(mut self, v: impl std::convert::Into<terrars::BlockAssignable<DataCeTagsFilterElNotEl>>) -> Self {
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
    pub fn set_or(mut self, v: impl std::convert::Into<terrars::BlockAssignable<DataCeTagsFilterElOrEl>>) -> Self {
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
    pub fn set_tags(mut self, v: impl std::convert::Into<terrars::BlockAssignable<DataCeTagsFilterElTagsEl>>) -> Self {
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

impl terrars::ToListMappable for DataCeTagsFilterEl {
    type O = terrars::BlockAssignable<DataCeTagsFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterEl {}

impl BuildDataCeTagsFilterEl {
    pub fn build(self) -> DataCeTagsFilterEl {
        DataCeTagsFilterEl {
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

pub struct DataCeTagsFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTagsFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTagsFilterElRef {
        DataCeTagsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<DataCeTagsFilterElCostCategoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<DataCeTagsFilterElDimensionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> terrars::ListRef<DataCeTagsFilterElNotElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DataCeTagsFilterElTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeTagsSortByEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort_order: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCeTagsSortByEl {
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

impl terrars::ToListMappable for DataCeTagsSortByEl {
    type O = terrars::BlockAssignable<DataCeTagsSortByEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsSortByEl {}

impl BuildDataCeTagsSortByEl {
    pub fn build(self) -> DataCeTagsSortByEl {
        DataCeTagsSortByEl {
            key: core::default::Default::default(),
            sort_order: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsSortByElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTagsSortByElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTagsSortByElRef {
        DataCeTagsSortByElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsSortByElRef {
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
pub struct DataCeTagsTimePeriodEl {
    end: terrars::PrimField<std::string::String>,
    start: terrars::PrimField<std::string::String>,
}

impl DataCeTagsTimePeriodEl { }

impl terrars::ToListMappable for DataCeTagsTimePeriodEl {
    type O = terrars::BlockAssignable<DataCeTagsTimePeriodEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsTimePeriodEl {
    #[doc = ""]
    pub end: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub start: terrars::PrimField<std::string::String>,
}

impl BuildDataCeTagsTimePeriodEl {
    pub fn build(self) -> DataCeTagsTimePeriodEl {
        DataCeTagsTimePeriodEl {
            end: self.end,
            start: self.start,
        }
    }
}

pub struct DataCeTagsTimePeriodElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeTagsTimePeriodElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeTagsTimePeriodElRef {
        DataCeTagsTimePeriodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsTimePeriodElRef {
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

#[derive(Serialize, Default)]
struct DataCeTagsDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataCeTagsFilterEl>>,
    sort_by: std::option::Option<terrars::DynamicBlock<DataCeTagsSortByEl>>,
    time_period: std::option::Option<terrars::DynamicBlock<DataCeTagsTimePeriodEl>>,
}
