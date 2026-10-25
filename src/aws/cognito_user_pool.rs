#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CognitoUserPoolData_ {
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
    alias_attributes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_verified_attributes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_verification_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_verification_subject: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mfa_configuration: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sms_authentication_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sms_verification_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_pool_tier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username_attributes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_recovery_setting: std::option::Option<std::vec::Vec<CognitoUserPool_AccountRecoverySetting>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_create_user_config: std::option::Option<std::vec::Vec<CognitoUserPool_AdminCreateUserConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_configuration: std::option::Option<std::vec::Vec<CognitoUserPool_DeviceConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_configuration: std::option::Option<std::vec::Vec<CognitoUserPool_EmailConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_mfa_configuration: std::option::Option<std::vec::Vec<CognitoUserPool_EmailMfaConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_config: std::option::Option<std::vec::Vec<CognitoUserPool_LambdaConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_policy: std::option::Option<std::vec::Vec<CognitoUserPool_PasswordPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<std::vec::Vec<CognitoUserPool_Schema>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sign_in_policy: std::option::Option<std::vec::Vec<CognitoUserPool_SignInPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sms_configuration: std::option::Option<std::vec::Vec<CognitoUserPool_SmsConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    software_token_mfa_configuration: std::option::Option<std::vec::Vec<CognitoUserPool_SoftwareTokenMfaConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_attribute_update_settings: std::option::Option<std::vec::Vec<CognitoUserPool_UserAttributeUpdateSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_pool_add_ons: std::option::Option<std::vec::Vec<CognitoUserPool_UserPoolAddOns>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username_configuration: std::option::Option<std::vec::Vec<CognitoUserPool_UsernameConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verification_message_template: std::option::Option<std::vec::Vec<CognitoUserPool_VerificationMessageTemplate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_authn_configuration: std::option::Option<std::vec::Vec<CognitoUserPool_WebAuthnConfiguration>>,
    dynamic: CognitoUserPoolDynamic,
}

struct CognitoUserPool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CognitoUserPoolData_>,
}

#[derive(Clone)]
pub struct CognitoUserPool(std::rc::Rc<CognitoUserPool_>);

impl CognitoUserPool {
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

