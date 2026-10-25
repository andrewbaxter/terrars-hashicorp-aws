#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct WafByteMatchSetData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    byte_match_tuples: std::option::Option<std::vec::Vec<WafByteMatchSet_ByteMatchTuples>>,
    dynamic: WafByteMatchSetDynamic,
}

struct WafByteMatchSet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WafByteMatchSetData_>,
}

#[derive(Clone)]
pub struct WafByteMatchSet(std::rc::Rc<WafByteMatchSet_>);

impl WafByteMatchSet {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `byte_match_tuples`.\n"]
    pub fn set_byte_match_tuples(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WafByteMatchSet_ByteMatchTuples>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().byte_match_tuples = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.byte_match_tuples = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

impl terrars::Referable for WafByteMatchSet {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for WafByteMatchSet { }

impl terrars::ToListMappable for WafByteMatchSet {
    type O = terrars::ListRef<WafByteMatchSetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for WafByteMatchSet_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_waf_byte_match_set".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWafByteMatchSet {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWafByteMatchSet {
    pub fn build(self, stack: &mut terrars::Stack) -> WafByteMatchSet {
        let out = WafByteMatchSet(std::rc::Rc::new(WafByteMatchSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WafByteMatchSetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                byte_match_tuples: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WafByteMatchSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafByteMatchSetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WafByteMatchSetRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct WafByteMatchSetDynamic {
    byte_match_tuples: std::option::Option<terrars::DynamicBlock<WafByteMatchSet_ByteMatchTuples>>,
}

#[derive(Serialize)]
pub struct WafByteMatchSet_ByteMatchTuples_FieldToMatch {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl WafByteMatchSet_ByteMatchTuples_FieldToMatch {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WafByteMatchSet_ByteMatchTuples_FieldToMatch {
    type O = terrars::BlockAssignable<WafByteMatchSet_ByteMatchTuples_FieldToMatch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafByteMatchSet_ByteMatchTuples_FieldToMatch {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWafByteMatchSet_ByteMatchTuples_FieldToMatch {
    pub fn build(self) -> WafByteMatchSet_ByteMatchTuples_FieldToMatch {
        WafByteMatchSet_ByteMatchTuples_FieldToMatch {
            data: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct WafByteMatchSet_ByteMatchTuples_FieldToMatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafByteMatchSet_ByteMatchTuples_FieldToMatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WafByteMatchSet_ByteMatchTuples_FieldToMatchRef {
        WafByteMatchSet_ByteMatchTuples_FieldToMatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafByteMatchSet_ByteMatchTuples_FieldToMatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct WafByteMatchSet_ByteMatchTuplesDynamic {
    field_to_match: std::option::Option<terrars::DynamicBlock<WafByteMatchSet_ByteMatchTuples_FieldToMatch>>,
}

#[derive(Serialize)]
pub struct WafByteMatchSet_ByteMatchTuples {
    positional_constraint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_string: std::option::Option<terrars::PrimField<std::string::String>>,
    text_transformation: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_to_match: std::option::Option<std::vec::Vec<WafByteMatchSet_ByteMatchTuples_FieldToMatch>>,
    dynamic: WafByteMatchSet_ByteMatchTuplesDynamic,
}

impl WafByteMatchSet_ByteMatchTuples {
    #[doc = "Set the field `target_string`.\n"]
    pub fn set_target_string(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field_to_match`.\n"]
    pub fn set_field_to_match(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WafByteMatchSet_ByteMatchTuples_FieldToMatch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_to_match = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_to_match = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for WafByteMatchSet_ByteMatchTuples {
    type O = terrars::BlockAssignable<WafByteMatchSet_ByteMatchTuples>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafByteMatchSet_ByteMatchTuples {
    #[doc = ""]
    pub positional_constraint: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub text_transformation: terrars::PrimField<std::string::String>,
}

impl BuildWafByteMatchSet_ByteMatchTuples {
    pub fn build(self) -> WafByteMatchSet_ByteMatchTuples {
        WafByteMatchSet_ByteMatchTuples {
            positional_constraint: self.positional_constraint,
            target_string: core::default::Default::default(),
            text_transformation: self.text_transformation,
            field_to_match: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WafByteMatchSet_ByteMatchTuplesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafByteMatchSet_ByteMatchTuplesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WafByteMatchSet_ByteMatchTuplesRef {
        WafByteMatchSet_ByteMatchTuplesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafByteMatchSet_ByteMatchTuplesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `positional_constraint` after provisioning.\n"]
    pub fn positional_constraint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.positional_constraint", self.base))
    }

    #[doc = "Get a reference to the value of field `target_string` after provisioning.\n"]
    pub fn target_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_string", self.base))
    }

    #[doc = "Get a reference to the value of field `text_transformation` after provisioning.\n"]
    pub fn text_transformation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text_transformation", self.base))
    }

    #[doc = "Get a reference to the value of field `field_to_match` after provisioning.\n"]
    pub fn field_to_match(&self) -> terrars::ListRef<WafByteMatchSet_ByteMatchTuples_FieldToMatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_to_match", self.base))
    }
}
