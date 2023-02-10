use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCeCostCategoryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cost_category_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataCeCostCategory_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCeCostCategoryData>,
}

#[derive(Clone)]
pub struct DataCeCostCategory(Rc<DataCeCostCategory_>);

impl DataCeCostCategory {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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

    #[doc= "Get a reference to the value of field `cost_category_arn` after provisioning.\n"]
    pub fn cost_category_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cost_category_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_end` after provisioning.\n"]
    pub fn effective_end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.effective_end", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_start` after provisioning.\n"]
    pub fn effective_start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.effective_start", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> SetRef<DataCeCostCategoryRuleElRef> {
        SetRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rule_version` after provisioning.\n"]
    pub fn rule_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `split_charge_rule` after provisioning.\n"]
    pub fn split_charge_rule(&self) -> SetRef<DataCeCostCategorySplitChargeRuleElRef> {
        SetRef::new(self.shared().clone(), format!("{}.split_charge_rule", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl Referable for DataCeCostCategory {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataCeCostCategory { }

impl ToListMappable for DataCeCostCategory {
    type O = ListRef<DataCeCostCategoryRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCeCostCategory_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ce_cost_category".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCeCostCategory {
    pub tf_id: String,
    #[doc= ""]
    pub cost_category_arn: PrimField<String>,
}

impl BuildDataCeCostCategory {
    pub fn build(self, stack: &mut Stack) -> DataCeCostCategory {
        let out = DataCeCostCategory(Rc::new(DataCeCostCategory_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCeCostCategoryData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                cost_category_arn: self.cost_category_arn,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCeCostCategoryRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCeCostCategoryRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `cost_category_arn` after provisioning.\n"]
    pub fn cost_category_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cost_category_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_end` after provisioning.\n"]
    pub fn effective_end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.effective_end", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_start` after provisioning.\n"]
    pub fn effective_start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.effective_start", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> SetRef<DataCeCostCategoryRuleElRef> {
        SetRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rule_version` after provisioning.\n"]
    pub fn rule_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `split_charge_rule` after provisioning.\n"]
    pub fn split_charge_rule(&self) -> SetRef<DataCeCostCategorySplitChargeRuleElRef> {
        SetRef::new(self.shared().clone(), format!("{}.split_charge_rule", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElInheritedValueEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension_name: Option<PrimField<String>>,
}

impl DataCeCostCategoryRuleElInheritedValueEl {
    #[doc= "Set the field `dimension_key`.\n"]
    pub fn set_dimension_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dimension_key = Some(v.into());
        self
    }

    #[doc= "Set the field `dimension_name`.\n"]
    pub fn set_dimension_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dimension_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElInheritedValueEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElInheritedValueEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElInheritedValueEl {}

impl BuildDataCeCostCategoryRuleElInheritedValueEl {
    pub fn build(self) -> DataCeCostCategoryRuleElInheritedValueEl {
        DataCeCostCategoryRuleElInheritedValueEl {
            dimension_key: core::default::Default::default(),
            dimension_name: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElInheritedValueElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElInheritedValueElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElInheritedValueElRef {
        DataCeCostCategoryRuleElInheritedValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElInheritedValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dimension_key` after provisioning.\n"]
    pub fn dimension_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dimension_key", self.base))
    }

    #[doc= "Get a reference to the value of field `dimension_name` after provisioning.\n"]
    pub fn dimension_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dimension_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElAndElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElAndElCostCategoryEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElAndElCostCategoryEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElAndElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElAndElCostCategoryEl {}

