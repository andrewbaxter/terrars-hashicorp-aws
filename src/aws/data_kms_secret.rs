use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataKmsSecretData {
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
    secret: std::option::Option<std::vec::Vec<DataKmsSecretSecretEl>>,
    dynamic: DataKmsSecretDynamic,
}

struct DataKmsSecret_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataKmsSecretData>,
}

#[derive(Clone)]
pub struct DataKmsSecret(std::rc::Rc<DataKmsSecret_>);

impl DataKmsSecret {
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

    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(self, v: impl std::convert::Into<terrars::BlockAssignable<DataKmsSecretSecretEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secret = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secret = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

impl terrars::Referable for DataKmsSecret {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataKmsSecret { }

impl terrars::ToListMappable for DataKmsSecret {
    type O = terrars::ListRef<DataKmsSecretRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataKmsSecret_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_kms_secret".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataKmsSecret {
    pub tf_id: std::string::String,
}

impl BuildDataKmsSecret {
    pub fn build(self, stack: &mut terrars::Stack) -> DataKmsSecret {
        let out = DataKmsSecret(std::rc::Rc::new(DataKmsSecret_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataKmsSecretData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                secret: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataKmsSecretRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKmsSecretRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataKmsSecretRef {
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
}

#[derive(Serialize)]
pub struct DataKmsSecretSecretEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    context: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grant_tokens: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    payload: terrars::PrimField<std::string::String>,
}

impl DataKmsSecretSecretEl {
    #[doc = "Set the field `context`.\n"]
    pub fn set_context(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grant_tokens`.\n"]
    pub fn set_grant_tokens(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.grant_tokens = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKmsSecretSecretEl {
    type O = terrars::BlockAssignable<DataKmsSecretSecretEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKmsSecretSecretEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub payload: terrars::PrimField<std::string::String>,
}

impl BuildDataKmsSecretSecretEl {
    pub fn build(self) -> DataKmsSecretSecretEl {
        DataKmsSecretSecretEl {
            context: core::default::Default::default(),
            grant_tokens: core::default::Default::default(),
            name: self.name,
            payload: self.payload,
        }
    }
}

pub struct DataKmsSecretSecretElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKmsSecretSecretElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKmsSecretSecretElRef {
        DataKmsSecretSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKmsSecretSecretElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `context` after provisioning.\n"]
    pub fn context(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.context", self.base))
    }

    #[doc = "Get a reference to the value of field `grant_tokens` after provisioning.\n"]
    pub fn grant_tokens(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grant_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `payload` after provisioning.\n"]
    pub fn payload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataKmsSecretDynamic {
    secret: std::option::Option<terrars::DynamicBlock<DataKmsSecretSecretEl>>,
}
