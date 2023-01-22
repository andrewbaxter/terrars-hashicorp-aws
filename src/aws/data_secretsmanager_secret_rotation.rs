use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSecretsmanagerSecretRotationData {
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    secret_id: PrimField<String>,
}

struct DataSecretsmanagerSecretRotation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSecretsmanagerSecretRotationData>,
}

#[derive(Clone)]
pub struct DataSecretsmanagerSecretRotation(Rc<DataSecretsmanagerSecretRotation_>);

impl DataSecretsmanagerSecretRotation {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rotation_enabled` after provisioning.\n"]
    pub fn rotation_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.rotation_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rotation_lambda_arn` after provisioning.\n"]
    pub fn rotation_lambda_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rotation_lambda_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rotation_rules` after provisioning.\n"]
    pub fn rotation_rules(&self) -> ListRef<DataSecretsmanagerSecretRotationRotationRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rotation_rules", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.extract_ref()))
    }
}

impl Datasource for DataSecretsmanagerSecretRotation {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for DataSecretsmanagerSecretRotation {
    type O = ListRef<DataSecretsmanagerSecretRotationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSecretsmanagerSecretRotation_ {
    fn extract_datasource_type(&self) -> String {
        "aws_secretsmanager_secret_rotation".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSecretsmanagerSecretRotation {
    pub tf_id: String,
    #[doc= ""]
    pub secret_id: PrimField<String>,
}

impl BuildDataSecretsmanagerSecretRotation {
    pub fn build(self, stack: &mut Stack) -> DataSecretsmanagerSecretRotation {
        let out = DataSecretsmanagerSecretRotation(Rc::new(DataSecretsmanagerSecretRotation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSecretsmanagerSecretRotationData {
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                secret_id: self.secret_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSecretsmanagerSecretRotationRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSecretsmanagerSecretRotationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSecretsmanagerSecretRotationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rotation_enabled` after provisioning.\n"]
    pub fn rotation_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.rotation_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rotation_lambda_arn` after provisioning.\n"]
    pub fn rotation_lambda_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rotation_lambda_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rotation_rules` after provisioning.\n"]
    pub fn rotation_rules(&self) -> ListRef<DataSecretsmanagerSecretRotationRotationRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rotation_rules", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSecretsmanagerSecretRotationRotationRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    automatically_after_days: Option<PrimField<f64>>,
}

impl DataSecretsmanagerSecretRotationRotationRulesEl {
    #[doc= "Set the field `automatically_after_days`.\n"]
    pub fn set_automatically_after_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.automatically_after_days = Some(v.into());
        self
    }
}

impl ToListMappable for DataSecretsmanagerSecretRotationRotationRulesEl {
    type O = BlockAssignable<DataSecretsmanagerSecretRotationRotationRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataSecretsmanagerSecretRotationRotationRulesElRef {
    fn new(shared: StackShared, base: String) -> DataSecretsmanagerSecretRotationRotationRulesElRef {
        DataSecretsmanagerSecretRotationRotationRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretsmanagerSecretRotationRotationRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `automatically_after_days` after provisioning.\n"]
    pub fn automatically_after_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.automatically_after_days", self.base))
    }
}