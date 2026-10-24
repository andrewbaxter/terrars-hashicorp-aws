use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SecretsmanagerSecretRotationData {
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
    rotation_lambda_arn: terrars::PrimField<std::string::String>,
    secret_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rotation_rules: std::option::Option<std::vec::Vec<SecretsmanagerSecretRotationRotationRulesEl>>,
    dynamic: SecretsmanagerSecretRotationDynamic,
}

struct SecretsmanagerSecretRotation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SecretsmanagerSecretRotationData>,
}

#[derive(Clone)]
pub struct SecretsmanagerSecretRotation(std::rc::Rc<SecretsmanagerSecretRotation_>);

impl SecretsmanagerSecretRotation {
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

    #[doc = "Set the field `rotation_rules`.\n"]
    pub fn set_rotation_rules(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecretsmanagerSecretRotationRotationRulesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rotation_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rotation_rules = std::option::Option::Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rotation_rules` after provisioning.\n"]
    pub fn rotation_rules(&self) -> terrars::ListRef<SecretsmanagerSecretRotationRotationRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rotation_rules", self.extract_ref()))
    }
}

impl terrars::Referable for SecretsmanagerSecretRotation {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SecretsmanagerSecretRotation { }

impl terrars::ToListMappable for SecretsmanagerSecretRotation {
    type O = terrars::ListRef<SecretsmanagerSecretRotationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SecretsmanagerSecretRotation_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_secretsmanager_secret_rotation".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecretsmanagerSecretRotation {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub rotation_lambda_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub secret_id: terrars::PrimField<std::string::String>,
}

impl BuildSecretsmanagerSecretRotation {
    pub fn build(self, stack: &mut terrars::Stack) -> SecretsmanagerSecretRotation {
        let out = SecretsmanagerSecretRotation(std::rc::Rc::new(SecretsmanagerSecretRotation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SecretsmanagerSecretRotationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                rotation_lambda_arn: self.rotation_lambda_arn,
                secret_id: self.secret_id,
                rotation_rules: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecretsmanagerSecretRotationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecretsmanagerSecretRotationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SecretsmanagerSecretRotationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rotation_rules` after provisioning.\n"]
    pub fn rotation_rules(&self) -> terrars::ListRef<SecretsmanagerSecretRotationRotationRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rotation_rules", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SecretsmanagerSecretRotationRotationRulesEl {
    automatically_after_days: terrars::PrimField<f64>,
}

impl SecretsmanagerSecretRotationRotationRulesEl { }

impl terrars::ToListMappable for SecretsmanagerSecretRotationRotationRulesEl {
    type O = terrars::BlockAssignable<SecretsmanagerSecretRotationRotationRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecretsmanagerSecretRotationRotationRulesEl {
    #[doc = ""]
    pub automatically_after_days: terrars::PrimField<f64>,
}

impl BuildSecretsmanagerSecretRotationRotationRulesEl {
    pub fn build(self) -> SecretsmanagerSecretRotationRotationRulesEl {
        SecretsmanagerSecretRotationRotationRulesEl { automatically_after_days: self.automatically_after_days }
    }
}

pub struct SecretsmanagerSecretRotationRotationRulesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecretsmanagerSecretRotationRotationRulesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecretsmanagerSecretRotationRotationRulesElRef {
        SecretsmanagerSecretRotationRotationRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecretsmanagerSecretRotationRotationRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `automatically_after_days` after provisioning.\n"]
    pub fn automatically_after_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatically_after_days", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecretsmanagerSecretRotationDynamic {
    rotation_rules: std::option::Option<terrars::DynamicBlock<SecretsmanagerSecretRotationRotationRulesEl>>,
}
