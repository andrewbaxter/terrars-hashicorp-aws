#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct WorkmailUserData_ {
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
    city: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    department: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    email: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hidden_from_global_address_list: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider_user_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initials: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_title: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_name: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    office: std::option::Option<terrars::PrimField<std::string::String>>,
    organization_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    street: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    telephone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zip_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct WorkmailUser_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WorkmailUserData_>,
}

#[derive(Clone)]
pub struct WorkmailUser(std::rc::Rc<WorkmailUser_>);

impl WorkmailUser {
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

    #[doc = "Set the field `city`.\nCity where the user is located."]
    pub fn set_city(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().city = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company`.\nCompany associated with the user."]
    pub fn set_company(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().company = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `country`.\nCountry where the user is located."]
    pub fn set_country(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().country = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `department`.\nDepartment associated with the user."]
    pub fn set_department(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().department = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `first_name`.\nFirst name of the user."]
    pub fn set_first_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().first_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hidden_from_global_address_list`.\nWhether to hide the user from the global address list."]
    pub fn set_hidden_from_global_address_list(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().hidden_from_global_address_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_provider_user_id`.\nUser ID from IAM Identity Center associated with the user."]
    pub fn set_identity_provider_user_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().identity_provider_user_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initials`.\nInitials of the user."]
    pub fn set_initials(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().initials = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `job_title`.\nJob title of the user."]
    pub fn set_job_title(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().job_title = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_name`.\nLast name of the user."]
    pub fn set_last_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().last_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `office`.\nOffice where the user is located."]
    pub fn set_office(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().office = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password`.\nPassword to set for the user."]
    pub fn set_password(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `street`.\nStreet address of the user."]
    pub fn set_street(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().street = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `telephone`.\nTelephone number of the user."]
    pub fn set_telephone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().telephone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_role`.\nRole assigned to the user."]
    pub fn set_user_role(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zip_code`.\nZIP or postal code of the user."]
    pub fn set_zip_code(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zip_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `city` after provisioning.\nCity where the user is located."]
    pub fn city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.city", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company` after provisioning.\nCompany associated with the user."]
    pub fn company(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\nCountry where the user is located."]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `department` after provisioning.\nDepartment associated with the user."]
    pub fn department(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.department", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled_date` after provisioning.\nTimestamp when the user was disabled from WorkMail use."]
    pub fn disabled_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the user."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `email` after provisioning.\nPrimary email address used to register the user with WorkMail."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled_date` after provisioning.\nTimestamp when the user was enabled for WorkMail use."]
    pub fn enabled_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `first_name` after provisioning.\nFirst name of the user."]
    pub fn first_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hidden_from_global_address_list` after provisioning.\nWhether to hide the user from the global address list."]
    pub fn hidden_from_global_address_list(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.hidden_from_global_address_list", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_identity_store_id` after provisioning.\nIdentity store ID from IAM Identity Center associated with the user."]
    pub fn identity_provider_identity_store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_provider_identity_store_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_user_id` after provisioning.\nUser ID from IAM Identity Center associated with the user."]
    pub fn identity_provider_user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_user_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initials` after provisioning.\nInitials of the user."]
    pub fn initials(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initials", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_title` after provisioning.\nJob title of the user."]
    pub fn job_title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_title", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_name` after provisioning.\nLast name of the user."]
    pub fn last_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mailbox_deprovisioned_date` after provisioning.\nTimestamp when the mailbox was removed for the user."]
    pub fn mailbox_deprovisioned_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mailbox_deprovisioned_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mailbox_provisioned_date` after provisioning.\nTimestamp when the mailbox was created for the user."]
    pub fn mailbox_provisioned_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mailbox_provisioned_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nUsername of the user."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `office` after provisioning.\nOffice where the user is located."]
    pub fn office(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.office", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization_id` after provisioning.\nIdentifier of the WorkMail organization where the user is managed."]
    pub fn organization_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword to set for the user."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent WorkMail state of the user."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `street` after provisioning.\nStreet address of the user."]
    pub fn street(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.street", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `telephone` after provisioning.\nTelephone number of the user."]
    pub fn telephone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.telephone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\nIdentifier of the user."]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_role` after provisioning.\nRole assigned to the user."]
    pub fn user_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zip_code` after provisioning.\nZIP or postal code of the user."]
    pub fn zip_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zip_code", self.extract_ref()))
    }
}

impl terrars::Referable for WorkmailUser {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for WorkmailUser { }

impl terrars::ToListMappable for WorkmailUser {
    type O = terrars::ListRef<WorkmailUserRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for WorkmailUser_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_workmail_user".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWorkmailUser {
    pub tf_id: std::string::String,
    #[doc = "Display name of the user."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "Primary email address used to register the user with WorkMail."]
    pub email: terrars::PrimField<std::string::String>,
    #[doc = "Username of the user."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Identifier of the WorkMail organization where the user is managed."]
    pub organization_id: terrars::PrimField<std::string::String>,
}

impl BuildWorkmailUser {
    pub fn build(self, stack: &mut terrars::Stack) -> WorkmailUser {
        let out = WorkmailUser(std::rc::Rc::new(WorkmailUser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WorkmailUserData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                city: core::default::Default::default(),
                company: core::default::Default::default(),
                country: core::default::Default::default(),
                department: core::default::Default::default(),
                display_name: self.display_name,
                email: self.email,
                first_name: core::default::Default::default(),
                hidden_from_global_address_list: core::default::Default::default(),
                identity_provider_user_id: core::default::Default::default(),
                initials: core::default::Default::default(),
                job_title: core::default::Default::default(),
                last_name: core::default::Default::default(),
                name: self.name,
                office: core::default::Default::default(),
                organization_id: self.organization_id,
                password: core::default::Default::default(),
                region: core::default::Default::default(),
                street: core::default::Default::default(),
                telephone: core::default::Default::default(),
                user_role: core::default::Default::default(),
                zip_code: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WorkmailUserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkmailUserRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WorkmailUserRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `city` after provisioning.\nCity where the user is located."]
    pub fn city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.city", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company` after provisioning.\nCompany associated with the user."]
    pub fn company(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\nCountry where the user is located."]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `department` after provisioning.\nDepartment associated with the user."]
    pub fn department(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.department", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled_date` after provisioning.\nTimestamp when the user was disabled from WorkMail use."]
    pub fn disabled_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the user."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `email` after provisioning.\nPrimary email address used to register the user with WorkMail."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled_date` after provisioning.\nTimestamp when the user was enabled for WorkMail use."]
    pub fn enabled_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `first_name` after provisioning.\nFirst name of the user."]
    pub fn first_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hidden_from_global_address_list` after provisioning.\nWhether to hide the user from the global address list."]
    pub fn hidden_from_global_address_list(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.hidden_from_global_address_list", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_identity_store_id` after provisioning.\nIdentity store ID from IAM Identity Center associated with the user."]
    pub fn identity_provider_identity_store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_provider_identity_store_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `identity_provider_user_id` after provisioning.\nUser ID from IAM Identity Center associated with the user."]
    pub fn identity_provider_user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_user_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initials` after provisioning.\nInitials of the user."]
    pub fn initials(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initials", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_title` after provisioning.\nJob title of the user."]
    pub fn job_title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_title", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_name` after provisioning.\nLast name of the user."]
    pub fn last_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mailbox_deprovisioned_date` after provisioning.\nTimestamp when the mailbox was removed for the user."]
    pub fn mailbox_deprovisioned_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mailbox_deprovisioned_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mailbox_provisioned_date` after provisioning.\nTimestamp when the mailbox was created for the user."]
    pub fn mailbox_provisioned_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mailbox_provisioned_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nUsername of the user."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `office` after provisioning.\nOffice where the user is located."]
    pub fn office(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.office", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization_id` after provisioning.\nIdentifier of the WorkMail organization where the user is managed."]
    pub fn organization_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword to set for the user."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent WorkMail state of the user."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `street` after provisioning.\nStreet address of the user."]
    pub fn street(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.street", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `telephone` after provisioning.\nTelephone number of the user."]
    pub fn telephone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.telephone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\nIdentifier of the user."]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_role` after provisioning.\nRole assigned to the user."]
    pub fn user_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zip_code` after provisioning.\nZIP or postal code of the user."]
    pub fn zip_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zip_code", self.extract_ref()))
    }
}
