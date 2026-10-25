#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOrganizationsOrganizationData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_organization_only: std::option::Option<terrars::PrimField<bool>>,
}

struct DataOrganizationsOrganization_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOrganizationsOrganizationData_>,
}

#[derive(Clone)]
pub struct DataOrganizationsOrganization(std::rc::Rc<DataOrganizationsOrganization_>);

impl DataOrganizationsOrganization {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `return_organization_only`.\n"]
    pub fn set_return_organization_only(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().return_organization_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `accounts` after provisioning.\n"]
    pub fn accounts(&self) -> terrars::ListRef<DataOrganizationsOrganization_AccountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accounts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_service_access_principals` after provisioning.\n"]
    pub fn aws_service_access_principals(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.aws_service_access_principals", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled_policy_types` after provisioning.\n"]
    pub fn enabled_policy_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.enabled_policy_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feature_set` after provisioning.\n"]
    pub fn feature_set(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature_set", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_account_arn` after provisioning.\n"]
    pub fn master_account_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_account_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_account_email` after provisioning.\n"]
    pub fn master_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_account_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_account_id` after provisioning.\n"]
    pub fn master_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_account_name` after provisioning.\n"]
    pub fn master_account_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_account_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `non_master_accounts` after provisioning.\n"]
    pub fn non_master_accounts(&self) -> terrars::ListRef<DataOrganizationsOrganization_NonMasterAccountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.non_master_accounts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `return_organization_only` after provisioning.\n"]
    pub fn return_organization_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return_organization_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `roots` after provisioning.\n"]
    pub fn roots(&self) -> terrars::ListRef<DataOrganizationsOrganization_RootsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.roots", self.extract_ref()))
    }
}

impl terrars::Referable for DataOrganizationsOrganization {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOrganizationsOrganization { }

impl terrars::ToListMappable for DataOrganizationsOrganization {
    type O = terrars::ListRef<DataOrganizationsOrganizationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOrganizationsOrganization_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_organizations_organization".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOrganizationsOrganization {
    pub tf_id: std::string::String,
}

impl BuildDataOrganizationsOrganization {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOrganizationsOrganization {
        let out = DataOrganizationsOrganization(std::rc::Rc::new(DataOrganizationsOrganization_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOrganizationsOrganizationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                return_organization_only: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOrganizationsOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizationsOrganizationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOrganizationsOrganizationRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `accounts` after provisioning.\n"]
    pub fn accounts(&self) -> terrars::ListRef<DataOrganizationsOrganization_AccountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accounts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_service_access_principals` after provisioning.\n"]
    pub fn aws_service_access_principals(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.aws_service_access_principals", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled_policy_types` after provisioning.\n"]
    pub fn enabled_policy_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.enabled_policy_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feature_set` after provisioning.\n"]
    pub fn feature_set(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature_set", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_account_arn` after provisioning.\n"]
    pub fn master_account_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_account_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_account_email` after provisioning.\n"]
    pub fn master_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_account_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_account_id` after provisioning.\n"]
    pub fn master_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_account_name` after provisioning.\n"]
    pub fn master_account_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_account_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `non_master_accounts` after provisioning.\n"]
    pub fn non_master_accounts(&self) -> terrars::ListRef<DataOrganizationsOrganization_NonMasterAccountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.non_master_accounts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `return_organization_only` after provisioning.\n"]
    pub fn return_organization_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return_organization_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `roots` after provisioning.\n"]
    pub fn roots(&self) -> terrars::ListRef<DataOrganizationsOrganization_RootsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.roots", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOrganizationsOrganization_Accounts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    joined_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    joined_timestamp: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOrganizationsOrganization_Accounts {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `joined_method`.\n"]
    pub fn set_joined_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.joined_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `joined_timestamp`.\n"]
    pub fn set_joined_timestamp(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.joined_timestamp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOrganizationsOrganization_Accounts {
    type O = terrars::BlockAssignable<DataOrganizationsOrganization_Accounts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationsOrganization_Accounts {}

impl BuildDataOrganizationsOrganization_Accounts {
    pub fn build(self) -> DataOrganizationsOrganization_Accounts {
        DataOrganizationsOrganization_Accounts {
            arn: core::default::Default::default(),
            email: core::default::Default::default(),
            id: core::default::Default::default(),
            joined_method: core::default::Default::default(),
            joined_timestamp: core::default::Default::default(),
            name: core::default::Default::default(),
            state: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOrganizationsOrganization_AccountsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizationsOrganization_AccountsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOrganizationsOrganization_AccountsRef {
        DataOrganizationsOrganization_AccountsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationsOrganization_AccountsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `joined_method` after provisioning.\n"]
    pub fn joined_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.joined_method", self.base))
    }

    #[doc = "Get a reference to the value of field `joined_timestamp` after provisioning.\n"]
    pub fn joined_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.joined_timestamp", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOrganizationsOrganization_NonMasterAccounts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    joined_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    joined_timestamp: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOrganizationsOrganization_NonMasterAccounts {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `joined_method`.\n"]
    pub fn set_joined_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.joined_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `joined_timestamp`.\n"]
    pub fn set_joined_timestamp(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.joined_timestamp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOrganizationsOrganization_NonMasterAccounts {
    type O = terrars::BlockAssignable<DataOrganizationsOrganization_NonMasterAccounts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationsOrganization_NonMasterAccounts {}

impl BuildDataOrganizationsOrganization_NonMasterAccounts {
    pub fn build(self) -> DataOrganizationsOrganization_NonMasterAccounts {
        DataOrganizationsOrganization_NonMasterAccounts {
            arn: core::default::Default::default(),
            email: core::default::Default::default(),
            id: core::default::Default::default(),
            joined_method: core::default::Default::default(),
            joined_timestamp: core::default::Default::default(),
            name: core::default::Default::default(),
            state: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOrganizationsOrganization_NonMasterAccountsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizationsOrganization_NonMasterAccountsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOrganizationsOrganization_NonMasterAccountsRef {
        DataOrganizationsOrganization_NonMasterAccountsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationsOrganization_NonMasterAccountsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `joined_method` after provisioning.\n"]
    pub fn joined_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.joined_method", self.base))
    }

    #[doc = "Get a reference to the value of field `joined_timestamp` after provisioning.\n"]
    pub fn joined_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.joined_timestamp", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOrganizationsOrganization_Roots_PolicyTypes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOrganizationsOrganization_Roots_PolicyTypes {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOrganizationsOrganization_Roots_PolicyTypes {
    type O = terrars::BlockAssignable<DataOrganizationsOrganization_Roots_PolicyTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationsOrganization_Roots_PolicyTypes {}

impl BuildDataOrganizationsOrganization_Roots_PolicyTypes {
    pub fn build(self) -> DataOrganizationsOrganization_Roots_PolicyTypes {
        DataOrganizationsOrganization_Roots_PolicyTypes {
            status: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOrganizationsOrganization_Roots_PolicyTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizationsOrganization_Roots_PolicyTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOrganizationsOrganization_Roots_PolicyTypesRef {
        DataOrganizationsOrganization_Roots_PolicyTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationsOrganization_Roots_PolicyTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOrganizationsOrganization_Roots {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_types: std::option::Option<terrars::ListField<DataOrganizationsOrganization_Roots_PolicyTypes>>,
}

impl DataOrganizationsOrganization_Roots {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_types`.\n"]
    pub fn set_policy_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOrganizationsOrganization_Roots_PolicyTypes>>,
    ) -> Self {
        self.policy_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOrganizationsOrganization_Roots {
    type O = terrars::BlockAssignable<DataOrganizationsOrganization_Roots>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationsOrganization_Roots {}

impl BuildDataOrganizationsOrganization_Roots {
    pub fn build(self) -> DataOrganizationsOrganization_Roots {
        DataOrganizationsOrganization_Roots {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            policy_types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOrganizationsOrganization_RootsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizationsOrganization_RootsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOrganizationsOrganization_RootsRef {
        DataOrganizationsOrganization_RootsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationsOrganization_RootsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_types` after provisioning.\n"]
    pub fn policy_types(&self) -> terrars::ListRef<DataOrganizationsOrganization_Roots_PolicyTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_types", self.base))
    }
}
