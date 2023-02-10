use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct WafSqlInjectionMatchSetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_injection_match_tuples: Option<Vec<WafSqlInjectionMatchSetSqlInjectionMatchTuplesEl>>,
    dynamic: WafSqlInjectionMatchSetDynamic,
}

struct WafSqlInjectionMatchSet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WafSqlInjectionMatchSetData>,
}

#[derive(Clone)]
pub struct WafSqlInjectionMatchSet(Rc<WafSqlInjectionMatchSet_>);

impl WafSqlInjectionMatchSet {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `sql_injection_match_tuples`.\n"]
    pub fn set_sql_injection_match_tuples(
        self,
        v: impl Into<BlockAssignable<WafSqlInjectionMatchSetSqlInjectionMatchTuplesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sql_injection_match_tuples = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sql_injection_match_tuples = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

impl Referable for WafSqlInjectionMatchSet {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for WafSqlInjectionMatchSet { }

impl ToListMappable for WafSqlInjectionMatchSet {
    type O = ListRef<WafSqlInjectionMatchSetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for WafSqlInjectionMatchSet_ {
    fn extract_resource_type(&self) -> String {
        "aws_waf_sql_injection_match_set".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWafSqlInjectionMatchSet {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildWafSqlInjectionMatchSet {
    pub fn build(self, stack: &mut Stack) -> WafSqlInjectionMatchSet {
        let out = WafSqlInjectionMatchSet(Rc::new(WafSqlInjectionMatchSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WafSqlInjectionMatchSetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                sql_injection_match_tuples: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WafSqlInjectionMatchSetRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafSqlInjectionMatchSetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WafSqlInjectionMatchSetRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct WafSqlInjectionMatchSetSqlInjectionMatchTuplesElFieldToMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl WafSqlInjectionMatchSetSqlInjectionMatchTuplesElFieldToMatchEl {
    #[doc= "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data = Some(v.into());
        self
    }
}

impl ToListMappable for WafSqlInjectionMatchSetSqlInjectionMatchTuplesElFieldToMatchEl {
    type O = BlockAssignable<WafSqlInjectionMatchSetSqlInjectionMatchTuplesElFieldToMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafSqlInjectionMatchSetSqlInjectionMatchTuplesElFieldToMatchEl {
    #[doc= ""]
    pub type_: PrimField<String>,
}

impl BuildWafSqlInjectionMatchSetSqlInjectionMatchTuplesElFieldToMatchEl {
    pub fn build(self) -> WafSqlInjectionMatchSetSqlInjectionMatchTuplesElFieldToMatchEl {
        WafSqlInjectionMatchSetSqlInjectionMatchTuplesElFieldToMatchEl {
            data: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct WafSqlInjectionMatchSetSqlInjectionMatchTuplesElFieldToMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafSqlInjectionMatchSetSqlInjectionMatchTuplesElFieldToMatchElRef {
    fn new(shared: StackShared, base: String) -> WafSqlInjectionMatchSetSqlInjectionMatchTuplesElFieldToMatchElRef {
        WafSqlInjectionMatchSetSqlInjectionMatchTuplesElFieldToMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafSqlInjectionMatchSetSqlInjectionMatchTuplesElFieldToMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct WafSqlInjectionMatchSetSqlInjectionMatchTuplesElDynamic {
    field_to_match: Option<DynamicBlock<WafSqlInjectionMatchSetSqlInjectionMatchTuplesElFieldToMatchEl>>,
}

#[derive(Serialize)]
pub struct WafSqlInjectionMatchSetSqlInjectionMatchTuplesEl {
    text_transformation: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_to_match: Option<Vec<WafSqlInjectionMatchSetSqlInjectionMatchTuplesElFieldToMatchEl>>,
    dynamic: WafSqlInjectionMatchSetSqlInjectionMatchTuplesElDynamic,
}

impl WafSqlInjectionMatchSetSqlInjectionMatchTuplesEl {
    #[doc= "Set the field `field_to_match`.\n"]
    pub fn set_field_to_match(
        mut self,
        v: impl Into<BlockAssignable<WafSqlInjectionMatchSetSqlInjectionMatchTuplesElFieldToMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.field_to_match = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.field_to_match = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for WafSqlInjectionMatchSetSqlInjectionMatchTuplesEl {
    type O = BlockAssignable<WafSqlInjectionMatchSetSqlInjectionMatchTuplesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafSqlInjectionMatchSetSqlInjectionMatchTuplesEl {
    #[doc= ""]
    pub text_transformation: PrimField<String>,
}

impl BuildWafSqlInjectionMatchSetSqlInjectionMatchTuplesEl {
    pub fn build(self) -> WafSqlInjectionMatchSetSqlInjectionMatchTuplesEl {
        WafSqlInjectionMatchSetSqlInjectionMatchTuplesEl {
            text_transformation: self.text_transformation,
            field_to_match: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct WafSqlInjectionMatchSetSqlInjectionMatchTuplesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafSqlInjectionMatchSetSqlInjectionMatchTuplesElRef {
    fn new(shared: StackShared, base: String) -> WafSqlInjectionMatchSetSqlInjectionMatchTuplesElRef {
        WafSqlInjectionMatchSetSqlInjectionMatchTuplesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafSqlInjectionMatchSetSqlInjectionMatchTuplesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `text_transformation` after provisioning.\n"]
    pub fn text_transformation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text_transformation", self.base))
    }

    #[doc= "Get a reference to the value of field `field_to_match` after provisioning.\n"]
    pub fn field_to_match(&self) -> ListRef<WafSqlInjectionMatchSetSqlInjectionMatchTuplesElFieldToMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.field_to_match", self.base))
    }
}

#[derive(Serialize, Default)]
struct WafSqlInjectionMatchSetDynamic {
    sql_injection_match_tuples: Option<DynamicBlock<WafSqlInjectionMatchSetSqlInjectionMatchTuplesEl>>,
}
