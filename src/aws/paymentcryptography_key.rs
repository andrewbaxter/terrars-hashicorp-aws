#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct PaymentcryptographyKeyData_ {
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
    deletion_window_in_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    exportable: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_check_value_algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_attributes: std::option::Option<std::vec::Vec<PaymentcryptographyKey_KeyAttributes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<PaymentcryptographyKey_Timeouts>,
    dynamic: PaymentcryptographyKeyDynamic,
}

struct PaymentcryptographyKey_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PaymentcryptographyKeyData_>,
}

#[derive(Clone)]
pub struct PaymentcryptographyKey(std::rc::Rc<PaymentcryptographyKey_>);

impl PaymentcryptographyKey {
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

    #[doc = "Set the field `deletion_window_in_days`.\n"]
    pub fn set_deletion_window_in_days(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().deletion_window_in_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_check_value_algorithm`.\n"]
    pub fn set_key_check_value_algorithm(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().key_check_value_algorithm = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_attributes`.\n"]
    pub fn set_key_attributes(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PaymentcryptographyKey_KeyAttributes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().key_attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.key_attributes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<PaymentcryptographyKey_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_window_in_days` after provisioning.\n"]
    pub fn deletion_window_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_window_in_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exportable` after provisioning.\n"]
    pub fn exportable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exportable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_check_value` after provisioning.\n"]
    pub fn key_check_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_check_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_check_value_algorithm` after provisioning.\n"]
    pub fn key_check_value_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_check_value_algorithm", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_origin` after provisioning.\n"]
    pub fn key_origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_origin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_state` after provisioning.\n"]
    pub fn key_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_attributes` after provisioning.\n"]
    pub fn key_attributes(&self) -> terrars::ListRef<PaymentcryptographyKey_KeyAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PaymentcryptographyKey_TimeoutsRef {
        PaymentcryptographyKey_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for PaymentcryptographyKey {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PaymentcryptographyKey { }

impl terrars::ToListMappable for PaymentcryptographyKey {
    type O = terrars::ListRef<PaymentcryptographyKeyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PaymentcryptographyKey_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_paymentcryptography_key".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPaymentcryptographyKey {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub exportable: terrars::PrimField<bool>,
}

impl BuildPaymentcryptographyKey {
    pub fn build(self, stack: &mut terrars::Stack) -> PaymentcryptographyKey {
        let out = PaymentcryptographyKey(std::rc::Rc::new(PaymentcryptographyKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PaymentcryptographyKeyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_window_in_days: core::default::Default::default(),
                enabled: core::default::Default::default(),
                exportable: self.exportable,
                key_check_value_algorithm: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                key_attributes: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PaymentcryptographyKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PaymentcryptographyKeyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PaymentcryptographyKeyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_window_in_days` after provisioning.\n"]
    pub fn deletion_window_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_window_in_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exportable` after provisioning.\n"]
    pub fn exportable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exportable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_check_value` after provisioning.\n"]
    pub fn key_check_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_check_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_check_value_algorithm` after provisioning.\n"]
    pub fn key_check_value_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_check_value_algorithm", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_origin` after provisioning.\n"]
    pub fn key_origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_origin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_state` after provisioning.\n"]
    pub fn key_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_attributes` after provisioning.\n"]
    pub fn key_attributes(&self) -> terrars::ListRef<PaymentcryptographyKey_KeyAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PaymentcryptographyKey_TimeoutsRef {
        PaymentcryptographyKey_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct PaymentcryptographyKeyDynamic {
    key_attributes: std::option::Option<terrars::DynamicBlock<PaymentcryptographyKey_KeyAttributes>>,
}

#[derive(Serialize)]
pub struct PaymentcryptographyKey_KeyAttributes_KeyModesOfUse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    decrypt: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    derive_key: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypt: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generate: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_restrictions: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sign: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unwrap: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verify: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wrap: std::option::Option<terrars::PrimField<bool>>,
}

impl PaymentcryptographyKey_KeyAttributes_KeyModesOfUse {
    #[doc = "Set the field `decrypt`.\n"]
    pub fn set_decrypt(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.decrypt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `derive_key`.\n"]
    pub fn set_derive_key(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.derive_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encrypt`.\n"]
    pub fn set_encrypt(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.encrypt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `generate`.\n"]
    pub fn set_generate(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.generate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_restrictions`.\n"]
    pub fn set_no_restrictions(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_restrictions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sign`.\n"]
    pub fn set_sign(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.sign = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unwrap`.\n"]
    pub fn set_unwrap(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.unwrap = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `verify`.\n"]
    pub fn set_verify(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.verify = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wrap`.\n"]
    pub fn set_wrap(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.wrap = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PaymentcryptographyKey_KeyAttributes_KeyModesOfUse {
    type O = terrars::BlockAssignable<PaymentcryptographyKey_KeyAttributes_KeyModesOfUse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPaymentcryptographyKey_KeyAttributes_KeyModesOfUse {}

impl BuildPaymentcryptographyKey_KeyAttributes_KeyModesOfUse {
    pub fn build(self) -> PaymentcryptographyKey_KeyAttributes_KeyModesOfUse {
        PaymentcryptographyKey_KeyAttributes_KeyModesOfUse {
            decrypt: core::default::Default::default(),
            derive_key: core::default::Default::default(),
            encrypt: core::default::Default::default(),
            generate: core::default::Default::default(),
            no_restrictions: core::default::Default::default(),
            sign: core::default::Default::default(),
            unwrap: core::default::Default::default(),
            verify: core::default::Default::default(),
            wrap: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PaymentcryptographyKey_KeyAttributes_KeyModesOfUseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PaymentcryptographyKey_KeyAttributes_KeyModesOfUseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PaymentcryptographyKey_KeyAttributes_KeyModesOfUseRef {
        PaymentcryptographyKey_KeyAttributes_KeyModesOfUseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PaymentcryptographyKey_KeyAttributes_KeyModesOfUseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `decrypt` after provisioning.\n"]
    pub fn decrypt(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.decrypt", self.base))
    }

    #[doc = "Get a reference to the value of field `derive_key` after provisioning.\n"]
    pub fn derive_key(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.derive_key", self.base))
    }

    #[doc = "Get a reference to the value of field `encrypt` after provisioning.\n"]
    pub fn encrypt(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypt", self.base))
    }

    #[doc = "Get a reference to the value of field `generate` after provisioning.\n"]
    pub fn generate(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generate", self.base))
    }

    #[doc = "Get a reference to the value of field `no_restrictions` after provisioning.\n"]
    pub fn no_restrictions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_restrictions", self.base))
    }

    #[doc = "Get a reference to the value of field `sign` after provisioning.\n"]
    pub fn sign(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sign", self.base))
    }

    #[doc = "Get a reference to the value of field `unwrap` after provisioning.\n"]
    pub fn unwrap(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unwrap", self.base))
    }

    #[doc = "Get a reference to the value of field `verify` after provisioning.\n"]
    pub fn verify(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verify", self.base))
    }

    #[doc = "Get a reference to the value of field `wrap` after provisioning.\n"]
    pub fn wrap(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wrap", self.base))
    }
}

#[derive(Serialize, Default)]
struct PaymentcryptographyKey_KeyAttributesDynamic {
    key_modes_of_use: std::option::Option<terrars::DynamicBlock<PaymentcryptographyKey_KeyAttributes_KeyModesOfUse>>,
}

#[derive(Serialize)]
pub struct PaymentcryptographyKey_KeyAttributes {
    key_algorithm: terrars::PrimField<std::string::String>,
    key_class: terrars::PrimField<std::string::String>,
    key_usage: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_modes_of_use: std::option::Option<std::vec::Vec<PaymentcryptographyKey_KeyAttributes_KeyModesOfUse>>,
    dynamic: PaymentcryptographyKey_KeyAttributesDynamic,
}

impl PaymentcryptographyKey_KeyAttributes {
    #[doc = "Set the field `key_modes_of_use`.\n"]
    pub fn set_key_modes_of_use(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PaymentcryptographyKey_KeyAttributes_KeyModesOfUse>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.key_modes_of_use = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.key_modes_of_use = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PaymentcryptographyKey_KeyAttributes {
    type O = terrars::BlockAssignable<PaymentcryptographyKey_KeyAttributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPaymentcryptographyKey_KeyAttributes {
    #[doc = ""]
    pub key_algorithm: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key_class: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key_usage: terrars::PrimField<std::string::String>,
}

impl BuildPaymentcryptographyKey_KeyAttributes {
    pub fn build(self) -> PaymentcryptographyKey_KeyAttributes {
        PaymentcryptographyKey_KeyAttributes {
            key_algorithm: self.key_algorithm,
            key_class: self.key_class,
            key_usage: self.key_usage,
            key_modes_of_use: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PaymentcryptographyKey_KeyAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PaymentcryptographyKey_KeyAttributesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PaymentcryptographyKey_KeyAttributesRef {
        PaymentcryptographyKey_KeyAttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PaymentcryptographyKey_KeyAttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_algorithm` after provisioning.\n"]
    pub fn key_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `key_class` after provisioning.\n"]
    pub fn key_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_class", self.base))
    }

    #[doc = "Get a reference to the value of field `key_usage` after provisioning.\n"]
    pub fn key_usage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `key_modes_of_use` after provisioning.\n"]
    pub fn key_modes_of_use(&self) -> terrars::ListRef<PaymentcryptographyKey_KeyAttributes_KeyModesOfUseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_modes_of_use", self.base))
    }
}

#[derive(Serialize)]
pub struct PaymentcryptographyKey_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PaymentcryptographyKey_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PaymentcryptographyKey_Timeouts {
    type O = terrars::BlockAssignable<PaymentcryptographyKey_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPaymentcryptographyKey_Timeouts {}

impl BuildPaymentcryptographyKey_Timeouts {
    pub fn build(self) -> PaymentcryptographyKey_Timeouts {
        PaymentcryptographyKey_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PaymentcryptographyKey_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PaymentcryptographyKey_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PaymentcryptographyKey_TimeoutsRef {
        PaymentcryptographyKey_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PaymentcryptographyKey_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
