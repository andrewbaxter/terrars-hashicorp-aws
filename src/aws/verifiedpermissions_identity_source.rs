#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VerifiedpermissionsIdentitySourceData_ {
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
    policy_store_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal_entity_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<VerifiedpermissionsIdentitySource_Configuration>>,
    dynamic: VerifiedpermissionsIdentitySourceDynamic,
}

struct VerifiedpermissionsIdentitySource_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VerifiedpermissionsIdentitySourceData_>,
}

#[derive(Clone)]
pub struct VerifiedpermissionsIdentitySource(std::rc::Rc<VerifiedpermissionsIdentitySource_>);

impl VerifiedpermissionsIdentitySource {
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

    #[doc = "Set the field `principal_entity_type`.\n"]
    pub fn set_principal_entity_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().principal_entity_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VerifiedpermissionsIdentitySource_Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_store_id` after provisioning.\n"]
    pub fn policy_store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_store_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal_entity_type` after provisioning.\n"]
    pub fn principal_entity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_entity_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<VerifiedpermissionsIdentitySource_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

impl terrars::Referable for VerifiedpermissionsIdentitySource {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VerifiedpermissionsIdentitySource { }

impl terrars::ToListMappable for VerifiedpermissionsIdentitySource {
    type O = terrars::ListRef<VerifiedpermissionsIdentitySourceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VerifiedpermissionsIdentitySource_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_verifiedpermissions_identity_source".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVerifiedpermissionsIdentitySource {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub policy_store_id: terrars::PrimField<std::string::String>,
}

impl BuildVerifiedpermissionsIdentitySource {
    pub fn build(self, stack: &mut terrars::Stack) -> VerifiedpermissionsIdentitySource {
        let out = VerifiedpermissionsIdentitySource(std::rc::Rc::new(VerifiedpermissionsIdentitySource_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VerifiedpermissionsIdentitySourceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                policy_store_id: self.policy_store_id,
                principal_entity_type: core::default::Default::default(),
                region: core::default::Default::default(),
                configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VerifiedpermissionsIdentitySourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedpermissionsIdentitySourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VerifiedpermissionsIdentitySourceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_store_id` after provisioning.\n"]
    pub fn policy_store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_store_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal_entity_type` after provisioning.\n"]
    pub fn principal_entity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_entity_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<VerifiedpermissionsIdentitySource_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct VerifiedpermissionsIdentitySourceDynamic {
    configuration: std::option::Option<terrars::DynamicBlock<VerifiedpermissionsIdentitySource_Configuration>>,
}

#[derive(Serialize)]
pub struct VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration_GroupConfiguration {
    group_entity_type: terrars::PrimField<std::string::String>,
}

impl VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration_GroupConfiguration { }

impl terrars
::ToListMappable for VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration_GroupConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration_GroupConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration_GroupConfiguration {
    #[doc = ""]
    pub group_entity_type: terrars::PrimField<std::string::String>,
}

impl BuildVerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration_GroupConfiguration {
    pub fn build(
        self,
    ) -> VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration_GroupConfiguration {
        VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration_GroupConfiguration {
            group_entity_type: self.group_entity_type,
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration_GroupConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration_GroupConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration_GroupConfigurationRef {
        VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration_GroupConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration_GroupConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_entity_type` after provisioning.\n"]
    pub fn group_entity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_entity_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfigurationDynamic {
    group_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration_GroupConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    user_pool_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_configuration: std
    ::option
    ::Option<
        std::vec::Vec<VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration_GroupConfiguration>,
    >,
    dynamic: VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfigurationDynamic,
}

impl VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration {
    #[doc = "Set the field `client_ids`.\n"]
    pub fn set_client_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.client_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_configuration`.\n"]
    pub fn set_group_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration_GroupConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.group_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.group_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration {
    type O = terrars::BlockAssignable<VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration {
    #[doc = ""]
    pub user_pool_arn: terrars::PrimField<std::string::String>,
}

impl BuildVerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration {
    pub fn build(self) -> VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration {
        VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration {
            client_ids: core::default::Default::default(),
            user_pool_arn: self.user_pool_arn,
            group_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfigurationRef {
        VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_ids` after provisioning.\n"]
    pub fn client_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `user_pool_arn` after provisioning.\n"]
    pub fn user_pool_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `group_configuration` after provisioning.\n"]
    pub fn group_configuration(
        &self,
    ) -> terrars
    ::ListRef<VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration_GroupConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_GroupConfiguration {
    group_claim: terrars::PrimField<std::string::String>,
    group_entity_type: terrars::PrimField<std::string::String>,
}

impl VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_GroupConfiguration { }

impl terrars
::ToListMappable for VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_GroupConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_GroupConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_GroupConfiguration {
    #[doc = ""]
    pub group_claim: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub group_entity_type: terrars::PrimField<std::string::String>,
}

impl BuildVerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_GroupConfiguration {
    pub fn build(
        self,
    ) -> VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_GroupConfiguration {
        VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_GroupConfiguration {
            group_claim: self.group_claim,
            group_entity_type: self.group_entity_type,
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_GroupConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_GroupConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_GroupConfigurationRef {
        VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_GroupConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_GroupConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_claim` after provisioning.\n"]
    pub fn group_claim(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_claim", self.base))
    }

    #[doc = "Get a reference to the value of field `group_entity_type` after provisioning.\n"]
    pub fn group_entity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_entity_type", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_AccessTokenOnly {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audiences: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal_id_claim: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_AccessTokenOnly {
    #[doc = "Set the field `audiences`.\n"]
    pub fn set_audiences(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.audiences = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principal_id_claim`.\n"]
    pub fn set_principal_id_claim(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.principal_id_claim = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_AccessTokenOnly {
    type O =
        terrars
        ::BlockAssignable<
            VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_AccessTokenOnly,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_AccessTokenOnly {}

impl BuildVerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_AccessTokenOnly {
    pub fn build(
        self,
    ) -> VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_AccessTokenOnly {
        VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_AccessTokenOnly {
            audiences: core::default::Default::default(),
            principal_id_claim: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_AccessTokenOnlyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_AccessTokenOnlyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_AccessTokenOnlyRef {
        VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_AccessTokenOnlyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_AccessTokenOnlyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audiences` after provisioning.\n"]
    pub fn audiences(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audiences", self.base))
    }

    #[doc = "Get a reference to the value of field `principal_id_claim` after provisioning.\n"]
    pub fn principal_id_claim(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_id_claim", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_IdentityTokenOnly {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal_id_claim: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_IdentityTokenOnly {
    #[doc = "Set the field `client_ids`.\n"]
    pub fn set_client_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.client_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principal_id_claim`.\n"]
    pub fn set_principal_id_claim(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.principal_id_claim = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_IdentityTokenOnly {
    type O =
        terrars
        ::BlockAssignable<
            VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_IdentityTokenOnly,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_IdentityTokenOnly {}

impl BuildVerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_IdentityTokenOnly {
    pub fn build(
        self,
    ) -> VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_IdentityTokenOnly {
        VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_IdentityTokenOnly {
            client_ids: core::default::Default::default(),
            principal_id_claim: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_IdentityTokenOnlyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_IdentityTokenOnlyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_IdentityTokenOnlyRef {
        VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_IdentityTokenOnlyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_IdentityTokenOnlyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_ids` after provisioning.\n"]
    pub fn client_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `principal_id_claim` after provisioning.\n"]
    pub fn principal_id_claim(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_id_claim", self.base))
    }
}

#[derive(Serialize, Default)]
struct VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelectionDynamic {
    access_token_only: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_AccessTokenOnly,
        >,
    >,
    identity_token_only: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_IdentityTokenOnly,
        >,
    >,
}

#[derive(Serialize)]
pub struct VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_token_only: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_AccessTokenOnly,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_token_only: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_IdentityTokenOnly,
        >,
    >,
    dynamic: VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelectionDynamic,
}

impl VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection {
    #[doc = "Set the field `access_token_only`.\n"]
    pub fn set_access_token_only(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_AccessTokenOnly,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access_token_only = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access_token_only = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `identity_token_only`.\n"]
    pub fn set_identity_token_only(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_IdentityTokenOnly,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.identity_token_only = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.identity_token_only = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection {
    type O =
        terrars
        ::BlockAssignable<VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection {}

impl BuildVerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection {
    pub fn build(self) -> VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection {
        VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection {
            access_token_only: core::default::Default::default(),
            identity_token_only: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelectionRef {
        VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_token_only` after provisioning.\n"]
    pub fn access_token_only(
        &self,
    ) -> terrars
    ::ListRef<
        VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_AccessTokenOnlyRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_token_only", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_token_only` after provisioning.\n"]
    pub fn identity_token_only(
        &self,
    ) -> terrars
    ::ListRef<
        VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection_IdentityTokenOnlyRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_token_only", self.base))
    }
}

#[derive(Serialize, Default)]
struct VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfigurationDynamic {
    group_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_GroupConfiguration>,
    >,
    token_selection: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection>,
    >,
}

#[derive(Serialize)]
pub struct VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entity_id_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    issuer: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_configuration: std
    ::option
    ::Option<
        std::vec::Vec<VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_GroupConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_selection: std
    ::option
    ::Option<
        std::vec::Vec<VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection>,
    >,
    dynamic: VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfigurationDynamic,
}

impl VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration {
    #[doc = "Set the field `entity_id_prefix`.\n"]
    pub fn set_entity_id_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.entity_id_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_configuration`.\n"]
    pub fn set_group_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_GroupConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.group_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.group_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `token_selection`.\n"]
    pub fn set_token_selection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelection,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.token_selection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.token_selection = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration {
    type O = terrars::BlockAssignable<VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration {
    #[doc = ""]
    pub issuer: terrars::PrimField<std::string::String>,
}

impl BuildVerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration {
    pub fn build(self) -> VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration {
        VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration {
            entity_id_prefix: core::default::Default::default(),
            issuer: self.issuer,
            group_configuration: core::default::Default::default(),
            token_selection: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfigurationRef {
        VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entity_id_prefix` after provisioning.\n"]
    pub fn entity_id_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_id_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `group_configuration` after provisioning.\n"]
    pub fn group_configuration(
        &self,
    ) -> terrars
    ::ListRef<VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_GroupConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `token_selection` after provisioning.\n"]
    pub fn token_selection(
        &self,
    ) -> terrars
    ::ListRef<VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration_TokenSelectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.token_selection", self.base))
    }
}

#[derive(Serialize, Default)]
struct VerifiedpermissionsIdentitySource_ConfigurationDynamic {
    cognito_user_pool_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration>>,
    open_id_connect_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration>>,
}

#[derive(Serialize)]
pub struct VerifiedpermissionsIdentitySource_Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cognito_user_pool_configuration: std
    ::option
    ::Option<std::vec::Vec<VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_id_connect_configuration: std
    ::option
    ::Option<std::vec::Vec<VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration>>,
    dynamic: VerifiedpermissionsIdentitySource_ConfigurationDynamic,
}

impl VerifiedpermissionsIdentitySource_Configuration {
    #[doc = "Set the field `cognito_user_pool_configuration`.\n"]
    pub fn set_cognito_user_pool_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cognito_user_pool_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cognito_user_pool_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `open_id_connect_configuration`.\n"]
    pub fn set_open_id_connect_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.open_id_connect_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.open_id_connect_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VerifiedpermissionsIdentitySource_Configuration {
    type O = terrars::BlockAssignable<VerifiedpermissionsIdentitySource_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsIdentitySource_Configuration {}

impl BuildVerifiedpermissionsIdentitySource_Configuration {
    pub fn build(self) -> VerifiedpermissionsIdentitySource_Configuration {
        VerifiedpermissionsIdentitySource_Configuration {
            cognito_user_pool_configuration: core::default::Default::default(),
            open_id_connect_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedpermissionsIdentitySource_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedpermissionsIdentitySource_ConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VerifiedpermissionsIdentitySource_ConfigurationRef {
        VerifiedpermissionsIdentitySource_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsIdentitySource_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cognito_user_pool_configuration` after provisioning.\n"]
    pub fn cognito_user_pool_configuration(
        &self,
    ) -> terrars::ListRef<VerifiedpermissionsIdentitySource_Configuration_CognitoUserPoolConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cognito_user_pool_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `open_id_connect_configuration` after provisioning.\n"]
    pub fn open_id_connect_configuration(
        &self,
    ) -> terrars::ListRef<VerifiedpermissionsIdentitySource_Configuration_OpenIdConnectConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_id_connect_configuration", self.base))
    }
}
