#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ShieldProactiveEngagementData_ {
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
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    emergency_contact: std::option::Option<std::vec::Vec<ShieldProactiveEngagement_EmergencyContact>>,
    dynamic: ShieldProactiveEngagementDynamic,
}

struct ShieldProactiveEngagement_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ShieldProactiveEngagementData_>,
}

#[derive(Clone)]
pub struct ShieldProactiveEngagement(std::rc::Rc<ShieldProactiveEngagement_>);

impl ShieldProactiveEngagement {
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

    #[doc = "Set the field `emergency_contact`.\n"]
    pub fn set_emergency_contact(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ShieldProactiveEngagement_EmergencyContact>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().emergency_contact = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.emergency_contact = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `emergency_contact` after provisioning.\n"]
    pub fn emergency_contact(&self) -> terrars::ListRef<ShieldProactiveEngagement_EmergencyContactRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emergency_contact", self.extract_ref()))
    }
}

impl terrars::Referable for ShieldProactiveEngagement {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ShieldProactiveEngagement { }

impl terrars::ToListMappable for ShieldProactiveEngagement {
    type O = terrars::ListRef<ShieldProactiveEngagementRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ShieldProactiveEngagement_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_shield_proactive_engagement".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildShieldProactiveEngagement {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildShieldProactiveEngagement {
    pub fn build(self, stack: &mut terrars::Stack) -> ShieldProactiveEngagement {
        let out = ShieldProactiveEngagement(std::rc::Rc::new(ShieldProactiveEngagement_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ShieldProactiveEngagementData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                enabled: self.enabled,
                emergency_contact: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ShieldProactiveEngagementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ShieldProactiveEngagementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ShieldProactiveEngagementRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `emergency_contact` after provisioning.\n"]
    pub fn emergency_contact(&self) -> terrars::ListRef<ShieldProactiveEngagement_EmergencyContactRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emergency_contact", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ShieldProactiveEngagementDynamic {
    emergency_contact: std::option::Option<terrars::DynamicBlock<ShieldProactiveEngagement_EmergencyContact>>,
}

#[derive(Serialize)]
pub struct ShieldProactiveEngagement_EmergencyContact {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contact_notes: std::option::Option<terrars::PrimField<std::string::String>>,
    email_address: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_number: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ShieldProactiveEngagement_EmergencyContact {
    #[doc = "Set the field `contact_notes`.\n"]
    pub fn set_contact_notes(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contact_notes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `phone_number`.\n"]
    pub fn set_phone_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.phone_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ShieldProactiveEngagement_EmergencyContact {
    type O = terrars::BlockAssignable<ShieldProactiveEngagement_EmergencyContact>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildShieldProactiveEngagement_EmergencyContact {
    #[doc = ""]
    pub email_address: terrars::PrimField<std::string::String>,
}

impl BuildShieldProactiveEngagement_EmergencyContact {
    pub fn build(self) -> ShieldProactiveEngagement_EmergencyContact {
        ShieldProactiveEngagement_EmergencyContact {
            contact_notes: core::default::Default::default(),
            email_address: self.email_address,
            phone_number: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ShieldProactiveEngagement_EmergencyContactRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ShieldProactiveEngagement_EmergencyContactRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ShieldProactiveEngagement_EmergencyContactRef {
        ShieldProactiveEngagement_EmergencyContactRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ShieldProactiveEngagement_EmergencyContactRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `contact_notes` after provisioning.\n"]
    pub fn contact_notes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_notes", self.base))
    }

    #[doc = "Get a reference to the value of field `email_address` after provisioning.\n"]
    pub fn email_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_address", self.base))
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}
