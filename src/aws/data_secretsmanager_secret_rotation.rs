use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSecretsmanagerSecretRotationData {
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
    secret_id: terrars::PrimField<std::string::String>,
}

struct DataSecretsmanagerSecretRotation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSecretsmanagerSecretRotationData>,
}

#[derive(Clone)]
pub struct DataSecretsmanagerSecretRotation(std::rc::Rc<DataSecretsmanagerSecretRotation_>);

impl DataSecretsmanagerSecretRotation {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rotation_enabled` after provisioning.\n"]
    pub fn rotation_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rotation_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rotation_lambda_arn` after provisioning.\n"]
    pub fn rotation_lambda_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rotation_lambda_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rotation_rules` after provisioning.\n"]
    pub fn rotation_rules(&self) -> terrars::ListRef<DataSecretsmanagerSecretRotationRotationRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rotation_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataSecretsmanagerSecretRotation {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSecretsmanagerSecretRotation { }

impl terrars::ToListMappable for DataSecretsmanagerSecretRotation {
    type O = terrars::ListRef<DataSecretsmanagerSecretRotationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSecretsmanagerSecretRotation_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_secretsmanager_secret_rotation".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSecretsmanagerSecretRotation {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub secret_id: terrars::PrimField<std::string::String>,
}

impl BuildDataSecretsmanagerSecretRotation {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSecretsmanagerSecretRotation {
        let out = DataSecretsmanagerSecretRotation(std::rc::Rc::new(DataSecretsmanagerSecretRotation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSecretsmanagerSecretRotationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                secret_id: self.secret_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSecretsmanagerSecretRotationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretsmanagerSecretRotationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSecretsmanagerSecretRotationRef {
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

    #[doc = "Get a reference to the value of field `rotation_enabled` after provisioning.\n"]
    pub fn rotation_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rotation_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rotation_lambda_arn` after provisioning.\n"]
    pub fn rotation_lambda_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rotation_lambda_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rotation_rules` after provisioning.\n"]
    pub fn rotation_rules(&self) -> terrars::ListRef<DataSecretsmanagerSecretRotationRotationRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rotation_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSecretsmanagerSecretRotationRotationRulesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatically_after_days: std::option::Option<terrars::PrimField<f64>>,
}

impl DataSecretsmanagerSecretRotationRotationRulesEl {
    #[doc = "Set the field `automatically_after_days`.\n"]
    pub fn set_automatically_after_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.automatically_after_days = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecretsmanagerSecretRotationRotationRulesEl {
    type O = terrars::BlockAssignable<DataSecretsmanagerSecretRotationRotationRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretsmanagerSecretRotationRotationRulesEl {}

impl BuildDataSecretsmanagerSecretRotationRotationRulesEl {
    pub fn build(self) -> DataSecretsmanagerSecretRotationRotationRulesEl {
        DataSecretsmanagerSecretRotationRotationRulesEl {
            automatically_after_days: core::default::Default::default(),
        }
    }
}

pub struct DataSecretsmanagerSecretRotationRotationRulesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretsmanagerSecretRotationRotationRulesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSecretsmanagerSecretRotationRotationRulesElRef {
        DataSecretsmanagerSecretRotationRotationRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretsmanagerSecretRotationRotationRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `automatically_after_days` after provisioning.\n"]
    pub fn automatically_after_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatically_after_days", self.base))
    }
}
