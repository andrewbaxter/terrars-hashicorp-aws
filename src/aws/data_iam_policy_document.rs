#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataIamPolicyDocumentData_ {
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
    override_json: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_policy_documents: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_json: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_policy_documents: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statement: std::option::Option<std::vec::Vec<DataIamPolicyDocument_Statement>>,
    dynamic: DataIamPolicyDocumentDynamic,
}

struct DataIamPolicyDocument_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataIamPolicyDocumentData_>,
}

#[derive(Clone)]
pub struct DataIamPolicyDocument(std::rc::Rc<DataIamPolicyDocument_>);

impl DataIamPolicyDocument {
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

    #[doc = "Set the field `override_json`.\n"]
    pub fn set_override_json(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().override_json = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `override_policy_documents`.\n"]
    pub fn set_override_policy_documents(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().override_policy_documents = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_id`.\n"]
    pub fn set_policy_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().policy_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_json`.\n"]
    pub fn set_source_json(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source_json = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_policy_documents`.\n"]
    pub fn set_source_policy_documents(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().source_policy_documents = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `statement`.\n"]
    pub fn set_statement(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataIamPolicyDocument_Statement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `minified_json` after provisioning.\n"]
    pub fn minified_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minified_json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `override_json` after provisioning.\n"]
    pub fn override_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override_json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `override_policy_documents` after provisioning.\n"]
    pub fn override_policy_documents(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.override_policy_documents", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\n"]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_json` after provisioning.\n"]
    pub fn source_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_policy_documents` after provisioning.\n"]
    pub fn source_policy_documents(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_policy_documents", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(&self) -> terrars::ListRef<DataIamPolicyDocument_StatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statement", self.extract_ref()))
    }
}

impl terrars::Referable for DataIamPolicyDocument {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataIamPolicyDocument { }

impl terrars::ToListMappable for DataIamPolicyDocument {
    type O = terrars::ListRef<DataIamPolicyDocumentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataIamPolicyDocument_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_iam_policy_document".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIamPolicyDocument {
    pub tf_id: std::string::String,
}

impl BuildDataIamPolicyDocument {
    pub fn build(self, stack: &mut terrars::Stack) -> DataIamPolicyDocument {
        let out = DataIamPolicyDocument(std::rc::Rc::new(DataIamPolicyDocument_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataIamPolicyDocumentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                override_json: core::default::Default::default(),
                override_policy_documents: core::default::Default::default(),
                policy_id: core::default::Default::default(),
                source_json: core::default::Default::default(),
                source_policy_documents: core::default::Default::default(),
                version: core::default::Default::default(),
                statement: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIamPolicyDocumentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamPolicyDocumentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataIamPolicyDocumentRef {
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

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `minified_json` after provisioning.\n"]
    pub fn minified_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minified_json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `override_json` after provisioning.\n"]
    pub fn override_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override_json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `override_policy_documents` after provisioning.\n"]
    pub fn override_policy_documents(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.override_policy_documents", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\n"]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_json` after provisioning.\n"]
    pub fn source_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_policy_documents` after provisioning.\n"]
    pub fn source_policy_documents(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_policy_documents", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(&self) -> terrars::ListRef<DataIamPolicyDocument_StatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statement", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataIamPolicyDocumentDynamic {
    statement: std::option::Option<terrars::DynamicBlock<DataIamPolicyDocument_Statement>>,
}

#[derive(Serialize)]
pub struct DataIamPolicyDocument_Statement_Condition {
    test: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
    variable: terrars::PrimField<std::string::String>,
}

impl DataIamPolicyDocument_Statement_Condition { }

impl terrars::ToListMappable for DataIamPolicyDocument_Statement_Condition {
    type O = terrars::BlockAssignable<DataIamPolicyDocument_Statement_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamPolicyDocument_Statement_Condition {
    #[doc = ""]
    pub test: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub variable: terrars::PrimField<std::string::String>,
}

impl BuildDataIamPolicyDocument_Statement_Condition {
    pub fn build(self) -> DataIamPolicyDocument_Statement_Condition {
        DataIamPolicyDocument_Statement_Condition {
            test: self.test,
            values: self.values,
            variable: self.variable,
        }
    }
}

#[allow(dead_code)]
pub struct DataIamPolicyDocument_Statement_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamPolicyDocument_Statement_ConditionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIamPolicyDocument_Statement_ConditionRef {
        DataIamPolicyDocument_Statement_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamPolicyDocument_Statement_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `test` after provisioning.\n"]
    pub fn test(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }

    #[doc = "Get a reference to the value of field `variable` after provisioning.\n"]
    pub fn variable(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.variable", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIamPolicyDocument_Statement_NotPrincipals {
    identifiers: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl DataIamPolicyDocument_Statement_NotPrincipals { }

impl terrars::ToListMappable for DataIamPolicyDocument_Statement_NotPrincipals {
    type O = terrars::BlockAssignable<DataIamPolicyDocument_Statement_NotPrincipals>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamPolicyDocument_Statement_NotPrincipals {
    #[doc = ""]
    pub identifiers: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDataIamPolicyDocument_Statement_NotPrincipals {
    pub fn build(self) -> DataIamPolicyDocument_Statement_NotPrincipals {
        DataIamPolicyDocument_Statement_NotPrincipals {
            identifiers: self.identifiers,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct DataIamPolicyDocument_Statement_NotPrincipalsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamPolicyDocument_Statement_NotPrincipalsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataIamPolicyDocument_Statement_NotPrincipalsRef {
        DataIamPolicyDocument_Statement_NotPrincipalsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamPolicyDocument_Statement_NotPrincipalsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identifiers` after provisioning.\n"]
    pub fn identifiers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.identifiers", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIamPolicyDocument_Statement_Principals {
    identifiers: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl DataIamPolicyDocument_Statement_Principals { }

impl terrars::ToListMappable for DataIamPolicyDocument_Statement_Principals {
    type O = terrars::BlockAssignable<DataIamPolicyDocument_Statement_Principals>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamPolicyDocument_Statement_Principals {
    #[doc = ""]
    pub identifiers: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDataIamPolicyDocument_Statement_Principals {
    pub fn build(self) -> DataIamPolicyDocument_Statement_Principals {
        DataIamPolicyDocument_Statement_Principals {
            identifiers: self.identifiers,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct DataIamPolicyDocument_Statement_PrincipalsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamPolicyDocument_Statement_PrincipalsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIamPolicyDocument_Statement_PrincipalsRef {
        DataIamPolicyDocument_Statement_PrincipalsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamPolicyDocument_Statement_PrincipalsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identifiers` after provisioning.\n"]
    pub fn identifiers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.identifiers", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataIamPolicyDocument_StatementDynamic {
    condition: std::option::Option<terrars::DynamicBlock<DataIamPolicyDocument_Statement_Condition>>,
    not_principals: std::option::Option<terrars::DynamicBlock<DataIamPolicyDocument_Statement_NotPrincipals>>,
    principals: std::option::Option<terrars::DynamicBlock<DataIamPolicyDocument_Statement_Principals>>,
}

#[derive(Serialize)]
pub struct DataIamPolicyDocument_Statement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_actions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<DataIamPolicyDocument_Statement_Condition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_principals: std::option::Option<std::vec::Vec<DataIamPolicyDocument_Statement_NotPrincipals>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principals: std::option::Option<std::vec::Vec<DataIamPolicyDocument_Statement_Principals>>,
    dynamic: DataIamPolicyDocument_StatementDynamic,
}

impl DataIamPolicyDocument_Statement {
    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.actions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `effect`.\n"]
    pub fn set_effect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.effect = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_actions`.\n"]
    pub fn set_not_actions(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.not_actions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_resources`.\n"]
    pub fn set_not_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.not_resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sid`.\n"]
    pub fn set_sid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataIamPolicyDocument_Statement_Condition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `not_principals`.\n"]
    pub fn set_not_principals(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataIamPolicyDocument_Statement_NotPrincipals>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.not_principals = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.not_principals = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `principals`.\n"]
    pub fn set_principals(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataIamPolicyDocument_Statement_Principals>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.principals = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.principals = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataIamPolicyDocument_Statement {
    type O = terrars::BlockAssignable<DataIamPolicyDocument_Statement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamPolicyDocument_Statement {}

impl BuildDataIamPolicyDocument_Statement {
    pub fn build(self) -> DataIamPolicyDocument_Statement {
        DataIamPolicyDocument_Statement {
            actions: core::default::Default::default(),
            effect: core::default::Default::default(),
            not_actions: core::default::Default::default(),
            not_resources: core::default::Default::default(),
            resources: core::default::Default::default(),
            sid: core::default::Default::default(),
            condition: core::default::Default::default(),
            not_principals: core::default::Default::default(),
            principals: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIamPolicyDocument_StatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamPolicyDocument_StatementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIamPolicyDocument_StatementRef {
        DataIamPolicyDocument_StatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamPolicyDocument_StatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }

    #[doc = "Get a reference to the value of field `effect` after provisioning.\n"]
    pub fn effect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }

    #[doc = "Get a reference to the value of field `not_actions` after provisioning.\n"]
    pub fn not_actions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.not_actions", self.base))
    }

    #[doc = "Get a reference to the value of field `not_resources` after provisioning.\n"]
    pub fn not_resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.not_resources", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `sid` after provisioning.\n"]
    pub fn sid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sid", self.base))
    }
}
