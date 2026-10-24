use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CeCostCategoryData {
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
    rule: std::option::Option<std::vec::Vec<CeCostCategoryRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    split_charge_rule: std::option::Option<std::vec::Vec<CeCostCategorySplitChargeRuleEl>>,
    dynamic: CeCostCategoryDynamic,
}

struct CeCostCategory_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CeCostCategoryData>,
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
    pub fn set_rule(self, v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategoryRuleEl>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategorySplitChargeRuleEl>>,
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
            data: std::cell::RefCell::new(CeCostCategoryData {
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
}

#[derive(Serialize)]
pub struct CeCostCategoryRuleElInheritedValueEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CeCostCategoryRuleElInheritedValueEl {
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

impl terrars::ToListMappable for CeCostCategoryRuleElInheritedValueEl {
    type O = terrars::BlockAssignable<CeCostCategoryRuleElInheritedValueEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategoryRuleElInheritedValueEl {}

impl BuildCeCostCategoryRuleElInheritedValueEl {
    pub fn build(self) -> CeCostCategoryRuleElInheritedValueEl {
        CeCostCategoryRuleElInheritedValueEl {
            dimension_key: core::default::Default::default(),
            dimension_name: core::default::Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElInheritedValueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategoryRuleElInheritedValueElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategoryRuleElInheritedValueElRef {
        CeCostCategoryRuleElInheritedValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElInheritedValueElRef {
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
pub struct CeCostCategoryRuleElRuleElAndElCostCategoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeCostCategoryRuleElRuleElAndElCostCategoryEl {
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

impl terrars::ToListMappable for CeCostCategoryRuleElRuleElAndElCostCategoryEl {
    type O = terrars::BlockAssignable<CeCostCategoryRuleElRuleElAndElCostCategoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategoryRuleElRuleElAndElCostCategoryEl {}

impl BuildCeCostCategoryRuleElRuleElAndElCostCategoryEl {
    pub fn build(self) -> CeCostCategoryRuleElRuleElAndElCostCategoryEl {
        CeCostCategoryRuleElRuleElAndElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElRuleElAndElCostCategoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategoryRuleElRuleElAndElCostCategoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeCostCategoryRuleElRuleElAndElCostCategoryElRef {
        CeCostCategoryRuleElRuleElAndElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElAndElCostCategoryElRef {
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
pub struct CeCostCategoryRuleElRuleElAndElDimensionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeCostCategoryRuleElRuleElAndElDimensionEl {
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

impl terrars::ToListMappable for CeCostCategoryRuleElRuleElAndElDimensionEl {
    type O = terrars::BlockAssignable<CeCostCategoryRuleElRuleElAndElDimensionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategoryRuleElRuleElAndElDimensionEl {}

impl BuildCeCostCategoryRuleElRuleElAndElDimensionEl {
    pub fn build(self) -> CeCostCategoryRuleElRuleElAndElDimensionEl {
        CeCostCategoryRuleElRuleElAndElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElRuleElAndElDimensionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategoryRuleElRuleElAndElDimensionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategoryRuleElRuleElAndElDimensionElRef {
        CeCostCategoryRuleElRuleElAndElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElAndElDimensionElRef {
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
pub struct CeCostCategoryRuleElRuleElAndElTagsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeCostCategoryRuleElRuleElAndElTagsEl {
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

impl terrars::ToListMappable for CeCostCategoryRuleElRuleElAndElTagsEl {
    type O = terrars::BlockAssignable<CeCostCategoryRuleElRuleElAndElTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategoryRuleElRuleElAndElTagsEl {}

impl BuildCeCostCategoryRuleElRuleElAndElTagsEl {
    pub fn build(self) -> CeCostCategoryRuleElRuleElAndElTagsEl {
        CeCostCategoryRuleElRuleElAndElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElRuleElAndElTagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategoryRuleElRuleElAndElTagsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategoryRuleElRuleElAndElTagsElRef {
        CeCostCategoryRuleElRuleElAndElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElAndElTagsElRef {
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
struct CeCostCategoryRuleElRuleElAndElDynamic {
    cost_category: std::option::Option<terrars::DynamicBlock<CeCostCategoryRuleElRuleElAndElCostCategoryEl>>,
    dimension: std::option::Option<terrars::DynamicBlock<CeCostCategoryRuleElRuleElAndElDimensionEl>>,
    tags: std::option::Option<terrars::DynamicBlock<CeCostCategoryRuleElRuleElAndElTagsEl>>,
}

#[derive(Serialize)]
pub struct CeCostCategoryRuleElRuleElAndEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<CeCostCategoryRuleElRuleElAndElCostCategoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<CeCostCategoryRuleElRuleElAndElDimensionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<CeCostCategoryRuleElRuleElAndElTagsEl>>,
    dynamic: CeCostCategoryRuleElRuleElAndElDynamic,
}

impl CeCostCategoryRuleElRuleElAndEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategoryRuleElRuleElAndElCostCategoryEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategoryRuleElRuleElAndElDimensionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategoryRuleElRuleElAndElTagsEl>>,
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

impl terrars::ToListMappable for CeCostCategoryRuleElRuleElAndEl {
    type O = terrars::BlockAssignable<CeCostCategoryRuleElRuleElAndEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategoryRuleElRuleElAndEl {}

impl BuildCeCostCategoryRuleElRuleElAndEl {
    pub fn build(self) -> CeCostCategoryRuleElRuleElAndEl {
        CeCostCategoryRuleElRuleElAndEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElRuleElAndElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategoryRuleElRuleElAndElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategoryRuleElRuleElAndElRef {
        CeCostCategoryRuleElRuleElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElAndElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<CeCostCategoryRuleElRuleElAndElCostCategoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<CeCostCategoryRuleElRuleElAndElDimensionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<CeCostCategoryRuleElRuleElAndElTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct CeCostCategoryRuleElRuleElCostCategoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeCostCategoryRuleElRuleElCostCategoryEl {
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

impl terrars::ToListMappable for CeCostCategoryRuleElRuleElCostCategoryEl {
    type O = terrars::BlockAssignable<CeCostCategoryRuleElRuleElCostCategoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategoryRuleElRuleElCostCategoryEl {}

impl BuildCeCostCategoryRuleElRuleElCostCategoryEl {
    pub fn build(self) -> CeCostCategoryRuleElRuleElCostCategoryEl {
        CeCostCategoryRuleElRuleElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElRuleElCostCategoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategoryRuleElRuleElCostCategoryElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategoryRuleElRuleElCostCategoryElRef {
        CeCostCategoryRuleElRuleElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElCostCategoryElRef {
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
pub struct CeCostCategoryRuleElRuleElDimensionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeCostCategoryRuleElRuleElDimensionEl {
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

impl terrars::ToListMappable for CeCostCategoryRuleElRuleElDimensionEl {
    type O = terrars::BlockAssignable<CeCostCategoryRuleElRuleElDimensionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategoryRuleElRuleElDimensionEl {}

impl BuildCeCostCategoryRuleElRuleElDimensionEl {
    pub fn build(self) -> CeCostCategoryRuleElRuleElDimensionEl {
        CeCostCategoryRuleElRuleElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElRuleElDimensionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategoryRuleElRuleElDimensionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategoryRuleElRuleElDimensionElRef {
        CeCostCategoryRuleElRuleElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElDimensionElRef {
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
pub struct CeCostCategoryRuleElRuleElNotElCostCategoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeCostCategoryRuleElRuleElNotElCostCategoryEl {
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

impl terrars::ToListMappable for CeCostCategoryRuleElRuleElNotElCostCategoryEl {
    type O = terrars::BlockAssignable<CeCostCategoryRuleElRuleElNotElCostCategoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategoryRuleElRuleElNotElCostCategoryEl {}

impl BuildCeCostCategoryRuleElRuleElNotElCostCategoryEl {
    pub fn build(self) -> CeCostCategoryRuleElRuleElNotElCostCategoryEl {
        CeCostCategoryRuleElRuleElNotElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElRuleElNotElCostCategoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategoryRuleElRuleElNotElCostCategoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeCostCategoryRuleElRuleElNotElCostCategoryElRef {
        CeCostCategoryRuleElRuleElNotElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElNotElCostCategoryElRef {
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
pub struct CeCostCategoryRuleElRuleElNotElDimensionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeCostCategoryRuleElRuleElNotElDimensionEl {
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

impl terrars::ToListMappable for CeCostCategoryRuleElRuleElNotElDimensionEl {
    type O = terrars::BlockAssignable<CeCostCategoryRuleElRuleElNotElDimensionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategoryRuleElRuleElNotElDimensionEl {}

impl BuildCeCostCategoryRuleElRuleElNotElDimensionEl {
    pub fn build(self) -> CeCostCategoryRuleElRuleElNotElDimensionEl {
        CeCostCategoryRuleElRuleElNotElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElRuleElNotElDimensionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategoryRuleElRuleElNotElDimensionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategoryRuleElRuleElNotElDimensionElRef {
        CeCostCategoryRuleElRuleElNotElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElNotElDimensionElRef {
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
pub struct CeCostCategoryRuleElRuleElNotElTagsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeCostCategoryRuleElRuleElNotElTagsEl {
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

impl terrars::ToListMappable for CeCostCategoryRuleElRuleElNotElTagsEl {
    type O = terrars::BlockAssignable<CeCostCategoryRuleElRuleElNotElTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategoryRuleElRuleElNotElTagsEl {}

impl BuildCeCostCategoryRuleElRuleElNotElTagsEl {
    pub fn build(self) -> CeCostCategoryRuleElRuleElNotElTagsEl {
        CeCostCategoryRuleElRuleElNotElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElRuleElNotElTagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategoryRuleElRuleElNotElTagsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategoryRuleElRuleElNotElTagsElRef {
        CeCostCategoryRuleElRuleElNotElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElNotElTagsElRef {
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
struct CeCostCategoryRuleElRuleElNotElDynamic {
    cost_category: std::option::Option<terrars::DynamicBlock<CeCostCategoryRuleElRuleElNotElCostCategoryEl>>,
    dimension: std::option::Option<terrars::DynamicBlock<CeCostCategoryRuleElRuleElNotElDimensionEl>>,
    tags: std::option::Option<terrars::DynamicBlock<CeCostCategoryRuleElRuleElNotElTagsEl>>,
}

#[derive(Serialize)]
pub struct CeCostCategoryRuleElRuleElNotEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<CeCostCategoryRuleElRuleElNotElCostCategoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<CeCostCategoryRuleElRuleElNotElDimensionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<CeCostCategoryRuleElRuleElNotElTagsEl>>,
    dynamic: CeCostCategoryRuleElRuleElNotElDynamic,
}

impl CeCostCategoryRuleElRuleElNotEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategoryRuleElRuleElNotElCostCategoryEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategoryRuleElRuleElNotElDimensionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategoryRuleElRuleElNotElTagsEl>>,
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

impl terrars::ToListMappable for CeCostCategoryRuleElRuleElNotEl {
    type O = terrars::BlockAssignable<CeCostCategoryRuleElRuleElNotEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategoryRuleElRuleElNotEl {}

impl BuildCeCostCategoryRuleElRuleElNotEl {
    pub fn build(self) -> CeCostCategoryRuleElRuleElNotEl {
        CeCostCategoryRuleElRuleElNotEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElRuleElNotElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategoryRuleElRuleElNotElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategoryRuleElRuleElNotElRef {
        CeCostCategoryRuleElRuleElNotElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElNotElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<CeCostCategoryRuleElRuleElNotElCostCategoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<CeCostCategoryRuleElRuleElNotElDimensionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<CeCostCategoryRuleElRuleElNotElTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct CeCostCategoryRuleElRuleElOrElCostCategoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeCostCategoryRuleElRuleElOrElCostCategoryEl {
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

impl terrars::ToListMappable for CeCostCategoryRuleElRuleElOrElCostCategoryEl {
    type O = terrars::BlockAssignable<CeCostCategoryRuleElRuleElOrElCostCategoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategoryRuleElRuleElOrElCostCategoryEl {}

impl BuildCeCostCategoryRuleElRuleElOrElCostCategoryEl {
    pub fn build(self) -> CeCostCategoryRuleElRuleElOrElCostCategoryEl {
        CeCostCategoryRuleElRuleElOrElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElRuleElOrElCostCategoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategoryRuleElRuleElOrElCostCategoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeCostCategoryRuleElRuleElOrElCostCategoryElRef {
        CeCostCategoryRuleElRuleElOrElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElOrElCostCategoryElRef {
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
pub struct CeCostCategoryRuleElRuleElOrElDimensionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeCostCategoryRuleElRuleElOrElDimensionEl {
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

impl terrars::ToListMappable for CeCostCategoryRuleElRuleElOrElDimensionEl {
    type O = terrars::BlockAssignable<CeCostCategoryRuleElRuleElOrElDimensionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategoryRuleElRuleElOrElDimensionEl {}

impl BuildCeCostCategoryRuleElRuleElOrElDimensionEl {
    pub fn build(self) -> CeCostCategoryRuleElRuleElOrElDimensionEl {
        CeCostCategoryRuleElRuleElOrElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElRuleElOrElDimensionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategoryRuleElRuleElOrElDimensionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategoryRuleElRuleElOrElDimensionElRef {
        CeCostCategoryRuleElRuleElOrElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElOrElDimensionElRef {
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
pub struct CeCostCategoryRuleElRuleElOrElTagsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeCostCategoryRuleElRuleElOrElTagsEl {
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

impl terrars::ToListMappable for CeCostCategoryRuleElRuleElOrElTagsEl {
    type O = terrars::BlockAssignable<CeCostCategoryRuleElRuleElOrElTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategoryRuleElRuleElOrElTagsEl {}

impl BuildCeCostCategoryRuleElRuleElOrElTagsEl {
    pub fn build(self) -> CeCostCategoryRuleElRuleElOrElTagsEl {
        CeCostCategoryRuleElRuleElOrElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElRuleElOrElTagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategoryRuleElRuleElOrElTagsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategoryRuleElRuleElOrElTagsElRef {
        CeCostCategoryRuleElRuleElOrElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElOrElTagsElRef {
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
struct CeCostCategoryRuleElRuleElOrElDynamic {
    cost_category: std::option::Option<terrars::DynamicBlock<CeCostCategoryRuleElRuleElOrElCostCategoryEl>>,
    dimension: std::option::Option<terrars::DynamicBlock<CeCostCategoryRuleElRuleElOrElDimensionEl>>,
    tags: std::option::Option<terrars::DynamicBlock<CeCostCategoryRuleElRuleElOrElTagsEl>>,
}

#[derive(Serialize)]
pub struct CeCostCategoryRuleElRuleElOrEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<CeCostCategoryRuleElRuleElOrElCostCategoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<CeCostCategoryRuleElRuleElOrElDimensionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<CeCostCategoryRuleElRuleElOrElTagsEl>>,
    dynamic: CeCostCategoryRuleElRuleElOrElDynamic,
}

impl CeCostCategoryRuleElRuleElOrEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategoryRuleElRuleElOrElCostCategoryEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategoryRuleElRuleElOrElDimensionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategoryRuleElRuleElOrElTagsEl>>,
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

impl terrars::ToListMappable for CeCostCategoryRuleElRuleElOrEl {
    type O = terrars::BlockAssignable<CeCostCategoryRuleElRuleElOrEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategoryRuleElRuleElOrEl {}

impl BuildCeCostCategoryRuleElRuleElOrEl {
    pub fn build(self) -> CeCostCategoryRuleElRuleElOrEl {
        CeCostCategoryRuleElRuleElOrEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElRuleElOrElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategoryRuleElRuleElOrElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategoryRuleElRuleElOrElRef {
        CeCostCategoryRuleElRuleElOrElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElOrElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<CeCostCategoryRuleElRuleElOrElCostCategoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<CeCostCategoryRuleElRuleElOrElDimensionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<CeCostCategoryRuleElRuleElOrElTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct CeCostCategoryRuleElRuleElTagsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeCostCategoryRuleElRuleElTagsEl {
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

impl terrars::ToListMappable for CeCostCategoryRuleElRuleElTagsEl {
    type O = terrars::BlockAssignable<CeCostCategoryRuleElRuleElTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategoryRuleElRuleElTagsEl {}

impl BuildCeCostCategoryRuleElRuleElTagsEl {
    pub fn build(self) -> CeCostCategoryRuleElRuleElTagsEl {
        CeCostCategoryRuleElRuleElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElRuleElTagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategoryRuleElRuleElTagsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategoryRuleElRuleElTagsElRef {
        CeCostCategoryRuleElRuleElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElTagsElRef {
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
struct CeCostCategoryRuleElRuleElDynamic {
    and: std::option::Option<terrars::DynamicBlock<CeCostCategoryRuleElRuleElAndEl>>,
    cost_category: std::option::Option<terrars::DynamicBlock<CeCostCategoryRuleElRuleElCostCategoryEl>>,
    dimension: std::option::Option<terrars::DynamicBlock<CeCostCategoryRuleElRuleElDimensionEl>>,
    not: std::option::Option<terrars::DynamicBlock<CeCostCategoryRuleElRuleElNotEl>>,
    or: std::option::Option<terrars::DynamicBlock<CeCostCategoryRuleElRuleElOrEl>>,
    tags: std::option::Option<terrars::DynamicBlock<CeCostCategoryRuleElRuleElTagsEl>>,
}

#[derive(Serialize)]
pub struct CeCostCategoryRuleElRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<CeCostCategoryRuleElRuleElAndEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<CeCostCategoryRuleElRuleElCostCategoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<CeCostCategoryRuleElRuleElDimensionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not: std::option::Option<std::vec::Vec<CeCostCategoryRuleElRuleElNotEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or: std::option::Option<std::vec::Vec<CeCostCategoryRuleElRuleElOrEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<CeCostCategoryRuleElRuleElTagsEl>>,
    dynamic: CeCostCategoryRuleElRuleElDynamic,
}

impl CeCostCategoryRuleElRuleEl {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategoryRuleElRuleElAndEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategoryRuleElRuleElCostCategoryEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategoryRuleElRuleElDimensionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategoryRuleElRuleElNotEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategoryRuleElRuleElOrEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategoryRuleElRuleElTagsEl>>,
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

impl terrars::ToListMappable for CeCostCategoryRuleElRuleEl {
    type O = terrars::BlockAssignable<CeCostCategoryRuleElRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategoryRuleElRuleEl {}

impl BuildCeCostCategoryRuleElRuleEl {
    pub fn build(self) -> CeCostCategoryRuleElRuleEl {
        CeCostCategoryRuleElRuleEl {
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

pub struct CeCostCategoryRuleElRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategoryRuleElRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategoryRuleElRuleElRef {
        CeCostCategoryRuleElRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<CeCostCategoryRuleElRuleElCostCategoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<CeCostCategoryRuleElRuleElDimensionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> terrars::ListRef<CeCostCategoryRuleElRuleElNotElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<CeCostCategoryRuleElRuleElTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct CeCostCategoryRuleElDynamic {
    inherited_value: std::option::Option<terrars::DynamicBlock<CeCostCategoryRuleElInheritedValueEl>>,
    rule: std::option::Option<terrars::DynamicBlock<CeCostCategoryRuleElRuleEl>>,
}

#[derive(Serialize)]
pub struct CeCostCategoryRuleEl {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inherited_value: std::option::Option<std::vec::Vec<CeCostCategoryRuleElInheritedValueEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<CeCostCategoryRuleElRuleEl>>,
    dynamic: CeCostCategoryRuleElDynamic,
}

impl CeCostCategoryRuleEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategoryRuleElInheritedValueEl>>,
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
    pub fn set_rule(mut self, v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategoryRuleElRuleEl>>) -> Self {
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

impl terrars::ToListMappable for CeCostCategoryRuleEl {
    type O = terrars::BlockAssignable<CeCostCategoryRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategoryRuleEl {}

impl BuildCeCostCategoryRuleEl {
    pub fn build(self) -> CeCostCategoryRuleEl {
        CeCostCategoryRuleEl {
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
            inherited_value: core::default::Default::default(),
            rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategoryRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategoryRuleElRef {
        CeCostCategoryRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRef {
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
    pub fn inherited_value(&self) -> terrars::ListRef<CeCostCategoryRuleElInheritedValueElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inherited_value", self.base))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<CeCostCategoryRuleElRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}

#[derive(Serialize)]
pub struct CeCostCategorySplitChargeRuleElParameterEl {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl CeCostCategorySplitChargeRuleElParameterEl {
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

impl terrars::ToListMappable for CeCostCategorySplitChargeRuleElParameterEl {
    type O = terrars::BlockAssignable<CeCostCategorySplitChargeRuleElParameterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategorySplitChargeRuleElParameterEl {}

impl BuildCeCostCategorySplitChargeRuleElParameterEl {
    pub fn build(self) -> CeCostCategorySplitChargeRuleElParameterEl {
        CeCostCategorySplitChargeRuleElParameterEl {
            type_: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeCostCategorySplitChargeRuleElParameterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategorySplitChargeRuleElParameterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategorySplitChargeRuleElParameterElRef {
        CeCostCategorySplitChargeRuleElParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategorySplitChargeRuleElParameterElRef {
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
struct CeCostCategorySplitChargeRuleElDynamic {
    parameter: std::option::Option<terrars::DynamicBlock<CeCostCategorySplitChargeRuleElParameterEl>>,
}

#[derive(Serialize)]
pub struct CeCostCategorySplitChargeRuleEl {
    method: terrars::PrimField<std::string::String>,
    source: terrars::PrimField<std::string::String>,
    targets: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<std::vec::Vec<CeCostCategorySplitChargeRuleElParameterEl>>,
    dynamic: CeCostCategorySplitChargeRuleElDynamic,
}

impl CeCostCategorySplitChargeRuleEl {
    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeCostCategorySplitChargeRuleElParameterEl>>,
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

impl terrars::ToListMappable for CeCostCategorySplitChargeRuleEl {
    type O = terrars::BlockAssignable<CeCostCategorySplitChargeRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategorySplitChargeRuleEl {
    #[doc = ""]
    pub method: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub targets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildCeCostCategorySplitChargeRuleEl {
    pub fn build(self) -> CeCostCategorySplitChargeRuleEl {
        CeCostCategorySplitChargeRuleEl {
            method: self.method,
            source: self.source,
            targets: self.targets,
            parameter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CeCostCategorySplitChargeRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeCostCategorySplitChargeRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeCostCategorySplitChargeRuleElRef {
        CeCostCategorySplitChargeRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategorySplitChargeRuleElRef {
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
    pub fn targets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.targets", self.base))
    }
}

#[derive(Serialize, Default)]
struct CeCostCategoryDynamic {
    rule: std::option::Option<terrars::DynamicBlock<CeCostCategoryRuleEl>>,
    split_charge_rule: std::option::Option<terrars::DynamicBlock<CeCostCategorySplitChargeRuleEl>>,
}
