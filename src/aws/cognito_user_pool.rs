use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CognitoUserPoolData {
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
    sms_authentication_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sms_verification_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username_attributes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_recovery_setting: std::option::Option<std::vec::Vec<CognitoUserPoolAccountRecoverySettingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_create_user_config: std::option::Option<std::vec::Vec<CognitoUserPoolAdminCreateUserConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_configuration: std::option::Option<std::vec::Vec<CognitoUserPoolDeviceConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_configuration: std::option::Option<std::vec::Vec<CognitoUserPoolEmailConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_config: std::option::Option<std::vec::Vec<CognitoUserPoolLambdaConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_policy: std::option::Option<std::vec::Vec<CognitoUserPoolPasswordPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<std::vec::Vec<CognitoUserPoolSchemaEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sms_configuration: std::option::Option<std::vec::Vec<CognitoUserPoolSmsConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    software_token_mfa_configuration: std::option::Option<std::vec::Vec<CognitoUserPoolSoftwareTokenMfaConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_attribute_update_settings: std::option::Option<std::vec::Vec<CognitoUserPoolUserAttributeUpdateSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_pool_add_ons: std::option::Option<std::vec::Vec<CognitoUserPoolUserPoolAddOnsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username_configuration: std::option::Option<std::vec::Vec<CognitoUserPoolUsernameConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verification_message_template: std::option::Option<std::vec::Vec<CognitoUserPoolVerificationMessageTemplateEl>>,
    dynamic: CognitoUserPoolDynamic,
}

