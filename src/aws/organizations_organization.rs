use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OrganizationsOrganizationData {
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
    aws_service_access_principals: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled_policy_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feature_set: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct OrganizationsOrganization_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OrganizationsOrganizationData>,
}

#[derive(Clone)]
pub struct OrganizationsOrganization(std::rc::Rc<OrganizationsOrganization_>);

impl OrganizationsOrganization {
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

    #[doc = "Set the field `aws_service_access_principals`.\n"]
    pub fn set_aws_service_access_principals(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().aws_service_access_principals = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled_policy_types`.\n"]
    pub fn set_enabled_policy_types(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().enabled_policy_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `feature_set`.\n"]
    pub fn set_feature_set(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().feature_set = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `accounts` after provisioning.\n"]
    pub fn accounts(&self) -> terrars::ListRef<OrganizationsOrganizationAccountsElRef> {
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
    pub fn non_master_accounts(&self) -> terrars::ListRef<OrganizationsOrganizationNonMasterAccountsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.non_master_accounts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `roots` after provisioning.\n"]
    pub fn roots(&self) -> terrars::ListRef<OrganizationsOrganizationRootsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.roots", self.extract_ref()))
    }
}

impl terrars::Referable for OrganizationsOrganization {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OrganizationsOrganization { }

impl terrars::ToListMappable for OrganizationsOrganization {
    type O = terrars::ListRef<OrganizationsOrganizationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OrganizationsOrganization_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_organizations_organization".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOrganizationsOrganization {
    pub tf_id: std::string::String,
}

impl BuildOrganizationsOrganization {
    pub fn build(self, stack: &mut terrars::Stack) -> OrganizationsOrganization {
        let out = OrganizationsOrganization(std::rc::Rc::new(OrganizationsOrganization_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OrganizationsOrganizationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                aws_service_access_principals: core::default::Default::default(),
                enabled_policy_types: core::default::Default::default(),
                feature_set: core::default::Default::default(),
                id: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OrganizationsOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OrganizationsOrganizationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OrganizationsOrganizationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accounts` after provisioning.\n"]
    pub fn accounts(&self) -> terrars::ListRef<OrganizationsOrganizationAccountsElRef> {
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
    pub fn non_master_accounts(&self) -> terrars::ListRef<OrganizationsOrganizationNonMasterAccountsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.non_master_accounts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `roots` after provisioning.\n"]
    pub fn roots(&self) -> terrars::ListRef<OrganizationsOrganizationRootsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.roots", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct OrganizationsOrganizationAccountsEl {
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

impl OrganizationsOrganizationAccountsEl {
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

impl terrars::ToListMappable for OrganizationsOrganizationAccountsEl {
    type O = terrars::BlockAssignable<OrganizationsOrganizationAccountsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganizationsOrganizationAccountsEl {}

impl BuildOrganizationsOrganizationAccountsEl {
    pub fn build(self) -> OrganizationsOrganizationAccountsEl {
        OrganizationsOrganizationAccountsEl {
            arn: core::default::Default::default(),
            email: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct OrganizationsOrganizationAccountsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OrganizationsOrganizationAccountsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OrganizationsOrganizationAccountsElRef {
        OrganizationsOrganizationAccountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrganizationsOrganizationAccountsElRef {
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
pub struct OrganizationsOrganizationNonMasterAccountsEl {
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

impl OrganizationsOrganizationNonMasterAccountsEl {
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

impl terrars::ToListMappable for OrganizationsOrganizationNonMasterAccountsEl {
    type O = terrars::BlockAssignable<OrganizationsOrganizationNonMasterAccountsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganizationsOrganizationNonMasterAccountsEl {}

impl BuildOrganizationsOrganizationNonMasterAccountsEl {
    pub fn build(self) -> OrganizationsOrganizationNonMasterAccountsEl {
        OrganizationsOrganizationNonMasterAccountsEl {
            arn: core::default::Default::default(),
            email: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct OrganizationsOrganizationNonMasterAccountsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OrganizationsOrganizationNonMasterAccountsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OrganizationsOrganizationNonMasterAccountsElRef {
        OrganizationsOrganizationNonMasterAccountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrganizationsOrganizationNonMasterAccountsElRef {
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
pub struct OrganizationsOrganizationRootsElPolicyTypesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OrganizationsOrganizationRootsElPolicyTypesEl {
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

impl terrars::ToListMappable for OrganizationsOrganizationRootsElPolicyTypesEl {
    type O = terrars::BlockAssignable<OrganizationsOrganizationRootsElPolicyTypesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganizationsOrganizationRootsElPolicyTypesEl {}

impl BuildOrganizationsOrganizationRootsElPolicyTypesEl {
    pub fn build(self) -> OrganizationsOrganizationRootsElPolicyTypesEl {
        OrganizationsOrganizationRootsElPolicyTypesEl {
            status: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct OrganizationsOrganizationRootsElPolicyTypesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OrganizationsOrganizationRootsElPolicyTypesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OrganizationsOrganizationRootsElPolicyTypesElRef {
        OrganizationsOrganizationRootsElPolicyTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrganizationsOrganizationRootsElPolicyTypesElRef {
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
pub struct OrganizationsOrganizationRootsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_types: std::option::Option<terrars::ListField<OrganizationsOrganizationRootsElPolicyTypesEl>>,
}

impl OrganizationsOrganizationRootsEl {
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
        v: impl std::convert::Into<terrars::ListField<OrganizationsOrganizationRootsElPolicyTypesEl>>,
    ) -> Self {
        self.policy_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OrganizationsOrganizationRootsEl {
    type O = terrars::BlockAssignable<OrganizationsOrganizationRootsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganizationsOrganizationRootsEl {}

impl BuildOrganizationsOrganizationRootsEl {
    pub fn build(self) -> OrganizationsOrganizationRootsEl {
        OrganizationsOrganizationRootsEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            policy_types: core::default::Default::default(),
        }
    }
}

pub struct OrganizationsOrganizationRootsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OrganizationsOrganizationRootsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OrganizationsOrganizationRootsElRef {
        OrganizationsOrganizationRootsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrganizationsOrganizationRootsElRef {
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
    pub fn policy_types(&self) -> terrars::ListRef<OrganizationsOrganizationRootsElPolicyTypesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_types", self.base))
    }
}
