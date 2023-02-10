use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CeCostCategoryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_start: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    rule_version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<CeCostCategoryRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    split_charge_rule: Option<Vec<CeCostCategorySplitChargeRuleEl>>,
    dynamic: CeCostCategoryDynamic,
}

struct CeCostCategory_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CeCostCategoryData>,
}

#[derive(Clone)]
pub struct CeCostCategory(Rc<CeCostCategory_>);

impl CeCostCategory {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `default_value`.\n"]
    pub fn set_default_value(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_value = Some(v.into());
        self
    }

    #[doc= "Set the field `effective_start`.\n"]
    pub fn set_effective_start(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().effective_start = Some(v.into());
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

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `rule`.\n"]
    pub fn set_rule(self, v: impl Into<BlockAssignable<CeCostCategoryRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `split_charge_rule`.\n"]
    pub fn set_split_charge_rule(self, v: impl Into<BlockAssignable<CeCostCategorySplitChargeRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().split_charge_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.split_charge_rule = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `rule_version` after provisioning.\n"]
    pub fn rule_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

impl Resource for CeCostCategory {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for CeCostCategory {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for CeCostCategory {
    type O = ListRef<CeCostCategoryRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for CeCostCategory_ {
    fn extract_resource_type(&self) -> String {
        "aws_ce_cost_category".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCeCostCategory {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub rule_version: PrimField<String>,
}

impl BuildCeCostCategory {
    pub fn build(self, stack: &mut Stack) -> CeCostCategory {
        let out = CeCostCategory(Rc::new(CeCostCategory_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CeCostCategoryData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                default_value: core::default::Default::default(),
                effective_start: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                rule_version: self.rule_version,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                rule: core::default::Default::default(),
                split_charge_rule: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CeCostCategoryRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeCostCategoryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CeCostCategoryRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `rule_version` after provisioning.\n"]
    pub fn rule_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CeCostCategoryRuleElInheritedValueEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension_name: Option<PrimField<String>>,
}

impl CeCostCategoryRuleElInheritedValueEl {
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

impl ToListMappable for CeCostCategoryRuleElInheritedValueEl {
    type O = BlockAssignable<CeCostCategoryRuleElInheritedValueEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for CeCostCategoryRuleElInheritedValueElRef {
    fn new(shared: StackShared, base: String) -> CeCostCategoryRuleElInheritedValueElRef {
        CeCostCategoryRuleElInheritedValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElInheritedValueElRef {
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
pub struct CeCostCategoryRuleElRuleElAndElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeCostCategoryRuleElRuleElAndElCostCategoryEl {
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

impl ToListMappable for CeCostCategoryRuleElRuleElAndElCostCategoryEl {
    type O = BlockAssignable<CeCostCategoryRuleElRuleElAndElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for CeCostCategoryRuleElRuleElAndElCostCategoryElRef {
    fn new(shared: StackShared, base: String) -> CeCostCategoryRuleElRuleElAndElCostCategoryElRef {
        CeCostCategoryRuleElRuleElAndElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElAndElCostCategoryElRef {
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
pub struct CeCostCategoryRuleElRuleElAndElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeCostCategoryRuleElRuleElAndElDimensionEl {
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

impl ToListMappable for CeCostCategoryRuleElRuleElAndElDimensionEl {
    type O = BlockAssignable<CeCostCategoryRuleElRuleElAndElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for CeCostCategoryRuleElRuleElAndElDimensionElRef {
    fn new(shared: StackShared, base: String) -> CeCostCategoryRuleElRuleElAndElDimensionElRef {
        CeCostCategoryRuleElRuleElAndElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElAndElDimensionElRef {
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
pub struct CeCostCategoryRuleElRuleElAndElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeCostCategoryRuleElRuleElAndElTagsEl {
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

impl ToListMappable for CeCostCategoryRuleElRuleElAndElTagsEl {
    type O = BlockAssignable<CeCostCategoryRuleElRuleElAndElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for CeCostCategoryRuleElRuleElAndElTagsElRef {
    fn new(shared: StackShared, base: String) -> CeCostCategoryRuleElRuleElAndElTagsElRef {
        CeCostCategoryRuleElRuleElAndElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElAndElTagsElRef {
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

#[derive(Serialize, Default)]
struct CeCostCategoryRuleElRuleElAndElDynamic {
    cost_category: Option<DynamicBlock<CeCostCategoryRuleElRuleElAndElCostCategoryEl>>,
    dimension: Option<DynamicBlock<CeCostCategoryRuleElRuleElAndElDimensionEl>>,
    tags: Option<DynamicBlock<CeCostCategoryRuleElRuleElAndElTagsEl>>,
}

#[derive(Serialize)]
pub struct CeCostCategoryRuleElRuleElAndEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<Vec<CeCostCategoryRuleElRuleElAndElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<Vec<CeCostCategoryRuleElRuleElAndElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CeCostCategoryRuleElRuleElAndElTagsEl>>,
    dynamic: CeCostCategoryRuleElRuleElAndElDynamic,
}

impl CeCostCategoryRuleElRuleElAndEl {
    #[doc= "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<BlockAssignable<CeCostCategoryRuleElRuleElAndElCostCategoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cost_category = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_category = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `dimension`.\n"]
    pub fn set_dimension(mut self, v: impl Into<BlockAssignable<CeCostCategoryRuleElRuleElAndElDimensionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimension = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<BlockAssignable<CeCostCategoryRuleElRuleElAndElTagsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tags = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CeCostCategoryRuleElRuleElAndEl {
    type O = BlockAssignable<CeCostCategoryRuleElRuleElAndEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElRuleElAndElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeCostCategoryRuleElRuleElAndElRef {
    fn new(shared: StackShared, base: String) -> CeCostCategoryRuleElRuleElAndElRef {
        CeCostCategoryRuleElRuleElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElAndElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> ListRef<CeCostCategoryRuleElRuleElAndElCostCategoryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc= "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<CeCostCategoryRuleElRuleElAndElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<CeCostCategoryRuleElRuleElAndElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct CeCostCategoryRuleElRuleElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeCostCategoryRuleElRuleElCostCategoryEl {
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

impl ToListMappable for CeCostCategoryRuleElRuleElCostCategoryEl {
    type O = BlockAssignable<CeCostCategoryRuleElRuleElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for CeCostCategoryRuleElRuleElCostCategoryElRef {
    fn new(shared: StackShared, base: String) -> CeCostCategoryRuleElRuleElCostCategoryElRef {
        CeCostCategoryRuleElRuleElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElCostCategoryElRef {
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
pub struct CeCostCategoryRuleElRuleElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeCostCategoryRuleElRuleElDimensionEl {
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

impl ToListMappable for CeCostCategoryRuleElRuleElDimensionEl {
    type O = BlockAssignable<CeCostCategoryRuleElRuleElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for CeCostCategoryRuleElRuleElDimensionElRef {
    fn new(shared: StackShared, base: String) -> CeCostCategoryRuleElRuleElDimensionElRef {
        CeCostCategoryRuleElRuleElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElDimensionElRef {
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
pub struct CeCostCategoryRuleElRuleElNotElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeCostCategoryRuleElRuleElNotElCostCategoryEl {
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

impl ToListMappable for CeCostCategoryRuleElRuleElNotElCostCategoryEl {
    type O = BlockAssignable<CeCostCategoryRuleElRuleElNotElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for CeCostCategoryRuleElRuleElNotElCostCategoryElRef {
    fn new(shared: StackShared, base: String) -> CeCostCategoryRuleElRuleElNotElCostCategoryElRef {
        CeCostCategoryRuleElRuleElNotElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElNotElCostCategoryElRef {
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
pub struct CeCostCategoryRuleElRuleElNotElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeCostCategoryRuleElRuleElNotElDimensionEl {
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

impl ToListMappable for CeCostCategoryRuleElRuleElNotElDimensionEl {
    type O = BlockAssignable<CeCostCategoryRuleElRuleElNotElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for CeCostCategoryRuleElRuleElNotElDimensionElRef {
    fn new(shared: StackShared, base: String) -> CeCostCategoryRuleElRuleElNotElDimensionElRef {
        CeCostCategoryRuleElRuleElNotElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElNotElDimensionElRef {
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
pub struct CeCostCategoryRuleElRuleElNotElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeCostCategoryRuleElRuleElNotElTagsEl {
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

impl ToListMappable for CeCostCategoryRuleElRuleElNotElTagsEl {
    type O = BlockAssignable<CeCostCategoryRuleElRuleElNotElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for CeCostCategoryRuleElRuleElNotElTagsElRef {
    fn new(shared: StackShared, base: String) -> CeCostCategoryRuleElRuleElNotElTagsElRef {
        CeCostCategoryRuleElRuleElNotElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElNotElTagsElRef {
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

#[derive(Serialize, Default)]
struct CeCostCategoryRuleElRuleElNotElDynamic {
    cost_category: Option<DynamicBlock<CeCostCategoryRuleElRuleElNotElCostCategoryEl>>,
    dimension: Option<DynamicBlock<CeCostCategoryRuleElRuleElNotElDimensionEl>>,
    tags: Option<DynamicBlock<CeCostCategoryRuleElRuleElNotElTagsEl>>,
}

#[derive(Serialize)]
pub struct CeCostCategoryRuleElRuleElNotEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<Vec<CeCostCategoryRuleElRuleElNotElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<Vec<CeCostCategoryRuleElRuleElNotElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CeCostCategoryRuleElRuleElNotElTagsEl>>,
    dynamic: CeCostCategoryRuleElRuleElNotElDynamic,
}

impl CeCostCategoryRuleElRuleElNotEl {
    #[doc= "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<BlockAssignable<CeCostCategoryRuleElRuleElNotElCostCategoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cost_category = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_category = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `dimension`.\n"]
    pub fn set_dimension(mut self, v: impl Into<BlockAssignable<CeCostCategoryRuleElRuleElNotElDimensionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimension = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<BlockAssignable<CeCostCategoryRuleElRuleElNotElTagsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tags = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CeCostCategoryRuleElRuleElNotEl {
    type O = BlockAssignable<CeCostCategoryRuleElRuleElNotEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElRuleElNotElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeCostCategoryRuleElRuleElNotElRef {
    fn new(shared: StackShared, base: String) -> CeCostCategoryRuleElRuleElNotElRef {
        CeCostCategoryRuleElRuleElNotElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElNotElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> ListRef<CeCostCategoryRuleElRuleElNotElCostCategoryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc= "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<CeCostCategoryRuleElRuleElNotElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<CeCostCategoryRuleElRuleElNotElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct CeCostCategoryRuleElRuleElOrElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeCostCategoryRuleElRuleElOrElCostCategoryEl {
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

impl ToListMappable for CeCostCategoryRuleElRuleElOrElCostCategoryEl {
    type O = BlockAssignable<CeCostCategoryRuleElRuleElOrElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for CeCostCategoryRuleElRuleElOrElCostCategoryElRef {
    fn new(shared: StackShared, base: String) -> CeCostCategoryRuleElRuleElOrElCostCategoryElRef {
        CeCostCategoryRuleElRuleElOrElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElOrElCostCategoryElRef {
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
pub struct CeCostCategoryRuleElRuleElOrElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeCostCategoryRuleElRuleElOrElDimensionEl {
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

impl ToListMappable for CeCostCategoryRuleElRuleElOrElDimensionEl {
    type O = BlockAssignable<CeCostCategoryRuleElRuleElOrElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for CeCostCategoryRuleElRuleElOrElDimensionElRef {
    fn new(shared: StackShared, base: String) -> CeCostCategoryRuleElRuleElOrElDimensionElRef {
        CeCostCategoryRuleElRuleElOrElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElOrElDimensionElRef {
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
pub struct CeCostCategoryRuleElRuleElOrElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeCostCategoryRuleElRuleElOrElTagsEl {
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

impl ToListMappable for CeCostCategoryRuleElRuleElOrElTagsEl {
    type O = BlockAssignable<CeCostCategoryRuleElRuleElOrElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for CeCostCategoryRuleElRuleElOrElTagsElRef {
    fn new(shared: StackShared, base: String) -> CeCostCategoryRuleElRuleElOrElTagsElRef {
        CeCostCategoryRuleElRuleElOrElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElOrElTagsElRef {
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

#[derive(Serialize, Default)]
struct CeCostCategoryRuleElRuleElOrElDynamic {
    cost_category: Option<DynamicBlock<CeCostCategoryRuleElRuleElOrElCostCategoryEl>>,
    dimension: Option<DynamicBlock<CeCostCategoryRuleElRuleElOrElDimensionEl>>,
    tags: Option<DynamicBlock<CeCostCategoryRuleElRuleElOrElTagsEl>>,
}

#[derive(Serialize)]
pub struct CeCostCategoryRuleElRuleElOrEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<Vec<CeCostCategoryRuleElRuleElOrElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<Vec<CeCostCategoryRuleElRuleElOrElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CeCostCategoryRuleElRuleElOrElTagsEl>>,
    dynamic: CeCostCategoryRuleElRuleElOrElDynamic,
}

impl CeCostCategoryRuleElRuleElOrEl {
    #[doc= "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<BlockAssignable<CeCostCategoryRuleElRuleElOrElCostCategoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cost_category = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_category = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `dimension`.\n"]
    pub fn set_dimension(mut self, v: impl Into<BlockAssignable<CeCostCategoryRuleElRuleElOrElDimensionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimension = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<BlockAssignable<CeCostCategoryRuleElRuleElOrElTagsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tags = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CeCostCategoryRuleElRuleElOrEl {
    type O = BlockAssignable<CeCostCategoryRuleElRuleElOrEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElRuleElOrElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeCostCategoryRuleElRuleElOrElRef {
    fn new(shared: StackShared, base: String) -> CeCostCategoryRuleElRuleElOrElRef {
        CeCostCategoryRuleElRuleElOrElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElOrElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> ListRef<CeCostCategoryRuleElRuleElOrElCostCategoryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc= "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<CeCostCategoryRuleElRuleElOrElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<CeCostCategoryRuleElRuleElOrElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct CeCostCategoryRuleElRuleElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeCostCategoryRuleElRuleElTagsEl {
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

impl ToListMappable for CeCostCategoryRuleElRuleElTagsEl {
    type O = BlockAssignable<CeCostCategoryRuleElRuleElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for CeCostCategoryRuleElRuleElTagsElRef {
    fn new(shared: StackShared, base: String) -> CeCostCategoryRuleElRuleElTagsElRef {
        CeCostCategoryRuleElRuleElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElTagsElRef {
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

#[derive(Serialize, Default)]
struct CeCostCategoryRuleElRuleElDynamic {
    and: Option<DynamicBlock<CeCostCategoryRuleElRuleElAndEl>>,
    cost_category: Option<DynamicBlock<CeCostCategoryRuleElRuleElCostCategoryEl>>,
    dimension: Option<DynamicBlock<CeCostCategoryRuleElRuleElDimensionEl>>,
    not: Option<DynamicBlock<CeCostCategoryRuleElRuleElNotEl>>,
    or: Option<DynamicBlock<CeCostCategoryRuleElRuleElOrEl>>,
    tags: Option<DynamicBlock<CeCostCategoryRuleElRuleElTagsEl>>,
}

#[derive(Serialize)]
pub struct CeCostCategoryRuleElRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<Vec<CeCostCategoryRuleElRuleElAndEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<Vec<CeCostCategoryRuleElRuleElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<Vec<CeCostCategoryRuleElRuleElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not: Option<Vec<CeCostCategoryRuleElRuleElNotEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    or: Option<Vec<CeCostCategoryRuleElRuleElOrEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CeCostCategoryRuleElRuleElTagsEl>>,
    dynamic: CeCostCategoryRuleElRuleElDynamic,
}

impl CeCostCategoryRuleElRuleEl {
    #[doc= "Set the field `and`.\n"]
    pub fn set_and(mut self, v: impl Into<BlockAssignable<CeCostCategoryRuleElRuleElAndEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.and = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.and = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `cost_category`.\n"]
    pub fn set_cost_category(mut self, v: impl Into<BlockAssignable<CeCostCategoryRuleElRuleElCostCategoryEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cost_category = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_category = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `dimension`.\n"]
    pub fn set_dimension(mut self, v: impl Into<BlockAssignable<CeCostCategoryRuleElRuleElDimensionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimension = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `not`.\n"]
    pub fn set_not(mut self, v: impl Into<BlockAssignable<CeCostCategoryRuleElRuleElNotEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.not = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.not = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `or`.\n"]
    pub fn set_or(mut self, v: impl Into<BlockAssignable<CeCostCategoryRuleElRuleElOrEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.or = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.or = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<BlockAssignable<CeCostCategoryRuleElRuleElTagsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tags = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CeCostCategoryRuleElRuleEl {
    type O = BlockAssignable<CeCostCategoryRuleElRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeCostCategoryRuleElRuleElRef {
    fn new(shared: StackShared, base: String) -> CeCostCategoryRuleElRuleElRef {
        CeCostCategoryRuleElRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> ListRef<CeCostCategoryRuleElRuleElCostCategoryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc= "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<CeCostCategoryRuleElRuleElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc= "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> ListRef<CeCostCategoryRuleElRuleElNotElRef> {
        ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<CeCostCategoryRuleElRuleElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct CeCostCategoryRuleElDynamic {
    inherited_value: Option<DynamicBlock<CeCostCategoryRuleElInheritedValueEl>>,
    rule: Option<DynamicBlock<CeCostCategoryRuleElRuleEl>>,
}

#[derive(Serialize)]
pub struct CeCostCategoryRuleEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inherited_value: Option<Vec<CeCostCategoryRuleElInheritedValueEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<CeCostCategoryRuleElRuleEl>>,
    dynamic: CeCostCategoryRuleElDynamic,
}

impl CeCostCategoryRuleEl {
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

    #[doc= "Set the field `inherited_value`.\n"]
    pub fn set_inherited_value(mut self, v: impl Into<BlockAssignable<CeCostCategoryRuleElInheritedValueEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inherited_value = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inherited_value = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl Into<BlockAssignable<CeCostCategoryRuleElRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rule = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CeCostCategoryRuleEl {
    type O = BlockAssignable<CeCostCategoryRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct CeCostCategoryRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeCostCategoryRuleElRef {
    fn new(shared: StackShared, base: String) -> CeCostCategoryRuleElRef {
        CeCostCategoryRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategoryRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc= "Get a reference to the value of field `inherited_value` after provisioning.\n"]
    pub fn inherited_value(&self) -> ListRef<CeCostCategoryRuleElInheritedValueElRef> {
        ListRef::new(self.shared().clone(), format!("{}.inherited_value", self.base))
    }

    #[doc= "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<CeCostCategoryRuleElRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}

#[derive(Serialize)]
pub struct CeCostCategorySplitChargeRuleElParameterEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}

impl CeCostCategorySplitChargeRuleElParameterEl {
    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for CeCostCategorySplitChargeRuleElParameterEl {
    type O = BlockAssignable<CeCostCategorySplitChargeRuleElParameterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for CeCostCategorySplitChargeRuleElParameterElRef {
    fn new(shared: StackShared, base: String) -> CeCostCategorySplitChargeRuleElParameterElRef {
        CeCostCategorySplitChargeRuleElParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategorySplitChargeRuleElParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct CeCostCategorySplitChargeRuleElDynamic {
    parameter: Option<DynamicBlock<CeCostCategorySplitChargeRuleElParameterEl>>,
}

#[derive(Serialize)]
pub struct CeCostCategorySplitChargeRuleEl {
    method: PrimField<String>,
    source: PrimField<String>,
    targets: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<Vec<CeCostCategorySplitChargeRuleElParameterEl>>,
    dynamic: CeCostCategorySplitChargeRuleElDynamic,
}

impl CeCostCategorySplitChargeRuleEl {
    #[doc= "Set the field `parameter`.\n"]
    pub fn set_parameter(mut self, v: impl Into<BlockAssignable<CeCostCategorySplitChargeRuleElParameterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parameter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parameter = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CeCostCategorySplitChargeRuleEl {
    type O = BlockAssignable<CeCostCategorySplitChargeRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeCostCategorySplitChargeRuleEl {
    #[doc= ""]
    pub method: PrimField<String>,
    #[doc= ""]
    pub source: PrimField<String>,
    #[doc= ""]
    pub targets: SetField<PrimField<String>>,
}

impl BuildCeCostCategorySplitChargeRuleEl {
    pub fn build(self) -> CeCostCategorySplitChargeRuleEl {
        CeCostCategorySplitChargeRuleEl {
            method: self.method,
            source: self.source,
            targets: self.targets,
            parameter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CeCostCategorySplitChargeRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeCostCategorySplitChargeRuleElRef {
    fn new(shared: StackShared, base: String) -> CeCostCategorySplitChargeRuleElRef {
        CeCostCategorySplitChargeRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeCostCategorySplitChargeRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
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

#[derive(Serialize, Default)]
struct CeCostCategoryDynamic {
    rule: Option<DynamicBlock<CeCostCategoryRuleEl>>,
    split_charge_rule: Option<DynamicBlock<CeCostCategorySplitChargeRuleEl>>,
}
