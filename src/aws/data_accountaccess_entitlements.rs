#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAccountaccessEntitlementsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    application_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataAccountaccessEntitlements_Filter>>,
    dynamic: DataAccountaccessEntitlementsDynamic,
}

struct DataAccountaccessEntitlements_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAccountaccessEntitlementsData_>,
}

#[derive(Clone)]
pub struct DataAccountaccessEntitlements(std::rc::Rc<DataAccountaccessEntitlements_>);

impl DataAccountaccessEntitlements {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataAccountaccessEntitlements_Filter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `application_arn` after provisioning.\n"]
    pub fn application_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entitlements` after provisioning.\n"]
    pub fn entitlements(&self) -> terrars::ListRef<DataAccountaccessEntitlements_EntitlementsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entitlements", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<DataAccountaccessEntitlements_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }
}

impl terrars::Referable for DataAccountaccessEntitlements {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAccountaccessEntitlements { }

impl terrars::ToListMappable for DataAccountaccessEntitlements {
    type O = terrars::ListRef<DataAccountaccessEntitlementsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAccountaccessEntitlements_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_accountaccess_entitlements".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAccountaccessEntitlements {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub application_arn: terrars::PrimField<std::string::String>,
}

impl BuildDataAccountaccessEntitlements {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAccountaccessEntitlements {
        let out = DataAccountaccessEntitlements(std::rc::Rc::new(DataAccountaccessEntitlements_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAccountaccessEntitlementsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                application_arn: self.application_arn,
                region: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAccountaccessEntitlementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountaccessEntitlementsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAccountaccessEntitlementsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `application_arn` after provisioning.\n"]
    pub fn application_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entitlements` after provisioning.\n"]
    pub fn entitlements(&self) -> terrars::ListRef<DataAccountaccessEntitlements_EntitlementsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entitlements", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<DataAccountaccessEntitlements_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataAccountaccessEntitlementsDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataAccountaccessEntitlements_Filter>>,
}

