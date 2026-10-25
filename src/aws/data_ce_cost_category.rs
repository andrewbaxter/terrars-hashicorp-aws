#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCeCostCategoryData_ {
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
    data: std::cell::RefCell<DataCeCostCategoryData_>,
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

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.extract_ref()))
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
    pub fn rule(&self) -> terrars::SetRef<DataCeCostCategory_RuleRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_version` after provisioning.\n"]
    pub fn rule_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `split_charge_rule` after provisioning.\n"]
    pub fn split_charge_rule(&self) -> terrars::SetRef<DataCeCostCategory_SplitChargeRuleRef> {
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
            data: std::cell::RefCell::new(DataCeCostCategoryData_ {
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

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.extract_ref()))
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
    pub fn rule(&self) -> terrars::SetRef<DataCeCostCategory_RuleRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_version` after provisioning.\n"]
    pub fn rule_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `split_charge_rule` after provisioning.\n"]
    pub fn split_charge_rule(&self) -> terrars::SetRef<DataCeCostCategory_SplitChargeRuleRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.split_charge_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategory_Rule_InheritedValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCeCostCategory_Rule_InheritedValue {
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

impl terrars::ToListMappable for DataCeCostCategory_Rule_InheritedValue {
    type O = terrars::BlockAssignable<DataCeCostCategory_Rule_InheritedValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategory_Rule_InheritedValue {}

impl BuildDataCeCostCategory_Rule_InheritedValue {
    pub fn build(self) -> DataCeCostCategory_Rule_InheritedValue {
        DataCeCostCategory_Rule_InheritedValue {
            dimension_key: core::default::Default::default(),
            dimension_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeCostCategory_Rule_InheritedValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategory_Rule_InheritedValueRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategory_Rule_InheritedValueRef {
        DataCeCostCategory_Rule_InheritedValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategory_Rule_InheritedValueRef {
    #[allow(dead_code)]
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
pub struct DataCeCostCategory_Rule_Rule_CostCategory {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeCostCategory_Rule_Rule_CostCategory {
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

impl terrars::ToListMappable for DataCeCostCategory_Rule_Rule_CostCategory {
    type O = terrars::BlockAssignable<DataCeCostCategory_Rule_Rule_CostCategory>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategory_Rule_Rule_CostCategory {}

impl BuildDataCeCostCategory_Rule_Rule_CostCategory {
    pub fn build(self) -> DataCeCostCategory_Rule_Rule_CostCategory {
        DataCeCostCategory_Rule_Rule_CostCategory {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeCostCategory_Rule_Rule_CostCategoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategory_Rule_Rule_CostCategoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategory_Rule_Rule_CostCategoryRef {
        DataCeCostCategory_Rule_Rule_CostCategoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategory_Rule_Rule_CostCategoryRef {
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
pub struct DataCeCostCategory_Rule_Rule_Dimension {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeCostCategory_Rule_Rule_Dimension {
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

impl terrars::ToListMappable for DataCeCostCategory_Rule_Rule_Dimension {
    type O = terrars::BlockAssignable<DataCeCostCategory_Rule_Rule_Dimension>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategory_Rule_Rule_Dimension {}

impl BuildDataCeCostCategory_Rule_Rule_Dimension {
    pub fn build(self) -> DataCeCostCategory_Rule_Rule_Dimension {
        DataCeCostCategory_Rule_Rule_Dimension {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeCostCategory_Rule_Rule_DimensionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategory_Rule_Rule_DimensionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategory_Rule_Rule_DimensionRef {
        DataCeCostCategory_Rule_Rule_DimensionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategory_Rule_Rule_DimensionRef {
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
pub struct DataCeCostCategory_Rule_Rule_Tags {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeCostCategory_Rule_Rule_Tags {
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

impl terrars::ToListMappable for DataCeCostCategory_Rule_Rule_Tags {
    type O = terrars::BlockAssignable<DataCeCostCategory_Rule_Rule_Tags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategory_Rule_Rule_Tags {}

impl BuildDataCeCostCategory_Rule_Rule_Tags {
    pub fn build(self) -> DataCeCostCategory_Rule_Rule_Tags {
        DataCeCostCategory_Rule_Rule_Tags {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeCostCategory_Rule_Rule_TagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategory_Rule_Rule_TagsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategory_Rule_Rule_TagsRef {
        DataCeCostCategory_Rule_Rule_TagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategory_Rule_Rule_TagsRef {
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
pub struct DataCeCostCategory_Rule_Rule_Or_And {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_Tags>>,
}

impl DataCeCostCategory_Rule_Rule_Or_And {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_CostCategory>>,
    ) -> Self {
        self.cost_category = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_Dimension>>,
    ) -> Self {
        self.dimension = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_Tags>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCeCostCategory_Rule_Rule_Or_And {
    type O = terrars::BlockAssignable<DataCeCostCategory_Rule_Rule_Or_And>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategory_Rule_Rule_Or_And {}

impl BuildDataCeCostCategory_Rule_Rule_Or_And {
    pub fn build(self) -> DataCeCostCategory_Rule_Rule_Or_And {
        DataCeCostCategory_Rule_Rule_Or_And {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeCostCategory_Rule_Rule_Or_AndRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategory_Rule_Rule_Or_AndRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategory_Rule_Rule_Or_AndRef {
        DataCeCostCategory_Rule_Rule_Or_AndRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategory_Rule_Rule_Or_AndRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategory_Rule_Rule_Or_Not {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_Tags>>,
}

impl DataCeCostCategory_Rule_Rule_Or_Not {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_CostCategory>>,
    ) -> Self {
        self.cost_category = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_Dimension>>,
    ) -> Self {
        self.dimension = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_Tags>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCeCostCategory_Rule_Rule_Or_Not {
    type O = terrars::BlockAssignable<DataCeCostCategory_Rule_Rule_Or_Not>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategory_Rule_Rule_Or_Not {}

impl BuildDataCeCostCategory_Rule_Rule_Or_Not {
    pub fn build(self) -> DataCeCostCategory_Rule_Rule_Or_Not {
        DataCeCostCategory_Rule_Rule_Or_Not {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeCostCategory_Rule_Rule_Or_NotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategory_Rule_Rule_Or_NotRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategory_Rule_Rule_Or_NotRef {
        DataCeCostCategory_Rule_Rule_Or_NotRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategory_Rule_Rule_Or_NotRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategory_Rule_Rule_Or_Or {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_Tags>>,
}

impl DataCeCostCategory_Rule_Rule_Or_Or {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_CostCategory>>,
    ) -> Self {
        self.cost_category = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_Dimension>>,
    ) -> Self {
        self.dimension = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_Tags>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCeCostCategory_Rule_Rule_Or_Or {
    type O = terrars::BlockAssignable<DataCeCostCategory_Rule_Rule_Or_Or>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategory_Rule_Rule_Or_Or {}

impl BuildDataCeCostCategory_Rule_Rule_Or_Or {
    pub fn build(self) -> DataCeCostCategory_Rule_Rule_Or_Or {
        DataCeCostCategory_Rule_Rule_Or_Or {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeCostCategory_Rule_Rule_Or_OrRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategory_Rule_Rule_Or_OrRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategory_Rule_Rule_Or_OrRef {
        DataCeCostCategory_Rule_Rule_Or_OrRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategory_Rule_Rule_Or_OrRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategory_Rule_Rule_And {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<terrars::SetField<DataCeCostCategory_Rule_Rule_Or_And>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_Or_Not>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or: std::option::Option<terrars::SetField<DataCeCostCategory_Rule_Rule_Or_Or>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_Tags>>,
}

impl DataCeCostCategory_Rule_Rule_And {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataCeCostCategory_Rule_Rule_Or_And>>,
    ) -> Self {
        self.and = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_CostCategory>>,
    ) -> Self {
        self.cost_category = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_Dimension>>,
    ) -> Self {
        self.dimension = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not`.\n"]
    pub fn set_not(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_Or_Not>>,
    ) -> Self {
        self.not = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `or`.\n"]
    pub fn set_or(mut self, v: impl std::convert::Into<terrars::SetField<DataCeCostCategory_Rule_Rule_Or_Or>>) -> Self {
        self.or = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_Tags>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCeCostCategory_Rule_Rule_And {
    type O = terrars::BlockAssignable<DataCeCostCategory_Rule_Rule_And>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategory_Rule_Rule_And {}

impl BuildDataCeCostCategory_Rule_Rule_And {
    pub fn build(self) -> DataCeCostCategory_Rule_Rule_And {
        DataCeCostCategory_Rule_Rule_And {
            and: core::default::Default::default(),
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            not: core::default::Default::default(),
            or: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeCostCategory_Rule_Rule_AndRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategory_Rule_Rule_AndRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategory_Rule_Rule_AndRef {
        DataCeCostCategory_Rule_Rule_AndRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategory_Rule_Rule_AndRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::SetRef<DataCeCostCategory_Rule_Rule_Or_AndRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.and", self.base))
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_Or_NotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `or` after provisioning.\n"]
    pub fn or(&self) -> terrars::SetRef<DataCeCostCategory_Rule_Rule_Or_OrRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.or", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategory_Rule_Rule_Not {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<terrars::SetField<DataCeCostCategory_Rule_Rule_Or_And>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_Or_Not>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or: std::option::Option<terrars::SetField<DataCeCostCategory_Rule_Rule_Or_Or>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_Tags>>,
}

impl DataCeCostCategory_Rule_Rule_Not {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataCeCostCategory_Rule_Rule_Or_And>>,
    ) -> Self {
        self.and = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_CostCategory>>,
    ) -> Self {
        self.cost_category = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_Dimension>>,
    ) -> Self {
        self.dimension = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not`.\n"]
    pub fn set_not(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_Or_Not>>,
    ) -> Self {
        self.not = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `or`.\n"]
    pub fn set_or(mut self, v: impl std::convert::Into<terrars::SetField<DataCeCostCategory_Rule_Rule_Or_Or>>) -> Self {
        self.or = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_Tags>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCeCostCategory_Rule_Rule_Not {
    type O = terrars::BlockAssignable<DataCeCostCategory_Rule_Rule_Not>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategory_Rule_Rule_Not {}

impl BuildDataCeCostCategory_Rule_Rule_Not {
    pub fn build(self) -> DataCeCostCategory_Rule_Rule_Not {
        DataCeCostCategory_Rule_Rule_Not {
            and: core::default::Default::default(),
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            not: core::default::Default::default(),
            or: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeCostCategory_Rule_Rule_NotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategory_Rule_Rule_NotRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategory_Rule_Rule_NotRef {
        DataCeCostCategory_Rule_Rule_NotRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategory_Rule_Rule_NotRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::SetRef<DataCeCostCategory_Rule_Rule_Or_AndRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.and", self.base))
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_Or_NotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `or` after provisioning.\n"]
    pub fn or(&self) -> terrars::SetRef<DataCeCostCategory_Rule_Rule_Or_OrRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.or", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategory_Rule_Rule_Or {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<terrars::SetField<DataCeCostCategory_Rule_Rule_Or_And>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_Or_Not>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or: std::option::Option<terrars::SetField<DataCeCostCategory_Rule_Rule_Or_Or>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_Tags>>,
}

impl DataCeCostCategory_Rule_Rule_Or {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataCeCostCategory_Rule_Rule_Or_And>>,
    ) -> Self {
        self.and = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_CostCategory>>,
    ) -> Self {
        self.cost_category = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_Dimension>>,
    ) -> Self {
        self.dimension = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not`.\n"]
    pub fn set_not(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_Or_Not>>,
    ) -> Self {
        self.not = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `or`.\n"]
    pub fn set_or(mut self, v: impl std::convert::Into<terrars::SetField<DataCeCostCategory_Rule_Rule_Or_Or>>) -> Self {
        self.or = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_Tags>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCeCostCategory_Rule_Rule_Or {
    type O = terrars::BlockAssignable<DataCeCostCategory_Rule_Rule_Or>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategory_Rule_Rule_Or {}

impl BuildDataCeCostCategory_Rule_Rule_Or {
    pub fn build(self) -> DataCeCostCategory_Rule_Rule_Or {
        DataCeCostCategory_Rule_Rule_Or {
            and: core::default::Default::default(),
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            not: core::default::Default::default(),
            or: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeCostCategory_Rule_Rule_OrRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategory_Rule_Rule_OrRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategory_Rule_Rule_OrRef {
        DataCeCostCategory_Rule_Rule_OrRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategory_Rule_Rule_OrRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::SetRef<DataCeCostCategory_Rule_Rule_Or_AndRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.and", self.base))
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_Or_NotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `or` after provisioning.\n"]
    pub fn or(&self) -> terrars::SetRef<DataCeCostCategory_Rule_Rule_Or_OrRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.or", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategory_Rule_Rule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<terrars::SetField<DataCeCostCategory_Rule_Rule_And>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_Not>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or: std::option::Option<terrars::SetField<DataCeCostCategory_Rule_Rule_Or>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule_Tags>>,
}

impl DataCeCostCategory_Rule_Rule {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(mut self, v: impl std::convert::Into<terrars::SetField<DataCeCostCategory_Rule_Rule_And>>) -> Self {
        self.and = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_CostCategory>>,
    ) -> Self {
        self.cost_category = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_Dimension>>,
    ) -> Self {
        self.dimension = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not`.\n"]
    pub fn set_not(mut self, v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_Not>>) -> Self {
        self.not = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `or`.\n"]
    pub fn set_or(mut self, v: impl std::convert::Into<terrars::SetField<DataCeCostCategory_Rule_Rule_Or>>) -> Self {
        self.or = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule_Tags>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCeCostCategory_Rule_Rule {
    type O = terrars::BlockAssignable<DataCeCostCategory_Rule_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategory_Rule_Rule {}

impl BuildDataCeCostCategory_Rule_Rule {
    pub fn build(self) -> DataCeCostCategory_Rule_Rule {
        DataCeCostCategory_Rule_Rule {
            and: core::default::Default::default(),
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            not: core::default::Default::default(),
            or: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeCostCategory_Rule_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategory_Rule_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategory_Rule_RuleRef {
        DataCeCostCategory_Rule_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategory_Rule_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::SetRef<DataCeCostCategory_Rule_Rule_AndRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.and", self.base))
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_NotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `or` after provisioning.\n"]
    pub fn or(&self) -> terrars::SetRef<DataCeCostCategory_Rule_Rule_OrRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.or", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DataCeCostCategory_Rule_Rule_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategory_Rule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inherited_value: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_InheritedValue>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<terrars::ListField<DataCeCostCategory_Rule_Rule>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCeCostCategory_Rule {
    #[doc = "Set the field `inherited_value`.\n"]
    pub fn set_inherited_value(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_InheritedValue>>,
    ) -> Self {
        self.inherited_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl std::convert::Into<terrars::ListField<DataCeCostCategory_Rule_Rule>>) -> Self {
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

impl terrars::ToListMappable for DataCeCostCategory_Rule {
    type O = terrars::BlockAssignable<DataCeCostCategory_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategory_Rule {}

impl BuildDataCeCostCategory_Rule {
    pub fn build(self) -> DataCeCostCategory_Rule {
        DataCeCostCategory_Rule {
            inherited_value: core::default::Default::default(),
            rule: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeCostCategory_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategory_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategory_RuleRef {
        DataCeCostCategory_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategory_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `inherited_value` after provisioning.\n"]
    pub fn inherited_value(&self) -> terrars::ListRef<DataCeCostCategory_Rule_InheritedValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inherited_value", self.base))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<DataCeCostCategory_Rule_RuleRef> {
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
pub struct DataCeCostCategory_SplitChargeRule_Parameter {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeCostCategory_SplitChargeRule_Parameter {
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

impl terrars::ToListMappable for DataCeCostCategory_SplitChargeRule_Parameter {
    type O = terrars::BlockAssignable<DataCeCostCategory_SplitChargeRule_Parameter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategory_SplitChargeRule_Parameter {}

impl BuildDataCeCostCategory_SplitChargeRule_Parameter {
    pub fn build(self) -> DataCeCostCategory_SplitChargeRule_Parameter {
        DataCeCostCategory_SplitChargeRule_Parameter {
            type_: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeCostCategory_SplitChargeRule_ParameterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategory_SplitChargeRule_ParameterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCeCostCategory_SplitChargeRule_ParameterRef {
        DataCeCostCategory_SplitChargeRule_ParameterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategory_SplitChargeRule_ParameterRef {
    #[allow(dead_code)]
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
pub struct DataCeCostCategory_SplitChargeRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<terrars::SetField<DataCeCostCategory_SplitChargeRule_Parameter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    targets: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCeCostCategory_SplitChargeRule {
    #[doc = "Set the field `method`.\n"]
    pub fn set_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataCeCostCategory_SplitChargeRule_Parameter>>,
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

impl terrars::ToListMappable for DataCeCostCategory_SplitChargeRule {
    type O = terrars::BlockAssignable<DataCeCostCategory_SplitChargeRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategory_SplitChargeRule {}

impl BuildDataCeCostCategory_SplitChargeRule {
    pub fn build(self) -> DataCeCostCategory_SplitChargeRule {
        DataCeCostCategory_SplitChargeRule {
            method: core::default::Default::default(),
            parameter: core::default::Default::default(),
            source: core::default::Default::default(),
            targets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCeCostCategory_SplitChargeRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCeCostCategory_SplitChargeRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCeCostCategory_SplitChargeRuleRef {
        DataCeCostCategory_SplitChargeRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategory_SplitChargeRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc = "Get a reference to the value of field `parameter` after provisioning.\n"]
    pub fn parameter(&self) -> terrars::SetRef<DataCeCostCategory_SplitChargeRule_ParameterRef> {
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
