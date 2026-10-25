#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SecretsmanagerSecretRotationData_ {
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
    external_secret_rotation_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rotate_immediately: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rotation_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rotation_lambda_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    secret_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_secret_rotation_metadata: std
    ::option
    ::Option<std::vec::Vec<SecretsmanagerSecretRotation_ExternalSecretRotationMetadata>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rotation_rules: std::option::Option<std::vec::Vec<SecretsmanagerSecretRotation_RotationRules>>,
    dynamic: SecretsmanagerSecretRotationDynamic,
}

struct SecretsmanagerSecretRotation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SecretsmanagerSecretRotationData_>,
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

    #[doc = "Set the field `external_secret_rotation_role_arn`.\n"]
    pub fn set_external_secret_rotation_role_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().external_secret_rotation_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rotate_immediately`.\n"]
    pub fn set_rotate_immediately(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().rotate_immediately = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rotation_enabled`.\n"]
    pub fn set_rotation_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().rotation_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rotation_lambda_arn`.\n"]
    pub fn set_rotation_lambda_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().rotation_lambda_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_secret_rotation_metadata`.\n"]
    pub fn set_external_secret_rotation_metadata(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecretsmanagerSecretRotation_ExternalSecretRotationMetadata>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().external_secret_rotation_metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.external_secret_rotation_metadata = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rotation_rules`.\n"]
    pub fn set_rotation_rules(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecretsmanagerSecretRotation_RotationRules>>,
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

    #[doc = "Get a reference to the value of field `external_secret_rotation_role_arn` after provisioning.\n"]
    pub fn external_secret_rotation_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_secret_rotation_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rotate_immediately` after provisioning.\n"]
    pub fn rotate_immediately(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rotate_immediately", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `external_secret_rotation_metadata` after provisioning.\n"]
    pub fn external_secret_rotation_metadata(
        &self,
    ) -> terrars::ListRef<SecretsmanagerSecretRotation_ExternalSecretRotationMetadataRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.external_secret_rotation_metadata", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rotation_rules` after provisioning.\n"]
    pub fn rotation_rules(&self) -> terrars::ListRef<SecretsmanagerSecretRotation_RotationRulesRef> {
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
    pub secret_id: terrars::PrimField<std::string::String>,
}

impl BuildSecretsmanagerSecretRotation {
    pub fn build(self, stack: &mut terrars::Stack) -> SecretsmanagerSecretRotation {
        let out = SecretsmanagerSecretRotation(std::rc::Rc::new(SecretsmanagerSecretRotation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SecretsmanagerSecretRotationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                external_secret_rotation_role_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                rotate_immediately: core::default::Default::default(),
                rotation_enabled: core::default::Default::default(),
                rotation_lambda_arn: core::default::Default::default(),
                secret_id: self.secret_id,
                external_secret_rotation_metadata: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `external_secret_rotation_role_arn` after provisioning.\n"]
    pub fn external_secret_rotation_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_secret_rotation_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rotate_immediately` after provisioning.\n"]
    pub fn rotate_immediately(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rotate_immediately", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `external_secret_rotation_metadata` after provisioning.\n"]
    pub fn external_secret_rotation_metadata(
        &self,
    ) -> terrars::ListRef<SecretsmanagerSecretRotation_ExternalSecretRotationMetadataRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.external_secret_rotation_metadata", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rotation_rules` after provisioning.\n"]
    pub fn rotation_rules(&self) -> terrars::ListRef<SecretsmanagerSecretRotation_RotationRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rotation_rules", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SecretsmanagerSecretRotationDynamic {
    external_secret_rotation_metadata: std
    ::option
    ::Option<terrars::DynamicBlock<SecretsmanagerSecretRotation_ExternalSecretRotationMetadata>>,
    rotation_rules: std::option::Option<terrars::DynamicBlock<SecretsmanagerSecretRotation_RotationRules>>,
}

#[derive(Serialize)]
pub struct SecretsmanagerSecretRotation_ExternalSecretRotationMetadata {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecretsmanagerSecretRotation_ExternalSecretRotationMetadata { }

impl terrars::ToListMappable for SecretsmanagerSecretRotation_ExternalSecretRotationMetadata {
    type O = terrars::BlockAssignable<SecretsmanagerSecretRotation_ExternalSecretRotationMetadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecretsmanagerSecretRotation_ExternalSecretRotationMetadata {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecretsmanagerSecretRotation_ExternalSecretRotationMetadata {
    pub fn build(self) -> SecretsmanagerSecretRotation_ExternalSecretRotationMetadata {
        SecretsmanagerSecretRotation_ExternalSecretRotationMetadata {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecretsmanagerSecretRotation_ExternalSecretRotationMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecretsmanagerSecretRotation_ExternalSecretRotationMetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecretsmanagerSecretRotation_ExternalSecretRotationMetadataRef {
        SecretsmanagerSecretRotation_ExternalSecretRotationMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecretsmanagerSecretRotation_ExternalSecretRotationMetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecretsmanagerSecretRotation_RotationRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatically_after_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_expression: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecretsmanagerSecretRotation_RotationRules {
    #[doc = "Set the field `automatically_after_days`.\n"]
    pub fn set_automatically_after_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.automatically_after_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_expression`.\n"]
    pub fn set_schedule_expression(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.schedule_expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecretsmanagerSecretRotation_RotationRules {
    type O = terrars::BlockAssignable<SecretsmanagerSecretRotation_RotationRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecretsmanagerSecretRotation_RotationRules {}

impl BuildSecretsmanagerSecretRotation_RotationRules {
    pub fn build(self) -> SecretsmanagerSecretRotation_RotationRules {
        SecretsmanagerSecretRotation_RotationRules {
            automatically_after_days: core::default::Default::default(),
            duration: core::default::Default::default(),
            schedule_expression: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecretsmanagerSecretRotation_RotationRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecretsmanagerSecretRotation_RotationRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecretsmanagerSecretRotation_RotationRulesRef {
        SecretsmanagerSecretRotation_RotationRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecretsmanagerSecretRotation_RotationRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `automatically_after_days` after provisioning.\n"]
    pub fn automatically_after_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatically_after_days", self.base))
    }

    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression", self.base))
    }
}
