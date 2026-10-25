#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataIamOutboundWebIdentityFederationData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
}

struct DataIamOutboundWebIdentityFederation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataIamOutboundWebIdentityFederationData_>,
}

#[derive(Clone)]
pub struct DataIamOutboundWebIdentityFederation(std::rc::Rc<DataIamOutboundWebIdentityFederation_>);

impl DataIamOutboundWebIdentityFederation {
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

    #[doc = "Get a reference to the value of field `issuer_identifier` after provisioning.\n"]
    pub fn issuer_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer_identifier", self.extract_ref()))
    }
}

impl terrars::Referable for DataIamOutboundWebIdentityFederation {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataIamOutboundWebIdentityFederation { }

impl terrars::ToListMappable for DataIamOutboundWebIdentityFederation {
    type O = terrars::ListRef<DataIamOutboundWebIdentityFederationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataIamOutboundWebIdentityFederation_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_iam_outbound_web_identity_federation".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIamOutboundWebIdentityFederation {
    pub tf_id: std::string::String,
}

impl BuildDataIamOutboundWebIdentityFederation {
    pub fn build(self, stack: &mut terrars::Stack) -> DataIamOutboundWebIdentityFederation {
        let out = DataIamOutboundWebIdentityFederation(std::rc::Rc::new(DataIamOutboundWebIdentityFederation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataIamOutboundWebIdentityFederationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIamOutboundWebIdentityFederationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamOutboundWebIdentityFederationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataIamOutboundWebIdentityFederationRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `issuer_identifier` after provisioning.\n"]
    pub fn issuer_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer_identifier", self.extract_ref()))
    }
}
