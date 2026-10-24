use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCeCostCategoryData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    cost_category_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataCeCostCategory_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCeCostCategoryData>,
}

#[derive(Clone)]
pub struct DataCeCostCategory(std::rc::Rc<DataCeCostCategory_>);

impl DataCeCostCategory {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `cost_category_arn` after provisioning.\n"]
    pub fn cost_category_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cost_category_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `effective_end` after provisioning.\n"]
    pub fn effective_end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_end", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `effective_start` after provisioning.\n"]
    pub fn effective_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::SetRef<DataCeCostCategoryRuleElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_version` after provisioning.\n"]
    pub fn rule_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `split_charge_rule` after provisioning.\n"]
    pub fn split_charge_rule(&self) -> terrars::SetRef<DataCeCostCategorySplitChargeRuleElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.split_charge_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataCeCostCategory {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCeCostCategory { }

impl terrars::ToListMappable for DataCeCostCategory {
    type O = terrars::ListRef<DataCeCostCategoryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCeCostCategory_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ce_cost_category".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCeCostCategory {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cost_category_arn: terrars::PrimField<std::string::String>,
}

impl BuildDataCeCostCategory {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCeCostCategory {
        let out = DataCeCostCategory(std::rc::Rc::new(DataCeCostCategory_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCeCostCategoryData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCeCostCategoryRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `cost_category_arn` after provisioning.\n"]
    pub fn cost_category_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cost_category_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `effective_end` after provisioning.\n"]
    pub fn effective_end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_end", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `effective_start` after provisioning.\n"]
    pub fn effective_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::SetRef<DataCeCostCategoryRuleElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_version` after provisioning.\n"]
    pub fn rule_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `split_charge_rule` after provisioning.\n"]
    pub fn split_charge_rule(&self) -> terrars::SetRef<DataCeCostCategorySplitChargeRuleElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.split_charge_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElInheritedValueEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCeCostCategoryRuleElInheritedValueEl {
    #[doc = "Set the field `dimension_key`.\n"]
    pub fn set_dimension_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dimension_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimension_name`.\n"]
    pub fn set_dimension_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dimension_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCeCostCategoryRuleElInheritedValueEl {
    type O = terrars::BlockAssignable<DataCeCostCategoryRuleElInheritedValueEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategoryRuleElInheritedValueElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategoryRuleElInheritedValueElRef {
        DataCeCostCategoryRuleElInheritedValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElInheritedValueElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dimension_key` after provisioning.\n"]
    pub fn dimension_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dimension_key", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension_name` after provisioning.\n"]
    pub fn dimension_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dimension_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElAndElCostCategoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeCostCategoryRuleElRuleElAndElCostCategoryEl {
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

impl terrars::ToListMappable for DataCeCostCategoryRuleElRuleElAndElCostCategoryEl {
    type O = terrars::BlockAssignable<DataCeCostCategoryRuleElRuleElAndElCostCategoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategoryRuleElRuleElAndElCostCategoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCeCostCategoryRuleElRuleElAndElCostCategoryElRef {
        DataCeCostCategoryRuleElRuleElAndElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElCostCategoryElRef {
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
pub struct DataCeCostCategoryRuleElRuleElAndElDimensionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeCostCategoryRuleElRuleElAndElDimensionEl {
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

impl terrars::ToListMappable for DataCeCostCategoryRuleElRuleElAndElDimensionEl {
    type O = terrars::BlockAssignable<DataCeCostCategoryRuleElRuleElAndElDimensionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategoryRuleElRuleElAndElDimensionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCeCostCategoryRuleElRuleElAndElDimensionElRef {
        DataCeCostCategoryRuleElRuleElAndElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElDimensionElRef {
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
pub struct DataCeCostCategoryRuleElRuleElAndElTagsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeCostCategoryRuleElRuleElAndElTagsEl {
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

impl terrars::ToListMappable for DataCeCostCategoryRuleElRuleElAndElTagsEl {
    type O = terrars::BlockAssignable<DataCeCostCategoryRuleElRuleElAndElTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategoryRuleElRuleElAndElTagsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategoryRuleElRuleElAndElTagsElRef {
        DataCeCostCategoryRuleElRuleElAndElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElTagsElRef {
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
pub struct DataCeCostCategoryRuleElRuleElAndEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<terrars::ListField<DataCeCostCategoryRuleElRuleElAndElCostCategoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<terrars::ListField<DataCeCostCategoryRuleElRuleElAndElDimensionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<DataCeCostCategoryRuleElRuleElAndElTagsEl>>,
}

impl DataCeCostCategoryRuleElRuleElAndEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategoryRuleElRuleElAndElCostCategoryEl>>,
    ) -> Self {
        self.cost_category = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategoryRuleElRuleElAndElDimensionEl>>,
    ) -> Self {
        self.dimension = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategoryRuleElRuleElAndElTagsEl>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCeCostCategoryRuleElRuleElAndEl {
    type O = terrars::BlockAssignable<DataCeCostCategoryRuleElRuleElAndEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategoryRuleElRuleElAndElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategoryRuleElRuleElAndElRef {
        DataCeCostCategoryRuleElRuleElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<DataCeCostCategoryRuleElRuleElAndElCostCategoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<DataCeCostCategoryRuleElRuleElAndElDimensionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DataCeCostCategoryRuleElRuleElAndElTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElCostCategoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeCostCategoryRuleElRuleElCostCategoryEl {
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

impl terrars::ToListMappable for DataCeCostCategoryRuleElRuleElCostCategoryEl {
    type O = terrars::BlockAssignable<DataCeCostCategoryRuleElRuleElCostCategoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategoryRuleElRuleElCostCategoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCeCostCategoryRuleElRuleElCostCategoryElRef {
        DataCeCostCategoryRuleElRuleElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElCostCategoryElRef {
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
pub struct DataCeCostCategoryRuleElRuleElDimensionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeCostCategoryRuleElRuleElDimensionEl {
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

impl terrars::ToListMappable for DataCeCostCategoryRuleElRuleElDimensionEl {
    type O = terrars::BlockAssignable<DataCeCostCategoryRuleElRuleElDimensionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategoryRuleElRuleElDimensionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategoryRuleElRuleElDimensionElRef {
        DataCeCostCategoryRuleElRuleElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElDimensionElRef {
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
pub struct DataCeCostCategoryRuleElRuleElNotElCostCategoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeCostCategoryRuleElRuleElNotElCostCategoryEl {
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

impl terrars::ToListMappable for DataCeCostCategoryRuleElRuleElNotElCostCategoryEl {
    type O = terrars::BlockAssignable<DataCeCostCategoryRuleElRuleElNotElCostCategoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategoryRuleElRuleElNotElCostCategoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCeCostCategoryRuleElRuleElNotElCostCategoryElRef {
        DataCeCostCategoryRuleElRuleElNotElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElCostCategoryElRef {
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
pub struct DataCeCostCategoryRuleElRuleElNotElDimensionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeCostCategoryRuleElRuleElNotElDimensionEl {
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

impl terrars::ToListMappable for DataCeCostCategoryRuleElRuleElNotElDimensionEl {
    type O = terrars::BlockAssignable<DataCeCostCategoryRuleElRuleElNotElDimensionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategoryRuleElRuleElNotElDimensionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCeCostCategoryRuleElRuleElNotElDimensionElRef {
        DataCeCostCategoryRuleElRuleElNotElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElDimensionElRef {
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
pub struct DataCeCostCategoryRuleElRuleElNotElTagsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeCostCategoryRuleElRuleElNotElTagsEl {
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

impl terrars::ToListMappable for DataCeCostCategoryRuleElRuleElNotElTagsEl {
    type O = terrars::BlockAssignable<DataCeCostCategoryRuleElRuleElNotElTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategoryRuleElRuleElNotElTagsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategoryRuleElRuleElNotElTagsElRef {
        DataCeCostCategoryRuleElRuleElNotElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElTagsElRef {
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
pub struct DataCeCostCategoryRuleElRuleElNotEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<terrars::ListField<DataCeCostCategoryRuleElRuleElNotElCostCategoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<terrars::ListField<DataCeCostCategoryRuleElRuleElNotElDimensionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<DataCeCostCategoryRuleElRuleElNotElTagsEl>>,
}

impl DataCeCostCategoryRuleElRuleElNotEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategoryRuleElRuleElNotElCostCategoryEl>>,
    ) -> Self {
        self.cost_category = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategoryRuleElRuleElNotElDimensionEl>>,
    ) -> Self {
        self.dimension = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategoryRuleElRuleElNotElTagsEl>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCeCostCategoryRuleElRuleElNotEl {
    type O = terrars::BlockAssignable<DataCeCostCategoryRuleElRuleElNotEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategoryRuleElRuleElNotElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategoryRuleElRuleElNotElRef {
        DataCeCostCategoryRuleElRuleElNotElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<DataCeCostCategoryRuleElRuleElNotElCostCategoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<DataCeCostCategoryRuleElRuleElNotElDimensionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DataCeCostCategoryRuleElRuleElNotElTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElOrElCostCategoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeCostCategoryRuleElRuleElOrElCostCategoryEl {
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

impl terrars::ToListMappable for DataCeCostCategoryRuleElRuleElOrElCostCategoryEl {
    type O = terrars::BlockAssignable<DataCeCostCategoryRuleElRuleElOrElCostCategoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategoryRuleElRuleElOrElCostCategoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCeCostCategoryRuleElRuleElOrElCostCategoryElRef {
        DataCeCostCategoryRuleElRuleElOrElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElCostCategoryElRef {
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
pub struct DataCeCostCategoryRuleElRuleElOrElDimensionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeCostCategoryRuleElRuleElOrElDimensionEl {
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

impl terrars::ToListMappable for DataCeCostCategoryRuleElRuleElOrElDimensionEl {
    type O = terrars::BlockAssignable<DataCeCostCategoryRuleElRuleElOrElDimensionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategoryRuleElRuleElOrElDimensionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCeCostCategoryRuleElRuleElOrElDimensionElRef {
        DataCeCostCategoryRuleElRuleElOrElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElDimensionElRef {
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
pub struct DataCeCostCategoryRuleElRuleElOrElTagsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeCostCategoryRuleElRuleElOrElTagsEl {
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

impl terrars::ToListMappable for DataCeCostCategoryRuleElRuleElOrElTagsEl {
    type O = terrars::BlockAssignable<DataCeCostCategoryRuleElRuleElOrElTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategoryRuleElRuleElOrElTagsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategoryRuleElRuleElOrElTagsElRef {
        DataCeCostCategoryRuleElRuleElOrElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElTagsElRef {
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
pub struct DataCeCostCategoryRuleElRuleElOrEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<terrars::ListField<DataCeCostCategoryRuleElRuleElOrElCostCategoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<terrars::ListField<DataCeCostCategoryRuleElRuleElOrElDimensionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<DataCeCostCategoryRuleElRuleElOrElTagsEl>>,
}

impl DataCeCostCategoryRuleElRuleElOrEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategoryRuleElRuleElOrElCostCategoryEl>>,
    ) -> Self {
        self.cost_category = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategoryRuleElRuleElOrElDimensionEl>>,
    ) -> Self {
        self.dimension = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategoryRuleElRuleElOrElTagsEl>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCeCostCategoryRuleElRuleElOrEl {
    type O = terrars::BlockAssignable<DataCeCostCategoryRuleElRuleElOrEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategoryRuleElRuleElOrElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategoryRuleElRuleElOrElRef {
        DataCeCostCategoryRuleElRuleElOrElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<DataCeCostCategoryRuleElRuleElOrElCostCategoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<DataCeCostCategoryRuleElRuleElOrElDimensionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DataCeCostCategoryRuleElRuleElOrElTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElTagsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeCostCategoryRuleElRuleElTagsEl {
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

impl terrars::ToListMappable for DataCeCostCategoryRuleElRuleElTagsEl {
    type O = terrars::BlockAssignable<DataCeCostCategoryRuleElRuleElTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategoryRuleElRuleElTagsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategoryRuleElRuleElTagsElRef {
        DataCeCostCategoryRuleElRuleElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElTagsElRef {
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
pub struct DataCeCostCategoryRuleElRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<terrars::SetField<DataCeCostCategoryRuleElRuleElAndEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<terrars::ListField<DataCeCostCategoryRuleElRuleElCostCategoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<terrars::ListField<DataCeCostCategoryRuleElRuleElDimensionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not: std::option::Option<terrars::ListField<DataCeCostCategoryRuleElRuleElNotEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or: std::option::Option<terrars::SetField<DataCeCostCategoryRuleElRuleElOrEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<DataCeCostCategoryRuleElRuleElTagsEl>>,
}

impl DataCeCostCategoryRuleElRuleEl {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataCeCostCategoryRuleElRuleElAndEl>>,
    ) -> Self {
        self.and = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategoryRuleElRuleElCostCategoryEl>>,
    ) -> Self {
        self.cost_category = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategoryRuleElRuleElDimensionEl>>,
    ) -> Self {
        self.dimension = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not`.\n"]
    pub fn set_not(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategoryRuleElRuleElNotEl>>,
    ) -> Self {
        self.not = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `or`.\n"]
    pub fn set_or(mut self, v: impl std::convert::Into<terrars::SetField<DataCeCostCategoryRuleElRuleElOrEl>>) -> Self {
        self.or = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategoryRuleElRuleElTagsEl>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCeCostCategoryRuleElRuleEl {
    type O = terrars::BlockAssignable<DataCeCostCategoryRuleElRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategoryRuleElRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategoryRuleElRuleElRef {
        DataCeCostCategoryRuleElRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::SetRef<DataCeCostCategoryRuleElRuleElAndElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.and", self.base))
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<DataCeCostCategoryRuleElRuleElCostCategoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<DataCeCostCategoryRuleElRuleElDimensionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> terrars::ListRef<DataCeCostCategoryRuleElRuleElNotElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `or` after provisioning.\n"]
    pub fn or(&self) -> terrars::SetRef<DataCeCostCategoryRuleElRuleElOrElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.or", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DataCeCostCategoryRuleElRuleElTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inherited_value: std::option::Option<terrars::ListField<DataCeCostCategoryRuleElInheritedValueEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<terrars::ListField<DataCeCostCategoryRuleElRuleEl>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCeCostCategoryRuleEl {
    #[doc = "Set the field `inherited_value`.\n"]
    pub fn set_inherited_value(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategoryRuleElInheritedValueEl>>,
    ) -> Self {
        self.inherited_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl std::convert::Into<terrars::ListField<DataCeCostCategoryRuleElRuleEl>>) -> Self {
        self.rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCeCostCategoryRuleEl {
    type O = terrars::BlockAssignable<DataCeCostCategoryRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategoryRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategoryRuleElRef {
        DataCeCostCategoryRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `inherited_value` after provisioning.\n"]
    pub fn inherited_value(&self) -> terrars::ListRef<DataCeCostCategoryRuleElInheritedValueElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inherited_value", self.base))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<DataCeCostCategoryRuleElRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategorySplitChargeRuleElParameterEl {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeCostCategorySplitChargeRuleElParameterEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for DataCeCostCategorySplitChargeRuleElParameterEl {
    type O = terrars::BlockAssignable<DataCeCostCategorySplitChargeRuleElParameterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategorySplitChargeRuleElParameterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCeCostCategorySplitChargeRuleElParameterElRef {
        DataCeCostCategorySplitChargeRuleElParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategorySplitChargeRuleElParameterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategorySplitChargeRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<terrars::SetField<DataCeCostCategorySplitChargeRuleElParameterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    targets: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeCostCategorySplitChargeRuleEl {
    #[doc = "Set the field `method`.\n"]
    pub fn set_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataCeCostCategorySplitChargeRuleElParameterEl>>,
    ) -> Self {
        self.parameter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `targets`.\n"]
    pub fn set_targets(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.targets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCeCostCategorySplitChargeRuleEl {
    type O = terrars::BlockAssignable<DataCeCostCategorySplitChargeRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategorySplitChargeRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategorySplitChargeRuleElRef {
        DataCeCostCategorySplitChargeRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategorySplitChargeRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc = "Get a reference to the value of field `parameter` after provisioning.\n"]
    pub fn parameter(&self) -> terrars::SetRef<DataCeCostCategorySplitChargeRuleElParameterElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.parameter", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.targets", self.base))
    }
}
