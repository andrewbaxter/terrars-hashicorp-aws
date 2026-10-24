use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct IamAccountPasswordPolicyData {
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
    allow_users_to_change_password: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hard_expiry: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_password_age: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_password_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_reuse_prevention: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_lowercase_characters: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_numbers: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_symbols: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_uppercase_characters: std::option::Option<terrars::PrimField<bool>>,
}

struct IamAccountPasswordPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IamAccountPasswordPolicyData>,
}

#[derive(Clone)]
pub struct IamAccountPasswordPolicy(std::rc::Rc<IamAccountPasswordPolicy_>);

impl IamAccountPasswordPolicy {
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

    #[doc = "Set the field `allow_users_to_change_password`.\n"]
    pub fn set_allow_users_to_change_password(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_users_to_change_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hard_expiry`.\n"]
    pub fn set_hard_expiry(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().hard_expiry = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_password_age`.\n"]
    pub fn set_max_password_age(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_password_age = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minimum_password_length`.\n"]
    pub fn set_minimum_password_length(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().minimum_password_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password_reuse_prevention`.\n"]
    pub fn set_password_reuse_prevention(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().password_reuse_prevention = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_lowercase_characters`.\n"]
    pub fn set_require_lowercase_characters(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_lowercase_characters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_numbers`.\n"]
    pub fn set_require_numbers(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_numbers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_symbols`.\n"]
    pub fn set_require_symbols(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_symbols = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_uppercase_characters`.\n"]
    pub fn set_require_uppercase_characters(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_uppercase_characters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `allow_users_to_change_password` after provisioning.\n"]
    pub fn allow_users_to_change_password(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_users_to_change_password", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `expire_passwords` after provisioning.\n"]
    pub fn expire_passwords(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_passwords", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hard_expiry` after provisioning.\n"]
    pub fn hard_expiry(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hard_expiry", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_password_age` after provisioning.\n"]
    pub fn max_password_age(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_password_age", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `minimum_password_length` after provisioning.\n"]
    pub fn minimum_password_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_password_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_reuse_prevention` after provisioning.\n"]
    pub fn password_reuse_prevention(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_reuse_prevention", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_lowercase_characters` after provisioning.\n"]
    pub fn require_lowercase_characters(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_lowercase_characters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_numbers` after provisioning.\n"]
    pub fn require_numbers(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_numbers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_symbols` after provisioning.\n"]
    pub fn require_symbols(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_symbols", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_uppercase_characters` after provisioning.\n"]
    pub fn require_uppercase_characters(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_uppercase_characters", self.extract_ref()))
    }
}

impl terrars::Referable for IamAccountPasswordPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IamAccountPasswordPolicy { }

impl terrars::ToListMappable for IamAccountPasswordPolicy {
    type O = terrars::ListRef<IamAccountPasswordPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IamAccountPasswordPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_iam_account_password_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIamAccountPasswordPolicy {
    pub tf_id: std::string::String,
}

impl BuildIamAccountPasswordPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> IamAccountPasswordPolicy {
        let out = IamAccountPasswordPolicy(std::rc::Rc::new(IamAccountPasswordPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IamAccountPasswordPolicyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allow_users_to_change_password: core::default::Default::default(),
                hard_expiry: core::default::Default::default(),
                id: core::default::Default::default(),
                max_password_age: core::default::Default::default(),
                minimum_password_length: core::default::Default::default(),
                password_reuse_prevention: core::default::Default::default(),
                require_lowercase_characters: core::default::Default::default(),
                require_numbers: core::default::Default::default(),
                require_symbols: core::default::Default::default(),
                require_uppercase_characters: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IamAccountPasswordPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamAccountPasswordPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IamAccountPasswordPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_users_to_change_password` after provisioning.\n"]
    pub fn allow_users_to_change_password(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_users_to_change_password", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `expire_passwords` after provisioning.\n"]
    pub fn expire_passwords(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_passwords", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hard_expiry` after provisioning.\n"]
    pub fn hard_expiry(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hard_expiry", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_password_age` after provisioning.\n"]
    pub fn max_password_age(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_password_age", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `minimum_password_length` after provisioning.\n"]
    pub fn minimum_password_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_password_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_reuse_prevention` after provisioning.\n"]
    pub fn password_reuse_prevention(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_reuse_prevention", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_lowercase_characters` after provisioning.\n"]
    pub fn require_lowercase_characters(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_lowercase_characters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_numbers` after provisioning.\n"]
    pub fn require_numbers(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_numbers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_symbols` after provisioning.\n"]
    pub fn require_symbols(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_symbols", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_uppercase_characters` after provisioning.\n"]
    pub fn require_uppercase_characters(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_uppercase_characters", self.extract_ref()))
    }
}
