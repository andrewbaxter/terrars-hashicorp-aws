#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AccountaccessEntitlementData_ {
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
    application_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entitlement: std::option::Option<std::vec::Vec<AccountaccessEntitlement_Entitlement>>,
    dynamic: AccountaccessEntitlementDynamic,
}

struct AccountaccessEntitlement_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AccountaccessEntitlementData_>,
}

#[derive(Clone)]
pub struct AccountaccessEntitlement(std::rc::Rc<AccountaccessEntitlement_>);

impl AccountaccessEntitlement {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entitlement`.\n"]
    pub fn set_entitlement(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccountaccessEntitlement_Entitlement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().entitlement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.entitlement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `application_arn` after provisioning.\n"]
    pub fn application_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entitlement_id` after provisioning.\n"]
    pub fn entitlement_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entitlement_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entitlement` after provisioning.\n"]
    pub fn entitlement(&self) -> terrars::ListRef<AccountaccessEntitlement_EntitlementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entitlement", self.extract_ref()))
    }
}

impl terrars::Referable for AccountaccessEntitlement {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AccountaccessEntitlement { }

impl terrars::ToListMappable for AccountaccessEntitlement {
    type O = terrars::ListRef<AccountaccessEntitlementRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AccountaccessEntitlement_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_accountaccess_entitlement".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccountaccessEntitlement {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub application_arn: terrars::PrimField<std::string::String>,
}

impl BuildAccountaccessEntitlement {
    pub fn build(self, stack: &mut terrars::Stack) -> AccountaccessEntitlement {
        let out = AccountaccessEntitlement(std::rc::Rc::new(AccountaccessEntitlement_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AccountaccessEntitlementData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                application_arn: self.application_arn,
                region: core::default::Default::default(),
                entitlement: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccountaccessEntitlementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccountaccessEntitlementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccountaccessEntitlementRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_arn` after provisioning.\n"]
    pub fn application_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entitlement_id` after provisioning.\n"]
    pub fn entitlement_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entitlement_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entitlement` after provisioning.\n"]
    pub fn entitlement(&self) -> terrars::ListRef<AccountaccessEntitlement_EntitlementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entitlement", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AccountaccessEntitlementDynamic {
    entitlement: std::option::Option<terrars::DynamicBlock<AccountaccessEntitlement_Entitlement>>,
}

#[derive(Serialize)]
pub struct AccountaccessEntitlement_Entitlement_PrincipalRole_Principal_IdentityCenter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccountaccessEntitlement_Entitlement_PrincipalRole_Principal_IdentityCenter {
    #[doc = "Set the field `group_id`.\n"]
    pub fn set_group_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_id`.\n"]
    pub fn set_user_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccountaccessEntitlement_Entitlement_PrincipalRole_Principal_IdentityCenter {
    type O = terrars::BlockAssignable<AccountaccessEntitlement_Entitlement_PrincipalRole_Principal_IdentityCenter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccountaccessEntitlement_Entitlement_PrincipalRole_Principal_IdentityCenter {}

impl BuildAccountaccessEntitlement_Entitlement_PrincipalRole_Principal_IdentityCenter {
    pub fn build(self) -> AccountaccessEntitlement_Entitlement_PrincipalRole_Principal_IdentityCenter {
        AccountaccessEntitlement_Entitlement_PrincipalRole_Principal_IdentityCenter {
            group_id: core::default::Default::default(),
            user_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccountaccessEntitlement_Entitlement_PrincipalRole_Principal_IdentityCenterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccountaccessEntitlement_Entitlement_PrincipalRole_Principal_IdentityCenterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccountaccessEntitlement_Entitlement_PrincipalRole_Principal_IdentityCenterRef {
        AccountaccessEntitlement_Entitlement_PrincipalRole_Principal_IdentityCenterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccountaccessEntitlement_Entitlement_PrincipalRole_Principal_IdentityCenterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccountaccessEntitlement_Entitlement_PrincipalRole_PrincipalDynamic {
    identity_center: std
    ::option
    ::Option<terrars::DynamicBlock<AccountaccessEntitlement_Entitlement_PrincipalRole_Principal_IdentityCenter>>,
}

#[derive(Serialize)]
pub struct AccountaccessEntitlement_Entitlement_PrincipalRole_Principal {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_center: std
    ::option
    ::Option<std::vec::Vec<AccountaccessEntitlement_Entitlement_PrincipalRole_Principal_IdentityCenter>>,
    dynamic: AccountaccessEntitlement_Entitlement_PrincipalRole_PrincipalDynamic,
}

impl AccountaccessEntitlement_Entitlement_PrincipalRole_Principal {
    #[doc = "Set the field `identity_center`.\n"]
    pub fn set_identity_center(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AccountaccessEntitlement_Entitlement_PrincipalRole_Principal_IdentityCenter>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.identity_center = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.identity_center = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccountaccessEntitlement_Entitlement_PrincipalRole_Principal {
    type O = terrars::BlockAssignable<AccountaccessEntitlement_Entitlement_PrincipalRole_Principal>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccountaccessEntitlement_Entitlement_PrincipalRole_Principal {}

impl BuildAccountaccessEntitlement_Entitlement_PrincipalRole_Principal {
    pub fn build(self) -> AccountaccessEntitlement_Entitlement_PrincipalRole_Principal {
        AccountaccessEntitlement_Entitlement_PrincipalRole_Principal {
            identity_center: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccountaccessEntitlement_Entitlement_PrincipalRole_PrincipalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccountaccessEntitlement_Entitlement_PrincipalRole_PrincipalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccountaccessEntitlement_Entitlement_PrincipalRole_PrincipalRef {
        AccountaccessEntitlement_Entitlement_PrincipalRole_PrincipalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccountaccessEntitlement_Entitlement_PrincipalRole_PrincipalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identity_center` after provisioning.\n"]
    pub fn identity_center(
        &self,
    ) -> terrars::ListRef<AccountaccessEntitlement_Entitlement_PrincipalRole_Principal_IdentityCenterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_center", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccountaccessEntitlement_Entitlement_PrincipalRoleDynamic {
    principal: std
    ::option
    ::Option<terrars::DynamicBlock<AccountaccessEntitlement_Entitlement_PrincipalRole_Principal>>,
}

