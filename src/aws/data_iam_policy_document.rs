use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataIamPolicyDocumentData {
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
    statement: std::option::Option<std::vec::Vec<DataIamPolicyDocumentStatementEl>>,
    dynamic: DataIamPolicyDocumentDynamic,
}

struct DataIamPolicyDocument_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataIamPolicyDocumentData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataIamPolicyDocumentStatementEl>>,
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
    pub fn statement(&self) -> terrars::ListRef<DataIamPolicyDocumentStatementElRef> {
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
            data: std::cell::RefCell::new(DataIamPolicyDocumentData {
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
    pub fn statement(&self) -> terrars::ListRef<DataIamPolicyDocumentStatementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statement", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataIamPolicyDocumentStatementElConditionEl {
    test: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
    variable: terrars::PrimField<std::string::String>,
}

impl DataIamPolicyDocumentStatementElConditionEl { }

impl terrars::ToListMappable for DataIamPolicyDocumentStatementElConditionEl {
    type O = terrars::BlockAssignable<DataIamPolicyDocumentStatementElConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamPolicyDocumentStatementElConditionEl {
    #[doc = ""]
    pub test: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub variable: terrars::PrimField<std::string::String>,
}

impl BuildDataIamPolicyDocumentStatementElConditionEl {
    pub fn build(self) -> DataIamPolicyDocumentStatementElConditionEl {
        DataIamPolicyDocumentStatementElConditionEl {
            test: self.test,
            values: self.values,
            variable: self.variable,
        }
    }
}

pub struct DataIamPolicyDocumentStatementElConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamPolicyDocumentStatementElConditionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataIamPolicyDocumentStatementElConditionElRef {
        DataIamPolicyDocumentStatementElConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamPolicyDocumentStatementElConditionElRef {
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
pub struct DataIamPolicyDocumentStatementElNotPrincipalsEl {
    identifiers: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl DataIamPolicyDocumentStatementElNotPrincipalsEl { }

impl terrars::ToListMappable for DataIamPolicyDocumentStatementElNotPrincipalsEl {
    type O = terrars::BlockAssignable<DataIamPolicyDocumentStatementElNotPrincipalsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamPolicyDocumentStatementElNotPrincipalsEl {
    #[doc = ""]
    pub identifiers: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDataIamPolicyDocumentStatementElNotPrincipalsEl {
    pub fn build(self) -> DataIamPolicyDocumentStatementElNotPrincipalsEl {
        DataIamPolicyDocumentStatementElNotPrincipalsEl {
            identifiers: self.identifiers,
            type_: self.type_,
        }
    }
}

pub struct DataIamPolicyDocumentStatementElNotPrincipalsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamPolicyDocumentStatementElNotPrincipalsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataIamPolicyDocumentStatementElNotPrincipalsElRef {
        DataIamPolicyDocumentStatementElNotPrincipalsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamPolicyDocumentStatementElNotPrincipalsElRef {
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
pub struct DataIamPolicyDocumentStatementElPrincipalsEl {
    identifiers: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl DataIamPolicyDocumentStatementElPrincipalsEl { }

impl terrars::ToListMappable for DataIamPolicyDocumentStatementElPrincipalsEl {
    type O = terrars::BlockAssignable<DataIamPolicyDocumentStatementElPrincipalsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamPolicyDocumentStatementElPrincipalsEl {
    #[doc = ""]
    pub identifiers: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDataIamPolicyDocumentStatementElPrincipalsEl {
    pub fn build(self) -> DataIamPolicyDocumentStatementElPrincipalsEl {
        DataIamPolicyDocumentStatementElPrincipalsEl {
            identifiers: self.identifiers,
            type_: self.type_,
        }
    }
}

pub struct DataIamPolicyDocumentStatementElPrincipalsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamPolicyDocumentStatementElPrincipalsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataIamPolicyDocumentStatementElPrincipalsElRef {
        DataIamPolicyDocumentStatementElPrincipalsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamPolicyDocumentStatementElPrincipalsElRef {
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
struct DataIamPolicyDocumentStatementElDynamic {
    condition: std::option::Option<terrars::DynamicBlock<DataIamPolicyDocumentStatementElConditionEl>>,
    not_principals: std::option::Option<terrars::DynamicBlock<DataIamPolicyDocumentStatementElNotPrincipalsEl>>,
    principals: std::option::Option<terrars::DynamicBlock<DataIamPolicyDocumentStatementElPrincipalsEl>>,
}

#[derive(Serialize)]
pub struct DataIamPolicyDocumentStatementEl {
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
    condition: std::option::Option<std::vec::Vec<DataIamPolicyDocumentStatementElConditionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_principals: std::option::Option<std::vec::Vec<DataIamPolicyDocumentStatementElNotPrincipalsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principals: std::option::Option<std::vec::Vec<DataIamPolicyDocumentStatementElPrincipalsEl>>,
    dynamic: DataIamPolicyDocumentStatementElDynamic,
}

impl DataIamPolicyDocumentStatementEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataIamPolicyDocumentStatementElConditionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataIamPolicyDocumentStatementElNotPrincipalsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataIamPolicyDocumentStatementElPrincipalsEl>>,
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

impl terrars::ToListMappable for DataIamPolicyDocumentStatementEl {
    type O = terrars::BlockAssignable<DataIamPolicyDocumentStatementEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamPolicyDocumentStatementEl {}

impl BuildDataIamPolicyDocumentStatementEl {
    pub fn build(self) -> DataIamPolicyDocumentStatementEl {
        DataIamPolicyDocumentStatementEl {
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

pub struct DataIamPolicyDocumentStatementElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamPolicyDocumentStatementElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIamPolicyDocumentStatementElRef {
        DataIamPolicyDocumentStatementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamPolicyDocumentStatementElRef {
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

#[derive(Serialize, Default)]
struct DataIamPolicyDocumentDynamic {
    statement: std::option::Option<terrars::DynamicBlock<DataIamPolicyDocumentStatementEl>>,
}
