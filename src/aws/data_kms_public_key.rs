use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataKmsPublicKeyData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grant_tokens: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    key_id: terrars::PrimField<std::string::String>,
}

struct DataKmsPublicKey_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataKmsPublicKeyData>,
}

#[derive(Clone)]
pub struct DataKmsPublicKey(std::rc::Rc<DataKmsPublicKey_>);

impl DataKmsPublicKey {
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

    #[doc = "Set the field `grant_tokens`.\n"]
    pub fn set_grant_tokens(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().grant_tokens = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_master_key_spec` after provisioning.\n"]
    pub fn customer_master_key_spec(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_master_key_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_algorithms` after provisioning.\n"]
    pub fn encryption_algorithms(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_algorithms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grant_tokens` after provisioning.\n"]
    pub fn grant_tokens(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grant_tokens", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_usage` after provisioning.\n"]
    pub fn key_usage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_usage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_key_pem` after provisioning.\n"]
    pub fn public_key_pem(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key_pem", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signing_algorithms` after provisioning.\n"]
    pub fn signing_algorithms(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.signing_algorithms", self.extract_ref()))
    }
}

impl terrars::Referable for DataKmsPublicKey {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataKmsPublicKey { }

impl terrars::ToListMappable for DataKmsPublicKey {
    type O = terrars::ListRef<DataKmsPublicKeyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataKmsPublicKey_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_kms_public_key".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataKmsPublicKey {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub key_id: terrars::PrimField<std::string::String>,
}

impl BuildDataKmsPublicKey {
    pub fn build(self, stack: &mut terrars::Stack) -> DataKmsPublicKey {
        let out = DataKmsPublicKey(std::rc::Rc::new(DataKmsPublicKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataKmsPublicKeyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                grant_tokens: core::default::Default::default(),
                id: core::default::Default::default(),
                key_id: self.key_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataKmsPublicKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKmsPublicKeyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataKmsPublicKeyRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_master_key_spec` after provisioning.\n"]
    pub fn customer_master_key_spec(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_master_key_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_algorithms` after provisioning.\n"]
    pub fn encryption_algorithms(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_algorithms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grant_tokens` after provisioning.\n"]
    pub fn grant_tokens(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grant_tokens", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_usage` after provisioning.\n"]
    pub fn key_usage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_usage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_key_pem` after provisioning.\n"]
    pub fn public_key_pem(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key_pem", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signing_algorithms` after provisioning.\n"]
    pub fn signing_algorithms(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.signing_algorithms", self.extract_ref()))
    }
}
