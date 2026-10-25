#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CustomerprofilesProfileData_ {
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
    account_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_information: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attributes: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    birth_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business_email_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business_phone_number: std::option::Option<terrars::PrimField<std::string::String>>,
    domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gender_string: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    home_phone_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    middle_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mobile_phone_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    party_type_string: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    personal_email_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<std::vec::Vec<CustomerprofilesProfile_Address>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    billing_address: std::option::Option<std::vec::Vec<CustomerprofilesProfile_BillingAddress>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mailing_address: std::option::Option<std::vec::Vec<CustomerprofilesProfile_MailingAddress>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shipping_address: std::option::Option<std::vec::Vec<CustomerprofilesProfile_ShippingAddress>>,
    dynamic: CustomerprofilesProfileDynamic,
}

struct CustomerprofilesProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CustomerprofilesProfileData_>,
}

#[derive(Clone)]
pub struct CustomerprofilesProfile(std::rc::Rc<CustomerprofilesProfile_>);

impl CustomerprofilesProfile {
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

    #[doc = "Set the field `account_number`.\n"]
    pub fn set_account_number(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_information`.\n"]
    pub fn set_additional_information(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().additional_information = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `birth_date`.\n"]
    pub fn set_birth_date(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().birth_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `business_email_address`.\n"]
    pub fn set_business_email_address(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().business_email_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `business_name`.\n"]
    pub fn set_business_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().business_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `business_phone_number`.\n"]
    pub fn set_business_phone_number(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().business_phone_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_address`.\n"]
    pub fn set_email_address(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().email_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `first_name`.\n"]
    pub fn set_first_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().first_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gender_string`.\n"]
    pub fn set_gender_string(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().gender_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `home_phone_number`.\n"]
    pub fn set_home_phone_number(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().home_phone_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_name`.\n"]
    pub fn set_last_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().last_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `middle_name`.\n"]
    pub fn set_middle_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().middle_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mobile_phone_number`.\n"]
    pub fn set_mobile_phone_number(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().mobile_phone_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `party_type_string`.\n"]
    pub fn set_party_type_string(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().party_type_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `personal_email_address`.\n"]
    pub fn set_personal_email_address(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().personal_email_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `phone_number`.\n"]
    pub fn set_phone_number(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().phone_number = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address`.\n"]
    pub fn set_address(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CustomerprofilesProfile_Address>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().address = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.address = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `billing_address`.\n"]
    pub fn set_billing_address(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CustomerprofilesProfile_BillingAddress>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().billing_address = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.billing_address = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mailing_address`.\n"]
    pub fn set_mailing_address(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CustomerprofilesProfile_MailingAddress>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mailing_address = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mailing_address = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `shipping_address`.\n"]
    pub fn set_shipping_address(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CustomerprofilesProfile_ShippingAddress>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().shipping_address = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.shipping_address = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `account_number` after provisioning.\n"]
    pub fn account_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `additional_information` after provisioning.\n"]
    pub fn additional_information(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_information", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `birth_date` after provisioning.\n"]
    pub fn birth_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.birth_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `business_email_address` after provisioning.\n"]
    pub fn business_email_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_email_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `business_name` after provisioning.\n"]
    pub fn business_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `business_phone_number` after provisioning.\n"]
    pub fn business_phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_phone_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_address` after provisioning.\n"]
    pub fn email_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gender_string` after provisioning.\n"]
    pub fn gender_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gender_string", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_phone_number` after provisioning.\n"]
    pub fn home_phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.home_phone_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `middle_name` after provisioning.\n"]
    pub fn middle_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.middle_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mobile_phone_number` after provisioning.\n"]
    pub fn mobile_phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mobile_phone_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `party_type_string` after provisioning.\n"]
    pub fn party_type_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.party_type_string", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `personal_email_address` after provisioning.\n"]
    pub fn personal_email_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.personal_email_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> terrars::ListRef<CustomerprofilesProfile_AddressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_address` after provisioning.\n"]
    pub fn billing_address(&self) -> terrars::ListRef<CustomerprofilesProfile_BillingAddressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.billing_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mailing_address` after provisioning.\n"]
    pub fn mailing_address(&self) -> terrars::ListRef<CustomerprofilesProfile_MailingAddressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mailing_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shipping_address` after provisioning.\n"]
    pub fn shipping_address(&self) -> terrars::ListRef<CustomerprofilesProfile_ShippingAddressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shipping_address", self.extract_ref()))
    }
}

impl terrars::Referable for CustomerprofilesProfile {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CustomerprofilesProfile { }

impl terrars::ToListMappable for CustomerprofilesProfile {
    type O = terrars::ListRef<CustomerprofilesProfileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CustomerprofilesProfile_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_customerprofiles_profile".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCustomerprofilesProfile {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
}

impl BuildCustomerprofilesProfile {
    pub fn build(self, stack: &mut terrars::Stack) -> CustomerprofilesProfile {
        let out = CustomerprofilesProfile(std::rc::Rc::new(CustomerprofilesProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CustomerprofilesProfileData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_number: core::default::Default::default(),
                additional_information: core::default::Default::default(),
                attributes: core::default::Default::default(),
                birth_date: core::default::Default::default(),
                business_email_address: core::default::Default::default(),
                business_name: core::default::Default::default(),
                business_phone_number: core::default::Default::default(),
                domain_name: self.domain_name,
                email_address: core::default::Default::default(),
                first_name: core::default::Default::default(),
                gender_string: core::default::Default::default(),
                home_phone_number: core::default::Default::default(),
                id: core::default::Default::default(),
                last_name: core::default::Default::default(),
                middle_name: core::default::Default::default(),
                mobile_phone_number: core::default::Default::default(),
                party_type_string: core::default::Default::default(),
                personal_email_address: core::default::Default::default(),
                phone_number: core::default::Default::default(),
                region: core::default::Default::default(),
                address: core::default::Default::default(),
                billing_address: core::default::Default::default(),
                mailing_address: core::default::Default::default(),
                shipping_address: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CustomerprofilesProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerprofilesProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CustomerprofilesProfileRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_number` after provisioning.\n"]
    pub fn account_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `additional_information` after provisioning.\n"]
    pub fn additional_information(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_information", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `birth_date` after provisioning.\n"]
    pub fn birth_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.birth_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `business_email_address` after provisioning.\n"]
    pub fn business_email_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_email_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `business_name` after provisioning.\n"]
    pub fn business_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `business_phone_number` after provisioning.\n"]
    pub fn business_phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_phone_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_address` after provisioning.\n"]
    pub fn email_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gender_string` after provisioning.\n"]
    pub fn gender_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gender_string", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_phone_number` after provisioning.\n"]
    pub fn home_phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.home_phone_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `middle_name` after provisioning.\n"]
    pub fn middle_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.middle_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mobile_phone_number` after provisioning.\n"]
    pub fn mobile_phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mobile_phone_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `party_type_string` after provisioning.\n"]
    pub fn party_type_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.party_type_string", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `personal_email_address` after provisioning.\n"]
    pub fn personal_email_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.personal_email_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> terrars::ListRef<CustomerprofilesProfile_AddressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_address` after provisioning.\n"]
    pub fn billing_address(&self) -> terrars::ListRef<CustomerprofilesProfile_BillingAddressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.billing_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mailing_address` after provisioning.\n"]
    pub fn mailing_address(&self) -> terrars::ListRef<CustomerprofilesProfile_MailingAddressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mailing_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shipping_address` after provisioning.\n"]
    pub fn shipping_address(&self) -> terrars::ListRef<CustomerprofilesProfile_ShippingAddressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shipping_address", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CustomerprofilesProfileDynamic {
    address: std::option::Option<terrars::DynamicBlock<CustomerprofilesProfile_Address>>,
    billing_address: std::option::Option<terrars::DynamicBlock<CustomerprofilesProfile_BillingAddress>>,
    mailing_address: std::option::Option<terrars::DynamicBlock<CustomerprofilesProfile_MailingAddress>>,
    shipping_address: std::option::Option<terrars::DynamicBlock<CustomerprofilesProfile_ShippingAddress>>,
}

#[derive(Serialize)]
pub struct CustomerprofilesProfile_Address {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_3: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_4: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    city: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    county: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postal_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    province: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CustomerprofilesProfile_Address {
    #[doc = "Set the field `address_1`.\n"]
    pub fn set_address_1(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_2`.\n"]
    pub fn set_address_2(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_3`.\n"]
    pub fn set_address_3(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_3 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_4`.\n"]
    pub fn set_address_4(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_4 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `city`.\n"]
    pub fn set_city(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.city = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `country`.\n"]
    pub fn set_country(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.country = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `county`.\n"]
    pub fn set_county(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.county = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `postal_code`.\n"]
    pub fn set_postal_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.postal_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `province`.\n"]
    pub fn set_province(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.province = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CustomerprofilesProfile_Address {
    type O = terrars::BlockAssignable<CustomerprofilesProfile_Address>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesProfile_Address {}

impl BuildCustomerprofilesProfile_Address {
    pub fn build(self) -> CustomerprofilesProfile_Address {
        CustomerprofilesProfile_Address {
            address_1: core::default::Default::default(),
            address_2: core::default::Default::default(),
            address_3: core::default::Default::default(),
            address_4: core::default::Default::default(),
            city: core::default::Default::default(),
            country: core::default::Default::default(),
            county: core::default::Default::default(),
            postal_code: core::default::Default::default(),
            province: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CustomerprofilesProfile_AddressRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerprofilesProfile_AddressRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomerprofilesProfile_AddressRef {
        CustomerprofilesProfile_AddressRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesProfile_AddressRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_1` after provisioning.\n"]
    pub fn address_1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_1", self.base))
    }

    #[doc = "Get a reference to the value of field `address_2` after provisioning.\n"]
    pub fn address_2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_2", self.base))
    }

    #[doc = "Get a reference to the value of field `address_3` after provisioning.\n"]
    pub fn address_3(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_3", self.base))
    }

    #[doc = "Get a reference to the value of field `address_4` after provisioning.\n"]
    pub fn address_4(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_4", self.base))
    }

    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.city", self.base))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\n"]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.base))
    }

    #[doc = "Get a reference to the value of field `county` after provisioning.\n"]
    pub fn county(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.county", self.base))
    }

    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\n"]
    pub fn postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.base))
    }

    #[doc = "Get a reference to the value of field `province` after provisioning.\n"]
    pub fn province(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.province", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomerprofilesProfile_BillingAddress {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_3: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_4: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    city: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    county: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postal_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    province: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CustomerprofilesProfile_BillingAddress {
    #[doc = "Set the field `address_1`.\n"]
    pub fn set_address_1(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_2`.\n"]
    pub fn set_address_2(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_3`.\n"]
    pub fn set_address_3(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_3 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_4`.\n"]
    pub fn set_address_4(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_4 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `city`.\n"]
    pub fn set_city(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.city = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `country`.\n"]
    pub fn set_country(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.country = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `county`.\n"]
    pub fn set_county(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.county = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `postal_code`.\n"]
    pub fn set_postal_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.postal_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `province`.\n"]
    pub fn set_province(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.province = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CustomerprofilesProfile_BillingAddress {
    type O = terrars::BlockAssignable<CustomerprofilesProfile_BillingAddress>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesProfile_BillingAddress {}

impl BuildCustomerprofilesProfile_BillingAddress {
    pub fn build(self) -> CustomerprofilesProfile_BillingAddress {
        CustomerprofilesProfile_BillingAddress {
            address_1: core::default::Default::default(),
            address_2: core::default::Default::default(),
            address_3: core::default::Default::default(),
            address_4: core::default::Default::default(),
            city: core::default::Default::default(),
            country: core::default::Default::default(),
            county: core::default::Default::default(),
            postal_code: core::default::Default::default(),
            province: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CustomerprofilesProfile_BillingAddressRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerprofilesProfile_BillingAddressRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomerprofilesProfile_BillingAddressRef {
        CustomerprofilesProfile_BillingAddressRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesProfile_BillingAddressRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_1` after provisioning.\n"]
    pub fn address_1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_1", self.base))
    }

    #[doc = "Get a reference to the value of field `address_2` after provisioning.\n"]
    pub fn address_2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_2", self.base))
    }

    #[doc = "Get a reference to the value of field `address_3` after provisioning.\n"]
    pub fn address_3(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_3", self.base))
    }

    #[doc = "Get a reference to the value of field `address_4` after provisioning.\n"]
    pub fn address_4(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_4", self.base))
    }

    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.city", self.base))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\n"]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.base))
    }

    #[doc = "Get a reference to the value of field `county` after provisioning.\n"]
    pub fn county(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.county", self.base))
    }

    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\n"]
    pub fn postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.base))
    }

    #[doc = "Get a reference to the value of field `province` after provisioning.\n"]
    pub fn province(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.province", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomerprofilesProfile_MailingAddress {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_3: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_4: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    city: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    county: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postal_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    province: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CustomerprofilesProfile_MailingAddress {
    #[doc = "Set the field `address_1`.\n"]
    pub fn set_address_1(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_2`.\n"]
    pub fn set_address_2(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_3`.\n"]
    pub fn set_address_3(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_3 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_4`.\n"]
    pub fn set_address_4(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_4 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `city`.\n"]
    pub fn set_city(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.city = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `country`.\n"]
    pub fn set_country(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.country = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `county`.\n"]
    pub fn set_county(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.county = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `postal_code`.\n"]
    pub fn set_postal_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.postal_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `province`.\n"]
    pub fn set_province(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.province = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CustomerprofilesProfile_MailingAddress {
    type O = terrars::BlockAssignable<CustomerprofilesProfile_MailingAddress>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesProfile_MailingAddress {}

impl BuildCustomerprofilesProfile_MailingAddress {
    pub fn build(self) -> CustomerprofilesProfile_MailingAddress {
        CustomerprofilesProfile_MailingAddress {
            address_1: core::default::Default::default(),
            address_2: core::default::Default::default(),
            address_3: core::default::Default::default(),
            address_4: core::default::Default::default(),
            city: core::default::Default::default(),
            country: core::default::Default::default(),
            county: core::default::Default::default(),
            postal_code: core::default::Default::default(),
            province: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CustomerprofilesProfile_MailingAddressRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerprofilesProfile_MailingAddressRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomerprofilesProfile_MailingAddressRef {
        CustomerprofilesProfile_MailingAddressRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesProfile_MailingAddressRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_1` after provisioning.\n"]
    pub fn address_1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_1", self.base))
    }

    #[doc = "Get a reference to the value of field `address_2` after provisioning.\n"]
    pub fn address_2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_2", self.base))
    }

    #[doc = "Get a reference to the value of field `address_3` after provisioning.\n"]
    pub fn address_3(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_3", self.base))
    }

    #[doc = "Get a reference to the value of field `address_4` after provisioning.\n"]
    pub fn address_4(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_4", self.base))
    }

    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.city", self.base))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\n"]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.base))
    }

    #[doc = "Get a reference to the value of field `county` after provisioning.\n"]
    pub fn county(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.county", self.base))
    }

    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\n"]
    pub fn postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.base))
    }

    #[doc = "Get a reference to the value of field `province` after provisioning.\n"]
    pub fn province(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.province", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomerprofilesProfile_ShippingAddress {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_3: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_4: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    city: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    county: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postal_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    province: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CustomerprofilesProfile_ShippingAddress {
    #[doc = "Set the field `address_1`.\n"]
    pub fn set_address_1(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_2`.\n"]
    pub fn set_address_2(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_3`.\n"]
    pub fn set_address_3(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_3 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_4`.\n"]
    pub fn set_address_4(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_4 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `city`.\n"]
    pub fn set_city(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.city = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `country`.\n"]
    pub fn set_country(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.country = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `county`.\n"]
    pub fn set_county(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.county = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `postal_code`.\n"]
    pub fn set_postal_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.postal_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `province`.\n"]
    pub fn set_province(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.province = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CustomerprofilesProfile_ShippingAddress {
    type O = terrars::BlockAssignable<CustomerprofilesProfile_ShippingAddress>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesProfile_ShippingAddress {}

impl BuildCustomerprofilesProfile_ShippingAddress {
    pub fn build(self) -> CustomerprofilesProfile_ShippingAddress {
        CustomerprofilesProfile_ShippingAddress {
            address_1: core::default::Default::default(),
            address_2: core::default::Default::default(),
            address_3: core::default::Default::default(),
            address_4: core::default::Default::default(),
            city: core::default::Default::default(),
            country: core::default::Default::default(),
            county: core::default::Default::default(),
            postal_code: core::default::Default::default(),
            province: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CustomerprofilesProfile_ShippingAddressRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerprofilesProfile_ShippingAddressRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomerprofilesProfile_ShippingAddressRef {
        CustomerprofilesProfile_ShippingAddressRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesProfile_ShippingAddressRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_1` after provisioning.\n"]
    pub fn address_1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_1", self.base))
    }

    #[doc = "Get a reference to the value of field `address_2` after provisioning.\n"]
    pub fn address_2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_2", self.base))
    }

    #[doc = "Get a reference to the value of field `address_3` after provisioning.\n"]
    pub fn address_3(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_3", self.base))
    }

    #[doc = "Get a reference to the value of field `address_4` after provisioning.\n"]
    pub fn address_4(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_4", self.base))
    }

    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.city", self.base))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\n"]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.base))
    }

    #[doc = "Get a reference to the value of field `county` after provisioning.\n"]
    pub fn county(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.county", self.base))
    }

    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\n"]
    pub fn postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.base))
    }

    #[doc = "Get a reference to the value of field `province` after provisioning.\n"]
    pub fn province(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.province", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
