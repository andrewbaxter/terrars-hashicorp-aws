#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCognitoUserPoolData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    user_pool_id: terrars::PrimField<std::string::String>,
}

struct DataCognitoUserPool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCognitoUserPoolData_>,
}

#[derive(Clone)]
pub struct DataCognitoUserPool(std::rc::Rc<DataCognitoUserPool_>);

impl DataCognitoUserPool {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_recovery_setting` after provisioning.\n"]
    pub fn account_recovery_setting(&self) -> terrars::ListRef<DataCognitoUserPool_AccountRecoverySettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.account_recovery_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_create_user_config` after provisioning.\n"]
    pub fn admin_create_user_config(&self) -> terrars::ListRef<DataCognitoUserPool_AdminCreateUserConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_create_user_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_verified_attributes` after provisioning.\n"]
    pub fn auto_verified_attributes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_verified_attributes", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `device_configuration` after provisioning.\n"]
    pub fn device_configuration(&self) -> terrars::ListRef<DataCognitoUserPool_DeviceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.device_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_configuration` after provisioning.\n"]
    pub fn email_configuration(&self) -> terrars::ListRef<DataCognitoUserPool_EmailConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `estimated_number_of_users` after provisioning.\n"]
    pub fn estimated_number_of_users(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.estimated_number_of_users", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_config` after provisioning.\n"]
    pub fn lambda_config(&self) -> terrars::ListRef<DataCognitoUserPool_LambdaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_config", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `schema_attributes` after provisioning.\n"]
    pub fn schema_attributes(&self) -> terrars::ListRef<DataCognitoUserPool_SchemaAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_authentication_message` after provisioning.\n"]
    pub fn sms_authentication_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sms_authentication_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_configuration_failure` after provisioning.\n"]
    pub fn sms_configuration_failure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sms_configuration_failure", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_verification_message` after provisioning.\n"]
    pub fn sms_verification_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sms_verification_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_add_ons` after provisioning.\n"]
    pub fn user_pool_add_ons(&self) -> terrars::ListRef<DataCognitoUserPool_UserPoolAddOnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_pool_add_ons", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_tags` after provisioning.\n"]
    pub fn user_pool_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.user_pool_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `username_attributes` after provisioning.\n"]
    pub fn username_attributes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.username_attributes", self.extract_ref()))
    }
}

