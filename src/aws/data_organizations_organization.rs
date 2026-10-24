use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOrganizationsOrganizationData {
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
}

struct DataOrganizationsOrganization_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOrganizationsOrganizationData>,
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

    #[doc = "Get a reference to the value of field `accounts` after provisioning.\n"]
    pub fn accounts(&self) -> terrars::ListRef<DataOrganizationsOrganizationAccountsElRef> {
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

    #[doc = "Get a reference to the value of field `non_master_accounts` after provisioning.\n"]
    pub fn non_master_accounts(&self) -> terrars::ListRef<DataOrganizationsOrganizationNonMasterAccountsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.non_master_accounts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `roots` after provisioning.\n"]
    pub fn roots(&self) -> terrars::ListRef<DataOrganizationsOrganizationRootsElRef> {
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
            data: std::cell::RefCell::new(DataOrganizationsOrganizationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
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
    pub fn accounts(&self) -> terrars::ListRef<DataOrganizationsOrganizationAccountsElRef> {
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

    #[doc = "Get a reference to the value of field `non_master_accounts` after provisioning.\n"]
    pub fn non_master_accounts(&self) -> terrars::ListRef<DataOrganizationsOrganizationNonMasterAccountsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.non_master_accounts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `roots` after provisioning.\n"]
    pub fn roots(&self) -> terrars::ListRef<DataOrganizationsOrganizationRootsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.roots", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOrganizationsOrganizationAccountsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOrganizationsOrganizationAccountsEl {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOrganizationsOrganizationAccountsEl {
    type O = terrars::BlockAssignable<DataOrganizationsOrganizationAccountsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationsOrganizationAccountsEl {}

impl BuildDataOrganizationsOrganizationAccountsEl {
    pub fn build(self) -> DataOrganizationsOrganizationAccountsEl {
        DataOrganizationsOrganizationAccountsEl {
            arn: core::default::Default::default(),
            email: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct DataOrganizationsOrganizationAccountsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizationsOrganizationAccountsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOrganizationsOrganizationAccountsElRef {
        DataOrganizationsOrganizationAccountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationsOrganizationAccountsElRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOrganizationsOrganizationNonMasterAccountsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOrganizationsOrganizationNonMasterAccountsEl {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOrganizationsOrganizationNonMasterAccountsEl {
    type O = terrars::BlockAssignable<DataOrganizationsOrganizationNonMasterAccountsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationsOrganizationNonMasterAccountsEl {}

impl BuildDataOrganizationsOrganizationNonMasterAccountsEl {
    pub fn build(self) -> DataOrganizationsOrganizationNonMasterAccountsEl {
        DataOrganizationsOrganizationNonMasterAccountsEl {
            arn: core::default::Default::default(),
            email: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct DataOrganizationsOrganizationNonMasterAccountsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizationsOrganizationNonMasterAccountsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOrganizationsOrganizationNonMasterAccountsElRef {
        DataOrganizationsOrganizationNonMasterAccountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationsOrganizationNonMasterAccountsElRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOrganizationsOrganizationRootsElPolicyTypesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOrganizationsOrganizationRootsElPolicyTypesEl {
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

impl terrars::ToListMappable for DataOrganizationsOrganizationRootsElPolicyTypesEl {
    type O = terrars::BlockAssignable<DataOrganizationsOrganizationRootsElPolicyTypesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationsOrganizationRootsElPolicyTypesEl {}

impl BuildDataOrganizationsOrganizationRootsElPolicyTypesEl {
    pub fn build(self) -> DataOrganizationsOrganizationRootsElPolicyTypesEl {
        DataOrganizationsOrganizationRootsElPolicyTypesEl {
            status: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataOrganizationsOrganizationRootsElPolicyTypesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizationsOrganizationRootsElPolicyTypesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOrganizationsOrganizationRootsElPolicyTypesElRef {
        DataOrganizationsOrganizationRootsElPolicyTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationsOrganizationRootsElPolicyTypesElRef {
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
pub struct DataOrganizationsOrganizationRootsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_types: std::option::Option<terrars::ListField<DataOrganizationsOrganizationRootsElPolicyTypesEl>>,
}

impl DataOrganizationsOrganizationRootsEl {
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
        v: impl std::convert::Into<terrars::ListField<DataOrganizationsOrganizationRootsElPolicyTypesEl>>,
    ) -> Self {
        self.policy_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOrganizationsOrganizationRootsEl {
    type O = terrars::BlockAssignable<DataOrganizationsOrganizationRootsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationsOrganizationRootsEl {}

impl BuildDataOrganizationsOrganizationRootsEl {
    pub fn build(self) -> DataOrganizationsOrganizationRootsEl {
        DataOrganizationsOrganizationRootsEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            policy_types: core::default::Default::default(),
        }
    }
}

pub struct DataOrganizationsOrganizationRootsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizationsOrganizationRootsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOrganizationsOrganizationRootsElRef {
        DataOrganizationsOrganizationRootsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationsOrganizationRootsElRef {
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
    pub fn policy_types(&self) -> terrars::ListRef<DataOrganizationsOrganizationRootsElPolicyTypesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_types", self.base))
    }
}