#[derive(Serialize)]
pub struct DataAccountaccessEntitlements_Filter_PrincipalRole_Principal_IdentityCenter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAccountaccessEntitlements_Filter_PrincipalRole_Principal_IdentityCenter {
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

impl terrars::ToListMappable for DataAccountaccessEntitlements_Filter_PrincipalRole_Principal_IdentityCenter {
    type O = terrars::BlockAssignable<DataAccountaccessEntitlements_Filter_PrincipalRole_Principal_IdentityCenter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountaccessEntitlements_Filter_PrincipalRole_Principal_IdentityCenter {}

impl BuildDataAccountaccessEntitlements_Filter_PrincipalRole_Principal_IdentityCenter {
    pub fn build(self) -> DataAccountaccessEntitlements_Filter_PrincipalRole_Principal_IdentityCenter {
        DataAccountaccessEntitlements_Filter_PrincipalRole_Principal_IdentityCenter {
            group_id: core::default::Default::default(),
            user_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAccountaccessEntitlements_Filter_PrincipalRole_Principal_IdentityCenterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountaccessEntitlements_Filter_PrincipalRole_Principal_IdentityCenterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountaccessEntitlements_Filter_PrincipalRole_Principal_IdentityCenterRef {
        DataAccountaccessEntitlements_Filter_PrincipalRole_Principal_IdentityCenterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountaccessEntitlements_Filter_PrincipalRole_Principal_IdentityCenterRef {
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

#[derive(Serialize)]
pub struct DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole_Principal {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_center: std
    ::option
    ::Option<terrars::ListField<DataAccountaccessEntitlements_Filter_PrincipalRole_Principal_IdentityCenter>>,
}

impl DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole_Principal {
    #[doc = "Set the field `identity_center`.\n"]
    pub fn set_identity_center(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataAccountaccessEntitlements_Filter_PrincipalRole_Principal_IdentityCenter>,
                    >,
    ) -> Self {
        self.identity_center = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole_Principal {
    type O =
        terrars::BlockAssignable<DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole_Principal>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole_Principal {}

impl BuildDataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole_Principal {
    pub fn build(self) -> DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole_Principal {
        DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole_Principal {
            identity_center: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole_PrincipalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole_PrincipalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole_PrincipalRef {
        DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole_PrincipalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole_PrincipalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identity_center` after provisioning.\n"]
    pub fn identity_center(
        &self,
    ) -> terrars::ListRef<DataAccountaccessEntitlements_Filter_PrincipalRole_Principal_IdentityCenterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_center", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std
    ::option
    ::Option<terrars::ListField<DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole_Principal>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `account_name`.\n"]
    pub fn set_account_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole_Principal>,
                    >,
    ) -> Self {
        self.principal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole {
    type O = terrars::BlockAssignable<DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole {}

impl BuildDataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole {
    pub fn build(self) -> DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole {
        DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole {
            account_id: core::default::Default::default(),
            account_name: core::default::Default::default(),
            principal: core::default::Default::default(),
            role_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRoleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRoleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRoleRef {
        DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRoleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRoleRef {
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

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(
        &self,
    ) -> terrars::ListRef<DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole_PrincipalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountaccessEntitlements_Entitlements_Entitlement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal_role: std
    ::option
    ::Option<terrars::ListField<DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole>>,
}

impl DataAccountaccessEntitlements_Entitlements_Entitlement {
    #[doc = "Set the field `principal_role`.\n"]
    pub fn set_principal_role(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRole>>,
    ) -> Self {
        self.principal_role = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAccountaccessEntitlements_Entitlements_Entitlement {
    type O = terrars::BlockAssignable<DataAccountaccessEntitlements_Entitlements_Entitlement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountaccessEntitlements_Entitlements_Entitlement {}

impl BuildDataAccountaccessEntitlements_Entitlements_Entitlement {
    pub fn build(self) -> DataAccountaccessEntitlements_Entitlements_Entitlement {
        DataAccountaccessEntitlements_Entitlements_Entitlement { principal_role: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAccountaccessEntitlements_Entitlements_EntitlementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountaccessEntitlements_Entitlements_EntitlementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountaccessEntitlements_Entitlements_EntitlementRef {
        DataAccountaccessEntitlements_Entitlements_EntitlementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountaccessEntitlements_Entitlements_EntitlementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `principal_role` after provisioning.\n"]
    pub fn principal_role(
        &self,
    ) -> terrars::ListRef<DataAccountaccessEntitlements_Entitlements_Entitlement_PrincipalRoleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal_role", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountaccessEntitlements_Entitlements {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entitlement: std::option::Option<terrars::ListField<DataAccountaccessEntitlements_Entitlements_Entitlement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entitlement_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAccountaccessEntitlements_Entitlements {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entitlement`.\n"]
    pub fn set_entitlement(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAccountaccessEntitlements_Entitlements_Entitlement>>,
    ) -> Self {
        self.entitlement = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entitlement_id`.\n"]
    pub fn set_entitlement_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.entitlement_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAccountaccessEntitlements_Entitlements {
    type O = terrars::BlockAssignable<DataAccountaccessEntitlements_Entitlements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountaccessEntitlements_Entitlements {}

impl BuildDataAccountaccessEntitlements_Entitlements {
    pub fn build(self) -> DataAccountaccessEntitlements_Entitlements {
        DataAccountaccessEntitlements_Entitlements {
            created_at: core::default::Default::default(),
            entitlement: core::default::Default::default(),
            entitlement_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAccountaccessEntitlements_EntitlementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountaccessEntitlements_EntitlementsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountaccessEntitlements_EntitlementsRef {
        DataAccountaccessEntitlements_EntitlementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountaccessEntitlements_EntitlementsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `entitlement` after provisioning.\n"]
    pub fn entitlement(&self) -> terrars::ListRef<DataAccountaccessEntitlements_Entitlements_EntitlementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entitlement", self.base))
    }

    #[doc = "Get a reference to the value of field `entitlement_id` after provisioning.\n"]
    pub fn entitlement_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entitlement_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataAccountaccessEntitlements_Filter_PrincipalRole_PrincipalDynamic {
    identity_center: std
    ::option
    ::Option<terrars::DynamicBlock<DataAccountaccessEntitlements_Filter_PrincipalRole_Principal_IdentityCenter>>,
}

#[derive(Serialize)]
pub struct DataAccountaccessEntitlements_Filter_PrincipalRole_Principal {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_center: std
    ::option
    ::Option<std::vec::Vec<DataAccountaccessEntitlements_Filter_PrincipalRole_Principal_IdentityCenter>>,
    dynamic: DataAccountaccessEntitlements_Filter_PrincipalRole_PrincipalDynamic,
}

impl DataAccountaccessEntitlements_Filter_PrincipalRole_Principal {
    #[doc = "Set the field `identity_center`.\n"]
    pub fn set_identity_center(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataAccountaccessEntitlements_Filter_PrincipalRole_Principal_IdentityCenter>,
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

impl terrars::ToListMappable for DataAccountaccessEntitlements_Filter_PrincipalRole_Principal {
    type O = terrars::BlockAssignable<DataAccountaccessEntitlements_Filter_PrincipalRole_Principal>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountaccessEntitlements_Filter_PrincipalRole_Principal {}

impl BuildDataAccountaccessEntitlements_Filter_PrincipalRole_Principal {
    pub fn build(self) -> DataAccountaccessEntitlements_Filter_PrincipalRole_Principal {
        DataAccountaccessEntitlements_Filter_PrincipalRole_Principal {
            identity_center: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAccountaccessEntitlements_Filter_PrincipalRole_PrincipalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountaccessEntitlements_Filter_PrincipalRole_PrincipalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountaccessEntitlements_Filter_PrincipalRole_PrincipalRef {
        DataAccountaccessEntitlements_Filter_PrincipalRole_PrincipalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountaccessEntitlements_Filter_PrincipalRole_PrincipalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identity_center` after provisioning.\n"]
    pub fn identity_center(
        &self,
    ) -> terrars::ListRef<DataAccountaccessEntitlements_Filter_PrincipalRole_Principal_IdentityCenterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_center", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataAccountaccessEntitlements_Filter_PrincipalRoleDynamic {
    principal: std
    ::option
    ::Option<terrars::DynamicBlock<DataAccountaccessEntitlements_Filter_PrincipalRole_Principal>>,
}

#[derive(Serialize)]
pub struct DataAccountaccessEntitlements_Filter_PrincipalRole {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<std::vec::Vec<DataAccountaccessEntitlements_Filter_PrincipalRole_Principal>>,
    dynamic: DataAccountaccessEntitlements_Filter_PrincipalRoleDynamic,
}

impl DataAccountaccessEntitlements_Filter_PrincipalRole {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataAccountaccessEntitlements_Filter_PrincipalRole_Principal>>,
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

impl terrars::ToListMappable for DataAccountaccessEntitlements_Filter_PrincipalRole {
    type O = terrars::BlockAssignable<DataAccountaccessEntitlements_Filter_PrincipalRole>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountaccessEntitlements_Filter_PrincipalRole {}

impl BuildDataAccountaccessEntitlements_Filter_PrincipalRole {
    pub fn build(self) -> DataAccountaccessEntitlements_Filter_PrincipalRole {
        DataAccountaccessEntitlements_Filter_PrincipalRole {
            account_id: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            principal: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAccountaccessEntitlements_Filter_PrincipalRoleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountaccessEntitlements_Filter_PrincipalRoleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountaccessEntitlements_Filter_PrincipalRoleRef {
        DataAccountaccessEntitlements_Filter_PrincipalRoleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountaccessEntitlements_Filter_PrincipalRoleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::ListRef<DataAccountaccessEntitlements_Filter_PrincipalRole_PrincipalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataAccountaccessEntitlements_FilterDynamic {
    principal_role: std::option::Option<terrars::DynamicBlock<DataAccountaccessEntitlements_Filter_PrincipalRole>>,
}

#[derive(Serialize)]
pub struct DataAccountaccessEntitlements_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal_role: std::option::Option<std::vec::Vec<DataAccountaccessEntitlements_Filter_PrincipalRole>>,
    dynamic: DataAccountaccessEntitlements_FilterDynamic,
}

impl DataAccountaccessEntitlements_Filter {
    #[doc = "Set the field `principal_role`.\n"]
    pub fn set_principal_role(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataAccountaccessEntitlements_Filter_PrincipalRole>>,
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

impl terrars::ToListMappable for DataAccountaccessEntitlements_Filter {
    type O = terrars::BlockAssignable<DataAccountaccessEntitlements_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountaccessEntitlements_Filter {}

impl BuildDataAccountaccessEntitlements_Filter {
    pub fn build(self) -> DataAccountaccessEntitlements_Filter {
        DataAccountaccessEntitlements_Filter {
            principal_role: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAccountaccessEntitlements_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountaccessEntitlements_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountaccessEntitlements_FilterRef {
        DataAccountaccessEntitlements_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountaccessEntitlements_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `principal_role` after provisioning.\n"]
    pub fn principal_role(&self) -> terrars::ListRef<DataAccountaccessEntitlements_Filter_PrincipalRoleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal_role", self.base))
    }
}