    #[doc = "Set the field `alias_attributes`.\n"]
    pub fn set_alias_attributes(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().alias_attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_verified_attributes`.\n"]
    pub fn set_auto_verified_attributes(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().auto_verified_attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_protection`.\n"]
    pub fn set_deletion_protection(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_verification_message`.\n"]
    pub fn set_email_verification_message(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().email_verification_message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_verification_subject`.\n"]
    pub fn set_email_verification_subject(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().email_verification_subject = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mfa_configuration`.\n"]
    pub fn set_mfa_configuration(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().mfa_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sms_authentication_message`.\n"]
    pub fn set_sms_authentication_message(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().sms_authentication_message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sms_verification_message`.\n"]
    pub fn set_sms_verification_message(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().sms_verification_message = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_pool_tier`.\n"]
    pub fn set_user_pool_tier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user_pool_tier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username_attributes`.\n"]
    pub fn set_username_attributes(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().username_attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `account_recovery_setting`.\n"]
    pub fn set_account_recovery_setting(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPool_AccountRecoverySetting>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().account_recovery_setting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.account_recovery_setting = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `admin_create_user_config`.\n"]
    pub fn set_admin_create_user_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPool_AdminCreateUserConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().admin_create_user_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.admin_create_user_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `device_configuration`.\n"]
    pub fn set_device_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPool_DeviceConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().device_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.device_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `email_configuration`.\n"]
    pub fn set_email_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPool_EmailConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().email_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.email_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `email_mfa_configuration`.\n"]
    pub fn set_email_mfa_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPool_EmailMfaConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().email_mfa_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.email_mfa_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_config`.\n"]
    pub fn set_lambda_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPool_LambdaConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lambda_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lambda_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `password_policy`.\n"]
    pub fn set_password_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPool_PasswordPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().password_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.password_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(self, v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPool_Schema>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schema = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sign_in_policy`.\n"]
    pub fn set_sign_in_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPool_SignInPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sign_in_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sign_in_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sms_configuration`.\n"]
    pub fn set_sms_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPool_SmsConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sms_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sms_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `software_token_mfa_configuration`.\n"]
    pub fn set_software_token_mfa_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPool_SoftwareTokenMfaConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().software_token_mfa_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.software_token_mfa_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_attribute_update_settings`.\n"]
    pub fn set_user_attribute_update_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPool_UserAttributeUpdateSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_attribute_update_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_attribute_update_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_pool_add_ons`.\n"]
    pub fn set_user_pool_add_ons(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPool_UserPoolAddOns>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_pool_add_ons = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_pool_add_ons = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `username_configuration`.\n"]
    pub fn set_username_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPool_UsernameConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().username_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.username_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `verification_message_template`.\n"]
    pub fn set_verification_message_template(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPool_VerificationMessageTemplate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().verification_message_template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.verification_message_template = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `web_authn_configuration`.\n"]
    pub fn set_web_authn_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPool_WebAuthnConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().web_authn_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.web_authn_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `alias_attributes` after provisioning.\n"]
    pub fn alias_attributes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.alias_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_verified_attributes` after provisioning.\n"]
    pub fn auto_verified_attributes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.auto_verified_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_domain` after provisioning.\n"]
    pub fn custom_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_verification_message` after provisioning.\n"]
    pub fn email_verification_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_verification_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_verification_subject` after provisioning.\n"]
    pub fn email_verification_subject(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_verification_subject", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `estimated_number_of_users` after provisioning.\n"]
    pub fn estimated_number_of_users(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.estimated_number_of_users", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_date` after provisioning.\n"]
    pub fn last_modified_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mfa_configuration` after provisioning.\n"]
    pub fn mfa_configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mfa_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_authentication_message` after provisioning.\n"]
    pub fn sms_authentication_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sms_authentication_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_verification_message` after provisioning.\n"]
    pub fn sms_verification_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sms_verification_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_tier` after provisioning.\n"]
    pub fn user_pool_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_tier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `username_attributes` after provisioning.\n"]
    pub fn username_attributes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.username_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_recovery_setting` after provisioning.\n"]
    pub fn account_recovery_setting(&self) -> terrars::ListRef<CognitoUserPool_AccountRecoverySettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.account_recovery_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_create_user_config` after provisioning.\n"]
    pub fn admin_create_user_config(&self) -> terrars::ListRef<CognitoUserPool_AdminCreateUserConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_create_user_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device_configuration` after provisioning.\n"]
    pub fn device_configuration(&self) -> terrars::ListRef<CognitoUserPool_DeviceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.device_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_configuration` after provisioning.\n"]
    pub fn email_configuration(&self) -> terrars::ListRef<CognitoUserPool_EmailConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_mfa_configuration` after provisioning.\n"]
    pub fn email_mfa_configuration(&self) -> terrars::ListRef<CognitoUserPool_EmailMfaConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_mfa_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_config` after provisioning.\n"]
    pub fn lambda_config(&self) -> terrars::ListRef<CognitoUserPool_LambdaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_policy` after provisioning.\n"]
    pub fn password_policy(&self) -> terrars::ListRef<CognitoUserPool_PasswordPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.password_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sign_in_policy` after provisioning.\n"]
    pub fn sign_in_policy(&self) -> terrars::ListRef<CognitoUserPool_SignInPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sign_in_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_configuration` after provisioning.\n"]
    pub fn sms_configuration(&self) -> terrars::ListRef<CognitoUserPool_SmsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sms_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `software_token_mfa_configuration` after provisioning.\n"]
    pub fn software_token_mfa_configuration(&self) -> terrars::ListRef<CognitoUserPool_SoftwareTokenMfaConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.software_token_mfa_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_attribute_update_settings` after provisioning.\n"]
    pub fn user_attribute_update_settings(&self) -> terrars::ListRef<CognitoUserPool_UserAttributeUpdateSettingsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.user_attribute_update_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_pool_add_ons` after provisioning.\n"]
    pub fn user_pool_add_ons(&self) -> terrars::ListRef<CognitoUserPool_UserPoolAddOnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_pool_add_ons", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `username_configuration` after provisioning.\n"]
    pub fn username_configuration(&self) -> terrars::ListRef<CognitoUserPool_UsernameConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.username_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verification_message_template` after provisioning.\n"]
    pub fn verification_message_template(&self) -> terrars::ListRef<CognitoUserPool_VerificationMessageTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.verification_message_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_authn_configuration` after provisioning.\n"]
    pub fn web_authn_configuration(&self) -> terrars::ListRef<CognitoUserPool_WebAuthnConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_authn_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for CognitoUserPool {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CognitoUserPool { }

impl terrars::ToListMappable for CognitoUserPool {
    type O = terrars::ListRef<CognitoUserPoolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CognitoUserPool_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cognito_user_pool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCognitoUserPool {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCognitoUserPool {
    pub fn build(self, stack: &mut terrars::Stack) -> CognitoUserPool {
        let out = CognitoUserPool(std::rc::Rc::new(CognitoUserPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CognitoUserPoolData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                alias_attributes: core::default::Default::default(),
                auto_verified_attributes: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                email_verification_message: core::default::Default::default(),
                email_verification_subject: core::default::Default::default(),
                id: core::default::Default::default(),
                mfa_configuration: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                sms_authentication_message: core::default::Default::default(),
                sms_verification_message: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                user_pool_tier: core::default::Default::default(),
                username_attributes: core::default::Default::default(),
                account_recovery_setting: core::default::Default::default(),
                admin_create_user_config: core::default::Default::default(),
                device_configuration: core::default::Default::default(),
                email_configuration: core::default::Default::default(),
                email_mfa_configuration: core::default::Default::default(),
                lambda_config: core::default::Default::default(),
                password_policy: core::default::Default::default(),
                schema: core::default::Default::default(),
                sign_in_policy: core::default::Default::default(),
                sms_configuration: core::default::Default::default(),
                software_token_mfa_configuration: core::default::Default::default(),
                user_attribute_update_settings: core::default::Default::default(),
                user_pool_add_ons: core::default::Default::default(),
                username_configuration: core::default::Default::default(),
                verification_message_template: core::default::Default::default(),
                web_authn_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CognitoUserPoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CognitoUserPoolRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alias_attributes` after provisioning.\n"]
    pub fn alias_attributes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.alias_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_verified_attributes` after provisioning.\n"]
    pub fn auto_verified_attributes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.auto_verified_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_domain` after provisioning.\n"]
    pub fn custom_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_verification_message` after provisioning.\n"]
    pub fn email_verification_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_verification_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_verification_subject` after provisioning.\n"]
    pub fn email_verification_subject(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_verification_subject", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `estimated_number_of_users` after provisioning.\n"]
    pub fn estimated_number_of_users(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.estimated_number_of_users", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_date` after provisioning.\n"]
    pub fn last_modified_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mfa_configuration` after provisioning.\n"]
    pub fn mfa_configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mfa_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_authentication_message` after provisioning.\n"]
    pub fn sms_authentication_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sms_authentication_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_verification_message` after provisioning.\n"]
    pub fn sms_verification_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sms_verification_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_tier` after provisioning.\n"]
    pub fn user_pool_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_tier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `username_attributes` after provisioning.\n"]
    pub fn username_attributes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.username_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_recovery_setting` after provisioning.\n"]
    pub fn account_recovery_setting(&self) -> terrars::ListRef<CognitoUserPool_AccountRecoverySettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.account_recovery_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_create_user_config` after provisioning.\n"]
    pub fn admin_create_user_config(&self) -> terrars::ListRef<CognitoUserPool_AdminCreateUserConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_create_user_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device_configuration` after provisioning.\n"]
    pub fn device_configuration(&self) -> terrars::ListRef<CognitoUserPool_DeviceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.device_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_configuration` after provisioning.\n"]
    pub fn email_configuration(&self) -> terrars::ListRef<CognitoUserPool_EmailConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_mfa_configuration` after provisioning.\n"]
    pub fn email_mfa_configuration(&self) -> terrars::ListRef<CognitoUserPool_EmailMfaConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_mfa_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_config` after provisioning.\n"]
    pub fn lambda_config(&self) -> terrars::ListRef<CognitoUserPool_LambdaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_policy` after provisioning.\n"]
    pub fn password_policy(&self) -> terrars::ListRef<CognitoUserPool_PasswordPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.password_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sign_in_policy` after provisioning.\n"]
    pub fn sign_in_policy(&self) -> terrars::ListRef<CognitoUserPool_SignInPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sign_in_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_configuration` after provisioning.\n"]
    pub fn sms_configuration(&self) -> terrars::ListRef<CognitoUserPool_SmsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sms_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `software_token_mfa_configuration` after provisioning.\n"]
    pub fn software_token_mfa_configuration(&self) -> terrars::ListRef<CognitoUserPool_SoftwareTokenMfaConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.software_token_mfa_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_attribute_update_settings` after provisioning.\n"]
    pub fn user_attribute_update_settings(&self) -> terrars::ListRef<CognitoUserPool_UserAttributeUpdateSettingsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.user_attribute_update_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_pool_add_ons` after provisioning.\n"]
    pub fn user_pool_add_ons(&self) -> terrars::ListRef<CognitoUserPool_UserPoolAddOnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_pool_add_ons", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `username_configuration` after provisioning.\n"]
    pub fn username_configuration(&self) -> terrars::ListRef<CognitoUserPool_UsernameConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.username_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verification_message_template` after provisioning.\n"]
    pub fn verification_message_template(&self) -> terrars::ListRef<CognitoUserPool_VerificationMessageTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.verification_message_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_authn_configuration` after provisioning.\n"]
    pub fn web_authn_configuration(&self) -> terrars::ListRef<CognitoUserPool_WebAuthnConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_authn_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CognitoUserPoolDynamic {
    account_recovery_setting: std::option::Option<terrars::DynamicBlock<CognitoUserPool_AccountRecoverySetting>>,
    admin_create_user_config: std::option::Option<terrars::DynamicBlock<CognitoUserPool_AdminCreateUserConfig>>,
    device_configuration: std::option::Option<terrars::DynamicBlock<CognitoUserPool_DeviceConfiguration>>,
    email_configuration: std::option::Option<terrars::DynamicBlock<CognitoUserPool_EmailConfiguration>>,
    email_mfa_configuration: std::option::Option<terrars::DynamicBlock<CognitoUserPool_EmailMfaConfiguration>>,
    lambda_config: std::option::Option<terrars::DynamicBlock<CognitoUserPool_LambdaConfig>>,
    password_policy: std::option::Option<terrars::DynamicBlock<CognitoUserPool_PasswordPolicy>>,
    schema: std::option::Option<terrars::DynamicBlock<CognitoUserPool_Schema>>,
    sign_in_policy: std::option::Option<terrars::DynamicBlock<CognitoUserPool_SignInPolicy>>,
    sms_configuration: std::option::Option<terrars::DynamicBlock<CognitoUserPool_SmsConfiguration>>,
    software_token_mfa_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoUserPool_SoftwareTokenMfaConfiguration>>,
    user_attribute_update_settings: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoUserPool_UserAttributeUpdateSettings>>,
    user_pool_add_ons: std::option::Option<terrars::DynamicBlock<CognitoUserPool_UserPoolAddOns>>,
    username_configuration: std::option::Option<terrars::DynamicBlock<CognitoUserPool_UsernameConfiguration>>,
    verification_message_template: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoUserPool_VerificationMessageTemplate>>,
    web_authn_configuration: std::option::Option<terrars::DynamicBlock<CognitoUserPool_WebAuthnConfiguration>>,
}

#[derive(Serialize)]
pub struct CognitoUserPool_AccountRecoverySetting_RecoveryMechanism {
    name: terrars::PrimField<std::string::String>,
    priority: terrars::PrimField<f64>,
}

impl CognitoUserPool_AccountRecoverySetting_RecoveryMechanism { }

impl terrars::ToListMappable for CognitoUserPool_AccountRecoverySetting_RecoveryMechanism {
    type O = terrars::BlockAssignable<CognitoUserPool_AccountRecoverySetting_RecoveryMechanism>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_AccountRecoverySetting_RecoveryMechanism {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub priority: terrars::PrimField<f64>,
}

impl BuildCognitoUserPool_AccountRecoverySetting_RecoveryMechanism {
    pub fn build(self) -> CognitoUserPool_AccountRecoverySetting_RecoveryMechanism {
        CognitoUserPool_AccountRecoverySetting_RecoveryMechanism {
            name: self.name,
            priority: self.priority,
        }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_AccountRecoverySetting_RecoveryMechanismRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_AccountRecoverySetting_RecoveryMechanismRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoUserPool_AccountRecoverySetting_RecoveryMechanismRef {
        CognitoUserPool_AccountRecoverySetting_RecoveryMechanismRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_AccountRecoverySetting_RecoveryMechanismRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoUserPool_AccountRecoverySettingDynamic {
    recovery_mechanism: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoUserPool_AccountRecoverySetting_RecoveryMechanism>>,
}

#[derive(Serialize)]
pub struct CognitoUserPool_AccountRecoverySetting {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recovery_mechanism: std::option::Option<std::vec::Vec<CognitoUserPool_AccountRecoverySetting_RecoveryMechanism>>,
    dynamic: CognitoUserPool_AccountRecoverySettingDynamic,
}

impl CognitoUserPool_AccountRecoverySetting {
    #[doc = "Set the field `recovery_mechanism`.\n"]
    pub fn set_recovery_mechanism(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CognitoUserPool_AccountRecoverySetting_RecoveryMechanism>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.recovery_mechanism = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.recovery_mechanism = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CognitoUserPool_AccountRecoverySetting {
    type O = terrars::BlockAssignable<CognitoUserPool_AccountRecoverySetting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_AccountRecoverySetting {}

impl BuildCognitoUserPool_AccountRecoverySetting {
    pub fn build(self) -> CognitoUserPool_AccountRecoverySetting {
        CognitoUserPool_AccountRecoverySetting {
            recovery_mechanism: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_AccountRecoverySettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_AccountRecoverySettingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPool_AccountRecoverySettingRef {
        CognitoUserPool_AccountRecoverySettingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_AccountRecoverySettingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct CognitoUserPool_AdminCreateUserConfig_InviteMessageTemplate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_subject: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sms_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CognitoUserPool_AdminCreateUserConfig_InviteMessageTemplate {
    #[doc = "Set the field `email_message`.\n"]
    pub fn set_email_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email_message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_subject`.\n"]
    pub fn set_email_subject(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email_subject = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sms_message`.\n"]
    pub fn set_sms_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sms_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoUserPool_AdminCreateUserConfig_InviteMessageTemplate {
    type O = terrars::BlockAssignable<CognitoUserPool_AdminCreateUserConfig_InviteMessageTemplate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_AdminCreateUserConfig_InviteMessageTemplate {}

impl BuildCognitoUserPool_AdminCreateUserConfig_InviteMessageTemplate {
    pub fn build(self) -> CognitoUserPool_AdminCreateUserConfig_InviteMessageTemplate {
        CognitoUserPool_AdminCreateUserConfig_InviteMessageTemplate {
            email_message: core::default::Default::default(),
            email_subject: core::default::Default::default(),
            sms_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_AdminCreateUserConfig_InviteMessageTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_AdminCreateUserConfig_InviteMessageTemplateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoUserPool_AdminCreateUserConfig_InviteMessageTemplateRef {
        CognitoUserPool_AdminCreateUserConfig_InviteMessageTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_AdminCreateUserConfig_InviteMessageTemplateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email_message` after provisioning.\n"]
    pub fn email_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_message", self.base))
    }

    #[doc = "Get a reference to the value of field `email_subject` after provisioning.\n"]
    pub fn email_subject(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_subject", self.base))
    }

    #[doc = "Get a reference to the value of field `sms_message` after provisioning.\n"]
    pub fn sms_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sms_message", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoUserPool_AdminCreateUserConfigDynamic {
    invite_message_template: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoUserPool_AdminCreateUserConfig_InviteMessageTemplate>>,
}

#[derive(Serialize)]
pub struct CognitoUserPool_AdminCreateUserConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_admin_create_user_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invite_message_template: std
    ::option
    ::Option<std::vec::Vec<CognitoUserPool_AdminCreateUserConfig_InviteMessageTemplate>>,
    dynamic: CognitoUserPool_AdminCreateUserConfigDynamic,
}

impl CognitoUserPool_AdminCreateUserConfig {
    #[doc = "Set the field `allow_admin_create_user_only`.\n"]
    pub fn set_allow_admin_create_user_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_admin_create_user_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `invite_message_template`.\n"]
    pub fn set_invite_message_template(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CognitoUserPool_AdminCreateUserConfig_InviteMessageTemplate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.invite_message_template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.invite_message_template = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CognitoUserPool_AdminCreateUserConfig {
    type O = terrars::BlockAssignable<CognitoUserPool_AdminCreateUserConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_AdminCreateUserConfig {}

impl BuildCognitoUserPool_AdminCreateUserConfig {
    pub fn build(self) -> CognitoUserPool_AdminCreateUserConfig {
        CognitoUserPool_AdminCreateUserConfig {
            allow_admin_create_user_only: core::default::Default::default(),
            invite_message_template: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_AdminCreateUserConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_AdminCreateUserConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPool_AdminCreateUserConfigRef {
        CognitoUserPool_AdminCreateUserConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_AdminCreateUserConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_admin_create_user_only` after provisioning.\n"]
    pub fn allow_admin_create_user_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_admin_create_user_only", self.base))
    }

    #[doc = "Get a reference to the value of field `invite_message_template` after provisioning.\n"]
    pub fn invite_message_template(
        &self,
    ) -> terrars::ListRef<CognitoUserPool_AdminCreateUserConfig_InviteMessageTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.invite_message_template", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPool_DeviceConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    challenge_required_on_new_device: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_only_remembered_on_user_prompt: std::option::Option<terrars::PrimField<bool>>,
}

impl CognitoUserPool_DeviceConfiguration {
    #[doc = "Set the field `challenge_required_on_new_device`.\n"]
    pub fn set_challenge_required_on_new_device(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.challenge_required_on_new_device = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_only_remembered_on_user_prompt`.\n"]
    pub fn set_device_only_remembered_on_user_prompt(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.device_only_remembered_on_user_prompt = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoUserPool_DeviceConfiguration {
    type O = terrars::BlockAssignable<CognitoUserPool_DeviceConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_DeviceConfiguration {}

impl BuildCognitoUserPool_DeviceConfiguration {
    pub fn build(self) -> CognitoUserPool_DeviceConfiguration {
        CognitoUserPool_DeviceConfiguration {
            challenge_required_on_new_device: core::default::Default::default(),
            device_only_remembered_on_user_prompt: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_DeviceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_DeviceConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPool_DeviceConfigurationRef {
        CognitoUserPool_DeviceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_DeviceConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `challenge_required_on_new_device` after provisioning.\n"]
    pub fn challenge_required_on_new_device(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.challenge_required_on_new_device", self.base))
    }

    #[doc = "Get a reference to the value of field `device_only_remembered_on_user_prompt` after provisioning.\n"]
    pub fn device_only_remembered_on_user_prompt(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_only_remembered_on_user_prompt", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPool_EmailConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration_set: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_sending_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_email_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reply_to_email_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CognitoUserPool_EmailConfiguration {
    #[doc = "Set the field `configuration_set`.\n"]
    pub fn set_configuration_set(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.configuration_set = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_sending_account`.\n"]
    pub fn set_email_sending_account(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.email_sending_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `from_email_address`.\n"]
    pub fn set_from_email_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.from_email_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reply_to_email_address`.\n"]
    pub fn set_reply_to_email_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.reply_to_email_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_arn`.\n"]
    pub fn set_source_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoUserPool_EmailConfiguration {
    type O = terrars::BlockAssignable<CognitoUserPool_EmailConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_EmailConfiguration {}

impl BuildCognitoUserPool_EmailConfiguration {
    pub fn build(self) -> CognitoUserPool_EmailConfiguration {
        CognitoUserPool_EmailConfiguration {
            configuration_set: core::default::Default::default(),
            email_sending_account: core::default::Default::default(),
            from_email_address: core::default::Default::default(),
            reply_to_email_address: core::default::Default::default(),
            source_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_EmailConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_EmailConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPool_EmailConfigurationRef {
        CognitoUserPool_EmailConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_EmailConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configuration_set` after provisioning.\n"]
    pub fn configuration_set(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_set", self.base))
    }

    #[doc = "Get a reference to the value of field `email_sending_account` after provisioning.\n"]
    pub fn email_sending_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_sending_account", self.base))
    }

    #[doc = "Get a reference to the value of field `from_email_address` after provisioning.\n"]
    pub fn from_email_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_email_address", self.base))
    }

    #[doc = "Get a reference to the value of field `reply_to_email_address` after provisioning.\n"]
    pub fn reply_to_email_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reply_to_email_address", self.base))
    }

    #[doc = "Get a reference to the value of field `source_arn` after provisioning.\n"]
    pub fn source_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPool_EmailMfaConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CognitoUserPool_EmailMfaConfiguration {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject`.\n"]
    pub fn set_subject(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subject = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoUserPool_EmailMfaConfiguration {
    type O = terrars::BlockAssignable<CognitoUserPool_EmailMfaConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_EmailMfaConfiguration {}

impl BuildCognitoUserPool_EmailMfaConfiguration {
    pub fn build(self) -> CognitoUserPool_EmailMfaConfiguration {
        CognitoUserPool_EmailMfaConfiguration {
            message: core::default::Default::default(),
            subject: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_EmailMfaConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_EmailMfaConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPool_EmailMfaConfigurationRef {
        CognitoUserPool_EmailMfaConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_EmailMfaConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `subject` after provisioning.\n"]
    pub fn subject(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subject", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPool_LambdaConfig_CustomEmailSender {
    lambda_arn: terrars::PrimField<std::string::String>,
    lambda_version: terrars::PrimField<std::string::String>,
}

impl CognitoUserPool_LambdaConfig_CustomEmailSender { }

impl terrars::ToListMappable for CognitoUserPool_LambdaConfig_CustomEmailSender {
    type O = terrars::BlockAssignable<CognitoUserPool_LambdaConfig_CustomEmailSender>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_LambdaConfig_CustomEmailSender {
    #[doc = ""]
    pub lambda_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub lambda_version: terrars::PrimField<std::string::String>,
}

impl BuildCognitoUserPool_LambdaConfig_CustomEmailSender {
    pub fn build(self) -> CognitoUserPool_LambdaConfig_CustomEmailSender {
        CognitoUserPool_LambdaConfig_CustomEmailSender {
            lambda_arn: self.lambda_arn,
            lambda_version: self.lambda_version,
        }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_LambdaConfig_CustomEmailSenderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_LambdaConfig_CustomEmailSenderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoUserPool_LambdaConfig_CustomEmailSenderRef {
        CognitoUserPool_LambdaConfig_CustomEmailSenderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_LambdaConfig_CustomEmailSenderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_version` after provisioning.\n"]
    pub fn lambda_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda_version", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPool_LambdaConfig_CustomSmsSender {
    lambda_arn: terrars::PrimField<std::string::String>,
    lambda_version: terrars::PrimField<std::string::String>,
}

impl CognitoUserPool_LambdaConfig_CustomSmsSender { }

impl terrars::ToListMappable for CognitoUserPool_LambdaConfig_CustomSmsSender {
    type O = terrars::BlockAssignable<CognitoUserPool_LambdaConfig_CustomSmsSender>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_LambdaConfig_CustomSmsSender {
    #[doc = ""]
    pub lambda_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub lambda_version: terrars::PrimField<std::string::String>,
}

impl BuildCognitoUserPool_LambdaConfig_CustomSmsSender {
    pub fn build(self) -> CognitoUserPool_LambdaConfig_CustomSmsSender {
        CognitoUserPool_LambdaConfig_CustomSmsSender {
            lambda_arn: self.lambda_arn,
            lambda_version: self.lambda_version,
        }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_LambdaConfig_CustomSmsSenderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_LambdaConfig_CustomSmsSenderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoUserPool_LambdaConfig_CustomSmsSenderRef {
        CognitoUserPool_LambdaConfig_CustomSmsSenderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_LambdaConfig_CustomSmsSenderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_version` after provisioning.\n"]
    pub fn lambda_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda_version", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPool_LambdaConfig_PreTokenGenerationConfig {
    lambda_arn: terrars::PrimField<std::string::String>,
    lambda_version: terrars::PrimField<std::string::String>,
}

impl CognitoUserPool_LambdaConfig_PreTokenGenerationConfig { }

impl terrars::ToListMappable for CognitoUserPool_LambdaConfig_PreTokenGenerationConfig {
    type O = terrars::BlockAssignable<CognitoUserPool_LambdaConfig_PreTokenGenerationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_LambdaConfig_PreTokenGenerationConfig {
    #[doc = ""]
    pub lambda_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub lambda_version: terrars::PrimField<std::string::String>,
}

impl BuildCognitoUserPool_LambdaConfig_PreTokenGenerationConfig {
    pub fn build(self) -> CognitoUserPool_LambdaConfig_PreTokenGenerationConfig {
        CognitoUserPool_LambdaConfig_PreTokenGenerationConfig {
            lambda_arn: self.lambda_arn,
            lambda_version: self.lambda_version,
        }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_LambdaConfig_PreTokenGenerationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_LambdaConfig_PreTokenGenerationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoUserPool_LambdaConfig_PreTokenGenerationConfigRef {
        CognitoUserPool_LambdaConfig_PreTokenGenerationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_LambdaConfig_PreTokenGenerationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_version` after provisioning.\n"]
    pub fn lambda_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoUserPool_LambdaConfigDynamic {
    custom_email_sender: std::option::Option<terrars::DynamicBlock<CognitoUserPool_LambdaConfig_CustomEmailSender>>,
    custom_sms_sender: std::option::Option<terrars::DynamicBlock<CognitoUserPool_LambdaConfig_CustomSmsSender>>,
    pre_token_generation_config: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoUserPool_LambdaConfig_PreTokenGenerationConfig>>,
}

#[derive(Serialize)]
pub struct CognitoUserPool_LambdaConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_auth_challenge: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    define_auth_challenge: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_authentication: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_confirmation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pre_authentication: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pre_sign_up: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pre_token_generation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_migration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verify_auth_challenge_response: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_email_sender: std::option::Option<std::vec::Vec<CognitoUserPool_LambdaConfig_CustomEmailSender>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_sms_sender: std::option::Option<std::vec::Vec<CognitoUserPool_LambdaConfig_CustomSmsSender>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pre_token_generation_config: std
    ::option
    ::Option<std::vec::Vec<CognitoUserPool_LambdaConfig_PreTokenGenerationConfig>>,
    dynamic: CognitoUserPool_LambdaConfigDynamic,
}

impl CognitoUserPool_LambdaConfig {
    #[doc = "Set the field `create_auth_challenge`.\n"]
    pub fn set_create_auth_challenge(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.create_auth_challenge = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_message`.\n"]
    pub fn set_custom_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.custom_message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `define_auth_challenge`.\n"]
    pub fn set_define_auth_challenge(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.define_auth_challenge = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `post_authentication`.\n"]
    pub fn set_post_authentication(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.post_authentication = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `post_confirmation`.\n"]
    pub fn set_post_confirmation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.post_confirmation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pre_authentication`.\n"]
    pub fn set_pre_authentication(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pre_authentication = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pre_sign_up`.\n"]
    pub fn set_pre_sign_up(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pre_sign_up = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pre_token_generation`.\n"]
    pub fn set_pre_token_generation(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pre_token_generation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_migration`.\n"]
    pub fn set_user_migration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_migration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `verify_auth_challenge_response`.\n"]
    pub fn set_verify_auth_challenge_response(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.verify_auth_challenge_response = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_email_sender`.\n"]
    pub fn set_custom_email_sender(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPool_LambdaConfig_CustomEmailSender>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_email_sender = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_email_sender = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_sms_sender`.\n"]
    pub fn set_custom_sms_sender(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPool_LambdaConfig_CustomSmsSender>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_sms_sender = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_sms_sender = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pre_token_generation_config`.\n"]
    pub fn set_pre_token_generation_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPool_LambdaConfig_PreTokenGenerationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pre_token_generation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pre_token_generation_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CognitoUserPool_LambdaConfig {
    type O = terrars::BlockAssignable<CognitoUserPool_LambdaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_LambdaConfig {}

impl BuildCognitoUserPool_LambdaConfig {
    pub fn build(self) -> CognitoUserPool_LambdaConfig {
        CognitoUserPool_LambdaConfig {
            create_auth_challenge: core::default::Default::default(),
            custom_message: core::default::Default::default(),
            define_auth_challenge: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            post_authentication: core::default::Default::default(),
            post_confirmation: core::default::Default::default(),
            pre_authentication: core::default::Default::default(),
            pre_sign_up: core::default::Default::default(),
            pre_token_generation: core::default::Default::default(),
            user_migration: core::default::Default::default(),
            verify_auth_challenge_response: core::default::Default::default(),
            custom_email_sender: core::default::Default::default(),
            custom_sms_sender: core::default::Default::default(),
            pre_token_generation_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_LambdaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_LambdaConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPool_LambdaConfigRef {
        CognitoUserPool_LambdaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_LambdaConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create_auth_challenge` after provisioning.\n"]
    pub fn create_auth_challenge(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_auth_challenge", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_message` after provisioning.\n"]
    pub fn custom_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_message", self.base))
    }

    #[doc = "Get a reference to the value of field `define_auth_challenge` after provisioning.\n"]
    pub fn define_auth_challenge(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.define_auth_challenge", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `post_authentication` after provisioning.\n"]
    pub fn post_authentication(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.post_authentication", self.base))
    }

    #[doc = "Get a reference to the value of field `post_confirmation` after provisioning.\n"]
    pub fn post_confirmation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.post_confirmation", self.base))
    }

    #[doc = "Get a reference to the value of field `pre_authentication` after provisioning.\n"]
    pub fn pre_authentication(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pre_authentication", self.base))
    }

    #[doc = "Get a reference to the value of field `pre_sign_up` after provisioning.\n"]
    pub fn pre_sign_up(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pre_sign_up", self.base))
    }

    #[doc = "Get a reference to the value of field `pre_token_generation` after provisioning.\n"]
    pub fn pre_token_generation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pre_token_generation", self.base))
    }

    #[doc = "Get a reference to the value of field `user_migration` after provisioning.\n"]
    pub fn user_migration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_migration", self.base))
    }

    #[doc = "Get a reference to the value of field `verify_auth_challenge_response` after provisioning.\n"]
    pub fn verify_auth_challenge_response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verify_auth_challenge_response", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_email_sender` after provisioning.\n"]
    pub fn custom_email_sender(&self) -> terrars::ListRef<CognitoUserPool_LambdaConfig_CustomEmailSenderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_email_sender", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_sms_sender` after provisioning.\n"]
    pub fn custom_sms_sender(&self) -> terrars::ListRef<CognitoUserPool_LambdaConfig_CustomSmsSenderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_sms_sender", self.base))
    }

    #[doc = "Get a reference to the value of field `pre_token_generation_config` after provisioning.\n"]
    pub fn pre_token_generation_config(
        &self,
    ) -> terrars::ListRef<CognitoUserPool_LambdaConfig_PreTokenGenerationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pre_token_generation_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPool_PasswordPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_history_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_lowercase: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_numbers: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_symbols: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_uppercase: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temporary_password_validity_days: std::option::Option<terrars::PrimField<f64>>,
}

impl CognitoUserPool_PasswordPolicy {
    #[doc = "Set the field `minimum_length`.\n"]
    pub fn set_minimum_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minimum_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password_history_size`.\n"]
    pub fn set_password_history_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.password_history_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_lowercase`.\n"]
    pub fn set_require_lowercase(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_lowercase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_numbers`.\n"]
    pub fn set_require_numbers(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_numbers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_symbols`.\n"]
    pub fn set_require_symbols(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_symbols = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_uppercase`.\n"]
    pub fn set_require_uppercase(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_uppercase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `temporary_password_validity_days`.\n"]
    pub fn set_temporary_password_validity_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.temporary_password_validity_days = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoUserPool_PasswordPolicy {
    type O = terrars::BlockAssignable<CognitoUserPool_PasswordPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_PasswordPolicy {}

impl BuildCognitoUserPool_PasswordPolicy {
    pub fn build(self) -> CognitoUserPool_PasswordPolicy {
        CognitoUserPool_PasswordPolicy {
            minimum_length: core::default::Default::default(),
            password_history_size: core::default::Default::default(),
            require_lowercase: core::default::Default::default(),
            require_numbers: core::default::Default::default(),
            require_symbols: core::default::Default::default(),
            require_uppercase: core::default::Default::default(),
            temporary_password_validity_days: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_PasswordPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_PasswordPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPool_PasswordPolicyRef {
        CognitoUserPool_PasswordPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_PasswordPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `minimum_length` after provisioning.\n"]
    pub fn minimum_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_length", self.base))
    }

    #[doc = "Get a reference to the value of field `password_history_size` after provisioning.\n"]
    pub fn password_history_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_history_size", self.base))
    }

    #[doc = "Get a reference to the value of field `require_lowercase` after provisioning.\n"]
    pub fn require_lowercase(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_lowercase", self.base))
    }

    #[doc = "Get a reference to the value of field `require_numbers` after provisioning.\n"]
    pub fn require_numbers(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_numbers", self.base))
    }

    #[doc = "Get a reference to the value of field `require_symbols` after provisioning.\n"]
    pub fn require_symbols(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_symbols", self.base))
    }

    #[doc = "Get a reference to the value of field `require_uppercase` after provisioning.\n"]
    pub fn require_uppercase(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_uppercase", self.base))
    }

    #[doc = "Get a reference to the value of field `temporary_password_validity_days` after provisioning.\n"]
    pub fn temporary_password_validity_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temporary_password_validity_days", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPool_Schema_NumberAttributeConstraints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CognitoUserPool_Schema_NumberAttributeConstraints {
    #[doc = "Set the field `max_value`.\n"]
    pub fn set_max_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_value`.\n"]
    pub fn set_min_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoUserPool_Schema_NumberAttributeConstraints {
    type O = terrars::BlockAssignable<CognitoUserPool_Schema_NumberAttributeConstraints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_Schema_NumberAttributeConstraints {}

impl BuildCognitoUserPool_Schema_NumberAttributeConstraints {
    pub fn build(self) -> CognitoUserPool_Schema_NumberAttributeConstraints {
        CognitoUserPool_Schema_NumberAttributeConstraints {
            max_value: core::default::Default::default(),
            min_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_Schema_NumberAttributeConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_Schema_NumberAttributeConstraintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoUserPool_Schema_NumberAttributeConstraintsRef {
        CognitoUserPool_Schema_NumberAttributeConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_Schema_NumberAttributeConstraintsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_value` after provisioning.\n"]
    pub fn max_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_value", self.base))
    }

    #[doc = "Get a reference to the value of field `min_value` after provisioning.\n"]
    pub fn min_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_value", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPool_Schema_StringAttributeConstraints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_length: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_length: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CognitoUserPool_Schema_StringAttributeConstraints {
    #[doc = "Set the field `max_length`.\n"]
    pub fn set_max_length(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_length`.\n"]
    pub fn set_min_length(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_length = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoUserPool_Schema_StringAttributeConstraints {
    type O = terrars::BlockAssignable<CognitoUserPool_Schema_StringAttributeConstraints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_Schema_StringAttributeConstraints {}

impl BuildCognitoUserPool_Schema_StringAttributeConstraints {
    pub fn build(self) -> CognitoUserPool_Schema_StringAttributeConstraints {
        CognitoUserPool_Schema_StringAttributeConstraints {
            max_length: core::default::Default::default(),
            min_length: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_Schema_StringAttributeConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_Schema_StringAttributeConstraintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoUserPool_Schema_StringAttributeConstraintsRef {
        CognitoUserPool_Schema_StringAttributeConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_Schema_StringAttributeConstraintsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_length` after provisioning.\n"]
    pub fn max_length(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_length", self.base))
    }

    #[doc = "Get a reference to the value of field `min_length` after provisioning.\n"]
    pub fn min_length(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_length", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoUserPool_SchemaDynamic {
    number_attribute_constraints: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoUserPool_Schema_NumberAttributeConstraints>>,
    string_attribute_constraints: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoUserPool_Schema_StringAttributeConstraints>>,
}

#[derive(Serialize)]
pub struct CognitoUserPool_Schema {
    attribute_data_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    developer_only_attribute: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mutable: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_attribute_constraints: std::option::Option<std::vec::Vec<CognitoUserPool_Schema_NumberAttributeConstraints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_attribute_constraints: std::option::Option<std::vec::Vec<CognitoUserPool_Schema_StringAttributeConstraints>>,
    dynamic: CognitoUserPool_SchemaDynamic,
}

impl CognitoUserPool_Schema {
    #[doc = "Set the field `developer_only_attribute`.\n"]
    pub fn set_developer_only_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.developer_only_attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mutable`.\n"]
    pub fn set_mutable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.mutable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `required`.\n"]
    pub fn set_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_attribute_constraints`.\n"]
    pub fn set_number_attribute_constraints(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPool_Schema_NumberAttributeConstraints>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.number_attribute_constraints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.number_attribute_constraints = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `string_attribute_constraints`.\n"]
    pub fn set_string_attribute_constraints(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPool_Schema_StringAttributeConstraints>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.string_attribute_constraints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.string_attribute_constraints = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CognitoUserPool_Schema {
    type O = terrars::BlockAssignable<CognitoUserPool_Schema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_Schema {
    #[doc = ""]
    pub attribute_data_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCognitoUserPool_Schema {
    pub fn build(self) -> CognitoUserPool_Schema {
        CognitoUserPool_Schema {
            attribute_data_type: self.attribute_data_type,
            developer_only_attribute: core::default::Default::default(),
            mutable: core::default::Default::default(),
            name: self.name,
            required: core::default::Default::default(),
            number_attribute_constraints: core::default::Default::default(),
            string_attribute_constraints: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_SchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_SchemaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPool_SchemaRef {
        CognitoUserPool_SchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_SchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute_data_type` after provisioning.\n"]
    pub fn attribute_data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `developer_only_attribute` after provisioning.\n"]
    pub fn developer_only_attribute(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.developer_only_attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `mutable` after provisioning.\n"]
    pub fn mutable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mutable", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `number_attribute_constraints` after provisioning.\n"]
    pub fn number_attribute_constraints(&self) -> terrars::ListRef<CognitoUserPool_Schema_NumberAttributeConstraintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.number_attribute_constraints", self.base))
    }

    #[doc = "Get a reference to the value of field `string_attribute_constraints` after provisioning.\n"]
    pub fn string_attribute_constraints(&self) -> terrars::ListRef<CognitoUserPool_Schema_StringAttributeConstraintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_attribute_constraints", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPool_SignInPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_first_auth_factors: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CognitoUserPool_SignInPolicy {
    #[doc = "Set the field `allowed_first_auth_factors`.\n"]
    pub fn set_allowed_first_auth_factors(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_first_auth_factors = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoUserPool_SignInPolicy {
    type O = terrars::BlockAssignable<CognitoUserPool_SignInPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_SignInPolicy {}

impl BuildCognitoUserPool_SignInPolicy {
    pub fn build(self) -> CognitoUserPool_SignInPolicy {
        CognitoUserPool_SignInPolicy { allowed_first_auth_factors: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_SignInPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_SignInPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPool_SignInPolicyRef {
        CognitoUserPool_SignInPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_SignInPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_first_auth_factors` after provisioning.\n"]
    pub fn allowed_first_auth_factors(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_first_auth_factors", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPool_SmsConfiguration {
    external_id: terrars::PrimField<std::string::String>,
    sns_caller_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sns_region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CognitoUserPool_SmsConfiguration {
    #[doc = "Set the field `sns_region`.\n"]
    pub fn set_sns_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sns_region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoUserPool_SmsConfiguration {
    type O = terrars::BlockAssignable<CognitoUserPool_SmsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_SmsConfiguration {
    #[doc = ""]
    pub external_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sns_caller_arn: terrars::PrimField<std::string::String>,
}

impl BuildCognitoUserPool_SmsConfiguration {
    pub fn build(self) -> CognitoUserPool_SmsConfiguration {
        CognitoUserPool_SmsConfiguration {
            external_id: self.external_id,
            sns_caller_arn: self.sns_caller_arn,
            sns_region: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_SmsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_SmsConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPool_SmsConfigurationRef {
        CognitoUserPool_SmsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_SmsConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `sns_caller_arn` after provisioning.\n"]
    pub fn sns_caller_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sns_caller_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sns_region` after provisioning.\n"]
    pub fn sns_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sns_region", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPool_SoftwareTokenMfaConfiguration {
    enabled: terrars::PrimField<bool>,
}

impl CognitoUserPool_SoftwareTokenMfaConfiguration { }

impl terrars::ToListMappable for CognitoUserPool_SoftwareTokenMfaConfiguration {
    type O = terrars::BlockAssignable<CognitoUserPool_SoftwareTokenMfaConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_SoftwareTokenMfaConfiguration {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildCognitoUserPool_SoftwareTokenMfaConfiguration {
    pub fn build(self) -> CognitoUserPool_SoftwareTokenMfaConfiguration {
        CognitoUserPool_SoftwareTokenMfaConfiguration { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_SoftwareTokenMfaConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_SoftwareTokenMfaConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoUserPool_SoftwareTokenMfaConfigurationRef {
        CognitoUserPool_SoftwareTokenMfaConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_SoftwareTokenMfaConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPool_UserAttributeUpdateSettings {
    attributes_require_verification_before_update: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl CognitoUserPool_UserAttributeUpdateSettings { }

impl terrars::ToListMappable for CognitoUserPool_UserAttributeUpdateSettings {
    type O = terrars::BlockAssignable<CognitoUserPool_UserAttributeUpdateSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_UserAttributeUpdateSettings {
    #[doc = ""]
    pub attributes_require_verification_before_update: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildCognitoUserPool_UserAttributeUpdateSettings {
    pub fn build(self) -> CognitoUserPool_UserAttributeUpdateSettings {
        CognitoUserPool_UserAttributeUpdateSettings {
            attributes_require_verification_before_update: self.attributes_require_verification_before_update,
        }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_UserAttributeUpdateSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_UserAttributeUpdateSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoUserPool_UserAttributeUpdateSettingsRef {
        CognitoUserPool_UserAttributeUpdateSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_UserAttributeUpdateSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attributes_require_verification_before_update` after provisioning.\n"]
    pub fn attributes_require_verification_before_update(
        &self,
    ) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.attributes_require_verification_before_update", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlows {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_auth_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlows {
    #[doc = "Set the field `custom_auth_mode`.\n"]
    pub fn set_custom_auth_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.custom_auth_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlows {
    type O = terrars::BlockAssignable<CognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlows>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlows {}

impl BuildCognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlows {
    pub fn build(self) -> CognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlows {
        CognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlows {
            custom_auth_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlowsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlowsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlowsRef {
        CognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlowsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlowsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_auth_mode` after provisioning.\n"]
    pub fn custom_auth_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_auth_mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoUserPool_UserPoolAddOnsDynamic {
    advanced_security_additional_flows: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlows>>,
}

#[derive(Serialize)]
pub struct CognitoUserPool_UserPoolAddOns {
    advanced_security_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_security_additional_flows: std
    ::option
    ::Option<std::vec::Vec<CognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlows>>,
    dynamic: CognitoUserPool_UserPoolAddOnsDynamic,
}

impl CognitoUserPool_UserPoolAddOns {
    #[doc = "Set the field `advanced_security_additional_flows`.\n"]
    pub fn set_advanced_security_additional_flows(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlows>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.advanced_security_additional_flows = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_security_additional_flows = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CognitoUserPool_UserPoolAddOns {
    type O = terrars::BlockAssignable<CognitoUserPool_UserPoolAddOns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_UserPoolAddOns {
    #[doc = ""]
    pub advanced_security_mode: terrars::PrimField<std::string::String>,
}

impl BuildCognitoUserPool_UserPoolAddOns {
    pub fn build(self) -> CognitoUserPool_UserPoolAddOns {
        CognitoUserPool_UserPoolAddOns {
            advanced_security_mode: self.advanced_security_mode,
            advanced_security_additional_flows: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_UserPoolAddOnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_UserPoolAddOnsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPool_UserPoolAddOnsRef {
        CognitoUserPool_UserPoolAddOnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_UserPoolAddOnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `advanced_security_mode` after provisioning.\n"]
    pub fn advanced_security_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.advanced_security_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_security_additional_flows` after provisioning.\n"]
    pub fn advanced_security_additional_flows(
        &self,
    ) -> terrars::ListRef<CognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlowsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_security_additional_flows", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPool_UsernameConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    case_sensitive: std::option::Option<terrars::PrimField<bool>>,
}

impl CognitoUserPool_UsernameConfiguration {
    #[doc = "Set the field `case_sensitive`.\n"]
    pub fn set_case_sensitive(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.case_sensitive = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoUserPool_UsernameConfiguration {
    type O = terrars::BlockAssignable<CognitoUserPool_UsernameConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_UsernameConfiguration {}

impl BuildCognitoUserPool_UsernameConfiguration {
    pub fn build(self) -> CognitoUserPool_UsernameConfiguration {
        CognitoUserPool_UsernameConfiguration { case_sensitive: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_UsernameConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_UsernameConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPool_UsernameConfigurationRef {
        CognitoUserPool_UsernameConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_UsernameConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `case_sensitive` after provisioning.\n"]
    pub fn case_sensitive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.case_sensitive", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPool_VerificationMessageTemplate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_email_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_message_by_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_subject: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_subject_by_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sms_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CognitoUserPool_VerificationMessageTemplate {
    #[doc = "Set the field `default_email_option`.\n"]
    pub fn set_default_email_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_email_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_message`.\n"]
    pub fn set_email_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email_message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_message_by_link`.\n"]
    pub fn set_email_message_by_link(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.email_message_by_link = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_subject`.\n"]
    pub fn set_email_subject(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email_subject = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_subject_by_link`.\n"]
    pub fn set_email_subject_by_link(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.email_subject_by_link = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sms_message`.\n"]
    pub fn set_sms_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sms_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoUserPool_VerificationMessageTemplate {
    type O = terrars::BlockAssignable<CognitoUserPool_VerificationMessageTemplate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_VerificationMessageTemplate {}

impl BuildCognitoUserPool_VerificationMessageTemplate {
    pub fn build(self) -> CognitoUserPool_VerificationMessageTemplate {
        CognitoUserPool_VerificationMessageTemplate {
            default_email_option: core::default::Default::default(),
            email_message: core::default::Default::default(),
            email_message_by_link: core::default::Default::default(),
            email_subject: core::default::Default::default(),
            email_subject_by_link: core::default::Default::default(),
            sms_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_VerificationMessageTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_VerificationMessageTemplateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoUserPool_VerificationMessageTemplateRef {
        CognitoUserPool_VerificationMessageTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_VerificationMessageTemplateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_email_option` after provisioning.\n"]
    pub fn default_email_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_email_option", self.base))
    }

    #[doc = "Get a reference to the value of field `email_message` after provisioning.\n"]
    pub fn email_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_message", self.base))
    }

    #[doc = "Get a reference to the value of field `email_message_by_link` after provisioning.\n"]
    pub fn email_message_by_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_message_by_link", self.base))
    }

    #[doc = "Get a reference to the value of field `email_subject` after provisioning.\n"]
    pub fn email_subject(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_subject", self.base))
    }

    #[doc = "Get a reference to the value of field `email_subject_by_link` after provisioning.\n"]
    pub fn email_subject_by_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_subject_by_link", self.base))
    }

    #[doc = "Get a reference to the value of field `sms_message` after provisioning.\n"]
    pub fn sms_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sms_message", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPool_WebAuthnConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relying_party_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_verification: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CognitoUserPool_WebAuthnConfiguration {
    #[doc = "Set the field `relying_party_id`.\n"]
    pub fn set_relying_party_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.relying_party_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_verification`.\n"]
    pub fn set_user_verification(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_verification = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoUserPool_WebAuthnConfiguration {
    type O = terrars::BlockAssignable<CognitoUserPool_WebAuthnConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPool_WebAuthnConfiguration {}

impl BuildCognitoUserPool_WebAuthnConfiguration {
    pub fn build(self) -> CognitoUserPool_WebAuthnConfiguration {
        CognitoUserPool_WebAuthnConfiguration {
            relying_party_id: core::default::Default::default(),
            user_verification: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoUserPool_WebAuthnConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPool_WebAuthnConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPool_WebAuthnConfigurationRef {
        CognitoUserPool_WebAuthnConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPool_WebAuthnConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `relying_party_id` after provisioning.\n"]
    pub fn relying_party_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.relying_party_id", self.base))
    }

    #[doc = "Get a reference to the value of field `user_verification` after provisioning.\n"]
    pub fn user_verification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_verification", self.base))
    }
}
