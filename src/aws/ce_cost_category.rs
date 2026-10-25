#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CeCostCategoryData_ {
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
    default_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effective_start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    rule_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<CeCostCategory_Rule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    split_charge_rule: std::option::Option<std::vec::Vec<CeCostCategory_SplitChargeRule>>,
    dynamic: CeCostCategoryDynamic,
}

struct CeCostCategory_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CeCostCategoryData_>,
}

#[derive(Clone)]
pub struct CeCostCategory(std::rc::Rc<CeCostCategory_>);

impl CeCostCategory {
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

    #[doc = "Set the field `default_value`.\n"]
    pub fn set_default_value(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().default_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `effective_start`.\n"]
    pub fn set_effective_start(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().effective_start = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(self, v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `split_charge_rule`.\n"]
    pub fn set_split_charge_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_SplitChargeRule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().split_charge_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.split_charge_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `rule_version` after provisioning.\n"]
    pub fn rule_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<CeCostCategory_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }
}

impl terrars::Referable for CeCostCategory {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CeCostCategory { }

impl terrars::ToListMappable for CeCostCategory {
    type O = terrars::ListRef<CeCostCategoryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CeCostCategory_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ce_cost_category".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCeCostCategory {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule_version: terrars::PrimField<std::string::String>,
}

impl BuildCeCostCategory {
    pub fn build(self, stack: &mut terrars::Stack) -> CeCostCategory {
        let out = CeCostCategory(std::rc::Rc::new(CeCostCategory_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CeCostCategoryData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                default_value: core::default::Default::default(),
                effective_start: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                rule_version: self.rule_version,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                rule: core::default::Default::default(),
                split_charge_rule: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CeCostCategoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CeCostCategoryRef {
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

    #[doc = "Get a reference to the value of field `rule_version` after provisioning.\n"]
    pub fn rule_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<CeCostCategory_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CeCostCategoryDynamic {
    rule: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule>>,
    split_charge_rule: std::option::Option<terrars::DynamicBlock<CeCostCategory_SplitChargeRule>>,
}

#[derive(Serialize)]
pub struct CeCostCategory_Rule_InheritedValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CeCostCategory_Rule_InheritedValue {
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

impl terrars::ToListMappable for CeCostCategory_Rule_InheritedValue {
    type O = terrars::BlockAssignable<CeCostCategory_Rule_InheritedValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategory_Rule_InheritedValue {}

impl BuildCeCostCategory_Rule_InheritedValue {
    pub fn build(self) -> CeCostCategory_Rule_InheritedValue {
        CeCostCategory_Rule_InheritedValue {
            dimension_key: core::default::Default::default(),
            dimension_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CeCostCategory_Rule_InheritedValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategory_Rule_InheritedValueRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategory_Rule_InheritedValueRef {
        CeCostCategory_Rule_InheritedValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategory_Rule_InheritedValueRef {
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
pub struct CeCostCategory_Rule_Rule_CostCategory {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeCostCategory_Rule_Rule_CostCategory {
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

impl terrars::ToListMappable for CeCostCategory_Rule_Rule_CostCategory {
    type O = terrars::BlockAssignable<CeCostCategory_Rule_Rule_CostCategory>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategory_Rule_Rule_CostCategory {}

impl BuildCeCostCategory_Rule_Rule_CostCategory {
    pub fn build(self) -> CeCostCategory_Rule_Rule_CostCategory {
        CeCostCategory_Rule_Rule_CostCategory {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CeCostCategory_Rule_Rule_CostCategoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategory_Rule_Rule_CostCategoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategory_Rule_Rule_CostCategoryRef {
        CeCostCategory_Rule_Rule_CostCategoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategory_Rule_Rule_CostCategoryRef {
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
pub struct CeCostCategory_Rule_Rule_Dimension {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeCostCategory_Rule_Rule_Dimension {
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

impl terrars::ToListMappable for CeCostCategory_Rule_Rule_Dimension {
    type O = terrars::BlockAssignable<CeCostCategory_Rule_Rule_Dimension>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategory_Rule_Rule_Dimension {}

impl BuildCeCostCategory_Rule_Rule_Dimension {
    pub fn build(self) -> CeCostCategory_Rule_Rule_Dimension {
        CeCostCategory_Rule_Rule_Dimension {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CeCostCategory_Rule_Rule_DimensionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategory_Rule_Rule_DimensionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategory_Rule_Rule_DimensionRef {
        CeCostCategory_Rule_Rule_DimensionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategory_Rule_Rule_DimensionRef {
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
pub struct CeCostCategory_Rule_Rule_Tags {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeCostCategory_Rule_Rule_Tags {
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

impl terrars::ToListMappable for CeCostCategory_Rule_Rule_Tags {
    type O = terrars::BlockAssignable<CeCostCategory_Rule_Rule_Tags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategory_Rule_Rule_Tags {}

impl BuildCeCostCategory_Rule_Rule_Tags {
    pub fn build(self) -> CeCostCategory_Rule_Rule_Tags {
        CeCostCategory_Rule_Rule_Tags {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CeCostCategory_Rule_Rule_TagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategory_Rule_Rule_TagsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategory_Rule_Rule_TagsRef {
        CeCostCategory_Rule_Rule_TagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategory_Rule_Rule_TagsRef {
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
struct CeCostCategory_Rule_Rule_Or_AndDynamic {
    cost_category: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_CostCategory>>,
    dimension: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Dimension>>,
    tags: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Tags>>,
}

#[derive(Serialize)]
pub struct CeCostCategory_Rule_Rule_Or_And {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Tags>>,
    dynamic: CeCostCategory_Rule_Rule_Or_AndDynamic,
}

impl CeCostCategory_Rule_Rule_Or_And {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_CostCategory>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Dimension>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Tags>>,
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

impl terrars::ToListMappable for CeCostCategory_Rule_Rule_Or_And {
    type O = terrars::BlockAssignable<CeCostCategory_Rule_Rule_Or_And>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategory_Rule_Rule_Or_And {}

impl BuildCeCostCategory_Rule_Rule_Or_And {
    pub fn build(self) -> CeCostCategory_Rule_Rule_Or_And {
        CeCostCategory_Rule_Rule_Or_And {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CeCostCategory_Rule_Rule_Or_AndRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategory_Rule_Rule_Or_AndRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategory_Rule_Rule_Or_AndRef {
        CeCostCategory_Rule_Rule_Or_AndRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategory_Rule_Rule_Or_AndRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct CeCostCategory_Rule_Rule_Or_NotDynamic {
    cost_category: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_CostCategory>>,
    dimension: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Dimension>>,
    tags: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Tags>>,
}

#[derive(Serialize)]
pub struct CeCostCategory_Rule_Rule_Or_Not {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Tags>>,
    dynamic: CeCostCategory_Rule_Rule_Or_NotDynamic,
}

impl CeCostCategory_Rule_Rule_Or_Not {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_CostCategory>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Dimension>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Tags>>,
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

impl terrars::ToListMappable for CeCostCategory_Rule_Rule_Or_Not {
    type O = terrars::BlockAssignable<CeCostCategory_Rule_Rule_Or_Not>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategory_Rule_Rule_Or_Not {}

impl BuildCeCostCategory_Rule_Rule_Or_Not {
    pub fn build(self) -> CeCostCategory_Rule_Rule_Or_Not {
        CeCostCategory_Rule_Rule_Or_Not {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CeCostCategory_Rule_Rule_Or_NotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategory_Rule_Rule_Or_NotRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategory_Rule_Rule_Or_NotRef {
        CeCostCategory_Rule_Rule_Or_NotRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategory_Rule_Rule_Or_NotRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct CeCostCategory_Rule_Rule_Or_OrDynamic {
    cost_category: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_CostCategory>>,
    dimension: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Dimension>>,
    tags: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Tags>>,
}

#[derive(Serialize)]
pub struct CeCostCategory_Rule_Rule_Or_Or {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Tags>>,
    dynamic: CeCostCategory_Rule_Rule_Or_OrDynamic,
}

impl CeCostCategory_Rule_Rule_Or_Or {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_CostCategory>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Dimension>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Tags>>,
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

impl terrars::ToListMappable for CeCostCategory_Rule_Rule_Or_Or {
    type O = terrars::BlockAssignable<CeCostCategory_Rule_Rule_Or_Or>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategory_Rule_Rule_Or_Or {}

impl BuildCeCostCategory_Rule_Rule_Or_Or {
    pub fn build(self) -> CeCostCategory_Rule_Rule_Or_Or {
        CeCostCategory_Rule_Rule_Or_Or {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CeCostCategory_Rule_Rule_Or_OrRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategory_Rule_Rule_Or_OrRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategory_Rule_Rule_Or_OrRef {
        CeCostCategory_Rule_Rule_Or_OrRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategory_Rule_Rule_Or_OrRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct CeCostCategory_Rule_Rule_AndDynamic {
    and: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Or_And>>,
    cost_category: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_CostCategory>>,
    dimension: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Dimension>>,
    not: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Or_Not>>,
    or: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Or_Or>>,
    tags: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Tags>>,
}

#[derive(Serialize)]
pub struct CeCostCategory_Rule_Rule_And {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Or_And>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Or_Not>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Or_Or>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Tags>>,
    dynamic: CeCostCategory_Rule_Rule_AndDynamic,
}

impl CeCostCategory_Rule_Rule_And {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Or_And>>,
    ) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_CostCategory>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Dimension>>,
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
    pub fn set_not(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Or_Not>>,
    ) -> Self {
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
    pub fn set_or(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Or_Or>>,
    ) -> Self {
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
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Tags>>,
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

impl terrars::ToListMappable for CeCostCategory_Rule_Rule_And {
    type O = terrars::BlockAssignable<CeCostCategory_Rule_Rule_And>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategory_Rule_Rule_And {}

impl BuildCeCostCategory_Rule_Rule_And {
    pub fn build(self) -> CeCostCategory_Rule_Rule_And {
        CeCostCategory_Rule_Rule_And {
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
pub struct CeCostCategory_Rule_Rule_AndRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategory_Rule_Rule_AndRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategory_Rule_Rule_AndRef {
        CeCostCategory_Rule_Rule_AndRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategory_Rule_Rule_AndRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_Or_NotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct CeCostCategory_Rule_Rule_NotDynamic {
    and: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Or_And>>,
    cost_category: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_CostCategory>>,
    dimension: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Dimension>>,
    not: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Or_Not>>,
    or: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Or_Or>>,
    tags: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Tags>>,
}

#[derive(Serialize)]
pub struct CeCostCategory_Rule_Rule_Not {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Or_And>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Or_Not>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Or_Or>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Tags>>,
    dynamic: CeCostCategory_Rule_Rule_NotDynamic,
}

impl CeCostCategory_Rule_Rule_Not {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Or_And>>,
    ) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_CostCategory>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Dimension>>,
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
    pub fn set_not(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Or_Not>>,
    ) -> Self {
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
    pub fn set_or(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Or_Or>>,
    ) -> Self {
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
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Tags>>,
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

impl terrars::ToListMappable for CeCostCategory_Rule_Rule_Not {
    type O = terrars::BlockAssignable<CeCostCategory_Rule_Rule_Not>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategory_Rule_Rule_Not {}

impl BuildCeCostCategory_Rule_Rule_Not {
    pub fn build(self) -> CeCostCategory_Rule_Rule_Not {
        CeCostCategory_Rule_Rule_Not {
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
pub struct CeCostCategory_Rule_Rule_NotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategory_Rule_Rule_NotRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategory_Rule_Rule_NotRef {
        CeCostCategory_Rule_Rule_NotRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategory_Rule_Rule_NotRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_Or_NotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct CeCostCategory_Rule_Rule_OrDynamic {
    and: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Or_And>>,
    cost_category: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_CostCategory>>,
    dimension: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Dimension>>,
    not: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Or_Not>>,
    or: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Or_Or>>,
    tags: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Tags>>,
}

#[derive(Serialize)]
pub struct CeCostCategory_Rule_Rule_Or {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Or_And>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Or_Not>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Or_Or>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Tags>>,
    dynamic: CeCostCategory_Rule_Rule_OrDynamic,
}

impl CeCostCategory_Rule_Rule_Or {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Or_And>>,
    ) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_CostCategory>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Dimension>>,
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
    pub fn set_not(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Or_Not>>,
    ) -> Self {
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
    pub fn set_or(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Or_Or>>,
    ) -> Self {
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
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Tags>>,
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

impl terrars::ToListMappable for CeCostCategory_Rule_Rule_Or {
    type O = terrars::BlockAssignable<CeCostCategory_Rule_Rule_Or>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategory_Rule_Rule_Or {}

impl BuildCeCostCategory_Rule_Rule_Or {
    pub fn build(self) -> CeCostCategory_Rule_Rule_Or {
        CeCostCategory_Rule_Rule_Or {
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
pub struct CeCostCategory_Rule_Rule_OrRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategory_Rule_Rule_OrRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategory_Rule_Rule_OrRef {
        CeCostCategory_Rule_Rule_OrRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategory_Rule_Rule_OrRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_Or_NotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct CeCostCategory_Rule_RuleDynamic {
    and: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_And>>,
    cost_category: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_CostCategory>>,
    dimension: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Dimension>>,
    not: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Not>>,
    or: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Or>>,
    tags: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule_Tags>>,
}

#[derive(Serialize)]
pub struct CeCostCategory_Rule_Rule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_And>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Not>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Or>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule_Tags>>,
    dynamic: CeCostCategory_Rule_RuleDynamic,
}

impl CeCostCategory_Rule_Rule {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_And>>,
    ) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_CostCategory>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Dimension>>,
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
    pub fn set_not(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Not>>,
    ) -> Self {
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
    pub fn set_or(mut self, v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Or>>) -> Self {
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
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule_Tags>>,
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

impl terrars::ToListMappable for CeCostCategory_Rule_Rule {
    type O = terrars::BlockAssignable<CeCostCategory_Rule_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategory_Rule_Rule {}

impl BuildCeCostCategory_Rule_Rule {
    pub fn build(self) -> CeCostCategory_Rule_Rule {
        CeCostCategory_Rule_Rule {
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
pub struct CeCostCategory_Rule_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategory_Rule_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategory_Rule_RuleRef {
        CeCostCategory_Rule_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategory_Rule_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_NotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<CeCostCategory_Rule_Rule_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct CeCostCategory_RuleDynamic {
    inherited_value: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_InheritedValue>>,
    rule: std::option::Option<terrars::DynamicBlock<CeCostCategory_Rule_Rule>>,
}

#[derive(Serialize)]
pub struct CeCostCategory_Rule {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inherited_value: std::option::Option<std::vec::Vec<CeCostCategory_Rule_InheritedValue>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<CeCostCategory_Rule_Rule>>,
    dynamic: CeCostCategory_RuleDynamic,
}

impl CeCostCategory_Rule {
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

    #[doc = "Set the field `inherited_value`.\n"]
    pub fn set_inherited_value(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_InheritedValue>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inherited_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inherited_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_Rule_Rule>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CeCostCategory_Rule {
    type O = terrars::BlockAssignable<CeCostCategory_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategory_Rule {}

impl BuildCeCostCategory_Rule {
    pub fn build(self) -> CeCostCategory_Rule {
        CeCostCategory_Rule {
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
            inherited_value: core::default::Default::default(),
            rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CeCostCategory_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategory_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategory_RuleRef {
        CeCostCategory_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategory_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc = "Get a reference to the value of field `inherited_value` after provisioning.\n"]
    pub fn inherited_value(&self) -> terrars::ListRef<CeCostCategory_Rule_InheritedValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inherited_value", self.base))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<CeCostCategory_Rule_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}

#[derive(Serialize)]
pub struct CeCostCategory_SplitChargeRule_Parameter {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl CeCostCategory_SplitChargeRule_Parameter {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CeCostCategory_SplitChargeRule_Parameter {
    type O = terrars::BlockAssignable<CeCostCategory_SplitChargeRule_Parameter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategory_SplitChargeRule_Parameter {}

impl BuildCeCostCategory_SplitChargeRule_Parameter {
    pub fn build(self) -> CeCostCategory_SplitChargeRule_Parameter {
        CeCostCategory_SplitChargeRule_Parameter {
            type_: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CeCostCategory_SplitChargeRule_ParameterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategory_SplitChargeRule_ParameterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategory_SplitChargeRule_ParameterRef {
        CeCostCategory_SplitChargeRule_ParameterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategory_SplitChargeRule_ParameterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct CeCostCategory_SplitChargeRuleDynamic {
    parameter: std::option::Option<terrars::DynamicBlock<CeCostCategory_SplitChargeRule_Parameter>>,
}

#[derive(Serialize)]
pub struct CeCostCategory_SplitChargeRule {
    method: terrars::PrimField<std::string::String>,
    source: terrars::PrimField<std::string::String>,
    targets: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<std::vec::Vec<CeCostCategory_SplitChargeRule_Parameter>>,
    dynamic: CeCostCategory_SplitChargeRuleDynamic,
}

impl CeCostCategory_SplitChargeRule {
    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategory_SplitChargeRule_Parameter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CeCostCategory_SplitChargeRule {
    type O = terrars::BlockAssignable<CeCostCategory_SplitChargeRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategory_SplitChargeRule {
    #[doc = ""]
    pub method: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub targets: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildCeCostCategory_SplitChargeRule {
    pub fn build(self) -> CeCostCategory_SplitChargeRule {
        CeCostCategory_SplitChargeRule {
            method: self.method,
            source: self.source,
            targets: self.targets,
            parameter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CeCostCategory_SplitChargeRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategory_SplitChargeRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategory_SplitChargeRuleRef {
        CeCostCategory_SplitChargeRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategory_SplitChargeRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.targets", self.base))
    }
}