impl terrars::Referable for DataCognitoUserPool {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCognitoUserPool { }

impl terrars::ToListMappable for DataCognitoUserPool {
    type O = terrars::ListRef<DataCognitoUserPoolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCognitoUserPool_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_cognito_user_pool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCognitoUserPool {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub user_pool_id: terrars::PrimField<std::string::String>,
}

impl BuildDataCognitoUserPool {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCognitoUserPool {
        let out = DataCognitoUserPool(std::rc::Rc::new(DataCognitoUserPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCognitoUserPoolData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                region: core::default::Default::default(),
                user_pool_id: self.user_pool_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCognitoUserPoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCognitoUserPoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCognitoUserPoolRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_recovery_setting` after provisioning.\n"]
    pub fn account_recovery_setting(&self) -> terrars::ListRef<DataCognitoUserPool_AccountRecoverySettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.account_recovery_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_create_user_config` after provisioning.\n"]
    pub fn admin_create_user_config(&self) -> terrars::ListRef<DataCognitoUserPool_AdminCreateUserConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_create_user_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_verified_attributes` after provisioning.\n"]
    pub fn auto_verified_attributes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_verified_attributes", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `device_configuration` after provisioning.\n"]
    pub fn device_configuration(&self) -> terrars::ListRef<DataCognitoUserPool_DeviceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.device_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_configuration` after provisioning.\n"]
    pub fn email_configuration(&self) -> terrars::ListRef<DataCognitoUserPool_EmailConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `estimated_number_of_users` after provisioning.\n"]
    pub fn estimated_number_of_users(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.estimated_number_of_users", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_config` after provisioning.\n"]
    pub fn lambda_config(&self) -> terrars::ListRef<DataCognitoUserPool_LambdaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_config", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `schema_attributes` after provisioning.\n"]
    pub fn schema_attributes(&self) -> terrars::ListRef<DataCognitoUserPool_SchemaAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_authentication_message` after provisioning.\n"]
    pub fn sms_authentication_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sms_authentication_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_configuration_failure` after provisioning.\n"]
    pub fn sms_configuration_failure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sms_configuration_failure", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_verification_message` after provisioning.\n"]
    pub fn sms_verification_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sms_verification_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_add_ons` after provisioning.\n"]
    pub fn user_pool_add_ons(&self) -> terrars::ListRef<DataCognitoUserPool_UserPoolAddOnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_pool_add_ons", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_tags` after provisioning.\n"]
    pub fn user_pool_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.user_pool_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `username_attributes` after provisioning.\n"]
    pub fn username_attributes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.username_attributes", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPool_AccountRecoverySetting_RecoveryMechanism {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
}

impl DataCognitoUserPool_AccountRecoverySetting_RecoveryMechanism {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCognitoUserPool_AccountRecoverySetting_RecoveryMechanism {
    type O = terrars::BlockAssignable<DataCognitoUserPool_AccountRecoverySetting_RecoveryMechanism>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPool_AccountRecoverySetting_RecoveryMechanism {}

impl BuildDataCognitoUserPool_AccountRecoverySetting_RecoveryMechanism {
    pub fn build(self) -> DataCognitoUserPool_AccountRecoverySetting_RecoveryMechanism {
        DataCognitoUserPool_AccountRecoverySetting_RecoveryMechanism {
            name: core::default::Default::default(),
            priority: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCognitoUserPool_AccountRecoverySetting_RecoveryMechanismRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCognitoUserPool_AccountRecoverySetting_RecoveryMechanismRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCognitoUserPool_AccountRecoverySetting_RecoveryMechanismRef {
        DataCognitoUserPool_AccountRecoverySetting_RecoveryMechanismRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPool_AccountRecoverySetting_RecoveryMechanismRef {
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

#[derive(Serialize)]
pub struct DataCognitoUserPool_AccountRecoverySetting {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recovery_mechanism: std
    ::option
    ::Option<terrars::ListField<DataCognitoUserPool_AccountRecoverySetting_RecoveryMechanism>>,
}

impl DataCognitoUserPool_AccountRecoverySetting {
    #[doc = "Set the field `recovery_mechanism`.\n"]
    pub fn set_recovery_mechanism(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCognitoUserPool_AccountRecoverySetting_RecoveryMechanism>>,
    ) -> Self {
        self.recovery_mechanism = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCognitoUserPool_AccountRecoverySetting {
    type O = terrars::BlockAssignable<DataCognitoUserPool_AccountRecoverySetting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPool_AccountRecoverySetting {}

impl BuildDataCognitoUserPool_AccountRecoverySetting {
    pub fn build(self) -> DataCognitoUserPool_AccountRecoverySetting {
        DataCognitoUserPool_AccountRecoverySetting { recovery_mechanism: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataCognitoUserPool_AccountRecoverySettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCognitoUserPool_AccountRecoverySettingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCognitoUserPool_AccountRecoverySettingRef {
        DataCognitoUserPool_AccountRecoverySettingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPool_AccountRecoverySettingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `recovery_mechanism` after provisioning.\n"]
    pub fn recovery_mechanism(
        &self,
    ) -> terrars::ListRef<DataCognitoUserPool_AccountRecoverySetting_RecoveryMechanismRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recovery_mechanism", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPool_AdminCreateUserConfig_InviteMessageTemplate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_subject: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sms_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCognitoUserPool_AdminCreateUserConfig_InviteMessageTemplate {
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

impl terrars::ToListMappable for DataCognitoUserPool_AdminCreateUserConfig_InviteMessageTemplate {
    type O = terrars::BlockAssignable<DataCognitoUserPool_AdminCreateUserConfig_InviteMessageTemplate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPool_AdminCreateUserConfig_InviteMessageTemplate {}

impl BuildDataCognitoUserPool_AdminCreateUserConfig_InviteMessageTemplate {
    pub fn build(self) -> DataCognitoUserPool_AdminCreateUserConfig_InviteMessageTemplate {
        DataCognitoUserPool_AdminCreateUserConfig_InviteMessageTemplate {
            email_message: core::default::Default::default(),
            email_subject: core::default::Default::default(),
            sms_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCognitoUserPool_AdminCreateUserConfig_InviteMessageTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCognitoUserPool_AdminCreateUserConfig_InviteMessageTemplateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCognitoUserPool_AdminCreateUserConfig_InviteMessageTemplateRef {
        DataCognitoUserPool_AdminCreateUserConfig_InviteMessageTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPool_AdminCreateUserConfig_InviteMessageTemplateRef {
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

#[derive(Serialize)]
pub struct DataCognitoUserPool_AdminCreateUserConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_admin_create_user_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invite_message_template: std
    ::option
    ::Option<terrars::ListField<DataCognitoUserPool_AdminCreateUserConfig_InviteMessageTemplate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unused_account_validity_days: std::option::Option<terrars::PrimField<f64>>,
}

impl DataCognitoUserPool_AdminCreateUserConfig {
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
                    ::Into<terrars::ListField<DataCognitoUserPool_AdminCreateUserConfig_InviteMessageTemplate>>,
    ) -> Self {
        self.invite_message_template = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unused_account_validity_days`.\n"]
    pub fn set_unused_account_validity_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.unused_account_validity_days = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCognitoUserPool_AdminCreateUserConfig {
    type O = terrars::BlockAssignable<DataCognitoUserPool_AdminCreateUserConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPool_AdminCreateUserConfig {}

impl BuildDataCognitoUserPool_AdminCreateUserConfig {
    pub fn build(self) -> DataCognitoUserPool_AdminCreateUserConfig {
        DataCognitoUserPool_AdminCreateUserConfig {
            allow_admin_create_user_only: core::default::Default::default(),
            invite_message_template: core::default::Default::default(),
            unused_account_validity_days: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCognitoUserPool_AdminCreateUserConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCognitoUserPool_AdminCreateUserConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCognitoUserPool_AdminCreateUserConfigRef {
        DataCognitoUserPool_AdminCreateUserConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPool_AdminCreateUserConfigRef {
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
    ) -> terrars::ListRef<DataCognitoUserPool_AdminCreateUserConfig_InviteMessageTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.invite_message_template", self.base))
    }

    #[doc = "Get a reference to the value of field `unused_account_validity_days` after provisioning.\n"]
    pub fn unused_account_validity_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unused_account_validity_days", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPool_DeviceConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    challenge_required_on_new_device: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_only_remembered_on_user_prompt: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCognitoUserPool_DeviceConfiguration {
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

impl terrars::ToListMappable for DataCognitoUserPool_DeviceConfiguration {
    type O = terrars::BlockAssignable<DataCognitoUserPool_DeviceConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPool_DeviceConfiguration {}

impl BuildDataCognitoUserPool_DeviceConfiguration {
    pub fn build(self) -> DataCognitoUserPool_DeviceConfiguration {
        DataCognitoUserPool_DeviceConfiguration {
            challenge_required_on_new_device: core::default::Default::default(),
            device_only_remembered_on_user_prompt: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCognitoUserPool_DeviceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCognitoUserPool_DeviceConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCognitoUserPool_DeviceConfigurationRef {
        DataCognitoUserPool_DeviceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPool_DeviceConfigurationRef {
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
pub struct DataCognitoUserPool_EmailConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration_set: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_sending_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reply_to_email_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCognitoUserPool_EmailConfiguration {
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

    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.from = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for DataCognitoUserPool_EmailConfiguration {
    type O = terrars::BlockAssignable<DataCognitoUserPool_EmailConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPool_EmailConfiguration {}

impl BuildDataCognitoUserPool_EmailConfiguration {
    pub fn build(self) -> DataCognitoUserPool_EmailConfiguration {
        DataCognitoUserPool_EmailConfiguration {
            configuration_set: core::default::Default::default(),
            email_sending_account: core::default::Default::default(),
            from: core::default::Default::default(),
            reply_to_email_address: core::default::Default::default(),
            source_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCognitoUserPool_EmailConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCognitoUserPool_EmailConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCognitoUserPool_EmailConfigurationRef {
        DataCognitoUserPool_EmailConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPool_EmailConfigurationRef {
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

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
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
pub struct DataCognitoUserPool_LambdaConfig_CustomEmailSender {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCognitoUserPool_LambdaConfig_CustomEmailSender {
    #[doc = "Set the field `lambda_arn`.\n"]
    pub fn set_lambda_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lambda_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lambda_version`.\n"]
    pub fn set_lambda_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lambda_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCognitoUserPool_LambdaConfig_CustomEmailSender {
    type O = terrars::BlockAssignable<DataCognitoUserPool_LambdaConfig_CustomEmailSender>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPool_LambdaConfig_CustomEmailSender {}

impl BuildDataCognitoUserPool_LambdaConfig_CustomEmailSender {
    pub fn build(self) -> DataCognitoUserPool_LambdaConfig_CustomEmailSender {
        DataCognitoUserPool_LambdaConfig_CustomEmailSender {
            lambda_arn: core::default::Default::default(),
            lambda_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCognitoUserPool_LambdaConfig_CustomEmailSenderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCognitoUserPool_LambdaConfig_CustomEmailSenderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCognitoUserPool_LambdaConfig_CustomEmailSenderRef {
        DataCognitoUserPool_LambdaConfig_CustomEmailSenderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPool_LambdaConfig_CustomEmailSenderRef {
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
pub struct DataCognitoUserPool_LambdaConfig_CustomSmsSender {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCognitoUserPool_LambdaConfig_CustomSmsSender {
    #[doc = "Set the field `lambda_arn`.\n"]
    pub fn set_lambda_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lambda_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lambda_version`.\n"]
    pub fn set_lambda_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lambda_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCognitoUserPool_LambdaConfig_CustomSmsSender {
    type O = terrars::BlockAssignable<DataCognitoUserPool_LambdaConfig_CustomSmsSender>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPool_LambdaConfig_CustomSmsSender {}

impl BuildDataCognitoUserPool_LambdaConfig_CustomSmsSender {
    pub fn build(self) -> DataCognitoUserPool_LambdaConfig_CustomSmsSender {
        DataCognitoUserPool_LambdaConfig_CustomSmsSender {
            lambda_arn: core::default::Default::default(),
            lambda_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCognitoUserPool_LambdaConfig_CustomSmsSenderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCognitoUserPool_LambdaConfig_CustomSmsSenderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCognitoUserPool_LambdaConfig_CustomSmsSenderRef {
        DataCognitoUserPool_LambdaConfig_CustomSmsSenderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPool_LambdaConfig_CustomSmsSenderRef {
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
pub struct DataCognitoUserPool_LambdaConfig_PreTokenGenerationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCognitoUserPool_LambdaConfig_PreTokenGenerationConfig {
    #[doc = "Set the field `lambda_arn`.\n"]
    pub fn set_lambda_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lambda_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lambda_version`.\n"]
    pub fn set_lambda_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lambda_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCognitoUserPool_LambdaConfig_PreTokenGenerationConfig {
    type O = terrars::BlockAssignable<DataCognitoUserPool_LambdaConfig_PreTokenGenerationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPool_LambdaConfig_PreTokenGenerationConfig {}

impl BuildDataCognitoUserPool_LambdaConfig_PreTokenGenerationConfig {
    pub fn build(self) -> DataCognitoUserPool_LambdaConfig_PreTokenGenerationConfig {
        DataCognitoUserPool_LambdaConfig_PreTokenGenerationConfig {
            lambda_arn: core::default::Default::default(),
            lambda_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCognitoUserPool_LambdaConfig_PreTokenGenerationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCognitoUserPool_LambdaConfig_PreTokenGenerationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCognitoUserPool_LambdaConfig_PreTokenGenerationConfigRef {
        DataCognitoUserPool_LambdaConfig_PreTokenGenerationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPool_LambdaConfig_PreTokenGenerationConfigRef {
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
pub struct DataCognitoUserPool_LambdaConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_auth_challenge: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_email_sender: std::option::Option<terrars::ListField<DataCognitoUserPool_LambdaConfig_CustomEmailSender>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_sms_sender: std::option::Option<terrars::ListField<DataCognitoUserPool_LambdaConfig_CustomSmsSender>>,
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
    pre_token_generation_config: std
    ::option
    ::Option<terrars::ListField<DataCognitoUserPool_LambdaConfig_PreTokenGenerationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_migration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verify_auth_challenge_response: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCognitoUserPool_LambdaConfig {
    #[doc = "Set the field `create_auth_challenge`.\n"]
    pub fn set_create_auth_challenge(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.create_auth_challenge = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_email_sender`.\n"]
    pub fn set_custom_email_sender(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCognitoUserPool_LambdaConfig_CustomEmailSender>>,
    ) -> Self {
        self.custom_email_sender = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_message`.\n"]
    pub fn set_custom_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.custom_message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_sms_sender`.\n"]
    pub fn set_custom_sms_sender(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCognitoUserPool_LambdaConfig_CustomSmsSender>>,
    ) -> Self {
        self.custom_sms_sender = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `pre_token_generation_config`.\n"]
    pub fn set_pre_token_generation_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCognitoUserPool_LambdaConfig_PreTokenGenerationConfig>>,
    ) -> Self {
        self.pre_token_generation_config = std::option::Option::Some(v.into());
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
}

impl terrars::ToListMappable for DataCognitoUserPool_LambdaConfig {
    type O = terrars::BlockAssignable<DataCognitoUserPool_LambdaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPool_LambdaConfig {}

impl BuildDataCognitoUserPool_LambdaConfig {
    pub fn build(self) -> DataCognitoUserPool_LambdaConfig {
        DataCognitoUserPool_LambdaConfig {
            create_auth_challenge: core::default::Default::default(),
            custom_email_sender: core::default::Default::default(),
            custom_message: core::default::Default::default(),
            custom_sms_sender: core::default::Default::default(),
            define_auth_challenge: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            post_authentication: core::default::Default::default(),
            post_confirmation: core::default::Default::default(),
            pre_authentication: core::default::Default::default(),
            pre_sign_up: core::default::Default::default(),
            pre_token_generation: core::default::Default::default(),
            pre_token_generation_config: core::default::Default::default(),
            user_migration: core::default::Default::default(),
            verify_auth_challenge_response: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCognitoUserPool_LambdaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCognitoUserPool_LambdaConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCognitoUserPool_LambdaConfigRef {
        DataCognitoUserPool_LambdaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPool_LambdaConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create_auth_challenge` after provisioning.\n"]
    pub fn create_auth_challenge(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_auth_challenge", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_email_sender` after provisioning.\n"]
    pub fn custom_email_sender(&self) -> terrars::ListRef<DataCognitoUserPool_LambdaConfig_CustomEmailSenderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_email_sender", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_message` after provisioning.\n"]
    pub fn custom_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_message", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_sms_sender` after provisioning.\n"]
    pub fn custom_sms_sender(&self) -> terrars::ListRef<DataCognitoUserPool_LambdaConfig_CustomSmsSenderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_sms_sender", self.base))
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

    #[doc = "Get a reference to the value of field `pre_token_generation_config` after provisioning.\n"]
    pub fn pre_token_generation_config(
        &self,
    ) -> terrars::ListRef<DataCognitoUserPool_LambdaConfig_PreTokenGenerationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pre_token_generation_config", self.base))
    }

    #[doc = "Get a reference to the value of field `user_migration` after provisioning.\n"]
    pub fn user_migration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_migration", self.base))
    }

    #[doc = "Get a reference to the value of field `verify_auth_challenge_response` after provisioning.\n"]
    pub fn verify_auth_challenge_response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verify_auth_challenge_response", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPool_SchemaAttributes_NumberAttributeConstraints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCognitoUserPool_SchemaAttributes_NumberAttributeConstraints {
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

impl terrars::ToListMappable for DataCognitoUserPool_SchemaAttributes_NumberAttributeConstraints {
    type O = terrars::BlockAssignable<DataCognitoUserPool_SchemaAttributes_NumberAttributeConstraints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPool_SchemaAttributes_NumberAttributeConstraints {}

impl BuildDataCognitoUserPool_SchemaAttributes_NumberAttributeConstraints {
    pub fn build(self) -> DataCognitoUserPool_SchemaAttributes_NumberAttributeConstraints {
        DataCognitoUserPool_SchemaAttributes_NumberAttributeConstraints {
            max_value: core::default::Default::default(),
            min_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCognitoUserPool_SchemaAttributes_NumberAttributeConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCognitoUserPool_SchemaAttributes_NumberAttributeConstraintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCognitoUserPool_SchemaAttributes_NumberAttributeConstraintsRef {
        DataCognitoUserPool_SchemaAttributes_NumberAttributeConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPool_SchemaAttributes_NumberAttributeConstraintsRef {
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
pub struct DataCognitoUserPool_SchemaAttributes_StringAttributeConstraints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_length: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_length: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCognitoUserPool_SchemaAttributes_StringAttributeConstraints {
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

impl terrars::ToListMappable for DataCognitoUserPool_SchemaAttributes_StringAttributeConstraints {
    type O = terrars::BlockAssignable<DataCognitoUserPool_SchemaAttributes_StringAttributeConstraints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPool_SchemaAttributes_StringAttributeConstraints {}

impl BuildDataCognitoUserPool_SchemaAttributes_StringAttributeConstraints {
    pub fn build(self) -> DataCognitoUserPool_SchemaAttributes_StringAttributeConstraints {
        DataCognitoUserPool_SchemaAttributes_StringAttributeConstraints {
            max_length: core::default::Default::default(),
            min_length: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCognitoUserPool_SchemaAttributes_StringAttributeConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCognitoUserPool_SchemaAttributes_StringAttributeConstraintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCognitoUserPool_SchemaAttributes_StringAttributeConstraintsRef {
        DataCognitoUserPool_SchemaAttributes_StringAttributeConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPool_SchemaAttributes_StringAttributeConstraintsRef {
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

#[derive(Serialize)]
pub struct DataCognitoUserPool_SchemaAttributes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_data_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    developer_only_attribute: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mutable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_attribute_constraints: std
    ::option
    ::Option<terrars::ListField<DataCognitoUserPool_SchemaAttributes_NumberAttributeConstraints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_attribute_constraints: std
    ::option
    ::Option<terrars::ListField<DataCognitoUserPool_SchemaAttributes_StringAttributeConstraints>>,
}

impl DataCognitoUserPool_SchemaAttributes {
    #[doc = "Set the field `attribute_data_type`.\n"]
    pub fn set_attribute_data_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.attribute_data_type = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_attribute_constraints`.\n"]
    pub fn set_number_attribute_constraints(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCognitoUserPool_SchemaAttributes_NumberAttributeConstraints>>,
    ) -> Self {
        self.number_attribute_constraints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `required`.\n"]
    pub fn set_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_attribute_constraints`.\n"]
    pub fn set_string_attribute_constraints(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCognitoUserPool_SchemaAttributes_StringAttributeConstraints>>,
    ) -> Self {
        self.string_attribute_constraints = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCognitoUserPool_SchemaAttributes {
    type O = terrars::BlockAssignable<DataCognitoUserPool_SchemaAttributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPool_SchemaAttributes {}

impl BuildDataCognitoUserPool_SchemaAttributes {
    pub fn build(self) -> DataCognitoUserPool_SchemaAttributes {
        DataCognitoUserPool_SchemaAttributes {
            attribute_data_type: core::default::Default::default(),
            developer_only_attribute: core::default::Default::default(),
            mutable: core::default::Default::default(),
            name: core::default::Default::default(),
            number_attribute_constraints: core::default::Default::default(),
            required: core::default::Default::default(),
            string_attribute_constraints: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCognitoUserPool_SchemaAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCognitoUserPool_SchemaAttributesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCognitoUserPool_SchemaAttributesRef {
        DataCognitoUserPool_SchemaAttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPool_SchemaAttributesRef {
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

    #[doc = "Get a reference to the value of field `number_attribute_constraints` after provisioning.\n"]
    pub fn number_attribute_constraints(
        &self,
    ) -> terrars::ListRef<DataCognitoUserPool_SchemaAttributes_NumberAttributeConstraintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.number_attribute_constraints", self.base))
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `string_attribute_constraints` after provisioning.\n"]
    pub fn string_attribute_constraints(
        &self,
    ) -> terrars::ListRef<DataCognitoUserPool_SchemaAttributes_StringAttributeConstraintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_attribute_constraints", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlows {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_auth_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlows {
    #[doc = "Set the field `custom_auth_mode`.\n"]
    pub fn set_custom_auth_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.custom_auth_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlows {
    type O = terrars::BlockAssignable<DataCognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlows>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlows {}

impl BuildDataCognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlows {
    pub fn build(self) -> DataCognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlows {
        DataCognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlows {
            custom_auth_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlowsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlowsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlowsRef {
        DataCognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlowsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlowsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_auth_mode` after provisioning.\n"]
    pub fn custom_auth_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_auth_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPool_UserPoolAddOns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_security_additional_flows: std
    ::option
    ::Option<terrars::ListField<DataCognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlows>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_security_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCognitoUserPool_UserPoolAddOns {
    #[doc = "Set the field `advanced_security_additional_flows`.\n"]
    pub fn set_advanced_security_additional_flows(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlows>>,
    ) -> Self {
        self.advanced_security_additional_flows = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `advanced_security_mode`.\n"]
    pub fn set_advanced_security_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.advanced_security_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCognitoUserPool_UserPoolAddOns {
    type O = terrars::BlockAssignable<DataCognitoUserPool_UserPoolAddOns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPool_UserPoolAddOns {}

impl BuildDataCognitoUserPool_UserPoolAddOns {
    pub fn build(self) -> DataCognitoUserPool_UserPoolAddOns {
        DataCognitoUserPool_UserPoolAddOns {
            advanced_security_additional_flows: core::default::Default::default(),
            advanced_security_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCognitoUserPool_UserPoolAddOnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCognitoUserPool_UserPoolAddOnsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCognitoUserPool_UserPoolAddOnsRef {
        DataCognitoUserPool_UserPoolAddOnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPool_UserPoolAddOnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `advanced_security_additional_flows` after provisioning.\n"]
    pub fn advanced_security_additional_flows(
        &self,
    ) -> terrars::ListRef<DataCognitoUserPool_UserPoolAddOns_AdvancedSecurityAdditionalFlowsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_security_additional_flows", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_security_mode` after provisioning.\n"]
    pub fn advanced_security_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.advanced_security_mode", self.base))
    }
}