impl BuildDataCeCostCategoryRuleElRuleElAndElCostCategoryEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElAndElCostCategoryEl {
        DataCeCostCategoryRuleElRuleElAndElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElAndElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElAndElCostCategoryElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElAndElCostCategoryElRef {
        DataCeCostCategoryRuleElRuleElAndElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElCostCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElAndElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElAndElDimensionEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElAndElDimensionEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElAndElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElAndElDimensionEl {}

impl BuildDataCeCostCategoryRuleElRuleElAndElDimensionEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElAndElDimensionEl {
        DataCeCostCategoryRuleElRuleElAndElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElAndElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElAndElDimensionElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElAndElDimensionElRef {
        DataCeCostCategoryRuleElRuleElAndElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElAndElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElAndElTagsEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElAndElTagsEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElAndElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElAndElTagsEl {}

impl BuildDataCeCostCategoryRuleElRuleElAndElTagsEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElAndElTagsEl {
        DataCeCostCategoryRuleElRuleElAndElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElAndElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElAndElTagsElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElAndElTagsElRef {
        DataCeCostCategoryRuleElRuleElAndElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElAndEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<ListField<DataCeCostCategoryRuleElRuleElAndElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<ListField<DataCeCostCategoryRuleElRuleElAndElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<DataCeCostCategoryRuleElRuleElAndElTagsEl>>,
}

impl DataCeCostCategoryRuleElRuleElAndEl {
    #[doc= "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElAndElCostCategoryEl>>,
    ) -> Self {
        self.cost_category = Some(v.into());
        self
    }

    #[doc= "Set the field `dimension`.\n"]
    pub fn set_dimension(mut self, v: impl Into<ListField<DataCeCostCategoryRuleElRuleElAndElDimensionEl>>) -> Self {
        self.dimension = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<ListField<DataCeCostCategoryRuleElRuleElAndElTagsEl>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElAndEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElAndEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElAndEl {}

impl BuildDataCeCostCategoryRuleElRuleElAndEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElAndEl {
        DataCeCostCategoryRuleElRuleElAndEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElAndElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElAndElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElAndElRef {
        DataCeCostCategoryRuleElRuleElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> ListRef<DataCeCostCategoryRuleElRuleElAndElCostCategoryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc= "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<DataCeCostCategoryRuleElRuleElAndElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataCeCostCategoryRuleElRuleElAndElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElCostCategoryEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElCostCategoryEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElCostCategoryEl {}

impl BuildDataCeCostCategoryRuleElRuleElCostCategoryEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElCostCategoryEl {
        DataCeCostCategoryRuleElRuleElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElCostCategoryElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElCostCategoryElRef {
        DataCeCostCategoryRuleElRuleElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElCostCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElDimensionEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElDimensionEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElDimensionEl {}

impl BuildDataCeCostCategoryRuleElRuleElDimensionEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElDimensionEl {
        DataCeCostCategoryRuleElRuleElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElDimensionElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElDimensionElRef {
        DataCeCostCategoryRuleElRuleElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElNotElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElNotElCostCategoryEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElNotElCostCategoryEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElNotElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElNotElCostCategoryEl {}

impl BuildDataCeCostCategoryRuleElRuleElNotElCostCategoryEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElNotElCostCategoryEl {
        DataCeCostCategoryRuleElRuleElNotElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElNotElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElNotElCostCategoryElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElNotElCostCategoryElRef {
        DataCeCostCategoryRuleElRuleElNotElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElCostCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElNotElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElNotElDimensionEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElNotElDimensionEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElNotElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElNotElDimensionEl {}

impl BuildDataCeCostCategoryRuleElRuleElNotElDimensionEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElNotElDimensionEl {
        DataCeCostCategoryRuleElRuleElNotElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElNotElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElNotElDimensionElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElNotElDimensionElRef {
        DataCeCostCategoryRuleElRuleElNotElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElNotElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElNotElTagsEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElNotElTagsEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElNotElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElNotElTagsEl {}

impl BuildDataCeCostCategoryRuleElRuleElNotElTagsEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElNotElTagsEl {
        DataCeCostCategoryRuleElRuleElNotElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElNotElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElNotElTagsElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElNotElTagsElRef {
        DataCeCostCategoryRuleElRuleElNotElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElNotEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<ListField<DataCeCostCategoryRuleElRuleElNotElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<ListField<DataCeCostCategoryRuleElRuleElNotElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<DataCeCostCategoryRuleElRuleElNotElTagsEl>>,
}

impl DataCeCostCategoryRuleElRuleElNotEl {
    #[doc= "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElNotElCostCategoryEl>>,
    ) -> Self {
        self.cost_category = Some(v.into());
        self
    }

    #[doc= "Set the field `dimension`.\n"]
    pub fn set_dimension(mut self, v: impl Into<ListField<DataCeCostCategoryRuleElRuleElNotElDimensionEl>>) -> Self {
        self.dimension = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<ListField<DataCeCostCategoryRuleElRuleElNotElTagsEl>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElNotEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElNotEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElNotEl {}

impl BuildDataCeCostCategoryRuleElRuleElNotEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElNotEl {
        DataCeCostCategoryRuleElRuleElNotEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElNotElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElNotElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElNotElRef {
        DataCeCostCategoryRuleElRuleElNotElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> ListRef<DataCeCostCategoryRuleElRuleElNotElCostCategoryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc= "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<DataCeCostCategoryRuleElRuleElNotElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataCeCostCategoryRuleElRuleElNotElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElOrElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElOrElCostCategoryEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElOrElCostCategoryEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElOrElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElOrElCostCategoryEl {}

impl BuildDataCeCostCategoryRuleElRuleElOrElCostCategoryEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElOrElCostCategoryEl {
        DataCeCostCategoryRuleElRuleElOrElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElOrElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElOrElCostCategoryElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElOrElCostCategoryElRef {
        DataCeCostCategoryRuleElRuleElOrElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElCostCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElOrElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElOrElDimensionEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElOrElDimensionEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElOrElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElOrElDimensionEl {}

impl BuildDataCeCostCategoryRuleElRuleElOrElDimensionEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElOrElDimensionEl {
        DataCeCostCategoryRuleElRuleElOrElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElOrElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElOrElDimensionElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElOrElDimensionElRef {
        DataCeCostCategoryRuleElRuleElOrElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElOrElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElOrElTagsEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElOrElTagsEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElOrElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElOrElTagsEl {}

impl BuildDataCeCostCategoryRuleElRuleElOrElTagsEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElOrElTagsEl {
        DataCeCostCategoryRuleElRuleElOrElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElOrElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElOrElTagsElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElOrElTagsElRef {
        DataCeCostCategoryRuleElRuleElOrElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElOrEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<ListField<DataCeCostCategoryRuleElRuleElOrElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<ListField<DataCeCostCategoryRuleElRuleElOrElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<DataCeCostCategoryRuleElRuleElOrElTagsEl>>,
}

impl DataCeCostCategoryRuleElRuleElOrEl {
    #[doc= "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElOrElCostCategoryEl>>,
    ) -> Self {
        self.cost_category = Some(v.into());
        self
    }

    #[doc= "Set the field `dimension`.\n"]
    pub fn set_dimension(mut self, v: impl Into<ListField<DataCeCostCategoryRuleElRuleElOrElDimensionEl>>) -> Self {
        self.dimension = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<ListField<DataCeCostCategoryRuleElRuleElOrElTagsEl>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElOrEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElOrEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElOrEl {}

impl BuildDataCeCostCategoryRuleElRuleElOrEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElOrEl {
        DataCeCostCategoryRuleElRuleElOrEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElOrElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElOrElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElOrElRef {
        DataCeCostCategoryRuleElRuleElOrElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> ListRef<DataCeCostCategoryRuleElRuleElOrElCostCategoryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc= "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<DataCeCostCategoryRuleElRuleElOrElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataCeCostCategoryRuleElRuleElOrElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElTagsEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElTagsEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElTagsEl {}

impl BuildDataCeCostCategoryRuleElRuleElTagsEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElTagsEl {
        DataCeCostCategoryRuleElRuleElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElTagsElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElTagsElRef {
        DataCeCostCategoryRuleElRuleElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<SetField<DataCeCostCategoryRuleElRuleElAndEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<ListField<DataCeCostCategoryRuleElRuleElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<ListField<DataCeCostCategoryRuleElRuleElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not: Option<ListField<DataCeCostCategoryRuleElRuleElNotEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    or: Option<SetField<DataCeCostCategoryRuleElRuleElOrEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<DataCeCostCategoryRuleElRuleElTagsEl>>,
}

impl DataCeCostCategoryRuleElRuleEl {
    #[doc= "Set the field `and`.\n"]
    pub fn set_and(mut self, v: impl Into<SetField<DataCeCostCategoryRuleElRuleElAndEl>>) -> Self {
        self.and = Some(v.into());
        self
    }

    #[doc= "Set the field `cost_category`.\n"]
    pub fn set_cost_category(mut self, v: impl Into<ListField<DataCeCostCategoryRuleElRuleElCostCategoryEl>>) -> Self {
        self.cost_category = Some(v.into());
        self
    }

    #[doc= "Set the field `dimension`.\n"]
    pub fn set_dimension(mut self, v: impl Into<ListField<DataCeCostCategoryRuleElRuleElDimensionEl>>) -> Self {
        self.dimension = Some(v.into());
        self
    }

    #[doc= "Set the field `not`.\n"]
    pub fn set_not(mut self, v: impl Into<ListField<DataCeCostCategoryRuleElRuleElNotEl>>) -> Self {
        self.not = Some(v.into());
        self
    }

    #[doc= "Set the field `or`.\n"]
    pub fn set_or(mut self, v: impl Into<SetField<DataCeCostCategoryRuleElRuleElOrEl>>) -> Self {
        self.or = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<ListField<DataCeCostCategoryRuleElRuleElTagsEl>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleEl {}

impl BuildDataCeCostCategoryRuleElRuleEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleEl {
        DataCeCostCategoryRuleElRuleEl {
            and: core::default::Default::default(),
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            not: core::default::Default::default(),
            or: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElRef {
        DataCeCostCategoryRuleElRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> SetRef<DataCeCostCategoryRuleElRuleElAndElRef> {
        SetRef::new(self.shared().clone(), format!("{}.and", self.base))
    }

    #[doc= "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> ListRef<DataCeCostCategoryRuleElRuleElCostCategoryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc= "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<DataCeCostCategoryRuleElRuleElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc= "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> ListRef<DataCeCostCategoryRuleElRuleElNotElRef> {
        ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc= "Get a reference to the value of field `or` after provisioning.\n"]
    pub fn or(&self) -> SetRef<DataCeCostCategoryRuleElRuleElOrElRef> {
        SetRef::new(self.shared().clone(), format!("{}.or", self.base))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataCeCostCategoryRuleElRuleElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    inherited_value: Option<ListField<DataCeCostCategoryRuleElInheritedValueEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<ListField<DataCeCostCategoryRuleElRuleEl>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataCeCostCategoryRuleEl {
    #[doc= "Set the field `inherited_value`.\n"]
    pub fn set_inherited_value(mut self, v: impl Into<ListField<DataCeCostCategoryRuleElInheritedValueEl>>) -> Self {
        self.inherited_value = Some(v.into());
        self
    }

    #[doc= "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl Into<ListField<DataCeCostCategoryRuleElRuleEl>>) -> Self {
        self.rule = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleEl {
    type O = BlockAssignable<DataCeCostCategoryRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleEl {}

impl BuildDataCeCostCategoryRuleEl {
    pub fn build(self) -> DataCeCostCategoryRuleEl {
        DataCeCostCategoryRuleEl {
            inherited_value: core::default::Default::default(),
            rule: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRef {
        DataCeCostCategoryRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `inherited_value` after provisioning.\n"]
    pub fn inherited_value(&self) -> ListRef<DataCeCostCategoryRuleElInheritedValueElRef> {
        ListRef::new(self.shared().clone(), format!("{}.inherited_value", self.base))
    }

    #[doc= "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<DataCeCostCategoryRuleElRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategorySplitChargeRuleElParameterEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategorySplitChargeRuleElParameterEl {
    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategorySplitChargeRuleElParameterEl {
    type O = BlockAssignable<DataCeCostCategorySplitChargeRuleElParameterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategorySplitChargeRuleElParameterEl {}

impl BuildDataCeCostCategorySplitChargeRuleElParameterEl {
    pub fn build(self) -> DataCeCostCategorySplitChargeRuleElParameterEl {
        DataCeCostCategorySplitChargeRuleElParameterEl {
            type_: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategorySplitChargeRuleElParameterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategorySplitChargeRuleElParameterElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategorySplitChargeRuleElParameterElRef {
        DataCeCostCategorySplitChargeRuleElParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategorySplitChargeRuleElParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategorySplitChargeRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<SetField<DataCeCostCategorySplitChargeRuleElParameterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    targets: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategorySplitChargeRuleEl {
    #[doc= "Set the field `method`.\n"]
    pub fn set_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.method = Some(v.into());
        self
    }

    #[doc= "Set the field `parameter`.\n"]
    pub fn set_parameter(mut self, v: impl Into<SetField<DataCeCostCategorySplitChargeRuleElParameterEl>>) -> Self {
        self.parameter = Some(v.into());
        self
    }

    #[doc= "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source = Some(v.into());
        self
    }

    #[doc= "Set the field `targets`.\n"]
    pub fn set_targets(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.targets = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategorySplitChargeRuleEl {
    type O = BlockAssignable<DataCeCostCategorySplitChargeRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategorySplitChargeRuleEl {}

impl BuildDataCeCostCategorySplitChargeRuleEl {
    pub fn build(self) -> DataCeCostCategorySplitChargeRuleEl {
        DataCeCostCategorySplitChargeRuleEl {
            method: core::default::Default::default(),
            parameter: core::default::Default::default(),
            source: core::default::Default::default(),
            targets: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategorySplitChargeRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategorySplitChargeRuleElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategorySplitChargeRuleElRef {
        DataCeCostCategorySplitChargeRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategorySplitChargeRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc= "Get a reference to the value of field `parameter` after provisioning.\n"]
    pub fn parameter(&self) -> SetRef<DataCeCostCategorySplitChargeRuleElParameterElRef> {
        SetRef::new(self.shared().clone(), format!("{}.parameter", self.base))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc= "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.targets", self.base))
    }
}