#[derive(Serialize)]
pub struct AccountaccessEntitlement_Entitlement_PrincipalRole {
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<std::vec::Vec<AccountaccessEntitlement_Entitlement_PrincipalRole_Principal>>,
    dynamic: AccountaccessEntitlement_Entitlement_PrincipalRoleDynamic,
}

impl AccountaccessEntitlement_Entitlement_PrincipalRole {
    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AccountaccessEntitlement_Entitlement_PrincipalRole_Principal>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.principal = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.principal = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccountaccessEntitlement_Entitlement_PrincipalRole {
    type O = terrars::BlockAssignable<AccountaccessEntitlement_Entitlement_PrincipalRole>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccountaccessEntitlement_Entitlement_PrincipalRole {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildAccountaccessEntitlement_Entitlement_PrincipalRole {
    pub fn build(self) -> AccountaccessEntitlement_Entitlement_PrincipalRole {
        AccountaccessEntitlement_Entitlement_PrincipalRole {
            role_arn: self.role_arn,
            principal: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccountaccessEntitlement_Entitlement_PrincipalRoleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccountaccessEntitlement_Entitlement_PrincipalRoleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccountaccessEntitlement_Entitlement_PrincipalRoleRef {
        AccountaccessEntitlement_Entitlement_PrincipalRoleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccountaccessEntitlement_Entitlement_PrincipalRoleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `account_name` after provisioning.\n"]
    pub fn account_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_name", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::ListRef<AccountaccessEntitlement_Entitlement_PrincipalRole_PrincipalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccountaccessEntitlement_EntitlementDynamic {
    principal_role: std::option::Option<terrars::DynamicBlock<AccountaccessEntitlement_Entitlement_PrincipalRole>>,
}

#[derive(Serialize)]
pub struct AccountaccessEntitlement_Entitlement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal_role: std::option::Option<std::vec::Vec<AccountaccessEntitlement_Entitlement_PrincipalRole>>,
    dynamic: AccountaccessEntitlement_EntitlementDynamic,
}

impl AccountaccessEntitlement_Entitlement {
    #[doc = "Set the field `principal_role`.\n"]
    pub fn set_principal_role(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccountaccessEntitlement_Entitlement_PrincipalRole>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.principal_role = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.principal_role = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccountaccessEntitlement_Entitlement {
    type O = terrars::BlockAssignable<AccountaccessEntitlement_Entitlement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccountaccessEntitlement_Entitlement {}

impl BuildAccountaccessEntitlement_Entitlement {
    pub fn build(self) -> AccountaccessEntitlement_Entitlement {
        AccountaccessEntitlement_Entitlement {
            principal_role: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccountaccessEntitlement_EntitlementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccountaccessEntitlement_EntitlementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccountaccessEntitlement_EntitlementRef {
        AccountaccessEntitlement_EntitlementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccountaccessEntitlement_EntitlementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `principal_role` after provisioning.\n"]
    pub fn principal_role(&self) -> terrars::ListRef<AccountaccessEntitlement_Entitlement_PrincipalRoleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal_role", self.base))
    }
}