struct CognitoUserPool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CognitoUserPoolData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPoolAccountRecoverySettingEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPoolAdminCreateUserConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPoolDeviceConfigurationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPoolEmailConfigurationEl>>,
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

    #[doc = "Set the field `lambda_config`.\n"]
    pub fn set_lambda_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPoolLambdaConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPoolPasswordPolicyEl>>,
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
    pub fn set_schema(self, v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPoolSchemaEl>>) -> Self {
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

    #[doc = "Set the field `sms_configuration`.\n"]
    pub fn set_sms_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPoolSmsConfigurationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPoolSoftwareTokenMfaConfigurationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPoolUserAttributeUpdateSettingsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPoolUserPoolAddOnsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPoolUsernameConfigurationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPoolVerificationMessageTemplateEl>>,
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

    #[doc = "Get a reference to the value of field `username_attributes` after provisioning.\n"]
    pub fn username_attributes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.username_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_recovery_setting` after provisioning.\n"]
    pub fn account_recovery_setting(&self) -> terrars::ListRef<CognitoUserPoolAccountRecoverySettingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.account_recovery_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_create_user_config` after provisioning.\n"]
    pub fn admin_create_user_config(&self) -> terrars::ListRef<CognitoUserPoolAdminCreateUserConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_create_user_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device_configuration` after provisioning.\n"]
    pub fn device_configuration(&self) -> terrars::ListRef<CognitoUserPoolDeviceConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.device_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_configuration` after provisioning.\n"]
    pub fn email_configuration(&self) -> terrars::ListRef<CognitoUserPoolEmailConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_config` after provisioning.\n"]
    pub fn lambda_config(&self) -> terrars::ListRef<CognitoUserPoolLambdaConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_policy` after provisioning.\n"]
    pub fn password_policy(&self) -> terrars::ListRef<CognitoUserPoolPasswordPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.password_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_configuration` after provisioning.\n"]
    pub fn sms_configuration(&self) -> terrars::ListRef<CognitoUserPoolSmsConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sms_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `software_token_mfa_configuration` after provisioning.\n"]
    pub fn software_token_mfa_configuration(
        &self,
    ) -> terrars::ListRef<CognitoUserPoolSoftwareTokenMfaConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.software_token_mfa_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_attribute_update_settings` after provisioning.\n"]
    pub fn user_attribute_update_settings(&self) -> terrars::ListRef<CognitoUserPoolUserAttributeUpdateSettingsElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.user_attribute_update_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_pool_add_ons` after provisioning.\n"]
    pub fn user_pool_add_ons(&self) -> terrars::ListRef<CognitoUserPoolUserPoolAddOnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_pool_add_ons", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `username_configuration` after provisioning.\n"]
    pub fn username_configuration(&self) -> terrars::ListRef<CognitoUserPoolUsernameConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.username_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verification_message_template` after provisioning.\n"]
    pub fn verification_message_template(&self) -> terrars::ListRef<CognitoUserPoolVerificationMessageTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.verification_message_template", self.extract_ref()))
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
            data: std::cell::RefCell::new(CognitoUserPoolData {
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
                sms_authentication_message: core::default::Default::default(),
                sms_verification_message: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                username_attributes: core::default::Default::default(),
                account_recovery_setting: core::default::Default::default(),
                admin_create_user_config: core::default::Default::default(),
                device_configuration: core::default::Default::default(),
                email_configuration: core::default::Default::default(),
                lambda_config: core::default::Default::default(),
                password_policy: core::default::Default::default(),
                schema: core::default::Default::default(),
                sms_configuration: core::default::Default::default(),
                software_token_mfa_configuration: core::default::Default::default(),
                user_attribute_update_settings: core::default::Default::default(),
                user_pool_add_ons: core::default::Default::default(),
                username_configuration: core::default::Default::default(),
                verification_message_template: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `username_attributes` after provisioning.\n"]
    pub fn username_attributes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.username_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_recovery_setting` after provisioning.\n"]
    pub fn account_recovery_setting(&self) -> terrars::ListRef<CognitoUserPoolAccountRecoverySettingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.account_recovery_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_create_user_config` after provisioning.\n"]
    pub fn admin_create_user_config(&self) -> terrars::ListRef<CognitoUserPoolAdminCreateUserConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_create_user_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device_configuration` after provisioning.\n"]
    pub fn device_configuration(&self) -> terrars::ListRef<CognitoUserPoolDeviceConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.device_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_configuration` after provisioning.\n"]
    pub fn email_configuration(&self) -> terrars::ListRef<CognitoUserPoolEmailConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_config` after provisioning.\n"]
    pub fn lambda_config(&self) -> terrars::ListRef<CognitoUserPoolLambdaConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_policy` after provisioning.\n"]
    pub fn password_policy(&self) -> terrars::ListRef<CognitoUserPoolPasswordPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.password_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_configuration` after provisioning.\n"]
    pub fn sms_configuration(&self) -> terrars::ListRef<CognitoUserPoolSmsConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sms_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `software_token_mfa_configuration` after provisioning.\n"]
    pub fn software_token_mfa_configuration(
        &self,
    ) -> terrars::ListRef<CognitoUserPoolSoftwareTokenMfaConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.software_token_mfa_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_attribute_update_settings` after provisioning.\n"]
    pub fn user_attribute_update_settings(&self) -> terrars::ListRef<CognitoUserPoolUserAttributeUpdateSettingsElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.user_attribute_update_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_pool_add_ons` after provisioning.\n"]
    pub fn user_pool_add_ons(&self) -> terrars::ListRef<CognitoUserPoolUserPoolAddOnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_pool_add_ons", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `username_configuration` after provisioning.\n"]
    pub fn username_configuration(&self) -> terrars::ListRef<CognitoUserPoolUsernameConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.username_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verification_message_template` after provisioning.\n"]
    pub fn verification_message_template(&self) -> terrars::ListRef<CognitoUserPoolVerificationMessageTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.verification_message_template", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl {
    name: terrars::PrimField<std::string::String>,
    priority: terrars::PrimField<f64>,
}

impl CognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl { }

impl terrars::ToListMappable for CognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl {
    type O = terrars::BlockAssignable<CognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub priority: terrars::PrimField<f64>,
}

impl BuildCognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl {
    pub fn build(self) -> CognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl {
        CognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl {
            name: self.name,
            priority: self.priority,
        }
    }
}

pub struct CognitoUserPoolAccountRecoverySettingElRecoveryMechanismElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolAccountRecoverySettingElRecoveryMechanismElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoUserPoolAccountRecoverySettingElRecoveryMechanismElRef {
        CognitoUserPoolAccountRecoverySettingElRecoveryMechanismElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolAccountRecoverySettingElRecoveryMechanismElRef {
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
struct CognitoUserPoolAccountRecoverySettingElDynamic {
    recovery_mechanism: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl>>,
}

#[derive(Serialize)]
pub struct CognitoUserPoolAccountRecoverySettingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recovery_mechanism: std::option::Option<std::vec::Vec<CognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl>>,
    dynamic: CognitoUserPoolAccountRecoverySettingElDynamic,
}

impl CognitoUserPoolAccountRecoverySettingEl {
    #[doc = "Set the field `recovery_mechanism`.\n"]
    pub fn set_recovery_mechanism(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CognitoUserPoolAccountRecoverySettingElRecoveryMechanismEl>>,
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

impl terrars::ToListMappable for CognitoUserPoolAccountRecoverySettingEl {
    type O = terrars::BlockAssignable<CognitoUserPoolAccountRecoverySettingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolAccountRecoverySettingEl {}

impl BuildCognitoUserPoolAccountRecoverySettingEl {
    pub fn build(self) -> CognitoUserPoolAccountRecoverySettingEl {
        CognitoUserPoolAccountRecoverySettingEl {
            recovery_mechanism: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolAccountRecoverySettingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolAccountRecoverySettingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPoolAccountRecoverySettingElRef {
        CognitoUserPoolAccountRecoverySettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolAccountRecoverySettingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_subject: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sms_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl {
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

impl terrars::ToListMappable for CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl {
    type O = terrars::BlockAssignable<CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl {}

impl BuildCognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl {
    pub fn build(self) -> CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl {
        CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl {
            email_message: core::default::Default::default(),
            email_subject: core::default::Default::default(),
            sms_message: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateElRef {
        CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateElRef {
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
struct CognitoUserPoolAdminCreateUserConfigElDynamic {
    invite_message_template: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl>>,
}

#[derive(Serialize)]
pub struct CognitoUserPoolAdminCreateUserConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_admin_create_user_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invite_message_template: std
    ::option
    ::Option<std::vec::Vec<CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl>>,
    dynamic: CognitoUserPoolAdminCreateUserConfigElDynamic,
}

impl CognitoUserPoolAdminCreateUserConfigEl {
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
                    ::Into<terrars::BlockAssignable<CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateEl>>,
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

impl terrars::ToListMappable for CognitoUserPoolAdminCreateUserConfigEl {
    type O = terrars::BlockAssignable<CognitoUserPoolAdminCreateUserConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolAdminCreateUserConfigEl {}

impl BuildCognitoUserPoolAdminCreateUserConfigEl {
    pub fn build(self) -> CognitoUserPoolAdminCreateUserConfigEl {
        CognitoUserPoolAdminCreateUserConfigEl {
            allow_admin_create_user_only: core::default::Default::default(),
            invite_message_template: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolAdminCreateUserConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolAdminCreateUserConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPoolAdminCreateUserConfigElRef {
        CognitoUserPoolAdminCreateUserConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolAdminCreateUserConfigElRef {
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
    ) -> terrars::ListRef<CognitoUserPoolAdminCreateUserConfigElInviteMessageTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.invite_message_template", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolDeviceConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    challenge_required_on_new_device: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_only_remembered_on_user_prompt: std::option::Option<terrars::PrimField<bool>>,
}

impl CognitoUserPoolDeviceConfigurationEl {
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

impl terrars::ToListMappable for CognitoUserPoolDeviceConfigurationEl {
    type O = terrars::BlockAssignable<CognitoUserPoolDeviceConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolDeviceConfigurationEl {}

impl BuildCognitoUserPoolDeviceConfigurationEl {
    pub fn build(self) -> CognitoUserPoolDeviceConfigurationEl {
        CognitoUserPoolDeviceConfigurationEl {
            challenge_required_on_new_device: core::default::Default::default(),
            device_only_remembered_on_user_prompt: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolDeviceConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolDeviceConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPoolDeviceConfigurationElRef {
        CognitoUserPoolDeviceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolDeviceConfigurationElRef {
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
pub struct CognitoUserPoolEmailConfigurationEl {
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

impl CognitoUserPoolEmailConfigurationEl {
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

impl terrars::ToListMappable for CognitoUserPoolEmailConfigurationEl {
    type O = terrars::BlockAssignable<CognitoUserPoolEmailConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolEmailConfigurationEl {}

impl BuildCognitoUserPoolEmailConfigurationEl {
    pub fn build(self) -> CognitoUserPoolEmailConfigurationEl {
        CognitoUserPoolEmailConfigurationEl {
            configuration_set: core::default::Default::default(),
            email_sending_account: core::default::Default::default(),
            from_email_address: core::default::Default::default(),
            reply_to_email_address: core::default::Default::default(),
            source_arn: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolEmailConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolEmailConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPoolEmailConfigurationElRef {
        CognitoUserPoolEmailConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolEmailConfigurationElRef {
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
pub struct CognitoUserPoolLambdaConfigElCustomEmailSenderEl {
    lambda_arn: terrars::PrimField<std::string::String>,
    lambda_version: terrars::PrimField<std::string::String>,
}

impl CognitoUserPoolLambdaConfigElCustomEmailSenderEl { }

impl terrars::ToListMappable for CognitoUserPoolLambdaConfigElCustomEmailSenderEl {
    type O = terrars::BlockAssignable<CognitoUserPoolLambdaConfigElCustomEmailSenderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolLambdaConfigElCustomEmailSenderEl {
    #[doc = ""]
    pub lambda_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub lambda_version: terrars::PrimField<std::string::String>,
}

impl BuildCognitoUserPoolLambdaConfigElCustomEmailSenderEl {
    pub fn build(self) -> CognitoUserPoolLambdaConfigElCustomEmailSenderEl {
        CognitoUserPoolLambdaConfigElCustomEmailSenderEl {
            lambda_arn: self.lambda_arn,
            lambda_version: self.lambda_version,
        }
    }
}

pub struct CognitoUserPoolLambdaConfigElCustomEmailSenderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolLambdaConfigElCustomEmailSenderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoUserPoolLambdaConfigElCustomEmailSenderElRef {
        CognitoUserPoolLambdaConfigElCustomEmailSenderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolLambdaConfigElCustomEmailSenderElRef {
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
pub struct CognitoUserPoolLambdaConfigElCustomSmsSenderEl {
    lambda_arn: terrars::PrimField<std::string::String>,
    lambda_version: terrars::PrimField<std::string::String>,
}

impl CognitoUserPoolLambdaConfigElCustomSmsSenderEl { }

impl terrars::ToListMappable for CognitoUserPoolLambdaConfigElCustomSmsSenderEl {
    type O = terrars::BlockAssignable<CognitoUserPoolLambdaConfigElCustomSmsSenderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolLambdaConfigElCustomSmsSenderEl {
    #[doc = ""]
    pub lambda_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub lambda_version: terrars::PrimField<std::string::String>,
}

impl BuildCognitoUserPoolLambdaConfigElCustomSmsSenderEl {
    pub fn build(self) -> CognitoUserPoolLambdaConfigElCustomSmsSenderEl {
        CognitoUserPoolLambdaConfigElCustomSmsSenderEl {
            lambda_arn: self.lambda_arn,
            lambda_version: self.lambda_version,
        }
    }
}

pub struct CognitoUserPoolLambdaConfigElCustomSmsSenderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolLambdaConfigElCustomSmsSenderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoUserPoolLambdaConfigElCustomSmsSenderElRef {
        CognitoUserPoolLambdaConfigElCustomSmsSenderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolLambdaConfigElCustomSmsSenderElRef {
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
struct CognitoUserPoolLambdaConfigElDynamic {
    custom_email_sender: std::option::Option<terrars::DynamicBlock<CognitoUserPoolLambdaConfigElCustomEmailSenderEl>>,
    custom_sms_sender: std::option::Option<terrars::DynamicBlock<CognitoUserPoolLambdaConfigElCustomSmsSenderEl>>,
}

#[derive(Serialize)]
pub struct CognitoUserPoolLambdaConfigEl {
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
    custom_email_sender: std::option::Option<std::vec::Vec<CognitoUserPoolLambdaConfigElCustomEmailSenderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_sms_sender: std::option::Option<std::vec::Vec<CognitoUserPoolLambdaConfigElCustomSmsSenderEl>>,
    dynamic: CognitoUserPoolLambdaConfigElDynamic,
}

impl CognitoUserPoolLambdaConfigEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPoolLambdaConfigElCustomEmailSenderEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPoolLambdaConfigElCustomSmsSenderEl>>,
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
}

impl terrars::ToListMappable for CognitoUserPoolLambdaConfigEl {
    type O = terrars::BlockAssignable<CognitoUserPoolLambdaConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolLambdaConfigEl {}

impl BuildCognitoUserPoolLambdaConfigEl {
    pub fn build(self) -> CognitoUserPoolLambdaConfigEl {
        CognitoUserPoolLambdaConfigEl {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolLambdaConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolLambdaConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPoolLambdaConfigElRef {
        CognitoUserPoolLambdaConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolLambdaConfigElRef {
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
    pub fn custom_email_sender(&self) -> terrars::ListRef<CognitoUserPoolLambdaConfigElCustomEmailSenderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_email_sender", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_sms_sender` after provisioning.\n"]
    pub fn custom_sms_sender(&self) -> terrars::ListRef<CognitoUserPoolLambdaConfigElCustomSmsSenderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_sms_sender", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolPasswordPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_length: std::option::Option<terrars::PrimField<f64>>,
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

impl CognitoUserPoolPasswordPolicyEl {
    #[doc = "Set the field `minimum_length`.\n"]
    pub fn set_minimum_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minimum_length = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for CognitoUserPoolPasswordPolicyEl {
    type O = terrars::BlockAssignable<CognitoUserPoolPasswordPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolPasswordPolicyEl {}

impl BuildCognitoUserPoolPasswordPolicyEl {
    pub fn build(self) -> CognitoUserPoolPasswordPolicyEl {
        CognitoUserPoolPasswordPolicyEl {
            minimum_length: core::default::Default::default(),
            require_lowercase: core::default::Default::default(),
            require_numbers: core::default::Default::default(),
            require_symbols: core::default::Default::default(),
            require_uppercase: core::default::Default::default(),
            temporary_password_validity_days: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolPasswordPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolPasswordPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPoolPasswordPolicyElRef {
        CognitoUserPoolPasswordPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolPasswordPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `minimum_length` after provisioning.\n"]
    pub fn minimum_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_length", self.base))
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
pub struct CognitoUserPoolSchemaElNumberAttributeConstraintsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CognitoUserPoolSchemaElNumberAttributeConstraintsEl {
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

impl terrars::ToListMappable for CognitoUserPoolSchemaElNumberAttributeConstraintsEl {
    type O = terrars::BlockAssignable<CognitoUserPoolSchemaElNumberAttributeConstraintsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolSchemaElNumberAttributeConstraintsEl {}

impl BuildCognitoUserPoolSchemaElNumberAttributeConstraintsEl {
    pub fn build(self) -> CognitoUserPoolSchemaElNumberAttributeConstraintsEl {
        CognitoUserPoolSchemaElNumberAttributeConstraintsEl {
            max_value: core::default::Default::default(),
            min_value: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolSchemaElNumberAttributeConstraintsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolSchemaElNumberAttributeConstraintsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoUserPoolSchemaElNumberAttributeConstraintsElRef {
        CognitoUserPoolSchemaElNumberAttributeConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolSchemaElNumberAttributeConstraintsElRef {
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
pub struct CognitoUserPoolSchemaElStringAttributeConstraintsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_length: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_length: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CognitoUserPoolSchemaElStringAttributeConstraintsEl {
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

impl terrars::ToListMappable for CognitoUserPoolSchemaElStringAttributeConstraintsEl {
    type O = terrars::BlockAssignable<CognitoUserPoolSchemaElStringAttributeConstraintsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolSchemaElStringAttributeConstraintsEl {}

impl BuildCognitoUserPoolSchemaElStringAttributeConstraintsEl {
    pub fn build(self) -> CognitoUserPoolSchemaElStringAttributeConstraintsEl {
        CognitoUserPoolSchemaElStringAttributeConstraintsEl {
            max_length: core::default::Default::default(),
            min_length: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolSchemaElStringAttributeConstraintsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolSchemaElStringAttributeConstraintsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoUserPoolSchemaElStringAttributeConstraintsElRef {
        CognitoUserPoolSchemaElStringAttributeConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolSchemaElStringAttributeConstraintsElRef {
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
struct CognitoUserPoolSchemaElDynamic {
    number_attribute_constraints: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoUserPoolSchemaElNumberAttributeConstraintsEl>>,
    string_attribute_constraints: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoUserPoolSchemaElStringAttributeConstraintsEl>>,
}

#[derive(Serialize)]
pub struct CognitoUserPoolSchemaEl {
    attribute_data_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    developer_only_attribute: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mutable: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_attribute_constraints: std::option::Option<std::vec::Vec<CognitoUserPoolSchemaElNumberAttributeConstraintsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_attribute_constraints: std::option::Option<std::vec::Vec<CognitoUserPoolSchemaElStringAttributeConstraintsEl>>,
    dynamic: CognitoUserPoolSchemaElDynamic,
}

impl CognitoUserPoolSchemaEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPoolSchemaElNumberAttributeConstraintsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPoolSchemaElStringAttributeConstraintsEl>>,
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

impl terrars::ToListMappable for CognitoUserPoolSchemaEl {
    type O = terrars::BlockAssignable<CognitoUserPoolSchemaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolSchemaEl {
    #[doc = ""]
    pub attribute_data_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCognitoUserPoolSchemaEl {
    pub fn build(self) -> CognitoUserPoolSchemaEl {
        CognitoUserPoolSchemaEl {
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

pub struct CognitoUserPoolSchemaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolSchemaElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPoolSchemaElRef {
        CognitoUserPoolSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolSchemaElRef {
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
    pub fn number_attribute_constraints(
        &self,
    ) -> terrars::ListRef<CognitoUserPoolSchemaElNumberAttributeConstraintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.number_attribute_constraints", self.base))
    }

    #[doc = "Get a reference to the value of field `string_attribute_constraints` after provisioning.\n"]
    pub fn string_attribute_constraints(
        &self,
    ) -> terrars::ListRef<CognitoUserPoolSchemaElStringAttributeConstraintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_attribute_constraints", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolSmsConfigurationEl {
    external_id: terrars::PrimField<std::string::String>,
    sns_caller_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sns_region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CognitoUserPoolSmsConfigurationEl {
    #[doc = "Set the field `sns_region`.\n"]
    pub fn set_sns_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sns_region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoUserPoolSmsConfigurationEl {
    type O = terrars::BlockAssignable<CognitoUserPoolSmsConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolSmsConfigurationEl {
    #[doc = ""]
    pub external_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sns_caller_arn: terrars::PrimField<std::string::String>,
}

impl BuildCognitoUserPoolSmsConfigurationEl {
    pub fn build(self) -> CognitoUserPoolSmsConfigurationEl {
        CognitoUserPoolSmsConfigurationEl {
            external_id: self.external_id,
            sns_caller_arn: self.sns_caller_arn,
            sns_region: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolSmsConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolSmsConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPoolSmsConfigurationElRef {
        CognitoUserPoolSmsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolSmsConfigurationElRef {
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
pub struct CognitoUserPoolSoftwareTokenMfaConfigurationEl {
    enabled: terrars::PrimField<bool>,
}

impl CognitoUserPoolSoftwareTokenMfaConfigurationEl { }

impl terrars::ToListMappable for CognitoUserPoolSoftwareTokenMfaConfigurationEl {
    type O = terrars::BlockAssignable<CognitoUserPoolSoftwareTokenMfaConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolSoftwareTokenMfaConfigurationEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildCognitoUserPoolSoftwareTokenMfaConfigurationEl {
    pub fn build(self) -> CognitoUserPoolSoftwareTokenMfaConfigurationEl {
        CognitoUserPoolSoftwareTokenMfaConfigurationEl { enabled: self.enabled }
    }
}

pub struct CognitoUserPoolSoftwareTokenMfaConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolSoftwareTokenMfaConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoUserPoolSoftwareTokenMfaConfigurationElRef {
        CognitoUserPoolSoftwareTokenMfaConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolSoftwareTokenMfaConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolUserAttributeUpdateSettingsEl {
    attributes_require_verification_before_update: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl CognitoUserPoolUserAttributeUpdateSettingsEl { }

impl terrars::ToListMappable for CognitoUserPoolUserAttributeUpdateSettingsEl {
    type O = terrars::BlockAssignable<CognitoUserPoolUserAttributeUpdateSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolUserAttributeUpdateSettingsEl {
    #[doc = ""]
    pub attributes_require_verification_before_update: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildCognitoUserPoolUserAttributeUpdateSettingsEl {
    pub fn build(self) -> CognitoUserPoolUserAttributeUpdateSettingsEl {
        CognitoUserPoolUserAttributeUpdateSettingsEl {
            attributes_require_verification_before_update: self.attributes_require_verification_before_update,
        }
    }
}

pub struct CognitoUserPoolUserAttributeUpdateSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolUserAttributeUpdateSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoUserPoolUserAttributeUpdateSettingsElRef {
        CognitoUserPoolUserAttributeUpdateSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolUserAttributeUpdateSettingsElRef {
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
pub struct CognitoUserPoolUserPoolAddOnsEl {
    advanced_security_mode: terrars::PrimField<std::string::String>,
}

impl CognitoUserPoolUserPoolAddOnsEl { }

impl terrars::ToListMappable for CognitoUserPoolUserPoolAddOnsEl {
    type O = terrars::BlockAssignable<CognitoUserPoolUserPoolAddOnsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolUserPoolAddOnsEl {
    #[doc = ""]
    pub advanced_security_mode: terrars::PrimField<std::string::String>,
}

impl BuildCognitoUserPoolUserPoolAddOnsEl {
    pub fn build(self) -> CognitoUserPoolUserPoolAddOnsEl {
        CognitoUserPoolUserPoolAddOnsEl { advanced_security_mode: self.advanced_security_mode }
    }
}

pub struct CognitoUserPoolUserPoolAddOnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolUserPoolAddOnsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPoolUserPoolAddOnsElRef {
        CognitoUserPoolUserPoolAddOnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolUserPoolAddOnsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `advanced_security_mode` after provisioning.\n"]
    pub fn advanced_security_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.advanced_security_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolUsernameConfigurationEl {
    case_sensitive: terrars::PrimField<bool>,
}

impl CognitoUserPoolUsernameConfigurationEl { }

impl terrars::ToListMappable for CognitoUserPoolUsernameConfigurationEl {
    type O = terrars::BlockAssignable<CognitoUserPoolUsernameConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolUsernameConfigurationEl {
    #[doc = ""]
    pub case_sensitive: terrars::PrimField<bool>,
}

impl BuildCognitoUserPoolUsernameConfigurationEl {
    pub fn build(self) -> CognitoUserPoolUsernameConfigurationEl {
        CognitoUserPoolUsernameConfigurationEl { case_sensitive: self.case_sensitive }
    }
}

pub struct CognitoUserPoolUsernameConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolUsernameConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPoolUsernameConfigurationElRef {
        CognitoUserPoolUsernameConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolUsernameConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `case_sensitive` after provisioning.\n"]
    pub fn case_sensitive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.case_sensitive", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolVerificationMessageTemplateEl {
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

impl CognitoUserPoolVerificationMessageTemplateEl {
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

impl terrars::ToListMappable for CognitoUserPoolVerificationMessageTemplateEl {
    type O = terrars::BlockAssignable<CognitoUserPoolVerificationMessageTemplateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolVerificationMessageTemplateEl {}

impl BuildCognitoUserPoolVerificationMessageTemplateEl {
    pub fn build(self) -> CognitoUserPoolVerificationMessageTemplateEl {
        CognitoUserPoolVerificationMessageTemplateEl {
            default_email_option: core::default::Default::default(),
            email_message: core::default::Default::default(),
            email_message_by_link: core::default::Default::default(),
            email_subject: core::default::Default::default(),
            email_subject_by_link: core::default::Default::default(),
            sms_message: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolVerificationMessageTemplateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolVerificationMessageTemplateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoUserPoolVerificationMessageTemplateElRef {
        CognitoUserPoolVerificationMessageTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolVerificationMessageTemplateElRef {
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

#[derive(Serialize, Default)]
struct CognitoUserPoolDynamic {
    account_recovery_setting: std::option::Option<terrars::DynamicBlock<CognitoUserPoolAccountRecoverySettingEl>>,
    admin_create_user_config: std::option::Option<terrars::DynamicBlock<CognitoUserPoolAdminCreateUserConfigEl>>,
    device_configuration: std::option::Option<terrars::DynamicBlock<CognitoUserPoolDeviceConfigurationEl>>,
    email_configuration: std::option::Option<terrars::DynamicBlock<CognitoUserPoolEmailConfigurationEl>>,
    lambda_config: std::option::Option<terrars::DynamicBlock<CognitoUserPoolLambdaConfigEl>>,
    password_policy: std::option::Option<terrars::DynamicBlock<CognitoUserPoolPasswordPolicyEl>>,
    schema: std::option::Option<terrars::DynamicBlock<CognitoUserPoolSchemaEl>>,
    sms_configuration: std::option::Option<terrars::DynamicBlock<CognitoUserPoolSmsConfigurationEl>>,
    software_token_mfa_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoUserPoolSoftwareTokenMfaConfigurationEl>>,
    user_attribute_update_settings: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoUserPoolUserAttributeUpdateSettingsEl>>,
    user_pool_add_ons: std::option::Option<terrars::DynamicBlock<CognitoUserPoolUserPoolAddOnsEl>>,
    username_configuration: std::option::Option<terrars::DynamicBlock<CognitoUserPoolUsernameConfigurationEl>>,
    verification_message_template: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoUserPoolVerificationMessageTemplateEl>>,
}
