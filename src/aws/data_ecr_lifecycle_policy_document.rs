#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEcrLifecyclePolicyDocumentData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<DataEcrLifecyclePolicyDocument_Rule>>,
    dynamic: DataEcrLifecyclePolicyDocumentDynamic,
}

struct DataEcrLifecyclePolicyDocument_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEcrLifecyclePolicyDocumentData_>,
}

#[derive(Clone)]
pub struct DataEcrLifecyclePolicyDocument(std::rc::Rc<DataEcrLifecyclePolicyDocument_>);

impl DataEcrLifecyclePolicyDocument {
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

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataEcrLifecyclePolicyDocument_Rule>>,
    ) -> Self {
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

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<DataEcrLifecyclePolicyDocument_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }
}

impl terrars::Referable for DataEcrLifecyclePolicyDocument {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEcrLifecyclePolicyDocument { }

impl terrars::ToListMappable for DataEcrLifecyclePolicyDocument {
    type O = terrars::ListRef<DataEcrLifecyclePolicyDocumentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEcrLifecyclePolicyDocument_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ecr_lifecycle_policy_document".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEcrLifecyclePolicyDocument {
    pub tf_id: std::string::String,
}

impl BuildDataEcrLifecyclePolicyDocument {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEcrLifecyclePolicyDocument {
        let out = DataEcrLifecyclePolicyDocument(std::rc::Rc::new(DataEcrLifecyclePolicyDocument_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEcrLifecyclePolicyDocumentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                rule: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEcrLifecyclePolicyDocumentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcrLifecyclePolicyDocumentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEcrLifecyclePolicyDocumentRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<DataEcrLifecyclePolicyDocument_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataEcrLifecyclePolicyDocumentDynamic {
    rule: std::option::Option<terrars::DynamicBlock<DataEcrLifecyclePolicyDocument_Rule>>,
}

#[derive(Serialize)]
pub struct DataEcrLifecyclePolicyDocument_Rule_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl DataEcrLifecyclePolicyDocument_Rule_Action {
    #[doc = "Set the field `target_storage_class`.\n"]
    pub fn set_target_storage_class(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.target_storage_class = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcrLifecyclePolicyDocument_Rule_Action {
    type O = terrars::BlockAssignable<DataEcrLifecyclePolicyDocument_Rule_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcrLifecyclePolicyDocument_Rule_Action {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDataEcrLifecyclePolicyDocument_Rule_Action {
    pub fn build(self) -> DataEcrLifecyclePolicyDocument_Rule_Action {
        DataEcrLifecyclePolicyDocument_Rule_Action {
            target_storage_class: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct DataEcrLifecyclePolicyDocument_Rule_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcrLifecyclePolicyDocument_Rule_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcrLifecyclePolicyDocument_Rule_ActionRef {
        DataEcrLifecyclePolicyDocument_Rule_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcrLifecyclePolicyDocument_Rule_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_storage_class` after provisioning.\n"]
    pub fn target_storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_storage_class", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcrLifecyclePolicyDocument_Rule_Selection {
    count_number: terrars::PrimField<f64>,
    count_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count_unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_pattern_list: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_prefix_list: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    tag_status: terrars::PrimField<std::string::String>,
}

impl DataEcrLifecyclePolicyDocument_Rule_Selection {
    #[doc = "Set the field `count_unit`.\n"]
    pub fn set_count_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.count_unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_class`.\n"]
    pub fn set_storage_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_pattern_list`.\n"]
    pub fn set_tag_pattern_list(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tag_pattern_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_prefix_list`.\n"]
    pub fn set_tag_prefix_list(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tag_prefix_list = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcrLifecyclePolicyDocument_Rule_Selection {
    type O = terrars::BlockAssignable<DataEcrLifecyclePolicyDocument_Rule_Selection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcrLifecyclePolicyDocument_Rule_Selection {
    #[doc = ""]
    pub count_number: terrars::PrimField<f64>,
    #[doc = ""]
    pub count_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub tag_status: terrars::PrimField<std::string::String>,
}

impl BuildDataEcrLifecyclePolicyDocument_Rule_Selection {
    pub fn build(self) -> DataEcrLifecyclePolicyDocument_Rule_Selection {
        DataEcrLifecyclePolicyDocument_Rule_Selection {
            count_number: self.count_number,
            count_type: self.count_type,
            count_unit: core::default::Default::default(),
            storage_class: core::default::Default::default(),
            tag_pattern_list: core::default::Default::default(),
            tag_prefix_list: core::default::Default::default(),
            tag_status: self.tag_status,
        }
    }
}

#[allow(dead_code)]
pub struct DataEcrLifecyclePolicyDocument_Rule_SelectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcrLifecyclePolicyDocument_Rule_SelectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEcrLifecyclePolicyDocument_Rule_SelectionRef {
        DataEcrLifecyclePolicyDocument_Rule_SelectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcrLifecyclePolicyDocument_Rule_SelectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count_number` after provisioning.\n"]
    pub fn count_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count_number", self.base))
    }

    #[doc = "Get a reference to the value of field `count_type` after provisioning.\n"]
    pub fn count_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count_type", self.base))
    }

    #[doc = "Get a reference to the value of field `count_unit` after provisioning.\n"]
    pub fn count_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count_unit", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_pattern_list` after provisioning.\n"]
    pub fn tag_pattern_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_pattern_list", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_prefix_list` after provisioning.\n"]
    pub fn tag_prefix_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_prefix_list", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_status` after provisioning.\n"]
    pub fn tag_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_status", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEcrLifecyclePolicyDocument_RuleDynamic {
    action: std::option::Option<terrars::DynamicBlock<DataEcrLifecyclePolicyDocument_Rule_Action>>,
    selection: std::option::Option<terrars::DynamicBlock<DataEcrLifecyclePolicyDocument_Rule_Selection>>,
}

#[derive(Serialize)]
pub struct DataEcrLifecyclePolicyDocument_Rule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    priority: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<DataEcrLifecyclePolicyDocument_Rule_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selection: std::option::Option<std::vec::Vec<DataEcrLifecyclePolicyDocument_Rule_Selection>>,
    dynamic: DataEcrLifecyclePolicyDocument_RuleDynamic,
}

impl DataEcrLifecyclePolicyDocument_Rule {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataEcrLifecyclePolicyDocument_Rule_Action>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `selection`.\n"]
    pub fn set_selection(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataEcrLifecyclePolicyDocument_Rule_Selection>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.selection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.selection = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataEcrLifecyclePolicyDocument_Rule {
    type O = terrars::BlockAssignable<DataEcrLifecyclePolicyDocument_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcrLifecyclePolicyDocument_Rule {
    #[doc = ""]
    pub priority: terrars::PrimField<f64>,
}

impl BuildDataEcrLifecyclePolicyDocument_Rule {
    pub fn build(self) -> DataEcrLifecyclePolicyDocument_Rule {
        DataEcrLifecyclePolicyDocument_Rule {
            description: core::default::Default::default(),
            priority: self.priority,
            action: core::default::Default::default(),
            selection: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcrLifecyclePolicyDocument_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcrLifecyclePolicyDocument_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcrLifecyclePolicyDocument_RuleRef {
        DataEcrLifecyclePolicyDocument_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcrLifecyclePolicyDocument_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<DataEcrLifecyclePolicyDocument_Rule_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `selection` after provisioning.\n"]
    pub fn selection(&self) -> terrars::ListRef<DataEcrLifecyclePolicyDocument_Rule_SelectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selection", self.base))
    }
}
